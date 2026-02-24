use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_metric_alarm_basic() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_metric_alarm" "cloudwatch_alarm_basic" {
  alarm_name          = "terraform-test-cpu-alarm"
  comparison_operator = "GreaterThanOrEqualToThreshold"
  evaluation_periods  = 2
  metric_name         = "CPUUtilization"
  namespace           = "AWS/EC2"
  period              = 120
  statistic           = "Average"
  threshold           = 80

  dimensions = {
    InstanceId = "i-1234567890abcdef0"
  }

  alarm_description = "This metric monitors ec2 cpu utilization"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-cpu-alarm"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_metric_alarm_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_metric_alarm" "cloudwatch_alarm_tags" {
  alarm_name          = "terraform-tagged-alarm"
  comparison_operator = "LessThanThreshold"
  evaluation_periods  = 1
  metric_name         = "HealthyHostCount"
  namespace           = "AWS/ApplicationELB"
  period              = 60
  statistic           = "Average"
  threshold           = 1

  tags = {
    Environment = "test"
    Service     = "web"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-tagged-alarm"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_metric_alarm_with_actions() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_metric_alarm" "cloudwatch_alarm_actions" {
  alarm_name          = "terraform-alarm-with-actions"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 3
  metric_name         = "Errors"
  namespace           = "AWS/Lambda"
  period              = 300
  statistic           = "Sum"
  threshold           = 5

  alarm_actions             = ["arn:aws:sns:us-east-1:123456789012:my-alerts"]
  ok_actions                = ["arn:aws:sns:us-east-1:123456789012:my-alerts"]
  insufficient_data_actions = []

  treat_missing_data = "notBreaching"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-alarm-with-actions"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_dashboard_basic() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_dashboard" "cloudwatch_dashboard_basic" {
  dashboard_name = "terraform-test-dashboard"

  dashboard_body = jsonencode({
    widgets = [
      {
        type       = "metric"
        x          = 0
        y          = 0
        width      = 12
        height     = 6
        properties = {
          metrics = [
            ["AWS/EC2", "CPUUtilization", "InstanceId", "i-1234567890abcdef0"]
          ]
          view    = "timeSeries"
          stacked = false
          region  = "us-east-1"
          title   = "EC2 CPU Utilization"
        }
      }
    ]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-dashboard"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_dashboard_with_text_widget() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_dashboard" "cloudwatch_dashboard_text" {
  dashboard_name = "terraform-text-dashboard"

  dashboard_body = jsonencode({
    widgets = [
      {
        type       = "text"
        x          = 0
        y          = 0
        width      = 6
        height     = 3
        properties = {
          markdown = "My Dashboard - This is a test dashboard."
        }
      }
    ]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-text-dashboard"));
}

// test_cloudwatch_dashboard_modify_in_place tests that a dashboard can be updated
// without re-creation (uses two sequential apply passes).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_dashboard_modify_in_place() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("cloudwatch-dashboard-modify");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_cloudwatch_dashboard" "test" {
  dashboard_name = "modify-test-dashboard"

  dashboard_body = jsonencode({
    widgets = []
  })
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "first apply failed:\n{stderr}");

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_cloudwatch_dashboard" "test" {
  dashboard_name = "modify-test-dashboard"

  dashboard_body = jsonencode({
    widgets = [
      {
        type   = "text"
        x      = 0
        y      = 0
        width  = 12
        height = 3
        properties = { markdown = "Updated" }
      }
    ]
  })
}
"#,
    )
    .unwrap();

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second apply (modify) failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 0 added, 1 changed, 0 destroyed"),
        "Expected in-place update:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_metric_stream
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_metric_stream_basic() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_metric_stream" "cloudwatch_stream_basic" {
  name          = "terraform-test-stream"
  role_arn      = "arn:aws:iam::123456789012:role/MetricStreamRole"
  firehose_arn  = "arn:aws:firehose:us-east-1:123456789012:deliverystream/test-stream"
  output_format = "json"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-stream"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_metric_stream_with_include_filter() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_metric_stream" "cloudwatch_stream_include" {
  name          = "terraform-stream-include"
  role_arn      = "arn:aws:iam::123456789012:role/MetricStreamRole"
  firehose_arn  = "arn:aws:firehose:us-east-1:123456789012:deliverystream/test-stream"
  output_format = "json"

  include_filter {
    namespace = "AWS/EC2"
  }

  include_filter {
    namespace = "AWS/EBS"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-stream-include"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_metric_stream_with_exclude_filter() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_metric_stream" "cloudwatch_stream_exclude" {
  name          = "terraform-stream-exclude"
  role_arn      = "arn:aws:iam::123456789012:role/MetricStreamRole"
  firehose_arn  = "arn:aws:firehose:us-east-1:123456789012:deliverystream/test-stream"
  output_format = "opentelemetry0.7"

  exclude_filter {
    namespace = "AWS/Usage"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-stream-exclude"));
}

// test_cloudwatch_metric_stream_modify_in_place tests that a stream can be updated
// without re-creation (uses two sequential apply passes).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_metric_stream_modify_in_place() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("cloudwatch-stream-modify");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_cloudwatch_metric_stream" "test" {
  name          = "modify-test-stream"
  role_arn      = "arn:aws:iam::123456789012:role/MetricStreamRole"
  firehose_arn  = "arn:aws:firehose:us-east-1:123456789012:deliverystream/test-stream"
  output_format = "json"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "first apply failed:\n{stderr}");

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_cloudwatch_metric_stream" "test" {
  name          = "modify-test-stream"
  role_arn      = "arn:aws:iam::123456789012:role/MetricStreamRole"
  firehose_arn  = "arn:aws:firehose:us-east-1:123456789012:deliverystream/test-stream"
  output_format = "opentelemetry0.7"

  include_filter {
    namespace = "AWS/EC2"
  }
}
"#,
    )
    .unwrap();

    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second apply (modify) failed:\n{stderr}");

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_metric_alarm (extended_statistic variant)
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_metric_alarm_with_extended_statistic() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_metric_alarm" "cloudwatch_alarm_percentile" {
  alarm_name          = "terraform-percentile-alarm"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 1
  metric_name         = "Latency"
  namespace           = "AWS/ELB"
  period              = 60
  extended_statistic  = "p99"
  threshold           = 500

  alarm_description = "p99 latency alarm"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-percentile-alarm"));
}

// ---------------------------------------------------------------------------
// Full-stack tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_metric_alarm" "cloudwatch_full_cpu" {
  alarm_name          = "full-stack-cpu-alarm"
  comparison_operator = "GreaterThanOrEqualToThreshold"
  evaluation_periods  = 2
  metric_name         = "CPUUtilization"
  namespace           = "AWS/EC2"
  period              = 300
  statistic           = "Average"
  threshold           = 90

  alarm_description = "High CPU alarm for full-stack test"

  tags = {
    Stack = "full"
  }
}

resource "aws_cloudwatch_metric_alarm" "cloudwatch_full_mem" {
  alarm_name          = "full-stack-memory-alarm"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 1
  metric_name         = "MemoryUtilization"
  namespace           = "CWAgent"
  period              = 60
  statistic           = "Average"
  threshold           = 85

  tags = {
    Stack = "full"
  }
}

resource "aws_cloudwatch_dashboard" "cloudwatch_full_dash" {
  dashboard_name = "full-stack-dashboard"

  dashboard_body = jsonencode({
    widgets = [
      {
        type       = "alarm"
        x          = 0
        y          = 0
        width      = 6
        height     = 3
        properties = {
          title  = "Alarms"
          alarms = [aws_cloudwatch_metric_alarm.cloudwatch_full_cpu.arn]
        }
      }
    ]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("full-stack-cpu-alarm"));
    assert!(result.state.contains("full-stack-memory-alarm"));
    assert!(result.state.contains("full-stack-dashboard"));
}
