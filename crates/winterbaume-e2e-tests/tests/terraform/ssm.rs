use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ssm_parameter_string() {
    let result = batch_apply(
        r#"
resource "aws_ssm_parameter" "ssm_param_string" {
  name  = "/test/string-param"
  type  = "String"
  value = "hello-world"

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("/test/string-param"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ssm_parameter_secure_string() {
    let result = batch_apply(
        r#"
resource "aws_ssm_parameter" "ssm_param_secure" {
  name  = "/test/secure-param"
  type  = "SecureString"
  value = "secret-value"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("/test/secure-param"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ssm_parameter_string_list() {
    let result = batch_apply(
        r#"
resource "aws_ssm_parameter" "ssm_param_list" {
  name  = "/test/list-param"
  type  = "StringList"
  value = "one,two,three"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("/test/list-param"));
}

// test_ssm_parameter_modify_in_place requires two sequential apply passes
// (apply initial config, then apply modified config). It uses its own isolated
// server and directory rather than batch_apply.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ssm_parameter_modify_in_place() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ssm-param-modify");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_ssm_parameter" "test" {
  name  = "/test/modify-param"
  type  = "String"
  value = "initial-value"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "first apply failed:\n{stderr}");

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_ssm_parameter" "test" {
  name  = "/test/modify-param"
  type  = "String"
  value = "updated-value"
}
"#,
    )
    .unwrap();

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second apply (modify) failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 0 added, 1 changed, 0 destroyed"),
        "Expected in-place update:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_ssm_document
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ssm_document_command() {
    let result = batch_apply(
        r#"
resource "aws_ssm_document" "ssm_doc_cmd" {
  name            = "TestCommandDocument"
  document_type   = "Command"
  document_format = "JSON"

  content = jsonencode({
    schemaVersion = "2.2"
    description   = "Run a simple command"
    mainSteps = [{
      action = "aws:runShellScript"
      name   = "runShell"
      inputs = {
        runCommand = ["echo hello"]
      }
    }]
  })

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("TestCommandDocument"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ssm_document_automation() {
    let result = batch_apply(
        r#"
resource "aws_ssm_document" "ssm_doc_auto" {
  name            = "TestAutomationDocument"
  document_type   = "Automation"
  document_format = "JSON"

  content = jsonencode({
    schemaVersion = "0.3"
    description   = "Test automation doc"
    mainSteps = [{
      name   = "step1"
      action = "aws:sleep"
      inputs = {
        Duration = "PT1S"
      }
    }]
  })
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("TestAutomationDocument"));
}

// ---------------------------------------------------------------------------
// aws_ssm_maintenance_window
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ssm_maintenance_window() {
    let result = batch_apply(
        r#"
resource "aws_ssm_maintenance_window" "ssm_mw" {
  name     = "test-maintenance-window"
  schedule = "cron(0 16 ? * TUE *)"
  duration = 3
  cutoff   = 1
  enabled  = true
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-maintenance-window"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ssm_maintenance_window_disabled() {
    let result = batch_apply(
        r#"
resource "aws_ssm_maintenance_window" "ssm_mw_disabled" {
  name     = "disabled-maintenance-window"
  schedule = "cron(0 16 ? * TUE *)"
  duration = 2
  cutoff   = 0
  enabled  = false
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("disabled-maintenance-window"));
}

// ---------------------------------------------------------------------------
// aws_ssm_maintenance_window_target
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ssm_maintenance_window_target() {
    let result = batch_apply(
        r#"
resource "aws_ssm_maintenance_window" "ssm_mw_for_target" {
  name     = "mw-for-target-test"
  schedule = "cron(0 16 ? * TUE *)"
  duration = 3
  cutoff   = 1
}

resource "aws_ssm_maintenance_window_target" "ssm_mw_target" {
  window_id     = aws_ssm_maintenance_window.ssm_mw_for_target.id
  resource_type = "INSTANCE"

  targets {
    key    = "tag:Environment"
    values = ["production"]
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("mw-for-target-test"));
    assert!(result.state.contains("INSTANCE"));
}

// ---------------------------------------------------------------------------
// aws_ssm_patch_baseline
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ssm_patch_baseline() {
    let result = batch_apply(
        r#"
resource "aws_ssm_patch_baseline" "ssm_pb" {
  name             = "test-patch-baseline"
  operating_system = "AMAZON_LINUX_2"
  description      = "Patch baseline for E2E testing"

  approved_patches = ["KB123456"]

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-patch-baseline"));
}

// ---------------------------------------------------------------------------
// aws_ssm_association (depends on a document)
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ssm_association() {
    let result = batch_apply(
        r#"
resource "aws_ssm_document" "ssm_doc_for_assoc" {
  name            = "AssocTestDocument"
  document_type   = "Command"
  document_format = "JSON"

  content = jsonencode({
    schemaVersion = "2.2"
    description   = "Document for association test"
    mainSteps = [{
      action = "aws:runShellScript"
      name   = "runShell"
      inputs = {
        runCommand = ["echo test"]
      }
    }]
  })
}

resource "aws_ssm_association" "ssm_assoc" {
  name             = aws_ssm_document.ssm_doc_for_assoc.name
  association_name = "test-association"

  targets {
    key    = "tag:Environment"
    values = ["test"]
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-association"));
}

// ---------------------------------------------------------------------------
// Multi-step: modify maintenance window in place
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ssm_maintenance_window_modify_in_place() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ssm-mw-modify");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_ssm_maintenance_window" "test" {
  name     = "modify-mw-test"
  schedule = "cron(0 16 ? * TUE *)"
  duration = 3
  cutoff   = 1
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "first apply failed:\n{stderr}");

    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_ssm_maintenance_window" "test" {
  name     = "modify-mw-test"
  schedule = "cron(0 18 ? * WED *)"
  duration = 4
  cutoff   = 2
}
"#,
    )
    .unwrap();

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second apply (modify) failed:\n{stderr}");
    assert!(
        stdout.contains("1 changed"),
        "Expected in-place update:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}
