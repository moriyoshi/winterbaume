use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_events_rule_schedule() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_event_rule" "events_rule_schedule" {
  name                = "terraform-test-schedule-rule"
  description         = "Runs every 5 minutes"
  schedule_expression = "rate(5 minutes)"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-schedule-rule"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_events_rule_pattern() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_event_rule" "events_rule_pattern" {
  name        = "terraform-test-pattern-rule"
  description = "Matches EC2 state changes"

  event_pattern = jsonencode({
    source      = ["aws.ec2"]
    detail-type = ["EC2 Instance State-change Notification"]
  })

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-pattern-rule"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_events_event_bus() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_event_bus" "events_event_bus" {
  name = "terraform-test-event-bus"

  tags = {
    Purpose = "testing"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-event-bus"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_events_rule_on_custom_bus() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_event_bus" "events_custom_bus" {
  name = "terraform-custom-bus"
}

resource "aws_cloudwatch_event_rule" "events_rule_custom_bus" {
  name           = "terraform-test-custom-bus-rule"
  event_bus_name = aws_cloudwatch_event_bus.events_custom_bus.name

  event_pattern = jsonencode({
    source = ["custom.app"]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-custom-bus"));
    assert!(result.state.contains("terraform-test-custom-bus-rule"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_events_target() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_event_rule" "events_target_rule" {
  name                = "terraform-target-test-rule"
  schedule_expression = "rate(1 hour)"
}

resource "aws_cloudwatch_event_target" "events_target" {
  rule      = aws_cloudwatch_event_rule.events_target_rule.name
  target_id = "my-target"
  arn       = "arn:aws:sqs:us-east-1:123456789012:my-queue"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-target-test-rule"));
    assert!(result.state.contains("my-target"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_events_archive() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_event_bus" "events_archive_bus" {
  name = "terraform-archive-bus"
}

resource "aws_cloudwatch_event_archive" "events_archive" {
  name             = "terraform-test-archive"
  event_source_arn = aws_cloudwatch_event_bus.events_archive_bus.arn
  description      = "Test event archive"
  retention_days   = 7
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-archive"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_events_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_event_bus" "events_full_bus" {
  name = "terraform-full-stack-bus"

  tags = {
    Stack = "full"
  }
}

resource "aws_cloudwatch_event_rule" "events_full_rule" {
  name           = "terraform-full-stack-rule"
  event_bus_name = aws_cloudwatch_event_bus.events_full_bus.name

  event_pattern = jsonencode({
    source      = ["custom.myapp"]
    detail-type = ["OrderPlaced"]
  })

  tags = {
    Stack = "full"
  }
}

resource "aws_cloudwatch_event_target" "events_full_target" {
  rule           = aws_cloudwatch_event_rule.events_full_rule.name
  event_bus_name = aws_cloudwatch_event_bus.events_full_bus.name
  target_id      = "order-processor"
  arn            = "arn:aws:sqs:us-east-1:123456789012:order-queue"
}

resource "aws_cloudwatch_event_archive" "events_full_archive" {
  name             = "terraform-full-stack-archive"
  event_source_arn = aws_cloudwatch_event_bus.events_full_bus.arn
  retention_days   = 30
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-full-stack-bus"));
    assert!(result.state.contains("terraform-full-stack-rule"));
    assert!(result.state.contains("order-processor"));
    assert!(result.state.contains("terraform-full-stack-archive"));
}
