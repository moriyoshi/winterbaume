use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sns_topic_basic() {
    let result = batch_apply(
        r#"
resource "aws_sns_topic" "sns_topic_basic" {
  name = "terraform-test-topic"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-topic"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sns_topic_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_sns_topic" "sns_topic_tags" {
  name         = "terraform-tagged-topic"
  display_name = "Test Topic"

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-tagged-topic"));
}

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_sns_topic_subscription_sqs() {
    let result = batch_apply(
        r#"
resource "aws_sqs_queue" "sns_sub_queue" {
  name = "tf-sns-sub-queue"
}

resource "aws_sns_topic" "sns_sub_topic" {
  name = "tf-sns-sub-topic"
}

resource "aws_sns_topic_subscription" "sns_sub" {
  topic_arn = aws_sns_topic.sns_sub_topic.arn
  protocol  = "sqs"
  endpoint  = aws_sqs_queue.sns_sub_queue.arn
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("tf-sns-sub-topic"));
    assert!(result.state.contains("tf-sns-sub-queue"));
    assert!(
        result.state.contains("\"aws_sns_topic_subscription\""),
        "subscription resource not found in state:\n{}",
        result.state
    );
}
