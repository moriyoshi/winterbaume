use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_workspaces_ip_group_basic() {
    let result = batch_apply(
        r#"
resource "aws_workspaces_ip_group" "workspaces_ip_group_basic" {
  name        = "terraform-test-ip-group"
  description = "Basic IP group for E2E testing"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-ip-group"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_workspaces_ip_group_with_rules() {
    let result = batch_apply(
        r#"
resource "aws_workspaces_ip_group" "workspaces_ip_group_rules" {
  name        = "terraform-test-ip-group-rules"
  description = "IP group with rules"

  rules {
    source      = "10.0.0.0/16"
    description = "Home network"
  }

  rules {
    source      = "192.168.0.0/24"
    description = "Office network"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-ip-group-rules"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_workspaces_ip_group_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_workspaces_ip_group" "workspaces_ip_group_tags" {
  name        = "terraform-test-ip-group-tags"
  description = "IP group with tags"

  tags = {
    Environment = "test"
    Purpose     = "e2e"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("terraform-test-ip-group-tags"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_workspaces_connection_alias_basic() {
    let result = batch_apply(
        r#"
resource "aws_workspaces_connection_alias" "workspaces_conn_alias_basic" {
  connection_string = "testdomain.example.com"

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("testdomain.example.com"));
}
