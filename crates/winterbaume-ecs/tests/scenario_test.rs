//! End-to-end scenario tests for the ECS service mock.
//!
//! Each scenario chains 3+ operations and asserts on business outcomes rather
//! than per-API response shapes.

use aws_sdk_ecs::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ecs::EcsService;

async fn make_client() -> aws_sdk_ecs::Client {
    let mock = MockAws::builder().with_service(EcsService::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ecs::config::Region::new("us-east-1".to_string()))
        .load()
        .await;
    aws_sdk_ecs::Client::new(&config)
}

// ============================================================================
// Scenario: containerised service deployment lifecycle
//
// Models the typical Terraform aws_ecs_cluster + aws_ecs_task_definition +
// aws_ecs_service resource lifecycle:
//   create cluster → register task definition → create service →
//   describe service (assert ACTIVE) → update service (scale) →
//   delete service (assert INACTIVE) → deregister task definition →
//   delete cluster
// ============================================================================

/// Scenario: containerised service deployment lifecycle
///
/// Verifies that a Fargate service can be created, scaled, and torn down cleanly,
/// and that the cluster reflects the expected state at each step.
#[tokio::test]
async fn test_fargate_service_deployment_lifecycle() {
    let client = make_client().await;

    // Step 1: create the cluster
    let cluster_resp = client
        .create_cluster()
        .cluster_name("scenario-cluster")
        .send()
        .await
        .expect("create_cluster should succeed");
    let cluster = cluster_resp.cluster().expect("cluster in response");
    assert_eq!(cluster.status(), Some("ACTIVE"));

    // Step 2: register a Fargate task definition
    let td_resp = client
        .register_task_definition()
        .family("scenario-td")
        .requires_compatibilities(aws_sdk_ecs::types::Compatibility::Fargate)
        .cpu("256")
        .memory("512")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("app")
                .image("nginx:latest")
                .cpu(256)
                .memory(512)
                .essential(true)
                .build(),
        )
        .send()
        .await
        .expect("register_task_definition should succeed");
    let td = td_resp
        .task_definition()
        .expect("task_definition in response");
    assert_eq!(td.family(), Some("scenario-td"));
    assert_eq!(td.revision(), 1);
    let td_arn = td.task_definition_arn().expect("task_definition_arn");

    // Step 3: create the ECS service with 2 desired tasks
    let svc_resp = client
        .create_service()
        .cluster("scenario-cluster")
        .service_name("scenario-svc")
        .task_definition("scenario-td")
        .desired_count(2)
        .launch_type(aws_sdk_ecs::types::LaunchType::Fargate)
        .send()
        .await
        .expect("create_service should succeed");
    let svc = svc_resp.service().expect("service in response");
    assert_eq!(svc.status(), Some("ACTIVE"));
    assert_eq!(svc.desired_count(), 2);

    // Step 4: describe service — assert it is visible and ACTIVE
    let desc_resp = client
        .describe_services()
        .cluster("scenario-cluster")
        .services("scenario-svc")
        .send()
        .await
        .expect("describe_services should succeed");
    let described_svc = desc_resp
        .services()
        .first()
        .expect("service in describe response");
    assert_eq!(described_svc.status(), Some("ACTIVE"));
    assert!(desc_resp.failures().is_empty(), "should have no failures");

    // Step 5: scale the service to 4 tasks
    let update_resp = client
        .update_service()
        .cluster("scenario-cluster")
        .service("scenario-svc")
        .desired_count(4)
        .send()
        .await
        .expect("update_service should succeed");
    let updated_svc = update_resp.service().expect("service in update response");
    assert_eq!(updated_svc.desired_count(), 4);

    // Step 6: delete the service
    let del_svc_resp = client
        .delete_service()
        .cluster("scenario-cluster")
        .service("scenario-svc")
        .send()
        .await
        .expect("delete_service should succeed");
    let del_svc = del_svc_resp.service().expect("service in delete response");
    assert_eq!(del_svc.status(), Some("INACTIVE"));

    // Step 7: deregister the task definition
    let dereg_resp = client
        .deregister_task_definition()
        .task_definition(td_arn)
        .send()
        .await
        .expect("deregister_task_definition should succeed");
    let deregistered_td = dereg_resp
        .task_definition()
        .expect("task_definition in deregister response");
    assert_eq!(
        deregistered_td.status(),
        Some(aws_sdk_ecs::types::TaskDefinitionStatus::Inactive).as_ref()
    );

    // Step 8: delete the cluster
    let del_cluster_resp = client
        .delete_cluster()
        .cluster("scenario-cluster")
        .send()
        .await
        .expect("delete_cluster should succeed");
    let del_cluster = del_cluster_resp
        .cluster()
        .expect("cluster in delete response");
    assert_eq!(del_cluster.status(), Some("INACTIVE"));
}

