use aws_sdk_secretsmanager::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_secretsmanager::SecretsManagerService;

async fn make_sm_client() -> aws_sdk_secretsmanager::Client {
    let mock = MockAws::builder()
        .with_service(SecretsManagerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_secretsmanager::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_secretsmanager::Client::new(&config)
}

// =============================================================================
// CreateSecret tests (from moto test_create_secret, test_secret_arn, etc.)
// =============================================================================

#[tokio::test]
async fn test_create_secret() {
    let client = make_sm_client().await;

    let result = client
        .create_secret()
        .name("test-secret")
        .secret_string("foosecret")
        .send()
        .await
        .expect("create_secret should succeed");

    assert!(result.arn().is_some());
    assert_eq!(result.name(), Some("test-secret"));
    assert!(result.version_id().is_some());

    let secret = client
        .get_secret_value()
        .secret_id("test-secret")
        .send()
        .await
        .unwrap();
    assert_eq!(secret.secret_string(), Some("foosecret"));
}

#[tokio::test]
async fn test_secret_arn() {
    let client = make_sm_client().await;

    let result = client
        .create_secret()
        .name("test-secret-arn")
        .secret_string("secret_string")
        .send()
        .await
        .unwrap();

    let arn = result.arn().expect("should have ARN");
    // ARN should match the pattern arn:aws:secretsmanager:us-east-1:123456789012:secret:test-secret-arn-XXXXXX
    assert!(
        arn.starts_with("arn:aws:secretsmanager:us-east-1:123456789012:secret:test-secret-arn-")
    );
}

#[tokio::test]
async fn test_create_secret_with_tags() {
    let client = make_sm_client().await;

    let result = client
        .create_secret()
        .name("test-secret-with-tags")
        .secret_string("foosecret")
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key("Foo")
                .value("Bar")
                .build(),
        )
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key("Mykey")
                .value("Myvalue")
                .build(),
        )
        .send()
        .await
        .unwrap();

    assert!(result.arn().is_some());
    assert_eq!(result.name(), Some("test-secret-with-tags"));

    let secret_value = client
        .get_secret_value()
        .secret_id("test-secret-with-tags")
        .send()
        .await
        .unwrap();
    assert_eq!(secret_value.secret_string(), Some("foosecret"));

    let details = client
        .describe_secret()
        .secret_id("test-secret-with-tags")
        .send()
        .await
        .unwrap();

    let tags = details.tags();
    assert_eq!(tags.len(), 2);
    // Tags may be in any order, so check both
    let tag_map: std::collections::HashMap<&str, &str> = tags
        .iter()
        .map(|t| (t.key().unwrap_or(""), t.value().unwrap_or("")))
        .collect();
    assert_eq!(tag_map.get("Foo"), Some(&"Bar"));
    assert_eq!(tag_map.get("Mykey"), Some(&"Myvalue"));
}

#[tokio::test]
async fn test_create_secret_with_description() {
    let client = make_sm_client().await;

    let result = client
        .create_secret()
        .name("test-secret-with-desc")
        .secret_string("foosecret")
        .description("desc")
        .send()
        .await
        .unwrap();

    assert!(result.arn().is_some());
    assert_eq!(result.name(), Some("test-secret-with-desc"));

    let details = client
        .describe_secret()
        .secret_id("test-secret-with-desc")
        .send()
        .await
        .unwrap();
    assert_eq!(details.description(), Some("desc"));
}

#[tokio::test]
async fn test_create_secret_without_value() {
    let client = make_sm_client().await;

    let create = client
        .create_secret()
        .name("secret-no-value")
        .send()
        .await
        .unwrap();

    // Without a value, there should be no VersionId
    assert!(create.arn().is_some());
    assert_eq!(create.name(), Some("secret-no-value"));
    assert!(create.version_id().is_none());

    // Describe should not have VersionIdsToStages
    let describe = client
        .describe_secret()
        .secret_id("secret-no-value")
        .send()
        .await
        .unwrap();
    assert_eq!(describe.name(), Some("secret-no-value"));
    assert!(
        describe.version_ids_to_stages().is_none()
            || describe.version_ids_to_stages().unwrap().is_empty()
    );

    // Getting value should fail
    let err = client
        .get_secret_value()
        .secret_id("secret-no-value")
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_create_secret_that_has_no_value_and_then_update() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("secret-no-value-then-update")
        .send()
        .await
        .unwrap();

    client
        .update_secret()
        .secret_id("secret-no-value-then-update")
        .secret_string("barsecret")
        .description("desc")
        .send()
        .await
        .unwrap();

    let secret = client
        .get_secret_value()
        .secret_id("secret-no-value-then-update")
        .send()
        .await
        .unwrap();
    assert_eq!(secret.secret_string(), Some("barsecret"));
}

#[tokio::test]
async fn test_create_duplicate_fails() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test/dup")
        .secret_string("val")
        .send()
        .await
        .unwrap();

    let result = client
        .create_secret()
        .name("test/dup")
        .secret_string("val2")
        .send()
        .await;

    assert!(result.is_err());
}

// =============================================================================
// GetSecretValue tests (from moto test_get_secret_value, etc.)
// =============================================================================

#[tokio::test]
async fn test_get_secret_value() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("java-util-test-password")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    let result = client
        .get_secret_value()
        .secret_id("java-util-test-password")
        .send()
        .await
        .unwrap();

    assert_eq!(result.secret_string(), Some("foosecret"));
}

#[tokio::test]
async fn test_get_secret_value_by_arn() {
    let client = make_sm_client().await;

    let create_result = client
        .create_secret()
        .name("get-by-arn")
        .secret_string("test_get_secret_value_by_arn")
        .send()
        .await
        .unwrap();

    let arn = create_result.arn().unwrap();
    assert!(arn.starts_with("arn:aws:secretsmanager:us-east-1:123456789012:secret:get-by-arn"));

    let result = client
        .get_secret_value()
        .secret_id(arn)
        .send()
        .await
        .unwrap();

    assert_eq!(result.secret_string(), Some("test_get_secret_value_by_arn"));
}

#[tokio::test]
async fn test_get_secret_that_does_not_exist() {
    let client = make_sm_client().await;

    let err = client
        .get_secret_value()
        .secret_id("i-dont-exist")
        .send()
        .await;

    assert!(err.is_err());
}

#[tokio::test]
async fn test_get_secret_that_does_not_match() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("java-util-test-password")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    let err = client
        .get_secret_value()
        .secret_id("i-dont-match")
        .send()
        .await;

    assert!(err.is_err());
}

#[tokio::test]
async fn test_get_secret_value_that_is_marked_deleted() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test-secret-del")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    client
        .delete_secret()
        .secret_id("test-secret-del")
        .send()
        .await
        .unwrap();

    let err = client
        .get_secret_value()
        .secret_id("test-secret-del")
        .send()
        .await;

    assert!(err.is_err());
}

#[tokio::test]
async fn test_get_secret_that_has_no_value() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("secret-no-value-get")
        .send()
        .await
        .unwrap();

    let err = client
        .get_secret_value()
        .secret_id("secret-no-value-get")
        .send()
        .await;

    assert!(err.is_err());
}

// =============================================================================
// PutSecretValue tests (from moto)
// =============================================================================

#[tokio::test]
async fn test_put_secret_value() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test/secret2")
        .secret_string("v1")
        .send()
        .await
        .unwrap();

    client
        .put_secret_value()
        .secret_id("test/secret2")
        .secret_string("v2")
        .send()
        .await
        .expect("put_secret_value should succeed");

    let get_resp = client
        .get_secret_value()
        .secret_id("test/secret2")
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.secret_string(), Some("v2"));
}

// =============================================================================
// DeleteSecret tests (from moto test_delete_secret, etc.)
// =============================================================================

