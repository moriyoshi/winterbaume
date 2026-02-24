use crate::harness::*;

// Auto Scaling terraform resources tested here:
//   aws_autoscaling_group          (CreateAutoScalingGroup, DescribeAutoScalingGroups,
//                                    UpdateAutoScalingGroup, DeleteAutoScalingGroup,
//                                    CreateOrUpdateTags, DescribeTags)
//   aws_launch_configuration       (CreateLaunchConfiguration, DescribeLaunchConfigurations,
//                                    DeleteLaunchConfiguration)
//   aws_autoscaling_policy         (PutScalingPolicy, DescribePolicies, DeletePolicy)
//   aws_autoscaling_schedule       (PutScheduledUpdateGroupAction, DescribeScheduledActions,
//                                    DeleteScheduledAction)
//   aws_autoscaling_lifecycle_hook (PutLifecycleHook, DescribeLifecycleHooks,
//                                    DeleteLifecycleHook)

// ---------------------------------------------------------------------------
// Smoke test
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn debug_autoscaling_smoke() {
    let result = smoke_test_terraform(
        test_services(),
        r#"
resource "aws_launch_configuration" "smoke" {
  name          = "asg-smoke-lc"
  image_id      = "ami-00000000"
  instance_type = "t2.micro"
}

resource "aws_autoscaling_group" "smoke" {
  name                 = "asg-smoke"
  launch_configuration = aws_launch_configuration.smoke.name
  min_size             = 0
  max_size             = 1
  availability_zones   = ["us-east-1a"]
}
"#,
        std::time::Duration::from_secs(90),
    )
    .await;
    eprintln!("{result}");
    for call in &result.api_calls {
        eprintln!("  {} -> {}", call.operation, call.status_code);
    }
    eprintln!("Missing ops: {:?}", result.missing_operations);
    eprintln!("Failed ops: {:?}", result.failed_operations);
    assert!(result.success, "smoke test failed:\n{}", result.stderr);
}

