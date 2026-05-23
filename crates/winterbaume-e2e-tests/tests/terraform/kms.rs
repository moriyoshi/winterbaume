use crate::harness::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_kms_key_basic() {
    let result = batch_apply(
        r#"
resource "aws_kms_key" "kms_key_basic" {
  description = "kms-key-basic"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("kms-key-basic"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_kms_key_with_policy_and_tags() {
    let result = batch_apply(
        r#"
resource "aws_kms_key" "kms_key_full" {
  description = "kms-key-full"

  tags = {
    Environment = "test"
    Purpose     = "e2e"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("kms-key-full"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_kms_alias() {
    let result = batch_apply(
        r#"
resource "aws_kms_key" "kms_alias" {
  description = "kms-alias-key"
}

resource "aws_kms_alias" "kms_alias" {
  name          = "alias/terraform-test"
  target_key_id = aws_kms_key.kms_alias.key_id
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("alias/terraform-test"));
}

// Regression coverage for GitHub issue #5 (IncorrectKeyException on Decrypt
// when the request-side KeyId does not match the ciphertext's key).
//
// Terraform reaches this code path via `data "aws_kms_secrets"`, which
// accepts a per-secret `key_id` and forwards it to the KMS Decrypt API
// (terraform-provider-aws internal/service/kms/secrets_data_source.go).
// Encryption happens at apply time via `data "aws_kms_ciphertext"`.
//
// Both tests run in isolated terraform directories rather than the shared
// batch because (a) the wrong-key test expects `terraform apply` to fail,
// which would poison sibling tests in the batched wave, and (b) the happy
// path is the symmetric control case for the failure assertion.

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_kms_decrypt_with_wrong_key_id_fails() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("kms-issue5-wrong-key-id");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_kms_key" "kms_issue5_wrong_a" {
  description = "kms-issue5-wrong-key-a"
}

resource "aws_kms_key" "kms_issue5_wrong_b" {
  description = "kms-issue5-wrong-key-b"
}

# Encrypt at apply time under key A.
data "aws_kms_ciphertext" "kms_issue5_wrong_under_a" {
  key_id    = aws_kms_key.kms_issue5_wrong_a.key_id
  plaintext = "issue5-wrong-key-payload"
}

# Try to decrypt at apply time using key B's id — must fail with
# IncorrectKeyException; without the fix this returned plaintext silently.
data "aws_kms_secrets" "kms_issue5_wrong_secrets" {
  secret {
    name    = "issue5"
    payload = data.aws_kms_ciphertext.kms_issue5_wrong_under_a.ciphertext_blob
    key_id  = aws_kms_key.kms_issue5_wrong_b.key_id
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(
        !ok,
        "terraform apply unexpectedly succeeded — Decrypt should have been rejected:\n{stderr}"
    );
    assert!(
        stderr.contains("IncorrectKeyException"),
        "expected IncorrectKeyException in error output, got:\n{stderr}"
    );
    assert!(
        stderr.contains("issue5"),
        "expected the failing secret's name (issue5) in the provider's wrapping error, got:\n{stderr}"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_kms_decrypt_with_matching_key_id_succeeds() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("kms-issue5-matching-key-id");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_kms_key" "kms_issue5_ok" {
  description = "kms-issue5-matching-key"
}

data "aws_kms_ciphertext" "kms_issue5_ok_ct" {
  key_id    = aws_kms_key.kms_issue5_ok.key_id
  plaintext = "issue5-ok-payload"
}

# Decrypt using the same key id — must succeed and round-trip the plaintext.
# Guards against the fix accidentally rejecting matching ids.
data "aws_kms_secrets" "kms_issue5_ok_secrets" {
  secret {
    name    = "issue5_ok"
    payload = data.aws_kms_ciphertext.kms_issue5_ok_ct.ciphertext_blob
    key_id  = aws_kms_key.kms_issue5_ok.key_id
  }
}

# Surface the decrypted plaintext through state so the test can assert it.
output "kms_issue5_ok_plaintext" {
  value     = data.aws_kms_secrets.kms_issue5_ok_secrets.plaintext["issue5_ok"]
  sensitive = true
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("issue5-ok-payload"),
        "decrypted plaintext should appear in state, got:\n{state}"
    );
    cleanup_tf_dir(&dir);
}
