use crate::harness::*;

// CloudFront terraform resources tested here:
//   aws_cloudfront_distribution         (CreateDistribution, GetDistribution, GetDistributionConfig,
//                                        UpdateDistribution, DeleteDistribution, ListTagsForResource)
//   aws_cloudfront_origin_access_control (CreateOriginAccessControl, GetOriginAccessControl,
//                                         GetOriginAccessControlConfig, UpdateOriginAccessControl,
//                                         DeleteOriginAccessControl)
//
// Note: aws_cloudfront_distribution is tested with isolated servers because the provider
// uses ETag-based conditional updates.  The ETag is stable between calls (set at creation),
// so batch_apply would work in principle, but isolation avoids any cross-test drift.
//
// Not tested:
//   aws_cloudfront_origin_access_identity — stub handler returns empty struct; terraform
//     detects drift on S3CanonicalUserId and fails.
//   aws_cloudfront_cache_policy           — stub handler (no state backing).
//   aws_cloudfront_response_headers_policy — stub handler (no state backing).

// ---------------------------------------------------------------------------
// aws_cloudfront_distribution
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudfront_distribution_basic() {
    // Isolated server: the provider uses ETag-based conditional updates and
    // calls GetDistributionConfig on every refresh.
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("cloudfront-dist-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_cloudfront_distribution" "cloudfront_distribution_basic" {
  origin {
    domain_name = "example.s3.amazonaws.com"
    origin_id   = "myS3Origin"
  }

  enabled         = true
  is_ipv6_enabled = false
  comment         = "cloudfront-dist-basic"

  default_cache_behavior {
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]
    target_origin_id       = "myS3Origin"
    viewer_protocol_policy = "allow-all"

    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    cloudfront_default_certificate = true
  }

  tags = {
    Name = "cloudfront-dist-basic"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("cloudfront-dist-basic"),
        "state missing distribution"
    );
    assert!(
        state.contains("aws_cloudfront_distribution"),
        "state missing resource type"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudfront_distribution_disabled() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("cloudfront-dist-disabled");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_cloudfront_distribution" "cloudfront_distribution_disabled" {
  origin {
    domain_name = "disabled-example.s3.amazonaws.com"
    origin_id   = "myOriginDisabled"
  }

  enabled = false
  comment = "cloudfront-dist-disabled"

  default_cache_behavior {
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]
    target_origin_id       = "myOriginDisabled"
    viewer_protocol_policy = "redirect-to-https"

    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    cloudfront_default_certificate = true
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("cloudfront-dist-disabled"),
        "state missing distribution comment"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_cloudfront_origin_access_control
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudfront_origin_access_control_basic() {
    let result = batch_apply(
        r#"
resource "aws_cloudfront_origin_access_control" "cloudfront_oac_basic" {
  name                              = "cloudfront-oac-basic"
  origin_access_control_origin_type = "s3"
  signing_behavior                  = "always"
  signing_protocol                  = "sigv4"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("cloudfront-oac-basic"),
        "state missing OAC name"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudfront_origin_access_control_with_description() {
    let result = batch_apply(
        r#"
resource "aws_cloudfront_origin_access_control" "cloudfront_oac_desc" {
  name                              = "cloudfront-oac-desc"
  description                       = "OAC for E2E test"
  origin_access_control_origin_type = "s3"
  signing_behavior                  = "no-override"
  signing_protocol                  = "sigv4"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("cloudfront-oac-desc"),
        "state missing OAC"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudfront_origin_access_control_mediastore() {
    let result = batch_apply(
        r#"
resource "aws_cloudfront_origin_access_control" "cloudfront_oac_mediastore" {
  name                              = "cloudfront-oac-mediastore"
  description                       = "OAC for MediaStore origin"
  origin_access_control_origin_type = "mediastore"
  signing_behavior                  = "always"
  signing_protocol                  = "sigv4"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("cloudfront-oac-mediastore"),
        "state missing OAC"
    );
}

// ---------------------------------------------------------------------------
// Full-stack: distribution + OAC together
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudfront_full_stack() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("cloudfront-full-stack");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_cloudfront_origin_access_control" "cloudfront_full_oac" {
  name                              = "cloudfront-full-stack-oac"
  origin_access_control_origin_type = "s3"
  signing_behavior                  = "always"
  signing_protocol                  = "sigv4"
}

resource "aws_cloudfront_distribution" "cloudfront_full_dist" {
  origin {
    domain_name              = "full-stack.s3.amazonaws.com"
    origin_id                = "fullStackS3Origin"
    origin_access_control_id = aws_cloudfront_origin_access_control.cloudfront_full_oac.id
  }

  enabled         = true
  is_ipv6_enabled = true
  comment         = "cloudfront-full-stack"

  default_cache_behavior {
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]
    target_origin_id       = "fullStackS3Origin"
    viewer_protocol_policy = "https-only"

    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    cloudfront_default_certificate = true
  }

  tags = {
    Environment = "e2e-test"
    Stack       = "full"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("cloudfront-full-stack-oac"),
        "state missing OAC"
    );
    assert!(
        state.contains("cloudfront-full-stack"),
        "state missing distribution"
    );

    cleanup_tf_dir(&dir);
}