// ============================================================================
// Scenario: task execution and lifecycle
//
// Models the run-task → inspect → stop workflow used by ECS run-task callers:
//   create cluster → register task definition → run task →
//   list tasks (assert task present) → describe tasks (assert RUNNING) →
//   stop task → describe tasks (assert STOPPED) →
//   list tasks with RUNNING filter (assert empty)
// ============================================================================

/// Scenario: task execution and lifecycle
///
/// Verifies that tasks can be started, inspected, and stopped, and that the
/// list/describe filters correctly reflect the task's current desired status.
#[tokio::test]
async fn test_task_execution_lifecycle() {
    let client = make_client().await;

    // Step 1: create cluster and register task definition
    client
        .create_cluster()
        .cluster_name("task-cluster")
        .send()
        .await
        .expect("create_cluster should succeed");

    client
        .register_task_definition()
        .family("task-family")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("worker")
                .image("busybox:latest")
                .cpu(256)
                .memory(256)
                .essential(true)
                .build(),
        )
        .send()
        .await
        .expect("register_task_definition should succeed");

    // Step 2: run two tasks
    let run_resp = client
        .run_task()
        .cluster("task-cluster")
        .task_definition("task-family")
        .count(2)
        .launch_type(aws_sdk_ecs::types::LaunchType::Ec2)
        .send()
        .await
        .expect("run_task should succeed");
    let tasks = run_resp.tasks();
    assert_eq!(tasks.len(), 2, "should have started 2 tasks");
    let task_arns: Vec<&str> = tasks.iter().map(|t| t.task_arn().unwrap_or("")).collect();

    // Step 3: list tasks — both should appear
    let list_resp = client
        .list_tasks()
        .cluster("task-cluster")
        .send()
        .await
        .expect("list_tasks should succeed");
    assert_eq!(
        list_resp.task_arns().len(),
        2,
        "both tasks should be listed"
    );

    // Step 4: describe tasks — both should be RUNNING
    let desc_resp = client
        .describe_tasks()
        .cluster("task-cluster")
        .set_tasks(Some(task_arns.iter().map(|s| s.to_string()).collect()))
        .send()
        .await
        .expect("describe_tasks should succeed");
    for t in desc_resp.tasks() {
        assert_eq!(t.last_status(), Some("RUNNING"));
        assert_eq!(t.desired_status(), Some("RUNNING"));
    }

    // Step 5: stop one task
    let task_arn_to_stop = task_arns[0].to_string();
    let stop_resp = client
        .stop_task()
        .cluster("task-cluster")
        .task(&task_arn_to_stop)
        .reason("scenario test stop")
        .send()
        .await
        .expect("stop_task should succeed");
    let stopped = stop_resp.task().expect("task in stop response");
    assert_eq!(stopped.desired_status(), Some("STOPPED"));

    // Step 6: list tasks filtering by RUNNING status — only one remains
    let running_resp = client
        .list_tasks()
        .cluster("task-cluster")
        .desired_status(aws_sdk_ecs::types::DesiredStatus::Running)
        .send()
        .await
        .expect("list_tasks with RUNNING filter should succeed");
    assert_eq!(
        running_resp.task_arns().len(),
        1,
        "only one task should remain RUNNING"
    );

    // Step 7: list tasks filtering by STOPPED — the stopped task appears
    let stopped_resp = client
        .list_tasks()
        .cluster("task-cluster")
        .desired_status(aws_sdk_ecs::types::DesiredStatus::Stopped)
        .send()
        .await
        .expect("list_tasks with STOPPED filter should succeed");
    assert_eq!(
        stopped_resp.task_arns().len(),
        1,
        "one task should be STOPPED"
    );
}

