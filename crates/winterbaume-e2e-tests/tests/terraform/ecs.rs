use crate::harness::*;

// ECS terraform resources tested here:
//   aws_ecs_cluster
//   aws_ecs_task_definition
//   aws_ecs_service (requires cluster + task definition)
//   aws_ecs_cluster_capacity_providers

// ---------------------------------------------------------------------------
// Smoke test (converted from smoke_test_terraform to batch_apply)
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn debug_ecs_smoke() {
    // FIX(terraform-e2e): Was a smoke_test_terraform call (own server + init),
    //   which timed out because provider startup takes ~35 s against the large
    //   AWS schema. Converted to batch_apply which reuses the shared provider.
    let result = batch_apply(
        r#"
resource "aws_ecs_cluster" "debug_ecs_smoke" {
  name = "ecs-smoke-cluster"
}
"#,
    )
    .await;
    assert!(result.success, "apply failed:\n{}", result.stderr);
    assert!(result.state.contains("ecs-smoke-cluster"));
}

// ---------------------------------------------------------------------------
// Cluster tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecs_cluster_basic() {
    let result = batch_apply(
        r#"
resource "aws_ecs_cluster" "ecs_cluster_basic" {
  name = "ecs-cluster-basic"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("ecs-cluster-basic"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecs_cluster_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_ecs_cluster" "ecs_cluster_with_tags" {
  name = "ecs-cluster-tags"

  tags = {
    Environment = "test"
    Name        = "ecs-cluster-tags"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("ecs-cluster-tags"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecs_cluster_with_settings() {
    let result = batch_apply(
        r#"
resource "aws_ecs_cluster" "ecs_cluster_with_settings" {
  name = "ecs-cluster-settings"

  setting {
    name  = "containerInsights"
    value = "enabled"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("ecs-cluster-settings"));
}

// ---------------------------------------------------------------------------
// Task definition tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecs_task_definition_basic() {
    let result = batch_apply(
        r#"
resource "aws_ecs_task_definition" "ecs_task_definition_basic" {
  family                   = "ecs-task-def-basic"
  requires_compatibilities = ["FARGATE"]
  network_mode             = "awsvpc"
  cpu                      = 256
  memory                   = 512

  container_definitions = jsonencode([
    {
      name      = "app"
      image     = "nginx:latest"
      essential = true
      portMappings = [
        {
          containerPort = 80
          protocol      = "tcp"
        }
      ]
    }
  ])
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("ecs-task-def-basic"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecs_task_definition_ec2() {
    let result = batch_apply(
        r#"
resource "aws_ecs_task_definition" "ecs_task_definition_ec2" {
  family       = "ecs-task-def-ec2"
  network_mode = "bridge"

  container_definitions = jsonencode([
    {
      name      = "worker"
      image     = "alpine:latest"
      essential = true
      memory    = 128
      cpu       = 64
    }
  ])
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("ecs-task-def-ec2"));
}

// ---------------------------------------------------------------------------
// Service tests
// ---------------------------------------------------------------------------

// Note: aws_ecs_service uses an isolated server to avoid batch cascade issues.
// The ECS service response includes network_configuration, which terraform stores
// in state. If service_json doesn't return it, drift is detected on subsequent
// batch waves and UpdateService calls can cascade into other tests failing.
// Using an isolated server ensures the service resource stays out of shared state.

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecs_service_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ecs-service-basic");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_ecs_cluster" "test" {
  name = "ecs-service-cluster"
}

resource "aws_ecs_task_definition" "test" {
  family                   = "ecs-service-task"
  requires_compatibilities = ["FARGATE"]
  network_mode             = "awsvpc"
  cpu                      = 256
  memory                   = 512

  container_definitions = jsonencode([
    {
      name      = "app"
      image     = "nginx:latest"
      essential = true
    }
  ])
}

resource "aws_ecs_service" "test" {
  name            = "ecs-service-basic"
  cluster         = aws_ecs_cluster.test.id
  task_definition = aws_ecs_task_definition.test.arn
  desired_count   = 0
  launch_type     = "FARGATE"

  network_configuration {
    subnets = ["subnet-00000000"]
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("ecs-service-basic"));

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Full-stack test
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecs_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_ecs_cluster" "ecs_full_stack_cluster" {
  name = "ecs-full-stack-cluster"

  setting {
    name  = "containerInsights"
    value = "disabled"
  }

  tags = {
    Stack = "ecs-full-stack"
  }
}

resource "aws_ecs_task_definition" "ecs_full_stack_td" {
  family                   = "ecs-full-stack-task"
  requires_compatibilities = ["FARGATE"]
  network_mode             = "awsvpc"
  cpu                      = 512
  memory                   = 1024

  container_definitions = jsonencode([
    {
      name      = "web"
      image     = "nginx:latest"
      essential = true
      portMappings = [
        {
          containerPort = 80
          protocol      = "tcp"
        }
      ]
      logConfiguration = {
        logDriver = "awslogs"
        options = {
          "awslogs-group"  = "/ecs/ecs-full-stack"
          "awslogs-region" = "us-east-1"
        }
      }
    }
  ])
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("ecs-full-stack-cluster"));
    assert!(result.state.contains("ecs-full-stack-task"));
}