#[tokio::test]
async fn test_delete_secret() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test-secret-delete")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    let deleted = client
        .delete_secret()
        .secret_id("test-secret-delete")
        .send()
        .await
        .unwrap();

    assert!(deleted.arn().is_some());
    assert_eq!(deleted.name(), Some("test-secret-delete"));
    assert!(deleted.deletion_date().is_some());

    let details = client
        .describe_secret()
        .secret_id("test-secret-delete")
        .send()
        .await
        .unwrap();

    assert!(details.arn().is_some());
    assert_eq!(details.name(), Some("test-secret-delete"));
    assert!(details.deleted_date().is_some());
}

#[tokio::test]
async fn test_delete_secret_by_arn() {
    let client = make_sm_client().await;

    let secret = client
        .create_secret()
        .name("test-secret-del-arn")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    let arn = secret.arn().unwrap();

    let deleted = client.delete_secret().secret_id(arn).send().await.unwrap();

    assert_eq!(deleted.arn(), Some(arn));
    assert_eq!(deleted.name(), Some("test-secret-del-arn"));
    assert!(deleted.deletion_date().is_some());
}

#[tokio::test]
async fn test_delete_secret_force() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test-secret-force")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    let result = client
        .delete_secret()
        .secret_id("test-secret-force")
        .force_delete_without_recovery(true)
        .send()
        .await
        .unwrap();

    assert!(result.arn().is_some());
    assert!(result.deletion_date().is_some());
    assert_eq!(result.name(), Some("test-secret-force"));

    let err = client
        .get_secret_value()
        .secret_id("test-secret-force")
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_delete_secret_force_with_arn() {
    let client = make_sm_client().await;

    let create = client
        .create_secret()
        .name("test-secret-force-arn")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    let result = client
        .delete_secret()
        .secret_id(create.arn().unwrap())
        .force_delete_without_recovery(true)
        .send()
        .await
        .unwrap();

    assert!(result.arn().is_some());
    assert!(result.deletion_date().is_some());
    assert_eq!(result.name(), Some("test-secret-force-arn"));

    let err = client
        .get_secret_value()
        .secret_id("test-secret-force-arn")
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_delete_secret_that_does_not_exist() {
    let client = make_sm_client().await;

    let err = client
        .delete_secret()
        .secret_id("i-dont-exist")
        .send()
        .await;

    assert!(err.is_err());
}

#[tokio::test]
async fn test_delete_secret_fails_with_both_force_and_recovery_window() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test-secret-both")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    let err = client
        .delete_secret()
        .secret_id("test-secret-both")
        .recovery_window_in_days(7)
        .force_delete_without_recovery(true)
        .send()
        .await;

    assert!(err.is_err());
}

#[tokio::test]
async fn test_delete_secret_recovery_window_invalid_values() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test-secret-invalid-window")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    for nr in [0, 2, 6, 31, 100] {
        let err = client
            .delete_secret()
            .secret_id("test-secret-invalid-window")
            .recovery_window_in_days(nr)
            .send()
            .await;
        assert!(err.is_err(), "recovery window {nr} should fail");
    }
}

#[tokio::test]
async fn test_delete_secret_that_is_marked_deleted() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test-secret-double-del")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    client
        .delete_secret()
        .secret_id("test-secret-double-del")
        .send()
        .await
        .unwrap();

    let err = client
        .delete_secret()
        .secret_id("test-secret-double-del")
        .send()
        .await;

    assert!(err.is_err());
}

#[tokio::test]
async fn test_force_delete_secret_that_is_marked_deleted() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test-secret-force-del")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    client
        .delete_secret()
        .secret_id("test-secret-force-del")
        .send()
        .await
        .unwrap();

    // Force deleting an already-deleted secret should succeed
    client
        .delete_secret()
        .secret_id("test-secret-force-del")
        .force_delete_without_recovery(true)
        .send()
        .await
        .expect("force delete of already-deleted secret should succeed");
}

// =============================================================================
// RestoreSecret tests (from moto test_restore_secret, etc.)
// =============================================================================

#[tokio::test]
async fn test_restore_secret() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test-secret-restore")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    client
        .delete_secret()
        .secret_id("test-secret-restore")
        .send()
        .await
        .unwrap();

    // Verify deleted
    let described_before = client
        .describe_secret()
        .secret_id("test-secret-restore")
        .send()
        .await
        .unwrap();
    assert!(described_before.deleted_date().is_some());

    let restored = client
        .restore_secret()
        .secret_id("test-secret-restore")
        .send()
        .await
        .expect("restore_secret should succeed");

    assert!(restored.arn().is_some());
    assert_eq!(restored.name(), Some("test-secret-restore"));

    // Verify DeletedDate is gone
    let described_after = client
        .describe_secret()
        .secret_id("test-secret-restore")
        .send()
        .await
        .unwrap();
    assert!(described_after.deleted_date().is_none());
}

#[tokio::test]
async fn test_restore_secret_that_is_not_deleted() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test-secret-restore-not-del")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    // Restoring a non-deleted secret should succeed (no-op per moto/AWS)
    let restored = client
        .restore_secret()
        .secret_id("test-secret-restore-not-del")
        .send()
        .await;

    assert!(restored.is_ok());
    assert_eq!(
        restored.unwrap().name(),
        Some("test-secret-restore-not-del")
    );
}

#[tokio::test]
async fn test_restore_secret_that_does_not_exist() {
    let client = make_sm_client().await;

    let err = client
        .restore_secret()
        .secret_id("i-dont-exist")
        .send()
        .await;

    assert!(err.is_err());
}

// =============================================================================
// DescribeSecret tests (from moto test_describe_secret, etc.)
// =============================================================================

#[tokio::test]
async fn test_describe_secret() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test-secret-desc1")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    client
        .create_secret()
        .name("test-secret-desc2")
        .secret_string("barsecret")
        .send()
        .await
        .unwrap();

    let desc1 = client
        .describe_secret()
        .secret_id("test-secret-desc1")
        .send()
        .await
        .unwrap();

    let desc2 = client
        .describe_secret()
        .secret_id("test-secret-desc2")
        .send()
        .await
        .unwrap();

    assert_eq!(desc1.name(), Some("test-secret-desc1"));
    assert!(desc1.arn().is_some());
    assert!(!desc1.arn().unwrap().is_empty());
    assert_eq!(desc2.name(), Some("test-secret-desc2"));
    assert!(desc2.arn().is_some());
    assert!(!desc2.arn().unwrap().is_empty());
    assert!(desc1.created_date().is_some());
    assert!(desc2.created_date().is_some());
    assert!(desc1.last_changed_date().is_some());
    assert!(desc2.last_changed_date().is_some());
}

#[tokio::test]
async fn test_describe_secret_with_arn() {
    let client = make_sm_client().await;

    let results = client
        .create_secret()
        .name("testsecret")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_secret()
        .secret_id(results.arn().unwrap())
        .send()
        .await
        .unwrap();

    assert_eq!(desc.name(), Some("testsecret"));
    assert_eq!(desc.arn(), results.arn());

    // List secrets should also show the same ARN
    let list = client.list_secrets().send().await.unwrap();
    assert_eq!(list.secret_list()[0].arn(), results.arn());
}

#[tokio::test]
async fn test_describe_secret_that_does_not_exist() {
    let client = make_sm_client().await;

    let err = client
        .describe_secret()
        .secret_id("i-dont-exist")
        .send()
        .await;

    assert!(err.is_err());
}

// =============================================================================
// ListSecrets tests (from moto test_list_secrets, etc.)
// =============================================================================

#[tokio::test]
async fn test_list_secrets_empty() {
    let client = make_sm_client().await;

    let resp = client.list_secrets().send().await.unwrap();
    assert_eq!(resp.secret_list().len(), 0);
}

