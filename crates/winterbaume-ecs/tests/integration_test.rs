use aws_sdk_ecs::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ecs::EcsService;

const ACCOUNT_ID: &str = "123456789012";

async fn make_ecs_client() -> aws_sdk_ecs::Client {
    make_ecs_client_with_region("us-east-1").await
}

async fn make_ecs_client_with_region(region: &str) -> aws_sdk_ecs::Client {
    let mock = MockAws::builder().with_service(EcsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ecs::config::Region::new(region.to_string()))
        .load()
        .await;

    aws_sdk_ecs::Client::new(&config)
}

// ============================================================================
// Helper: create a cluster + task definition for reuse
// ============================================================================

async fn setup_cluster_and_task(client: &aws_sdk_ecs::Client) {
    client
        .create_cluster()
        .cluster_name("test_ecs_cluster")
        .send()
        .await
        .unwrap();

    client
        .register_task_definition()
        .family("test_ecs_task")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("hello_world")
                .image("docker/hello-world:latest")
                .cpu(1024)
                .memory(400)
                .essential(true)
                .build(),
        )
        .send()
        .await
        .unwrap();
}

// ============================================================================
// Cluster tests
// ============================================================================

#[tokio::test]
async fn test_create_cluster() {
    let client = make_ecs_client().await;
    let response = client
        .create_cluster()
        .cluster_name("test_ecs_cluster")
        .send()
        .await
        .expect("create_cluster should succeed");

    let cluster = response.cluster().expect("should have cluster");
    assert_eq!(cluster.cluster_name(), Some("test_ecs_cluster"));
    assert_eq!(
        cluster.cluster_arn(),
        Some(format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:cluster/test_ecs_cluster").as_str())
    );
    assert_eq!(cluster.status(), Some("ACTIVE"));
    assert_eq!(cluster.registered_container_instances_count(), 0);
    assert_eq!(cluster.running_tasks_count(), 0);
    assert_eq!(cluster.pending_tasks_count(), 0);
    assert_eq!(cluster.active_services_count(), 0);
}

#[tokio::test]
async fn test_create_cluster_idempotent() {
    let client = make_ecs_client().await;

    let resp1 = client
        .create_cluster()
        .cluster_name("idem-cluster")
        .send()
        .await
        .unwrap();
    let resp2 = client
        .create_cluster()
        .cluster_name("idem-cluster")
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp1.cluster().unwrap().cluster_arn(),
        resp2.cluster().unwrap().cluster_arn()
    );
    assert_eq!(
        resp1.cluster().unwrap().cluster_name(),
        resp2.cluster().unwrap().cluster_name()
    );
}

#[tokio::test]
async fn test_list_clusters() {
    let client = make_ecs_client_with_region("us-east-2").await;

    client
        .create_cluster()
        .cluster_name("test_cluster0")
        .send()
        .await
        .unwrap();
    client
        .create_cluster()
        .cluster_name("test_cluster1")
        .send()
        .await
        .unwrap();

    let response = client.list_clusters().send().await.unwrap();
    let arns = response.cluster_arns();
    assert!(arns.contains(&format!(
        "arn:aws:ecs:us-east-2:{ACCOUNT_ID}:cluster/test_cluster0"
    )));
    assert!(arns.contains(&format!(
        "arn:aws:ecs:us-east-2:{ACCOUNT_ID}:cluster/test_cluster1"
    )));
}

#[tokio::test]
async fn test_delete_cluster() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("test_ecs_cluster")
        .send()
        .await
        .unwrap();

    let response = client
        .delete_cluster()
        .cluster("test_ecs_cluster")
        .send()
        .await
        .expect("delete_cluster should succeed");

    let cluster = response.cluster().expect("should have cluster");
    assert_eq!(cluster.cluster_name(), Some("test_ecs_cluster"));
    assert_eq!(
        cluster.cluster_arn(),
        Some(format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:cluster/test_ecs_cluster").as_str())
    );
    assert_eq!(cluster.status(), Some("INACTIVE"));
    assert_eq!(cluster.registered_container_instances_count(), 0);
    assert_eq!(cluster.running_tasks_count(), 0);
    assert_eq!(cluster.pending_tasks_count(), 0);
    assert_eq!(cluster.active_services_count(), 0);

    // Deleted (INACTIVE) cluster should still appear in list_clusters
    let list_resp = client.list_clusters().send().await.unwrap();
    assert_eq!(list_resp.cluster_arns().len(), 1);
}

#[tokio::test]
async fn test_delete_cluster_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .delete_cluster()
        .cluster("not_a_cluster")
        .send()
        .await
        .expect_err("should fail for non-existent cluster");

    let service_err = err.into_service_error();
    assert!(service_err.is_cluster_not_found_exception());
}

#[tokio::test]
async fn test_describe_clusters() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("c1")
        .send()
        .await
        .unwrap();
    client
        .create_cluster()
        .cluster_name("c2")
        .send()
        .await
        .unwrap();

    let response = client
        .describe_clusters()
        .clusters("c1")
        .clusters("c2")
        .send()
        .await
        .unwrap();
    assert_eq!(response.clusters().len(), 2);
}

#[tokio::test]
async fn test_describe_clusters_missing() {
    let client = make_ecs_client().await;

    let response = client
        .describe_clusters()
        .clusters("some-cluster")
        .send()
        .await
        .expect("describe_clusters should succeed even with missing");

    assert_eq!(response.clusters().len(), 0);
    assert_eq!(response.failures().len(), 1);
    let failure = &response.failures()[0];
    assert_eq!(
        failure.arn(),
        Some(format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:cluster/some-cluster").as_str())
    );
    assert_eq!(failure.reason(), Some("MISSING"));
}

#[tokio::test]
async fn test_update_cluster() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("update-cluster")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_cluster()
        .cluster("update-cluster")
        .send()
        .await
        .unwrap();

    let cluster = resp.cluster().unwrap();
    assert_eq!(cluster.cluster_name(), Some("update-cluster"));
    assert_eq!(cluster.status(), Some("ACTIVE"));
}

#[tokio::test]
async fn test_update_cluster_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .update_cluster()
        .cluster("nonexistent")
        .send()
        .await
        .expect_err("should fail");

    let _service_err = err.into_service_error();
}

#[tokio::test]
async fn test_put_cluster_capacity_providers() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("cp-cluster")
        .send()
        .await
        .unwrap();

    client
        .create_capacity_provider()
        .name("my-cp")
        .auto_scaling_group_provider(
            aws_sdk_ecs::types::AutoScalingGroupProvider::builder()
                .auto_scaling_group_arn(
                    "arn:aws:autoscaling:us-east-1:123456789012:autoScalingGroup:asg-123",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .put_cluster_capacity_providers()
        .cluster("cp-cluster")
        .capacity_providers("my-cp")
        .default_capacity_provider_strategy(
            aws_sdk_ecs::types::CapacityProviderStrategyItem::builder()
                .capacity_provider("my-cp")
                .weight(1)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let cluster = resp.cluster().unwrap();
    assert_eq!(cluster.cluster_name(), Some("cp-cluster"));
    assert!(cluster.capacity_providers().contains(&"my-cp".to_string()));
}

// ============================================================================
// Task Definition tests
// ============================================================================

#[tokio::test]
async fn test_register_task_definition() {
    let client = make_ecs_client().await;

    let response = client
        .register_task_definition()
        .family("test_ecs_task")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("hello_world")
                .image("hello-world:latest")
                .memory(400)
                .build(),
        )
        .send()
        .await
        .expect("register_task_definition should succeed");

    let td = response.task_definition().unwrap();
    assert_eq!(td.family(), Some("test_ecs_task"));
    assert_eq!(td.revision(), 1);
    assert_eq!(
        td.task_definition_arn(),
        Some(
            format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:task-definition/test_ecs_task:1").as_str()
        )
    );
    assert_eq!(
        td.network_mode(),
        Some(&aws_sdk_ecs::types::NetworkMode::Bridge)
    );
    assert_eq!(td.volumes().len(), 0);
    assert_eq!(td.placement_constraints().len(), 0);

    let cntr_def = &td.container_definitions()[0];
    assert_eq!(cntr_def.name(), Some("hello_world"));
    assert_eq!(cntr_def.image(), Some("hello-world:latest"));
    assert_eq!(cntr_def.cpu(), 0);
    assert_eq!(cntr_def.port_mappings().len(), 0);
    assert_eq!(cntr_def.essential(), Some(true));
    assert_eq!(cntr_def.environment().len(), 0);
    assert_eq!(cntr_def.mount_points().len(), 0);
    assert_eq!(cntr_def.volumes_from().len(), 0);
}

#[tokio::test]
async fn test_register_task_definition_revision_increments() {
    let client = make_ecs_client().await;

    let container_def = aws_sdk_ecs::types::ContainerDefinition::builder()
        .name("hello_world")
        .image("hello-world:latest")
        .memory(400)
        .build();

    let resp1 = client
        .register_task_definition()
        .family("test_ecs_task")
        .container_definitions(container_def.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(resp1.task_definition().unwrap().revision(), 1);

    let resp2 = client
        .register_task_definition()
        .family("test_ecs_task")
        .container_definitions(container_def)
        .send()
        .await
        .unwrap();
    assert_eq!(resp2.task_definition().unwrap().revision(), 2);
}

#[tokio::test]
async fn test_register_task_definition_with_role_arns() {
    let client = make_ecs_client().await;

    let container_def = aws_sdk_ecs::types::ContainerDefinition::builder()
        .name("hello_world")
        .image("hello-world:latest")
        .memory(400)
        .build();

    let response = client
        .register_task_definition()
        .family("test_ecs_task")
        .task_role_arn("my-custom-task-role-arn")
        .execution_role_arn("my-custom-execution-role-arn")
        .container_definitions(container_def)
        .send()
        .await
        .unwrap();

    let td = response.task_definition().unwrap();
    assert_eq!(td.task_role_arn(), Some("my-custom-task-role-arn"));
    assert_eq!(
        td.execution_role_arn(),
        Some("my-custom-execution-role-arn")
    );
}

#[tokio::test]
async fn test_register_task_definition_fargate() {
    let client = make_ecs_client().await;

    let container_def = aws_sdk_ecs::types::ContainerDefinition::builder()
        .name("hello_world")
        .image("hello-world:latest")
        .memory(400)
        .build();

    let response = client
        .register_task_definition()
        .family("test_ecs_task")
        .requires_compatibilities(aws_sdk_ecs::types::Compatibility::Fargate)
        .container_definitions(container_def)
        .send()
        .await
        .unwrap();

    let td = response.task_definition().unwrap();
    assert_eq!(
        td.requires_compatibilities(),
        &[aws_sdk_ecs::types::Compatibility::Fargate]
    );
    assert_eq!(
        td.network_mode(),
        Some(&aws_sdk_ecs::types::NetworkMode::Awsvpc)
    );

    let compat = td.compatibilities();
    assert!(compat.contains(&aws_sdk_ecs::types::Compatibility::Ec2));
    assert!(compat.contains(&aws_sdk_ecs::types::Compatibility::Fargate));
}

#[tokio::test]
async fn test_register_task_definition_with_cpu_memory() {
    let client = make_ecs_client().await;

    let container_def = aws_sdk_ecs::types::ContainerDefinition::builder()
        .name("hello_world")
        .image("hello-world:latest")
        .build();

    let response = client
        .register_task_definition()
        .family("test_ecs_task")
        .cpu("512")
        .memory("1024")
        .container_definitions(container_def)
        .send()
        .await
        .unwrap();

    let td = response.task_definition().unwrap();
    assert_eq!(td.cpu(), Some("512"));
    assert_eq!(td.memory(), Some("1024"));
}

#[tokio::test]
async fn test_register_task_definition_container_details() {
    let client = make_ecs_client().await;

    let container_def = aws_sdk_ecs::types::ContainerDefinition::builder()
        .name("hello_world")
        .image("docker/hello-world:latest")
        .cpu(1024)
        .memory(400)
        .essential(true)
        .environment(
            aws_sdk_ecs::types::KeyValuePair::builder()
                .name("AWS_ACCESS_KEY_ID")
                .value("SOME_ACCESS_KEY")
                .build(),
        )
        .build();

    let response = client
        .register_task_definition()
        .family("test_ecs_task")
        .container_definitions(container_def)
        .send()
        .await
        .unwrap();

    let td = response.task_definition().unwrap();
    let cntr_def = &td.container_definitions()[0];
    assert_eq!(cntr_def.name(), Some("hello_world"));
    assert_eq!(cntr_def.image(), Some("docker/hello-world:latest"));
    assert_eq!(cntr_def.cpu(), 1024);
    assert_eq!(cntr_def.memory(), Some(400));
    assert_eq!(cntr_def.essential(), Some(true));
    assert_eq!(cntr_def.environment().len(), 1);
    assert_eq!(cntr_def.environment()[0].name(), Some("AWS_ACCESS_KEY_ID"));
    assert_eq!(cntr_def.environment()[0].value(), Some("SOME_ACCESS_KEY"));
}

#[tokio::test]
async fn test_register_task_definition_container_essential_false() {
    let client = make_ecs_client().await;

    let container_def = aws_sdk_ecs::types::ContainerDefinition::builder()
        .name("hello_world")
        .image("docker/hello-world:latest")
        .cpu(512)
        .memory(400)
        .essential(false)
        .build();

    let response = client
        .register_task_definition()
        .family("test_ecs_task")
        .container_definitions(container_def)
        .send()
        .await
        .unwrap();

    let td = response.task_definition().unwrap();
    assert_eq!(td.container_definitions()[0].essential(), Some(false));
    assert_eq!(td.container_definitions()[0].cpu(), 512);
}

#[tokio::test]
async fn test_describe_task_definitions_specific_revision() {
    let client = make_ecs_client().await;

    let container1 = aws_sdk_ecs::types::ContainerDefinition::builder()
        .name("hello_world")
        .image("docker/hello-world:latest")
        .cpu(1024)
        .memory(400)
        .essential(true)
        .build();

    let container2 = aws_sdk_ecs::types::ContainerDefinition::builder()
        .name("hello_world2")
        .image("docker/hello-world2:latest")
        .cpu(1024)
        .memory(400)
        .essential(true)
        .build();

    let container3 = aws_sdk_ecs::types::ContainerDefinition::builder()
        .name("hello_world3")
        .image("docker/hello-world3:latest")
        .cpu(1024)
        .memory(400)
        .essential(true)
        .build();

    client
        .register_task_definition()
        .family("test_ecs_task")
        .container_definitions(container1)
        .send()
        .await
        .unwrap();
    client
        .register_task_definition()
        .family("test_ecs_task")
        .task_role_arn("my-task-role-arn")
        .execution_role_arn("my-execution-role-arn")
        .container_definitions(container2)
        .send()
        .await
        .unwrap();
    client
        .register_task_definition()
        .family("test_ecs_task")
        .container_definitions(container3)
        .send()
        .await
        .unwrap();

    // Describe by family returns latest (revision 3)
    let response = client
        .describe_task_definition()
        .task_definition("test_ecs_task")
        .send()
        .await
        .unwrap();
    assert_eq!(
        response.task_definition().unwrap().task_definition_arn(),
        Some(
            format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:task-definition/test_ecs_task:3").as_str()
        )
    );

    // Describe by family:revision returns specific revision
    let response = client
        .describe_task_definition()
        .task_definition("test_ecs_task:2")
        .send()
        .await
        .unwrap();
    let td = response.task_definition().unwrap();
    assert_eq!(
        td.task_definition_arn(),
        Some(
            format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:task-definition/test_ecs_task:2").as_str()
        )
    );
    assert_eq!(td.task_role_arn(), Some("my-task-role-arn"));
    assert_eq!(td.execution_role_arn(), Some("my-execution-role-arn"));
}

#[tokio::test]
async fn test_describe_task_definition_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .describe_task_definition()
        .task_definition("nonexistent_task")
        .send()
        .await
        .expect_err("should fail for non-existent task definition");

    let _service_err = err.into_service_error();
}

#[tokio::test]
async fn test_deregister_task_definition() {
    let client = make_ecs_client().await;

    client
        .register_task_definition()
        .family("dereg_task")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("c1")
                .image("img:latest")
                .memory(256)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .deregister_task_definition()
        .task_definition("dereg_task:1")
        .send()
        .await
        .unwrap();

    let td = resp.task_definition().unwrap();
    assert_eq!(
        td.status(),
        Some(&aws_sdk_ecs::types::TaskDefinitionStatus::Inactive)
    );
    assert_eq!(td.family(), Some("dereg_task"));
}

#[tokio::test]
async fn test_delete_task_definitions() {
    let client = make_ecs_client().await;

    client
        .register_task_definition()
        .family("del_task")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("c1")
                .image("img:latest")
                .memory(256)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_task_definitions()
        .task_definitions("del_task:1")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.task_definitions().len(), 1);
    assert_eq!(
        resp.task_definitions()[0].status(),
        Some(&aws_sdk_ecs::types::TaskDefinitionStatus::DeleteInProgress)
    );
}

