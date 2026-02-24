use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sqs_queue_basic() {
    let result = batch_apply(
        r#"
resource "aws_sqs_queue" "sqs_queue_basic" {
  name = "terraform-basic-queue"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-basic-queue"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sqs_queue_with_attributes() {
    let result = batch_apply(
        r#"
resource "aws_sqs_queue" "sqs_queue_attrs" {
  name                       = "terraform-attrs-queue"
  visibility_timeout_seconds = 45
  delay_seconds              = 5
  max_message_size           = 131072
  message_retention_seconds  = 86400
  receive_wait_time_seconds  = 10

  tags = {
    Environment = "test"
    Service     = "orders"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-attrs-queue"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sqs_queue_fifo() {
    let result = batch_apply(
        r#"
resource "aws_sqs_queue" "sqs_queue_fifo" {
  name                        = "terraform-fifo-queue.fifo"
  fifo_queue                  = true
  content_based_deduplication = true
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-fifo-queue.fifo"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sqs_queue_policy() {
    let result = batch_apply(
        r#"
resource "aws_sqs_queue" "sqs_queue_policy" {
  name = "terraform-policy-queue"
}

resource "aws_sqs_queue_policy" "sqs_queue_policy" {
  queue_url = aws_sqs_queue.sqs_queue_policy.url

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Sid       = "AllowSNS"
      Effect    = "Allow"
      Principal = { Service = "sns.amazonaws.com" }
      Action    = "sqs:SendMessage"
      Resource  = aws_sqs_queue.sqs_queue_policy.arn
    }]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-policy-queue"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sqs_queue_redrive_policy() {
    let result = batch_apply(
        r#"
resource "aws_sqs_queue" "sqs_redrive_dlq" {
  name                      = "terraform-dlq"
  message_retention_seconds = 1209600
}

resource "aws_sqs_queue" "sqs_redrive_main" {
  name                       = "terraform-main-queue"
  visibility_timeout_seconds = 30

  redrive_policy = jsonencode({
    deadLetterTargetArn = aws_sqs_queue.sqs_redrive_dlq.arn
    maxReceiveCount     = 5
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-main-queue"));
    assert!(result.state.contains("terraform-dlq"));
}

// test_sqs_queue_modify_in_place requires two sequential apply passes
// (apply initial config, then apply modified config). It uses its own isolated
// server and directory rather than batch_apply.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sqs_queue_modify_in_place() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("sqs-modify");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_sqs_queue" "test" {
  name                       = "modify-test-queue"
  visibility_timeout_seconds = 30
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
resource "aws_sqs_queue" "test" {
  name                       = "modify-test-queue"
  visibility_timeout_seconds = 60
  delay_seconds              = 10
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