#[tokio::test]
async fn test_list_secrets() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test-secret-list1")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    client
        .create_secret()
        .name("test-secret-list2")
        .secret_string("barsecret")
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key("a")
                .value("1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let secrets = client.list_secrets().send().await.unwrap();
    let secret_list = secrets.secret_list();
    assert_eq!(secret_list.len(), 2);

    // Find each secret by name
    let s1 = secret_list
        .iter()
        .find(|s| s.name() == Some("test-secret-list1"))
        .unwrap();
    let s2 = secret_list
        .iter()
        .find(|s| s.name() == Some("test-secret-list2"))
        .unwrap();

    assert!(s1.arn().is_some());
    assert!(s1.secret_versions_to_stages().is_some());
    assert!(s2.arn().is_some());
    assert!(s2.secret_versions_to_stages().is_some());

    // Second secret should have tags
    let tags = s2.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("a"));
    assert_eq!(tags[0].value(), Some("1"));

    assert!(s1.created_date().is_some());
    assert!(s2.created_date().is_some());
    assert!(s1.last_changed_date().is_some());
    assert!(s2.last_changed_date().is_some());
}

#[tokio::test]
async fn test_list_secrets_excludes_deleted() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("foo-exclude")
        .secret_string("secret")
        .send()
        .await
        .unwrap();

    client
        .create_secret()
        .name("bar-exclude")
        .secret_string("secret")
        .send()
        .await
        .unwrap();

    let secrets = client.list_secrets().send().await.unwrap();
    assert_eq!(secrets.secret_list().len(), 2);

    client
        .delete_secret()
        .secret_id("foo-exclude")
        .send()
        .await
        .unwrap();

    // Deleted secret should be excluded from default list
    let secrets = client.list_secrets().send().await.unwrap();
    assert_eq!(secrets.secret_list().len(), 1);
    assert_eq!(secrets.secret_list()[0].name(), Some("bar-exclude"));
}

// =============================================================================
// UpdateSecret tests (from moto test_update_secret_without_value, etc.)
// =============================================================================

#[tokio::test]
async fn test_update_secret() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test/upd")
        .secret_string("old")
        .send()
        .await
        .unwrap();

    client
        .update_secret()
        .secret_id("test/upd")
        .secret_string("new")
        .send()
        .await
        .expect("update_secret should succeed");

    let get_resp = client
        .get_secret_value()
        .secret_id("test/upd")
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.secret_string(), Some("new"));
}

#[tokio::test]
async fn test_update_secret_description_only() {
    let client = make_sm_client().await;

    let create = client
        .create_secret()
        .name("test-upd-desc")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();
    let version_id = create.version_id().unwrap().to_string();

    let updated = client
        .update_secret()
        .secret_id("test-upd-desc")
        .description("desc")
        .send()
        .await
        .unwrap();

    // When only updating description (no new value), VersionId should NOT be in response
    assert!(updated.version_id().is_none());

    // Secret value should still be the same
    let value = client
        .get_secret_value()
        .secret_id("test-upd-desc")
        .send()
        .await
        .unwrap();
    assert_eq!(value.secret_string(), Some("foosecret"));
    assert_eq!(value.version_id(), Some(version_id.as_str()));
}

// =============================================================================
// Delete + Restore lifecycle
// =============================================================================

#[tokio::test]
async fn test_delete_and_restore_secret() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("test/del-restore")
        .secret_string("val")
        .send()
        .await
        .unwrap();

    client
        .delete_secret()
        .secret_id("test/del-restore")
        .send()
        .await
        .expect("delete_secret should succeed");

    // Getting deleted secret should fail
    let result = client
        .get_secret_value()
        .secret_id("test/del-restore")
        .send()
        .await;
    assert!(result.is_err());

    // Restore
    client
        .restore_secret()
        .secret_id("test/del-restore")
        .send()
        .await
        .expect("restore_secret should succeed");

    // Now get should work
    let get_resp = client
        .get_secret_value()
        .secret_id("test/del-restore")
        .send()
        .await
        .expect("get after restore should succeed");
    assert_eq!(get_resp.secret_string(), Some("val"));
}

// =============================================================================
// BatchGetSecretValue tests
// =============================================================================

#[tokio::test]
async fn test_batch_get_secret_value() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("batch-secret-1")
        .secret_string("val1")
        .send()
        .await
        .unwrap();

    client
        .create_secret()
        .name("batch-secret-2")
        .secret_string("val2")
        .send()
        .await
        .unwrap();

    let result = client
        .batch_get_secret_value()
        .secret_id_list("batch-secret-1")
        .secret_id_list("batch-secret-2")
        .send()
        .await
        .expect("batch_get_secret_value should succeed");

    let values = result.secret_values();
    assert_eq!(values.len(), 2);

    let v1 = values
        .iter()
        .find(|v| v.name() == Some("batch-secret-1"))
        .unwrap();
    let v2 = values
        .iter()
        .find(|v| v.name() == Some("batch-secret-2"))
        .unwrap();
    assert_eq!(v1.secret_string(), Some("val1"));
    assert_eq!(v2.secret_string(), Some("val2"));
}

#[tokio::test]
async fn test_batch_get_secret_value_partial_errors() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("batch-exists")
        .secret_string("exists")
        .send()
        .await
        .unwrap();

    let result = client
        .batch_get_secret_value()
        .secret_id_list("batch-exists")
        .secret_id_list("batch-not-found")
        .send()
        .await
        .expect("batch_get_secret_value should succeed with partial errors");

    let values = result.secret_values();
    assert_eq!(values.len(), 1);
    assert_eq!(values[0].name(), Some("batch-exists"));

    let errors = result.errors();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].secret_id(), Some("batch-not-found"));
}

// =============================================================================
// CancelRotateSecret tests
// =============================================================================

#[tokio::test]
async fn test_cancel_rotate_secret() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("cancel-rotate")
        .secret_string("val")
        .send()
        .await
        .unwrap();

    let result = client
        .cancel_rotate_secret()
        .secret_id("cancel-rotate")
        .send()
        .await
        .expect("cancel_rotate_secret should succeed");

    assert!(result.arn().is_some());
    assert_eq!(result.name(), Some("cancel-rotate"));
}

#[tokio::test]
async fn test_cancel_rotate_secret_not_found() {
    let client = make_sm_client().await;

    let err = client
        .cancel_rotate_secret()
        .secret_id("nonexistent")
        .send()
        .await;

    assert!(err.is_err());
}

// =============================================================================
// GetRandomPassword tests
// =============================================================================

#[tokio::test]
async fn test_get_random_password() {
    let client = make_sm_client().await;

    let result = client
        .get_random_password()
        .send()
        .await
        .expect("get_random_password should succeed");

    let password = result.random_password().unwrap();
    assert_eq!(password.len(), 32); // Default length
}

#[tokio::test]
async fn test_get_random_password_custom_length() {
    let client = make_sm_client().await;

    let result = client
        .get_random_password()
        .password_length(64)
        .send()
        .await
        .expect("get_random_password with custom length should succeed");

    let password = result.random_password().unwrap();
    assert_eq!(password.len(), 64);
}

#[tokio::test]
async fn test_get_random_password_exclude_punctuation() {
    let client = make_sm_client().await;

    let result = client
        .get_random_password()
        .password_length(100)
        .exclude_punctuation(true)
        .send()
        .await
        .unwrap();

    let password = result.random_password().unwrap();
    let punctuation = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    for c in password.chars() {
        assert!(
            !punctuation.contains(c),
            "Password should not contain punctuation: {c}"
        );
    }
}

// =============================================================================
// ResourcePolicy tests (Put, Get, Delete)
// =============================================================================

#[tokio::test]
async fn test_put_and_get_resource_policy() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("policy-secret")
        .secret_string("val")
        .send()
        .await
        .unwrap();

    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;

    let put_result = client
        .put_resource_policy()
        .secret_id("policy-secret")
        .resource_policy(policy)
        .send()
        .await
        .expect("put_resource_policy should succeed");

    assert!(put_result.arn().is_some());
    assert_eq!(put_result.name(), Some("policy-secret"));

    let get_result = client
        .get_resource_policy()
        .secret_id("policy-secret")
        .send()
        .await
        .expect("get_resource_policy should succeed");

    assert_eq!(get_result.resource_policy(), Some(policy));
    assert_eq!(get_result.name(), Some("policy-secret"));
}