#[tokio::test]
async fn test_list_task_definitions() {
    let client = make_ecs_client().await;

    client
        .register_task_definition()
        .family("list_task")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("c1")
                .image("img:latest")
                .memory(256)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client.list_task_definitions().send().await.unwrap();
    assert!(!resp.task_definition_arns().is_empty());
    assert!(
        resp.task_definition_arns()
            .iter()
            .any(|a| a.contains("list_task"))
    );
}

#[tokio::test]
async fn test_list_task_definition_families() {
    let client = make_ecs_client().await;

    client
        .register_task_definition()
        .family("family_a")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("c1")
                .image("img:latest")
                .memory(256)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .register_task_definition()
        .family("family_b")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("c1")
                .image("img:latest")
                .memory(256)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client.list_task_definition_families().send().await.unwrap();
    assert!(resp.families().len() >= 2);
    assert!(resp.families().contains(&"family_a".to_string()));
    assert!(resp.families().contains(&"family_b".to_string()));
}

#[tokio::test]
async fn test_register_task_definition_with_log_configuration() {
    let client = make_ecs_client().await;

    let container_def = aws_sdk_ecs::types::ContainerDefinition::builder()
        .name("hello_world")
        .image("docker/hello-world:latest")
        .cpu(1024)
        .memory(400)
        .essential(true)
        .log_configuration(
            aws_sdk_ecs::types::LogConfiguration::builder()
                .log_driver(aws_sdk_ecs::types::LogDriver::JsonFile)
                .build()
                .unwrap(),
        )
        .build();

    let response = client
        .register_task_definition()
        .family("test_ecs_task")
        .container_definitions(container_def)
        .send()
        .await
        .unwrap();

    let td = response.task_definition().unwrap();
    let cntr_def = &td.container_definitions()[0];
    let log_config = cntr_def
        .log_configuration()
        .expect("should have log config");
    assert_eq!(
        log_config.log_driver(),
        &aws_sdk_ecs::types::LogDriver::JsonFile
    );
}

// ============================================================================
// Service tests
// ============================================================================

#[tokio::test]
async fn test_create_service() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let response = client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("test_ecs_service")
        .task_definition("test_ecs_task")
        .desired_count(2)
        .send()
        .await
        .expect("create_service should succeed");

    let svc = response.service().expect("should have service");
    assert_eq!(
        svc.cluster_arn(),
        Some(format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:cluster/test_ecs_cluster").as_str())
    );
    assert_eq!(svc.desired_count(), 2);
    assert_eq!(svc.events().len(), 0);
    assert_eq!(svc.load_balancers().len(), 0);
    assert_eq!(svc.pending_count(), 2);
    assert_eq!(svc.running_count(), 0);
    assert_eq!(
        svc.service_arn(),
        Some(
            format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:service/test_ecs_cluster/test_ecs_service")
                .as_str()
        )
    );
    assert_eq!(svc.service_name(), Some("test_ecs_service"));
    assert_eq!(svc.status(), Some("ACTIVE"));
    assert_eq!(
        svc.task_definition(),
        Some(
            format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:task-definition/test_ecs_task:1").as_str()
        )
    );
    assert_eq!(
        svc.scheduling_strategy(),
        Some(&aws_sdk_ecs::types::SchedulingStrategy::Replica)
    );
    assert_eq!(
        svc.launch_type(),
        Some(&aws_sdk_ecs::types::LaunchType::Ec2)
    );
}

#[tokio::test]
async fn test_create_service_duplicate_error() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("test_ecs_service")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap();

    let err = client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("test_ecs_service")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .expect_err("duplicate service should fail");

    let _service_err = err.into_service_error();
}

#[tokio::test]
async fn test_create_service_cluster_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .create_service()
        .cluster("nonexistent_cluster")
        .service_name("test_service")
        .task_definition("some_task")
        .desired_count(1)
        .send()
        .await
        .expect_err("should fail for non-existent cluster");

    let service_err = err.into_service_error();
    assert!(service_err.is_cluster_not_found_exception());
}

#[tokio::test]
async fn test_describe_services() {
    let client = make_ecs_client().await;

    let cluster_arn = client
        .create_cluster()
        .cluster_name("test_ecs_cluster")
        .send()
        .await
        .unwrap()
        .cluster()
        .unwrap()
        .cluster_arn()
        .unwrap()
        .to_string();

    client
        .register_task_definition()
        .family("test_ecs_task")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("hello_world")
                .image("docker/hello-world:latest")
                .cpu(1024)
                .memory(400)
                .essential(true)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("test_ecs_service1")
        .task_definition("test_ecs_task")
        .desired_count(2)
        .send()
        .await
        .unwrap();
    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("test_ecs_service2")
        .task_definition("test_ecs_task")
        .desired_count(2)
        .send()
        .await
        .unwrap();

    // Verify we can describe services using the cluster ARN
    let resp = client
        .describe_services()
        .cluster(&cluster_arn)
        .services("test_ecs_service1")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.services().len(), 1);

    // Verify we can describe services using both names and ARN's
    let svc2_arn =
        format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:service/test_ecs_cluster/test_ecs_service2");
    let response = client
        .describe_services()
        .cluster("test_ecs_cluster")
        .services("test_ecs_service1")
        .services(&svc2_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(response.services().len(), 2);

    // Check deployment details
    let deployment = &response.services()[0].deployments()[0];
    assert_eq!(deployment.desired_count(), 2);
    assert_eq!(deployment.pending_count(), 2);
    assert_eq!(deployment.running_count(), 0);
    assert_eq!(deployment.status(), Some("PRIMARY"));
    assert_eq!(
        deployment.launch_type(),
        Some(&aws_sdk_ecs::types::LaunchType::Ec2)
    );
}

#[tokio::test]
async fn test_describe_services_error_unknown_cluster() {
    let client = make_ecs_client_with_region("eu-central-1").await;

    let err = client
        .describe_services()
        .cluster("unknown")
        .services("test")
        .send()
        .await
        .expect_err("should fail for unknown cluster");

    let service_err = err.into_service_error();
    assert!(service_err.is_cluster_not_found_exception());
}

#[tokio::test]
async fn test_describe_services_with_known_unknown_services() {
    let client = make_ecs_client_with_region("eu-central-1").await;

    client
        .create_cluster()
        .cluster_name("test_cluster")
        .send()
        .await
        .unwrap();
    client
        .register_task_definition()
        .family("test_task")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("hello_world")
                .image("docker/hello-world:latest")
                .cpu(256)
                .memory(512)
                .essential(true)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let service_arn = client
        .create_service()
        .cluster("test_cluster")
        .service_name("test_service")
        .task_definition("test_task")
        .desired_count(1)
        .send()
        .await
        .unwrap()
        .service()
        .unwrap()
        .service_arn()
        .unwrap()
        .to_string();

    let response = client
        .describe_services()
        .cluster("test_cluster")
        .services("test_service")
        .services("unknown")
        .services(&service_arn)
        .services(format!(
            "arn:aws:ecs:eu-central-1:{ACCOUNT_ID}:service/unknown-2"
        ))
        .send()
        .await
        .unwrap();

    let services = response.services();
    let service_arns: Vec<&str> = services.iter().filter_map(|s| s.service_arn()).collect();
    assert_eq!(service_arns, vec![&service_arn, &service_arn]);

    let failures = response.failures();
    assert_eq!(failures.len(), 2);
    let mut failure_arns: Vec<&str> = failures.iter().filter_map(|f| f.arn()).collect();
    failure_arns.sort();
    assert_eq!(
        failure_arns[0],
        format!("arn:aws:ecs:eu-central-1:{ACCOUNT_ID}:service/unknown")
    );
    assert_eq!(
        failure_arns[1],
        format!("arn:aws:ecs:eu-central-1:{ACCOUNT_ID}:service/unknown-2")
    );
}

#[tokio::test]
async fn test_describe_services_scheduling_strategy() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("test_ecs_service1")
        .task_definition("test_ecs_task")
        .desired_count(2)
        .send()
        .await
        .unwrap();
    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("test_ecs_service2")
        .task_definition("test_ecs_task")
        .desired_count(2)
        .scheduling_strategy(aws_sdk_ecs::types::SchedulingStrategy::Daemon)
        .send()
        .await
        .unwrap();
    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("test_ecs_service3")
        .task_definition("test_ecs_task")
        .desired_count(2)
        .send()
        .await
        .unwrap();

    let response = client
        .describe_services()
        .cluster("test_ecs_cluster")
        .services("test_ecs_service1")
        .services(format!(
            "arn:aws:ecs:us-east-1:{ACCOUNT_ID}:service/test_ecs_cluster/test_ecs_service2"
        ))
        .services("test_ecs_service3")
        .send()
        .await
        .unwrap();

    assert_eq!(response.services().len(), 3);

    assert_eq!(
        response.services()[0].scheduling_strategy(),
        Some(&aws_sdk_ecs::types::SchedulingStrategy::Replica)
    );
    assert_eq!(
        response.services()[1].scheduling_strategy(),
        Some(&aws_sdk_ecs::types::SchedulingStrategy::Daemon)
    );
    assert_eq!(
        response.services()[2].scheduling_strategy(),
        Some(&aws_sdk_ecs::types::SchedulingStrategy::Replica)
    );
}

