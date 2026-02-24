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