#[tokio::test]
async fn test_delete_resource_policy() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("policy-del-secret")
        .secret_string("val")
        .send()
        .await
        .unwrap();

    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;
    client
        .put_resource_policy()
        .secret_id("policy-del-secret")
        .resource_policy(policy)
        .send()
        .await
        .unwrap();

    let del_result = client
        .delete_resource_policy()
        .secret_id("policy-del-secret")
        .send()
        .await
        .expect("delete_resource_policy should succeed");

    assert!(del_result.arn().is_some());
    assert_eq!(del_result.name(), Some("policy-del-secret"));

    // Verify policy is gone
    let get_result = client
        .get_resource_policy()
        .secret_id("policy-del-secret")
        .send()
        .await
        .unwrap();

    assert!(get_result.resource_policy().is_none());
}

#[tokio::test]
async fn test_get_resource_policy_not_found() {
    let client = make_sm_client().await;

    let err = client
        .get_resource_policy()
        .secret_id("nonexistent")
        .send()
        .await;

    assert!(err.is_err());
}

// =============================================================================
// ListSecretVersionIds tests
// =============================================================================

#[tokio::test]
async fn test_list_secret_version_ids() {
    let client = make_sm_client().await;

    let create = client
        .create_secret()
        .name("version-ids-secret")
        .secret_string("v1")
        .send()
        .await
        .unwrap();

    let v1_id = create.version_id().unwrap().to_string();

    // Add another version
    let put = client
        .put_secret_value()
        .secret_id("version-ids-secret")
        .secret_string("v2")
        .send()
        .await
        .unwrap();

    let v2_id = put.version_id().unwrap().to_string();

    let result = client
        .list_secret_version_ids()
        .secret_id("version-ids-secret")
        .send()
        .await
        .expect("list_secret_version_ids should succeed");

    assert_eq!(result.name(), Some("version-ids-secret"));
    assert!(result.arn().is_some());

    let versions = result.versions();
    assert_eq!(versions.len(), 2);

    let version_ids: Vec<&str> = versions.iter().filter_map(|v| v.version_id()).collect();
    assert!(version_ids.contains(&v1_id.as_str()));
    assert!(version_ids.contains(&v2_id.as_str()));
}

#[tokio::test]
async fn test_list_secret_version_ids_not_found() {
    let client = make_sm_client().await;

    let err = client
        .list_secret_version_ids()
        .secret_id("nonexistent")
        .send()
        .await;

    assert!(err.is_err());
}

// =============================================================================
// RotateSecret tests
// =============================================================================

#[tokio::test]
async fn test_rotate_secret() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("rotate-secret")
        .secret_string("original")
        .send()
        .await
        .unwrap();

    let result = client
        .rotate_secret()
        .secret_id("rotate-secret")
        .rotation_lambda_arn("arn:aws:lambda:us-east-1:123456789012:function:MyRotationFunction")
        .rotation_rules(
            aws_sdk_secretsmanager::types::RotationRulesType::builder()
                .automatically_after_days(30)
                .build(),
        )
        .send()
        .await
        .expect("rotate_secret should succeed");

    assert!(result.arn().is_some());
    assert_eq!(result.name(), Some("rotate-secret"));
    assert!(result.version_id().is_some());

    // Describe to verify rotation is enabled
    let desc = client
        .describe_secret()
        .secret_id("rotate-secret")
        .send()
        .await
        .unwrap();

    assert_eq!(desc.rotation_enabled(), Some(true));
    assert!(desc.rotation_lambda_arn().is_some());
    assert!(desc.last_rotated_date().is_some());
}

#[tokio::test]
async fn test_rotate_secret_not_found() {
    let client = make_sm_client().await;

    let err = client.rotate_secret().secret_id("nonexistent").send().await;

    assert!(err.is_err());
}

// =============================================================================
// TagResource / UntagResource tests
// =============================================================================

#[tokio::test]
async fn test_tag_and_untag_resource() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("tag-test-secret")
        .secret_string("val")
        .send()
        .await
        .unwrap();

    // Add tags
    client
        .tag_resource()
        .secret_id("tag-test-secret")
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key("Env")
                .value("Prod")
                .build(),
        )
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key("Team")
                .value("Backend")
                .build(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // Verify tags
    let desc = client
        .describe_secret()
        .secret_id("tag-test-secret")
        .send()
        .await
        .unwrap();

    let tags = desc.tags();
    assert_eq!(tags.len(), 2);
    let tag_map: std::collections::HashMap<&str, &str> = tags
        .iter()
        .map(|t| (t.key().unwrap_or(""), t.value().unwrap_or("")))
        .collect();
    assert_eq!(tag_map.get("Env"), Some(&"Prod"));
    assert_eq!(tag_map.get("Team"), Some(&"Backend"));

    // Remove one tag
    client
        .untag_resource()
        .secret_id("tag-test-secret")
        .tag_keys("Env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify only one tag remains
    let desc2 = client
        .describe_secret()
        .secret_id("tag-test-secret")
        .send()
        .await
        .unwrap();

    let tags2 = desc2.tags();
    assert_eq!(tags2.len(), 1);
    assert_eq!(tags2[0].key(), Some("Team"));
    assert_eq!(tags2[0].value(), Some("Backend"));
}

#[tokio::test]
async fn test_tag_resource_not_found() {
    let client = make_sm_client().await;

    let err = client
        .tag_resource()
        .secret_id("nonexistent")
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key("K")
                .value("V")
                .build(),
        )
        .send()
        .await;

    assert!(err.is_err());
}

#[tokio::test]
async fn test_untag_resource_not_found() {
    let client = make_sm_client().await;

    let err = client
        .untag_resource()
        .secret_id("nonexistent")
        .tag_keys("K")
        .send()
        .await;

    assert!(err.is_err());
}

// =============================================================================
// UpdateSecretVersionStage tests
// =============================================================================

#[tokio::test]
async fn test_update_secret_version_stage() {
    let client = make_sm_client().await;

    let create = client
        .create_secret()
        .name("stage-test")
        .secret_string("v1")
        .send()
        .await
        .unwrap();

    let v1_id = create.version_id().unwrap().to_string();

    // Put a new version
    let put = client
        .put_secret_value()
        .secret_id("stage-test")
        .secret_string("v2")
        .send()
        .await
        .unwrap();

    let v2_id = put.version_id().unwrap().to_string();

    // Move AWSCURRENT back to v1
    let result = client
        .update_secret_version_stage()
        .secret_id("stage-test")
        .version_stage("AWSCURRENT")
        .move_to_version_id(&v1_id)
        .remove_from_version_id(&v2_id)
        .send()
        .await
        .expect("update_secret_version_stage should succeed");

    assert!(result.arn().is_some());
    assert_eq!(result.name(), Some("stage-test"));

    // Verify the current value is now v1
    let value = client
        .get_secret_value()
        .secret_id("stage-test")
        .send()
        .await
        .unwrap();

    assert_eq!(value.secret_string(), Some("v1"));
    assert_eq!(value.version_id(), Some(v1_id.as_str()));
}

// =============================================================================
// ReplicateSecretToRegions / RemoveRegionsFromReplication tests
// =============================================================================

#[tokio::test]
async fn test_replicate_and_remove_regions() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("replicate-test")
        .secret_string("val")
        .send()
        .await
        .unwrap();

    // Replicate to regions
    let result = client
        .replicate_secret_to_regions()
        .secret_id("replicate-test")
        .add_replica_regions(
            aws_sdk_secretsmanager::types::ReplicaRegionType::builder()
                .region("eu-west-1")
                .build(),
        )
        .add_replica_regions(
            aws_sdk_secretsmanager::types::ReplicaRegionType::builder()
                .region("ap-southeast-1")
                .build(),
        )
        .send()
        .await
        .expect("replicate_secret_to_regions should succeed");

    assert!(result.arn().is_some());
    let repl_status = result.replication_status();
    assert_eq!(repl_status.len(), 2);

    // Remove one region
    let result = client
        .remove_regions_from_replication()
        .secret_id("replicate-test")
        .remove_replica_regions("eu-west-1")
        .send()
        .await
        .expect("remove_regions_from_replication should succeed");

    assert!(result.arn().is_some());
    let repl_status = result.replication_status();
    assert_eq!(repl_status.len(), 1);
    assert_eq!(repl_status[0].region(), Some("ap-southeast-1"));
}

