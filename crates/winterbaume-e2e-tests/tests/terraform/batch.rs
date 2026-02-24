use crate::harness::*;

// Batch terraform resources tested here:
//   aws_batch_compute_environment  (CreateComputeEnvironment, DescribeComputeEnvironments,
//                                    UpdateComputeEnvironment, DeleteComputeEnvironment)
//   aws_batch_job_queue            (CreateJobQueue, DescribeJobQueues, UpdateJobQueue,
//                                    DeleteJobQueue)
//   aws_batch_job_definition       (RegisterJobDefinition, DescribeJobDefinitions,
//                                    DeregisterJobDefinition)
//   aws_batch_scheduling_policy    (CreateSchedulingPolicy, DescribeSchedulingPolicies,
//                                    UpdateSchedulingPolicy, DeleteSchedulingPolicy)

// ---------------------------------------------------------------------------
// aws_batch_compute_environment
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_batch_compute_environment_managed() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("batch-ce-managed");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_batch_compute_environment" "batch_ce_managed" {
  name  = "batch-ce-managed"
  type  = "MANAGED"
  state = "ENABLED"

  compute_resources {
    type      = "FARGATE"
    max_vcpus = 16

    subnets = [
      "subnet-00000001"
    ]

    security_group_ids = [
      "sg-00000001"
    ]
  }

  service_role = "arn:aws:iam::123456789012:role/aws-batch-service-role"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("batch-ce-managed"),
        "state missing compute environment"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_batch_compute_environment_unmanaged() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("batch-ce-unmanaged");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_batch_compute_environment" "batch_ce_unmanaged" {
  name  = "batch-ce-unmanaged"
  type  = "UNMANAGED"
  state  = "ENABLED"

  service_role = "arn:aws:iam::123456789012:role/aws-batch-service-role"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("batch-ce-unmanaged"),
        "state missing compute environment"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_batch_scheduling_policy
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_batch_scheduling_policy_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("batch-sp-basic");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_batch_scheduling_policy" "batch_sp_basic" {
  name = "batch-sp-basic"

  fair_share_policy {
    compute_reservation = 1
    share_decay_seconds = 3600

    share_distribution {
      share_identifier = "A1*"
      weight_factor    = 0.1
    }
  }

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("batch-sp-basic"),
        "state missing scheduling policy"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_batch_job_queue
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_batch_job_queue_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("batch-jq-basic");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_batch_compute_environment" "jq_ce" {
  name  = "batch-jq-ce"
  type  = "MANAGED"
  state = "ENABLED"

  compute_resources {
    type      = "FARGATE"
    max_vcpus = 16

    subnets = [
      "subnet-00000001"
    ]

    security_group_ids = [
      "sg-00000001"
    ]
  }

  service_role = "arn:aws:iam::123456789012:role/aws-batch-service-role"
}

resource "aws_batch_job_queue" "batch_jq_basic" {
  name     = "batch-jq-basic"
  state    = "ENABLED"
  priority = 1

  compute_environment_order {
    order               = 1
    compute_environment = aws_batch_compute_environment.jq_ce.arn
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("batch-jq-basic"), "state missing job queue");

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_batch_job_queue_with_scheduling_policy() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("batch-jq-sp");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_batch_compute_environment" "jq_sp_ce" {
  name  = "batch-jq-sp-ce"
  type  = "MANAGED"
  state = "ENABLED"

  compute_resources {
    type      = "FARGATE"
    max_vcpus = 16

    subnets = [
      "subnet-00000001"
    ]

    security_group_ids = [
      "sg-00000001"
    ]
  }

  service_role = "arn:aws:iam::123456789012:role/aws-batch-service-role"
}

resource "aws_batch_scheduling_policy" "jq_sp" {
  name = "batch-jq-sp-policy"

  fair_share_policy {
    compute_reservation = 1
    share_decay_seconds = 3600

    share_distribution {
      share_identifier = "A1*"
      weight_factor    = 0.1
    }
  }
}

