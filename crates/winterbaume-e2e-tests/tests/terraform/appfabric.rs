use crate::harness::*;

// ---------------------------------------------------------------------------
// aws_appfabric_app_bundle
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_appfabric_app_bundle_basic() {
    let result = batch_apply(
        r#"
resource "aws_appfabric_app_bundle" "appfabric_bundle_basic" {
  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("appbundle/"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_appfabric_app_bundle_with_cmk() {
    let result = batch_apply(
        r#"
resource "aws_appfabric_app_bundle" "appfabric_bundle_cmk" {
  customer_managed_key_arn = "arn:aws:kms:us-east-1:123456789012:key/abc-1234"

  tags = {
    Name = "with-cmk"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("appbundle/"));
    assert!(result.state.contains("abc-1234"));
}
