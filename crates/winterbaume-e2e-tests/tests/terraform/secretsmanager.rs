use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_secretsmanager_secret_basic() {
    let result = batch_apply(
        r#"
resource "aws_secretsmanager_secret" "sm_secret_basic" {
  name = "terraform-test-secret"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-secret"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_secretsmanager_secret_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_secretsmanager_secret" "sm_secret_tags" {
  name        = "terraform-tagged-secret"
  description = "A test secret with tags"

  tags = {
    Environment = "test"
    Service     = "app"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-tagged-secret"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_secretsmanager_secret_version() {
    let result = batch_apply(
        r#"
resource "aws_secretsmanager_secret" "sm_secret_version" {
  name = "terraform-version-secret"
}

resource "aws_secretsmanager_secret_version" "sm_secret_version" {
  secret_id     = aws_secretsmanager_secret.sm_secret_version.id
  secret_string = "super-secret-value"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-version-secret"));
}
