use crate::harness::*;

// EFS terraform resources tested here:
//   aws_efs_file_system
//   aws_efs_access_point
//   aws_efs_file_system_policy
//   aws_efs_backup_policy

// ---------------------------------------------------------------------------
// File system tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_efs_file_system_basic() {
    let result = batch_apply(
        r#"
resource "aws_efs_file_system" "efs_file_system_basic" {
  tags = {
    Name = "efs-basic"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("efs-basic"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_efs_file_system_encrypted() {
    let result = batch_apply(
        r#"
resource "aws_efs_file_system" "efs_file_system_encrypted" {
  encrypted = true

  tags = {
    Name = "efs-encrypted"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("efs-encrypted"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_efs_file_system_provisioned_throughput() {
    let result = batch_apply(
        r#"
resource "aws_efs_file_system" "efs_file_system_provisioned" {
  throughput_mode                 = "provisioned"
  provisioned_throughput_in_mibps = 10

  tags = {
    Name = "efs-provisioned"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("efs-provisioned"));
}

// ---------------------------------------------------------------------------
// Access point tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_efs_access_point_basic() {
    let result = batch_apply(
        r#"
resource "aws_efs_file_system" "efs_ap_fs" {
  tags = {
    Name = "efs-ap-fs"
  }
}

resource "aws_efs_access_point" "efs_access_point_basic" {
  file_system_id = aws_efs_file_system.efs_ap_fs.id

  tags = {
    Name = "efs-access-point"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("efs-access-point"));
}

// ---------------------------------------------------------------------------
// File system policy tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_efs_file_system_policy() {
    let result = batch_apply(
        r#"
resource "aws_efs_file_system" "efs_policy_fs" {
  tags = {
    Name = "efs-policy-fs"
  }
}

resource "aws_efs_file_system_policy" "efs_file_system_policy" {
  file_system_id = aws_efs_file_system.efs_policy_fs.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect    = "Allow"
        Principal = { AWS = "*" }
        Action    = "elasticfilesystem:ClientMount"
        Resource  = aws_efs_file_system.efs_policy_fs.arn
        Condition = {
          Bool = {
            "aws:SecureTransport" = "true"
          }
        }
      }
    ]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("efs-policy-fs"));
}

// ---------------------------------------------------------------------------
// Lifecycle policy and protection tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_efs_file_system_with_lifecycle_policy() {
    let result = batch_apply(
        r#"
resource "aws_efs_file_system" "efs_lifecycle" {
  lifecycle_policy {
    transition_to_ia = "AFTER_30_DAYS"
  }

  lifecycle_policy {
    transition_to_primary_storage_class = "AFTER_1_ACCESS"
  }

  protection {
    replication_overwrite = "ENABLED"
  }

  tags = {
    Name = "efs-lifecycle-test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("efs-lifecycle-test"));
}

// ---------------------------------------------------------------------------
// Full-stack test
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_efs_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_efs_file_system" "efs_full_stack_fs" {
  encrypted       = false
  throughput_mode = "bursting"

  lifecycle_policy {
    transition_to_ia = "AFTER_30_DAYS"
  }

  tags = {
    Stack = "efs-full-stack"
    Name  = "efs-full-stack-fs"
  }
}

resource "aws_efs_access_point" "efs_full_stack_ap" {
  file_system_id = aws_efs_file_system.efs_full_stack_fs.id

  posix_user {
    uid = 1000
    gid = 1000
  }

  root_directory {
    path = "/data"

    creation_info {
      owner_uid   = 1000
      owner_gid   = 1000
      permissions = "755"
    }
  }

  tags = {
    Stack = "efs-full-stack"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("efs-full-stack-fs"));
}
