use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_kms_key_basic() {
    let result = batch_apply(
        r#"
resource "aws_kms_key" "kms_key_basic" {
  description = "kms-key-basic"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("kms-key-basic"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_kms_key_with_policy_and_tags() {
    let result = batch_apply(
        r#"
resource "aws_kms_key" "kms_key_full" {
  description = "kms-key-full"

  tags = {
    Environment = "test"
    Purpose     = "e2e"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("kms-key-full"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_kms_alias() {
    let result = batch_apply(
        r#"
resource "aws_kms_key" "kms_alias" {
  description = "kms-alias-key"
}

resource "aws_kms_alias" "kms_alias" {
  name          = "alias/terraform-test"
  target_key_id = aws_kms_key.kms_alias.key_id
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("alias/terraform-test"));
}
