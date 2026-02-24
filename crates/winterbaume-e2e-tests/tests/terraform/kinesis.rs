// NOTE: All kinesis tests use isolated servers (not batch_apply) because the terraform
// AWS provider triggers StopStreamEncryption with empty EncryptionType (AWS SDK client-side
// validation error) whenever kinesis streams are refreshed in a subsequent batch wave.
// This is a provider bug: the provider calls StopStreamEncryption unconditionally during
// certain update paths even when encryption is already disabled. Isolated servers guarantee
// no cross-batch state contamination.
use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_kinesis_stream_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("kinesis-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_kinesis_stream" "kinesis_stream_basic" {
  name        = "terraform-test-stream"
  shard_count = 1

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("terraform-test-stream"));
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_kinesis_stream_provisioned_multi_shard() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("kinesis-multi-shard");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_kinesis_stream" "kinesis_stream_multi" {
  name        = "terraform-multi-shard-stream"
  shard_count = 2

  tags = {
    Purpose = "testing"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("terraform-multi-shard-stream"));
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_kinesis_stream_on_demand() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("kinesis-on-demand");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_kinesis_stream" "kinesis_stream_on_demand" {
  name = "terraform-on-demand-stream"

  stream_mode_details {
    stream_mode = "ON_DEMAND"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("terraform-on-demand-stream"));
    cleanup_tf_dir(&dir);
}

// NOTE: test_kinesis_stream_modify_in_place is deliberately absent. The
// terraform AWS provider always calls StopStreamEncryption during any stream
// update, passing an empty EncryptionType which fails AWS SDK client
// validation before the request reaches the server. Documented provider bug.

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_kinesis_full_stack() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("kinesis-full-stack");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_kinesis_stream" "kinesis_full_ingest" {
  name        = "kinesis-full-ingest-stream"
  shard_count = 1

  tags = {
    Stack = "full"
    Role  = "ingest"
  }
}

resource "aws_kinesis_stream" "kinesis_full_process" {
  name        = "kinesis-full-process-stream"
  shard_count = 1

  tags = {
    Stack = "full"
    Role  = "process"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("kinesis-full-ingest-stream"));
    assert!(state.contains("kinesis-full-process-stream"));
    cleanup_tf_dir(&dir);
}
