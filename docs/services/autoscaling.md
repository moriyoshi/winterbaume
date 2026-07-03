# winterbaume-autoscaling

Auto Scaling service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Auto Scaling |
| AWS model | `auto-scaling` |
| Protocol | awsQuery |
| winterbaume coverage | 52/66 operations (78.8%) |
| stubs (routed, returns empty/default) | 0/66 operations (0.0%) |
| moto coverage | 39/66 operations (59.1%) |
| floci coverage | 0/66 operations (0.0%) |
| kumo coverage | 0/66 operations (0.0%) |
| fakecloud coverage | 13/66 operations (19.7%) |
| Coverage report date | 2026-07-03 |

## Server-mode usage

Install `winterbaume-server` from crates.io or run it from a workspace checkout, then point the AWS CLI at it:

```sh
# Installed binary ( from crates.io ):
cargo install winterbaume-server
winterbaume-server --host 127.0.0.1 --port 5555

# Or, from a workspace checkout:
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws autoscaling describe-auto-scaling-groups
```

## Current Network Resource Stub Semantics

Auto Scaling stores network placement inputs as local scalar or list fields.

- Launch configurations keep `security_groups` exactly as supplied by the request.
- Auto Scaling groups keep `vpc_zone_identifier` as an optional raw string; update operations replace that string when supplied.
- The service does not split the VPC zone identifier into subnets for validation, availability zone inference, or attachment tracking.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_autoscaling::config::BehaviorVersion;
use winterbaume_autoscaling::AutoScalingService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AutoScalingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_autoscaling::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_autoscaling::Client::new(&config);

    // Create a launch configuration
    client
        .create_launch_configuration()
        .launch_configuration_name("example-lc")
        .image_id("ami-12345678")
        .instance_type("t2.micro")
        .send()
        .await
        .expect("create_launch_configuration should succeed");

    // Create an Auto Scaling group
    client
        .create_auto_scaling_group()
        .auto_scaling_group_name("example-asg")
        .launch_configuration_name("example-lc")
        .min_size(1)
        .max_size(3)
        .desired_capacity(1)
        .availability_zones("us-east-1a")
        .send()
        .await
        .expect("create_auto_scaling_group should succeed");

    // Describe the Auto Scaling group
    let resp = client
        .describe_auto_scaling_groups()
        .auto_scaling_group_names("example-asg")
        .send()
        .await
        .expect("describe_auto_scaling_groups should succeed");

    for group in resp.auto_scaling_groups() {
        println!(
            "ASG: {:?} (min={:?}, max={:?}, desired={:?})",
            group.auto_scaling_group_name(),
            group.min_size(),
            group.max_size(),
            group.desired_capacity(),
        );
    }
}
```

## Implemented APIs (52)

- `AttachInstances`
- `AttachLoadBalancerTargetGroups`
- `AttachLoadBalancers`
- `BatchDeleteScheduledAction`
- `BatchPutScheduledUpdateGroupAction`
- `CreateAutoScalingGroup`
- `CreateLaunchConfiguration`
- `CreateOrUpdateTags`
- `DeleteAutoScalingGroup`
- `DeleteLaunchConfiguration`
- `DeleteLifecycleHook`
- `DeleteNotificationConfiguration`
- `DeletePolicy`
- `DeleteScheduledAction`
- `DeleteTags`
- `DeleteWarmPool`
- `DescribeAccountLimits`
- `DescribeAdjustmentTypes`
- `DescribeAutoScalingGroups`
- `DescribeAutoScalingInstances`
- `DescribeAutoScalingNotificationTypes`
- `DescribeLaunchConfigurations`
- `DescribeLifecycleHooks`
- `DescribeLoadBalancerTargetGroups`
- `DescribeLoadBalancers`
- `DescribeMetricCollectionTypes`
- `DescribeNotificationConfigurations`
- `DescribePolicies`
- `DescribeScalingActivities`
- `DescribeScalingProcessTypes`
- `DescribeScheduledActions`
- `DescribeTags`
- `DescribeTerminationPolicyTypes`
- `DescribeWarmPool`
- `DetachInstances`
- `DetachLoadBalancerTargetGroups`
- `DetachLoadBalancers`
- `DisableMetricsCollection`
- `EnableMetricsCollection`
- `ExecutePolicy`
- `PutLifecycleHook`
- `PutNotificationConfiguration`
- `PutScalingPolicy`
- `PutScheduledUpdateGroupAction`
- `PutWarmPool`
- `ResumeProcesses`
- `SetDesiredCapacity`
- `SetInstanceHealth`
- `SetInstanceProtection`
- `SuspendProcesses`
- `TerminateInstanceInAutoScalingGroup`
- `UpdateAutoScalingGroup`

<details><summary>Not yet implemented APIs (14)</summary>

- `AttachTrafficSources`
- `CancelInstanceRefresh`
- `CompleteLifecycleAction`
- `DescribeInstanceRefreshes`
- `DescribeLifecycleHookTypes`
- `DescribeTrafficSources`
- `DetachTrafficSources`
- `EnterStandby`
- `ExitStandby`
- `GetPredictiveScalingForecast`
- `LaunchInstances`
- `RecordLifecycleActionHeartbeat`
- `RollbackInstanceRefresh`
- `StartInstanceRefresh`

</details>
