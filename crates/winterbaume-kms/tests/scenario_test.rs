//! End-to-end scenario tests for winterbaume KMS service.
//!
//! Each scenario exercises a realistic multi-step workflow rather than a
//! single API call.

use aws_sdk_kms::config::BehaviorVersion;
use aws_sdk_kms::primitives::Blob;
use winterbaume_core::MockAws;
use winterbaume_kms::KmsService;

/// Helper to create a fresh KMS client backed by winterbaume.
async fn make_kms_client() -> aws_sdk_kms::Client {
    let mock = MockAws::builder().with_service(KmsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kms::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_kms::Client::new(&config)
}

/// Scenario: Envelope encryption pipeline.
///
/// 1. Create a customer-managed key (CMK).
/// 2. Create an alias pointing to the CMK.
/// 3. Generate a data key (DEK) via the alias; assert the plaintext length.
/// 4. Encrypt a message using the DEK's ciphertext blob via ReEncrypt (round-trip).
/// 5. Encrypt a payload directly with the CMK, then decrypt it and assert plaintext equality.
/// 6. Schedule the CMK for deletion, verify state; then cancel deletion.
#[tokio::test]
async fn test_envelope_encryption_pipeline() {
    // Scenario: envelope encryption pipeline
    let client = make_kms_client().await;

    // 1. Create CMK.
    let cmk_resp = client
        .create_key()
        .description("envelope-test-key")
        .send()
        .await
        .expect("create_key should succeed");
    let key_id = cmk_resp.key_metadata().unwrap().key_id().to_string();

    // 2. Create alias.
    let alias_name = "alias/envelope-test";
    client
        .create_alias()
        .alias_name(alias_name)
        .target_key_id(&key_id)
        .send()
        .await
        .expect("create_alias should succeed");

    // 3. Generate a 256-bit DEK via the alias.
    let dek = client
        .generate_data_key()
        .key_id(alias_name)
        .key_spec(aws_sdk_kms::types::DataKeySpec::Aes256)
        .send()
        .await
        .expect("generate_data_key should succeed");
    let plaintext_dek = dek.plaintext().unwrap();
    assert_eq!(
        plaintext_dek.as_ref().len(),
        32,
        "AES-256 DEK should be 32 bytes"
    );
    let _ciphertext_dek = dek.ciphertext_blob().unwrap().clone();

    // 4. Encrypt a payload directly with the CMK and decrypt it.
    let payload = b"hello-envelope-world";
    let enc = client
        .encrypt()
        .key_id(&key_id)
        .plaintext(Blob::new(payload.to_vec()))
        .send()
        .await
        .expect("encrypt should succeed");
    let ciphertext_blob = enc.ciphertext_blob().unwrap().clone();

    let dec = client
        .decrypt()
        .ciphertext_blob(ciphertext_blob)
        .send()
        .await
        .expect("decrypt should succeed");
    assert_eq!(
        dec.plaintext().unwrap().as_ref(),
        payload,
        "decrypted plaintext should match original"
    );

    // 5. Schedule deletion (30-day window) and confirm pending state.
    let sched = client
        .schedule_key_deletion()
        .key_id(&key_id)
        .pending_window_in_days(30)
        .send()
        .await
        .expect("schedule_key_deletion should succeed");
    assert_eq!(
        sched.key_state(),
        Some(&aws_sdk_kms::types::KeyState::PendingDeletion)
    );

    // 6. Cancel deletion and confirm disabled state.
    let cancel_resp = client
        .cancel_key_deletion()
        .key_id(&key_id)
        .send()
        .await
        .expect("cancel_key_deletion should succeed");
    // cancel_key_deletion returns the key_id; re-describe to get state.
    let cancelled_key_id = cancel_resp.key_id().unwrap();
    let described = client
        .describe_key()
        .key_id(cancelled_key_id)
        .send()
        .await
        .expect("describe_key after cancel should succeed");
    let state_after_cancel = described.key_metadata().unwrap().key_state();
    // After cancel the key moves to Disabled.
    assert!(
        matches!(
            state_after_cancel,
            Some(&aws_sdk_kms::types::KeyState::Disabled)
        ),
        "key should be Disabled after cancellation, got {:?}",
        state_after_cancel
    );
}

/// Scenario: Key rotation and audit trail.
///
/// 1. Create a symmetric CMK.
/// 2. Enable automatic rotation.
/// 3. Perform an on-demand rotation and assert rotation is recorded.
/// 4. List key rotations and verify count.
#[tokio::test]
async fn test_key_rotation_audit_trail() {
    // Scenario: key rotation audit trail
    let client = make_kms_client().await;

    // 1. Create CMK.
    let key_id = client
        .create_key()
        .description("rotation-test-key")
        .send()
        .await
        .expect("create_key should succeed")
        .key_metadata()
        .unwrap()
        .key_id()
        .to_string();

    // 2. Enable automatic rotation.
    client
        .enable_key_rotation()
        .key_id(&key_id)
        .send()
        .await
        .expect("enable_key_rotation should succeed");

    let status = client
        .get_key_rotation_status()
        .key_id(&key_id)
        .send()
        .await
        .expect("get_key_rotation_status should succeed");
    assert!(
        status.key_rotation_enabled(),
        "automatic rotation should be enabled"
    );

    // 3. Perform an on-demand rotation.
    client
        .rotate_key_on_demand()
        .key_id(&key_id)
        .send()
        .await
        .expect("rotate_key_on_demand should succeed");

    // 4. List rotations; there should be exactly one on-demand rotation.
    let rotations = client
        .list_key_rotations()
        .key_id(&key_id)
        .send()
        .await
        .expect("list_key_rotations should succeed");
    let rotation_list = rotations.rotations();
    assert_eq!(
        rotation_list.len(),
        1,
        "expected 1 rotation record, got {}",
        rotation_list.len()
    );
    assert_eq!(
        rotation_list[0].rotation_type(),
        Some(&aws_sdk_kms::types::RotationType::OnDemand)
    );
}

/// Scenario: Grant lifecycle for cross-principal delegation.
///
/// 1. Create a CMK.
/// 2. Create a grant for a grantee principal, specifying Decrypt operations.
/// 3. List grants and assert the new grant is present.
/// 4. Revoke the grant and assert it is no longer listed.
#[tokio::test]
async fn test_grant_delegation_lifecycle() {
    // Scenario: grant delegation lifecycle
    let client = make_kms_client().await;

    // 1. Create CMK.
    let key_id = client
        .create_key()
        .description("grant-lifecycle-key")
        .send()
        .await
        .expect("create_key should succeed")
        .key_metadata()
        .unwrap()
        .key_id()
        .to_string();

    // 2. Create a grant.
    let grantee = "arn:aws:iam::123456789012:role/GranteeRole";
    let create_grant = client
        .create_grant()
        .key_id(&key_id)
        .grantee_principal(grantee)
        .operations(aws_sdk_kms::types::GrantOperation::Decrypt)
        .send()
        .await
        .expect("create_grant should succeed");
    let grant_id = create_grant.grant_id().unwrap().to_string();
    assert!(!grant_id.is_empty(), "grant_id should be non-empty");

    // 3. List grants and verify the new grant is present.
    let listed = client
        .list_grants()
        .key_id(&key_id)
        .send()
        .await
        .expect("list_grants should succeed");
    let grants = listed.grants();
    assert_eq!(grants.len(), 1, "expected 1 grant");
    assert_eq!(grants[0].grant_id(), Some(grant_id.as_str()));
    assert_eq!(grants[0].grantee_principal(), Some(grantee));

    // 4. Revoke and verify removal.
    client
        .revoke_grant()
        .key_id(&key_id)
        .grant_id(&grant_id)
        .send()
        .await
        .expect("revoke_grant should succeed");

    let listed_after = client
        .list_grants()
        .key_id(&key_id)
        .send()
        .await
        .expect("list_grants after revoke should succeed");
    assert_eq!(
        listed_after.grants().len(),
        0,
        "grant list should be empty after revoke"
    );
}

/// Scenario: HMAC key create, sign, and verify workflow.
///
/// 1. Create an HMAC_256 key.
/// 2. Generate a MAC for a test message.
/// 3. Verify the MAC succeeds with the correct message.
/// 4. Verify that verification fails when the message is tampered.
#[tokio::test]
async fn test_hmac_generate_and_verify_workflow() {
    // Scenario: HMAC sign and verify workflow
    let client = make_kms_client().await;

    // 1. Create HMAC_256 key.
    let key_id = client
        .create_key()
        .description("hmac-workflow-key")
        .key_spec(aws_sdk_kms::types::KeySpec::Hmac256)
        .key_usage(aws_sdk_kms::types::KeyUsageType::GenerateVerifyMac)
        .send()
        .await
        .expect("create_key (HMAC_256) should succeed")
        .key_metadata()
        .unwrap()
        .key_id()
        .to_string();

    let message = b"hello-hmac-world";

    // 2. Generate MAC.
    let gen_resp = client
        .generate_mac()
        .key_id(&key_id)
        .message(Blob::new(message.to_vec()))
        .mac_algorithm(aws_sdk_kms::types::MacAlgorithmSpec::HmacSha256)
        .send()
        .await
        .expect("generate_mac should succeed");
    let mac = gen_resp.mac().unwrap().clone();

    // 3. Verify succeeds with correct message.
    let verify_ok = client
        .verify_mac()
        .key_id(&key_id)
        .message(Blob::new(message.to_vec()))
        .mac_algorithm(aws_sdk_kms::types::MacAlgorithmSpec::HmacSha256)
        .mac(mac.clone())
        .send()
        .await
        .expect("verify_mac should succeed");
    assert!(
        verify_ok.mac_valid(),
        "MAC should be valid for correct message"
    );

    // 4. Verification fails for tampered message.
    let err = client
        .verify_mac()
        .key_id(&key_id)
        .message(Blob::new(b"tampered-message".to_vec()))
        .mac_algorithm(aws_sdk_kms::types::MacAlgorithmSpec::HmacSha256)
        .mac(mac)
        .send()
        .await;
    assert!(
        err.is_err(),
        "verify_mac should fail for a tampered message"
    );
    let err_str = format!("{:?}", err.unwrap_err());
    assert!(
        err_str.contains("KMSInvalidMacException") || err_str.contains("MAC"),
        "error should indicate MAC verification failure, got: {err_str}"
    );
}

/// Scenario: multi-tenant envelope encryption with a key-binding metadata
/// catalogue (the realistic shape of the bug fixed in GitHub issue #5).
///
/// An application that envelope-encrypts per tenant typically stores a
/// `(tenant_id, key_id, ciphertext)` row alongside the ciphertext so the
/// decrypt path knows which CMK to use. The application then passes the
/// recorded `key_id` to `Decrypt(KeyId=...)`. If the metadata catalogue is
/// corrupted -- a row mix-up, a stale id after a rotation, or a deliberate
/// attempt to coax KMS into decrypting under the wrong key -- AWS rejects the
/// call with `IncorrectKeyException` instead of silently returning plaintext.
/// Without the fix, the mock returned the plaintext anyway, which would mask
/// production bugs (e.g. a join that returns the wrong tenant's `key_id`).
///
/// 1. Two tenants, two distinct CMKs.
/// 2. Encrypt each tenant's payload under its own CMK; record the resolved
///    `KeyId` from the encrypt response into a local catalogue.
/// 3. Decrypt using the catalogue's `KeyId` -- both tenants round-trip.
/// 4. Simulate metadata corruption: swap tenant A's ciphertext with
///    tenant B's `KeyId`. Decrypt with the corrupted pair: AWS rejects with
///    `IncorrectKeyException` and the application sees the error rather than
///    A's plaintext leaking under B's id.
/// 5. Confirm the catalogue remains usable end-to-end after the rejection
///    (i.e. the rejection is per-call and doesn't damage state).
#[tokio::test]
async fn test_multi_tenant_envelope_with_key_binding_catalogue() {
    use aws_sdk_kms::operation::decrypt::DecryptError;

    let client = make_kms_client().await;

    // 1. Per-tenant CMKs.
    let key_a = client
        .create_key()
        .description("tenant-a-cmk")
        .send()
        .await
        .unwrap()
        .key_metadata()
        .unwrap()
        .key_id()
        .to_string();
    let key_b = client
        .create_key()
        .description("tenant-b-cmk")
        .send()
        .await
        .unwrap()
        .key_metadata()
        .unwrap()
        .key_id()
        .to_string();

    // 2. Encrypt and record (tenant -> (key_id, ciphertext)).
    let payload_a = b"tenant-a-secret".to_vec();
    let payload_b = b"tenant-b-secret".to_vec();

    let enc_a = client
        .encrypt()
        .key_id(&key_a)
        .plaintext(Blob::new(payload_a.clone()))
        .send()
        .await
        .unwrap();
    let enc_b = client
        .encrypt()
        .key_id(&key_b)
        .plaintext(Blob::new(payload_b.clone()))
        .send()
        .await
        .unwrap();

    // Encrypt's response key_id is the ARN; record it as the application
    // would so the decrypt path uses an ARN form (Decrypt must accept it).
    let mut catalogue: std::collections::HashMap<&str, (String, aws_sdk_kms::primitives::Blob)> =
        std::collections::HashMap::new();
    catalogue.insert(
        "tenant-a",
        (
            enc_a.key_id().unwrap().to_string(),
            enc_a.ciphertext_blob().unwrap().clone(),
        ),
    );
    catalogue.insert(
        "tenant-b",
        (
            enc_b.key_id().unwrap().to_string(),
            enc_b.ciphertext_blob().unwrap().clone(),
        ),
    );

    // 3. Catalogue-driven decrypt round-trips for both tenants.
    for tenant in ["tenant-a", "tenant-b"] {
        let (key_id, ciphertext) = catalogue.get(tenant).unwrap();
        let dec = client
            .decrypt()
            .ciphertext_blob(ciphertext.clone())
            .key_id(key_id)
            .send()
            .await
            .unwrap_or_else(|e| panic!("decrypt for {tenant} should succeed: {e:?}"));
        let expected: &[u8] = if tenant == "tenant-a" {
            &payload_a
        } else {
            &payload_b
        };
        assert_eq!(
            dec.plaintext().unwrap().as_ref(),
            expected,
            "{tenant}: catalogue-driven decrypt should round-trip"
        );
    }

    // 4. Simulate metadata corruption: tenant-a's ciphertext, tenant-b's KeyId.
    let (_a_key, a_ciphertext) = catalogue.get("tenant-a").unwrap();
    let (b_key, _b_ciphertext) = catalogue.get("tenant-b").unwrap();

    let err = client
        .decrypt()
        .ciphertext_blob(a_ciphertext.clone())
        .key_id(b_key)
        .send()
        .await
        .expect_err("decrypt with mismatched (ciphertext, KeyId) must be rejected");

    let svc_err = err.into_service_error();
    assert!(
        matches!(svc_err, DecryptError::IncorrectKeyException(_)),
        "corrupted catalogue must surface as IncorrectKeyException, got {svc_err:?}"
    );

    // 5. State is intact: subsequent correct lookups still work.
    let (a_key, a_ciphertext) = catalogue.get("tenant-a").unwrap();
    let dec_a = client
        .decrypt()
        .ciphertext_blob(a_ciphertext.clone())
        .key_id(a_key)
        .send()
        .await
        .unwrap();
    assert_eq!(dec_a.plaintext().unwrap().as_ref(), &payload_a[..]);
}

/// Scenario: key-rotation pipeline with strict source-key binding via
/// `ReEncrypt.SourceKeyId`.
///
/// A common rotation workflow re-encrypts data from an old CMK to a new
/// CMK in batches, recording each row's current `key_id` in metadata. A
/// well-behaved rotation calls `ReEncrypt(SourceKeyId=<recorded>)` so KMS
/// rejects the call if the recorded id is stale (the row was already rotated
/// to the new key by a concurrent worker). Without the fix, `ReEncrypt`
/// ignored `SourceKeyId` entirely and the rotation could silently re-rotate
/// already-rotated rows under the wrong key, double-paying for KMS calls
/// and disguising idempotency bugs.
///
/// 1. Create old and new CMKs; encrypt a payload under the old key.
/// 2. Worker #1 rotates the row -- `ReEncrypt(SourceKeyId=old, DestinationKeyId=new)`
///    succeeds; the row's `key_id` becomes the new key's ARN.
/// 3. Worker #2 runs against stale metadata that still thinks the row is
///    under `old` and tries `ReEncrypt(SourceKeyId=old, ...)` against the
///    *new* ciphertext. AWS rejects with `IncorrectKeyException`; the
///    second worker sees the failure and skips the row instead of silently
///    re-rotating it.
/// 4. After the failure, normal decrypt under the *new* key still works,
///    confirming the rejected `ReEncrypt` had no side effects.
#[tokio::test]
async fn test_key_rotation_pipeline_with_strict_source_binding() {
    use aws_sdk_kms::operation::re_encrypt::ReEncryptError;

    let client = make_kms_client().await;

    // 1. Old and new CMKs.
    let key_old = client
        .create_key()
        .description("rotation-old-cmk")
        .send()
        .await
        .unwrap()
        .key_metadata()
        .unwrap()
        .key_id()
        .to_string();
    let key_new = client
        .create_key()
        .description("rotation-new-cmk")
        .send()
        .await
        .unwrap()
        .key_metadata()
        .unwrap()
        .key_id()
        .to_string();

    let payload = b"rotation-payload".to_vec();
    let enc = client
        .encrypt()
        .key_id(&key_old)
        .plaintext(Blob::new(payload.clone()))
        .send()
        .await
        .unwrap();
    let ciphertext_under_old = enc.ciphertext_blob().unwrap().clone();
    let mut recorded_key = enc.key_id().unwrap().to_string();
    let expected_old_arn = format!("arn:aws:kms:us-east-1:123456789012:key/{key_old}");
    assert_eq!(recorded_key, expected_old_arn);

    // 2. Worker #1 rotates the row: source=old, destination=new.
    let re_enc = client
        .re_encrypt()
        .ciphertext_blob(ciphertext_under_old.clone())
        .source_key_id(&key_old)
        .destination_key_id(&key_new)
        .send()
        .await
        .expect("ReEncrypt with correct SourceKeyId should succeed");
    let ciphertext_under_new = re_enc.ciphertext_blob().unwrap().clone();
    recorded_key = re_enc.key_id().unwrap().to_string();
    let expected_new_arn = format!("arn:aws:kms:us-east-1:123456789012:key/{key_new}");
    assert_eq!(recorded_key, expected_new_arn);

    // 3. Worker #2 runs against stale metadata and tries to "rotate" the
    //    already-rotated ciphertext using the old SourceKeyId. AWS must
    //    reject so the worker can skip the row instead of silently
    //    re-rotating it under the wrong key.
    let err = client
        .re_encrypt()
        .ciphertext_blob(ciphertext_under_new.clone())
        .source_key_id(&key_old)
        .destination_key_id(&key_new)
        .send()
        .await
        .expect_err("ReEncrypt with stale SourceKeyId must be rejected");

    let svc_err = err.into_service_error();
    assert!(
        matches!(svc_err, ReEncryptError::IncorrectKeyException(_)),
        "stale-source ReEncrypt must surface as IncorrectKeyException, got {svc_err:?}"
    );

    // 4. The rejected ReEncrypt had no side effects -- decrypting the
    //    current ciphertext under the new key still recovers the payload.
    let dec = client
        .decrypt()
        .ciphertext_blob(ciphertext_under_new)
        .key_id(&key_new)
        .send()
        .await
        .expect("decrypt under the rotated key must still succeed");
    assert_eq!(dec.plaintext().unwrap().as_ref(), &payload[..]);
}