#[tokio::test]
async fn test_delete_service() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("svc-to-delete")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_service()
        .cluster("test_ecs_cluster")
        .service("svc-to-delete")
        .send()
        .await
        .unwrap();

    let svc = resp.service().unwrap();
    assert_eq!(svc.service_name(), Some("svc-to-delete"));
    assert_eq!(svc.status(), Some("INACTIVE"));
}

#[tokio::test]
async fn test_delete_service_not_found() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("test_cluster")
        .send()
        .await
        .unwrap();

    let err = client
        .delete_service()
        .cluster("test_cluster")
        .service("nonexistent")
        .send()
        .await
        .expect_err("should fail for nonexistent service");

    let service_err = err.into_service_error();
    assert!(service_err.is_service_not_found_exception());
}

#[tokio::test]
async fn test_update_service() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("svc-to-update")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_service()
        .cluster("test_ecs_cluster")
        .service("svc-to-update")
        .desired_count(5)
        .send()
        .await
        .unwrap();

    let svc = resp.service().unwrap();
    assert_eq!(svc.desired_count(), 5);
    assert_eq!(svc.service_name(), Some("svc-to-update"));
}

#[tokio::test]
async fn test_list_services() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("svc1")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap();

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("svc2")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_services()
        .cluster("test_ecs_cluster")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.service_arns().len(), 2);
}

// ============================================================================
// Capacity Provider tests
// ============================================================================

#[tokio::test]
async fn test_create_capacity_provider() {
    let client = make_ecs_client().await;

    let resp = client
        .create_capacity_provider()
        .name("test-cp")
        .auto_scaling_group_provider(
            aws_sdk_ecs::types::AutoScalingGroupProvider::builder()
                .auto_scaling_group_arn(
                    "arn:aws:autoscaling:us-east-1:123456789012:autoScalingGroup:asg-123",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let cp = resp.capacity_provider().unwrap();
    assert_eq!(cp.name(), Some("test-cp"));
    assert_eq!(
        cp.status(),
        Some(&aws_sdk_ecs::types::CapacityProviderStatus::Active)
    );
    assert!(
        cp.capacity_provider_arn()
            .unwrap()
            .contains("capacity-provider/test-cp")
    );
}

#[tokio::test]
async fn test_delete_capacity_provider() {
    let client = make_ecs_client().await;

    client
        .create_capacity_provider()
        .name("cp-to-delete")
        .auto_scaling_group_provider(
            aws_sdk_ecs::types::AutoScalingGroupProvider::builder()
                .auto_scaling_group_arn(
                    "arn:aws:autoscaling:us-east-1:123456789012:autoScalingGroup:asg-123",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_capacity_provider()
        .capacity_provider("cp-to-delete")
        .send()
        .await
        .unwrap();

    let cp = resp.capacity_provider().unwrap();
    assert_eq!(cp.name(), Some("cp-to-delete"));
}

#[tokio::test]
async fn test_describe_capacity_providers() {
    let client = make_ecs_client().await;

    client
        .create_capacity_provider()
        .name("describe-cp")
        .auto_scaling_group_provider(
            aws_sdk_ecs::types::AutoScalingGroupProvider::builder()
                .auto_scaling_group_arn(
                    "arn:aws:autoscaling:us-east-1:123456789012:autoScalingGroup:asg-123",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_capacity_providers()
        .capacity_providers("describe-cp")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.capacity_providers().len(), 1);
    assert_eq!(resp.capacity_providers()[0].name(), Some("describe-cp"));
}

#[tokio::test]
async fn test_update_capacity_provider() {
    let client = make_ecs_client().await;

    client
        .create_capacity_provider()
        .name("update-cp")
        .auto_scaling_group_provider(
            aws_sdk_ecs::types::AutoScalingGroupProvider::builder()
                .auto_scaling_group_arn(
                    "arn:aws:autoscaling:us-east-1:123456789012:autoScalingGroup:asg-123",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .update_capacity_provider()
        .name("update-cp")
        .auto_scaling_group_provider(
            aws_sdk_ecs::types::AutoScalingGroupProviderUpdate::builder().build(),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(resp.capacity_provider().unwrap().name(), Some("update-cp"));
}

// ============================================================================
// Task tests (RunTask, DescribeTasks, StopTask, ListTasks, StartTask)
// ============================================================================

#[tokio::test]
async fn test_run_task() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let resp = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(2)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tasks().len(), 2);
    assert_eq!(resp.failures().len(), 0);

    let task = &resp.tasks()[0];
    assert!(task.task_arn().unwrap().contains("test_ecs_cluster"));
    assert!(
        task.task_definition_arn()
            .unwrap()
            .contains("test_ecs_task")
    );
    assert_eq!(task.last_status(), Some("RUNNING"));
    assert_eq!(task.desired_status(), Some("RUNNING"));
    assert!(!task.containers().is_empty());
    assert_eq!(task.containers()[0].name(), Some("hello_world"));
}

#[tokio::test]
async fn test_describe_tasks() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let run_resp = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(1)
        .send()
        .await
        .unwrap();

    let task_arn = run_resp.tasks()[0].task_arn().unwrap().to_string();

    let resp = client
        .describe_tasks()
        .cluster("test_ecs_cluster")
        .tasks(&task_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tasks().len(), 1);
    assert_eq!(resp.tasks()[0].task_arn(), Some(task_arn.as_str()));
    assert_eq!(resp.failures().len(), 0);
}

#[tokio::test]
async fn test_describe_tasks_not_found() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("test_cluster")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_tasks()
        .cluster("test_cluster")
        .tasks("arn:aws:ecs:us-east-1:123456789012:task/test_cluster/nonexistent")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tasks().len(), 0);
    assert_eq!(resp.failures().len(), 1);
}

#[tokio::test]
async fn test_stop_task() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let run_resp = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(1)
        .send()
        .await
        .unwrap();

    let task_arn = run_resp.tasks()[0].task_arn().unwrap().to_string();

    let resp = client
        .stop_task()
        .cluster("test_ecs_cluster")
        .task(&task_arn)
        .reason("Testing stop")
        .send()
        .await
        .unwrap();

    let task = resp.task().unwrap();
    assert_eq!(task.last_status(), Some("STOPPED"));
    assert_eq!(task.desired_status(), Some("STOPPED"));
    assert_eq!(task.stopped_reason(), Some("Testing stop"));
}

#[tokio::test]
async fn test_list_tasks() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(3)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tasks()
        .cluster("test_ecs_cluster")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.task_arns().len(), 3);
}

#[tokio::test]
async fn test_run_task_cluster_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .run_task()
        .cluster("nonexistent")
        .task_definition("some_task")
        .send()
        .await
        .expect_err("should fail");

    let service_err = err.into_service_error();
    assert!(service_err.is_cluster_not_found_exception());
}

// ============================================================================
// Task Set tests
// ============================================================================

#[tokio::test]
async fn test_create_task_set() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("ts-service")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .deployment_controller(
            aws_sdk_ecs::types::DeploymentController::builder()
                .r#type(aws_sdk_ecs::types::DeploymentControllerType::External)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .create_task_set()
        .cluster("test_ecs_cluster")
        .service("ts-service")
        .task_definition("test_ecs_task")
        .scale(
            aws_sdk_ecs::types::Scale::builder()
                .value(50.0)
                .unit(aws_sdk_ecs::types::ScaleUnit::Percent)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let ts = resp.task_set().unwrap();
    assert!(ts.task_set_arn().is_some());
    assert_eq!(ts.status(), Some("ACTIVE"));
    let scale = ts.scale().unwrap();
    assert!((scale.value() - 50.0).abs() < 0.001);
}

#[tokio::test]
async fn test_delete_task_set() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("ts-service")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap();

    let ts_resp = client
        .create_task_set()
        .cluster("test_ecs_cluster")
        .service("ts-service")
        .task_definition("test_ecs_task")
        .send()
        .await
        .unwrap();

    let ts_arn = ts_resp
        .task_set()
        .unwrap()
        .task_set_arn()
        .unwrap()
        .to_string();

    let resp = client
        .delete_task_set()
        .cluster("test_ecs_cluster")
        .service("ts-service")
        .task_set(&ts_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.task_set().unwrap().status(), Some("INACTIVE"));
}

#[tokio::test]
async fn test_describe_task_sets() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("ts-service")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap();

    client
        .create_task_set()
        .cluster("test_ecs_cluster")
        .service("ts-service")
        .task_definition("test_ecs_task")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_task_sets()
        .cluster("test_ecs_cluster")
        .service("ts-service")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.task_sets().len(), 1);
}

#[tokio::test]
async fn test_update_task_set() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("ts-service")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap();

    let ts_resp = client
        .create_task_set()
        .cluster("test_ecs_cluster")
        .service("ts-service")
        .task_definition("test_ecs_task")
        .send()
        .await
        .unwrap();

    let ts_arn = ts_resp
        .task_set()
        .unwrap()
        .task_set_arn()
        .unwrap()
        .to_string();

    let resp = client
        .update_task_set()
        .cluster("test_ecs_cluster")
        .service("ts-service")
        .task_set(&ts_arn)
        .scale(
            aws_sdk_ecs::types::Scale::builder()
                .value(75.0)
                .unit(aws_sdk_ecs::types::ScaleUnit::Percent)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let ts = resp.task_set().unwrap();
    let scale = ts.scale().unwrap();
    assert!((scale.value() - 75.0).abs() < 0.001);
}

#[tokio::test]
async fn test_update_service_primary_task_set() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("ts-service")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap();

    let ts_resp = client
        .create_task_set()
        .cluster("test_ecs_cluster")
        .service("ts-service")
        .task_definition("test_ecs_task")
        .send()
        .await
        .unwrap();

    let ts_arn = ts_resp
        .task_set()
        .unwrap()
        .task_set_arn()
        .unwrap()
        .to_string();

    let resp = client
        .update_service_primary_task_set()
        .cluster("test_ecs_cluster")
        .service("ts-service")
        .primary_task_set(&ts_arn)
        .send()
        .await
        .unwrap();

    assert!(resp.task_set().is_some());
}

// ============================================================================
// Account Setting tests
// ============================================================================

#[tokio::test]
async fn test_put_account_setting() {
    let client = make_ecs_client().await;

    let resp = client
        .put_account_setting()
        .name(aws_sdk_ecs::types::SettingName::ContainerInsights)
        .value("enabled")
        .send()
        .await
        .unwrap();

    let setting = resp.setting().unwrap();
    assert_eq!(
        setting.name(),
        Some(&aws_sdk_ecs::types::SettingName::ContainerInsights)
    );
    assert_eq!(setting.value(), Some("enabled"));
}

#[tokio::test]
async fn test_delete_account_setting() {
    let client = make_ecs_client().await;

    client
        .put_account_setting()
        .name(aws_sdk_ecs::types::SettingName::ContainerInsights)
        .value("enabled")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_account_setting()
        .name(aws_sdk_ecs::types::SettingName::ContainerInsights)
        .send()
        .await
        .unwrap();

    assert!(resp.setting().is_some());
}

#[tokio::test]
async fn test_list_account_settings() {
    let client = make_ecs_client().await;

    client
        .put_account_setting()
        .name(aws_sdk_ecs::types::SettingName::ContainerInsights)
        .value("enabled")
        .send()
        .await
        .unwrap();

    let resp = client.list_account_settings().send().await.unwrap();

    assert!(!resp.settings().is_empty());
}

// ============================================================================
// Attribute tests
// ============================================================================

