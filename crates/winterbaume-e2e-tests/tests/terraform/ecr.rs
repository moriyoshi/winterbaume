use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecr_repository_basic() {
    let result = batch_apply(
        r#"
resource "aws_ecr_repository" "ecr_repo_basic" {
  name = "terraform-test-repo"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-repo"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecr_repository_with_settings() {
    let result = batch_apply(
        r#"
resource "aws_ecr_repository" "ecr_repo_settings" {
  name                 = "terraform-settings-repo"
  image_tag_mutability = "IMMUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-settings-repo"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecr_lifecycle_policy() {
    let result = batch_apply(
        r#"
resource "aws_ecr_repository" "ecr_lifecycle" {
  name = "terraform-lifecycle-repo"
}

resource "aws_ecr_lifecycle_policy" "ecr_lifecycle" {
  repository = aws_ecr_repository.ecr_lifecycle.name

  policy = jsonencode({
    rules = [{
      rulePriority = 1
      description  = "Keep last 10 images"
      selection = {
        tagStatus   = "any"
        countType   = "imageCountMoreThan"
        countNumber = 10
      }
      action = {
        type = "expire"
      }
    }]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-lifecycle-repo"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecr_repository_policy() {
    let result = batch_apply(
        r#"
resource "aws_ecr_repository" "ecr_repo_policy" {
  name = "terraform-policy-repo"
}

resource "aws_ecr_repository_policy" "ecr_repo_policy" {
  repository = aws_ecr_repository.ecr_repo_policy.name

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Sid       = "AllowPull"
      Effect    = "Allow"
      Principal = { AWS = "arn:aws:iam::123456789012:root" }
      Action    = [
        "ecr:GetDownloadUrlForLayer",
        "ecr:BatchGetImage"
      ]
    }]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-policy-repo"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecr_replication_configuration() {
    let result = batch_apply(
        r#"
resource "aws_ecr_replication_configuration" "ecr_replication" {
  replication_configuration {
    rule {
      destination {
        region      = "eu-west-1"
        registry_id = "123456789012"
      }

      repository_filter {
        filter      = "prod-"
        filter_type = "PREFIX_MATCH"
      }
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("eu-west-1"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecr_pull_through_cache_rule() {
    let result = batch_apply(
        r#"
resource "aws_ecr_pull_through_cache_rule" "ecr_cache_rule" {
  ecr_repository_prefix = "ecr-public"
  upstream_registry_url = "public.ecr.aws"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("ecr-public"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecr_registry_scanning_configuration() {
    let result = batch_apply(
        r#"
resource "aws_ecr_registry_scanning_configuration" "ecr_scanning" {
  scan_type = "ENHANCED"

  rule {
    scan_frequency = "CONTINUOUS_SCAN"

    repository_filter {
      filter      = "*"
      filter_type = "WILDCARD"
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("ENHANCED"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ecr_repository_with_encryption() {
    let result = batch_apply(
        r#"
resource "aws_ecr_repository" "ecr_encrypted" {
  name = "terraform-encrypted-repo"

  encryption_configuration {
    encryption_type = "AES256"
  }

  image_scanning_configuration {
    scan_on_push = true
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-encrypted-repo"));
}
