use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_config_configuration_recorder() {
    let result = batch_apply(
        r#"
resource "aws_config_configuration_recorder" "config_recorder" {
  name     = "terraform-test-recorder"
  role_arn = "arn:aws:iam::123456789012:role/config-role"

  recording_group {
    all_supported                 = true
    include_global_resource_types = true
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-recorder"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_config_delivery_channel() {
    let result = batch_apply(
        r#"
resource "aws_config_configuration_recorder" "config_delivery_channel" {
  name     = "delivery-channel-recorder"
  role_arn = "arn:aws:iam::123456789012:role/config-role"
}

resource "aws_config_delivery_channel" "config_delivery_channel" {
  name           = "terraform-test-channel"
  s3_bucket_name = "my-config-bucket"

  depends_on = [aws_config_configuration_recorder.config_delivery_channel]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-channel"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_config_config_rule() {
    let result = batch_apply(
        r#"
resource "aws_config_configuration_recorder" "config_rule" {
  name     = "config-rule-recorder"
  role_arn = "arn:aws:iam::123456789012:role/config-role"
}

resource "aws_config_config_rule" "config_rule" {
  name = "terraform-test-rule"

  source {
    owner             = "AWS"
    source_identifier = "S3_BUCKET_VERSIONING_ENABLED"
  }

  depends_on = [aws_config_configuration_recorder.config_rule]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-rule"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_config_configuration_aggregator() {
    let result = batch_apply(
        r#"
resource "aws_config_configuration_aggregator" "config_aggregator" {
  name = "terraform-test-aggregator"

  account_aggregation_source {
    account_ids = ["123456789012"]
    regions     = ["us-east-1"]
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-aggregator"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_config_aggregation_authorization() {
    let result = batch_apply(
        r#"
resource "aws_config_aggregate_authorization" "config_agg_auth" {
  account_id = "123456789012"
  region     = "us-east-1"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("123456789012"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_config_retention_configuration() {
    let result = batch_apply(
        r#"
resource "aws_config_retention_configuration" "config_retention" {
  retention_period_in_days = 90
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("90"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_config_recorder_with_delivery_channel_and_rule() {
    let result = batch_apply(
        r#"
resource "aws_config_configuration_recorder" "config_full_stack" {
  name     = "full-stack-recorder"
  role_arn = "arn:aws:iam::123456789012:role/config-role"

  recording_group {
    all_supported                 = true
    include_global_resource_types = true
  }
}

resource "aws_config_delivery_channel" "config_full_stack" {
  name           = "full-stack-channel"
  s3_bucket_name = "full-stack-config-bucket"

  snapshot_delivery_properties {
    delivery_frequency = "Six_Hours"
  }

  depends_on = [aws_config_configuration_recorder.config_full_stack]
}

resource "aws_config_config_rule" "config_full_stack" {
  name = "full-stack-rule"

  source {
    owner             = "AWS"
    source_identifier = "REQUIRED_TAGS"
  }

  input_parameters = jsonencode({
    tag1Key = "Environment"
  })

  depends_on = [aws_config_configuration_recorder.config_full_stack]
}

resource "aws_config_configuration_recorder_status" "config_full_stack" {
  name       = aws_config_configuration_recorder.config_full_stack.name
  is_enabled = true

  depends_on = [aws_config_delivery_channel.config_full_stack]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("full-stack-recorder"));
    assert!(result.state.contains("full-stack-channel"));
    assert!(result.state.contains("full-stack-rule"));
}