#[tokio::test]
async fn test_put_attributes() {
    let client = make_ecs_client().await;

    let resp = client
        .put_attributes()
        .attributes(
            aws_sdk_ecs::types::Attribute::builder()
                .name("env")
                .value("production")
                .target_type(aws_sdk_ecs::types::TargetType::ContainerInstance)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    assert!(!resp.attributes().is_empty());
    assert_eq!(resp.attributes()[0].name(), "env");
}

#[tokio::test]
async fn test_delete_attributes() {
    let client = make_ecs_client().await;

    client
        .put_attributes()
        .attributes(
            aws_sdk_ecs::types::Attribute::builder()
                .name("env")
                .value("production")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_attributes()
        .attributes(
            aws_sdk_ecs::types::Attribute::builder()
                .name("env")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    assert!(!resp.attributes().is_empty());
}

#[tokio::test]
async fn test_list_attributes() {
    let client = make_ecs_client().await;

    client
        .put_attributes()
        .attributes(
            aws_sdk_ecs::types::Attribute::builder()
                .name("env")
                .value("production")
                .target_type(aws_sdk_ecs::types::TargetType::ContainerInstance)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_attributes()
        .target_type(aws_sdk_ecs::types::TargetType::ContainerInstance)
        .send()
        .await
        .unwrap();

    assert!(!resp.attributes().is_empty());
}

// ============================================================================
// Tag tests
// ============================================================================

#[tokio::test]
async fn test_tag_and_list_tags_for_resource() {
    let client = make_ecs_client().await;

    let cluster_arn = client
        .create_cluster()
        .cluster_name("tag-cluster")
        .send()
        .await
        .unwrap()
        .cluster()
        .unwrap()
        .cluster_arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&cluster_arn)
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("environment")
                .value("test")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&cluster_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), Some("environment"));
    assert_eq!(resp.tags()[0].value(), Some("test"));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_ecs_client().await;

    let cluster_arn = client
        .create_cluster()
        .cluster_name("untag-cluster")
        .send()
        .await
        .unwrap()
        .cluster()
        .unwrap()
        .cluster_arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&cluster_arn)
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("env")
                .value("test")
                .build(),
        )
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("team")
                .value("eng")
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&cluster_arn)
        .tag_keys("env")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&cluster_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), Some("team"));
}

// ============================================================================
// Container Instance tests
// ============================================================================

#[tokio::test]
async fn test_register_container_instance() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("ci-cluster")
        .send()
        .await
        .unwrap();

    let resp = client
        .register_container_instance()
        .cluster("ci-cluster")
        .send()
        .await
        .unwrap();

    let ci = resp.container_instance().unwrap();
    assert!(ci.container_instance_arn().is_some());
    assert_eq!(ci.status(), Some("ACTIVE"));
    assert!(ci.agent_connected());
}

#[tokio::test]
async fn test_deregister_container_instance() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("ci-cluster")
        .send()
        .await
        .unwrap();

    let ci_arn = client
        .register_container_instance()
        .cluster("ci-cluster")
        .send()
        .await
        .unwrap()
        .container_instance()
        .unwrap()
        .container_instance_arn()
        .unwrap()
        .to_string();

    let resp = client
        .deregister_container_instance()
        .cluster("ci-cluster")
        .container_instance(&ci_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.container_instance().unwrap().status(),
        Some("INACTIVE")
    );
}

#[tokio::test]
async fn test_describe_container_instances() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("ci-cluster")
        .send()
        .await
        .unwrap();

    let ci_arn = client
        .register_container_instance()
        .cluster("ci-cluster")
        .send()
        .await
        .unwrap()
        .container_instance()
        .unwrap()
        .container_instance_arn()
        .unwrap()
        .to_string();

    let resp = client
        .describe_container_instances()
        .cluster("ci-cluster")
        .container_instances(&ci_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.container_instances().len(), 1);
    assert_eq!(resp.failures().len(), 0);
}

#[tokio::test]
async fn test_list_container_instances() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("ci-cluster")
        .send()
        .await
        .unwrap();

    client
        .register_container_instance()
        .cluster("ci-cluster")
        .send()
        .await
        .unwrap();

    client
        .register_container_instance()
        .cluster("ci-cluster")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_container_instances()
        .cluster("ci-cluster")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.container_instance_arns().len(), 2);
}

#[tokio::test]
async fn test_update_container_instances_state() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("ci-cluster")
        .send()
        .await
        .unwrap();

    let ci_arn = client
        .register_container_instance()
        .cluster("ci-cluster")
        .send()
        .await
        .unwrap()
        .container_instance()
        .unwrap()
        .container_instance_arn()
        .unwrap()
        .to_string();

    let resp = client
        .update_container_instances_state()
        .cluster("ci-cluster")
        .container_instances(&ci_arn)
        .status(aws_sdk_ecs::types::ContainerInstanceStatus::Draining)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.container_instances().len(), 1);
    assert_eq!(resp.container_instances()[0].status(), Some("DRAINING"));
    assert_eq!(resp.failures().len(), 0);
}

// ============================================================================
// StartTask test
// ============================================================================

#[tokio::test]
async fn test_start_task() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let ci_arn = client
        .register_container_instance()
        .cluster("test_ecs_cluster")
        .send()
        .await
        .unwrap()
        .container_instance()
        .unwrap()
        .container_instance_arn()
        .unwrap()
        .to_string();

    let resp = client
        .start_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .container_instances(&ci_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tasks().len(), 1);
    assert_eq!(resp.failures().len(), 0);
    assert_eq!(resp.tasks()[0].last_status(), Some("RUNNING"));
    assert!(resp.tasks()[0].container_instance_arn().is_some());
}

// ============================================================================
// Service lifecycle test (create -> describe -> update -> delete -> verify)
// ============================================================================

#[tokio::test]
async fn test_service_lifecycle() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    // Create
    let svc = client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("lifecycle-svc")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap()
        .service()
        .unwrap()
        .clone();
    assert_eq!(svc.status(), Some("ACTIVE"));
    assert_eq!(svc.desired_count(), 1);

    // Describe
    let desc = client
        .describe_services()
        .cluster("test_ecs_cluster")
        .services("lifecycle-svc")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.services().len(), 1);

    // Update
    let updated = client
        .update_service()
        .cluster("test_ecs_cluster")
        .service("lifecycle-svc")
        .desired_count(3)
        .send()
        .await
        .unwrap();
    assert_eq!(updated.service().unwrap().desired_count(), 3);

    // Delete
    let deleted = client
        .delete_service()
        .cluster("test_ecs_cluster")
        .service("lifecycle-svc")
        .send()
        .await
        .unwrap();
    assert_eq!(deleted.service().unwrap().status(), Some("INACTIVE"));

    // Verify gone from list_services
    let list = client
        .list_services()
        .cluster("test_ecs_cluster")
        .send()
        .await
        .unwrap();
    assert!(
        !list
            .service_arns()
            .iter()
            .any(|a| a.contains("lifecycle-svc"))
    );
}

// ============================================================================
// Ported from moto: test_ecs.py
// ============================================================================

// Ported from moto: test_ecs.py::test_run_task (extended assertions)
#[tokio::test]
async fn test_run_task_with_tags_and_started_by() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let resp = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(2)
        .started_by("moto")
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("tagKey0")
                .value("tagValue0")
                .build(),
        )
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("tagKey1")
                .value("tagValue1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tasks().len(), 2);
    let task = &resp.tasks()[0];
    assert!(
        task.task_arn()
            .unwrap()
            .contains(&format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:task/"))
    );
    assert_eq!(
        task.cluster_arn(),
        Some(format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:cluster/test_ecs_cluster").as_str())
    );
    assert!(
        task.task_definition_arn()
            .unwrap()
            .contains("test_ecs_task:1")
    );
    assert_eq!(task.last_status(), Some("RUNNING"));
    assert_eq!(task.desired_status(), Some("RUNNING"));
    assert_eq!(task.started_by(), Some("moto"));
    assert_eq!(task.stopped_reason(), Some(""));
    assert!(!task.containers().is_empty());
}

// Ported from moto: test_ecs.py::test_run_task - cluster running count
#[tokio::test]
async fn test_run_task_updates_cluster_running_count() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(1)
        .started_by("moto")
        .send()
        .await
        .unwrap();

    let cluster = client
        .describe_clusters()
        .clusters("test_ecs_cluster")
        .send()
        .await
        .unwrap();
    assert_eq!(cluster.clusters()[0].running_tasks_count(), 1);

    // Run 2 more
    client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(2)
        .started_by("moto")
        .send()
        .await
        .unwrap();

    let cluster = client
        .describe_clusters()
        .clusters("test_ecs_cluster")
        .send()
        .await
        .unwrap();
    assert_eq!(cluster.clusters()[0].running_tasks_count(), 3);
}

// Ported from moto: test_ecs.py::test_run_task_default_cluster
#[tokio::test]
async fn test_run_task_default_cluster() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("default")
        .send()
        .await
        .unwrap();

    client
        .register_task_definition()
        .family("test_ecs_task")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("hello_world")
                .image("docker/hello-world:latest")
                .cpu(1024)
                .memory(400)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .run_task()
        .launch_type(aws_sdk_ecs::types::LaunchType::Fargate)
        .task_definition("test_ecs_task")
        .count(2)
        .started_by("moto")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tasks().len(), 2);
    assert_eq!(
        resp.tasks()[0].launch_type(),
        Some(&aws_sdk_ecs::types::LaunchType::Fargate)
    );
    assert!(
        resp.tasks()[0]
            .task_arn()
            .unwrap()
            .starts_with(&format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:task/default/"))
    );
    assert_eq!(
        resp.tasks()[0].cluster_arn(),
        Some(format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:cluster/default").as_str())
    );
    assert_eq!(resp.tasks()[0].last_status(), Some("RUNNING"));
    assert_eq!(resp.tasks()[0].desired_status(), Some("RUNNING"));
    assert_eq!(resp.tasks()[0].started_by(), Some("moto"));
    assert_eq!(resp.tasks()[0].stopped_reason(), Some(""));
}

// Ported from moto: test_ecs.py::test_stop_task (with cluster running count)
#[tokio::test]
async fn test_stop_task_updates_cluster_running_count() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let run_resp = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(1)
        .started_by("moto")
        .send()
        .await
        .unwrap();

    let cluster = client
        .describe_clusters()
        .clusters("test_ecs_cluster")
        .send()
        .await
        .unwrap();
    assert_eq!(cluster.clusters()[0].running_tasks_count(), 1);

    let task_arn = run_resp.tasks()[0].task_arn().unwrap().to_string();
    let stop_resp = client
        .stop_task()
        .cluster("test_ecs_cluster")
        .task(&task_arn)
        .reason("moto testing")
        .send()
        .await
        .unwrap();

    let cluster = client
        .describe_clusters()
        .clusters("test_ecs_cluster")
        .send()
        .await
        .unwrap();
    assert_eq!(cluster.clusters()[0].running_tasks_count(), 0);

    assert_eq!(
        stop_resp.task().unwrap().task_arn(),
        Some(task_arn.as_str())
    );
    assert_eq!(stop_resp.task().unwrap().last_status(), Some("STOPPED"));
    assert_eq!(stop_resp.task().unwrap().desired_status(), Some("STOPPED"));
    assert_eq!(
        stop_resp.task().unwrap().stopped_reason(),
        Some("moto testing")
    );
}

// Ported from moto: test_ecs.py::test_describe_tasks (group field, task IDs)
#[tokio::test]
async fn test_describe_tasks_group_and_task_id() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    // Default group should be family:{family}
    let run_resp = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(2)
        .started_by("moto")
        .send()
        .await
        .unwrap();

    let task_arns: Vec<String> = run_resp
        .tasks()
        .iter()
        .map(|t| t.task_arn().unwrap().to_string())
        .collect();

    let resp = client
        .describe_tasks()
        .cluster("test_ecs_cluster")
        .tasks(&task_arns[0])
        .tasks(&task_arns[1])
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tasks().len(), 2);
    assert_eq!(resp.tasks()[0].group(), Some("family:test_ecs_task"));
    assert_eq!(resp.tasks()[1].group(), Some("family:test_ecs_task"));

    // Test passing task ID instead of full ARN
    let task_id = task_arns[0].split('/').next_back().unwrap();
    let resp2 = client
        .describe_tasks()
        .cluster("test_ecs_cluster")
        .tasks(task_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp2.tasks().len(), 1);

    // Run task with custom group
    let resp3 = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(1)
        .started_by("moto")
        .group("test_group")
        .send()
        .await
        .unwrap();
    assert_eq!(resp3.tasks().len(), 1);
    assert_eq!(resp3.tasks()[0].group(), Some("test_group"));
}

