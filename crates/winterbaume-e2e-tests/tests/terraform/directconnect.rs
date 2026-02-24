use crate::harness::*;

// Direct Connect terraform resources tested here:
//   aws_dx_connection  (CreateConnection, DescribeConnections, DeleteConnection)

// ---------------------------------------------------------------------------
// aws_dx_connection
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_directconnect_connection_basic() {
    let result = batch_apply(
        r#"
resource "aws_dx_connection" "directconnect_connection_basic" {
  name      = "directconnect-connection-basic"
  bandwidth = "1Gbps"
  location  = "EqDC2"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("directconnect-connection-basic"));
    assert!(result.state.contains("EqDC2"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_directconnect_connection_with_tags() {
    let result = batch_apply(
        r#"
resource "aws_dx_connection" "directconnect_connection_with_tags" {
  name      = "directconnect-connection-tagged"
  bandwidth = "10Gbps"
  location  = "EqDC2"

  tags = {
    Environment = "test"
    Purpose     = "e2e"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("directconnect-connection-tagged"));
    assert!(result.state.contains("10Gbps"));
}
