use crate::harness::*;

// ---------------------------------------------------------------------------
// aws_appflow_flow
// ---------------------------------------------------------------------------

// Note: terraform-provider-aws's aws_appflow_flow requires a real
// AWS-managed connector profile for non-S3 sources. This test uses S3 -> S3
// because S3 connector profiles are AWS-managed by default and don't require
// CreateConnectorProfile (which winterbaume-appflow stubs as 501).

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_appflow_flow_s3_to_s3() {
    let result = batch_apply(
        r#"
resource "aws_s3_bucket" "appflow_src" {
  bucket = "appflow-source-bucket-test"
}

resource "aws_s3_bucket" "appflow_dst" {
  bucket = "appflow-dest-bucket-test"
}

resource "aws_appflow_flow" "appflow_flow_basic" {
  name = "test-appflow-flow"

  source_flow_config {
    connector_type = "S3"

    source_connector_properties {
      s3 {
        bucket_name = aws_s3_bucket.appflow_src.bucket
        bucket_prefix = "input/"
      }
    }
  }

  destination_flow_config {
    connector_type = "S3"

    destination_connector_properties {
      s3 {
        bucket_name = aws_s3_bucket.appflow_dst.bucket
      }
    }
  }

  task {
    source_fields     = ["col1"]
    destination_field = "col1"
    task_type         = "Map"

    connector_operator {
      s3 = "NO_OP"
    }
  }

  trigger_config {
    trigger_type = "OnDemand"
  }

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-appflow-flow"));
}