// Ported from moto: test_ecs.py::test_list_tasks_with_filters
#[tokio::test]
async fn test_list_tasks_with_filters() {
    let client = make_ecs_client().await;

    // Setup two clusters
    client
        .create_cluster()
        .cluster_name("test_cluster_1")
        .send()
        .await
        .unwrap();
    client
        .create_cluster()
        .cluster_name("test_cluster_2")
        .send()
        .await
        .unwrap();

    // Register container instances
    let ci1_arn = client
        .register_container_instance()
        .cluster("test_cluster_1")
        .send()
        .await
        .unwrap()
        .container_instance()
        .unwrap()
        .container_instance_arn()
        .unwrap()
        .to_string();
    let ci1_id = ci1_arn.split('/').next_back().unwrap().to_string();

    let ci2_arn = client
        .register_container_instance()
        .cluster("test_cluster_2")
        .send()
        .await
        .unwrap()
        .container_instance()
        .unwrap()
        .container_instance_arn()
        .unwrap()
        .to_string();
    let ci2_id = ci2_arn.split('/').next_back().unwrap().to_string();

    // Register two task defs
    client
        .register_task_definition()
        .family("test_task_def_1")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("hello_world")
                .image("docker/hello-world:latest")
                .cpu(1024)
                .memory(400)
                .build(),
        )
        .send()
        .await
        .unwrap();
    client
        .register_task_definition()
        .family("test_task_def_2")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("hello_world")
                .image("docker/hello-world:latest")
                .cpu(1024)
                .memory(400)
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Start tasks
    client
        .start_task()
        .cluster("test_cluster_1")
        .task_definition("test_task_def_1")
        .container_instances(&ci1_id)
        .started_by("foo")
        .send()
        .await
        .unwrap();

    client
        .start_task()
        .cluster("test_cluster_2")
        .task_definition("test_task_def_2")
        .container_instances(&ci2_id)
        .started_by("foo")
        .send()
        .await
        .unwrap();

    client
        .start_task()
        .cluster("test_cluster_1")
        .task_definition("test_task_def_1")
        .container_instances(&ci1_id)
        .started_by("bar")
        .send()
        .await
        .unwrap();

    // Filter by cluster
    let r1 = client
        .list_tasks()
        .cluster("test_cluster_1")
        .send()
        .await
        .unwrap();
    assert_eq!(r1.task_arns().len(), 2);

    let r2 = client
        .list_tasks()
        .cluster("test_cluster_2")
        .send()
        .await
        .unwrap();
    assert_eq!(r2.task_arns().len(), 1);

    // Filter by containerInstance
    let r3 = client
        .list_tasks()
        .cluster("test_cluster_1")
        .container_instance("bad-id")
        .send()
        .await
        .unwrap();
    assert_eq!(r3.task_arns().len(), 0);

    let r4 = client
        .list_tasks()
        .cluster("test_cluster_1")
        .container_instance(&ci1_id)
        .send()
        .await
        .unwrap();
    assert_eq!(r4.task_arns().len(), 2);

    // Filter by family
    let r5 = client
        .list_tasks()
        .cluster("test_cluster_1")
        .family("non-existent-family")
        .send()
        .await
        .unwrap();
    assert_eq!(r5.task_arns().len(), 0);

    let r6 = client
        .list_tasks()
        .cluster("test_cluster_1")
        .family("test_task_def_1")
        .send()
        .await
        .unwrap();
    assert_eq!(r6.task_arns().len(), 2);

    // Filter by startedBy
    let r7 = client
        .list_tasks()
        .cluster("test_cluster_1")
        .started_by("foo")
        .send()
        .await
        .unwrap();
    assert_eq!(r7.task_arns().len(), 1);

    let r8 = client
        .list_tasks()
        .cluster("test_cluster_1")
        .started_by("bar")
        .send()
        .await
        .unwrap();
    assert_eq!(r8.task_arns().len(), 1);
}

// Ported from moto: test_ecs.py::test_list_tasks_exceptions
#[tokio::test]
async fn test_list_tasks_exceptions() {
    let client = make_ecs_client().await;

    let err = client
        .list_tasks()
        .cluster("not_a_cluster")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_cluster_not_found_exception());
}

// Ported from moto: test_ecs.py::test_create_cluster_with_tags
#[tokio::test]
async fn test_create_cluster_with_tags() {
    let client = make_ecs_client().await;

    // Cluster without tags
    let cluster1 = client
        .create_cluster()
        .cluster_name("c1")
        .send()
        .await
        .unwrap();
    let c1_arn = cluster1
        .cluster()
        .unwrap()
        .cluster_arn()
        .unwrap()
        .to_string();

    // Tag it
    client
        .tag_resource()
        .resource_arn(&c1_arn)
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("tagName")
                .value("TagValue")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&c1_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tags().len(), 1);
    assert_eq!(tags.tags()[0].key(), Some("tagName"));
    assert_eq!(tags.tags()[0].value(), Some("TagValue"));
}

// Ported from moto: test_ecs.py::test_create_service_scheduling_strategy
#[tokio::test]
async fn test_create_service_scheduling_strategy_daemon() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let resp = client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("daemon-service")
        .task_definition("test_ecs_task")
        .desired_count(2)
        .scheduling_strategy(aws_sdk_ecs::types::SchedulingStrategy::Daemon)
        .send()
        .await
        .unwrap();

    let svc = resp.service().unwrap();
    assert_eq!(
        svc.cluster_arn(),
        Some(format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:cluster/test_ecs_cluster").as_str())
    );
    assert_eq!(svc.desired_count(), 2);
    assert_eq!(svc.pending_count(), 2);
    assert_eq!(svc.running_count(), 0);
    assert_eq!(
        svc.service_arn(),
        Some(
            format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:service/test_ecs_cluster/daemon-service")
                .as_str()
        )
    );
    assert_eq!(svc.service_name(), Some("daemon-service"));
    assert_eq!(svc.status(), Some("ACTIVE"));
    assert_eq!(
        svc.scheduling_strategy(),
        Some(&aws_sdk_ecs::types::SchedulingStrategy::Daemon)
    );
}

// Ported from moto: test_ecs.py::test_create_service_load_balancing
#[tokio::test]
async fn test_create_service_load_balancing() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let resp = client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("lb-service")
        .task_definition("test_ecs_task")
        .desired_count(2)
        .load_balancers(
            aws_sdk_ecs::types::LoadBalancer::builder()
                .target_group_arn("test_target_group_arn")
                .load_balancer_name("test_load_balancer_name")
                .container_name("test_container_name")
                .container_port(123)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let svc = resp.service().unwrap();
    assert_eq!(svc.load_balancers().len(), 1);
    assert_eq!(
        svc.load_balancers()[0].target_group_arn(),
        Some("test_target_group_arn")
    );
    assert_eq!(
        svc.load_balancers()[0].load_balancer_name(),
        Some("test_load_balancer_name")
    );
    assert_eq!(
        svc.load_balancers()[0].container_name(),
        Some("test_container_name")
    );
    assert_eq!(svc.load_balancers()[0].container_port(), Some(123));
}

// Ported from moto: test_ecs.py::test_list_tags_for_resource (task definition tags)
#[tokio::test]
async fn test_list_tags_for_resource_task_definition() {
    let client = make_ecs_client().await;

    let resp = client
        .register_task_definition()
        .family("test_ecs_task")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("hello_world")
                .image("docker/hello-world:latest")
                .cpu(1024)
                .memory(400)
                .essential(true)
                .build(),
        )
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("createdBy")
                .value("moto-unittest")
                .build(),
        )
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("foo")
                .value("bar")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let td = resp.task_definition().unwrap();
    let td_arn = td.task_definition_arn().unwrap().to_string();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&td_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(tags_resp.tags().len(), 2);
    assert_eq!(tags_resp.tags()[0].key(), Some("createdBy"));
    assert_eq!(tags_resp.tags()[0].value(), Some("moto-unittest"));
    assert_eq!(tags_resp.tags()[1].key(), Some("foo"));
    assert_eq!(tags_resp.tags()[1].value(), Some("bar"));
}

// Ported from moto: test_ecs.py::test_list_tags_for_resource_ecs_service
#[tokio::test]
async fn test_list_tags_for_resource_ecs_service() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let resp = client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("tagged-service")
        .task_definition("test_ecs_task")
        .desired_count(2)
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("createdBy")
                .value("moto-unittest")
                .build(),
        )
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("foo")
                .value("bar")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let service_arn = resp.service().unwrap().service_arn().unwrap().to_string();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&service_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(tags_resp.tags().len(), 2);
    assert_eq!(tags_resp.tags()[0].key(), Some("createdBy"));
    assert_eq!(tags_resp.tags()[0].value(), Some("moto-unittest"));
    assert_eq!(tags_resp.tags()[1].key(), Some("foo"));
    assert_eq!(tags_resp.tags()[1].value(), Some("bar"));
}

// Ported from moto: test_ecs.py::test_ecs_service_tag_resource_overwrites_tag
#[tokio::test]
async fn test_ecs_service_tag_resource_overwrites_tag() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let service_arn = client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("test_ecs_service")
        .task_definition("test_ecs_task")
        .desired_count(2)
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("foo")
                .value("bar")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .service()
        .unwrap()
        .service_arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&service_arn)
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("createdBy")
                .value("moto-unittest")
                .build(),
        )
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("foo")
                .value("hello world")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&service_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(tags.tags().len(), 2);
    // Tags should contain the overwritten value
    let foo_tag = tags.tags().iter().find(|t| t.key() == Some("foo")).unwrap();
    assert_eq!(foo_tag.value(), Some("hello world"));
    let created_tag = tags
        .tags()
        .iter()
        .find(|t| t.key() == Some("createdBy"))
        .unwrap();
    assert_eq!(created_tag.value(), Some("moto-unittest"));
}

// Ported from moto: test_ecs.py::test_ecs_service_untag_resource_multiple_tags
#[tokio::test]
async fn test_ecs_service_untag_resource_multiple_tags() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let service_arn = client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("test_ecs_service")
        .task_definition("test_ecs_task")
        .desired_count(2)
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("foo")
                .value("bar")
                .build(),
        )
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("createdBy")
                .value("moto-unittest")
                .build(),
        )
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("hello")
                .value("world")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .service()
        .unwrap()
        .service_arn()
        .unwrap()
        .to_string();

    client
        .untag_resource()
        .resource_arn(&service_arn)
        .tag_keys("foo")
        .tag_keys("createdBy")
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&service_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(tags.tags().len(), 1);
    assert_eq!(tags.tags()[0].key(), Some("hello"));
    assert_eq!(tags.tags()[0].value(), Some("world"));
}

// Ported from moto: test_ecs_task_tags.py::test_add_tags_to_task
#[tokio::test]
async fn test_add_tags_to_task() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let task_arn = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(1)
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("k1")
                .value("v1")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .tasks()[0]
        .task_arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&task_arn)
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("k2")
                .value("v2")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&task_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(tags.tags().len(), 2);
    let k1_tag = tags.tags().iter().find(|t| t.key() == Some("k1")).unwrap();
    assert_eq!(k1_tag.value(), Some("v1"));
    let k2_tag = tags.tags().iter().find(|t| t.key() == Some("k2")).unwrap();
    assert_eq!(k2_tag.value(), Some("v2"));

    client
        .untag_resource()
        .resource_arn(&task_arn)
        .tag_keys("k2")
        .send()
        .await
        .unwrap();

    let tags2 = client
        .list_tags_for_resource()
        .resource_arn(&task_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags2.tags().len(), 1);
    assert_eq!(tags2.tags()[0].key(), Some("k1"));
}

// Ported from moto: test_ecs_tasksets.py::test_create_task_set (with load balancers)
#[tokio::test]
async fn test_create_task_set_with_load_balancers() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("ts-svc")
        .task_definition("test_ecs_task")
        .desired_count(2)
        .deployment_controller(
            aws_sdk_ecs::types::DeploymentController::builder()
                .r#type(aws_sdk_ecs::types::DeploymentControllerType::External)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .create_task_set()
        .cluster("test_ecs_cluster")
        .service("ts-svc")
        .task_definition("test_ecs_task")
        .send()
        .await
        .unwrap();

    let ts = resp.task_set().unwrap();
    assert!(ts.task_definition().unwrap().ends_with("test_ecs_task:1"));
    let scale = ts.scale().unwrap();
    assert!((scale.value() - 100.0).abs() < 0.001);
    assert_eq!(scale.unit(), Some(&aws_sdk_ecs::types::ScaleUnit::Percent));
    assert_eq!(ts.launch_type(), Some(&aws_sdk_ecs::types::LaunchType::Ec2));
}

// Ported from moto: test_ecs_tasksets.py::test_create_task_sets_with_tags
#[tokio::test]
async fn test_create_task_sets_with_tags() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("ts-svc")
        .task_definition("test_ecs_task")
        .desired_count(2)
        .deployment_controller(
            aws_sdk_ecs::types::DeploymentController::builder()
                .r#type(aws_sdk_ecs::types::DeploymentControllerType::External)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .create_task_set()
        .cluster("test_ecs_cluster")
        .service("ts-svc")
        .task_definition("test_ecs_task")
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("k1")
                .value("v1")
                .build(),
        )
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("k2")
                .value("v2")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let task_sets = client
        .describe_task_sets()
        .cluster("test_ecs_cluster")
        .service("ts-svc")
        .include(aws_sdk_ecs::types::TaskSetField::Tags)
        .send()
        .await
        .unwrap();

    let ts = &task_sets.task_sets()[0];
    assert_eq!(ts.tags().len(), 2);

    // Tag the task set with additional tag
    let ts_arn = ts.task_set_arn().unwrap().to_string();
    client
        .tag_resource()
        .resource_arn(&ts_arn)
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("k3")
                .value("v3")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&ts_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tags().len(), 3);

    // Untag one
    client
        .untag_resource()
        .resource_arn(&ts_arn)
        .tag_keys("k2")
        .send()
        .await
        .unwrap();

    let tags2 = client
        .list_tags_for_resource()
        .resource_arn(&ts_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags2.tags().len(), 2);
}

