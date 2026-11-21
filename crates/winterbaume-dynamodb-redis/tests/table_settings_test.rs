//! Table settings through the Redis backend: deletion protection (issue #42)
//! and server-side encryption (issue #43).
//!
//! These tests need a Redis server. Set `WINTERBAUME_TEST_REDIS_URL` (for
//! example `redis://127.0.0.1:6379/15`) to run them; without it they return
//! early. Each test uses uniquely named tables (and, where it restores
//! state, a unique account) and deletes them afterwards, so the database
//! does not need to be empty.

use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::config::BehaviorVersion;
use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_sdk_dynamodb::operation::create_table::builders::CreateTableFluentBuilder;
use aws_sdk_dynamodb::types::{
    AttributeDefinition, AttributeValue, BillingMode, KeySchemaElement, KeyType,
    ProvisionedThroughput, ScalarAttributeType, SseSpecification, SseStatus, SseType,
    TableDescription, TableStatus,
};
use winterbaume_core::MockAws;
use winterbaume_dynamodb::state::DynamoDbError;
use winterbaume_dynamodb::types::{SseUpdate, TableSettings, TableSettingsUpdate};
use winterbaume_dynamodb::{DynamoDbBackend, DynamoDbService};
use winterbaume_dynamodb_redis::RedisDynamoDbBackend;

const DELETION_PROTECTED: &str = "Resource cannot be deleted as it is currently protected \
                                  against deletion. Disable deletion protection first.";

/// The Redis backend, or `None` when no Redis server is configured.
async fn backend() -> Option<Arc<RedisDynamoDbBackend>> {
    let url = std::env::var("WINTERBAUME_TEST_REDIS_URL").ok()?;
    Some(Arc::new(
        RedisDynamoDbBackend::from_connection_info(url.as_str())
            .await
            .expect("connect to WINTERBAUME_TEST_REDIS_URL"),
    ))
}

/// A client backed by `backend`.
async fn client_for(backend: Arc<RedisDynamoDbBackend>) -> Client {
    let mock = MockAws::builder()
        .with_service(DynamoDbService::with_backend(backend))
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dynamodb::config::Region::new("us-east-1"))
        .load()
        .await;
    Client::new(&config)
}

/// A client backed by the Redis backend, or `None` when no Redis server is
/// configured.
async fn client() -> Option<Client> {
    Some(client_for(backend().await?).await)
}

fn unique_table(prefix: &str) -> String {
    format!("{prefix}-{}", uuid::Uuid::new_v4().simple())
}

/// A `CreateTable` builder for an on-demand table keyed by `pk` (S).
fn on_demand_table(client: &Client, table_name: &str) -> CreateTableFluentBuilder {
    client
        .create_table()
        .table_name(table_name)
        .billing_mode(BillingMode::PayPerRequest)
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("pk")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("pk")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
}

/// Remove a table whatever its deletion protection says.
async fn drop_table(client: &Client, table_name: &str) {
    client
        .update_table()
        .table_name(table_name)
        .deletion_protection_enabled(false)
        .send()
        .await
        .unwrap();
    client
        .delete_table()
        .table_name(table_name)
        .send()
        .await
        .unwrap();
}

async fn describe(client: &Client, table_name: &str) -> TableDescription {
    client
        .describe_table()
        .table_name(table_name)
        .send()
        .await
        .unwrap()
        .table
        .unwrap()
}

/// Assert that `DeleteTable` is refused because of deletion protection and
/// that the table is still there afterwards.
async fn assert_delete_refused(client: &Client, table_name: &str) {
    let err = client
        .delete_table()
        .table_name(table_name)
        .send()
        .await
        .expect_err("DeleteTable must be rejected while deletion protection is on")
        .into_service_error();
    assert_eq!(err.code(), Some("ValidationException"));
    assert_eq!(err.message(), Some(DELETION_PROTECTED));

    let table = describe(client, table_name).await;
    assert_eq!(table.table_status(), Some(&TableStatus::Active));
    assert_eq!(table.deletion_protection_enabled(), Some(true));
    let names = client.list_tables().send().await.unwrap();
    assert!(names.table_names().contains(&table_name.to_string()));
}

