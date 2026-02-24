use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_log_group_basic() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_log_group" "logs_group_basic" {
  name = "/test/basic-log-group"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("/test/basic-log-group"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_log_group_with_retention() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_log_group" "logs_group_retention" {
  name              = "/test/retention-log-group"
  retention_in_days = 30

  tags = {
    Environment = "test"
    Service     = "app"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("/test/retention-log-group"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_log_metric_filter() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_log_group" "logs_metric_filter_group" {
  name = "/test/metric-filter-log-group"
}

resource "aws_cloudwatch_log_metric_filter" "logs_metric_filter" {
  name           = "terraform-test-metric-filter"
  pattern        = "[ip, user, username, timestamp, request, status_code=4*, bytes]"
  log_group_name = aws_cloudwatch_log_group.logs_metric_filter_group.name

  metric_transformation {
    name      = "4xxCount"
    namespace = "CustomMetrics/WebServer"
    value     = "1"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-metric-filter"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudwatch_log_resource_policy() {
    let result = batch_apply(
        r#"
resource "aws_cloudwatch_log_resource_policy" "logs_resource_policy" {
  policy_name = "terraform-test-log-resource-policy"

  policy_document = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Service = "delivery.logs.amazonaws.com" }
      Action    = ["logs:CreateLogStream", "logs:PutLogEvents"]
      Resource  = "arn:aws:logs:us-east-1:123456789012:log-group:/test/*:*"
    }]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-log-resource-policy"));
}
