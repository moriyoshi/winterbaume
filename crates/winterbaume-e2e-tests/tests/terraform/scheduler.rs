use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_scheduler_schedule_group_basic() {
    let result = batch_apply(
        r#"
resource "aws_scheduler_schedule_group" "test_group" {
  name = "terraform-test-schedule-group"

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-schedule-group"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_scheduler_schedule_basic() {
    let result = batch_apply(
        r#"
resource "aws_scheduler_schedule" "test_schedule" {
  name       = "terraform-test-schedule"
  group_name = "default"

  flexible_time_window {
    mode = "OFF"
  }

  schedule_expression = "rate(1 hours)"

  target {
    arn      = "arn:aws:sqs:us-east-1:123456789012:my-queue"
    role_arn = "arn:aws:iam::123456789012:role/scheduler-role"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-schedule"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_scheduler_schedule_with_description() {
    let result = batch_apply(
        r#"
resource "aws_scheduler_schedule" "test_schedule_desc" {
  name        = "terraform-test-schedule-desc"
  group_name  = "default"
  description = "A test schedule with a description"

  flexible_time_window {
    mode = "OFF"
  }

  schedule_expression = "rate(5 minutes)"

  target {
    arn      = "arn:aws:lambda:us-east-1:123456789012:function:my-func"
    role_arn = "arn:aws:iam::123456789012:role/scheduler-role"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-schedule-desc"));
    assert!(result.state.contains("A test schedule with a description"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_scheduler_schedule_with_flexible_time_window() {
    let result = batch_apply(
        r#"
resource "aws_scheduler_schedule" "test_schedule_ftw" {
  name       = "terraform-test-schedule-ftw"
  group_name = "default"

  flexible_time_window {
    mode                      = "FLEXIBLE"
    maximum_window_in_minutes = 15
  }

  schedule_expression = "rate(1 hours)"

  target {
    arn      = "arn:aws:sqs:us-east-1:123456789012:my-queue"
    role_arn = "arn:aws:iam::123456789012:role/scheduler-role"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-schedule-ftw"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_scheduler_schedule_with_custom_group() {
    let result = batch_apply(
        r#"
resource "aws_scheduler_schedule_group" "custom_group" {
  name = "terraform-custom-group"
}

resource "aws_scheduler_schedule" "test_schedule_custom_group" {
  name       = "terraform-test-schedule-cg"
  group_name = aws_scheduler_schedule_group.custom_group.name

  flexible_time_window {
    mode = "OFF"
  }

  schedule_expression = "rate(10 minutes)"

  target {
    arn      = "arn:aws:sqs:us-east-1:123456789012:my-queue"
    role_arn = "arn:aws:iam::123456789012:role/scheduler-role"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-custom-group"));
    assert!(result.state.contains("terraform-test-schedule-cg"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_scheduler_schedule_enabled_state() {
    let result = batch_apply(
        r#"
resource "aws_scheduler_schedule" "test_schedule_disabled" {
  name       = "terraform-test-schedule-disabled"
  group_name = "default"
  state      = "DISABLED"

  flexible_time_window {
    mode = "OFF"
  }

  schedule_expression = "rate(1 hours)"

  target {
    arn      = "arn:aws:sqs:us-east-1:123456789012:my-queue"
    role_arn = "arn:aws:iam::123456789012:role/scheduler-role"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-schedule-disabled"));
    assert!(result.state.contains("DISABLED"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_scheduler_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_scheduler_schedule_group" "full_group" {
  name = "terraform-full-stack-group"

  tags = {
    Stack = "full"
  }
}

resource "aws_scheduler_schedule" "full_schedule_1" {
  name       = "terraform-full-schedule-1"
  group_name = aws_scheduler_schedule_group.full_group.name

  flexible_time_window {
    mode = "OFF"
  }

  schedule_expression = "rate(5 minutes)"

  target {
    arn      = "arn:aws:sqs:us-east-1:123456789012:queue-1"
    role_arn = "arn:aws:iam::123456789012:role/scheduler-role"
  }
}

resource "aws_scheduler_schedule" "full_schedule_2" {
  name        = "terraform-full-schedule-2"
  group_name  = aws_scheduler_schedule_group.full_group.name
  description = "Second schedule in full stack"
  state       = "ENABLED"

  flexible_time_window {
    mode                      = "FLEXIBLE"
    maximum_window_in_minutes = 10
  }

  schedule_expression = "cron(0 12 * * ? *)"

  target {
    arn      = "arn:aws:lambda:us-east-1:123456789012:function:my-func"
    role_arn = "arn:aws:iam::123456789012:role/scheduler-role"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-full-stack-group"));
    assert!(result.state.contains("terraform-full-schedule-1"));
    assert!(result.state.contains("terraform-full-schedule-2"));
}
