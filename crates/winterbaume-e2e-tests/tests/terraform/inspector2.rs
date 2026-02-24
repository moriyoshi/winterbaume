use crate::harness::*;

// Inspector2 terraform resources tested here:
//   aws_inspector2_enabler  (Enable, Disable, BatchGetAccountStatus)
//
// Not tested:
//   - aws_inspector2_organization_configuration, aws_inspector2_delegated_admin_account,
//     aws_inspector2_member_association: no terraform converters are registered for
//     these resource types in `crates/winterbaume-terraform/src/converters/inspector2.rs`.
//     Although the underlying handlers exist, the converter exposes only
//     `aws_inspector2_enabler`, so other resource types cannot round-trip through
//     terraform apply against the in-process server.
//
// Account-id isolation:
//   `aws_inspector2_enabler` is keyed by account_id at the API level. When two
//   tests in the same `batch_apply` wave target the same account_id with
//   different `resource_types`, after both Enable calls the API reports the
//   union of resource types — and each resource then sees drift, triggers
//   Update -> Disable -> BatchGetAccountStatus polling, and the wave deadlines
//   out at 5m, cascading failures into every other test in the same wave (iam,
//   iot, etc.). To avoid this, each test below uses a distinct account_id.
//   See SKILL.md Principle 9.

// ---------------------------------------------------------------------------
// aws_inspector2_enabler
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_inspector2_enabler_ec2_only() {
    let result = batch_apply(
        r#"
resource "aws_inspector2_enabler" "inspector2_enabler_ec2_only" {
  account_ids    = ["111111111111"]
  resource_types = ["EC2"]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("aws_inspector2_enabler"),
        "state missing inspector2 enabler resource"
    );
    assert!(
        result.state.contains("EC2"),
        "state missing EC2 resource type"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_inspector2_enabler_ec2_and_ecr() {
    let result = batch_apply(
        r#"
resource "aws_inspector2_enabler" "inspector2_enabler_ec2_ecr" {
  account_ids    = ["222222222222"]
  resource_types = ["EC2", "ECR"]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("aws_inspector2_enabler"),
        "state missing inspector2 enabler resource"
    );
    assert!(
        result.state.contains("ECR"),
        "state missing ECR resource type"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_inspector2_enabler_lambda() {
    let result = batch_apply(
        r#"
resource "aws_inspector2_enabler" "inspector2_enabler_lambda" {
  account_ids    = ["333333333333"]
  resource_types = ["LAMBDA"]
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(
        result.state.contains("aws_inspector2_enabler"),
        "state missing inspector2 enabler resource"
    );
    assert!(
        result.state.contains("LAMBDA"),
        "state missing LAMBDA resource type"
    );
}
