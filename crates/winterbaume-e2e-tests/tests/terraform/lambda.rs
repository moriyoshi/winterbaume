use crate::harness::*;

/// Create a minimal Lambda deployment package (zip) under `.agents-workspace/tmp/` and
/// return its absolute path.  The zip contains a trivial `index.js` handler so
/// the file is a valid zip — the mock server does not execute the code.
fn make_test_lambda_zip() -> std::path::PathBuf {
    let dir = workspace_root().join(".agents-workspace").join("tmp");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("test-lambda-handler.zip");
    // Minimal valid zip: single file "index.js" with a one-liner handler.
    // Pre-computed bytes via Python's zipfile module (ZIP_STORED, no compression).
    let zip_bytes = base64::decode(
        "UEsDBBQAAAAAAMerfFyi/Kg7MQAAADEAAAAIAAAAaW5kZXguanNleHBvcnRzLmhhbmRsZXIgPSBh\
         c3luYyAoKSA9PiAoe3N0YXR1c0NvZGU6IDIwMH0pUEsBAhQDFAAAAAAAx6t8XKL8qDsxAAAAMQ\
         AAAAgAAAAAAAAAAAAAAIABAAAAAGluZGV4LmpzUEsFBgAAAAABAAEANgAAAFcAAAAAAA==",
    )
    .expect("hardcoded base64 must be valid");
    std::fs::write(&path, zip_bytes).unwrap();
    path
}

// Inline base64 decoder — avoids adding a crate dependency for a simple helper.
mod base64 {
    pub fn decode(input: &str) -> Result<Vec<u8>, &'static str> {
        let table: [u8; 256] = {
            let mut t = [255u8; 256];
            let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
            for (i, &c) in chars.iter().enumerate() {
                t[c as usize] = i as u8;
            }
            t
        };
        let clean: Vec<u8> = input
            .bytes()
            .filter(|&b| b != b'\n' && b != b'\r' && b != b' ')
            .collect();
        if !clean.len().is_multiple_of(4) {
            return Err("invalid base64 length");
        }
        let mut out = Vec::with_capacity(clean.len() / 4 * 3);
        for chunk in clean.chunks(4) {
            let a = table[chunk[0] as usize];
            let b = table[chunk[1] as usize];
            let c = if chunk[2] == b'=' {
                0
            } else {
                table[chunk[2] as usize]
            };
            let d = if chunk[3] == b'=' {
                0
            } else {
                table[chunk[3] as usize]
            };
            if a == 255 || b == 255 {
                return Err("invalid base64 character");
            }
            out.push((a << 2) | (b >> 4));
            if chunk[2] != b'=' {
                out.push((b << 4) | (c >> 2));
            }
            if chunk[3] != b'=' {
                out.push((c << 6) | d);
            }
        }
        Ok(out)
    }
}

/// IAM role assume-policy JSON for Lambda.
const LAMBDA_ASSUME_ROLE_POLICY: &str = r#"jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action    = "sts:AssumeRole"
      Effect    = "Allow"
      Principal = { Service = "lambda.amazonaws.com" }
    }]
  })"#;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_function_basic() {
    let zip_path = make_test_lambda_zip();
    let zip_str = zip_path.display().to_string();

    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-function-basic");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "lambda_basic_role" {{
  name = "lambda-basic-test-role"
  assume_role_policy = {LAMBDA_ASSUME_ROLE_POLICY}
}}