/// Assert an `SSEDescription` of an enabled KMS-encrypted table and return
/// its key ARN.
fn assert_sse_enabled(desc: &TableDescription) -> String {
    let sse = desc
        .sse_description()
        .expect("SSEDescription must be present when SSE is enabled");
    assert_eq!(sse.status(), Some(&SseStatus::Enabled));
    assert_eq!(sse.sse_type(), Some(&SseType::Kms));
    sse.kms_master_key_arn()
        .expect("KMSMasterKeyArn must be present")
        .to_string()
}

#[tokio::test]
async fn deletion_protection_from_create_table_blocks_delete_table() {
    let Some(client) = client().await else { return };
    let table = unique_table("dp-create");

    let created = on_demand_table(&client, &table)
        .deletion_protection_enabled(true)
        .send()
        .await
        .unwrap();
    assert_eq!(
        created
            .table_description()
            .unwrap()
            .deletion_protection_enabled(),
        Some(true)
    );
    client
        .put_item()
        .table_name(&table)
        .item("pk", AttributeValue::S("kept".into()))
        .send()
        .await
        .unwrap();

    assert_delete_refused(&client, &table).await;
    let item = client
        .get_item()
        .table_name(&table)
        .key("pk", AttributeValue::S("kept".into()))
        .send()
        .await
        .unwrap();
    assert!(item.item().is_some(), "the rejected delete keeps the items");

    // An UpdateTable that carries nothing but the deletion protection flag.
    let updated = client
        .update_table()
        .table_name(&table)
        .deletion_protection_enabled(false)
        .send()
        .await
        .expect("UpdateTable toggling only deletion protection must be accepted");
    assert_eq!(
        updated
            .table_description()
            .unwrap()
            .deletion_protection_enabled(),
        Some(false)
    );

    let deleted = client
        .delete_table()
        .table_name(&table)
        .send()
        .await
        .expect("DeleteTable must succeed once deletion protection is off");
    let desc = deleted.table_description().unwrap();
    assert_eq!(desc.table_status(), Some(&TableStatus::Deleting));
    assert_eq!(desc.deletion_protection_enabled(), Some(false));

    let err = client
        .describe_table()
        .table_name(&table)
        .send()
        .await
        .expect_err("the table must be gone")
        .into_service_error();
    assert!(err.is_resource_not_found_exception());
}

#[tokio::test]
async fn deletion_protection_enabled_through_update_table() {
    let Some(client) = client().await else { return };
    let table = unique_table("dp-update");

    let created = on_demand_table(&client, &table).send().await.unwrap();
    assert_eq!(
        created
            .table_description()
            .unwrap()
            .deletion_protection_enabled(),
        Some(false),
        "deletion protection is off unless requested"
    );

    let updated = client
        .update_table()
        .table_name(&table)
        .deletion_protection_enabled(true)
        .send()
        .await
        .expect("UpdateTable toggling only deletion protection must be accepted");
    assert_eq!(
        updated
            .table_description()
            .unwrap()
            .deletion_protection_enabled(),
        Some(true)
    );
    assert_delete_refused(&client, &table).await;

    // An unrelated UpdateTable rewrites the stored table; the flag must
    // survive that.
    client
        .update_table()
        .table_name(&table)
        .billing_mode(BillingMode::Provisioned)
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(1)
                .write_capacity_units(1)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    assert_delete_refused(&client, &table).await;

    drop_table(&client, &table).await;
}

/// A table deleted and re-created under the same name starts with default
/// settings.
#[tokio::test]
async fn settings_do_not_outlive_the_table() {
    let Some(client) = client().await else { return };
    let table = unique_table("dp-reuse");

    on_demand_table(&client, &table)
        .deletion_protection_enabled(true)
        .sse_specification(SseSpecification::builder().enabled(true).build())
        .send()
        .await
        .unwrap();
    drop_table(&client, &table).await;

    let recreated = on_demand_table(&client, &table).send().await.unwrap();
    let desc = recreated.table_description().unwrap();
    assert_eq!(desc.deletion_protection_enabled(), Some(false));
    assert!(desc.sse_description().is_none());
    client
        .delete_table()
        .table_name(&table)
        .send()
        .await
        .expect("the re-created table is not protected");
}