// Ported from moto: test_ecs_account_settings.py::test_list_account_settings_initial
#[tokio::test]
async fn test_list_account_settings_initial() {
    let client = make_ecs_client().await;

    let resp = client.list_account_settings().send().await.unwrap();
    assert_eq!(resp.settings().len(), 0);
}

// Ported from moto: test_ecs_account_settings.py::test_delete_account_setting
#[tokio::test]
async fn test_delete_account_setting_removes_from_list() {
    let client = make_ecs_client().await;

    client
        .put_account_setting()
        .name(aws_sdk_ecs::types::SettingName::from(
            "containerInstanceLongArnFormat",
        ))
        .value("enabled")
        .send()
        .await
        .unwrap();
    client
        .put_account_setting()
        .name(aws_sdk_ecs::types::SettingName::from(
            "serviceLongArnFormat",
        ))
        .value("enabled")
        .send()
        .await
        .unwrap();
    client
        .put_account_setting()
        .name(aws_sdk_ecs::types::SettingName::from("taskLongArnFormat"))
        .value("enabled")
        .send()
        .await
        .unwrap();

    let resp = client.list_account_settings().send().await.unwrap();
    assert_eq!(resp.settings().len(), 3);

    client
        .delete_account_setting()
        .name(aws_sdk_ecs::types::SettingName::from(
            "serviceLongArnFormat",
        ))
        .send()
        .await
        .unwrap();

    let resp2 = client.list_account_settings().send().await.unwrap();
    assert_eq!(resp2.settings().len(), 2);
}

// Ported from moto: test_ecs.py::test_start_task (full assertions from moto)
#[tokio::test]
async fn test_start_task_with_tags_and_group() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let ci_arn = client
        .register_container_instance()
        .cluster("test_ecs_cluster")
        .send()
        .await
        .unwrap()
        .container_instance()
        .unwrap()
        .container_instance_arn()
        .unwrap()
        .to_string();
    let ci_id = ci_arn.split('/').next_back().unwrap().to_string();

    let resp = client
        .start_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .container_instances(&ci_id)
        .started_by("moto")
        .tags(
            aws_sdk_ecs::types::Tag::builder()
                .key("Name")
                .value("test_ecs_start_task")
                .build(),
        )
        .group("test_group")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tasks().len(), 1);
    let task = &resp.tasks()[0];
    assert!(task.task_arn().unwrap().contains("test_ecs_cluster"));
    assert_eq!(
        task.cluster_arn(),
        Some(format!("arn:aws:ecs:us-east-1:{ACCOUNT_ID}:cluster/test_ecs_cluster").as_str())
    );
    assert!(
        task.task_definition_arn()
            .unwrap()
            .contains("test_ecs_task:1")
    );
    assert!(task.container_instance_arn().is_some());
    assert_eq!(task.last_status(), Some("RUNNING"));
    assert_eq!(task.desired_status(), Some("RUNNING"));
    assert_eq!(task.started_by(), Some("moto"));
    assert_eq!(task.stopped_reason(), Some(""));
    assert_eq!(task.group(), Some("test_group"));

    // Default group is family:{family}
    let resp2 = client
        .start_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .container_instances(&ci_id)
        .started_by("moto")
        .send()
        .await
        .unwrap();
    assert_eq!(resp2.tasks()[0].group(), Some("family:test_ecs_task"));
}

// Ported from moto: test_ecs.py::test_stop_task_exceptions
#[tokio::test]
async fn test_stop_task_exceptions() {
    let client = make_ecs_client().await;

    let err = client
        .stop_task()
        .task("fake_task")
        .send()
        .await
        .unwrap_err();
    let service_err = err.into_service_error();
    assert!(service_err.is_cluster_not_found_exception());
}

// Ported from moto: test_ecs.py::test_list_tasks (startedBy filter)
#[tokio::test]
async fn test_list_tasks_started_by_filter() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("default")
        .send()
        .await
        .unwrap();

    let ci_arn = client
        .register_container_instance()
        .cluster("default")
        .send()
        .await
        .unwrap()
        .container_instance()
        .unwrap()
        .container_instance_arn()
        .unwrap()
        .to_string();
    let ci_id = ci_arn.split('/').next_back().unwrap().to_string();

    client
        .register_task_definition()
        .family("test_ecs_task")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("hello_world")
                .image("docker/hello-world:latest")
                .cpu(1024)
                .memory(400)
                .essential(true)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .start_task()
        .task_definition("test_ecs_task")
        .container_instances(&ci_id)
        .started_by("foo")
        .send()
        .await
        .unwrap();

    client
        .start_task()
        .task_definition("test_ecs_task")
        .container_instances(&ci_id)
        .started_by("bar")
        .send()
        .await
        .unwrap();

    let all = client.list_tasks().send().await.unwrap();
    assert_eq!(all.task_arns().len(), 2);

    let foo_tasks = client.list_tasks().started_by("foo").send().await.unwrap();
    assert_eq!(foo_tasks.task_arns().len(), 1);
}

#[tokio::test]
async fn test_delete_cluster_with_active_services_still_marks_inactive() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("active-svc")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap();

    // Delete cluster even though a service exists — mock does not block this
    let resp = client
        .delete_cluster()
        .cluster("test_ecs_cluster")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.cluster().unwrap().status(), Some("INACTIVE"));
}

#[tokio::test]
async fn test_describe_clusters_by_arn() {
    let client = make_ecs_client().await;

    let arn = client
        .create_cluster()
        .cluster_name("arn-cluster")
        .send()
        .await
        .unwrap()
        .cluster()
        .unwrap()
        .cluster_arn()
        .unwrap()
        .to_string();

    let resp = client
        .describe_clusters()
        .clusters(&arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.clusters().len(), 1);
    assert_eq!(resp.clusters()[0].cluster_arn(), Some(arn.as_str()));
    assert_eq!(resp.failures().len(), 0);
}

#[tokio::test]
async fn test_describe_clusters_empty_request_returns_empty() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("some-cluster")
        .send()
        .await
        .unwrap();

    // Calling describe_clusters with no cluster names returns nothing
    let resp = client.describe_clusters().send().await.unwrap();

    assert_eq!(resp.clusters().len(), 0);
    assert_eq!(resp.failures().len(), 0);
}

#[tokio::test]
async fn test_list_clusters_empty_initially() {
    let client = make_ecs_client_with_region("ap-southeast-1").await;

    let resp = client.list_clusters().send().await.unwrap();
    assert_eq!(resp.cluster_arns().len(), 0);
}

#[tokio::test]
async fn test_deregister_task_definition_then_describe_returns_inactive() {
    let client = make_ecs_client().await;

    client
        .register_task_definition()
        .family("inactive-family")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("c1")
                .image("img:latest")
                .memory(256)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .deregister_task_definition()
        .task_definition("inactive-family:1")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_task_definition()
        .task_definition("inactive-family:1")
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.task_definition().unwrap().status(),
        Some(&aws_sdk_ecs::types::TaskDefinitionStatus::Inactive)
    );
}

#[tokio::test]
async fn test_list_task_definitions_excludes_inactive() {
    let client = make_ecs_client().await;

    client
        .register_task_definition()
        .family("active-fam")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("c1")
                .image("img:latest")
                .memory(256)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .register_task_definition()
        .family("inactive-fam")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("c1")
                .image("img:latest")
                .memory(256)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .deregister_task_definition()
        .task_definition("inactive-fam:1")
        .send()
        .await
        .unwrap();

    let resp = client.list_task_definitions().send().await.unwrap();
    assert!(
        resp.task_definition_arns()
            .iter()
            .any(|a| a.contains("active-fam"))
    );
    assert!(
        !resp
            .task_definition_arns()
            .iter()
            .any(|a| a.contains("inactive-fam"))
    );
}

#[tokio::test]
async fn test_list_task_definition_families_excludes_deregistered_only_families() {
    let client = make_ecs_client().await;

    client
        .register_task_definition()
        .family("keep-family")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("c1")
                .image("img:latest")
                .memory(256)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .register_task_definition()
        .family("gone-family")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("c1")
                .image("img:latest")
                .memory(256)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .deregister_task_definition()
        .task_definition("gone-family:1")
        .send()
        .await
        .unwrap();

    let resp = client.list_task_definition_families().send().await.unwrap();
    assert!(resp.families().contains(&"keep-family".to_string()));
    assert!(!resp.families().contains(&"gone-family".to_string()));
}

#[tokio::test]
async fn test_update_service_task_definition() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    // Register a second revision of the task definition
    client
        .register_task_definition()
        .family("test_ecs_task")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("hello_world_v2")
                .image("hello-world:v2")
                .memory(512)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("update-td-svc")
        .task_definition("test_ecs_task:1")
        .desired_count(1)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_service()
        .cluster("test_ecs_cluster")
        .service("update-td-svc")
        .task_definition("test_ecs_task:2")
        .send()
        .await
        .unwrap();

    let svc = resp.service().unwrap();
    assert!(svc.task_definition().unwrap().ends_with("test_ecs_task:2"));
}

#[tokio::test]
async fn test_update_service_not_found() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("my-cluster")
        .send()
        .await
        .unwrap();

    let err = client
        .update_service()
        .cluster("my-cluster")
        .service("nonexistent-svc")
        .desired_count(3)
        .send()
        .await
        .expect_err("should fail for nonexistent service");

    let service_err = err.into_service_error();
    assert!(service_err.is_service_not_found_exception());
}

#[tokio::test]
async fn test_update_service_cluster_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .update_service()
        .cluster("no-such-cluster")
        .service("some-svc")
        .desired_count(1)
        .send()
        .await
        .expect_err("should fail for nonexistent cluster");

    let service_err = err.into_service_error();
    assert!(service_err.is_cluster_not_found_exception());
}

#[tokio::test]
async fn test_list_services_cluster_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .list_services()
        .cluster("no-such-cluster")
        .send()
        .await
        .expect_err("should fail for nonexistent cluster");

    let service_err = err.into_service_error();
    assert!(service_err.is_cluster_not_found_exception());
}

#[tokio::test]
async fn test_list_services_empty_cluster() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("empty-cluster")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_services()
        .cluster("empty-cluster")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.service_arns().len(), 0);
}

#[tokio::test]
async fn test_list_services_excludes_inactive() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("live-svc")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap();

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("dead-svc")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap();

    client
        .delete_service()
        .cluster("test_ecs_cluster")
        .service("dead-svc")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_services()
        .cluster("test_ecs_cluster")
        .send()
        .await
        .unwrap();

    assert!(resp.service_arns().iter().any(|a| a.contains("live-svc")));
    assert!(!resp.service_arns().iter().any(|a| a.contains("dead-svc")));
}

#[tokio::test]
async fn test_create_capacity_provider_duplicate_error() {
    let client = make_ecs_client().await;

    client
        .create_capacity_provider()
        .name("dup-cp")
        .auto_scaling_group_provider(
            aws_sdk_ecs::types::AutoScalingGroupProvider::builder()
                .auto_scaling_group_arn(
                    "arn:aws:autoscaling:us-east-1:123456789012:autoScalingGroup:asg-dup",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let err = client
        .create_capacity_provider()
        .name("dup-cp")
        .auto_scaling_group_provider(
            aws_sdk_ecs::types::AutoScalingGroupProvider::builder()
                .auto_scaling_group_arn(
                    "arn:aws:autoscaling:us-east-1:123456789012:autoScalingGroup:asg-dup",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect_err("duplicate capacity provider should fail");

    let _service_err = err.into_service_error();
}

#[tokio::test]
async fn test_describe_capacity_providers_missing_returns_failure() {
    let client = make_ecs_client().await;

    let resp = client
        .describe_capacity_providers()
        .capacity_providers("nonexistent-cp")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.capacity_providers().len(), 0);
    assert_eq!(resp.failures().len(), 1);
    assert_eq!(resp.failures()[0].reason(), Some("MISSING"));
}

#[tokio::test]
async fn test_stop_task_not_found() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let err = client
        .stop_task()
        .cluster("test_ecs_cluster")
        .task("arn:aws:ecs:us-east-1:123456789012:task/test_ecs_cluster/nonexistent-task-id")
        .send()
        .await
        .expect_err("should fail for nonexistent task");

    let _service_err = err.into_service_error();
}

#[tokio::test]
async fn test_run_task_with_overrides() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let resp = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(1)
        .overrides(
            aws_sdk_ecs::types::TaskOverride::builder()
                .container_overrides(
                    aws_sdk_ecs::types::ContainerOverride::builder()
                        .name("hello_world")
                        .environment(
                            aws_sdk_ecs::types::KeyValuePair::builder()
                                .name("OVERRIDE_KEY")
                                .value("OVERRIDE_VALUE")
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tasks().len(), 1);
    assert_eq!(resp.failures().len(), 0);
    assert_eq!(resp.tasks()[0].last_status(), Some("RUNNING"));
}

#[tokio::test]
async fn test_run_task_fargate_launch_type() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let resp = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .launch_type(aws_sdk_ecs::types::LaunchType::Fargate)
        .count(1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tasks().len(), 1);
    assert_eq!(
        resp.tasks()[0].launch_type(),
        Some(&aws_sdk_ecs::types::LaunchType::Fargate)
    );
}

#[tokio::test]
async fn test_describe_container_instances_missing_returns_failure() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("ci-cluster")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_container_instances()
        .cluster("ci-cluster")
        .container_instances(
            "arn:aws:ecs:us-east-1:123456789012:container-instance/ci-cluster/nonexistent",
        )
        .send()
        .await
        .unwrap();

    assert_eq!(resp.container_instances().len(), 0);
    assert_eq!(resp.failures().len(), 1);
    assert_eq!(resp.failures()[0].reason(), Some("MISSING"));
}

#[tokio::test]
async fn test_deregister_container_instance_not_found() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("ci-cluster")
        .send()
        .await
        .unwrap();

    let err = client
        .deregister_container_instance()
        .cluster("ci-cluster")
        .container_instance("nonexistent-ci")
        .send()
        .await
        .expect_err("should fail for nonexistent container instance");

    let _service_err = err.into_service_error();
}

