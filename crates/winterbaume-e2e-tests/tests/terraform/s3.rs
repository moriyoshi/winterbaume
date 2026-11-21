use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_s3_bucket_basic() {
    let result = batch_apply(
        r#"
resource "aws_s3_bucket" "s3_bucket_basic" {
  bucket = "my-terraform-test-bucket"

  tags = {
    Name        = "test-bucket"
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("my-terraform-test-bucket"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_s3_bucket_versioning() {
    let result = batch_apply(
        r#"
resource "aws_s3_bucket" "s3_bucket_versioning" {
  bucket = "versioning-test-bucket"
}

resource "aws_s3_bucket_versioning" "s3_bucket_versioning" {
  bucket = aws_s3_bucket.s3_bucket_versioning.id
  versioning_configuration {
    status = "Enabled"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("versioning-test-bucket"));
    assert!(result.state.contains("Enabled"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_s3_bucket_policy() {
    let result = batch_apply(
        r#"
resource "aws_s3_bucket" "s3_bucket_policy" {
  bucket = "policy-test-bucket"
}

resource "aws_s3_bucket_policy" "s3_bucket_policy" {
  bucket = aws_s3_bucket.s3_bucket_policy.id
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Sid       = "PublicReadGetObject"
      Effect    = "Allow"
      Principal = "*"
      Action    = "s3:GetObject"
      Resource  = "${aws_s3_bucket.s3_bucket_policy.arn}/*"
    }]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("policy-test-bucket"));
    assert!(result.state.contains("PublicReadGetObject"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_s3_bucket_server_side_encryption() {
    let result = batch_apply(
        r#"
resource "aws_s3_bucket" "s3_bucket_sse" {
  bucket = "encryption-test-bucket"
}

resource "aws_s3_bucket_server_side_encryption_configuration" "s3_bucket_sse" {
  bucket = aws_s3_bucket.s3_bucket_sse.id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
    bucket_key_enabled = true
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("encryption-test-bucket"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_s3_bucket_public_access_block() {
    let result = batch_apply(
        r#"
resource "aws_s3_bucket" "s3_bucket_pab" {
  bucket = "public-access-test-bucket"
}

resource "aws_s3_bucket_public_access_block" "s3_bucket_pab" {
  bucket = aws_s3_bucket.s3_bucket_pab.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("public-access-test-bucket"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_s3_bucket_ownership_controls() {
    let result = batch_apply(
        r#"
resource "aws_s3_bucket" "s3_bucket_ownership" {
  bucket = "ownership-test-bucket"
}

resource "aws_s3_bucket_ownership_controls" "s3_bucket_ownership" {
  bucket = aws_s3_bucket.s3_bucket_ownership.id

  rule {
    object_ownership = "BucketOwnerPreferred"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("ownership-test-bucket"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_s3_bucket_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_s3_bucket" "s3_bucket_full" {
  bucket = "full-stack-test-bucket"

  tags = {
    Name = "full-stack"
  }
}

resource "aws_s3_bucket_versioning" "s3_bucket_full" {
  bucket = aws_s3_bucket.s3_bucket_full.id
  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "s3_bucket_full" {
  bucket = aws_s3_bucket.s3_bucket_full.id
  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

resource "aws_s3_bucket_public_access_block" "s3_bucket_full" {
  bucket = aws_s3_bucket.s3_bucket_full.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_s3_bucket_ownership_controls" "s3_bucket_full" {
  bucket = aws_s3_bucket.s3_bucket_full.id
  rule {
    object_ownership = "BucketOwnerEnforced"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("full-stack-test-bucket"));
}

// Regression for issue #41: the provider polls GetBucketLifecycleConfiguration
// after the put until the rules and `x-amz-transition-default-minimum-object-size`
// read back as sent, so the apply only converges when both responses carry it.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_s3_bucket_lifecycle_configuration() {
    let result = batch_apply(
        r#"
resource "aws_s3_bucket" "s3_bucket_lifecycle" {
  bucket = "lifecycle-test-bucket"
}

resource "aws_s3_bucket_lifecycle_configuration" "s3_bucket_lifecycle" {
  bucket = aws_s3_bucket.s3_bucket_lifecycle.id

  rule {
    id     = "expire-tmp"
    status = "Enabled"

    filter {
      prefix = "tmp/"
    }

    expiration {
      days = 7
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("lifecycle-test-bucket"));
    assert!(result.state.contains("expire-tmp"));
    assert!(result.state.contains("all_storage_classes_128K"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_s3_bucket_lifecycle_configuration_varies_by_storage_class() {
    let result = batch_apply(
        r#"
resource "aws_s3_bucket" "s3_bucket_lifecycle_varies" {
  bucket = "lifecycle-varies-test-bucket"
}

resource "aws_s3_bucket_lifecycle_configuration" "s3_bucket_lifecycle_varies" {
  bucket = aws_s3_bucket.s3_bucket_lifecycle_varies.id

  transition_default_minimum_object_size = "varies_by_storage_class"

  rule {
    id     = "archive-logs"
    status = "Enabled"

    filter {
      prefix = "logs/"
    }

    transition {
      days          = 30
      storage_class = "GLACIER"
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("lifecycle-varies-test-bucket"));
    assert!(result.state.contains("archive-logs"));
    assert!(result.state.contains("varies_by_storage_class"));
}

/// Regression for issue #50: destroying `aws_s3_bucket_ownership_controls`
/// calls DeleteBucketOwnershipControls and then polls
/// GetBucketOwnershipControls until it reports `OwnershipControlsNotFoundError`.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_s3_bucket_ownership_controls_destroy() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("s3-ownership-controls-destroy");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_s3_bucket" "s3_ownership_destroy" {
  bucket = "ownership-destroy-test-bucket"
}

resource "aws_s3_bucket_ownership_controls" "s3_ownership_destroy" {
  bucket = aws_s3_bucket.s3_ownership_destroy.id

  rule {
    object_ownership = "BucketOwnerPreferred"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let (ok, _stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    cleanup_tf_dir(&dir);
}

/// `GetBucketTagging` body for `bucket`, asked of the S3 service directly.
async fn s3_get_bucket_tagging(
    s3: &winterbaume_s3::S3Service,
    bucket: &str,
) -> winterbaume_core::MockResponse {
    use winterbaume_core::MockService as _;
    s3.handle(winterbaume_core::MockRequest {
        method: "GET".to_string(),
        uri: format!("https://s3.us-east-1.amazonaws.com/{bucket}?tagging"),
        headers: http::HeaderMap::new(),
        body: Default::default(),
    })
    .await
}

fn tagged_bucket_tf(tags: &str) -> String {
    format!(
        r#"
resource "aws_s3_bucket" "s3_bucket_tags" {{
  bucket = "bucket-tags-converge-test"
{tags}
}}
"#
    )
}

/// Regression for issue #53: an `aws_s3_bucket` with `tags` ( or provider
/// `default_tags` ) showed a `tags` / `tags_all` diff on every plan. The AWS
/// provider reads bucket tags through S3 Control `ListTagsForResource` on
/// `arn:aws:s3:::bucket` and changes them through `TagResource` /
/// `UntagResource`, so S3 Control has to serve the S3 bucket's own tag set.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_s3_bucket_tags_converge() {
    const BUCKET: &str = "bucket-tags-converge-test";

    let s3 = std::sync::Arc::new(winterbaume_s3::S3Service::new());
    let port = start_server(vec![
        std::sync::Arc::new(winterbaume_sts::StsService::new()),
        std::sync::Arc::new(winterbaume_s3control::S3ControlService::with_s3_service(
            std::sync::Arc::clone(&s3),
        )),
        std::sync::Arc::clone(&s3) as std::sync::Arc<dyn winterbaume_core::MockService>,
    ])
    .await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("s3-bucket-tags-converge");

    // Provider `default_tags` on top of the resource's own tags.
    write_provider_tf(&dir, &url);
    let provider_tf = std::fs::read_to_string(dir.join("provider.tf")).unwrap();
    let with_default_tags = provider_tf.replacen(
        "  access_key = \"test\"",
        "  default_tags {\n    tags = {\n      ManagedBy = \"terraform\"\n    }\n  }\n\n  access_key = \"test\"",
        1,
    );
    assert_ne!(provider_tf, with_default_tags, "default_tags not injected");
    std::fs::write(dir.join("provider.tf"), &with_default_tags).unwrap();

    // 1. Create with tags: the next plan must be empty.
    std::fs::write(
        dir.join("main.tf"),
        tagged_bucket_tf(
            r#"
  tags = {
    Name        = "tagged bucket"
    Environment = "dev"
    Owner       = "carina"
  }
"#,
        ),
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let (ok, stdout, stderr) = terraform_plan(&dir).await;
    assert!(ok, "plan after create is not empty:\n{stdout}\n{stderr}");

    let response = s3_get_bucket_tagging(&s3, BUCKET).await;
    let body = String::from_utf8_lossy(&response.body).to_string();
    assert_eq!(response.status, 200, "GetBucketTagging failed: {body}");
    for (key, value) in [
        ("Name", "tagged bucket"),
        ("Environment", "dev"),
        ("Owner", "carina"),
        ("ManagedBy", "terraform"),
    ] {
        assert!(
            body.contains(&format!("<Key>{key}</Key><Value>{value}</Value>")),
            "tag {key}={value} missing from GetBucketTagging: {body}"
        );
    }

    // 2. Change one tag value and remove another.
    std::fs::write(
        dir.join("main.tf"),
        tagged_bucket_tf(
            r#"
  tags = {
    Name        = "tagged bucket"
    Environment = "prod"
  }
"#,
        ),
    )
    .unwrap();
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply of the tag change failed:\n{stderr}");
    let (ok, stdout, stderr) = terraform_plan(&dir).await;
    assert!(
        ok,
        "plan after tag change is not empty:\n{stdout}\n{stderr}"
    );

    let response = s3_get_bucket_tagging(&s3, BUCKET).await;
    let body = String::from_utf8_lossy(&response.body).to_string();
    assert_eq!(response.status, 200, "GetBucketTagging failed: {body}");
    assert!(
        body.contains("<Key>Environment</Key><Value>prod</Value>"),
        "changed tag missing from GetBucketTagging: {body}"
    );
    assert!(
        body.contains("<Key>ManagedBy</Key><Value>terraform</Value>"),
        "default tag missing from GetBucketTagging: {body}"
    );
    assert!(
        !body.contains("Owner"),
        "removed tag still reported by GetBucketTagging: {body}"
    );

    // 3. Remove every tag, the provider default included.
    std::fs::write(dir.join("provider.tf"), &provider_tf).unwrap();
    std::fs::write(dir.join("main.tf"), tagged_bucket_tf("")).unwrap();
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply of the tag removal failed:\n{stderr}");
    let (ok, stdout, stderr) = terraform_plan(&dir).await;
    assert!(
        ok,
        "plan after tag removal is not empty:\n{stdout}\n{stderr}"
    );

    let response = s3_get_bucket_tagging(&s3, BUCKET).await;
    let body = String::from_utf8_lossy(&response.body).to_string();
    assert_eq!(response.status, 404, "tags left after removal: {body}");
    assert!(body.contains("NoSuchTagSet"), "unexpected error: {body}");

    cleanup_tf_dir(&dir);
}
