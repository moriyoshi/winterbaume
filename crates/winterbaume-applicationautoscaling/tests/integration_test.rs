//! Integration tests for winterbaume Application Auto Scaling service.

use aws_sdk_applicationautoscaling::config::BehaviorVersion;
use aws_sdk_applicationautoscaling::types::{
    AdjustmentType, MetricType, PolicyType, PredefinedMetricSpecification, ScalableDimension,
    ServiceNamespace, StepAdjustment, StepScalingPolicyConfiguration,
    SuspendedState as SdkSuspendedState, TargetTrackingScalingPolicyConfiguration,
};
use winterbaume_applicationautoscaling::ApplicationAutoScalingService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_applicationautoscaling::Client {
    let mock = MockAws::builder()
        .with_service(ApplicationAutoScalingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_applicationautoscaling::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    aws_sdk_applicationautoscaling::Client::new(&config)
}

#[tokio::test]
async fn test_register_scalable_target() {
    let client = make_client().await;

    client
        .register_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/my-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .min_capacity(1)
        .max_capacity(10)
        .send()
        .await
        .expect("register_scalable_target should succeed");
}

#[tokio::test]
async fn test_describe_scalable_targets() {
    let client = make_client().await;

    client
        .register_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/my-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .min_capacity(1)
        .max_capacity(10)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_scalable_targets()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .expect("describe_scalable_targets should succeed");

    let targets = resp.scalable_targets();
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].resource_id(), "service/default/my-service");
    assert_eq!(targets[0].min_capacity(), 1);
    assert_eq!(targets[0].max_capacity(), 10);
}

#[tokio::test]
async fn test_deregister_scalable_target() {
    let client = make_client().await;

    client
        .register_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/my-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .min_capacity(1)
        .max_capacity(10)
        .send()
        .await
        .unwrap();

    client
        .deregister_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/my-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .send()
        .await
        .expect("deregister_scalable_target should succeed");

    let resp = client
        .describe_scalable_targets()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.scalable_targets().len(), 0);
}

