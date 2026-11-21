//! Table tags through the Redis backend: creation-time tags (issue #51).
//!
//! These tests need a Redis server. Set `WINTERBAUME_TEST_REDIS_URL` (for
//! example `redis://127.0.0.1:6379/15`) to run them; without it they return
//! early. Each test uses uniquely named tables and deletes them afterwards,
//! so the database does not need to be empty.

use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::config::BehaviorVersion;
use aws_sdk_dynamodb::operation::create_table::builders::CreateTableFluentBuilder;
use aws_sdk_dynamodb::types::{
    AttributeDefinition, BillingMode, KeySchemaElement, KeyType, ScalarAttributeType, Tag,
};
use winterbaume_core::MockAws;
use winterbaume_dynamodb::DynamoDbService;
use winterbaume_dynamodb_redis::RedisDynamoDbBackend;

/// A client backed by the Redis backend, or `None` when no Redis server is
/// configured.
async fn client() -> Option<Client> {
    let url = std::env::var("WINTERBAUME_TEST_REDIS_URL").ok()?;
    let backend = Arc::new(
        RedisDynamoDbBackend::from_connection_info(url.as_str())
            .await
            .expect("connect to WINTERBAUME_TEST_REDIS_URL"),
    );
    let mock = MockAws::builder()
        .with_service(DynamoDbService::with_backend(backend))
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dynamodb::config::Region::new("us-east-1"))
        .load()
        .await;
    Some(Client::new(&config))
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

async fn drop_table(client: &Client, table_name: &str) {
    client
        .delete_table()
        .table_name(table_name)
        .send()
        .await
        .unwrap();
}

fn tag(key: &str, value: &str) -> Tag {
    Tag::builder().key(key).value(value).build().unwrap()
}

/// The tags of `arn` as `(key, value)` pairs sorted by key.
async fn sorted_tags(client: &Client, arn: &str) -> Vec<(String, String)> {
    let resp = client
        .list_tags_of_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("ListTagsOfResource should succeed");
    let mut tags: Vec<(String, String)> = resp
        .tags()
        .iter()
        .map(|t| (t.key().to_string(), t.value().to_string()))
        .collect();
    tags.sort();
    tags
}

fn pairs(tags: &[(&str, &str)]) -> Vec<(String, String)> {
    tags.iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// Tags given to `CreateTable` are returned by `ListTagsOfResource` and form
/// one tag set with those added, replaced and removed afterwards.
#[tokio::test]
async fn create_table_tags_are_listed_and_merge_with_tag_resource() {
    let Some(client) = client().await else { return };
    let table = unique_table("tag-create");
    let created = on_demand_table(&client, &table)
        .tags(tag("app", "demo"))
        .tags(tag("env", "test"))
        .send()
        .await
        .unwrap();
    let arn = created
        .table_description()
        .unwrap()
        .table_arn()
        .unwrap()
        .to_string();
    assert_eq!(
        sorted_tags(&client, &arn).await,
        pairs(&[("app", "demo"), ("env", "test")])
    );

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(tag("later", "x"))
        .tags(tag("env", "prod"))
        .send()
        .await
        .unwrap();
    assert_eq!(
        sorted_tags(&client, &arn).await,
        pairs(&[("app", "demo"), ("env", "prod"), ("later", "x")])
    );

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("app")
        .send()
        .await
        .unwrap();
    assert_eq!(
        sorted_tags(&client, &arn).await,
        pairs(&[("env", "prod"), ("later", "x")])
    );

    // A table created without tags has none.
    let plain_table = unique_table("tag-none");
    let plain = on_demand_table(&client, &plain_table).send().await.unwrap();
    let plain_arn = plain.table_description().unwrap().table_arn().unwrap();
    assert!(sorted_tags(&client, plain_arn).await.is_empty());

    drop_table(&client, &table).await;
    drop_table(&client, &plain_table).await;
}

/// Tags are dropped with the table: a table re-created under the same name
/// ( and so the same ARN ) carries only the tags of its own `CreateTable`.
#[tokio::test]
async fn tags_do_not_outlive_the_table() {
    let Some(client) = client().await else { return };
    let table = unique_table("tag-reuse");
    let created = on_demand_table(&client, &table)
        .tags(tag("app", "demo"))
        .send()
        .await
        .unwrap();
    let arn = created
        .table_description()
        .unwrap()
        .table_arn()
        .unwrap()
        .to_string();
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(tag("later", "x"))
        .send()
        .await
        .unwrap();
    drop_table(&client, &table).await;

    let recreated = on_demand_table(&client, &table).send().await.unwrap();
    assert_eq!(
        recreated.table_description().unwrap().table_arn(),
        Some(arn.as_str())
    );
    assert!(
        sorted_tags(&client, &arn).await.is_empty(),
        "tags of the deleted table must not leak into its successor"
    );
    drop_table(&client, &table).await;

    on_demand_table(&client, &table)
        .tags(tag("generation", "3"))
        .send()
        .await
        .unwrap();
    assert_eq!(
        sorted_tags(&client, &arn).await,
        pairs(&[("generation", "3")])
    );
    drop_table(&client, &table).await;
    assert!(
        sorted_tags(&client, &arn).await.is_empty(),
        "DeleteTable removes the tag hash"
    );
}

/// Tags are not copied to a restored table ( AWS documents them as something
/// to set up manually after a restore ), and the source keeps its own.
#[tokio::test]
async fn restored_tables_do_not_inherit_tags() {
    let Some(client) = client().await else { return };
    let source = unique_table("tag-src");
    let from_backup_name = unique_table("tag-from-backup");
    let from_pitr_name = unique_table("tag-from-pitr");
    let created = on_demand_table(&client, &source)
        .tags(tag("app", "demo"))
        .send()
        .await
        .unwrap();
    let source_arn = created
        .table_description()
        .unwrap()
        .table_arn()
        .unwrap()
        .to_string();

    let backup = client
        .create_backup()
        .table_name(&source)
        .backup_name("tag-backup")
        .send()
        .await
        .unwrap();
    let backup_arn = backup.backup_details().unwrap().backup_arn().to_string();
    let from_backup = client
        .restore_table_from_backup()
        .target_table_name(&from_backup_name)
        .backup_arn(&backup_arn)
        .send()
        .await
        .unwrap();
    let from_backup_arn = from_backup
        .table_description()
        .unwrap()
        .table_arn()
        .unwrap();
    assert!(sorted_tags(&client, from_backup_arn).await.is_empty());

    let from_pitr = client
        .restore_table_to_point_in_time()
        .source_table_name(&source)
        .target_table_name(&from_pitr_name)
        .use_latest_restorable_time(true)
        .send()
        .await
        .unwrap();
    let from_pitr_arn = from_pitr.table_description().unwrap().table_arn().unwrap();
    assert!(sorted_tags(&client, from_pitr_arn).await.is_empty());

    assert_eq!(
        sorted_tags(&client, &source_arn).await,
        pairs(&[("app", "demo")])
    );

    client
        .delete_backup()
        .backup_arn(&backup_arn)
        .send()
        .await
        .unwrap();
    drop_table(&client, &source).await;
    drop_table(&client, &from_backup_name).await;
    drop_table(&client, &from_pitr_name).await;
}
