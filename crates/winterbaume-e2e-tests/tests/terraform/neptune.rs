use crate::harness::*;

// Neptune terraform resources tested here:
//   aws_neptune_cluster (with serverless_v2_scaling_configuration)
//   aws_neptune_cluster_parameter_group (with parameter blocks)

// ---------------------------------------------------------------------------
// aws_neptune_cluster with serverless_v2_scaling_configuration
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_neptune_cluster_serverless() {
    let result = batch_apply(
        r#"
resource "aws_neptune_cluster" "serverless" {
  cluster_identifier  = "neptune-serverless-test"
  engine              = "neptune"
  skip_final_snapshot = true

  serverless_v2_scaling_configuration {
    min_capacity = 1.0
    max_capacity = 8.0
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("neptune-serverless-test"));
}

// ---------------------------------------------------------------------------
// aws_neptune_cluster_parameter_group with parameter blocks
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_neptune_parameter_group_with_params() {
    let result = batch_apply(
        r#"
resource "aws_neptune_cluster_parameter_group" "test" {
  family      = "neptune1.2"
  name        = "test-neptune-params"
  description = "Test parameter group"

  parameter {
    name  = "neptune_enable_audit_log"
    value = "1"
  }

  parameter {
    name         = "neptune_query_timeout"
    value        = "120000"
    apply_method = "pending-reboot"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-neptune-params"));
}