#[tokio::test]
async fn test_put_scaling_policy() {
    let client = make_client().await;

    // Register a scalable target first
    client
        .register_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/my-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .min_capacity(1)
        .max_capacity(10)
        .send()
        .await
        .unwrap();

    let resp = client
        .put_scaling_policy()
        .policy_name("my-scaling-policy")
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/my-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .policy_type(PolicyType::TargetTrackingScaling)
        .target_tracking_scaling_policy_configuration(
            TargetTrackingScalingPolicyConfiguration::builder()
                .target_value(50.0)
                .predefined_metric_specification(
                    PredefinedMetricSpecification::builder()
                        .predefined_metric_type(MetricType::EcsServiceAverageCpuUtilization)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_scaling_policy should succeed");

    assert!(!resp.policy_arn().is_empty());
}

#[tokio::test]
async fn test_describe_scaling_policies() {
    let client = make_client().await;

    client
        .register_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/my-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .min_capacity(1)
        .max_capacity(10)
        .send()
        .await
        .unwrap();

    client
        .put_scaling_policy()
        .policy_name("my-policy")
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/my-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .policy_type(PolicyType::TargetTrackingScaling)
        .target_tracking_scaling_policy_configuration(
            TargetTrackingScalingPolicyConfiguration::builder()
                .target_value(70.0)
                .predefined_metric_specification(
                    PredefinedMetricSpecification::builder()
                        .predefined_metric_type(MetricType::EcsServiceAverageCpuUtilization)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_scaling_policies()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .expect("describe_scaling_policies should succeed");

    let policies = resp.scaling_policies();
    assert_eq!(policies.len(), 1);
    assert_eq!(policies[0].policy_name(), "my-policy");
}

#[tokio::test]
async fn test_delete_scaling_policy() {
    let client = make_client().await;

    client
        .register_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/my-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .min_capacity(1)
        .max_capacity(10)
        .send()
        .await
        .unwrap();

    client
        .put_scaling_policy()
        .policy_name("delete-me")
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/my-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .policy_type(PolicyType::TargetTrackingScaling)
        .target_tracking_scaling_policy_configuration(
            TargetTrackingScalingPolicyConfiguration::builder()
                .target_value(50.0)
                .predefined_metric_specification(
                    PredefinedMetricSpecification::builder()
                        .predefined_metric_type(MetricType::EcsServiceAverageCpuUtilization)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_scaling_policy()
        .policy_name("delete-me")
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/my-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .send()
        .await
        .expect("delete_scaling_policy should succeed");

    let resp = client
        .describe_scaling_policies()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.scaling_policies().len(), 0);
}

// ============================================================================
// Tests derived from AWS documentation: Application Auto Scaling
// ============================================================================

#[tokio::test]
async fn test_describe_scalable_targets_empty() {
    let client = make_client().await;

    let resp = client
        .describe_scalable_targets()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .expect("describe_scalable_targets on empty store should succeed");

    assert_eq!(resp.scalable_targets().len(), 0);
}

#[tokio::test]
async fn test_deregister_nonexistent_scalable_target() {
    let client = make_client().await;

    let result = client
        .deregister_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/nonexistent-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .send()
        .await;
    assert!(
        result.is_err(),
        "deregister of nonexistent target should return error"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_scaling_policy() {
    let client = make_client().await;

    let result = client
        .delete_scaling_policy()
        .policy_name("nonexistent-policy")
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/my-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .send()
        .await;
    assert!(
        result.is_err(),
        "delete of nonexistent policy should return error"
    );
}

#[tokio::test]
async fn test_describe_scaling_policies_empty() {
    let client = make_client().await;

    let resp = client
        .describe_scaling_policies()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .expect("describe_scaling_policies on empty store should succeed");

    assert_eq!(resp.scaling_policies().len(), 0);
}

// ============================================================================
// Tests derived from AWS documentation: Application Auto Scaling
// ============================================================================

// Helper: register a standard ECS scalable target and return the resource_id used.
async fn register_test_target(
    client: &aws_sdk_applicationautoscaling::Client,
    resource_id: &str,
    min_capacity: i32,
    max_capacity: i32,
) {
    client
        .register_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id(resource_id)
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .min_capacity(min_capacity)
        .max_capacity(max_capacity)
        .send()
        .await
        .expect("register_scalable_target should succeed");
}

// Helper: put a target-tracking policy on the given resource.
async fn put_test_policy(
    client: &aws_sdk_applicationautoscaling::Client,
    policy_name: &str,
    resource_id: &str,
) -> String {
    let resp = client
        .put_scaling_policy()
        .policy_name(policy_name)
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id(resource_id)
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .policy_type(PolicyType::TargetTrackingScaling)
        .target_tracking_scaling_policy_configuration(
            TargetTrackingScalingPolicyConfiguration::builder()
                .target_value(50.0)
                .predefined_metric_specification(
                    PredefinedMetricSpecification::builder()
                        .predefined_metric_type(MetricType::EcsServiceAverageCpuUtilization)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_scaling_policy should succeed");
    resp.policy_arn().to_string()
}

/// RegisterScalableTarget must return a ScalableTargetARN in the response.
#[tokio::test]
async fn test_register_scalable_target_returns_arn() {
    let client = make_client().await;

    let resp = client
        .register_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/arn-test-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .min_capacity(1)
        .max_capacity(10)
        .send()
        .await
        .expect("register_scalable_target should succeed");

    assert!(
        resp.scalable_target_arn().is_some(),
        "ScalableTargetARN should be present in RegisterScalableTarget response"
    );
    assert!(
        !resp.scalable_target_arn().unwrap_or_default().is_empty(),
        "ScalableTargetARN should not be empty"
    );
}

/// RegisterScalableTarget acts as an upsert: calling it a second time on the same
/// (namespace, resource_id, dimension) key should update min/max capacity.
#[tokio::test]
async fn test_register_scalable_target_update() {
    let client = make_client().await;
    let resource_id = "service/default/update-test-service";

    register_test_target(&client, resource_id, 1, 5).await;

    // Update min and max capacity by re-registering
    client
        .register_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id(resource_id)
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .min_capacity(2)
        .max_capacity(20)
        .send()
        .await
        .expect("second register_scalable_target (update) should succeed");

    let resp = client
        .describe_scalable_targets()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .unwrap();

    let targets = resp.scalable_targets();
    assert_eq!(targets.len(), 1);
    assert_eq!(
        targets[0].min_capacity(),
        2,
        "min_capacity should be updated to 2"
    );
    assert_eq!(
        targets[0].max_capacity(),
        20,
        "max_capacity should be updated to 20"
    );
}

/// RegisterScalableTarget with SuspendedState - the suspended flags should be reflected
/// in DescribeScalableTargets.
#[tokio::test]
async fn test_register_scalable_target_with_suspended_state() {
    let client = make_client().await;

    client
        .register_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/suspended-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .min_capacity(1)
        .max_capacity(10)
        .suspended_state(
            SdkSuspendedState::builder()
                .dynamic_scaling_in_suspended(true)
                .dynamic_scaling_out_suspended(false)
                .scheduled_scaling_suspended(true)
                .build(),
        )
        .send()
        .await
        .expect("register_scalable_target with suspended_state should succeed");

    let resp = client
        .describe_scalable_targets()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .unwrap();

    let targets = resp.scalable_targets();
    assert_eq!(targets.len(), 1);
    let ss = targets[0]
        .suspended_state()
        .expect("suspended_state should be present");
    assert_eq!(
        ss.dynamic_scaling_in_suspended(),
        Some(true),
        "dynamic_scaling_in_suspended should be true"
    );
    assert_eq!(
        ss.dynamic_scaling_out_suspended(),
        Some(false),
        "dynamic_scaling_out_suspended should be false"
    );
    assert_eq!(
        ss.scheduled_scaling_suspended(),
        Some(true),
        "scheduled_scaling_suspended should be true"
    );
}

/// DescribeScalableTargets with ResourceIds filter returns only the specified targets.
#[tokio::test]
async fn test_describe_scalable_targets_filter_by_resource_id() {
    let client = make_client().await;

    register_test_target(&client, "service/default/svc-alpha", 1, 10).await;
    register_test_target(&client, "service/default/svc-beta", 1, 10).await;

    let resp = client
        .describe_scalable_targets()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_ids("service/default/svc-alpha")
        .send()
        .await
        .expect("describe_scalable_targets with resource_ids filter should succeed");

    let targets = resp.scalable_targets();
    assert_eq!(
        targets.len(),
        1,
        "Only the filtered resource should be returned"
    );
    assert_eq!(targets[0].resource_id(), "service/default/svc-alpha");
}

/// DescribeScalableTargets with ScalableDimension filter returns only matching targets.
#[tokio::test]
async fn test_describe_scalable_targets_filter_by_dimension() {
    let client = make_client().await;

    register_test_target(&client, "service/default/dim-svc", 1, 10).await;

    // Request with the correct dimension
    let resp_match = client
        .describe_scalable_targets()
        .service_namespace(ServiceNamespace::Ecs)
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .send()
        .await
        .unwrap();
    assert_eq!(resp_match.scalable_targets().len(), 1);

    // Request with a different dimension should return no results
    let resp_no_match = client
        .describe_scalable_targets()
        .service_namespace(ServiceNamespace::Ecs)
        .scalable_dimension(ScalableDimension::DynamoDbTableReadCapacityUnits)
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp_no_match.scalable_targets().len(),
        0,
        "No ECS targets should match a DynamoDB dimension"
    );
}

/// DeregisterScalableTarget should also remove all scaling policies on that target.
#[tokio::test]
async fn test_deregister_removes_associated_policies() {
    let client = make_client().await;
    let resource_id = "service/default/cleanup-svc";

    register_test_target(&client, resource_id, 1, 10).await;
    put_test_policy(&client, "cleanup-policy", resource_id).await;

    // Verify the policy exists
    let before = client
        .describe_scaling_policies()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .unwrap();
    assert_eq!(before.scaling_policies().len(), 1);

    // Deregister the target
    client
        .deregister_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id(resource_id)
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .send()
        .await
        .expect("deregister_scalable_target should succeed");

    // Policy should have been removed
    let after = client
        .describe_scaling_policies()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .unwrap();
    assert_eq!(
        after.scaling_policies().len(),
        0,
        "Policies should be removed when the scalable target is deregistered"
    );
}

/// PutScalingPolicy with PolicyType=StepScaling should succeed and return an ARN.
#[tokio::test]
async fn test_put_scaling_policy_with_step_scaling() {
    let client = make_client().await;
    let resource_id = "service/default/step-scaling-svc";

    register_test_target(&client, resource_id, 1, 10).await;

    let resp = client
        .put_scaling_policy()
        .policy_name("step-policy")
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id(resource_id)
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .policy_type(PolicyType::StepScaling)
        .step_scaling_policy_configuration(
            StepScalingPolicyConfiguration::builder()
                .adjustment_type(AdjustmentType::ChangeInCapacity)
                .cooldown(60)
                .step_adjustments(
                    StepAdjustment::builder()
                        .scaling_adjustment(2)
                        .metric_interval_lower_bound(0.0)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("put_scaling_policy with step scaling should succeed");

    assert!(
        !resp.policy_arn().is_empty(),
        "Policy ARN should be non-empty"
    );

    // Verify policy type is stored correctly
    let desc = client
        .describe_scaling_policies()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .unwrap();
    let policies = desc.scaling_policies();
    assert_eq!(policies.len(), 1);
    assert_eq!(
        policies[0].policy_type().as_str(),
        "StepScaling",
        "Policy type should be StepScaling"
    );
}

/// DescribeScalingPolicies filtered by resource_id returns only policies for that resource.
#[tokio::test]
async fn test_describe_scaling_policies_filter_by_resource_id() {
    let client = make_client().await;

    register_test_target(&client, "service/default/res-a", 1, 10).await;
    register_test_target(&client, "service/default/res-b", 1, 10).await;
    put_test_policy(&client, "policy-a", "service/default/res-a").await;
    put_test_policy(&client, "policy-b", "service/default/res-b").await;

    let resp = client
        .describe_scaling_policies()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/res-a")
        .send()
        .await
        .expect("describe_scaling_policies with resource_id filter should succeed");

    let policies = resp.scaling_policies();
    assert_eq!(
        policies.len(),
        1,
        "Only policies for res-a should be returned"
    );
    assert_eq!(policies[0].policy_name(), "policy-a");
}

/// DescribeScalingPolicies filtered by policy_names returns only matching policies.
#[tokio::test]
async fn test_describe_scaling_policies_filter_by_policy_names() {
    let client = make_client().await;
    let resource_id = "service/default/filter-by-name-svc";

    register_test_target(&client, resource_id, 1, 10).await;
    put_test_policy(&client, "named-policy-one", resource_id).await;
    put_test_policy(&client, "named-policy-two", resource_id).await;

    let resp = client
        .describe_scaling_policies()
        .service_namespace(ServiceNamespace::Ecs)
        .policy_names("named-policy-one")
        .send()
        .await
        .expect("describe_scaling_policies with policy_names filter should succeed");

    let policies = resp.scaling_policies();
    assert_eq!(
        policies.len(),
        1,
        "Only the named policy should be returned"
    );
    assert_eq!(policies[0].policy_name(), "named-policy-one");
}

/// PutScalingPolicy on a non-existent scalable target should return ObjectNotFoundException.
#[tokio::test]
async fn test_put_scaling_policy_no_target_error() {
    let client = make_client().await;

    let err = client
        .put_scaling_policy()
        .policy_name("orphan-policy")
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/nonexistent-svc")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .policy_type(PolicyType::TargetTrackingScaling)
        .target_tracking_scaling_policy_configuration(
            TargetTrackingScalingPolicyConfiguration::builder()
                .target_value(50.0)
                .predefined_metric_specification(
                    PredefinedMetricSpecification::builder()
                        .predefined_metric_type(MetricType::EcsServiceAverageCpuUtilization)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ObjectNotFoundException"),
        "Expected ObjectNotFoundException, got: {err_str}"
    );
}

/// PutScalingPolicy is an upsert: calling it twice with the same name should not
/// create duplicate policies.
#[tokio::test]
async fn test_put_scaling_policy_upsert() {
    let client = make_client().await;
    let resource_id = "service/default/upsert-svc";

    register_test_target(&client, resource_id, 1, 10).await;

    put_test_policy(&client, "upsert-policy", resource_id).await;
    put_test_policy(&client, "upsert-policy", resource_id).await;

    let resp = client
        .describe_scaling_policies()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.scaling_policies().len(),
        1,
        "Putting the same policy name twice should result in exactly 1 policy (upsert)"
    );
}

// ============================================================================
// Scheduled Actions tests
// ============================================================================

#[tokio::test]
async fn test_put_scheduled_action() {
    let client = make_client().await;
    let resource_id = "service/default/scheduled-svc";

    register_test_target(&client, resource_id, 1, 10).await;

    client
        .put_scheduled_action()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id(resource_id)
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .scheduled_action_name("my-scheduled-action")
        .schedule("cron(0 12 * * ? *)")
        .send()
        .await
        .expect("put_scheduled_action should succeed");
}

#[tokio::test]
async fn test_describe_scheduled_actions() {
    let client = make_client().await;
    let resource_id = "service/default/scheduled-svc-desc";

    register_test_target(&client, resource_id, 1, 10).await;

    client
        .put_scheduled_action()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id(resource_id)
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .scheduled_action_name("desc-scheduled-action")
        .schedule("rate(1 day)")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_scheduled_actions()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .expect("describe_scheduled_actions should succeed");

    let actions = resp.scheduled_actions();
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].scheduled_action_name(), "desc-scheduled-action");
}

#[tokio::test]
async fn test_delete_scheduled_action() {
    let client = make_client().await;
    let resource_id = "service/default/scheduled-svc-del";

    register_test_target(&client, resource_id, 1, 10).await;

    client
        .put_scheduled_action()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id(resource_id)
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .scheduled_action_name("delete-me-scheduled")
        .send()
        .await
        .unwrap();

    client
        .delete_scheduled_action()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id(resource_id)
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .scheduled_action_name("delete-me-scheduled")
        .send()
        .await
        .expect("delete_scheduled_action should succeed");

    let resp = client
        .describe_scheduled_actions()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.scheduled_actions().len(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_scheduled_action() {
    let client = make_client().await;

    let result = client
        .delete_scheduled_action()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/my-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .scheduled_action_name("nonexistent-action")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete of nonexistent scheduled action should return error"
    );
}

#[tokio::test]
async fn test_put_scheduled_action_no_target_error() {
    let client = make_client().await;

    let result = client
        .put_scheduled_action()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/no-such-service")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .scheduled_action_name("orphan-action")
        .send()
        .await;

    assert!(
        result.is_err(),
        "put_scheduled_action on nonexistent target should fail"
    );
}

// ============================================================================
// Tags tests
// ============================================================================

#[tokio::test]
async fn test_tag_and_list_tags_for_resource() {
    let client = make_client().await;
    let resource_id = "service/default/tag-svc";

    let register_resp = client
        .register_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id(resource_id)
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .min_capacity(1)
        .max_capacity(10)
        .send()
        .await
        .unwrap();

    let resource_arn = register_resp.scalable_target_arn().unwrap();

    client
        .tag_resource()
        .resource_arn(resource_arn)
        .tags("env", "test")
        .tags("project", "myproject")
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags().expect("tags should be present");
    assert_eq!(tags.get("env").map(|s: &String| s.as_str()), Some("test"));
    assert_eq!(
        tags.get("project").map(|s: &String| s.as_str()),
        Some("myproject")
    );
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;
    let resource_id = "service/default/untag-svc";

    let register_resp = client
        .register_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id(resource_id)
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .min_capacity(1)
        .max_capacity(10)
        .send()
        .await
        .unwrap();

    let resource_arn = register_resp.scalable_target_arn().unwrap();

    client
        .tag_resource()
        .resource_arn(resource_arn)
        .tags("key1", "val1")
        .tags("key2", "val2")
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(resource_arn)
        .tag_keys("key1")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(resource_arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags().expect("tags should be present");
    assert!(!tags.contains_key("key1"), "key1 should have been removed");
    assert_eq!(tags.get("key2").map(|s: &String| s.as_str()), Some("val2"));
}

#[tokio::test]
async fn test_list_tags_for_nonexistent_resource() {
    let client = make_client().await;

    let result = client
        .list_tags_for_resource()
        .resource_arn(
            "arn:aws:application-autoscaling:us-east-1:123456789012:scalable-target/nonexistent",
        )
        .send()
        .await;

    assert!(
        result.is_err(),
        "list tags for nonexistent resource should fail"
    );
}

// ============================================================================
// Describe Scaling Activities tests
// ============================================================================

#[tokio::test]
async fn test_describe_scaling_activities_returns_empty() {
    let client = make_client().await;

    let resp = client
        .describe_scaling_activities()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .expect("describe_scaling_activities should succeed");

    // No real activities are simulated; should return empty list
    assert_eq!(resp.scaling_activities().len(), 0);
}

// ============================================================================
// GetPredictiveScalingForecast tests
// ============================================================================

#[tokio::test]
async fn test_get_predictive_scaling_forecast_returns_response() {
    use aws_sdk_applicationautoscaling::primitives::DateTime;

    let client = make_client().await;

    // Should return a valid (empty) forecast regardless of inputs
    let resp = client
        .get_predictive_scaling_forecast()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id("service/default/any-svc")
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .policy_name("any-policy")
        .start_time(DateTime::from_secs(1_700_000_000))
        .end_time(DateTime::from_secs(1_700_003_600))
        .send()
        .await
        .expect("get_predictive_scaling_forecast should succeed");

    // The capacity forecast should be present (empty timestamps/values)
    let cf = resp
        .capacity_forecast()
        .expect("capacity forecast should be present");
    assert_eq!(cf.timestamps().len(), 0);
    assert_eq!(cf.values().len(), 0);
}

/// Full lifecycle: register target -> put policy -> describe -> delete policy -> deregister target.
#[tokio::test]
async fn test_lifecycle_scalable_target_and_policy() {
    let client = make_client().await;
    let resource_id = "service/default/lifecycle-svc";
    let policy_name = "lifecycle-policy";

    // Register
    register_test_target(&client, resource_id, 1, 10).await;

    // Describe target
    let desc_targets = client
        .describe_scalable_targets()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_targets.scalable_targets().len(), 1);

    // Put policy
    let policy_arn = put_test_policy(&client, policy_name, resource_id).await;
    assert!(!policy_arn.is_empty(), "Policy ARN should be non-empty");

    // Describe policies
    let desc_policies = client
        .describe_scaling_policies()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_policies.scaling_policies().len(), 1);
    assert_eq!(
        desc_policies.scaling_policies()[0].policy_name(),
        policy_name
    );

    // Delete policy
    client
        .delete_scaling_policy()
        .policy_name(policy_name)
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id(resource_id)
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .send()
        .await
        .expect("delete_scaling_policy should succeed");

    // Verify policy gone
    let after_delete = client
        .describe_scaling_policies()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .unwrap();
    assert_eq!(after_delete.scaling_policies().len(), 0);

    // Deregister target
    client
        .deregister_scalable_target()
        .service_namespace(ServiceNamespace::Ecs)
        .resource_id(resource_id)
        .scalable_dimension(ScalableDimension::EcsServiceDesiredCount)
        .send()
        .await
        .expect("deregister_scalable_target should succeed");

    // Verify target gone
    let after_deregister = client
        .describe_scalable_targets()
        .service_namespace(ServiceNamespace::Ecs)
        .send()
        .await
        .unwrap();
    assert_eq!(after_deregister.scalable_targets().len(), 0);
}