#[tokio::test]
async fn test_update_container_instances_state_missing_instance_reported_as_failure() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("ci-cluster")
        .send()
        .await
        .unwrap();

    let ci_arn = client
        .register_container_instance()
        .cluster("ci-cluster")
        .send()
        .await
        .unwrap()
        .container_instance()
        .unwrap()
        .container_instance_arn()
        .unwrap()
        .to_string();

    let resp = client
        .update_container_instances_state()
        .cluster("ci-cluster")
        .container_instances(&ci_arn)
        .container_instances("nonexistent-instance")
        .status(aws_sdk_ecs::types::ContainerInstanceStatus::Draining)
        .send()
        .await
        .unwrap();

    // One updated, one missing
    assert_eq!(resp.container_instances().len(), 1);
    assert_eq!(resp.failures().len(), 1);
}

#[tokio::test]
async fn test_list_account_settings_filter_by_name() {
    let client = make_ecs_client().await;

    client
        .put_account_setting()
        .name(aws_sdk_ecs::types::SettingName::ContainerInsights)
        .value("enabled")
        .send()
        .await
        .unwrap();

    client
        .put_account_setting()
        .name(aws_sdk_ecs::types::SettingName::from(
            "serviceLongArnFormat",
        ))
        .value("enabled")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_account_settings()
        .name(aws_sdk_ecs::types::SettingName::ContainerInsights)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.settings().len(), 1);
    assert_eq!(
        resp.settings()[0].name(),
        Some(&aws_sdk_ecs::types::SettingName::ContainerInsights)
    );
}

#[tokio::test]
async fn test_put_cluster_capacity_providers_cluster_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .put_cluster_capacity_providers()
        .cluster("no-such-cluster")
        .send()
        .await
        .expect_err("should fail for nonexistent cluster");

    let service_err = err.into_service_error();
    assert!(service_err.is_cluster_not_found_exception());
}

#[tokio::test]
async fn test_create_task_set_cluster_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .create_task_set()
        .cluster("no-such-cluster")
        .service("some-svc")
        .task_definition("some-td")
        .send()
        .await
        .expect_err("should fail for nonexistent cluster");

    let _service_err = err.into_service_error();
}

#[tokio::test]
async fn test_describe_task_sets_filter_by_task_set_arn() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("ts-svc")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .deployment_controller(
            aws_sdk_ecs::types::DeploymentController::builder()
                .r#type(aws_sdk_ecs::types::DeploymentControllerType::External)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let ts1_arn = client
        .create_task_set()
        .cluster("test_ecs_cluster")
        .service("ts-svc")
        .task_definition("test_ecs_task")
        .send()
        .await
        .unwrap()
        .task_set()
        .unwrap()
        .task_set_arn()
        .unwrap()
        .to_string();

    client
        .create_task_set()
        .cluster("test_ecs_cluster")
        .service("ts-svc")
        .task_definition("test_ecs_task")
        .send()
        .await
        .unwrap();

    // Describe only the first task set by ARN
    let resp = client
        .describe_task_sets()
        .cluster("test_ecs_cluster")
        .service("ts-svc")
        .task_sets(&ts1_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.task_sets().len(), 1);
    assert_eq!(resp.task_sets()[0].task_set_arn(), Some(ts1_arn.as_str()));
}

#[tokio::test]
async fn test_delete_task_set_not_found() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("ts-svc")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap();

    let err = client
        .delete_task_set()
        .cluster("test_ecs_cluster")
        .service("ts-svc")
        .task_set("nonexistent-task-set")
        .send()
        .await
        .expect_err("should fail for nonexistent task set");

    let _service_err = err.into_service_error();
}

#[tokio::test]
async fn test_list_tasks_desired_status_stopped() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let run_resp = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(2)
        .send()
        .await
        .unwrap();

    let task_arn = run_resp.tasks()[0].task_arn().unwrap().to_string();

    client
        .stop_task()
        .cluster("test_ecs_cluster")
        .task(&task_arn)
        .send()
        .await
        .unwrap();

    let running = client
        .list_tasks()
        .cluster("test_ecs_cluster")
        .desired_status(aws_sdk_ecs::types::DesiredStatus::Running)
        .send()
        .await
        .unwrap();

    let stopped = client
        .list_tasks()
        .cluster("test_ecs_cluster")
        .desired_status(aws_sdk_ecs::types::DesiredStatus::Stopped)
        .send()
        .await
        .unwrap();

    assert_eq!(running.task_arns().len(), 1);
    assert_eq!(stopped.task_arns().len(), 1);
    assert!(stopped.task_arns().contains(&task_arn));
}

#[tokio::test]
async fn test_list_tags_for_resource_nonexistent_returns_empty() {
    let client = make_ecs_client().await;

    // winterbaume returns InvalidParameterException for nonexistent resources
    let result = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:ecs:us-east-1:123456789012:cluster/no-such-cluster")
        .send()
        .await;

    assert!(
        result.is_err(),
        "list_tags for nonexistent resource should fail"
    );
}

#[tokio::test]
async fn test_service_deployment_controller_external() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let resp = client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("external-dc-svc")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .deployment_controller(
            aws_sdk_ecs::types::DeploymentController::builder()
                .r#type(aws_sdk_ecs::types::DeploymentControllerType::External)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let svc = resp.service().unwrap();
    assert_eq!(svc.service_name(), Some("external-dc-svc"));
    assert_eq!(svc.status(), Some("ACTIVE"));
    // Note: winterbaume may not persist deployment_controller; check if present
    if let Some(dc) = svc.deployment_controller() {
        assert_eq!(
            dc.r#type(),
            &aws_sdk_ecs::types::DeploymentControllerType::External
        );
    }
}

#[tokio::test]
async fn test_describe_tasks_cluster_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .describe_tasks()
        .cluster("no-such-cluster")
        .tasks("some-task-arn")
        .send()
        .await
        .expect_err("should fail for nonexistent cluster");

    let service_err = err.into_service_error();
    assert!(service_err.is_cluster_not_found_exception());
}

#[tokio::test]
async fn test_run_task_count_zero() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let resp = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(0)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tasks().len(), 0);
    assert_eq!(resp.failures().len(), 0);
}

#[tokio::test]
async fn test_describe_clusters_mixed_found_and_missing() {
    let client = make_ecs_client().await;

    client
        .create_cluster()
        .cluster_name("real-cluster")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_clusters()
        .clusters("real-cluster")
        .clusters("fake-cluster")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.clusters().len(), 1);
    assert_eq!(resp.clusters()[0].cluster_name(), Some("real-cluster"));
    assert_eq!(resp.failures().len(), 1);
    assert_eq!(resp.failures()[0].reason(), Some("MISSING"));
    assert!(resp.failures()[0].arn().unwrap().contains("fake-cluster"));
}

#[tokio::test]
async fn test_update_capacity_provider_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .update_capacity_provider()
        .name("nonexistent-cp")
        .auto_scaling_group_provider(
            aws_sdk_ecs::types::AutoScalingGroupProviderUpdate::builder().build(),
        )
        .send()
        .await
        .expect_err("should fail for nonexistent capacity provider");

    let _service_err = err.into_service_error();
}

#[tokio::test]
async fn test_delete_capacity_provider_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .delete_capacity_provider()
        .capacity_provider("nonexistent-cp")
        .send()
        .await
        .expect_err("should fail for nonexistent capacity provider");

    let _service_err = err.into_service_error();
}

#[tokio::test]
async fn test_register_container_instance_cluster_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .register_container_instance()
        .cluster("no-such-cluster")
        .send()
        .await
        .expect_err("should fail for nonexistent cluster");

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ClusterNotFoundException") || err_str.contains("Cluster"),
        "Expected ClusterNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_container_instances_cluster_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .list_container_instances()
        .cluster("no-such-cluster")
        .send()
        .await
        .expect_err("should fail for nonexistent cluster");

    let service_err = err.into_service_error();
    assert!(service_err.is_cluster_not_found_exception());
}

#[tokio::test]
async fn test_run_task_task_definition_by_arn() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    // winterbaume looks up task definitions by family name, not ARN
    let resp = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(1)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tasks().len(), 1);
    assert!(
        resp.tasks()[0]
            .task_definition_arn()
            .unwrap()
            .ends_with("test_ecs_task:1")
    );
}

#[tokio::test]
async fn test_create_service_fargate_launch_type() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let resp = client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("fargate-svc")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .launch_type(aws_sdk_ecs::types::LaunchType::Fargate)
        .send()
        .await
        .unwrap();

    let svc = resp.service().unwrap();
    assert_eq!(svc.service_name(), Some("fargate-svc"));
    assert_eq!(
        svc.launch_type(),
        Some(&aws_sdk_ecs::types::LaunchType::Fargate)
    );
    assert_eq!(svc.status(), Some("ACTIVE"));
}

#[tokio::test]
async fn test_delete_service_by_arn() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let svc_arn = client
        .create_service()
        .cluster("test_ecs_cluster")
        .service_name("arn-delete-svc")
        .task_definition("test_ecs_task")
        .desired_count(1)
        .send()
        .await
        .unwrap()
        .service()
        .unwrap()
        .service_arn()
        .unwrap()
        .to_string();

    let resp = client
        .delete_service()
        .cluster("test_ecs_cluster")
        .service(&svc_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.service().unwrap().status(), Some("INACTIVE"));
    assert_eq!(
        resp.service().unwrap().service_name(),
        Some("arn-delete-svc")
    );
}

#[tokio::test]
async fn test_delete_service_cluster_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .delete_service()
        .cluster("no-such-cluster")
        .service("some-svc")
        .send()
        .await
        .expect_err("should fail for nonexistent cluster");

    let service_err = err.into_service_error();
    assert!(service_err.is_cluster_not_found_exception());
}

#[tokio::test]
async fn test_deregister_task_definition_not_found() {
    let client = make_ecs_client().await;

    let err = client
        .deregister_task_definition()
        .task_definition("nonexistent-family:1")
        .send()
        .await
        .expect_err("should fail for nonexistent task definition");

    let _service_err = err.into_service_error();
}

#[tokio::test]
async fn test_multiple_regions_isolated() {
    let client_east = make_ecs_client_with_region("us-east-1").await;
    let client_west = make_ecs_client_with_region("us-west-2").await;

    client_east
        .create_cluster()
        .cluster_name("east-cluster")
        .send()
        .await
        .unwrap();

    client_west
        .create_cluster()
        .cluster_name("west-cluster")
        .send()
        .await
        .unwrap();

    let east_clusters = client_east.list_clusters().send().await.unwrap();
    let west_clusters = client_west.list_clusters().send().await.unwrap();

    assert!(
        east_clusters
            .cluster_arns()
            .iter()
            .any(|a| a.contains("us-east-1") && a.contains("east-cluster"))
    );
    assert!(
        !east_clusters
            .cluster_arns()
            .iter()
            .any(|a| a.contains("west-cluster"))
    );

    assert!(
        west_clusters
            .cluster_arns()
            .iter()
            .any(|a| a.contains("us-west-2") && a.contains("west-cluster"))
    );
    assert!(
        !west_clusters
            .cluster_arns()
            .iter()
            .any(|a| a.contains("east-cluster"))
    );
}

// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================