// ---------------------------------------------------------------------------
// aws_launch_configuration
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_autoscaling_launch_config_basic() {
    let result = batch_apply(
        r#"
resource "aws_launch_configuration" "autoscaling_launch_config_basic" {
  name          = "asg-lc-basic"
  image_id      = "ami-00000000"
  instance_type = "t2.micro"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("asg-lc-basic"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_autoscaling_launch_config_with_details() {
    let result = batch_apply(
        r#"
resource "aws_launch_configuration" "autoscaling_launch_config_with_details" {
  name              = "asg-lc-details"
  image_id          = "ami-00000001"
  instance_type     = "t3.small"
  enable_monitoring = true
  key_name          = "my-key"

  root_block_device {
    volume_size = 20
    volume_type = "gp2"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("asg-lc-details"));
}

// ---------------------------------------------------------------------------
// aws_autoscaling_group — basic
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_autoscaling_group_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("asg-group-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_launch_configuration" "test" {
  name          = "asg-group-basic-lc"
  image_id      = "ami-00000000"
  instance_type = "t2.micro"
}

resource "aws_autoscaling_group" "test" {
  name                 = "asg-group-basic"
  launch_configuration = aws_launch_configuration.test.name
  min_size             = 0
  max_size             = 2
  desired_capacity     = 0
  availability_zones   = ["us-east-1a"]
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("asg-group-basic"), "state missing ASG name");
    assert!(
        state.contains("asg-group-basic-lc"),
        "state missing launch configuration"
    );
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_autoscaling_group — with tags
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_autoscaling_group_with_tags() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("asg-group-tags");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_launch_configuration" "test" {
  name          = "asg-group-tags-lc"
  image_id      = "ami-00000000"
  instance_type = "t2.micro"
}

resource "aws_autoscaling_group" "test" {
  name                 = "asg-group-tags"
  launch_configuration = aws_launch_configuration.test.name
  min_size             = 0
  max_size             = 1
  availability_zones   = ["us-east-1a"]

  tag {
    key                 = "Name"
    value               = "asg-group-tags-test"
    propagate_at_launch = true
  }

  tag {
    key                 = "Environment"
    value               = "testing"
    propagate_at_launch = false
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("asg-group-tags"), "state missing ASG name");
    assert!(
        state.contains("asg-group-tags-test"),
        "state missing tag Name"
    );
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_autoscaling_group — with VPC zone identifier
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_autoscaling_group_vpc_zone() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("asg-group-vpc-zone");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_launch_configuration" "test" {
  name          = "asg-group-vpc-zone-lc"
  image_id      = "ami-00000000"
  instance_type = "t2.micro"
}

resource "aws_autoscaling_group" "test" {
  name                 = "asg-group-vpc-zone"
  launch_configuration = aws_launch_configuration.test.name
  min_size             = 0
  max_size             = 1
  vpc_zone_identifier  = ["subnet-12345678", "subnet-87654321"]
  health_check_type    = "ELB"
  health_check_grace_period = 300
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("asg-group-vpc-zone"),
        "state missing ASG name"
    );
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_autoscaling_policy
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_autoscaling_policy_simple() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("asg-policy-simple");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_launch_configuration" "test" {
  name          = "asg-policy-simple-lc"
  image_id      = "ami-00000000"
  instance_type = "t2.micro"
}

resource "aws_autoscaling_group" "test" {
  name                 = "asg-policy-simple-asg"
  launch_configuration = aws_launch_configuration.test.name
  min_size             = 0
  max_size             = 4
  availability_zones   = ["us-east-1a"]
}

resource "aws_autoscaling_policy" "test" {
  name                   = "asg-policy-simple"
  autoscaling_group_name = aws_autoscaling_group.test.name
  adjustment_type        = "ChangeInCapacity"
  scaling_adjustment     = 1
  cooldown               = 300
  policy_type            = "SimpleScaling"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("asg-policy-simple"),
        "state missing scaling policy"
    );
    assert!(
        state.contains("aws_autoscaling_policy"),
        "state missing autoscaling_policy resource"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_autoscaling_policy_target_tracking() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("asg-policy-target");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_launch_configuration" "test" {
  name          = "asg-policy-target-lc"
  image_id      = "ami-00000000"
  instance_type = "t2.micro"
}

resource "aws_autoscaling_group" "test" {
  name                 = "asg-policy-target-asg"
  launch_configuration = aws_launch_configuration.test.name
  min_size             = 0
  max_size             = 4
  availability_zones   = ["us-east-1a"]
}

resource "aws_autoscaling_policy" "test" {
  name                   = "asg-policy-target-tracking"
  autoscaling_group_name = aws_autoscaling_group.test.name
  policy_type            = "TargetTrackingScaling"

  target_tracking_configuration {
    predefined_metric_specification {
      predefined_metric_type = "ASGAverageCPUUtilization"
    }
    target_value = 50.0
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
        state.contains("asg-policy-target-tracking"),
        "state missing target tracking policy"
    );
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_autoscaling_schedule
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_autoscaling_schedule_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("asg-schedule-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_launch_configuration" "test" {
  name          = "asg-schedule-basic-lc"
  image_id      = "ami-00000000"
  instance_type = "t2.micro"
}

resource "aws_autoscaling_group" "test" {
  name                 = "asg-schedule-basic-asg"
  launch_configuration = aws_launch_configuration.test.name
  min_size             = 0
  max_size             = 4
  availability_zones   = ["us-east-1a"]
}

resource "aws_autoscaling_schedule" "test" {
  scheduled_action_name  = "asg-schedule-basic"
  autoscaling_group_name = aws_autoscaling_group.test.name
  min_size               = 1
  max_size               = 4
  desired_capacity       = 2
  recurrence             = "0 9 * * MON-FRI"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("asg-schedule-basic"),
        "state missing scheduled action"
    );
    assert!(
        state.contains("aws_autoscaling_schedule"),
        "state missing autoscaling_schedule resource"
    );
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_autoscaling_lifecycle_hook
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_autoscaling_lifecycle_hook_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("asg-lifecycle-hook");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_launch_configuration" "test" {
  name          = "asg-lifecycle-hook-lc"
  image_id      = "ami-00000000"
  instance_type = "t2.micro"
}

resource "aws_autoscaling_group" "test" {
  name                 = "asg-lifecycle-hook-asg"
  launch_configuration = aws_launch_configuration.test.name
  min_size             = 0
  max_size             = 1
  availability_zones   = ["us-east-1a"]
}

resource "aws_autoscaling_lifecycle_hook" "test" {
  name                   = "asg-lifecycle-hook"
  autoscaling_group_name = aws_autoscaling_group.test.name
  lifecycle_transition   = "autoscaling:EC2_INSTANCE_LAUNCHING"
  default_result         = "CONTINUE"
  heartbeat_timeout      = 300
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("asg-lifecycle-hook"),
        "state missing lifecycle hook"
    );
    assert!(
        state.contains("aws_autoscaling_lifecycle_hook"),
        "state missing autoscaling_lifecycle_hook resource"
    );
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Full-stack test
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_autoscaling_full_stack() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("asg-full-stack");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_launch_configuration" "full" {
  name          = "asg-full-stack-lc"
  image_id      = "ami-00000000"
  instance_type = "t2.micro"
}

resource "aws_autoscaling_group" "full" {
  name                 = "asg-full-stack"
  launch_configuration = aws_launch_configuration.full.name
  min_size             = 0
  max_size             = 4
  desired_capacity     = 0
  availability_zones   = ["us-east-1a"]

  tag {
    key                 = "Name"
    value               = "asg-full-stack-instance"
    propagate_at_launch = true
  }

  tag {
    key                 = "Stack"
    value               = "autoscaling-full"
    propagate_at_launch = false
  }
}

resource "aws_autoscaling_policy" "full" {
  name                   = "asg-full-stack-policy"
  autoscaling_group_name = aws_autoscaling_group.full.name
  adjustment_type        = "ChangeInCapacity"
  scaling_adjustment     = 1
  cooldown               = 300
  policy_type            = "SimpleScaling"
}

resource "aws_autoscaling_schedule" "full" {
  scheduled_action_name  = "asg-full-stack-schedule"
  autoscaling_group_name = aws_autoscaling_group.full.name
  min_size               = 1
  max_size               = 4
  desired_capacity       = 2
  recurrence             = "0 9 * * MON-FRI"
}

resource "aws_autoscaling_lifecycle_hook" "full" {
  name                   = "asg-full-stack-hook"
  autoscaling_group_name = aws_autoscaling_group.full.name
  lifecycle_transition   = "autoscaling:EC2_INSTANCE_LAUNCHING"
  default_result         = "CONTINUE"
  heartbeat_timeout      = 300
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("asg-full-stack"), "state missing ASG");
    assert!(
        state.contains("asg-full-stack-lc"),
        "state missing launch configuration"
    );
    assert!(
        state.contains("asg-full-stack-policy"),
        "state missing scaling policy"
    );
    assert!(
        state.contains("asg-full-stack-schedule"),
        "state missing scheduled action"
    );
    assert!(
        state.contains("asg-full-stack-hook"),
        "state missing lifecycle hook"
    );
    cleanup_tf_dir(&dir);
}
