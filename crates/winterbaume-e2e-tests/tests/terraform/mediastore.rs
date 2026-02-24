use crate::harness::*;

// MediaStore terraform resources tested here:
//   aws_media_store_container
//
// Surface is small: the converter only supports aws_media_store_container,
// and winterbaume's handler implements CreateContainer / DescribeContainer /
// DeleteContainer / ListContainers / ListTagsForResource (plus
// Put/GetContainerPolicy, Put/GetLifecyclePolicy, Put/GetMetricPolicy, but
// those have no dedicated terraform resource type via this converter).

// ---------------------------------------------------------------------------
// aws_media_store_container
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_mediastore_container_basic() {
    let result = batch_apply(
        r#"
resource "aws_media_store_container" "mediastore_container_basic" {
  name = "mediastore_container_basic"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("mediastore_container_basic"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_mediastore_container_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_media_store_container" "mediastore_container_tagged" {
  name = "mediastore_container_tagged"

  tags = {
    Environment = "test"
    Purpose     = "mediastore_e2e"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("mediastore_container_tagged"));
    assert!(result.state.contains("mediastore_e2e"));
}