#[tokio::test]
async fn update_table_settings_on_missing_table_is_not_found() {
    let Some(client) = client().await else { return };
    let table = unique_table("dp-missing");

    let err = client
        .update_table()
        .table_name(&table)
        .deletion_protection_enabled(true)
        .send()
        .await
        .expect_err("UpdateTable on a missing table must fail")
        .into_service_error();
    assert!(err.is_resource_not_found_exception());

    let created = on_demand_table(&client, &table).send().await.unwrap();
    assert_eq!(
        created
            .table_description()
            .unwrap()
            .deletion_protection_enabled(),
        Some(false),
        "the failed update left nothing behind"
    );
    drop_table(&client, &table).await;
}

#[tokio::test]
async fn sse_enabled_without_key_reports_aws_managed_key() {
    let Some(client) = client().await else { return };
    let table = unique_table("sse-managed");
    let other = unique_table("sse-managed");
    let sse_on = SseSpecification::builder().enabled(true).build();

    let created = on_demand_table(&client, &table)
        .sse_specification(sse_on.clone())
        .send()
        .await
        .unwrap();
    let desc = created.table_description().unwrap();
    let created_arn = assert_sse_enabled(desc);

    // arn:aws:dynamodb:<region>:<account>:table/<name>
    let account = desc.table_arn().unwrap().split(':').nth(4).unwrap();
    let prefix = format!("arn:aws:kms:us-east-1:{account}:key/");
    let key_id = created_arn
        .strip_prefix(&prefix)
        .unwrap_or_else(|| panic!("{created_arn} should start with {prefix}"));
    let groups: Vec<usize> = key_id.split('-').map(str::len).collect();
    assert_eq!(groups, [8, 4, 4, 4, 12], "key id {key_id} is UUID-shaped");

    assert_eq!(
        assert_sse_enabled(&describe(&client, &table).await),
        created_arn
    );

    // The AWS managed key is per account and region, not per table.
    let second = on_demand_table(&client, &other)
        .sse_specification(sse_on)
        .send()
        .await
        .unwrap();
    assert_eq!(
        assert_sse_enabled(second.table_description().unwrap()),
        created_arn
    );

    drop_table(&client, &table).await;
    drop_table(&client, &other).await;
}

#[tokio::test]
async fn sse_enabled_with_explicit_key_reports_its_arn() {
    let Some(client) = client().await else { return };

    let key_id = "1234abcd-12ab-34cd-56ef-1234567890ab";
    let account = "123456789012";
    let cases = [
        (
            key_id.to_string(),
            format!("arn:aws:kms:us-east-1:{account}:key/{key_id}"),
        ),
        (
            format!("arn:aws:kms:eu-west-1:210987654321:key/{key_id}"),
            format!("arn:aws:kms:eu-west-1:210987654321:key/{key_id}"),
        ),
        (
            "alias/my-table-key".to_string(),
            format!("arn:aws:kms:us-east-1:{account}:alias/my-table-key"),
        ),
        (
            format!("arn:aws:kms:us-east-1:{account}:alias/my-table-key"),
            format!("arn:aws:kms:us-east-1:{account}:alias/my-table-key"),
        ),
    ];

    for (key, expected_arn) in cases {
        let table = unique_table("sse-key");
        let created = on_demand_table(&client, &table)
            .sse_specification(
                SseSpecification::builder()
                    .enabled(true)
                    .sse_type(SseType::Kms)
                    .kms_master_key_id(&key)
                    .build(),
            )
            .send()
            .await
            .unwrap();
        let desc = created.table_description().unwrap();
        assert_eq!(
            desc.table_arn().unwrap().split(':').nth(4),
            Some(account),
            "the expectations above assume the default mock account"
        );
        assert_eq!(assert_sse_enabled(desc), expected_arn, "CreateTable {key}");
        assert_eq!(
            assert_sse_enabled(&describe(&client, &table).await),
            expected_arn,
            "DescribeTable {key}"
        );
        drop_table(&client, &table).await;
    }
}

