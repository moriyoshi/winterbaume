use crate::harness::*;

// ---------------------------------------------------------------------------
// Redshift cluster with logging
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_redshift_cluster_with_logging() {
    let result = batch_apply(
        r#"
resource "aws_redshift_cluster" "logged" {
  cluster_identifier  = "redshift-logged-test"
  node_type           = "dc2.large"
  master_username     = "admin"
  master_password     = "Password1!"
  skip_final_snapshot = true
}

resource "aws_redshift_logging" "logged" {
  cluster_identifier = aws_redshift_cluster.logged.id
  log_destination_type = "s3"
  bucket_name        = "my-logging-bucket"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("redshift-logged-test"));
}

// ---------------------------------------------------------------------------
// Redshift cluster with logging and s3_key_prefix
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_redshift_cluster_with_logging_prefix() {
    let result = batch_apply(
        r#"
resource "aws_redshift_cluster" "logged_prefix" {
  cluster_identifier  = "redshift-logged-prefix"
  node_type           = "dc2.large"
  master_username     = "admin"
  master_password     = "Password1!"
  skip_final_snapshot = true
}

resource "aws_redshift_logging" "logged_prefix" {
  cluster_identifier = aws_redshift_cluster.logged_prefix.id
  log_destination_type = "s3"
  bucket_name        = "my-logging-bucket"
  s3_key_prefix      = "redshift-logs/"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("redshift-logged-prefix"));
}

// ---------------------------------------------------------------------------
// Redshift cluster with snapshot_copy
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_redshift_cluster_with_snapshot_copy() {
    let result = batch_apply(
        r#"
resource "aws_redshift_cluster" "snap_copy" {
  cluster_identifier  = "redshift-snap-copy"
  node_type           = "dc2.large"
  master_username     = "admin"
  master_password     = "Password1!"
  skip_final_snapshot = true
}

resource "aws_redshift_snapshot_copy" "snap_copy" {
  cluster_identifier = aws_redshift_cluster.snap_copy.id
  destination_region = "eu-west-1"
  retention_period   = 7
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("redshift-snap-copy"));
}
