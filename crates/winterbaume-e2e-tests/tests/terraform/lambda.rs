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