#[tokio::test]
async fn sse_description_absent_unless_enabled() {
    let Some(client) = client().await else { return };
    let plain = unique_table("sse-none");
    let disabled = unique_table("sse-false");

    let created = on_demand_table(&client, &plain).send().await.unwrap();
    assert!(
        created
            .table_description()
            .unwrap()
            .sse_description()
            .is_none()
    );
    let created = on_demand_table(&client, &disabled)
        .sse_specification(SseSpecification::builder().enabled(false).build())
        .send()
        .await
        .unwrap();
    assert!(
        created
            .table_description()
            .unwrap()
            .sse_description()
            .is_none()
    );

    for table in [&plain, &disabled] {
        assert!(describe(&client, table).await.sse_description().is_none());
        drop_table(&client, table).await;
    }
}

#[tokio::test]
async fn sse_changed_through_update_table() {
    let Some(client) = client().await else { return };
    let table = unique_table("sse-update");
    on_demand_table(&client, &table).send().await.unwrap();

    let key_arn = "arn:aws:kms:us-east-1:123456789012:key/1234abcd-12ab-34cd-56ef-1234567890ab";
    let updated = client
        .update_table()
        .table_name(&table)
        .sse_specification(
            SseSpecification::builder()
                .enabled(true)
                .kms_master_key_id(key_arn)
                .build(),
        )
        .send()
        .await
        .expect("UpdateTable carrying only SSESpecification must be accepted");
    assert_eq!(
        assert_sse_enabled(updated.table_description().unwrap()),
        key_arn
    );

    // An unrelated UpdateTable keeps the encryption setting.
    let updated = client
        .update_table()
        .table_name(&table)
        .deletion_protection_enabled(true)
        .send()
        .await
        .unwrap();
    let desc = updated.table_description().unwrap();
    assert_eq!(assert_sse_enabled(desc), key_arn);
    assert_eq!(desc.deletion_protection_enabled(), Some(true));

    // Back to the AWS owned key; deletion protection is left alone.
    let updated = client
        .update_table()
        .table_name(&table)
        .sse_specification(SseSpecification::builder().enabled(false).build())
        .send()
        .await
        .unwrap();
    let desc = updated.table_description().unwrap();
    assert!(desc.sse_description().is_none());
    assert_eq!(desc.deletion_protection_enabled(), Some(true));

    let desc = describe(&client, &table).await;
    assert!(desc.sse_description().is_none());
    assert_eq!(desc.deletion_protection_enabled(), Some(true));

    drop_table(&client, &table).await;
}