// ============================================================================
// Ported from moto: test_secretsmanager.py — RotateSecret
// ============================================================================

// Ported from moto: test_secretsmanager.py::test_rotate_secret_enable_rotation
#[tokio::test]
async fn test_rotate_secret_enable_rotation() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("rotate-enable-test")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    // Before rotation, RotationEnabled should not be set
    let initial = client
        .describe_secret()
        .secret_id("rotate-enable-test")
        .send()
        .await
        .unwrap();
    assert!(initial.rotation_enabled().is_none() || initial.rotation_enabled() == Some(false));

    client
        .rotate_secret()
        .secret_id("rotate-enable-test")
        .rotation_rules(
            aws_sdk_secretsmanager::types::RotationRulesType::builder()
                .automatically_after_days(42)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_secret()
        .secret_id("rotate-enable-test")
        .send()
        .await
        .unwrap();

    assert_eq!(desc.rotation_enabled(), Some(true));
    let rules = desc.rotation_rules().unwrap();
    assert_eq!(rules.automatically_after_days(), Some(42));
}

// Ported from moto: test_secretsmanager.py::test_rotate_secret_that_is_marked_deleted
#[tokio::test]
async fn test_rotate_secret_that_is_marked_deleted() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("rotate-deleted")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    client
        .delete_secret()
        .secret_id("rotate-deleted")
        .send()
        .await
        .unwrap();

    let err = client
        .rotate_secret()
        .secret_id("rotate-deleted")
        .send()
        .await;
    assert!(err.is_err());
}

// Ported from moto: test_secretsmanager.py::test_rotate_secret_without_secretstring
#[tokio::test]
async fn test_rotate_secret_without_secretstring() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("rotate-no-value")
        .description("foodescription")
        .send()
        .await
        .unwrap();

    let rotated = client
        .rotate_secret()
        .secret_id("rotate-no-value")
        .send()
        .await
        .unwrap();

    assert_eq!(rotated.name(), Some("rotate-no-value"));

    let desc = client
        .describe_secret()
        .secret_id("rotate-no-value")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.description(), Some("foodescription"));
}

// Ported from moto: test_secretsmanager.py::test_rotate_secret_rotation_period_validation
#[tokio::test]
async fn test_rotate_secret_rotation_period_too_low() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("rotate-period-low")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    let err = client
        .rotate_secret()
        .secret_id("rotate-period-low")
        .rotation_rules(
            aws_sdk_secretsmanager::types::RotationRulesType::builder()
                .automatically_after_days(0)
                .build(),
        )
        .send()
        .await;
    assert!(err.is_err(), "AutomaticallyAfterDays=0 should fail");
    let err_str = format!("{:?}", err.unwrap_err());
    assert!(
        err_str.contains("InvalidParameter"),
        "Expected InvalidParameterException, got: {err_str}"
    );
}

// Ported from moto: test_secretsmanager.py::test_rotate_secret_rotation_period_too_long
#[tokio::test]
async fn test_rotate_secret_rotation_period_too_high() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("rotate-period-high")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    let err = client
        .rotate_secret()
        .secret_id("rotate-period-high")
        .rotation_rules(
            aws_sdk_secretsmanager::types::RotationRulesType::builder()
                .automatically_after_days(1001)
                .build(),
        )
        .send()
        .await;
    assert!(err.is_err(), "AutomaticallyAfterDays=1001 should fail");
    let err_str = format!("{:?}", err.unwrap_err());
    assert!(
        err_str.contains("InvalidParameter"),
        "Expected InvalidParameterException, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_secretsmanager.py — TagResource / UntagResource
// ============================================================================

// Ported from moto: test_secretsmanager.py::test_tag_resource
// Tests tag overwrite behavior: tagging same key replaces value
#[tokio::test]
async fn test_tag_resource_overwrite() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("tag-overwrite")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    // Initially no tags
    let desc = client
        .describe_secret()
        .secret_id("tag-overwrite")
        .send()
        .await
        .unwrap();
    assert!(desc.tags().is_empty());

    // Add a tag
    client
        .tag_resource()
        .secret_id("tag-overwrite")
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key("FirstTag")
                .value("SomeValue")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Overwrite the tag value
    client
        .tag_resource()
        .secret_id("tag-overwrite")
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key("FirstTag")
                .value("SomeOtherValue")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Add another tag
    client
        .tag_resource()
        .secret_id("tag-overwrite")
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key("SecondTag")
                .value("AnotherValue")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let secrets = client.list_secrets().send().await.unwrap();
    let secret = secrets
        .secret_list()
        .iter()
        .find(|s| s.name() == Some("tag-overwrite"))
        .unwrap();

    let tags = secret.tags();
    let tag_map: std::collections::HashMap<&str, &str> = tags
        .iter()
        .map(|t| (t.key().unwrap_or(""), t.value().unwrap_or("")))
        .collect();
    assert_eq!(tag_map.len(), 2);
    assert_eq!(tag_map.get("FirstTag"), Some(&"SomeOtherValue"));
    assert_eq!(tag_map.get("SecondTag"), Some(&"AnotherValue"));
}

// Ported from moto: test_secretsmanager.py::test_untag_resource
// Tests removing all tags leaves empty tags list
#[tokio::test]
async fn test_untag_resource_all_tags() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("untag-all")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    client
        .tag_resource()
        .secret_id("untag-all")
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key("FirstTag")
                .value("SomeValue")
                .build(),
        )
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key("SecondTag")
                .value("SomeValue")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Remove one tag
    client
        .untag_resource()
        .secret_id("untag-all")
        .tag_keys("FirstTag")
        .send()
        .await
        .unwrap();

    let secrets = client.list_secrets().send().await.unwrap();
    let secret = secrets
        .secret_list()
        .iter()
        .find(|s| s.name() == Some("untag-all"))
        .unwrap();
    let tags = secret.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("SecondTag"));

    // Re-add first tag, then remove both at once
    client
        .tag_resource()
        .secret_id("untag-all")
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key("FirstTag")
                .value("SomeValue")
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .secret_id("untag-all")
        .tag_keys("FirstTag")
        .tag_keys("SecondTag")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_secret()
        .secret_id("untag-all")
        .send()
        .await
        .unwrap();
    assert!(desc.tags().is_empty());
}

// ============================================================================
// Ported from moto: test_secretsmanager.py — GetRandomPassword
// ============================================================================

// Ported from moto: test_secretsmanager.py::test_get_random_password_default_requirements
#[tokio::test]
async fn test_get_random_password_default_requirements() {
    let client = make_sm_client().await;

    let result = client.get_random_password().send().await.unwrap();

    let password = result.random_password().unwrap();
    // Should contain lowercase, uppercase, digit, special character
    assert!(
        password.chars().any(|c| c.is_ascii_lowercase()),
        "Password should contain lowercase: {password}"
    );
    assert!(
        password.chars().any(|c| c.is_ascii_uppercase()),
        "Password should contain uppercase: {password}"
    );
    assert!(
        password.chars().any(|c| c.is_ascii_digit()),
        "Password should contain digit: {password}"
    );
    let punctuation = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    assert!(
        password.chars().any(|c| punctuation.contains(c)),
        "Password should contain punctuation: {password}"
    );
}

// Ported from moto: test_secretsmanager.py::test_get_random_exclude_lowercase
#[tokio::test]
async fn test_get_random_exclude_lowercase() {
    let client = make_sm_client().await;

    let result = client
        .get_random_password()
        .password_length(55)
        .exclude_lowercase(true)
        .send()
        .await
        .unwrap();

    let password = result.random_password().unwrap();
    assert!(
        !password.chars().any(|c| c.is_ascii_lowercase()),
        "Password should not contain lowercase: {password}"
    );
}

