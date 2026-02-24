use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_apply_multi_service() {
    let result = batch_apply(
        r#"
resource "aws_s3_bucket" "multi_svc" {
  bucket = "multi-test-data-bucket"
}

resource "aws_iam_role" "multi_svc" {
  name = "multi-test-app-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = { Service = "ecs-tasks.amazonaws.com" }
    }]
  })
}

resource "aws_iam_policy" "multi_svc_s3" {
  name = "multi-test-s3-access"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action   = ["s3:GetObject", "s3:PutObject"]
      Effect   = "Allow"
      Resource = "${aws_s3_bucket.multi_svc.arn}/*"
    }]
  })
}

resource "aws_iam_role_policy_attachment" "multi_svc" {
  role       = aws_iam_role.multi_svc.name
  policy_arn = aws_iam_policy.multi_svc_s3.arn
}

resource "aws_sqs_queue" "multi_svc" {
  name                       = "multi-test-events"
  visibility_timeout_seconds = 30
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("multi-test-data-bucket"));
    assert!(result.state.contains("multi-test-app-role"));
    assert!(result.state.contains("multi-test-s3-access"));
    assert!(result.state.contains("multi-test-events"));
}

// test_terraform_plan_idempotent_after_apply requires an apply followed by a
// plan check. It uses its own isolated server and directory rather than
// batch_apply so the plan sees only its own resources.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_terraform_plan_idempotent_after_apply() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("plan-idempotent");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_s3_bucket" "test" {
  bucket = "idempotent-test-bucket"
}

resource "aws_sqs_queue" "test" {
  name = "idempotent-test-queue"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    // With -detailed-exitcode: 0 = no changes, 2 = changes pending
    let (ok, stdout, stderr) = terraform_plan(&dir).await;
    assert!(
        ok,
        "terraform plan shows changes after apply (expected idempotent):\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("No changes"),
        "Expected 'No changes' in plan output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}