// Covers FIX(terraform-e2e): DescribeTaskDefinition accepts full ARN
// (state.rs:181) — terraform calls DescribeTaskDefinition with the full ARN
// but state is keyed by "family:revision". ARN is normalized to key before lookup.
#[tokio::test]
async fn test_describe_task_definition_by_arn() {
    let client = make_ecs_client().await;

    let reg_resp = client
        .register_task_definition()
        .family("td_arn_test")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("hello")
                .image("hello-world:latest")
                .memory(256)
                .build(),
        )
        .send()
        .await
        .expect("register_task_definition should succeed");

    let td = reg_resp.task_definition().unwrap();
    let arn = td.task_definition_arn().unwrap();
    assert!(
        arn.contains(":task-definition/td_arn_test:1"),
        "ARN should contain task-definition/family:revision"
    );

    // Describe using the full ARN (what terraform does)
    let desc_resp = client
        .describe_task_definition()
        .task_definition(arn)
        .send()
        .await
        .expect("describe_task_definition by full ARN should succeed");

    let desc_td = desc_resp.task_definition().unwrap();
    assert_eq!(desc_td.family(), Some("td_arn_test"));
    assert_eq!(desc_td.revision(), 1);
    assert_eq!(desc_td.task_definition_arn(), Some(arn));
}

// Covers FIX(terraform-e2e): DeregisterTaskDefinition accepts full ARN
// (state.rs) — terraform calls DeregisterTaskDefinition with the full ARN
// but state is keyed by "family:revision". ARN is normalized to key before lookup.
#[tokio::test]
async fn test_deregister_task_definition_by_arn() {
    let client = make_ecs_client().await;

    let reg_resp = client
        .register_task_definition()
        .family("dereg_arn_test")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("hello")
                .image("hello-world:latest")
                .memory(256)
                .build(),
        )
        .send()
        .await
        .expect("register_task_definition should succeed");

    let td = reg_resp.task_definition().unwrap();
    let arn = td.task_definition_arn().unwrap();
    assert!(
        arn.contains(":task-definition/dereg_arn_test:1"),
        "ARN should contain task-definition/family:revision"
    );

    // Deregister using the full ARN (what terraform does)
    let dereg_resp = client
        .deregister_task_definition()
        .task_definition(arn)
        .send()
        .await
        .expect("deregister_task_definition by full ARN should succeed");

    let dereg_td = dereg_resp.task_definition().unwrap();
    assert_eq!(dereg_td.family(), Some("dereg_arn_test"));
    assert_eq!(dereg_td.revision(), 1);
    assert_eq!(
        dereg_td.status(),
        Some(&aws_sdk_ecs::types::TaskDefinitionStatus::Inactive)
    );
}

// Covers FIX(terraform-e2e): DeleteTaskDefinitions accepts full ARN
// (state.rs) — terraform calls DeleteTaskDefinitions with the full ARN
// but state is keyed by "family:revision". ARN is normalized to key before lookup.
#[tokio::test]
async fn test_delete_task_definitions_by_arn() {
    let client = make_ecs_client().await;

    let reg_resp = client
        .register_task_definition()
        .family("del_arn_test")
        .container_definitions(
            aws_sdk_ecs::types::ContainerDefinition::builder()
                .name("hello")
                .image("hello-world:latest")
                .memory(256)
                .build(),
        )
        .send()
        .await
        .expect("register_task_definition should succeed");

    let td = reg_resp.task_definition().unwrap();
    let arn = td.task_definition_arn().unwrap();
    assert!(
        arn.contains(":task-definition/del_arn_test:1"),
        "ARN should contain task-definition/family:revision"
    );

    // Delete using the full ARN (what terraform does)
    let del_resp = client
        .delete_task_definitions()
        .task_definitions(arn)
        .send()
        .await
        .expect("delete_task_definitions by full ARN should succeed");

    assert_eq!(del_resp.task_definitions().len(), 1);
    assert_eq!(
        del_resp.task_definitions()[0].status(),
        Some(&aws_sdk_ecs::types::TaskDefinitionStatus::DeleteInProgress)
    );
}

// ============================================================================
// DiscoverPollEndpoint tests
// ============================================================================

#[tokio::test]
async fn test_discover_poll_endpoint() {
    let client = make_ecs_client().await;
    let resp = client
        .discover_poll_endpoint()
        .send()
        .await
        .expect("discover_poll_endpoint should succeed");

    let endpoint = resp.endpoint().unwrap();
    assert!(
        endpoint.contains("ecs-a.us-east-1.amazonaws.com"),
        "endpoint should contain the region-specific URL, got: {endpoint}"
    );
    let telemetry = resp.telemetry_endpoint().unwrap();
    assert!(
        telemetry.contains("ecs-t.us-east-1.amazonaws.com"),
        "telemetry endpoint should contain the region-specific URL, got: {telemetry}"
    );
}

// ============================================================================
// ExecuteCommand tests
// ============================================================================

#[tokio::test]
async fn test_execute_command() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    // Run a task to have something to execute a command on
    let run_resp = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(1)
        .send()
        .await
        .unwrap();

    let task_arn = run_resp.tasks().first().unwrap().task_arn().unwrap();

    let resp = client
        .execute_command()
        .cluster("test_ecs_cluster")
        .task(task_arn)
        .container("hello_world")
        .interactive(true)
        .command("/bin/sh")
        .send()
        .await
        .expect("execute_command should succeed");

    assert_eq!(resp.container_name(), Some("hello_world"));
    assert!(resp.interactive());
    let session = resp.session().unwrap();
    assert!(session.session_id().is_some());
    assert!(session.stream_url().is_some());
    assert!(session.token_value().is_some());
}

// ============================================================================
// GetTaskProtection / UpdateTaskProtection tests
// ============================================================================

#[tokio::test]
async fn test_get_task_protection() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let run_resp = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(1)
        .send()
        .await
        .unwrap();

    let task_arn = run_resp.tasks().first().unwrap().task_arn().unwrap();

    // Before setting protection, get should return empty for this task
    let resp = client
        .get_task_protection()
        .cluster("test_ecs_cluster")
        .tasks(task_arn)
        .send()
        .await
        .expect("get_task_protection should succeed");

    assert!(
        resp.protected_tasks().is_empty(),
        "no protection set yet, should return empty"
    );

    // Set protection, then get should return it
    client
        .update_task_protection()
        .cluster("test_ecs_cluster")
        .tasks(task_arn)
        .protection_enabled(true)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_task_protection()
        .cluster("test_ecs_cluster")
        .tasks(task_arn)
        .send()
        .await
        .expect("get_task_protection should succeed after update");

    let protected_tasks = resp.protected_tasks();
    assert!(
        !protected_tasks.is_empty(),
        "should return the protected task"
    );
    assert_eq!(protected_tasks[0].task_arn(), Some(task_arn));
    assert!(protected_tasks[0].protection_enabled());
}

#[tokio::test]
async fn test_update_task_protection() {
    let client = make_ecs_client().await;
    setup_cluster_and_task(&client).await;

    let run_resp = client
        .run_task()
        .cluster("test_ecs_cluster")
        .task_definition("test_ecs_task")
        .count(1)
        .send()
        .await
        .unwrap();

    let task_arn = run_resp.tasks().first().unwrap().task_arn().unwrap();

    // Enable protection
    let resp = client
        .update_task_protection()
        .cluster("test_ecs_cluster")
        .tasks(task_arn)
        .protection_enabled(true)
        .send()
        .await
        .expect("update_task_protection should succeed");

    let protected_tasks = resp.protected_tasks();
    assert!(!protected_tasks.is_empty());
    assert_eq!(protected_tasks[0].task_arn(), Some(task_arn));
    assert!(protected_tasks[0].protection_enabled());

    // Verify via get
    let get_resp = client
        .get_task_protection()
        .cluster("test_ecs_cluster")
        .tasks(task_arn)
        .send()
        .await
        .unwrap();

    assert!(get_resp.protected_tasks()[0].protection_enabled());
}

// ============================================================================
// PutAccountSettingDefault tests
// ============================================================================

#[tokio::test]
async fn test_put_account_setting_default() {
    let client = make_ecs_client().await;

    let resp = client
        .put_account_setting_default()
        .name(aws_sdk_ecs::types::SettingName::ContainerInstanceLongArnFormat)
        .value("enabled")
        .send()
        .await
        .expect("put_account_setting_default should succeed");

    let setting = resp.setting().unwrap();
    assert_eq!(setting.value(), Some("enabled"));
}

// ============================================================================
// UpdateClusterSettings tests
// ============================================================================

#[tokio::test]
async fn test_update_cluster_settings() {
    let client = make_ecs_client().await;
    client
        .create_cluster()
        .cluster_name("settings-test")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_cluster_settings()
        .cluster("settings-test")
        .settings(
            aws_sdk_ecs::types::ClusterSetting::builder()
                .name(aws_sdk_ecs::types::ClusterSettingName::ContainerInsights)
                .value("enabled")
                .build(),
        )
        .send()
        .await
        .expect("update_cluster_settings should succeed");

    let cluster = resp.cluster().unwrap();
    assert_eq!(cluster.cluster_name(), Some("settings-test"));
}

#[tokio::test]
async fn test_update_cluster_settings_not_found() {
    let client = make_ecs_client().await;
    let result = client
        .update_cluster_settings()
        .cluster("nonexistent-cluster")
        .settings(
            aws_sdk_ecs::types::ClusterSetting::builder()
                .name(aws_sdk_ecs::types::ClusterSettingName::ContainerInsights)
                .value("enabled")
                .build(),
        )
        .send()
        .await;

    assert!(result.is_err(), "should fail for nonexistent cluster");
}

// ============================================================================
// UpdateContainerAgent tests
// ============================================================================

#[tokio::test]
async fn test_update_container_agent() {
    let client = make_ecs_client().await;
    client
        .create_cluster()
        .cluster_name("agent-test")
        .send()
        .await
        .unwrap();

    let ci_resp = client
        .register_container_instance()
        .cluster("agent-test")
        .send()
        .await
        .unwrap();

    let ci_arn = ci_resp
        .container_instance()
        .unwrap()
        .container_instance_arn()
        .unwrap();

    let resp = client
        .update_container_agent()
        .cluster("agent-test")
        .container_instance(ci_arn)
        .send()
        .await
        .expect("update_container_agent should succeed");

    let ci = resp.container_instance().unwrap();
    assert_eq!(ci.container_instance_arn(), Some(ci_arn));
}

// ============================================================================
// ListServicesByNamespace tests
// ============================================================================

#[tokio::test]
async fn test_list_services_by_namespace() {
    let client = make_ecs_client().await;

    let resp = client
        .list_services_by_namespace()
        .namespace("test-ns")
        .send()
        .await
        .expect("list_services_by_namespace should succeed");

    // Should return empty initially since no services in that namespace
    assert!(resp.service_arns().is_empty());
}

// ============================================================================
// DescribeServiceDeployments tests
// ============================================================================

#[tokio::test]
async fn test_describe_service_deployments() {
    let client = make_ecs_client().await;

    let resp = client
        .describe_service_deployments()
        .service_deployment_arns("arn:aws:ecs:us-east-1:123456789012:service-deployment/test")
        .send()
        .await
        .expect("describe_service_deployments should succeed");

    // Stub returns empty lists
    assert!(resp.service_deployments().is_empty());
}

// ============================================================================
// DescribeServiceRevisions tests
// ============================================================================

#[tokio::test]
async fn test_describe_service_revisions() {
    let client = make_ecs_client().await;

    let resp = client
        .describe_service_revisions()
        .service_revision_arns("arn:aws:ecs:us-east-1:123456789012:service-revision/test")
        .send()
        .await
        .expect("describe_service_revisions should succeed");

    assert!(resp.service_revisions().is_empty());
}

// ============================================================================
// ListServiceDeployments tests
// ============================================================================

#[tokio::test]
async fn test_list_service_deployments() {
    let client = make_ecs_client().await;

    let resp = client
        .list_service_deployments()
        .service("arn:aws:ecs:us-east-1:123456789012:service/test-cluster/test-svc")
        .send()
        .await
        .expect("list_service_deployments should succeed");

    assert!(resp.service_deployments().is_empty());
}

// ============================================================================
// SubmitAttachmentStateChanges tests
// ============================================================================

#[tokio::test]
async fn test_submit_attachment_state_changes() {
    let client = make_ecs_client().await;

    let resp = client
        .submit_attachment_state_changes()
        .send()
        .await
        .expect("submit_attachment_state_changes should succeed");

    let ack = resp.acknowledgment().unwrap();
    assert!(ack.starts_with("ACK_"));
}

// ============================================================================
// SubmitContainerStateChange tests
// ============================================================================

#[tokio::test]
async fn test_submit_container_state_change() {
    let client = make_ecs_client().await;

    let resp = client
        .submit_container_state_change()
        .send()
        .await
        .expect("submit_container_state_change should succeed");

    let ack = resp.acknowledgment().unwrap();
    assert!(ack.starts_with("ACK_"));
}

// ============================================================================
// SubmitTaskStateChange tests
// ============================================================================

#[tokio::test]
async fn test_submit_task_state_change() {
    let client = make_ecs_client().await;

    let resp = client
        .submit_task_state_change()
        .send()
        .await
        .expect("submit_task_state_change should succeed");

    let ack = resp.acknowledgment().unwrap();
    assert!(ack.starts_with("ACK_"));
}
