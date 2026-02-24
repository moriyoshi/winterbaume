use crate::harness::*;

// AWS Transfer terraform resources tested here:
//   aws_transfer_server (CreateServer / DescribeServer / UpdateServer / DeleteServer)
//   aws_transfer_workflow (CreateWorkflow / DescribeWorkflow / DeleteWorkflow)
//   aws_transfer_user (CreateUser / DescribeUser / DeleteUser)
//   aws_transfer_connector (CreateConnector / DescribeConnector / DeleteConnector)
//
// Previously-known limitations (now fixed):
//   - aws_transfer_user: server_id was 32-char UUID hex; now truncated to 19 chars
//     (s- + 17 hex chars) matching terraform's 17-19 char validation.
//   - aws_transfer_connector: DescribeConnector now returns Status "ACTIVE" so
//     terraform no longer waits indefinitely.

// ---------------------------------------------------------------------------
// Server tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_transfer_server_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("transfer-server-basic");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_transfer_server" "basic" {
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_transfer_server"),
        "state should contain aws_transfer_server resource"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_transfer_server_sftp() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("transfer-server-sftp");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_transfer_server" "sftp" {
  endpoint_type          = "PUBLIC"
  identity_provider_type = "SERVICE_MANAGED"
  protocols              = ["SFTP"]
  domain                 = "S3"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("SFTP"), "state should contain SFTP protocol");
    assert!(
        state.contains("PUBLIC"),
        "state should contain PUBLIC endpoint type"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_transfer_server_with_tags() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("transfer-server-tags");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_transfer_server" "tagged" {
  protocols = ["SFTP"]

  tags = {
    Environment = "test"
    Name        = "transfer-server-tags"
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("transfer-server-tags"),
        "state should contain tag value"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Workflow tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_transfer_workflow_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("transfer-workflow-basic");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_transfer_workflow" "basic" {
  steps {
    type = "DELETE"
    delete_step_details {
      name                 = "delete-step"
      source_file_location = "$${original.file}"
    }
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_transfer_workflow"),
        "state should contain aws_transfer_workflow resource"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_transfer_workflow_with_description() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("transfer-workflow-desc");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_transfer_workflow" "desc" {
  description = "Test workflow with description"

  steps {
    type = "DELETE"
    delete_step_details {
      name                 = "delete-desc-step"
      source_file_location = "$${original.file}"
    }
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("Test workflow with description"),
        "state should contain workflow description"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_transfer_workflow_with_tags() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("transfer-workflow-tags");
    write_provider_tf(&dir, &url);

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_transfer_workflow" "tagged" {
  description = "Test workflow with tags"

  steps {
    type = "DELETE"
    delete_step_details {
      name                 = "delete-tagged-step"
      source_file_location = "$${original.file}"
    }
  }

  tags = {
    Environment = "test"
    Name        = "transfer-workflow-tags"
  }
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("transfer-workflow-tags"),
        "state should contain tag value"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// User tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_transfer_user_basic() {
    let result = batch_apply(
        r#"
resource "aws_iam_role" "transfer_user_role" {
  name = "transfer-user-role"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = { Service = "transfer.amazonaws.com" }
    }]
  })
}

resource "aws_transfer_server" "transfer_user_server" {
  identity_provider_type = "SERVICE_MANAGED"
  protocols              = ["SFTP"]
}

resource "aws_transfer_user" "transfer_user_basic" {
  server_id = aws_transfer_server.transfer_user_server.id
  user_name = "transfer-test-user"
  role      = aws_iam_role.transfer_user_role.arn
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("transfer-test-user"));
}

// ---------------------------------------------------------------------------
// Connector tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_transfer_connector_basic() {
    let result = batch_apply(
        r#"
resource "aws_iam_role" "transfer_connector_role" {
  name = "transfer-connector-role"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = { Service = "transfer.amazonaws.com" }
    }]
  })
}

resource "aws_transfer_connector" "transfer_connector_basic" {
  access_role = aws_iam_role.transfer_connector_role.arn
  url         = "sftp://test.example.com"

  sftp_config {
    trusted_host_keys = ["ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQDEexamplekey"]
    user_secret_id    = "arn:aws:secretsmanager:us-east-1:000000000000:secret:sftp-creds"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("sftp://test.example.com"));
}