// ============================================================================
// Scenario: capacity provider and cluster configuration
//
// Verifies that capacity providers can be created, attached to a cluster,
// and queried — and that the cluster reflects the attached provider.
//   create capacity provider → create cluster →
//   put cluster capacity providers → describe cluster (assert provider attached) →
//   describe capacity providers (assert ACTIVE) →
//   delete capacity provider (assert INACTIVE)
// ============================================================================

/// Scenario: capacity provider and cluster configuration
///
/// Verifies that a capacity provider can be created and linked to a cluster
/// via PutClusterCapacityProviders, then torn down cleanly.
#[tokio::test]
async fn test_capacity_provider_cluster_configuration() {
    let client = make_client().await;

    // Step 1: create a capacity provider
    let cp_resp = client
        .create_capacity_provider()
        .name("test-cp")
        .auto_scaling_group_provider(
            aws_sdk_ecs::types::AutoScalingGroupProvider::builder()
                .auto_scaling_group_arn(
                    "arn:aws:autoscaling:us-east-1:123456789012:autoScalingGroup:asg-id",
                )
                .build()
                .expect("asg provider build"),
        )
        .send()
        .await
        .expect("create_capacity_provider should succeed");
    let cp = cp_resp
        .capacity_provider()
        .expect("capacity_provider in response");
    assert_eq!(
        cp.status(),
        Some(aws_sdk_ecs::types::CapacityProviderStatus::Active).as_ref()
    );
    assert_eq!(cp.name(), Some("test-cp"));

    // Step 2: create cluster
    client
        .create_cluster()
        .cluster_name("cp-cluster")
        .send()
        .await
        .expect("create_cluster should succeed");

    // Step 3: attach capacity provider to the cluster
    client
        .put_cluster_capacity_providers()
        .cluster("cp-cluster")
        .capacity_providers("test-cp")
        .send()
        .await
        .expect("put_cluster_capacity_providers should succeed");

    // Step 4: describe the cluster — capacity provider should be present
    let desc_cluster_resp = client
        .describe_clusters()
        .clusters("cp-cluster")
        .send()
        .await
        .expect("describe_clusters should succeed");
    let described_cluster = desc_cluster_resp
        .clusters()
        .first()
        .expect("cluster in response");
    assert!(
        described_cluster
            .capacity_providers()
            .contains(&"test-cp".to_string()),
        "cluster should have capacity provider attached"
    );

    // Step 5: describe the capacity provider — should still be ACTIVE
    let desc_cp_resp = client
        .describe_capacity_providers()
        .capacity_providers("test-cp")
        .send()
        .await
        .expect("describe_capacity_providers should succeed");
    let described_cp = desc_cp_resp
        .capacity_providers()
        .first()
        .expect("cp in response");
    assert_eq!(
        described_cp.status(),
        Some(aws_sdk_ecs::types::CapacityProviderStatus::Active).as_ref()
    );

    // Step 6: delete the capacity provider
    let del_cp_resp = client
        .delete_capacity_provider()
        .capacity_provider("test-cp")
        .send()
        .await
        .expect("delete_capacity_provider should succeed");
    let del_cp = del_cp_resp
        .capacity_provider()
        .expect("capacity_provider in delete response");
    assert_eq!(
        del_cp.status(),
        Some(aws_sdk_ecs::types::CapacityProviderStatus::Inactive).as_ref()
    );
}