// Ported from moto: test_secretsmanager.py::test_get_random_exclude_uppercase
#[tokio::test]
async fn test_get_random_exclude_uppercase() {
    let client = make_sm_client().await;

    let result = client
        .get_random_password()
        .password_length(55)
        .exclude_uppercase(true)
        .send()
        .await
        .unwrap();

    let password = result.random_password().unwrap();
    assert!(
        !password.chars().any(|c| c.is_ascii_uppercase()),
        "Password should not contain uppercase: {password}"
    );
}

// Ported from moto: test_secretsmanager.py::test_get_random_exclude_numbers
#[tokio::test]
async fn test_get_random_exclude_numbers() {
    let client = make_sm_client().await;

    let result = client
        .get_random_password()
        .password_length(100)
        .exclude_numbers(true)
        .send()
        .await
        .unwrap();

    let password = result.random_password().unwrap();
    assert!(
        !password.chars().any(|c| c.is_ascii_digit()),
        "Password should not contain digits: {password}"
    );
}

// Ported from moto: test_secretsmanager.py::test_get_random_exclude_characters_and_symbols
#[tokio::test]
async fn test_get_random_exclude_characters_and_symbols() {
    let client = make_sm_client().await;

    let result = client
        .get_random_password()
        .password_length(20)
        .exclude_characters("xyzDje@?!.")
        .send()
        .await
        .unwrap();

    let password = result.random_password().unwrap();
    assert_eq!(password.len(), 20);
    for c in password.chars() {
        assert!(
            !"xyzDje@?!.".contains(c),
            "Password should not contain excluded char '{c}': {password}"
        );
    }
}

// Ported from moto: test_secretsmanager.py::test_get_random_include_space_false
#[tokio::test]
async fn test_get_random_include_space_false() {
    let client = make_sm_client().await;

    let result = client
        .get_random_password()
        .password_length(300)
        .send()
        .await
        .unwrap();

    let password = result.random_password().unwrap();
    assert!(
        !password.chars().any(|c| c.is_whitespace()),
        "Password should not contain spaces: {password}"
    );
}

// Ported from moto: test_secretsmanager.py::test_get_random_include_space_true
#[tokio::test]
async fn test_get_random_include_space_true() {
    let client = make_sm_client().await;

    let result = client
        .get_random_password()
        .password_length(4)
        .include_space(true)
        .send()
        .await
        .unwrap();

    let password = result.random_password().unwrap();
    assert!(
        password.chars().any(|c| c.is_whitespace()),
        "Password should contain space when IncludeSpace=true: {password}"
    );
}

// Ported from moto: test_secretsmanager.py::test_get_random_require_each_included_type
#[tokio::test]
async fn test_get_random_require_each_included_type() {
    let client = make_sm_client().await;

    let result = client
        .get_random_password()
        .password_length(4)
        .require_each_included_type(true)
        .send()
        .await
        .unwrap();

    let password = result.random_password().unwrap();
    let punctuation = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    assert!(
        password.chars().any(|c| punctuation.contains(c)),
        "Password should contain punctuation: {password}"
    );
    assert!(
        password.chars().any(|c| c.is_ascii_lowercase()),
        "Password should contain lowercase: {password}"
    );
    assert!(
        password.chars().any(|c| c.is_ascii_uppercase()),
        "Password should contain uppercase: {password}"
    );
    assert!(
        password.chars().any(|c| c.is_ascii_digit()),
        "Password should contain digit: {password}"
    );
}

// Ported from moto: test_secretsmanager.py::test_get_random_too_short_password
#[tokio::test]
async fn test_get_random_too_short_password() {
    let client = make_sm_client().await;

    let err = client.get_random_password().password_length(3).send().await;
    assert!(err.is_err(), "Password length 3 should be rejected");
}

// Ported from moto: test_secretsmanager.py::test_get_random_too_long_password
#[tokio::test]
async fn test_get_random_too_long_password() {
    let client = make_sm_client().await;

    let err = client
        .get_random_password()
        .password_length(5555)
        .send()
        .await;
    assert!(err.is_err(), "Password length 5555 should be rejected");
}

// ============================================================================
// Ported from moto: test_policy.py — ResourcePolicy
// ============================================================================

// Ported from moto: test_policy.py::test_get_initial_policy
#[tokio::test]
async fn test_get_initial_policy() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("policy-initial")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_resource_policy()
        .secret_id("policy-initial")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.name(), Some("policy-initial"));
    assert!(resp.arn().is_some());
    // No policy set yet, so ResourcePolicy should be None
    assert!(resp.resource_policy().is_none());
}

// Ported from moto: test_policy.py::test_put_resource_policy (JSON roundtrip)
#[tokio::test]
async fn test_put_resource_policy_json_roundtrip() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("policy-json-rt")
        .send()
        .await
        .unwrap();

    let policy = r#"{"Statement":[{"Action":"secretsmanager:GetSecretValue","Effect":"Allow","Principal":{"AWS":"arn:aws:iam::123456789012:role/tf-acc-test"},"Resource":"*","Sid":"EnableAllPermissions"}],"Version":"2012-10-17"}"#;

    let put_resp = client
        .put_resource_policy()
        .secret_id("policy-json-rt")
        .resource_policy(policy)
        .send()
        .await
        .unwrap();

    assert!(put_resp.arn().is_some());
    assert!(put_resp.name().is_some());

    let get_resp = client
        .get_resource_policy()
        .secret_id("policy-json-rt")
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.resource_policy(), Some(policy));
}

// Ported from moto: test_policy.py::test_policies_for_unknown_secrets
#[tokio::test]
async fn test_policies_for_unknown_secrets() {
    let client = make_sm_client().await;

    let err = client
        .put_resource_policy()
        .secret_id("unknown-secret")
        .resource_policy("p")
        .send()
        .await;
    assert!(err.is_err());

    let err = client
        .get_resource_policy()
        .secret_id("unknown-secret")
        .send()
        .await;
    assert!(err.is_err());

    let err = client
        .delete_resource_policy()
        .secret_id("unknown-secret")
        .send()
        .await;
    assert!(err.is_err());
}

// ============================================================================
// Ported from moto: test_secretsmanager.py — BatchGetSecretValue
// ============================================================================

// Ported from moto: test_secretsmanager.py::test_batch_get_secret_value_for_secret_id_list_without_matches
#[tokio::test]
async fn test_batch_get_secret_value_no_matches() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("batch-exists-only")
        .secret_string("secret")
        .send()
        .await
        .unwrap();

    let result = client
        .batch_get_secret_value()
        .secret_id_list("batch-no-match-1")
        .secret_id_list("batch-no-match-2")
        .send()
        .await
        .expect("batch_get_secret_value with all misses should still succeed");

    assert!(result.secret_values().is_empty());
    let errors = result.errors();
    assert_eq!(errors.len(), 2);
    assert_eq!(errors[0].secret_id(), Some("batch-no-match-1"));
    assert_eq!(errors[0].error_code(), Some("ResourceNotFoundException"));
    assert_eq!(errors[1].secret_id(), Some("batch-no-match-2"));
    assert_eq!(errors[1].error_code(), Some("ResourceNotFoundException"));
}

// Ported from moto: test_secretsmanager.py::test_batch_get_secret_value_for_secret_id_list_with_deleted_secret
#[tokio::test]
async fn test_batch_get_secret_value_with_deleted() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("batch-del-1")
        .secret_string("foosecret1")
        .send()
        .await
        .unwrap();

    client
        .create_secret()
        .name("batch-del-2")
        .secret_string("foosecret2")
        .send()
        .await
        .unwrap();

    client
        .delete_secret()
        .secret_id("batch-del-1")
        .send()
        .await
        .unwrap();

    let result = client
        .batch_get_secret_value()
        .secret_id_list("batch-del-1")
        .secret_id_list("batch-del-2")
        .send()
        .await
        .unwrap();

    // batch-del-1 is deleted, should be in errors
    let errors = result.errors();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].secret_id(), Some("batch-del-1"));
    assert_eq!(errors[0].error_code(), Some("InvalidRequestException"));

    // batch-del-2 is valid
    let values = result.secret_values();
    assert_eq!(values.len(), 1);
    assert_eq!(values[0].name(), Some("batch-del-2"));
    assert_eq!(values[0].secret_string(), Some("foosecret2"));
}