/// Settings travel through `snapshot`, `restore` and `merge`, and a view
/// without `table_settings` (as written before they existed) restores the
/// table with default settings.
#[tokio::test]
async fn table_settings_survive_snapshot_restore_and_merge() {
    let Some(backend) = backend().await else {
        return;
    };
    let client = client_for(backend.clone()).await;
    let region = "us-east-1".to_string();
    let table = unique_table("snap-settings");
    let plain = unique_table("snap-plain");

    let created = on_demand_table(&client, &table)
        .deletion_protection_enabled(true)
        .sse_specification(
            SseSpecification::builder()
                .enabled(true)
                .kms_master_key_id("alias/snap-key")
                .build(),
        )
        .send()
        .await
        .unwrap();
    let desc = created.table_description().unwrap();
    let key_arn = assert_sse_enabled(desc);
    let account = desc
        .table_arn()
        .unwrap()
        .split(':')
        .nth(4)
        .unwrap()
        .to_string();
    on_demand_table(&client, &plain).send().await.unwrap();

    let expected = TableSettings {
        deletion_protection_enabled: true,
        sse_kms_master_key_arn: Some(key_arn),
    };

    // Snapshot the account, keeping only this test's tables.
    let mut view = backend.snapshot(account.clone(), region.clone()).await;
    view.tables
        .retain(|name, _| *name == table || *name == plain);
    view.table_settings
        .retain(|name, _| *name == table || *name == plain);
    view.tags.clear();
    view.ttl_configs.clear();
    assert!(view.tables.contains_key(&table) && view.tables.contains_key(&plain));
    assert!(
        !view.table_settings.contains_key(&plain),
        "default settings are not written to the snapshot"
    );
    let json = serde_json::to_value(&view).expect("serialize DynamodbStateView");

    // `restore` replaces a scope wholesale, so aim it at accounts nothing
    // else uses.
    let scratch = || format!("9{:011}", uuid::Uuid::new_v4().as_u128() % 100_000_000_000);

    // Restore: the settings come back and still block DeleteTable.
    let restored_acct = scratch();
    backend
        .restore(
            restored_acct.clone(),
            region.clone(),
            serde_json::from_value(json.clone()).unwrap(),
        )
        .await
        .unwrap();
    let settings = backend
        .describe_table_settings(restored_acct.clone(), region.clone(), table.clone())
        .await
        .unwrap();
    assert_eq!(settings, expected);
    assert_eq!(
        backend
            .describe_table_settings(restored_acct.clone(), region.clone(), plain.clone())
            .await
            .unwrap(),
        TableSettings::default()
    );
    let err = backend
        .delete_table(restored_acct.clone(), region.clone(), table.clone())
        .await
        .expect_err("a restored protected table cannot be deleted");
    assert!(
        matches!(&err, DynamoDbError::ValidationError(m) if m == DELETION_PROTECTED),
        "unexpected error: {err:?}"
    );

    // Restoring a view without `table_settings` over it clears them.
    let mut legacy_json = json.clone();
    legacy_json
        .as_object_mut()
        .unwrap()
        .remove("table_settings");
    backend
        .restore(
            restored_acct.clone(),
            region.clone(),
            serde_json::from_value(legacy_json.clone())
                .expect("views without table_settings still load"),
        )
        .await
        .unwrap();
    assert_eq!(
        backend
            .describe_table_settings(restored_acct.clone(), region.clone(), table.clone())
            .await
            .unwrap(),
        TableSettings::default()
    );

    // Merge: settings in the view are applied; a view without any leaves
    // the stored ones alone.
    backend
        .merge(
            restored_acct.clone(),
            region.clone(),
            serde_json::from_value(json).unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        backend
            .describe_table_settings(restored_acct.clone(), region.clone(), table.clone())
            .await
            .unwrap(),
        expected
    );
    backend
        .merge(
            restored_acct.clone(),
            region.clone(),
            serde_json::from_value(legacy_json).unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        backend
            .describe_table_settings(restored_acct.clone(), region.clone(), table.clone())
            .await
            .unwrap(),
        expected
    );

    // Clean up both scopes.
    backend
        .update_table_settings(
            restored_acct.clone(),
            region.clone(),
            table.clone(),
            TableSettingsUpdate {
                deletion_protection_enabled: Some(false),
                sse: Some(SseUpdate::Disabled),
            },
        )
        .await
        .unwrap();
    for name in [&table, &plain] {
        backend
            .delete_table(restored_acct.clone(), region.clone(), name.clone())
            .await
            .unwrap();
    }
    drop_table(&client, &table).await;
    drop_table(&client, &plain).await;
}

/// A table stored before the settings existed has no settings key; it must
/// load as unprotected and unencrypted, and be deletable.
#[tokio::test]
async fn table_without_stored_settings_has_defaults() {
    let Some(backend) = backend().await else {
        return;
    };
    let client = client_for(backend.clone()).await;
    let table = unique_table("legacy");

    let created = on_demand_table(&client, &table).send().await.unwrap();
    let account = created
        .table_description()
        .unwrap()
        .table_arn()
        .unwrap()
        .split(':')
        .nth(4)
        .unwrap()
        .to_string();

    assert_eq!(
        backend
            .describe_table_settings(account, "us-east-1".to_string(), table.clone())
            .await
            .unwrap(),
        TableSettings::default()
    );
    let desc = describe(&client, &table).await;
    assert_eq!(desc.deletion_protection_enabled(), Some(false));
    assert!(desc.sse_description().is_none());
    client
        .delete_table()
        .table_name(&table)
        .send()
        .await
        .expect("an unprotected table is deletable");
}
