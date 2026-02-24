use crate::harness::*;

// ---------------------------------------------------------------------------
// aws_media_package_channel
// ---------------------------------------------------------------------------
//
// MediaPackage exposes a REST-JSON API. The terraform AWS provider talks to it
// via the `mediapackage` endpoint key, hitting `POST /channels`,
// `GET /channels/{id}`, `DELETE /channels/{id}`, and `GET /channels`.
//
// Only `aws_media_package_channel` has a winterbaume terraform converter at
// the time these tests were written. Origin-endpoint resources
// (`aws_media_package_origin_endpoint`) are exercised by the SDK-driven
// scenario tests in the mediapackage crate but not yet by terraform here.

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_mediapackage_channel_basic() {
    let result = batch_apply(
        r#"
resource "aws_media_package_channel" "mediapackage_channel_basic" {
  channel_id  = "mediapackage-channel-basic"
  description = "mediapackage channel basic"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("mediapackage-channel-basic"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_mediapackage_channel_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_media_package_channel" "mediapackage_channel_with_tags" {
  channel_id  = "mediapackage-channel-tagged"
  description = "mediapackage channel with tags"

  tags = {
    Environment = "test"
    Purpose     = "e2e"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("mediapackage-channel-tagged"));
    assert!(result.state.contains("Environment"));
}