// ============================================================================
// Ported from moto: test_secretsmanager.py — ListSecretVersionIds
// ============================================================================

// Ported from moto: test_secretsmanager.py::test_can_list_secret_version_ids
#[tokio::test]
async fn test_can_list_secret_version_ids_multiple_puts() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("list-versions-multi")
        .secret_string("v1")
        .send()
        .await
        .unwrap();

    let put1 = client
        .put_secret_value()
        .secret_id("list-versions-multi")
        .secret_string("v2")
        .send()
        .await
        .unwrap();
    let v2_id = put1.version_id().unwrap().to_string();

    let put2 = client
        .put_secret_value()
        .secret_id("list-versions-multi")
        .secret_string("v3")
        .send()
        .await
        .unwrap();
    let v3_id = put2.version_id().unwrap().to_string();

    let result = client
        .list_secret_version_ids()
        .secret_id("list-versions-multi")
        .send()
        .await
        .unwrap();

    let versions = result.versions();
    let version_ids: Vec<&str> = versions.iter().filter_map(|v| v.version_id()).collect();
    assert!(version_ids.contains(&v2_id.as_str()));
    assert!(version_ids.contains(&v3_id.as_str()));
    // v2 should have AWSPREVIOUS, v3 should have AWSCURRENT
    let v2 = versions
        .iter()
        .find(|v| v.version_id() == Some(v2_id.as_str()))
        .unwrap();
    let v3 = versions
        .iter()
        .find(|v| v.version_id() == Some(v3_id.as_str()))
        .unwrap();
    assert!(v3.version_stages().contains(&"AWSCURRENT".to_string()));
    assert!(v2.version_stages().contains(&"AWSPREVIOUS".to_string()));
}

// ============================================================================
// Ported from moto: test_secretsmanager.py — UpdateSecretVersionStage
// ============================================================================

// Ported from moto: test_secretsmanager.py::test_update_secret_version_stage_manually
#[tokio::test]
async fn test_update_secret_version_stage_manually() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("stage-manual")
        .secret_string("initial")
        .send()
        .await
        .unwrap();

    // Put a current version
    let current = client
        .put_secret_value()
        .secret_id("stage-manual")
        .secret_string("previous_secret")
        .send()
        .await
        .unwrap();
    let current_version = current.version_id().unwrap().to_string();

    // Verify it's AWSCURRENT
    let initial = client
        .get_secret_value()
        .secret_id("stage-manual")
        .version_stage("AWSCURRENT")
        .send()
        .await
        .unwrap();
    assert!(initial.version_stages().contains(&"AWSCURRENT".to_string()));
    assert_eq!(initial.secret_string(), Some("previous_secret"));

    // Put a pending version
    let token = uuid::Uuid::new_v4().to_string();
    client
        .put_secret_value()
        .secret_id("stage-manual")
        .client_request_token(&token)
        .secret_string("new_secret")
        .version_stages("AWSPENDING")
        .send()
        .await
        .unwrap();

    // Verify pending version
    let pending = client
        .get_secret_value()
        .secret_id("stage-manual")
        .version_stage("AWSPENDING")
        .send()
        .await
        .unwrap();
    assert!(pending.version_stages().contains(&"AWSPENDING".to_string()));
    assert_eq!(pending.secret_string(), Some("new_secret"));

    // Move AWSCURRENT from current_version to the pending token
    client
        .update_secret_version_stage()
        .secret_id("stage-manual")
        .version_stage("AWSCURRENT")
        .move_to_version_id(&token)
        .remove_from_version_id(&current_version)
        .send()
        .await
        .unwrap();

    // Now the token version should have both AWSCURRENT and AWSPENDING
    let new_current = client
        .get_secret_value()
        .secret_id("stage-manual")
        .version_stage("AWSCURRENT")
        .send()
        .await
        .unwrap();
    let mut stages = new_current.version_stages().to_vec();
    stages.sort();
    assert_eq!(stages, vec!["AWSCURRENT", "AWSPENDING"]);
    assert_eq!(new_current.secret_string(), Some("new_secret"));

    // Old version should have AWSPREVIOUS
    let prev = client
        .get_secret_value()
        .secret_id("stage-manual")
        .version_stage("AWSPREVIOUS")
        .send()
        .await
        .unwrap();
    assert!(prev.version_stages().contains(&"AWSPREVIOUS".to_string()));
    assert_eq!(prev.secret_string(), Some("previous_secret"));
}

// ============================================================================
// Ported from moto: test_secretsmanager.py — ReplicateSecretToRegions
// ============================================================================

// Ported from moto: verify describe_secret shows replication status after replicate
#[tokio::test]
async fn test_replicate_secret_describe_shows_replication() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("replicate-describe")
        .secret_string("val")
        .send()
        .await
        .unwrap();

    client
        .replicate_secret_to_regions()
        .secret_id("replicate-describe")
        .add_replica_regions(
            aws_sdk_secretsmanager::types::ReplicaRegionType::builder()
                .region("eu-west-1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_secret()
        .secret_id("replicate-describe")
        .send()
        .await
        .unwrap();

    let repl = desc.replication_status();
    assert_eq!(repl.len(), 1);
    assert_eq!(repl[0].region(), Some("eu-west-1"));
    assert_eq!(repl[0].status().map(|s| s.as_str()), Some("InSync"));
}

// ============================================================================
// Ported from moto: test_secretsmanager.py — VersionsToStages consistency
// ============================================================================

// Ported from moto: test_secretsmanager.py::test_secret_versions_to_stages_attribute_discrepancy
#[tokio::test]
async fn test_secret_versions_to_stages_consistency() {
    let client = make_sm_client().await;

    let create = client
        .create_secret()
        .name("vtos-test")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();
    let previous_version_id = create.version_id().unwrap().to_string();

    let put = client
        .put_secret_value()
        .secret_id("vtos-test")
        .secret_string("dupe_secret")
        .send()
        .await
        .unwrap();
    let current_version_id = put.version_id().unwrap().to_string();

    // Check describe_secret version_ids_to_stages
    let desc = client
        .describe_secret()
        .secret_id("vtos-test")
        .send()
        .await
        .unwrap();

    let vtos = desc.version_ids_to_stages().unwrap();
    assert_eq!(
        vtos.get(&current_version_id).unwrap(),
        &vec!["AWSCURRENT".to_string()]
    );
    assert_eq!(
        vtos.get(&previous_version_id).unwrap(),
        &vec!["AWSPREVIOUS".to_string()]
    );

    // Check list_secrets secret_versions_to_stages
    let secrets = client.list_secrets().send().await.unwrap();
    let secret = secrets
        .secret_list()
        .iter()
        .find(|s| s.name() == Some("vtos-test"))
        .unwrap();
    let svtos = secret.secret_versions_to_stages().unwrap();
    assert_eq!(
        svtos.get(&current_version_id).unwrap(),
        &vec!["AWSCURRENT".to_string()]
    );
    assert_eq!(
        svtos.get(&previous_version_id).unwrap(),
        &vec!["AWSPREVIOUS".to_string()]
    );
}

// ============================================================================
// Tests derived from AWS documentation: Secrets Manager
// Additional tests to fill coverage gaps identified 2026-03-28
// ============================================================================

// GetSecretValue by explicit VersionId
#[tokio::test]
async fn test_get_secret_value_by_version_id() {
    let client = make_sm_client().await;

    let create = client
        .create_secret()
        .name("get-by-version-id")
        .secret_string("version1-value")
        .send()
        .await
        .unwrap();

    let v1_id = create.version_id().unwrap().to_string();

    // Put a second version so AWSCURRENT moves on
    let put = client
        .put_secret_value()
        .secret_id("get-by-version-id")
        .secret_string("version2-value")
        .send()
        .await
        .unwrap();
    let v2_id = put.version_id().unwrap().to_string();

    // Fetch the first version by its explicit VersionId
    let result_v1 = client
        .get_secret_value()
        .secret_id("get-by-version-id")
        .version_id(&v1_id)
        .send()
        .await
        .expect("get_secret_value with VersionId (v1) should succeed");

    assert!(result_v1.arn().is_some());
    assert_eq!(result_v1.name(), Some("get-by-version-id"));
    assert_eq!(result_v1.version_id(), Some(v1_id.as_str()));
    assert_eq!(result_v1.secret_string(), Some("version1-value"));

    // Fetch the second version by its explicit VersionId
    let result_v2 = client
        .get_secret_value()
        .secret_id("get-by-version-id")
        .version_id(&v2_id)
        .send()
        .await
        .expect("get_secret_value with VersionId (v2) should succeed");

    assert!(result_v2.arn().is_some());
    assert_eq!(result_v2.name(), Some("get-by-version-id"));
    assert_eq!(result_v2.version_id(), Some(v2_id.as_str()));
    assert_eq!(result_v2.secret_string(), Some("version2-value"));
}

// PutSecretValue idempotency: same ClientRequestToken + same value => no error, same version returned
#[tokio::test]
async fn test_put_secret_value_idempotent_same_token() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("idem-put-secret")
        .secret_string("initial")
        .send()
        .await
        .unwrap();

    let token = uuid::Uuid::new_v4().to_string();

    let put1 = client
        .put_secret_value()
        .secret_id("idem-put-secret")
        .client_request_token(&token)
        .secret_string("same-value")
        .send()
        .await
        .expect("first put_secret_value should succeed");

    let v1_id = put1.version_id().unwrap().to_string();

    // Same token and same value — idempotent, should succeed
    let put2 = client
        .put_secret_value()
        .secret_id("idem-put-secret")
        .client_request_token(&token)
        .secret_string("same-value")
        .send()
        .await
        .expect("idempotent put_secret_value should succeed");

    // The same version id must be returned
    assert_eq!(put2.version_id(), Some(v1_id.as_str()));
}

