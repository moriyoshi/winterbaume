use crate::harness::*;

// ---------------------------------------------------------------------------
// aws_pinpoint_app
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_pinpoint_app_basic() {
    let result = batch_apply(
        r#"
resource "aws_pinpoint_app" "pinpoint_app_basic" {
  name = "pinpoint-app-basic"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("pinpoint-app-basic"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_pinpoint_app_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_pinpoint_app" "pinpoint_app_with_tags" {
  name = "pinpoint-app-with-tags"

  tags = {
    Environment = "test"
    Purpose     = "e2e-tags"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("pinpoint-app-with-tags"));
    assert!(result.state.contains("e2e-tags"));
}

// ---------------------------------------------------------------------------
// aws_pinpoint_email_channel — attached to an aws_pinpoint_app
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_pinpoint_email_channel_basic() {
    let result = batch_apply(
        r#"
resource "aws_pinpoint_app" "pinpoint_email_channel_basic" {
  name = "pinpoint-email-channel-basic-app"
}

resource "aws_pinpoint_email_channel" "pinpoint_email_channel_basic" {
  application_id = aws_pinpoint_app.pinpoint_email_channel_basic.application_id
  from_address   = "noreply@example.com"
  identity       = "arn:aws:ses:us-east-1:123456789012:identity/example.com"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("pinpoint-email-channel-basic-app"));
    assert!(result.state.contains("noreply@example.com"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_pinpoint_email_channel_with_role_arn() {
    let result = batch_apply(
        r#"
resource "aws_pinpoint_app" "pinpoint_email_channel_role" {
  name = "pinpoint-email-channel-role-app"
}

resource "aws_pinpoint_email_channel" "pinpoint_email_channel_role" {
  application_id = aws_pinpoint_app.pinpoint_email_channel_role.application_id
  from_address   = "alerts@example.org"
  identity       = "arn:aws:ses:us-east-1:123456789012:identity/example.org"
  role_arn       = "arn:aws:iam::123456789012:role/pinpoint-email-role"
  enabled        = true
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("pinpoint-email-channel-role-app"));
    assert!(result.state.contains("alerts@example.org"));
    assert!(result.state.contains("pinpoint-email-role"));
}