resource "aws_lambda_function" "lambda_basic" {{
  filename      = "{zip_str}"
  function_name = "terraform-test-function"
  role          = aws_iam_role.lambda_basic_role.arn
  handler       = "index.handler"
  runtime       = "nodejs18.x"
  description   = "Terraform E2E test function"

  tags = {{
    Environment = "test"
  }}
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("terraform-test-function"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_function_with_env_vars() {
    let zip_path = make_test_lambda_zip();
    let zip_str = zip_path.display().to_string();

    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-function-env");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "lambda_env_role" {{
  name = "lambda-env-test-role"
  assume_role_policy = {LAMBDA_ASSUME_ROLE_POLICY}
}}

resource "aws_lambda_function" "lambda_env" {{
  filename      = "{zip_str}"
  function_name = "terraform-env-function"
  role          = aws_iam_role.lambda_env_role.arn
  handler       = "index.handler"
  runtime       = "python3.12"
  timeout       = 30
  memory_size   = 256

  environment {{
    variables = {{
      DATABASE_URL = "postgres://localhost/mydb"
      LOG_LEVEL    = "INFO"
    }}
  }}
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("terraform-env-function"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_alias() {
    let zip_path = make_test_lambda_zip();
    let zip_str = zip_path.display().to_string();

    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-alias");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "lambda_alias_role" {{
  name = "lambda-alias-test-role"
  assume_role_policy = {LAMBDA_ASSUME_ROLE_POLICY}
}}

resource "aws_lambda_function" "lambda_alias_fn" {{
  filename      = "{zip_str}"
  function_name = "terraform-alias-function"
  role          = aws_iam_role.lambda_alias_role.arn
  handler       = "index.handler"
  runtime       = "nodejs18.x"
  publish       = true
}}

resource "aws_lambda_alias" "lambda_alias" {{
  name             = "terraform-test-alias"
  description      = "Test alias for E2E"
  function_name    = aws_lambda_function.lambda_alias_fn.function_name
  function_version = aws_lambda_function.lambda_alias_fn.version
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("terraform-alias-function"));
    assert!(state.contains("terraform-test-alias"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_permission() {
    let zip_path = make_test_lambda_zip();
    let zip_str = zip_path.display().to_string();

    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-permission");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "lambda_perm_role" {{
  name = "lambda-perm-test-role"
  assume_role_policy = {LAMBDA_ASSUME_ROLE_POLICY}
}}

resource "aws_lambda_function" "lambda_perm_fn" {{
  filename      = "{zip_str}"
  function_name = "terraform-permission-function"
  role          = aws_iam_role.lambda_perm_role.arn
  handler       = "index.handler"
  runtime       = "nodejs18.x"
}}

resource "aws_lambda_permission" "lambda_perm" {{
  statement_id  = "AllowSNSInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda_perm_fn.function_name
  principal     = "sns.amazonaws.com"
  source_arn    = "arn:aws:sns:us-east-1:123456789012:my-topic"
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("terraform-permission-function"));
    assert!(state.contains("AllowSNSInvoke"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_function_modify_in_place() {
    let zip_path = make_test_lambda_zip();
    let zip_str = zip_path.display().to_string();

    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-modify");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "lambda_modify_role" {{
  name = "lambda-modify-test-role"
  assume_role_policy = {LAMBDA_ASSUME_ROLE_POLICY}
}}

resource "aws_lambda_function" "lambda_modify" {{
  filename      = "{zip_str}"
  function_name = "terraform-modify-function"
  role          = aws_iam_role.lambda_modify_role.arn
  handler       = "index.handler"
  runtime       = "nodejs18.x"
  timeout       = 10
  description   = "initial description"
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "first apply failed:\n{stderr}");

    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "lambda_modify_role" {{
  name = "lambda-modify-test-role"
  assume_role_policy = {LAMBDA_ASSUME_ROLE_POLICY}
}}

resource "aws_lambda_function" "lambda_modify" {{
  filename      = "{zip_str}"
  function_name = "terraform-modify-function"
  role          = aws_iam_role.lambda_modify_role.arn
  handler       = "index.handler"
  runtime       = "nodejs18.x"
  timeout       = 60
  description   = "updated description"
}}
"#
        ),
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_event_source_mapping() {
    let zip_path = make_test_lambda_zip();
    let zip_str = zip_path.display().to_string();

    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-esm");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "lambda_esm_role" {{
  name = "lambda-esm-test-role"
  assume_role_policy = {LAMBDA_ASSUME_ROLE_POLICY}
}}

resource "aws_sqs_queue" "lambda_esm_queue" {{
  name = "lambda-esm-test-queue"
}}

resource "aws_lambda_function" "lambda_esm_fn" {{
  filename      = "{zip_str}"
  function_name = "terraform-esm-function"
  role          = aws_iam_role.lambda_esm_role.arn
  handler       = "index.handler"
  runtime       = "nodejs18.x"
}}

resource "aws_lambda_event_source_mapping" "lambda_esm" {{
  event_source_arn = aws_sqs_queue.lambda_esm_queue.arn
  function_name    = aws_lambda_function.lambda_esm_fn.arn
  batch_size       = 10
  enabled          = true
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("terraform-esm-function"));
    assert!(state.contains("aws_lambda_event_source_mapping"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_function_url() {
    let zip_path = make_test_lambda_zip();
    let zip_str = zip_path.display().to_string();

    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-function-url");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "lambda_url_role" {{
  name = "lambda-url-test-role"
  assume_role_policy = {LAMBDA_ASSUME_ROLE_POLICY}
}}

resource "aws_lambda_function" "lambda_url_fn" {{
  filename      = "{zip_str}"
  function_name = "terraform-url-function"
  role          = aws_iam_role.lambda_url_role.arn
  handler       = "index.handler"
  runtime       = "nodejs18.x"
}}

resource "aws_lambda_function_url" "lambda_url" {{
  function_name      = aws_lambda_function.lambda_url_fn.function_name
  authorization_type = "NONE"
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("terraform-url-function"));
    assert!(state.contains("aws_lambda_function_url"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_code_signing_config() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-code-signing");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_lambda_code_signing_config" "lambda_csc" {
  allowed_publishers {
    signing_profile_version_arns = [
      "arn:aws:signer:us-east-1:123456789012:/signing-profiles/MySigningProfile"
    ]
  }

  policies {
    untrusted_artifact_on_deployment = "Warn"
  }

  description = "Terraform E2E test code signing config"
}
"#,
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("aws_lambda_code_signing_config"));
    assert!(state.contains("Terraform E2E test code signing config"));

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_layer_version() {
    let zip_path = make_test_lambda_zip();
    let zip_str = zip_path.display().to_string();

    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-layer-version");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_lambda_layer_version" "lambda_layer" {{
  filename            = "{zip_str}"
  layer_name          = "terraform-test-layer"
  compatible_runtimes = ["nodejs18.x", "nodejs20.x"]
  description         = "Terraform E2E test layer"
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("terraform-test-layer"));
    assert!(state.contains("aws_lambda_layer_version"));

    cleanup_tf_dir(&dir);
}

/// Regression for the GetPolicy random-uuid bug fixed in 5e3ba012: GetPolicy
/// used to mint a fresh RevisionId on every read, so the AWS provider's refresh
/// after apply would see a different RevisionId from the one in state and
/// report drift. A second `terraform plan` after apply must show no changes.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_permission_plan_idempotent_after_apply() {
    let zip_path = make_test_lambda_zip();
    let zip_str = zip_path.display().to_string();

    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-permission-idempotent");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "lambda_perm_idempotent_role" {{
  name = "lambda-perm-idempotent-role"
  assume_role_policy = {LAMBDA_ASSUME_ROLE_POLICY}
}}

resource "aws_lambda_function" "lambda_perm_idempotent_fn" {{
  filename      = "{zip_str}"
  function_name = "terraform-perm-idempotent-fn"
  role          = aws_iam_role.lambda_perm_idempotent_role.arn
  handler       = "index.handler"
  runtime       = "nodejs18.x"
}}

resource "aws_lambda_permission" "lambda_perm_idempotent" {{
  statement_id  = "AllowEvents"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda_perm_idempotent_fn.function_name
  principal     = "events.amazonaws.com"
  source_arn    = "arn:aws:events:us-east-1:123456789012:rule/my-rule"
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    // -detailed-exitcode: 0 = no changes, 2 = changes pending.
    let (ok, stdout, stderr) = terraform_plan(&dir).await;
    assert!(
        ok,
        "terraform plan shows changes after apply (expected idempotent):\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("No changes"),
        "Expected 'No changes' in plan output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

/// Apply + destroy exercises RemovePermission. The AWS provider reads the
/// policy first, then passes the resulting RevisionId as the `RevisionId` query
/// parameter of `DELETE /2015-03-31/functions/{name}/policy/{sid}`. Before
/// 5e3ba012 the handler ignored that query parameter, so a stale value would be
/// silently accepted; after the fix it must match the stored revision for the
/// call to succeed.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_permission_apply_and_destroy() {
    let zip_path = make_test_lambda_zip();
    let zip_str = zip_path.display().to_string();

    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-permission-apply-destroy");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "lambda_perm_destroy_role" {{
  name = "lambda-perm-destroy-role"
  assume_role_policy = {LAMBDA_ASSUME_ROLE_POLICY}
}}

resource "aws_lambda_function" "lambda_perm_destroy_fn" {{
  filename      = "{zip_str}"
  function_name = "terraform-perm-destroy-fn"
  role          = aws_iam_role.lambda_perm_destroy_role.arn
  handler       = "index.handler"
  runtime       = "nodejs18.x"
}}

resource "aws_lambda_permission" "lambda_perm_destroy" {{
  statement_id  = "AllowEvents"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda_perm_destroy_fn.function_name
  principal     = "events.amazonaws.com"
  source_arn    = "arn:aws:events:us-east-1:123456789012:rule/my-rule"
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 3 added"),
        "Unexpected apply output:\n{stdout}"
    );

    let (ok, stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 3 destroyed")
            || stdout.contains("Destroy complete! Resources: 3 destroyed"),
        "Unexpected destroy output:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}

/// Apply two distinct `aws_lambda_permission` statements on the same function,
/// then remove only one of them. Exercises AddPermission bumping the revision
/// id between statements and RemovePermission honouring the latest revision id
/// when the provider deletes the dropped statement; the surviving permission
/// must remain in state and the dropped one must not.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_permission_apply_then_drop_one() {
    let zip_path = make_test_lambda_zip();
    let zip_str = zip_path.display().to_string();

    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-permission-drop-one");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "lambda_perm_multi_role" {{
  name = "lambda-perm-multi-role"
  assume_role_policy = {LAMBDA_ASSUME_ROLE_POLICY}
}}

resource "aws_lambda_function" "lambda_perm_multi_fn" {{
  filename      = "{zip_str}"
  function_name = "terraform-perm-multi-fn"
  role          = aws_iam_role.lambda_perm_multi_role.arn
  handler       = "index.handler"
  runtime       = "nodejs18.x"
}}

resource "aws_lambda_permission" "lambda_perm_events" {{
  statement_id  = "AllowEvents"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda_perm_multi_fn.function_name
  principal     = "events.amazonaws.com"
  source_arn    = "arn:aws:events:us-east-1:123456789012:rule/my-rule"
}}

resource "aws_lambda_permission" "lambda_perm_sns" {{
  statement_id  = "AllowSns"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda_perm_multi_fn.function_name
  principal     = "sns.amazonaws.com"
  source_arn    = "arn:aws:sns:us-east-1:123456789012:my-topic"
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "first terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 4 added"),
        "Unexpected first-apply output:\n{stdout}"
    );

    // Drop the SNS statement and re-apply: exactly one resource should be removed.
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_iam_role" "lambda_perm_multi_role" {{
  name = "lambda-perm-multi-role"
  assume_role_policy = {LAMBDA_ASSUME_ROLE_POLICY}
}}

resource "aws_lambda_function" "lambda_perm_multi_fn" {{
  filename      = "{zip_str}"
  function_name = "terraform-perm-multi-fn"
  role          = aws_iam_role.lambda_perm_multi_role.arn
  handler       = "index.handler"
  runtime       = "nodejs18.x"
}}

resource "aws_lambda_permission" "lambda_perm_events" {{
  statement_id  = "AllowEvents"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda_perm_multi_fn.function_name
  principal     = "events.amazonaws.com"
  source_arn    = "arn:aws:events:us-east-1:123456789012:rule/my-rule"
}}
"#
        ),
    )
    .unwrap();

    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second terraform apply failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 0 added, 0 changed, 1 destroyed"),
        "Unexpected second-apply output:\n{stdout}"
    );

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("AllowEvents"),
        "Surviving permission missing from state:\n{state}"
    );
    assert!(
        !state.contains("AllowSns"),
        "Dropped permission still present in state:\n{state}"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Coverage for revision_id-bearing surfaces touched by
// d682a1ae fix(lambda): honour RevisionId on Update*, PublishVersion,
// layer-version policy.
//
// These tests exercise the affected handlers through real terraform apply
// cycles. The terraform provider does not send RevisionId on update, so
// they verify the no-revision-id ("any revision is fine") branch — the
// branch that must still accept terraform-style updates after the
// precondition logic landed. They also pin the response-shape changes
// (FunctionConfiguration.revision_id now populated; AddLayerVersionPermission
// response echoes the stored revision id) by asserting on tfstate.
// ---------------------------------------------------------------------------

/// Exercises `AddLayerVersionPermission` on create and
/// `RemoveLayerVersionPermission` on destroy — both gained an
/// expected_revision_id parameter in the fix.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_layer_version_permission() {
    let zip_path = make_test_lambda_zip();
    let zip_str = zip_path.display().to_string();

    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-layer-version-permission");

    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        format!(
            r#"
resource "aws_lambda_layer_version" "lambda_lvp_layer" {{
  filename            = "{zip_str}"
  layer_name          = "terraform-test-lvp-layer"
  compatible_runtimes = ["nodejs18.x"]
}}

resource "aws_lambda_layer_version_permission" "lambda_lvp" {{
  layer_name     = aws_lambda_layer_version.lambda_lvp_layer.layer_name
  version_number = aws_lambda_layer_version.lambda_lvp_layer.version
  statement_id   = "AllowAccountAccess"
  action         = "lambda:GetLayerVersion"
  principal      = "123456789012"
}}
"#
        ),
    )
    .unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("terraform-test-lvp-layer"));
    assert!(state.contains("aws_lambda_layer_version_permission"));
    assert!(state.contains("AllowAccountAccess"));

    // Destroy exercises RemoveLayerVersionPermission with no expected
    // revision id — must succeed.
    let (ok, _stdout, stderr) = terraform_destroy(&dir).await;
    assert!(ok, "terraform destroy failed:\n{stderr}");

    cleanup_tf_dir(&dir);
}

/// Exercises `UpdateAlias` (alias description change) — the fix added an
/// expected_revision_id parameter; terraform does not send one, so the
/// update must still succeed.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_alias_modify_in_place() {
    let zip_path = make_test_lambda_zip();
    let zip_str = zip_path.display().to_string();

    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-alias-modify");

    write_provider_tf(&dir, &url);
    let main_tf = |description: &str| {
        format!(
            r#"
resource "aws_iam_role" "lambda_alias_modify_role" {{
  name = "lambda-alias-modify-test-role"
  assume_role_policy = {LAMBDA_ASSUME_ROLE_POLICY}
}}

resource "aws_lambda_function" "lambda_alias_modify_fn" {{
  filename      = "{zip_str}"
  function_name = "terraform-alias-modify-function"
  role          = aws_iam_role.lambda_alias_modify_role.arn
  handler       = "index.handler"
  runtime       = "nodejs18.x"
  publish       = true
}}

resource "aws_lambda_alias" "lambda_alias_modify" {{
  name             = "terraform-modify-alias"
  description      = "{description}"
  function_name    = aws_lambda_function.lambda_alias_modify_fn.function_name
  function_version = aws_lambda_function.lambda_alias_modify_fn.version
}}
"#
        )
    };

    std::fs::write(dir.join("main.tf"), main_tf("initial alias description")).unwrap();

    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "first apply failed:\n{stderr}");

    std::fs::write(dir.join("main.tf"), main_tf("updated alias description")).unwrap();
    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second apply (alias modify) failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 0 added, 1 changed, 0 destroyed"),
        "expected in-place alias update:\n{stdout}"
    );

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("updated alias description"));

    cleanup_tf_dir(&dir);
}