resource "aws_batch_job_queue" "batch_jq_sp" {
  name                  = "batch-jq-with-sp"
  state                 = "ENABLED"
  priority              = 10
  scheduling_policy_arn = aws_batch_scheduling_policy.jq_sp.arn

  compute_environment_order {
    order               = 1
    compute_environment = aws_batch_compute_environment.jq_sp_ce.arn
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("batch-jq-with-sp"),
        "state missing job queue"
    );
    assert!(
        state.contains("batch-jq-sp-policy"),
        "state missing scheduling policy"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_batch_job_definition
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_batch_job_definition_container() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("batch-jd-container");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_batch_job_definition" "batch_jd_container" {
  name = "batch-jd-container"
  type = "container"

  platform_capabilities = ["FARGATE"]

  container_properties = jsonencode({
    image   = "busybox:latest"
    command = ["echo", "hello"]

    fargatePlatformConfiguration = {
      platformVersion = "LATEST"
    }

    resourceRequirements = [
      {
        type  = "VCPU"
        value = "0.25"
      },
      {
        type  = "MEMORY"
        value = "512"
      }
    ]

    executionRoleArn = "arn:aws:iam::123456789012:role/ecsTaskExecutionRole"
  })

  tags = {
    Name = "batch-jd-container"
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("batch-jd-container"),
        "state missing job definition"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Full-stack test: compute env + scheduling policy + job queue + job definition
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_batch_full_stack() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("batch-full-stack");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_batch_compute_environment" "full_ce" {
  name  = "batch-full-ce"
  type  = "MANAGED"
  state = "ENABLED"

  compute_resources {
    type      = "FARGATE"
    max_vcpus = 16

    subnets = [
      "subnet-00000001"
    ]

    security_group_ids = [
      "sg-00000001"
    ]
  }

  service_role = "arn:aws:iam::123456789012:role/aws-batch-service-role"
}

resource "aws_batch_scheduling_policy" "full_sp" {
  name = "batch-full-sp"

  fair_share_policy {
    compute_reservation = 0
    share_decay_seconds = 0
  }
}

resource "aws_batch_job_queue" "full_jq" {
  name                  = "batch-full-jq"
  state                 = "ENABLED"
  priority              = 1
  scheduling_policy_arn = aws_batch_scheduling_policy.full_sp.arn

  compute_environment_order {
    order               = 1
    compute_environment = aws_batch_compute_environment.full_ce.arn
  }
}

resource "aws_batch_job_definition" "full_jd" {
  name = "batch-full-jd"
  type = "container"

  platform_capabilities = ["FARGATE"]

  container_properties = jsonencode({
    image   = "busybox:latest"
    command = ["echo", "hello world"]

    fargatePlatformConfiguration = {
      platformVersion = "LATEST"
    }

    resourceRequirements = [
      {
        type  = "VCPU"
        value = "0.25"
      },
      {
        type  = "MEMORY"
        value = "512"
      }
    ]

    executionRoleArn = "arn:aws:iam::123456789012:role/ecsTaskExecutionRole"
  })

  tags = {
    Stack = "batch-full-stack"
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("batch-full-ce"),
        "state missing compute environment"
    );
    assert!(
        state.contains("batch-full-sp"),
        "state missing scheduling policy"
    );
    assert!(state.contains("batch-full-jq"), "state missing job queue");
    assert!(
        state.contains("batch-full-jd"),
        "state missing job definition"
    );

    // Verify destroy works cleanly
    let (ok, _stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Destroy lifecycle test
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_batch_compute_environment_destroy() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("batch-ce-destroy");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_batch_compute_environment" "batch_ce_destroy" {
  name  = "batch-ce-destroy"
  type  = "MANAGED"
  state = "ENABLED"

  compute_resources {
    type      = "FARGATE"
    max_vcpus = 4

    subnets = [
      "subnet-00000001"
    ]

    security_group_ids = [
      "sg-00000001"
    ]
  }

  service_role = "arn:aws:iam::123456789012:role/aws-batch-service-role"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    // Now destroy
    let (ok, _stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");

    cleanup_tf_dir(&dir);
}