// PutSecretValue with explicit AWSPENDING stage
#[tokio::test]
async fn test_put_secret_value_awspending_stage() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("pending-stage-secret")
        .secret_string("current-value")
        .send()
        .await
        .unwrap();

    let pending_token = uuid::Uuid::new_v4().to_string();

    client
        .put_secret_value()
        .secret_id("pending-stage-secret")
        .client_request_token(&pending_token)
        .secret_string("pending-value")
        .version_stages("AWSPENDING")
        .send()
        .await
        .expect("put_secret_value with AWSPENDING stage should succeed");

    // Retrieve by AWSPENDING stage
    let result = client
        .get_secret_value()
        .secret_id("pending-stage-secret")
        .version_stage("AWSPENDING")
        .send()
        .await
        .expect("get_secret_value by AWSPENDING stage should succeed");

    assert_eq!(result.secret_string(), Some("pending-value"));
    assert!(result.version_stages().contains(&"AWSPENDING".to_string()));

    // AWSCURRENT should still be the original value
    let current = client
        .get_secret_value()
        .secret_id("pending-stage-secret")
        .send()
        .await
        .unwrap();
    assert_eq!(current.secret_string(), Some("current-value"));
}

// UpdateSecret on nonexistent secret should fail
#[tokio::test]
async fn test_update_secret_not_found() {
    let client = make_sm_client().await;

    let err = client
        .update_secret()
        .secret_id("nonexistent-update-target")
        .secret_string("new-value")
        .send()
        .await;

    assert!(err.is_err());
}

// PutSecretValue on nonexistent secret should fail
#[tokio::test]
async fn test_put_secret_value_not_found() {
    let client = make_sm_client().await;

    let err = client
        .put_secret_value()
        .secret_id("nonexistent-put-target")
        .secret_string("value")
        .send()
        .await;

    assert!(err.is_err());
}

// DeleteSecret with a valid recovery window (7 days — minimum allowed)
#[tokio::test]
async fn test_delete_secret_valid_recovery_window() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("del-recovery-7")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    let result = client
        .delete_secret()
        .secret_id("del-recovery-7")
        .recovery_window_in_days(7)
        .send()
        .await
        .expect("delete_secret with 7-day recovery window should succeed");

    assert!(result.arn().is_some());
    assert_eq!(result.name(), Some("del-recovery-7"));
    // DeletionDate should be set
    assert!(result.deletion_date().is_some());
}

// DeleteSecret with maximum valid recovery window (30 days)
#[tokio::test]
async fn test_delete_secret_valid_recovery_window_30days() {
    let client = make_sm_client().await;

    client
        .create_secret()
        .name("del-recovery-30")
        .secret_string("foosecret")
        .send()
        .await
        .unwrap();

    let result = client
        .delete_secret()
        .secret_id("del-recovery-30")
        .recovery_window_in_days(30)
        .send()
        .await
        .expect("delete_secret with 30-day recovery window should succeed");

    assert!(result.deletion_date().is_some());
    assert_eq!(result.name(), Some("del-recovery-30"));
}

// Full secret lifecycle: create -> describe -> update -> get -> delete -> restore -> get
#[tokio::test]
async fn test_full_secret_lifecycle() {
    let client = make_sm_client().await;

    // Step 1: Create
    let create = client
        .create_secret()
        .name("lifecycle-secret")
        .secret_string("original-value")
        .description("lifecycle test")
        .send()
        .await
        .expect("create should succeed");

    assert!(create.arn().is_some());
    assert_eq!(create.name(), Some("lifecycle-secret"));
    let arn = create.arn().unwrap().to_string();

    // Step 2: Describe
    let desc = client
        .describe_secret()
        .secret_id("lifecycle-secret")
        .send()
        .await
        .expect("describe should succeed");

    assert_eq!(desc.arn(), Some(arn.as_str()));
    assert_eq!(desc.description(), Some("lifecycle test"));
    assert!(desc.deleted_date().is_none());

    // Step 3: Update the secret value and description
    client
        .update_secret()
        .secret_id("lifecycle-secret")
        .secret_string("updated-value")
        .description("updated description")
        .send()
        .await
        .expect("update should succeed");

    // Step 4: Get and verify updated value
    let value = client
        .get_secret_value()
        .secret_id("lifecycle-secret")
        .send()
        .await
        .expect("get after update should succeed");
    assert_eq!(value.secret_string(), Some("updated-value"));

    // Step 5: Verify description was updated
    let desc2 = client
        .describe_secret()
        .secret_id("lifecycle-secret")
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.description(), Some("updated description"));

    // Step 6: Delete (soft delete with default recovery window)
    let deleted = client
        .delete_secret()
        .secret_id("lifecycle-secret")
        .send()
        .await
        .expect("delete should succeed");
    assert!(deleted.deletion_date().is_some());

    // Step 7: Get should fail after delete
    let err = client
        .get_secret_value()
        .secret_id("lifecycle-secret")
        .send()
        .await;
    assert!(err.is_err(), "get after delete should fail");

    // Step 8: Restore
    client
        .restore_secret()
        .secret_id("lifecycle-secret")
        .send()
        .await
        .expect("restore should succeed");

    // Step 9: Get should succeed after restore
    let restored_value = client
        .get_secret_value()
        .secret_id("lifecycle-secret")
        .send()
        .await
        .expect("get after restore should succeed");
    assert_eq!(restored_value.secret_string(), Some("updated-value"));

    // Step 10: Describe confirms no deletion date
    let desc3 = client
        .describe_secret()
        .secret_id("lifecycle-secret")
        .send()
        .await
        .unwrap();
    assert!(desc3.deleted_date().is_none());
}
