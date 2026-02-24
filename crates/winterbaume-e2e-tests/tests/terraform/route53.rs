use crate::harness::*;

// Route53 terraform resources tested here:
//   aws_route53_zone
//   aws_route53_record
//   aws_route53_health_check

// ---------------------------------------------------------------------------
// Hosted zone tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_route53_zone_basic() {
    let result = batch_apply(
        r#"
resource "aws_route53_zone" "route53_zone_basic" {
  name = "route53-basic.example.com"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("route53-basic.example.com"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_route53_zone_private() {
    let result = batch_apply(
        r#"
resource "aws_route53_zone" "route53_zone_private" {
  name = "route53-private.internal"

  vpc {
    vpc_id = "vpc-00000000"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("route53-private.internal"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_route53_zone_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_route53_zone" "route53_zone_tags" {
  name = "route53-tags.example.com"

  tags = {
    Environment = "test"
    Name        = "route53-tags"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("route53-tags.example.com"));
}

// ---------------------------------------------------------------------------
// Record tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_route53_record_a() {
    let result = batch_apply(
        r#"
resource "aws_route53_zone" "route53_record_a_zone" {
  name = "route53-record-a.example.com"
}

resource "aws_route53_record" "route53_record_a" {
  zone_id = aws_route53_zone.route53_record_a_zone.zone_id
  name    = "www.route53-record-a.example.com"
  type    = "A"
  ttl     = 300
  records = ["1.2.3.4"]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("route53-record-a.example.com"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_route53_record_cname() {
    let result = batch_apply(
        r#"
resource "aws_route53_zone" "route53_record_cname_zone" {
  name = "route53-record-cname.example.com"
}

resource "aws_route53_record" "route53_record_cname" {
  zone_id = aws_route53_zone.route53_record_cname_zone.zone_id
  name    = "api.route53-record-cname.example.com"
  type    = "CNAME"
  ttl     = 60
  records = ["backend.example.com"]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("route53-record-cname.example.com"));
}

// ---------------------------------------------------------------------------
// Full-stack test
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_route53_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_route53_zone" "route53_full_stack_zone" {
  name = "route53-full-stack.example.com"

  tags = {
    Stack = "full-stack"
  }
}

resource "aws_route53_record" "route53_full_stack_a" {
  zone_id = aws_route53_zone.route53_full_stack_zone.zone_id
  name    = "app.route53-full-stack.example.com"
  type    = "A"
  ttl     = 300
  records = ["10.0.0.1"]
}

resource "aws_route53_record" "route53_full_stack_txt" {
  zone_id = aws_route53_zone.route53_full_stack_zone.zone_id
  name    = "route53-full-stack.example.com"
  type    = "TXT"
  ttl     = 300
  records = ["v=spf1 include:amazonses.com ~all"]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("route53-full-stack.example.com"));
}
