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

// ---------------------------------------------------------------------------
// Destroy lifecycle — conditional-header regression.
// ---------------------------------------------------------------------------
//
// The Terraform AWS provider's `aws_cloudfront_distribution` destroy path is:
//
//   1. UpdateDistribution with `Enabled = false` and `If-Match: <current ETag>`
//      (provider obtains the ETag from a fresh GetDistribution).
//   2. Poll GetDistribution until `Status = "Deployed"`.
//   3. DeleteDistribution with `If-Match: <ETag from step 1>`.
//
// Real CloudFront rejects steps 1 and 3 with 412 PreconditionFailed when the
// ETag is stale. winterbaume now matches that contract on DeleteDistribution
// (it already matched on UpdateDistribution); this E2E test validates that
// the enforcement does not break the provider's normal destroy lifecycle.
// `aws_cloudfront_origin_access_control` has the same conditional surface on
// destroy, so the test also exercises an OAC alongside the distribution.

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudfront_distribution_destroy_lifecycle() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("cloudfront-dist-destroy");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_cloudfront_origin_access_control" "destroy_oac" {
  name                              = "cloudfront-destroy-oac"
  origin_access_control_origin_type = "s3"
  signing_behavior                  = "always"
  signing_protocol                  = "sigv4"
}

resource "aws_cloudfront_distribution" "destroy_dist" {
  origin {
    domain_name              = "destroy.s3.amazonaws.com"
    origin_id                = "destroyOrigin"
    origin_access_control_id = aws_cloudfront_origin_access_control.destroy_oac.id
  }

  enabled = true
  comment = "cloudfront-destroy-lifecycle"

  default_cache_behavior {
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]
    target_origin_id       = "destroyOrigin"
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
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (apply_ok, _stdout, apply_stderr) = terraform_apply(&dir).await;
    assert!(apply_ok, "terraform apply failed:\n{apply_stderr}");

    // The destroy path is where the conditional-header contract is exercised:
    // the provider disables the distribution (UpdateDistribution + If-Match),
    // polls for `Status = Deployed`, and only then issues DeleteDistribution
    // with the post-update ETag. A regression where DeleteDistribution
    // rejects a correct ETag would surface here as a failed destroy.
    let (destroy_ok, _destroy_stdout, destroy_stderr) = terraform_destroy(&dir).await;
    assert!(
        destroy_ok,
        "terraform destroy failed; conditional-header enforcement likely \
         rejected a correct ETag on DeleteDistribution or DeleteOriginAccessControl. \
         stderr:\n{destroy_stderr}"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Destroy lifecycle for secondary CloudFront resources.
//
// `aws_cloudfront_public_key`, `aws_cloudfront_key_group`,
// `aws_cloudfront_origin_request_policy`, and
// `aws_cloudfront_response_headers_policy` each have an If-Match-bearing
// Delete in the Smithy model. The CloudFront Terraform AWS provider sends
// `If-Match` on every destroy. Before the 2026-05-23 fix winterbaume
// silently accepted any value (Pattern A: `let _ = if_match`; Pattern B:
// dispatcher dropped it entirely), so this kind of stack would only have
// regressed if the provider sent a deliberately wrong tag — which it
// doesn't. The test below validates that the new enforcement does not
// break the provider's normal destroy lifecycle for all four resources at
// once.
//
// `aws_cloudfront_continuous_deployment_policy` is also covered by the
// If-Match enforcement, but the provider's resource requires an existing
// `staging_distribution_dns_names` block whose value is the DNS name of a
// staging CloudFront distribution. Modelling that in a self-contained
// terraform fixture inflates the test substantially, so it is omitted
// here and tracked as a follow-up.

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_cloudfront_secondary_resources_destroy_lifecycle() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("cloudfront-secondary-destroy");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_cloudfront_public_key" "destroy_pk" {
  name        = "cloudfront-destroy-pk"
  comment     = "If-Match destroy regression"
  encoded_key = <<-EOT
    -----BEGIN PUBLIC KEY-----
    MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEZh3yWl2Re6gd5LBfMs5W6tQVeRwz
    QcaTk0HoQv9R8jzGZw8s7m6L0Z6Bv0xv8FQ6RAW8tEHHFXfFszXVlOklBQ==
    -----END PUBLIC KEY-----
  EOT
}

resource "aws_cloudfront_key_group" "destroy_kg" {
  name  = "cloudfront-destroy-kg"
  items = [aws_cloudfront_public_key.destroy_pk.id]
}

resource "aws_cloudfront_origin_request_policy" "destroy_orp" {
  name    = "cloudfront-destroy-orp"
  comment = "If-Match destroy regression"

  cookies_config {
    cookie_behavior = "none"
  }
  headers_config {
    header_behavior = "none"
  }
  query_strings_config {
    query_string_behavior = "none"
  }
}

resource "aws_cloudfront_response_headers_policy" "destroy_rhp" {
  name    = "cloudfront-destroy-rhp"
  comment = "If-Match destroy regression"

  security_headers_config {
    content_type_options {
      override = false
    }
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (apply_ok, _apply_stdout, apply_stderr) = terraform_apply(&dir).await;
    assert!(apply_ok, "terraform apply failed:\n{apply_stderr}");

    // The destroy path issues four conditional DELETEs, one per resource.
    // Each provider call sends `If-Match: <ETag from the GET it just ran>`,
    // and the mock must accept the matching tag. A regression in
    // `delete_<thing>` enforcement would surface as a failed destroy here.
    let (destroy_ok, _destroy_stdout, destroy_stderr) = terraform_destroy(&dir).await;
    assert!(
        destroy_ok,
        "terraform destroy failed; If-Match enforcement on one of the four \
         secondary CloudFront resources likely rejected a correct ETag. \
         stderr:\n{destroy_stderr}"
    );

    cleanup_tf_dir(&dir);
}