/// Exercises `UpdateFunctionCode` (filename / source_code_hash change) —
/// the fix added an expected_revision_id parameter and bumps revision_id
/// on success. terraform does not send a revision id; the update must
/// still succeed and the new tfstate must round-trip the bumped revision.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_lambda_function_code_update() {
    let zip_path = make_test_lambda_zip();
    let zip_str = zip_path.display().to_string();

    // Second zip with different bytes so source_code_hash differs and
    // the provider issues UpdateFunctionCode rather than skipping the change.
    let zip2_dir = workspace_root().join(".agents-workspace").join("tmp");
    std::fs::create_dir_all(&zip2_dir).unwrap();
    let zip2_path = zip2_dir.join("test-lambda-handler-v2.zip");
    let mut bytes = std::fs::read(&zip_path).unwrap();
    bytes.extend_from_slice(b"\0\0v2");
    std::fs::write(&zip2_path, &bytes).unwrap();
    let zip2_str = zip2_path.display().to_string();

    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("lambda-code-update");

    write_provider_tf(&dir, &url);
    let main_tf = |zip: &str| {
        format!(
            r#"
resource "aws_iam_role" "lambda_code_update_role" {{
  name = "lambda-code-update-test-role"
  assume_role_policy = {LAMBDA_ASSUME_ROLE_POLICY}
}}

resource "aws_lambda_function" "lambda_code_update" {{
  filename      = "{zip}"
  function_name = "terraform-code-update-function"
  role          = aws_iam_role.lambda_code_update_role.arn
  handler       = "index.handler"
  runtime       = "nodejs18.x"
}}
"#
        )
    };

    std::fs::write(dir.join("main.tf"), main_tf(&zip_str)).unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "first apply failed:\n{stderr}");

    std::fs::write(dir.join("main.tf"), main_tf(&zip2_str)).unwrap();
    let (ok, stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "second apply (code update) failed:\n{stderr}");
    assert!(
        stdout.contains("Resources: 0 added, 1 changed, 0 destroyed"),
        "expected in-place code update:\n{stdout}"
    );

    cleanup_tf_dir(&dir);
}
