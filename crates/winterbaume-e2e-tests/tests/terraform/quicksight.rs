use crate::harness::*;

// ---------------------------------------------------------------------------
// aws_quicksight_data_source
//
// QuickSight resources accept an `aws_account_id` argument; the terraform
// converter falls back to `ctx.default_account_id` (the standard test account
// "123456789012"), so we hardcode the same value here for determinism rather
// than relying on `aws_caller_identity`.
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_quicksight_data_source_basic() {
    let result = batch_apply(
        r#"
resource "aws_quicksight_data_source" "quicksight_data_source_basic" {
  aws_account_id = "123456789012"
  data_source_id = "tf-qs-ds-basic"
  name           = "tf-qs-ds-basic-name"
  type           = "S3"

  parameters {
    s3 {
      manifest_file_location {
        bucket = "tf-qs-ds-bucket"
        key    = "manifest.json"
      }
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("tf-qs-ds-basic"));
    assert!(result.state.contains("tf-qs-ds-basic-name"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_quicksight_data_source_with_permissions() {
    let result = batch_apply(
        r#"
resource "aws_quicksight_data_source" "quicksight_data_source_perms" {
  aws_account_id = "123456789012"
  data_source_id = "tf-qs-ds-perms"
  name           = "tf-qs-ds-perms-name"
  type           = "S3"

  parameters {
    s3 {
      manifest_file_location {
        bucket = "tf-qs-perms-bucket"
        key    = "manifest.json"
      }
    }
  }

  permission {
    actions = [
      "quicksight:DescribeDataSource",
      "quicksight:DescribeDataSourcePermissions",
    ]
    principal = "arn:aws:quicksight:us-east-1:123456789012:user/default/tf-qs-perm-user"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("tf-qs-ds-perms"));
    assert!(result.state.contains("quicksight:DescribeDataSource"));
}

// ---------------------------------------------------------------------------
// aws_quicksight_group
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_quicksight_group_basic() {
    let result = batch_apply(
        r#"
resource "aws_quicksight_group" "quicksight_group_basic" {
  aws_account_id = "123456789012"
  group_name     = "tf-qs-group-basic"
  description    = "tf-qs-group-basic-description"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("tf-qs-group-basic"));
    assert!(result.state.contains("tf-qs-group-basic-description"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_quicksight_group_with_namespace() {
    let result = batch_apply(
        r#"
resource "aws_quicksight_group" "quicksight_group_namespaced" {
  aws_account_id = "123456789012"
  group_name     = "tf-qs-group-ns"
  namespace      = "default"
  description    = "tf-qs-group-namespaced"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("tf-qs-group-ns"));
    assert!(result.state.contains("tf-qs-group-namespaced"));
}

// ---------------------------------------------------------------------------
// aws_quicksight_user
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_quicksight_user_iam() {
    let result = batch_apply(
        r#"
resource "aws_quicksight_user" "quicksight_user_iam" {
  aws_account_id = "123456789012"
  user_name      = "tf-qs-user-iam"
  email          = "tf-qs-user-iam@example.com"
  identity_type  = "IAM"
  user_role      = "READER"
  iam_arn        = "arn:aws:iam::123456789012:user/tf-qs-user-iam"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("tf-qs-user-iam"));
    assert!(result.state.contains("tf-qs-user-iam@example.com"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_quicksight_user_quicksight_identity() {
    let result = batch_apply(
        r#"
resource "aws_quicksight_user" "quicksight_user_qs" {
  aws_account_id = "123456789012"
  user_name      = "tf-qs-user-qs"
  email          = "tf-qs-user-qs@example.com"
  identity_type  = "QUICKSIGHT"
  user_role      = "AUTHOR"
  namespace      = "default"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("tf-qs-user-qs"));
    assert!(result.state.contains("AUTHOR"));
}

// ---------------------------------------------------------------------------
// Combined: multiple QuickSight resource types in one apply
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_quicksight_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_quicksight_data_source" "quicksight_full_ds" {
  aws_account_id = "123456789012"
  data_source_id = "tf-qs-full-ds"
  name           = "tf-qs-full-ds-name"
  type           = "S3"

  parameters {
    s3 {
      manifest_file_location {
        bucket = "tf-qs-full-bucket"
        key    = "manifest.json"
      }
    }
  }
}

resource "aws_quicksight_group" "quicksight_full_group" {
  aws_account_id = "123456789012"
  group_name     = "tf-qs-full-group"
  description    = "tf-qs-full-group-description"
}

resource "aws_quicksight_user" "quicksight_full_user" {
  aws_account_id = "123456789012"
  user_name      = "tf-qs-full-user"
  email          = "tf-qs-full-user@example.com"
  identity_type  = "IAM"
  user_role      = "READER"
  iam_arn        = "arn:aws:iam::123456789012:user/tf-qs-full-user"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("tf-qs-full-ds"));
    assert!(result.state.contains("tf-qs-full-group"));
    assert!(result.state.contains("tf-qs-full-user"));
}
