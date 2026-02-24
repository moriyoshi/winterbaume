use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_firehose_delivery_stream_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("firehose-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_s3_bucket" "firehose_dest" {
  bucket = "firehose-test-dest-bucket"
}

resource "aws_kinesis_firehose_delivery_stream" "firehose_basic" {
  name        = "terraform-test-firehose-stream"
  destination = "extended_s3"

  extended_s3_configuration {
    role_arn   = "arn:aws:iam::123456789012:role/firehose-role"
    bucket_arn = aws_s3_bucket.firehose_dest.arn
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("terraform-test-firehose-stream"));
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_firehose_delivery_stream_with_tags() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("firehose-tags");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_s3_bucket" "firehose_tags_dest" {
  bucket = "firehose-tags-dest-bucket"
}

resource "aws_kinesis_firehose_delivery_stream" "firehose_tags" {
  name        = "terraform-tagged-firehose-stream"
  destination = "extended_s3"

  extended_s3_configuration {
    role_arn   = "arn:aws:iam::123456789012:role/firehose-role"
    bucket_arn = aws_s3_bucket.firehose_tags_dest.arn
  }

  tags = {
    Environment = "test"
    Purpose     = "e2e"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("terraform-tagged-firehose-stream"));
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_firehose_delivery_stream_destroy() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("firehose-destroy");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_s3_bucket" "firehose_destroy_dest" {
  bucket = "firehose-destroy-dest-bucket"
}

resource "aws_kinesis_firehose_delivery_stream" "firehose_destroy" {
  name        = "terraform-destroy-firehose-stream"
  destination = "extended_s3"

  extended_s3_configuration {
    role_arn   = "arn:aws:iam::123456789012:role/firehose-role"
    bucket_arn = aws_s3_bucket.firehose_destroy_dest.arn
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    // Now destroy
    let (ok, _stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    cleanup_tf_dir(&dir);
}
