//! Integration tests for winterbaume DynamoDB service.
//!
//! These tests verify that aws-sdk-dynamodb operations work end-to-end
//! through the winterbaume mock infrastructure.

use aws_sdk_dynamodb::config::BehaviorVersion;
use aws_sdk_dynamodb::types::{
    AttributeDefinition, AttributeValue, KeySchemaElement, KeyType, ProvisionedThroughput,
    ScalarAttributeType,
};
use winterbaume_core::MockAws;
use winterbaume_dynamodb::DynamoDbService;

/// Helper to create a configured DynamoDB client backed by winterbaume.
async fn make_dynamodb_client() -> aws_sdk_dynamodb::Client {
    let mock = MockAws::builder()
        .with_service(DynamoDbService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dynamodb::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_dynamodb::Client::new(&config)
}

/// Helper to create a simple hash-only table.
async fn create_hash_table(client: &aws_sdk_dynamodb::Client, table_name: &str) {
    client
        .create_table()
        .table_name(table_name)
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
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_table should succeed");
}

/// Helper to create a hash+range table.
async fn create_hash_range_table(client: &aws_sdk_dynamodb::Client, table_name: &str) {
    client
        .create_table()
        .table_name(table_name)
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("pk")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("sk")
                .key_type(KeyType::Range)
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
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("sk")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_table should succeed");
}

#[tokio::test]
async fn test_create_and_describe_table() {
    let client = make_dynamodb_client().await;

    let resp = client
        .create_table()
        .table_name("test-table")
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(10)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_table should succeed");

    let desc = resp
        .table_description()
        .expect("should have table description");
    assert_eq!(desc.table_name(), Some("test-table"));
    assert_eq!(
        desc.table_status(),
        Some(&aws_sdk_dynamodb::types::TableStatus::Active)
    );

    // Describe the table
    let desc_resp = client
        .describe_table()
        .table_name("test-table")
        .send()
        .await
        .expect("describe_table should succeed");

    let table = desc_resp.table().expect("should have table");
    assert_eq!(table.table_name(), Some("test-table"));
    assert!(table.table_arn().is_some(), "should have ARN");
}

#[tokio::test]
async fn test_delete_table() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "del-table").await;

    let resp = client
        .delete_table()
        .table_name("del-table")
        .send()
        .await
        .expect("delete_table should succeed");

    let desc = resp.table_description().unwrap();
    assert_eq!(
        desc.table_status(),
        Some(&aws_sdk_dynamodb::types::TableStatus::Deleting)
    );

    // describe should fail
    let err = client.describe_table().table_name("del-table").send().await;
    assert!(err.is_err(), "describe should fail after delete");
}

#[tokio::test]
async fn test_put_and_get_item() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "items-table").await;

    // Put an item
    client
        .put_item()
        .table_name("items-table")
        .item("pk", AttributeValue::S("user-1".to_string()))
        .item("name", AttributeValue::S("Alice".to_string()))
        .item("age", AttributeValue::N("30".to_string()))
        .send()
        .await
        .expect("put_item should succeed");

    // Get the item
    let resp = client
        .get_item()
        .table_name("items-table")
        .key("pk", AttributeValue::S("user-1".to_string()))
        .send()
        .await
        .expect("get_item should succeed");

    let item = resp.item().expect("should have item");
    assert_eq!(
        item.get("name").and_then(|v| v.as_s().ok()),
        Some(&"Alice".to_string())
    );
    assert_eq!(
        item.get("age").and_then(|v| v.as_n().ok()),
        Some(&"30".to_string())
    );
}

#[tokio::test]
async fn test_get_nonexistent_item() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "noitem-table").await;

    let resp = client
        .get_item()
        .table_name("noitem-table")
        .key("pk", AttributeValue::S("nonexistent".to_string()))
        .send()
        .await
        .expect("get_item should succeed (empty result)");

    assert!(resp.item().is_none(), "should have no item");
}

#[tokio::test]
async fn test_delete_item() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "delitem-table").await;

    client
        .put_item()
        .table_name("delitem-table")
        .item("pk", AttributeValue::S("item-1".to_string()))
        .item("data", AttributeValue::S("value".to_string()))
        .send()
        .await
        .unwrap();

    client
        .delete_item()
        .table_name("delitem-table")
        .key("pk", AttributeValue::S("item-1".to_string()))
        .send()
        .await
        .expect("delete_item should succeed");

    // Get should return nothing
    let resp = client
        .get_item()
        .table_name("delitem-table")
        .key("pk", AttributeValue::S("item-1".to_string()))
        .send()
        .await
        .unwrap();

    assert!(resp.item().is_none(), "item should be gone after delete");
}

#[tokio::test]
async fn test_put_item_overwrite() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "overwrite-table").await;

    // Put original
    client
        .put_item()
        .table_name("overwrite-table")
        .item("pk", AttributeValue::S("k1".to_string()))
        .item("val", AttributeValue::S("original".to_string()))
        .send()
        .await
        .unwrap();

    // Overwrite
    client
        .put_item()
        .table_name("overwrite-table")
        .item("pk", AttributeValue::S("k1".to_string()))
        .item("val", AttributeValue::S("updated".to_string()))
        .send()
        .await
        .unwrap();

    let resp = client
        .get_item()
        .table_name("overwrite-table")
        .key("pk", AttributeValue::S("k1".to_string()))
        .send()
        .await
        .unwrap();

    let item = resp.item().unwrap();
    assert_eq!(
        item.get("val").and_then(|v| v.as_s().ok()),
        Some(&"updated".to_string())
    );
}

#[tokio::test]
async fn test_scan() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "scan-table").await;

    // Put several items
    for i in 0..5 {
        client
            .put_item()
            .table_name("scan-table")
            .item("pk", AttributeValue::S(format!("item-{i}")))
            .item("idx", AttributeValue::N(i.to_string()))
            .send()
            .await
            .unwrap();
    }

    // Scan all
    let resp = client
        .scan()
        .table_name("scan-table")
        .send()
        .await
        .expect("scan should succeed");

    assert_eq!(resp.count(), 5);
    assert_eq!(resp.items().len(), 5);
}

#[tokio::test]
async fn test_scan_with_limit() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "scanlimit-table").await;

    for i in 0..5 {
        client
            .put_item()
            .table_name("scanlimit-table")
            .item("pk", AttributeValue::S(format!("item-{i:02}")))
            .send()
            .await
            .unwrap();
    }

    // Scan with limit
    let resp = client
        .scan()
        .table_name("scanlimit-table")
        .limit(2)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.count(), 2);
    assert!(
        resp.last_evaluated_key().is_some(),
        "should have last evaluated key for pagination"
    );
}

#[tokio::test]
async fn test_query_hash_range_table() {
    let client = make_dynamodb_client().await;
    create_hash_range_table(&client, "query-table").await;

    // Put items with same hash key but different range keys
    for i in 0..5 {
        client
            .put_item()
            .table_name("query-table")
            .item("pk", AttributeValue::S("user-1".to_string()))
            .item("sk", AttributeValue::S(format!("order-{i:02}")))
            .item("amount", AttributeValue::N((i * 100).to_string()))
            .send()
            .await
            .unwrap();
    }

    // Also put items for a different hash key
    client
        .put_item()
        .table_name("query-table")
        .item("pk", AttributeValue::S("user-2".to_string()))
        .item("sk", AttributeValue::S("order-00".to_string()))
        .item("amount", AttributeValue::N("999".to_string()))
        .send()
        .await
        .unwrap();

    // Query for user-1
    let resp = client
        .query()
        .table_name("query-table")
        .key_condition_expression("pk = :pk")
        .expression_attribute_values(":pk", AttributeValue::S("user-1".to_string()))
        .send()
        .await
        .expect("query should succeed");

    assert_eq!(resp.count(), 5, "should find 5 items for user-1");
    assert_eq!(resp.items().len(), 5);

    // Verify items are sorted by range key
    let items = resp.items();
    for (i, item) in items.iter().enumerate() {
        let sk = item.get("sk").and_then(|v| v.as_s().ok()).unwrap();
        assert_eq!(sk, &format!("order-{i:02}"));
    }
}

#[tokio::test]
async fn test_query_with_limit() {
    let client = make_dynamodb_client().await;
    create_hash_range_table(&client, "querylimit-table").await;

    for i in 0..5 {
        client
            .put_item()
            .table_name("querylimit-table")
            .item("pk", AttributeValue::S("pk1".to_string()))
            .item("sk", AttributeValue::S(format!("sk-{i:02}")))
            .send()
            .await
            .unwrap();
    }

    // Query with limit
    let resp = client
        .query()
        .table_name("querylimit-table")
        .key_condition_expression("pk = :pk")
        .expression_attribute_values(":pk", AttributeValue::S("pk1".to_string()))
        .limit(2)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.count(), 2);
    assert!(
        resp.last_evaluated_key().is_some(),
        "should have last evaluated key"
    );
}

#[tokio::test]
async fn test_create_duplicate_table_fails() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "dup-table").await;

    let err = client
        .create_table()
        .table_name("dup-table")
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
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(err.is_err(), "creating duplicate table should fail");
}

#[tokio::test]
async fn test_hash_range_get_item() {
    let client = make_dynamodb_client().await;
    create_hash_range_table(&client, "hr-get-table").await;

    client
        .put_item()
        .table_name("hr-get-table")
        .item("pk", AttributeValue::S("user-1".to_string()))
        .item("sk", AttributeValue::S("profile".to_string()))
        .item("name", AttributeValue::S("Alice".to_string()))
        .send()
        .await
        .unwrap();

    client
        .put_item()
        .table_name("hr-get-table")
        .item("pk", AttributeValue::S("user-1".to_string()))
        .item("sk", AttributeValue::S("settings".to_string()))
        .item("theme", AttributeValue::S("dark".to_string()))
        .send()
        .await
        .unwrap();

    // Get specific item by hash + range key
    let resp = client
        .get_item()
        .table_name("hr-get-table")
        .key("pk", AttributeValue::S("user-1".to_string()))
        .key("sk", AttributeValue::S("settings".to_string()))
        .send()
        .await
        .unwrap();

    let item = resp.item().expect("should have item");
    assert_eq!(
        item.get("theme").and_then(|v| v.as_s().ok()),
        Some(&"dark".to_string())
    );
}

#[tokio::test]
async fn test_describe_nonexistent_table_fails() {
    let client = make_dynamodb_client().await;

    let result = client
        .describe_table()
        .table_name("nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe_table on nonexistent table should fail"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_table_fails() {
    let client = make_dynamodb_client().await;

    let result = client.delete_table().table_name("nonexistent").send().await;

    assert!(
        result.is_err(),
        "delete_table on nonexistent table should fail"
    );
}

#[tokio::test]
async fn test_put_item_to_nonexistent_table_fails() {
    let client = make_dynamodb_client().await;

    let result = client
        .put_item()
        .table_name("nonexistent")
        .item("pk", AttributeValue::S("key-1".to_string()))
        .send()
        .await;

    assert!(result.is_err(), "put_item to nonexistent table should fail");
}

#[tokio::test]
async fn test_get_item_from_nonexistent_table_fails() {
    let client = make_dynamodb_client().await;

    let result = client
        .get_item()
        .table_name("nonexistent")
        .key("pk", AttributeValue::S("key-1".to_string()))
        .send()
        .await;

    assert!(
        result.is_err(),
        "get_item from nonexistent table should fail"
    );
}

#[tokio::test]
async fn test_scan_empty_table() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "empty-scan-table").await;

    let resp = client
        .scan()
        .table_name("empty-scan-table")
        .send()
        .await
        .expect("scan should succeed");

    assert_eq!(resp.count(), 0);
    assert!(resp.items().is_empty(), "items should be empty");
}

#[tokio::test]
async fn test_query_no_matching_items() {
    let client = make_dynamodb_client().await;
    create_hash_range_table(&client, "query-nomatch-table").await;

    // Put items for pk1
    for i in 0..3 {
        client
            .put_item()
            .table_name("query-nomatch-table")
            .item("pk", AttributeValue::S("pk1".to_string()))
            .item("sk", AttributeValue::S(format!("sk-{i:02}")))
            .send()
            .await
            .unwrap();
    }

    // Query for pk2 which has no items
    let resp = client
        .query()
        .table_name("query-nomatch-table")
        .key_condition_expression("pk = :pk")
        .expression_attribute_values(":pk", AttributeValue::S("pk2".to_string()))
        .send()
        .await
        .expect("query should succeed");

    assert_eq!(resp.count(), 0, "should find no items for pk2");
}

#[tokio::test]
async fn test_scan_pagination() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "scan-paginate-table").await;

    for i in 0..5 {
        client
            .put_item()
            .table_name("scan-paginate-table")
            .item("pk", AttributeValue::S(format!("item-{i:02}")))
            .send()
            .await
            .unwrap();
    }

    let mut all_items = Vec::new();

    // First page
    let resp = client
        .scan()
        .table_name("scan-paginate-table")
        .limit(2)
        .send()
        .await
        .expect("scan should succeed");

    assert_eq!(resp.count(), 2);
    all_items.extend(resp.items().to_vec());
    assert!(
        resp.last_evaluated_key().is_some(),
        "should have last_evaluated_key after first page"
    );

    let mut last_key = resp.last_evaluated_key().unwrap().clone();

    // Continue scanning until no more pages
    loop {
        let mut scan_builder = client.scan().table_name("scan-paginate-table").limit(2);

        for (k, v) in &last_key {
            scan_builder = scan_builder.exclusive_start_key(k.clone(), v.clone());
        }

        let resp = scan_builder.send().await.expect("scan should succeed");
        all_items.extend(resp.items().to_vec());

        match resp.last_evaluated_key() {
            Some(key) => {
                last_key = key.clone();
            }
            None => break,
        }
    }

    assert_eq!(
        all_items.len(),
        5,
        "should have collected all 5 items via pagination"
    );
}

#[tokio::test]
async fn test_query_with_range_key_begins_with() {
    let client = make_dynamodb_client().await;
    create_hash_range_table(&client, "query-begins-table").await;

    // Put items with pk="user" and various sk values
    client
        .put_item()
        .table_name("query-begins-table")
        .item("pk", AttributeValue::S("user".to_string()))
        .item("sk", AttributeValue::S("order-01".to_string()))
        .send()
        .await
        .unwrap();

    client
        .put_item()
        .table_name("query-begins-table")
        .item("pk", AttributeValue::S("user".to_string()))
        .item("sk", AttributeValue::S("order-02".to_string()))
        .send()
        .await
        .unwrap();

    client
        .put_item()
        .table_name("query-begins-table")
        .item("pk", AttributeValue::S("user".to_string()))
        .item("sk", AttributeValue::S("profile-01".to_string()))
        .send()
        .await
        .unwrap();

    // Query with begins_with on the range key
    let resp = client
        .query()
        .table_name("query-begins-table")
        .key_condition_expression("pk = :pk AND begins_with(sk, :prefix)")
        .expression_attribute_values(":pk", AttributeValue::S("user".to_string()))
        .expression_attribute_values(":prefix", AttributeValue::S("order".to_string()))
        .send()
        .await
        .expect("query with begins_with should succeed");

    // begins_with("order") matches order-01 and order-02 but not profile-01.
    assert_eq!(resp.count(), 2, "begins_with should filter on the sort key");
    assert_eq!(resp.items().len(), 2);
    for item in resp.items() {
        let sk = item.get("sk").and_then(|v| v.as_s().ok()).unwrap();
        assert!(
            sk.starts_with("order"),
            "every returned sk must start with 'order', got {sk}"
        );
    }
}

/// Regression test for the sort-key filter bug reported on 2026-04-30.
///
/// Before the fix, `<`, `<=`, `>`, `>=`, `BETWEEN`, and `begins_with` were
/// silently dropped from the `KeyConditionExpression`, causing Query to return
/// the entire partition instead of the requested range.
#[tokio::test]
async fn test_query_sort_key_conditions_full_operator_set() {
    let client = make_dynamodb_client().await;
    create_hash_range_table(&client, "query-sk-ops").await;

    let timestamps = [
        "2026-04-30T00:00:00Z",
        "2026-04-30T01:00:00Z",
        "2026-04-30T02:00:00Z",
        "2026-04-30T03:00:00Z",
    ];
    for ts in &timestamps {
        client
            .put_item()
            .table_name("query-sk-ops")
            .item("pk", AttributeValue::S("u-001".to_string()))
            .item("sk", AttributeValue::S(ts.to_string()))
            .send()
            .await
            .unwrap();
    }
    // Decoy in a different partition to verify hash-key filtering still works.
    client
        .put_item()
        .table_name("query-sk-ops")
        .item("pk", AttributeValue::S("u-999".to_string()))
        .item("sk", AttributeValue::S("2026-04-30T01:30:00Z".to_string()))
        .send()
        .await
        .unwrap();

    // sk > :t  → 2 items (02:00, 03:00)
    let resp = client
        .query()
        .table_name("query-sk-ops")
        .key_condition_expression("pk = :pk AND sk > :t")
        .expression_attribute_values(":pk", AttributeValue::S("u-001".to_string()))
        .expression_attribute_values(":t", AttributeValue::S("2026-04-30T01:30:00Z".to_string()))
        .send()
        .await
        .expect("query > should succeed");
    assert_eq!(resp.count(), 2, "sk > :t should match 2 items");

    // sk >= :t → 3 items (01:00, 02:00, 03:00)
    let resp = client
        .query()
        .table_name("query-sk-ops")
        .key_condition_expression("pk = :pk AND sk >= :t")
        .expression_attribute_values(":pk", AttributeValue::S("u-001".to_string()))
        .expression_attribute_values(":t", AttributeValue::S("2026-04-30T01:00:00Z".to_string()))
        .send()
        .await
        .expect("query >= should succeed");
    assert_eq!(resp.count(), 3, "sk >= :t should match 3 items");

    // sk < :t → 1 item (00:00)
    let resp = client
        .query()
        .table_name("query-sk-ops")
        .key_condition_expression("pk = :pk AND sk < :t")
        .expression_attribute_values(":pk", AttributeValue::S("u-001".to_string()))
        .expression_attribute_values(":t", AttributeValue::S("2026-04-30T01:00:00Z".to_string()))
        .send()
        .await
        .expect("query < should succeed");
    assert_eq!(resp.count(), 1, "sk < :t should match 1 item");

    // sk <= :t → 2 items (00:00, 01:00)
    let resp = client
        .query()
        .table_name("query-sk-ops")
        .key_condition_expression("pk = :pk AND sk <= :t")
        .expression_attribute_values(":pk", AttributeValue::S("u-001".to_string()))
        .expression_attribute_values(":t", AttributeValue::S("2026-04-30T01:00:00Z".to_string()))
        .send()
        .await
        .expect("query <= should succeed");
    assert_eq!(resp.count(), 2, "sk <= :t should match 2 items");

    // sk = :t → 1 item (exact match)
    let resp = client
        .query()
        .table_name("query-sk-ops")
        .key_condition_expression("pk = :pk AND sk = :t")
        .expression_attribute_values(":pk", AttributeValue::S("u-001".to_string()))
        .expression_attribute_values(":t", AttributeValue::S("2026-04-30T02:00:00Z".to_string()))
        .send()
        .await
        .expect("query = should succeed");
    assert_eq!(resp.count(), 1, "sk = :t should match 1 item");

    // sk BETWEEN :a AND :b → 2 items (01:00, 02:00); inclusive on both ends.
    let resp = client
        .query()
        .table_name("query-sk-ops")
        .key_condition_expression("pk = :pk AND sk BETWEEN :a AND :b")
        .expression_attribute_values(":pk", AttributeValue::S("u-001".to_string()))
        .expression_attribute_values(":a", AttributeValue::S("2026-04-30T01:00:00Z".to_string()))
        .expression_attribute_values(":b", AttributeValue::S("2026-04-30T02:00:00Z".to_string()))
        .send()
        .await
        .expect("query BETWEEN should succeed");
    assert_eq!(
        resp.count(),
        2,
        "sk BETWEEN should be inclusive on both ends"
    );
}

/// Regression test: range conditions on numeric sort keys must use numeric
/// comparison, not string lexicographic comparison.
#[tokio::test]
async fn test_query_numeric_sort_key_compares_numerically() {
    let client = make_dynamodb_client().await;
    client
        .create_table()
        .table_name("query-num-sk")
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("pk")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("sk")
                .key_type(KeyType::Range)
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
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("sk")
                .attribute_type(ScalarAttributeType::N)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_table should succeed");

    for n in &["2", "9", "10", "100"] {
        client
            .put_item()
            .table_name("query-num-sk")
            .item("pk", AttributeValue::S("p".to_string()))
            .item("sk", AttributeValue::N(n.to_string()))
            .send()
            .await
            .unwrap();
    }

    // Lexicographic order would say "10" < "9"; numeric must say 10 > 9.
    let resp = client
        .query()
        .table_name("query-num-sk")
        .key_condition_expression("pk = :pk AND sk > :n")
        .expression_attribute_values(":pk", AttributeValue::S("p".to_string()))
        .expression_attribute_values(":n", AttributeValue::N("9".to_string()))
        .send()
        .await
        .expect("numeric > should succeed");
    assert_eq!(resp.count(), 2, "sk > 9 must match 10 and 100, not 2");
}

// ===================================================================
// Moto-parity tests: translated from vendor/moto/tests/test_dynamodb/
// ===================================================================

/// Parity with moto test_create_table (test_dynamodb_table_without_range_key.py).
/// Verifies exact field values returned by DescribeTable.
#[tokio::test]
async fn test_moto_create_table_describe_fields() {
    let client = make_dynamodb_client().await;

    client
        .create_table()
        .table_name("moto-tbl")
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(1)
                .write_capacity_units(1)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_table should succeed");

    let actual = client
        .describe_table()
        .table_name("moto-tbl")
        .send()
        .await
        .unwrap()
        .table()
        .unwrap()
        .clone();

    // AttributeDefinitions
    assert_eq!(actual.attribute_definitions().len(), 1);
    assert_eq!(actual.attribute_definitions()[0].attribute_name(), "id");

    // KeySchema
    assert_eq!(actual.key_schema().len(), 1);
    assert_eq!(actual.key_schema()[0].attribute_name(), "id");
    assert_eq!(actual.key_schema()[0].key_type(), &KeyType::Hash);

    // ProvisionedThroughput
    let pt = actual.provisioned_throughput().unwrap();
    assert_eq!(pt.read_capacity_units(), Some(1));
    assert_eq!(pt.write_capacity_units(), Some(1));

    // Status, ARN, counts
    assert_eq!(
        actual.table_status(),
        Some(&aws_sdk_dynamodb::types::TableStatus::Active)
    );
    assert!(
        actual
            .table_arn()
            .unwrap()
            .contains("arn:aws:dynamodb:us-east-1:")
    );
    assert!(actual.table_arn().unwrap().ends_with(":table/moto-tbl"));
    assert_eq!(actual.table_name(), Some("moto-tbl"));
    assert_eq!(actual.item_count(), Some(0));
    assert_eq!(actual.table_size_bytes(), Some(0));
}

/// Parity with moto test_delete_table.
/// Verifies table is removed from ListTables after delete.
#[tokio::test]
async fn test_moto_delete_table_and_list_tables() {
    let client = make_dynamodb_client().await;

    create_hash_table(&client, "moto-del-tbl").await;

    // ListTables should show it
    let resp = client.list_tables().send().await.unwrap();
    assert!(resp.table_names().contains(&"moto-del-tbl".to_string()));

    // Delete
    client
        .delete_table()
        .table_name("moto-del-tbl")
        .send()
        .await
        .unwrap();

    // ListTables should no longer show it
    let resp = client.list_tables().send().await.unwrap();
    assert!(!resp.table_names().contains(&"moto-del-tbl".to_string()));

    // Deleting again should fail with ResourceNotFoundException
    let err = client
        .delete_table()
        .table_name("moto-del-tbl")
        .send()
        .await;
    assert!(err.is_err());
}

/// Parity with moto test_item_add_and_describe_and_update.
/// Tests PutItem, GetItem, and UpdateItem SET in sequence.
#[tokio::test]
async fn test_moto_put_get_update_item() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "moto-crud-tbl").await;

    // PutItem
    client
        .put_item()
        .table_name("moto-crud-tbl")
        .item("pk", AttributeValue::S("LOLCat Forum".into()))
        .item("Body", AttributeValue::S("http://url_to_lolcat.gif".into()))
        .item("SentBy", AttributeValue::S("User A".into()))
        .send()
        .await
        .unwrap();

    // GetItem - verify exact values
    let resp = client
        .get_item()
        .table_name("moto-crud-tbl")
        .key("pk", AttributeValue::S("LOLCat Forum".into()))
        .send()
        .await
        .unwrap();
    let item = resp.item().unwrap();
    assert_eq!(item.get("pk").unwrap().as_s().unwrap(), "LOLCat Forum");
    assert_eq!(
        item.get("Body").unwrap().as_s().unwrap(),
        "http://url_to_lolcat.gif"
    );
    assert_eq!(item.get("SentBy").unwrap().as_s().unwrap(), "User A");

    // UpdateItem SET SentBy=:user
    client
        .update_item()
        .table_name("moto-crud-tbl")
        .key("pk", AttributeValue::S("LOLCat Forum".into()))
        .update_expression("SET SentBy = :user")
        .expression_attribute_values(":user", AttributeValue::S("User B".into()))
        .send()
        .await
        .expect("update_item should succeed");

    // GetItem after update
    let resp = client
        .get_item()
        .table_name("moto-crud-tbl")
        .key("pk", AttributeValue::S("LOLCat Forum".into()))
        .send()
        .await
        .unwrap();
    let item = resp.item().unwrap();
    assert_eq!(item.get("SentBy").unwrap().as_s().unwrap(), "User B");
    // Body should remain unchanged
    assert_eq!(
        item.get("Body").unwrap().as_s().unwrap(),
        "http://url_to_lolcat.gif"
    );
}

/// Parity with moto test_update_item_set.
/// Tests SET + REMOVE in a single UpdateExpression.
#[tokio::test]
async fn test_moto_update_item_set_and_remove() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "moto-upd-sr-tbl").await;

    client
        .put_item()
        .table_name("moto-upd-sr-tbl")
        .item("pk", AttributeValue::S("steve".into()))
        .item("SentBy", AttributeValue::S("User A".into()))
        .send()
        .await
        .unwrap();

    client
        .update_item()
        .table_name("moto-upd-sr-tbl")
        .key("pk", AttributeValue::S("steve".into()))
        .update_expression("SET foo = :bar, blah = :baz REMOVE SentBy")
        .expression_attribute_values(":bar", AttributeValue::S("bar".into()))
        .expression_attribute_values(":baz", AttributeValue::S("baz".into()))
        .send()
        .await
        .unwrap();

    let resp = client
        .get_item()
        .table_name("moto-upd-sr-tbl")
        .key("pk", AttributeValue::S("steve".into()))
        .send()
        .await
        .unwrap();
    let item = resp.item().unwrap();
    assert_eq!(item.get("pk").unwrap().as_s().unwrap(), "steve");
    assert_eq!(item.get("foo").unwrap().as_s().unwrap(), "bar");
    assert_eq!(item.get("blah").unwrap().as_s().unwrap(), "baz");
    assert!(
        item.get("SentBy").is_none(),
        "SentBy should have been REMOVEd"
    );
}

/// Tests SET arithmetic: `SET quantity = quantity - :dec` and `SET counter = counter + :inc`.
#[tokio::test]
async fn test_update_item_set_arithmetic() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "set-arith-tbl").await;

    // Seed item with quantity=10 and counter=0
    client
        .put_item()
        .table_name("set-arith-tbl")
        .item("pk", AttributeValue::S("item1".into()))
        .item("quantity", AttributeValue::N("10".into()))
        .item("counter", AttributeValue::N("0".into()))
        .send()
        .await
        .unwrap();

    // Decrement quantity by 3
    client
        .update_item()
        .table_name("set-arith-tbl")
        .key("pk", AttributeValue::S("item1".into()))
        .update_expression("SET quantity = quantity - :dec")
        .expression_attribute_values(":dec", AttributeValue::N("3".into()))
        .send()
        .await
        .unwrap();

    let resp = client
        .get_item()
        .table_name("set-arith-tbl")
        .key("pk", AttributeValue::S("item1".into()))
        .send()
        .await
        .unwrap();
    let item = resp.item().unwrap();
    assert_eq!(item.get("quantity").unwrap().as_n().unwrap(), "7");

    // Increment counter by 5
    client
        .update_item()
        .table_name("set-arith-tbl")
        .key("pk", AttributeValue::S("item1".into()))
        .update_expression("SET counter = counter + :inc")
        .expression_attribute_values(":inc", AttributeValue::N("5".into()))
        .send()
        .await
        .unwrap();

    let resp = client
        .get_item()
        .table_name("set-arith-tbl")
        .key("pk", AttributeValue::S("item1".into()))
        .send()
        .await
        .unwrap();
    let item = resp.item().unwrap();
    assert_eq!(item.get("counter").unwrap().as_n().unwrap(), "5");
}

/// Parity with moto test_batch_write_item_to_multiple_tables.
/// Tests BatchWriteItem PutRequest and DeleteRequest across tables.
#[tokio::test]
async fn test_moto_batch_write_item() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "moto-bw-tbl0").await;
    create_hash_table(&client, "moto-bw-tbl1").await;
    create_hash_table(&client, "moto-bw-tbl2").await;

    // BatchWriteItem - put one item in each table
    use aws_sdk_dynamodb::types::{PutRequest, WriteRequest};
    client
        .batch_write_item()
        .request_items(
            "moto-bw-tbl0",
            vec![
                WriteRequest::builder()
                    .put_request(
                        PutRequest::builder()
                            .item("pk", AttributeValue::S("0".into()))
                            .build()
                            .unwrap(),
                    )
                    .build(),
            ],
        )
        .request_items(
            "moto-bw-tbl1",
            vec![
                WriteRequest::builder()
                    .put_request(
                        PutRequest::builder()
                            .item("pk", AttributeValue::S("1".into()))
                            .build()
                            .unwrap(),
                    )
                    .build(),
            ],
        )
        .request_items(
            "moto-bw-tbl2",
            vec![
                WriteRequest::builder()
                    .put_request(
                        PutRequest::builder()
                            .item("pk", AttributeValue::S("2".into()))
                            .build()
                            .unwrap(),
                    )
                    .build(),
            ],
        )
        .send()
        .await
        .expect("batch_write_item should succeed");

    // Verify each table has exactly 1 item with correct key
    for (idx, tbl) in ["moto-bw-tbl0", "moto-bw-tbl1", "moto-bw-tbl2"]
        .iter()
        .enumerate()
    {
        let resp = client
            .get_item()
            .table_name(*tbl)
            .key("pk", AttributeValue::S(idx.to_string()))
            .send()
            .await
            .unwrap();
        let item = resp.item().expect("should have item");
        assert_eq!(item.get("pk").unwrap().as_s().unwrap(), &idx.to_string());

        let scan = client.scan().table_name(*tbl).send().await.unwrap();
        assert_eq!(scan.count(), 1);
    }

    // BatchWriteItem - delete from each table
    use aws_sdk_dynamodb::types::DeleteRequest;
    client
        .batch_write_item()
        .request_items(
            "moto-bw-tbl0",
            vec![
                WriteRequest::builder()
                    .delete_request(
                        DeleteRequest::builder()
                            .key("pk", AttributeValue::S("0".into()))
                            .build()
                            .unwrap(),
                    )
                    .build(),
            ],
        )
        .request_items(
            "moto-bw-tbl1",
            vec![
                WriteRequest::builder()
                    .delete_request(
                        DeleteRequest::builder()
                            .key("pk", AttributeValue::S("1".into()))
                            .build()
                            .unwrap(),
                    )
                    .build(),
            ],
        )
        .request_items(
            "moto-bw-tbl2",
            vec![
                WriteRequest::builder()
                    .delete_request(
                        DeleteRequest::builder()
                            .key("pk", AttributeValue::S("2".into()))
                            .build()
                            .unwrap(),
                    )
                    .build(),
            ],
        )
        .send()
        .await
        .unwrap();

    // Verify all tables empty
    for tbl in &["moto-bw-tbl0", "moto-bw-tbl1", "moto-bw-tbl2"] {
        let scan = client.scan().table_name(*tbl).send().await.unwrap();
        assert_eq!(
            scan.count(),
            0,
            "table {} should be empty after delete",
            tbl
        );
    }
}

/// Parity with moto test_batch_items_returns_all (test_dynamodb_batch_get_item.py).
/// Tests BatchGetItem returns items that exist and skips non-existent keys.
#[tokio::test]
async fn test_moto_batch_get_item() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "moto-bg-tbl").await;

    // Put 3 items (user1, user2, user3)
    for i in 1..=3 {
        client
            .put_item()
            .table_name("moto-bg-tbl")
            .item("pk", AttributeValue::S(format!("user{i}")))
            .item("foo", AttributeValue::S("bar".into()))
            .send()
            .await
            .unwrap();
    }

    // BatchGetItem requesting 4 keys (user0 doesn't exist)
    use aws_sdk_dynamodb::types::KeysAndAttributes;
    let resp = client
        .batch_get_item()
        .request_items(
            "moto-bg-tbl",
            KeysAndAttributes::builder()
                .keys(std::collections::HashMap::from([(
                    "pk".to_string(),
                    AttributeValue::S("user0".into()),
                )]))
                .keys(std::collections::HashMap::from([(
                    "pk".to_string(),
                    AttributeValue::S("user1".into()),
                )]))
                .keys(std::collections::HashMap::from([(
                    "pk".to_string(),
                    AttributeValue::S("user2".into()),
                )]))
                .keys(std::collections::HashMap::from([(
                    "pk".to_string(),
                    AttributeValue::S("user3".into()),
                )]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("batch_get_item should succeed");

    let responses = resp.responses().unwrap();
    let items = responses.get("moto-bg-tbl").unwrap();
    assert_eq!(
        items.len(),
        3,
        "should return 3 items (user0 doesn't exist)"
    );

    // Verify all returned items have foo=bar
    for item in items {
        assert_eq!(item.get("foo").unwrap().as_s().unwrap(), "bar");
    }
}

/// Parity with moto test_delete_item: verifies item_count is updated.
#[tokio::test]
async fn test_moto_delete_item_count() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "moto-delcnt-tbl").await;

    client
        .put_item()
        .table_name("moto-delcnt-tbl")
        .item("pk", AttributeValue::S("LOLCat Forum".into()))
        .item("Body", AttributeValue::S("http://url_to_lolcat.gif".into()))
        .item("SentBy", AttributeValue::S("User A".into()))
        .item(
            "ReceivedTime",
            AttributeValue::S("12/9/2011 11:36:03 PM".into()),
        )
        .send()
        .await
        .unwrap();

    // Verify 1 item via scan
    let scan = client
        .scan()
        .table_name("moto-delcnt-tbl")
        .send()
        .await
        .unwrap();
    assert_eq!(scan.count(), 1);

    // Delete
    client
        .delete_item()
        .table_name("moto-delcnt-tbl")
        .key("pk", AttributeValue::S("LOLCat Forum".into()))
        .send()
        .await
        .unwrap();

    // Verify 0 items
    let scan = client
        .scan()
        .table_name("moto-delcnt-tbl")
        .send()
        .await
        .unwrap();
    assert_eq!(scan.count(), 0);

    // Delete again should succeed (idempotent)
    client
        .delete_item()
        .table_name("moto-delcnt-tbl")
        .key("pk", AttributeValue::S("LOLCat Forum".into()))
        .send()
        .await
        .expect("delete of non-existent item should succeed");
}

/// Parity with moto test_conditions.
/// Tests Query with KeyConditionExpression on hash-only table.
#[tokio::test]
async fn test_moto_query_hash_only_table() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "moto-qry-hash-tbl").await;

    client
        .put_item()
        .table_name("moto-qry-hash-tbl")
        .item("pk", AttributeValue::S("johndoe".into()))
        .send()
        .await
        .unwrap();
    client
        .put_item()
        .table_name("moto-qry-hash-tbl")
        .item("pk", AttributeValue::S("janedoe".into()))
        .send()
        .await
        .unwrap();

    let resp = client
        .query()
        .table_name("moto-qry-hash-tbl")
        .key_condition_expression("pk = :pk")
        .expression_attribute_values(":pk", AttributeValue::S("johndoe".into()))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.count(), 1);
    assert_eq!(resp.items().len(), 1);
    assert_eq!(
        resp.items()[0].get("pk").unwrap().as_s().unwrap(),
        "johndoe"
    );
}

/// Parity with moto test_key_condition_expressions (test_dynamodb_query.py).
/// Tests Query on hash+range table with forward and reverse order.
#[tokio::test]
async fn test_moto_query_key_condition_expressions() {
    let client = make_dynamodb_client().await;

    // Create hash+range table with forum_name/subject
    client
        .create_table()
        .table_name("moto-kce-tbl")
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("forum_name")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("subject")
                .key_type(KeyType::Range)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("forum_name")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("subject")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Put 3 items
    for subject in &["123", "456", "789"] {
        client
            .put_item()
            .table_name("moto-kce-tbl")
            .item("forum_name", AttributeValue::S("the-key".into()))
            .item("subject", AttributeValue::S(subject.to_string()))
            .send()
            .await
            .unwrap();
    }

    // Query all items forward
    let resp = client
        .query()
        .table_name("moto-kce-tbl")
        .key_condition_expression("forum_name = :fk")
        .expression_attribute_values(":fk", AttributeValue::S("the-key".into()))
        .scan_index_forward(true)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.count(), 3);
    let subjects: Vec<&str> = resp
        .items()
        .iter()
        .map(|it| it.get("subject").unwrap().as_s().unwrap().as_str())
        .collect();
    assert_eq!(subjects, vec!["123", "456", "789"]);

    // Query reverse
    let resp = client
        .query()
        .table_name("moto-kce-tbl")
        .key_condition_expression("forum_name = :fk")
        .expression_attribute_values(":fk", AttributeValue::S("the-key".into()))
        .scan_index_forward(false)
        .send()
        .await
        .unwrap();

    let subjects: Vec<&str> = resp
        .items()
        .iter()
        .map(|it| it.get("subject").unwrap().as_s().unwrap().as_str())
        .collect();
    assert_eq!(subjects, vec!["789", "456", "123"]);
}

/// Parity with moto test_item_put_without_table.
/// Exact error code assertion.
#[tokio::test]
async fn test_moto_put_item_nonexistent_table_error_code() {
    let client = make_dynamodb_client().await;

    let result = client
        .put_item()
        .table_name("nonexistent-moto-tbl")
        .item("pk", AttributeValue::S("key".into()))
        .send()
        .await;

    let err = result.err().unwrap();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_resource_not_found_exception(),
        "should be ResourceNotFoundException"
    );
}

/// Parity with moto test_scan_with_undeclared_table.
#[tokio::test]
async fn test_moto_scan_nonexistent_table_error_code() {
    let client = make_dynamodb_client().await;

    let result = client
        .scan()
        .table_name("nonexistent-moto-scan")
        .send()
        .await;

    let err = result.err().unwrap();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_resource_not_found_exception(),
        "should be ResourceNotFoundException"
    );
}

/// Parity with moto test_send_receive_message_without_attributes (DynamoDB scan/query pagination).
/// Tests paginated scan collecting all items.
#[tokio::test]
async fn test_moto_scan_pagination_all_items() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "moto-scanpg-tbl").await;

    // Insert 10 items
    for i in 0..10 {
        client
            .put_item()
            .table_name("moto-scanpg-tbl")
            .item("pk", AttributeValue::S(format!("item-{i:02}")))
            .item("data", AttributeValue::S(format!("data-{i}")))
            .send()
            .await
            .unwrap();
    }

    // Paginate with limit=3
    let mut all_items = Vec::new();
    let mut exclusive_start_key: Option<std::collections::HashMap<String, AttributeValue>> = None;

    loop {
        let mut scan_builder = client.scan().table_name("moto-scanpg-tbl").limit(3);

        if let Some(ref key) = exclusive_start_key {
            for (k, v) in key {
                scan_builder = scan_builder.exclusive_start_key(k.clone(), v.clone());
            }
        }

        let resp = scan_builder.send().await.unwrap();
        all_items.extend(resp.items().to_vec());

        match resp.last_evaluated_key() {
            Some(key) => exclusive_start_key = Some(key.clone()),
            None => break,
        }
    }

    assert_eq!(
        all_items.len(),
        10,
        "should collect all 10 items via scan pagination"
    );

    // Verify all original items are present
    let mut pks: Vec<String> = all_items
        .iter()
        .map(|it| it.get("pk").unwrap().as_s().unwrap().clone())
        .collect();
    pks.sort();
    let expected: Vec<String> = (0..10).map(|i| format!("item-{i:02}")).collect();
    assert_eq!(pks, expected);
}

/// Tests UpdateItem on non-existent item creates the item (upsert behavior).
/// This matches moto behavior where UpdateItem creates the item if not found.
#[tokio::test]
async fn test_moto_update_item_creates_nonexistent() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "moto-upsert-tbl").await;

    // UpdateItem on a key that doesn't exist yet
    client
        .update_item()
        .table_name("moto-upsert-tbl")
        .key("pk", AttributeValue::S("new-key".into()))
        .update_expression("SET xxx = :xxx")
        .expression_attribute_values(":xxx", AttributeValue::S("123".into()))
        .send()
        .await
        .unwrap();

    // Verify item was created
    let resp = client
        .get_item()
        .table_name("moto-upsert-tbl")
        .key("pk", AttributeValue::S("new-key".into()))
        .send()
        .await
        .unwrap();
    let item = resp.item().unwrap();
    assert_eq!(item.get("pk").unwrap().as_s().unwrap(), "new-key");
    assert_eq!(item.get("xxx").unwrap().as_s().unwrap(), "123");
}

/// Tests PutItem with ReturnValues=ALL_OLD.
/// Matches moto behavior of returning old item on overwrite.
#[tokio::test]
async fn test_moto_put_item_return_old() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "moto-retold-tbl").await;

    // Put original
    client
        .put_item()
        .table_name("moto-retold-tbl")
        .item("pk", AttributeValue::S("key1".into()))
        .item("foo", AttributeValue::S("bar".into()))
        .send()
        .await
        .unwrap();

    // Overwrite with ReturnValues=ALL_OLD
    let resp = client
        .put_item()
        .table_name("moto-retold-tbl")
        .item("pk", AttributeValue::S("key1".into()))
        .item("foo", AttributeValue::S("baz".into()))
        .return_values(aws_sdk_dynamodb::types::ReturnValue::AllOld)
        .send()
        .await
        .unwrap();

    // Should return old item
    let old = resp.attributes().unwrap();
    assert_eq!(old.get("foo").unwrap().as_s().unwrap(), "bar");
}

/// Parity: ListTables returns sorted table names.
#[tokio::test]
async fn test_moto_list_tables() {
    let client = make_dynamodb_client().await;

    create_hash_table(&client, "z-table").await;
    create_hash_table(&client, "a-table").await;
    create_hash_table(&client, "m-table").await;

    let resp = client.list_tables().send().await.unwrap();
    let names = resp.table_names();
    assert_eq!(names.len(), 3);
    assert_eq!(names, &["a-table", "m-table", "z-table"]);
}

#[tokio::test]
async fn test_put_item_multiple_attribute_types() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "multi-attr-table").await;

    client
        .put_item()
        .table_name("multi-attr-table")
        .item("pk", AttributeValue::S("key-1".to_string()))
        .item("name", AttributeValue::S("test-name".to_string()))
        .item("age", AttributeValue::N("25".to_string()))
        .item("active", AttributeValue::Bool(true))
        .item(
            "tags",
            AttributeValue::L(vec![
                AttributeValue::S("a".to_string()),
                AttributeValue::S("b".to_string()),
            ]),
        )
        .item(
            "details",
            AttributeValue::M(std::collections::HashMap::from([(
                "x".to_string(),
                AttributeValue::S("1".to_string()),
            )])),
        )
        .item("nullable", AttributeValue::Null(true))
        .send()
        .await
        .expect("put_item with multiple attribute types should succeed");

    let resp = client
        .get_item()
        .table_name("multi-attr-table")
        .key("pk", AttributeValue::S("key-1".to_string()))
        .send()
        .await
        .expect("get_item should succeed");

    let item = resp.item().expect("should have item");

    // Verify String
    assert_eq!(
        item.get("name").and_then(|v| v.as_s().ok()),
        Some(&"test-name".to_string())
    );

    // Verify Number
    assert_eq!(
        item.get("age").and_then(|v| v.as_n().ok()),
        Some(&"25".to_string())
    );

    // Verify Bool
    assert_eq!(
        item.get("active").and_then(|v| v.as_bool().ok()),
        Some(&true)
    );

    // Verify List
    let tags = item
        .get("tags")
        .and_then(|v| v.as_l().ok())
        .expect("should have tags list");
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].as_s().ok(), Some(&"a".to_string()));
    assert_eq!(tags[1].as_s().ok(), Some(&"b".to_string()));

    // Verify Map
    let details = item
        .get("details")
        .and_then(|v| v.as_m().ok())
        .expect("should have details map");
    assert_eq!(
        details.get("x").and_then(|v| v.as_s().ok()),
        Some(&"1".to_string())
    );

    // Verify Null
    assert_eq!(
        item.get("nullable").and_then(|v| v.as_null().ok()),
        Some(&true)
    );
}

#[tokio::test]
async fn test_delete_nonexistent_item_succeeds() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "delidempotent-table").await;

    // Deleting an item that does not exist should succeed (idempotent)
    client
        .delete_item()
        .table_name("delidempotent-table")
        .key("pk", AttributeValue::S("does-not-exist".to_string()))
        .send()
        .await
        .expect("delete_item on nonexistent key should succeed (idempotent)");
}

// ==================== New operation tests ====================

#[tokio::test]
async fn test_update_table() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "update-table-test").await;

    let resp = client
        .update_table()
        .table_name("update-table-test")
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .send()
        .await
        .expect("update_table should succeed");

    let desc = resp
        .table_description()
        .expect("should have table description");
    assert_eq!(desc.table_name(), Some("update-table-test"));
}

#[tokio::test]
async fn test_create_and_describe_backup() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "backup-table").await;

    // Create backup
    let resp = client
        .create_backup()
        .table_name("backup-table")
        .backup_name("my-backup")
        .send()
        .await
        .expect("create_backup should succeed");

    let details = resp.backup_details().expect("should have backup details");
    assert_eq!(details.backup_name(), "my-backup");
    assert_eq!(
        details.backup_status(),
        &aws_sdk_dynamodb::types::BackupStatus::Available
    );

    let backup_arn = details.backup_arn().to_string();

    // Describe backup
    let desc_resp = client
        .describe_backup()
        .backup_arn(&backup_arn)
        .send()
        .await
        .expect("describe_backup should succeed");

    let desc = desc_resp
        .backup_description()
        .expect("should have backup description");
    let bd = desc.backup_details().expect("should have details");
    assert_eq!(bd.backup_name(), "my-backup");
}

#[tokio::test]
async fn test_list_backups() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "list-backups-table").await;

    client
        .create_backup()
        .table_name("list-backups-table")
        .backup_name("backup-1")
        .send()
        .await
        .expect("create_backup should succeed");

    client
        .create_backup()
        .table_name("list-backups-table")
        .backup_name("backup-2")
        .send()
        .await
        .expect("create_backup should succeed");

    let resp = client
        .list_backups()
        .table_name("list-backups-table")
        .send()
        .await
        .expect("list_backups should succeed");

    let summaries = resp.backup_summaries();
    assert_eq!(summaries.len(), 2);
}

#[tokio::test]
async fn test_delete_backup() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "del-backup-table").await;

    let resp = client
        .create_backup()
        .table_name("del-backup-table")
        .backup_name("to-delete")
        .send()
        .await
        .expect("create_backup should succeed");

    let backup_arn = resp
        .backup_details()
        .expect("should have details")
        .backup_arn()
        .to_string();

    let del_resp = client
        .delete_backup()
        .backup_arn(&backup_arn)
        .send()
        .await
        .expect("delete_backup should succeed");

    assert!(del_resp.backup_description().is_some());

    // Should fail to describe deleted backup
    let err = client
        .describe_backup()
        .backup_arn(&backup_arn)
        .send()
        .await;
    assert!(err.is_err(), "describe should fail after delete");
}

#[tokio::test]
async fn test_restore_table_from_backup() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "restore-src").await;

    // Put an item
    client
        .put_item()
        .table_name("restore-src")
        .item("pk", AttributeValue::S("key1".to_string()))
        .item("data", AttributeValue::S("value1".to_string()))
        .send()
        .await
        .expect("put should succeed");

    // Create backup
    let backup_resp = client
        .create_backup()
        .table_name("restore-src")
        .backup_name("restore-backup")
        .send()
        .await
        .expect("create_backup should succeed");

    let backup_arn = backup_resp
        .backup_details()
        .unwrap()
        .backup_arn()
        .to_string();

    // Restore to new table
    let restore_resp = client
        .restore_table_from_backup()
        .target_table_name("restore-target")
        .backup_arn(&backup_arn)
        .send()
        .await
        .expect("restore should succeed");

    let desc = restore_resp.table_description().unwrap();
    assert_eq!(desc.table_name(), Some("restore-target"));

    // Verify restored data
    let get_resp = client
        .get_item()
        .table_name("restore-target")
        .key("pk", AttributeValue::S("key1".to_string()))
        .send()
        .await
        .expect("get should succeed");

    let item = get_resp.item().expect("should have item");
    assert_eq!(
        item.get("data").and_then(|v| v.as_s().ok()),
        Some(&"value1".to_string())
    );
}

#[tokio::test]
async fn test_restore_table_to_point_in_time() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pitr-src").await;

    client
        .put_item()
        .table_name("pitr-src")
        .item("pk", AttributeValue::S("pitr-key".to_string()))
        .send()
        .await
        .expect("put should succeed");

    let resp = client
        .restore_table_to_point_in_time()
        .source_table_name("pitr-src")
        .target_table_name("pitr-target")
        .use_latest_restorable_time(true)
        .send()
        .await
        .expect("restore should succeed");

    let desc = resp.table_description().unwrap();
    assert_eq!(desc.table_name(), Some("pitr-target"));
}

#[tokio::test]
async fn test_tag_untag_list_tags() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "tag-table").await;

    // Get the table ARN
    let desc = client
        .describe_table()
        .table_name("tag-table")
        .send()
        .await
        .expect("describe should succeed");
    let table_arn = desc.table().unwrap().table_arn().unwrap().to_string();

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&table_arn)
        .tags(
            aws_sdk_dynamodb::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_dynamodb::types::Tag::builder()
                .key("team")
                .value("backend")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let tags_resp = client
        .list_tags_of_resource()
        .resource_arn(&table_arn)
        .send()
        .await
        .expect("list_tags should succeed");

    let tags = tags_resp.tags();
    assert_eq!(tags.len(), 2);

    // Untag
    client
        .untag_resource()
        .resource_arn(&table_arn)
        .tag_keys("team")
        .send()
        .await
        .expect("untag should succeed");

    // List tags again
    let tags_resp2 = client
        .list_tags_of_resource()
        .resource_arn(&table_arn)
        .send()
        .await
        .expect("list_tags should succeed");

    let tags2 = tags_resp2.tags();
    assert_eq!(tags2.len(), 1);
    assert_eq!(tags2[0].key(), "env");
}

#[tokio::test]
async fn test_update_and_describe_time_to_live() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "ttl-table").await;

    // Describe TTL before setting (should be disabled)
    let desc = client
        .describe_time_to_live()
        .table_name("ttl-table")
        .send()
        .await
        .expect("describe_ttl should succeed");

    let ttl_desc = desc.time_to_live_description().unwrap();
    assert_eq!(
        ttl_desc.time_to_live_status(),
        Some(&aws_sdk_dynamodb::types::TimeToLiveStatus::Disabled)
    );

    // Enable TTL
    let update_resp = client
        .update_time_to_live()
        .table_name("ttl-table")
        .time_to_live_specification(
            aws_sdk_dynamodb::types::TimeToLiveSpecification::builder()
                .attribute_name("ttl")
                .enabled(true)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_ttl should succeed");

    let spec = update_resp.time_to_live_specification().unwrap();
    assert_eq!(spec.attribute_name(), "ttl");
    assert!(spec.enabled());

    // Describe TTL after setting
    let desc2 = client
        .describe_time_to_live()
        .table_name("ttl-table")
        .send()
        .await
        .expect("describe_ttl should succeed");

    let ttl_desc2 = desc2.time_to_live_description().unwrap();
    assert_eq!(
        ttl_desc2.time_to_live_status(),
        Some(&aws_sdk_dynamodb::types::TimeToLiveStatus::Enabled)
    );
}

#[tokio::test]
async fn test_update_and_describe_continuous_backups() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "cb-table").await;

    // Describe before update
    let desc = client
        .describe_continuous_backups()
        .table_name("cb-table")
        .send()
        .await
        .expect("describe_continuous_backups should succeed");

    let cb = desc.continuous_backups_description().unwrap();
    assert_eq!(
        cb.continuous_backups_status(),
        &aws_sdk_dynamodb::types::ContinuousBackupsStatus::Enabled
    );

    // Enable PITR
    let update_resp = client
        .update_continuous_backups()
        .table_name("cb-table")
        .point_in_time_recovery_specification(
            aws_sdk_dynamodb::types::PointInTimeRecoverySpecification::builder()
                .point_in_time_recovery_enabled(true)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_continuous_backups should succeed");

    let cb2 = update_resp.continuous_backups_description().unwrap();
    let pitr = cb2.point_in_time_recovery_description().unwrap();
    assert_eq!(
        pitr.point_in_time_recovery_status(),
        Some(&aws_sdk_dynamodb::types::PointInTimeRecoveryStatus::Enabled)
    );
}

#[tokio::test]
async fn test_resource_policy() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "policy-table").await;

    let desc = client
        .describe_table()
        .table_name("policy-table")
        .send()
        .await
        .unwrap();
    let table_arn = desc.table().unwrap().table_arn().unwrap().to_string();

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[]}"#;

    // Put policy
    let put_resp = client
        .put_resource_policy()
        .resource_arn(&table_arn)
        .policy(policy_doc)
        .send()
        .await
        .expect("put_resource_policy should succeed");
    assert!(put_resp.revision_id().is_some());

    // Get policy
    let get_resp = client
        .get_resource_policy()
        .resource_arn(&table_arn)
        .send()
        .await
        .expect("get_resource_policy should succeed");
    assert_eq!(get_resp.policy(), Some(policy_doc));

    // Delete policy
    let del_resp = client
        .delete_resource_policy()
        .resource_arn(&table_arn)
        .send()
        .await
        .expect("delete_resource_policy should succeed");
    assert!(del_resp.revision_id().is_some());

    // Get should return empty
    let get_resp2 = client
        .get_resource_policy()
        .resource_arn(&table_arn)
        .send()
        .await
        .expect("get should succeed after delete");
    assert!(get_resp2.policy().is_none());
}

#[tokio::test]
async fn test_describe_endpoints() {
    let client = make_dynamodb_client().await;

    let resp = client
        .describe_endpoints()
        .send()
        .await
        .expect("describe_endpoints should succeed");

    let endpoints = resp.endpoints();
    assert!(!endpoints.is_empty());
    assert!(endpoints[0].cache_period_in_minutes() > 0);
}

#[tokio::test]
async fn test_transact_get_items() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "tgi-table").await;

    client
        .put_item()
        .table_name("tgi-table")
        .item("pk", AttributeValue::S("k1".to_string()))
        .item("data", AttributeValue::S("v1".to_string()))
        .send()
        .await
        .unwrap();

    client
        .put_item()
        .table_name("tgi-table")
        .item("pk", AttributeValue::S("k2".to_string()))
        .item("data", AttributeValue::S("v2".to_string()))
        .send()
        .await
        .unwrap();

    let resp = client
        .transact_get_items()
        .transact_items(
            aws_sdk_dynamodb::types::TransactGetItem::builder()
                .get(
                    aws_sdk_dynamodb::types::Get::builder()
                        .table_name("tgi-table")
                        .key("pk", AttributeValue::S("k1".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .transact_items(
            aws_sdk_dynamodb::types::TransactGetItem::builder()
                .get(
                    aws_sdk_dynamodb::types::Get::builder()
                        .table_name("tgi-table")
                        .key("pk", AttributeValue::S("k2".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("transact_get_items should succeed");

    let responses = resp.responses();
    assert_eq!(responses.len(), 2);
}

#[tokio::test]
async fn test_transact_write_items() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "twi-table").await;

    client
        .transact_write_items()
        .transact_items(
            aws_sdk_dynamodb::types::TransactWriteItem::builder()
                .put(
                    aws_sdk_dynamodb::types::Put::builder()
                        .table_name("twi-table")
                        .item("pk", AttributeValue::S("tw1".to_string()))
                        .item("val", AttributeValue::S("hello".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .transact_items(
            aws_sdk_dynamodb::types::TransactWriteItem::builder()
                .put(
                    aws_sdk_dynamodb::types::Put::builder()
                        .table_name("twi-table")
                        .item("pk", AttributeValue::S("tw2".to_string()))
                        .item("val", AttributeValue::S("world".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("transact_write_items should succeed");

    // Verify items were written
    let get1 = client
        .get_item()
        .table_name("twi-table")
        .key("pk", AttributeValue::S("tw1".to_string()))
        .send()
        .await
        .unwrap();
    assert!(get1.item().is_some());

    let get2 = client
        .get_item()
        .table_name("twi-table")
        .key("pk", AttributeValue::S("tw2".to_string()))
        .send()
        .await
        .unwrap();
    assert!(get2.item().is_some());
}

// ─── PartiQL: ExecuteStatement ───────────────────────────────────────────────

#[tokio::test]
async fn test_execute_statement_select_star() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-sel-star").await;

    // Insert two items via PutItem API
    client
        .put_item()
        .table_name("pq-sel-star")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("msg1".into()),
        )
        .item(
            "body",
            aws_sdk_dynamodb::types::AttributeValue::S("some text".into()),
        )
        .send()
        .await
        .unwrap();
    client
        .put_item()
        .table_name("pq-sel-star")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("msg2".into()),
        )
        .item(
            "body",
            aws_sdk_dynamodb::types::AttributeValue::S("n/a".into()),
        )
        .send()
        .await
        .unwrap();

    // SELECT * should return both items
    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-sel-star\"")
        .send()
        .await
        .expect("execute_statement should succeed");

    let items = resp.items();
    assert_eq!(items.len(), 2);

    let pks: Vec<String> = items
        .iter()
        .filter_map(|item| item.get("pk")?.as_s().ok().cloned())
        .collect();
    assert!(pks.contains(&"msg1".to_string()));
    assert!(pks.contains(&"msg2".to_string()));
}

#[tokio::test]
async fn test_execute_statement_select_with_projection() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-sel-proj").await;

    client
        .put_item()
        .table_name("pq-sel-proj")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("msg1".into()),
        )
        .item(
            "body",
            aws_sdk_dynamodb::types::AttributeValue::S("some text".into()),
        )
        .item(
            "extra",
            aws_sdk_dynamodb::types::AttributeValue::S("data".into()),
        )
        .send()
        .await
        .unwrap();

    // SELECT only pk — should not include body or extra
    let resp = client
        .execute_statement()
        .statement("SELECT pk FROM \"pq-sel-proj\"")
        .send()
        .await
        .unwrap();

    let items = resp.items();
    assert_eq!(items.len(), 1);
    assert!(items[0].contains_key("pk"));
    assert!(!items[0].contains_key("body"));
    assert!(!items[0].contains_key("extra"));
}

#[tokio::test]
async fn test_execute_statement_with_parameter() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-sel-param").await;

    client
        .put_item()
        .table_name("pq-sel-param")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("msg1".into()),
        )
        .item(
            "body",
            aws_sdk_dynamodb::types::AttributeValue::S("some text".into()),
        )
        .send()
        .await
        .unwrap();
    client
        .put_item()
        .table_name("pq-sel-param")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("msg2".into()),
        )
        .send()
        .await
        .unwrap();

    // ? parameter binding
    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-sel-param\" WHERE pk = ?")
        .parameters(aws_sdk_dynamodb::types::AttributeValue::S("msg1".into()))
        .send()
        .await
        .unwrap();

    let items = resp.items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get("pk").unwrap().as_s().unwrap(), "msg1");
    assert_eq!(items[0].get("body").unwrap().as_s().unwrap(), "some text");
}

#[tokio::test]
async fn test_execute_statement_with_no_results() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-sel-empty").await;

    client
        .put_item()
        .table_name("pq-sel-empty")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("msg1".into()),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-sel-empty\" WHERE pk = ?")
        .parameters(aws_sdk_dynamodb::types::AttributeValue::S(
            "nonexistent".into(),
        ))
        .send()
        .await
        .unwrap();

    assert!(resp.items().is_empty());
}

// ─── PartiQL: INSERT ────────────────────────────────────────────────────────

#[tokio::test]
async fn test_execute_statement_insert() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-ins").await;

    // INSERT via PartiQL
    let resp = client
        .execute_statement()
        .statement("INSERT INTO \"pq-ins\" VALUE {'pk': 'msg1', 'body': 'hello'}")
        .send()
        .await
        .unwrap();
    assert!(resp.items().is_empty()); // writes return no items

    // Verify via Scan API
    let scan = client.scan().table_name("pq-ins").send().await.unwrap();
    let items = scan.items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get("pk").unwrap().as_s().unwrap(), "msg1");
    assert_eq!(items[0].get("body").unwrap().as_s().unwrap(), "hello");
}

#[tokio::test]
async fn test_execute_statement_insert_with_nested() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-ins-nested").await;

    client
        .execute_statement()
        .statement("INSERT INTO \"pq-ins-nested\" VALUE {'pk': 'msg1', 'attr': {'sth': ['other']}}")
        .send()
        .await
        .unwrap();

    let scan = client
        .scan()
        .table_name("pq-ins-nested")
        .send()
        .await
        .unwrap();
    let items = scan.items();
    assert_eq!(items.len(), 1);

    // attr should be M with sth -> L[S("other")]
    let attr = items[0].get("attr").unwrap().as_m().unwrap();
    let sth = attr.get("sth").unwrap().as_l().unwrap();
    assert_eq!(sth.len(), 1);
    assert_eq!(sth[0].as_s().unwrap(), "other");
}

// ─── PartiQL: UPDATE ────────────────────────────────────────────────────────

#[tokio::test]
async fn test_execute_statement_update() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-upd").await;

    // Seed data
    client
        .put_item()
        .table_name("pq-upd")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("msg1".into()),
        )
        .item(
            "body",
            aws_sdk_dynamodb::types::AttributeValue::S("original".into()),
        )
        .send()
        .await
        .unwrap();

    // Update existing attr
    client
        .execute_statement()
        .statement("UPDATE \"pq-upd\" SET body = 'updated' WHERE pk = 'msg1'")
        .send()
        .await
        .unwrap();

    let scan = client.scan().table_name("pq-upd").send().await.unwrap();
    let items = scan.items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get("body").unwrap().as_s().unwrap(), "updated");

    // Set new attr
    client
        .execute_statement()
        .statement("UPDATE \"pq-upd\" SET new_attr = 'asdf' WHERE pk = 'msg1'")
        .send()
        .await
        .unwrap();

    let scan = client.scan().table_name("pq-upd").send().await.unwrap();
    let items = scan.items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get("new_attr").unwrap().as_s().unwrap(), "asdf");
    assert_eq!(items[0].get("body").unwrap().as_s().unwrap(), "updated");

    // REMOVE attr
    client
        .execute_statement()
        .statement("UPDATE \"pq-upd\" REMOVE new_attr WHERE pk = 'msg1'")
        .send()
        .await
        .unwrap();

    let scan = client.scan().table_name("pq-upd").send().await.unwrap();
    let items = scan.items();
    assert_eq!(items.len(), 1);
    assert!(!items[0].contains_key("new_attr"));
    assert_eq!(items[0].get("body").unwrap().as_s().unwrap(), "updated");
}

// ─── PartiQL: DELETE ────────────────────────────────────────────────────────

#[tokio::test]
async fn test_execute_statement_delete() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-del").await;

    client
        .put_item()
        .table_name("pq-del")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("msg1".into()),
        )
        .item(
            "body",
            aws_sdk_dynamodb::types::AttributeValue::S("text".into()),
        )
        .send()
        .await
        .unwrap();
    client
        .put_item()
        .table_name("pq-del")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("msg2".into()),
        )
        .send()
        .await
        .unwrap();

    // DELETE msg1
    client
        .execute_statement()
        .statement("DELETE FROM \"pq-del\" WHERE pk = 'msg1'")
        .send()
        .await
        .unwrap();

    let scan = client.scan().table_name("pq-del").send().await.unwrap();
    let items = scan.items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get("pk").unwrap().as_s().unwrap(), "msg2");
}

// ─── PartiQL: ExecuteTransaction ────────────────────────────────────────────

#[tokio::test]
async fn test_execute_transaction_insert_and_select() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-txn").await;

    // INSERT via transaction
    client
        .execute_transaction()
        .transact_statements(
            aws_sdk_dynamodb::types::ParameterizedStatement::builder()
                .statement("INSERT INTO \"pq-txn\" VALUE {'pk': 'k1', 'data': 'v1'}")
                .build()
                .unwrap(),
        )
        .transact_statements(
            aws_sdk_dynamodb::types::ParameterizedStatement::builder()
                .statement("INSERT INTO \"pq-txn\" VALUE {'pk': 'k2', 'data': 'v2'}")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("execute_transaction should succeed");

    // Verify both items exist
    let scan = client.scan().table_name("pq-txn").send().await.unwrap();
    assert_eq!(scan.items().len(), 2);

    // SELECT via PartiQL to cross-verify
    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-txn\" WHERE pk = 'k1'")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.items().len(), 1);
    assert_eq!(resp.items()[0].get("data").unwrap().as_s().unwrap(), "v1");
}

// ─── PartiQL: BatchExecuteStatement ─────────────────────────────────────────

#[tokio::test]
async fn test_batch_execute_statement_select() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-batch").await;

    // Seed data
    client
        .put_item()
        .table_name("pq-batch")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("msg1".into()),
        )
        .item(
            "body",
            aws_sdk_dynamodb::types::AttributeValue::S("text1".into()),
        )
        .send()
        .await
        .unwrap();
    client
        .put_item()
        .table_name("pq-batch")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("msg2".into()),
        )
        .item(
            "body",
            aws_sdk_dynamodb::types::AttributeValue::S("text2".into()),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .batch_execute_statement()
        .statements(
            aws_sdk_dynamodb::types::BatchStatementRequest::builder()
                .statement("SELECT * FROM \"pq-batch\" WHERE pk = 'msg1'")
                .build()
                .unwrap(),
        )
        .statements(
            aws_sdk_dynamodb::types::BatchStatementRequest::builder()
                .statement("SELECT * FROM \"pq-batch\" WHERE pk = 'msg2'")
                .build()
                .unwrap(),
        )
        .statements(
            aws_sdk_dynamodb::types::BatchStatementRequest::builder()
                .statement("SELECT * FROM \"pq-batch\" WHERE pk = 'nonexistent'")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("batch_execute_statement should succeed");

    let responses = resp.responses();
    assert_eq!(responses.len(), 3);

    // First: msg1 found
    let item1 = responses[0].item().unwrap();
    assert_eq!(item1.get("pk").unwrap().as_s().unwrap(), "msg1");
    assert_eq!(item1.get("body").unwrap().as_s().unwrap(), "text1");

    // Second: msg2 found
    let item2 = responses[1].item().unwrap();
    assert_eq!(item2.get("pk").unwrap().as_s().unwrap(), "msg2");

    // Third: not found — no item
    assert!(responses[2].item().is_none());
}

#[tokio::test]
async fn test_batch_execute_statement_mixed_dml() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-batch-dml").await;

    // Seed
    client
        .put_item()
        .table_name("pq-batch-dml")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("old".into()),
        )
        .item(
            "body",
            aws_sdk_dynamodb::types::AttributeValue::S("text".into()),
        )
        .send()
        .await
        .unwrap();

    // Batch: UPDATE + DELETE + INSERT
    let resp = client
        .batch_execute_statement()
        .statements(
            aws_sdk_dynamodb::types::BatchStatementRequest::builder()
                .statement("UPDATE \"pq-batch-dml\" SET body = 'changed' WHERE pk = 'old'")
                .build()
                .unwrap(),
        )
        .statements(
            aws_sdk_dynamodb::types::BatchStatementRequest::builder()
                .statement("INSERT INTO \"pq-batch-dml\" VALUE {'pk': 'new'}")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(resp.responses().len(), 2);

    // Verify
    let scan = client
        .scan()
        .table_name("pq-batch-dml")
        .send()
        .await
        .unwrap();
    let items = scan.items();
    assert_eq!(items.len(), 2);

    let old = items
        .iter()
        .find(|i| i.get("pk").unwrap().as_s().unwrap() == "old")
        .unwrap();
    assert_eq!(old.get("body").unwrap().as_s().unwrap(), "changed");

    let new = items
        .iter()
        .find(|i| i.get("pk").unwrap().as_s().unwrap() == "new")
        .unwrap();
    assert!(!new.contains_key("body"));
}

// ─── PartiQL: Cross-API consistency ─────────────────────────────────────────

#[tokio::test]
async fn test_partiql_insert_then_get_item() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-cross").await;

    // INSERT via PartiQL
    client
        .execute_statement()
        .statement("INSERT INTO \"pq-cross\" VALUE {'pk': 'x', 'val': 42}")
        .send()
        .await
        .unwrap();

    // GetItem via standard API
    let resp = client
        .get_item()
        .table_name("pq-cross")
        .key("pk", aws_sdk_dynamodb::types::AttributeValue::S("x".into()))
        .send()
        .await
        .unwrap();

    let item = resp.item().unwrap();
    assert_eq!(item.get("pk").unwrap().as_s().unwrap(), "x");
    assert_eq!(item.get("val").unwrap().as_n().unwrap(), "42");
}

#[tokio::test]
async fn test_put_item_then_partiql_select() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-cross2").await;

    // PutItem via standard API
    client
        .put_item()
        .table_name("pq-cross2")
        .item("pk", aws_sdk_dynamodb::types::AttributeValue::S("y".into()))
        .item(
            "val",
            aws_sdk_dynamodb::types::AttributeValue::N("99".into()),
        )
        .send()
        .await
        .unwrap();

    // SELECT via PartiQL
    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-cross2\" WHERE pk = 'y'")
        .send()
        .await
        .unwrap();

    let items = resp.items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get("pk").unwrap().as_s().unwrap(), "y");
    assert_eq!(items[0].get("val").unwrap().as_n().unwrap(), "99");
}

#[tokio::test]
async fn test_describe_export() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "DescExportTable").await;

    let table_arn = "arn:aws:dynamodb:us-east-1:123456789012:table/DescExportTable";

    let export_resp = client
        .export_table_to_point_in_time()
        .table_arn(table_arn)
        .s3_bucket("my-export-bucket")
        .send()
        .await
        .expect("export should succeed");

    let export_arn = export_resp
        .export_description()
        .unwrap()
        .export_arn()
        .unwrap();

    let resp = client
        .describe_export()
        .export_arn(export_arn)
        .send()
        .await
        .expect("describe_export should succeed");

    let desc = resp.export_description().unwrap();
    assert_eq!(
        desc.export_status(),
        Some(&aws_sdk_dynamodb::types::ExportStatus::InProgress)
    );
}

#[tokio::test]
async fn test_describe_import() {
    let client = make_dynamodb_client().await;

    let import_resp = client
        .import_table()
        .s3_bucket_source(
            aws_sdk_dynamodb::types::S3BucketSource::builder()
                .s3_bucket("my-bucket")
                .build()
                .unwrap(),
        )
        .input_format(aws_sdk_dynamodb::types::InputFormat::Csv)
        .table_creation_parameters(
            aws_sdk_dynamodb::types::TableCreationParameters::builder()
                .table_name("desc-import-table")
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
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("import_table should succeed");

    let import_arn = import_resp
        .import_table_description()
        .unwrap()
        .import_arn()
        .unwrap();

    let resp = client
        .describe_import()
        .import_arn(import_arn)
        .send()
        .await
        .expect("describe_import should succeed");

    let desc = resp
        .import_table_description()
        .expect("should have import description");
    assert_eq!(
        desc.import_status(),
        Some(&aws_sdk_dynamodb::types::ImportStatus::InProgress)
    );
}

#[tokio::test]
async fn test_import_table() {
    let client = make_dynamodb_client().await;

    let resp = client
        .import_table()
        .s3_bucket_source(
            aws_sdk_dynamodb::types::S3BucketSource::builder()
                .s3_bucket("my-bucket")
                .build()
                .unwrap(),
        )
        .input_format(aws_sdk_dynamodb::types::InputFormat::Csv)
        .table_creation_parameters(
            aws_sdk_dynamodb::types::TableCreationParameters::builder()
                .table_name("imported-table")
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
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("import_table should succeed");

    let desc = resp
        .import_table_description()
        .expect("should have import description");
    assert!(desc.import_arn().is_some());
    assert_eq!(
        desc.import_status(),
        Some(&aws_sdk_dynamodb::types::ImportStatus::InProgress)
    );
}

#[tokio::test]
async fn test_list_exports() {
    let client = make_dynamodb_client().await;

    let resp = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");

    assert!(resp.export_summaries().is_empty());
}

// ============================================================================
// Ported from moto: test_dynamodb_transact.py, test_dynamodb_update_table.py,
//   test_dynamodb.py (tags, TTL, backups, continuous backups, endpoints)
// ============================================================================

// Ported from moto: test_dynamodb_transact.py::test_transact_get_items_should_return_empty_map_for_non_existent_item
#[tokio::test]
async fn test_moto_transact_get_items_empty_for_nonexistent() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-tgi-empty";
    create_hash_table(&client, table_name).await;

    // Put one item
    client
        .put_item()
        .table_name(table_name)
        .item("pk", AttributeValue::S("1".to_string()))
        .send()
        .await
        .unwrap();

    // TransactGetItems for one existing and one non-existing item
    let resp = client
        .transact_get_items()
        .transact_items(
            aws_sdk_dynamodb::types::TransactGetItem::builder()
                .get(
                    aws_sdk_dynamodb::types::Get::builder()
                        .table_name(table_name)
                        .key("pk", AttributeValue::S("1".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .transact_items(
            aws_sdk_dynamodb::types::TransactGetItem::builder()
                .get(
                    aws_sdk_dynamodb::types::Get::builder()
                        .table_name(table_name)
                        .key("pk", AttributeValue::S("2".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let responses = resp.responses();
    assert_eq!(responses.len(), 2);
    // First item should have the item
    assert!(responses[0].item().is_some());
    let item = responses[0].item().unwrap();
    assert_eq!(item.get("pk").unwrap().as_s().unwrap(), "1");
    // Second should be empty (no item)
    assert!(responses[1].item().is_none());
}

// Ported from moto: test_dynamodb_transact.py::test_transact_write_items_put
#[tokio::test]
async fn test_moto_transact_write_items_put() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-twi-put";

    client
        .create_table()
        .table_name(table_name)
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .send()
        .await
        .unwrap();

    // Put 5 items via transact_write
    let mut items = Vec::new();
    for i in 0..5 {
        items.push(
            aws_sdk_dynamodb::types::TransactWriteItem::builder()
                .put(
                    aws_sdk_dynamodb::types::Put::builder()
                        .table_name(table_name)
                        .item("id", AttributeValue::S(format!("foo{i}")))
                        .item("foo", AttributeValue::S("bar".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        );
    }

    let mut req = client.transact_write_items();
    for item in items {
        req = req.transact_items(item);
    }
    req.send().await.unwrap();

    // Verify all 5 items exist
    let scan = client.scan().table_name(table_name).send().await.unwrap();
    assert_eq!(scan.count(), 5);
}

// Ported from moto: test_dynamodb_transact.py::test_transact_write_items_delete
#[tokio::test]
async fn test_moto_transact_write_items_delete() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-twi-del";

    client
        .create_table()
        .table_name(table_name)
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .send()
        .await
        .unwrap();

    // Insert an item
    client
        .put_item()
        .table_name(table_name)
        .item("id", AttributeValue::S("foo".to_string()))
        .send()
        .await
        .unwrap();

    // Delete via transact_write
    client
        .transact_write_items()
        .transact_items(
            aws_sdk_dynamodb::types::TransactWriteItem::builder()
                .delete(
                    aws_sdk_dynamodb::types::Delete::builder()
                        .table_name(table_name)
                        .key("id", AttributeValue::S("foo".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Verify item is deleted
    let scan = client.scan().table_name(table_name).send().await.unwrap();
    assert_eq!(scan.count(), 0);
}

// Ported from moto: test_dynamodb_transact.py::test_transact_write_items_update
#[tokio::test]
async fn test_moto_transact_write_items_update() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-twi-upd";

    client
        .create_table()
        .table_name(table_name)
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .send()
        .await
        .unwrap();

    // Insert an item
    client
        .put_item()
        .table_name(table_name)
        .item("id", AttributeValue::S("foo".to_string()))
        .send()
        .await
        .unwrap();

    // Update via transact_write
    client
        .transact_write_items()
        .transact_items(
            aws_sdk_dynamodb::types::TransactWriteItem::builder()
                .update(
                    aws_sdk_dynamodb::types::Update::builder()
                        .table_name(table_name)
                        .key("id", AttributeValue::S("foo".to_string()))
                        .update_expression("SET #e = :v")
                        .expression_attribute_names("#e", "email_address")
                        .expression_attribute_values(
                            ":v",
                            AttributeValue::S("test@moto.com".to_string()),
                        )
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Verify item is updated
    let scan = client.scan().table_name(table_name).send().await.unwrap();
    assert_eq!(scan.count(), 1);
    let item = &scan.items()[0];
    assert_eq!(
        item.get("email_address").unwrap().as_s().unwrap(),
        "test@moto.com"
    );
}

// Ported from moto: test_dynamodb_update_table.py::test_update_table__billing_mode
#[tokio::test]
async fn test_moto_update_table_billing_mode() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-ut-billing";

    client
        .create_table()
        .table_name(table_name)
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("client")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("app")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("client")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("app")
                .key_type(KeyType::Range)
                .build()
                .unwrap(),
        )
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .send()
        .await
        .unwrap();

    // Update to PROVISIONED
    client
        .update_table()
        .table_name(table_name)
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::Provisioned)
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

    // Verify
    let desc = client
        .describe_table()
        .table_name(table_name)
        .send()
        .await
        .unwrap();
    let table = desc.table().unwrap();

    assert_eq!(
        table.billing_mode_summary().unwrap().billing_mode(),
        Some(&aws_sdk_dynamodb::types::BillingMode::Provisioned)
    );
    let pt = table.provisioned_throughput().unwrap();
    assert_eq!(pt.read_capacity_units(), Some(1));
    assert_eq!(pt.write_capacity_units(), Some(1));
}

// Ported from moto: test_dynamodb.py::test_list_table_tags
#[tokio::test]
async fn test_moto_list_table_tags() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-tags";
    create_hash_table(&client, table_name).await;

    let desc = client
        .describe_table()
        .table_name(table_name)
        .send()
        .await
        .unwrap();
    let arn = desc.table().unwrap().table_arn().unwrap();

    // Tag table
    client
        .tag_resource()
        .resource_arn(arn)
        .tags(
            aws_sdk_dynamodb::types::Tag::builder()
                .key("TestTag")
                .value("TestValue")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_dynamodb::types::Tag::builder()
                .key("TestTag2")
                .value("TestValue2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Check tags
    let resp = client
        .list_tags_of_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();
    let tags = resp.tags();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].key(), "TestTag");
    assert_eq!(tags[0].value(), "TestValue");
    assert_eq!(tags[1].key(), "TestTag2");
    assert_eq!(tags[1].value(), "TestValue2");

    // Remove 1 tag
    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("TestTag")
        .send()
        .await
        .unwrap();

    // Check tags
    let resp = client
        .list_tags_of_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();
    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "TestTag2");
    assert_eq!(tags[0].value(), "TestValue2");
}

// Ported from moto: test_dynamodb.py::test_list_table_tags_empty
#[tokio::test]
async fn test_moto_list_table_tags_empty() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-tags-empty";
    create_hash_table(&client, table_name).await;

    let desc = client
        .describe_table()
        .table_name(table_name)
        .send()
        .await
        .unwrap();
    let arn = desc.table().unwrap().table_arn().unwrap();

    let resp = client
        .list_tags_of_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();
    assert!(resp.tags().is_empty());
}

// Ported from moto: test_dynamodb.py::test_set_ttl
#[tokio::test]
async fn test_moto_set_ttl() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-ttl";
    create_hash_range_table(&client, table_name).await;

    // Enable TTL
    client
        .update_time_to_live()
        .table_name(table_name)
        .time_to_live_specification(
            aws_sdk_dynamodb::types::TimeToLiveSpecification::builder()
                .enabled(true)
                .attribute_name("expire")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_time_to_live()
        .table_name(table_name)
        .send()
        .await
        .unwrap();
    let ttl = resp.time_to_live_description().unwrap();
    assert_eq!(
        ttl.time_to_live_status(),
        Some(&aws_sdk_dynamodb::types::TimeToLiveStatus::Enabled)
    );
    assert_eq!(ttl.attribute_name(), Some("expire"));

    // Disable TTL
    client
        .update_time_to_live()
        .table_name(table_name)
        .time_to_live_specification(
            aws_sdk_dynamodb::types::TimeToLiveSpecification::builder()
                .enabled(false)
                .attribute_name("expire")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_time_to_live()
        .table_name(table_name)
        .send()
        .await
        .unwrap();
    let ttl = resp.time_to_live_description().unwrap();
    assert_eq!(
        ttl.time_to_live_status(),
        Some(&aws_sdk_dynamodb::types::TimeToLiveStatus::Disabled)
    );
}

// Ported from moto: test_dynamodb.py::test_describe_continuous_backups
#[tokio::test]
async fn test_moto_describe_continuous_backups_default() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-cb-default";

    client
        .create_table()
        .table_name(table_name)
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("client")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("app")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("client")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("app")
                .key_type(KeyType::Range)
                .build()
                .unwrap(),
        )
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_continuous_backups()
        .table_name(table_name)
        .send()
        .await
        .unwrap();

    let desc = resp.continuous_backups_description().unwrap();
    assert_eq!(
        desc.continuous_backups_status(),
        &aws_sdk_dynamodb::types::ContinuousBackupsStatus::Enabled
    );
    assert_eq!(
        desc.point_in_time_recovery_description()
            .unwrap()
            .point_in_time_recovery_status(),
        Some(&aws_sdk_dynamodb::types::PointInTimeRecoveryStatus::Disabled)
    );
}

// Ported from moto: test_dynamodb.py::test_describe_continuous_backups_errors
#[tokio::test]
async fn test_moto_describe_continuous_backups_errors() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-cb-err-nonexist";

    let err = client
        .describe_continuous_backups()
        .table_name(table_name)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("TableNotFoundException"),
        "Expected TableNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_dynamodb.py::test_update_continuous_backups
#[tokio::test]
async fn test_moto_update_continuous_backups() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-cb-update";

    client
        .create_table()
        .table_name(table_name)
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("client")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("app")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("client")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("app")
                .key_type(KeyType::Range)
                .build()
                .unwrap(),
        )
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .send()
        .await
        .unwrap();

    // Enable PITR
    let resp = client
        .update_continuous_backups()
        .table_name(table_name)
        .point_in_time_recovery_specification(
            aws_sdk_dynamodb::types::PointInTimeRecoverySpecification::builder()
                .point_in_time_recovery_enabled(true)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let desc = resp.continuous_backups_description().unwrap();
    assert_eq!(
        desc.continuous_backups_status(),
        &aws_sdk_dynamodb::types::ContinuousBackupsStatus::Enabled
    );
    assert_eq!(
        desc.point_in_time_recovery_description()
            .unwrap()
            .point_in_time_recovery_status(),
        Some(&aws_sdk_dynamodb::types::PointInTimeRecoveryStatus::Enabled)
    );

    // Disable PITR
    let resp = client
        .update_continuous_backups()
        .table_name(table_name)
        .point_in_time_recovery_specification(
            aws_sdk_dynamodb::types::PointInTimeRecoverySpecification::builder()
                .point_in_time_recovery_enabled(false)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let desc = resp.continuous_backups_description().unwrap();
    assert_eq!(
        desc.continuous_backups_status(),
        &aws_sdk_dynamodb::types::ContinuousBackupsStatus::Enabled
    );
    assert_eq!(
        desc.point_in_time_recovery_description()
            .unwrap()
            .point_in_time_recovery_status(),
        Some(&aws_sdk_dynamodb::types::PointInTimeRecoveryStatus::Disabled)
    );
}

// Ported from moto: test_dynamodb.py::test_update_continuous_backups_errors
#[tokio::test]
async fn test_moto_update_continuous_backups_errors() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-cb-upd-err-nonexist";

    let err = client
        .update_continuous_backups()
        .table_name(table_name)
        .point_in_time_recovery_specification(
            aws_sdk_dynamodb::types::PointInTimeRecoverySpecification::builder()
                .point_in_time_recovery_enabled(true)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("TableNotFoundException"),
        "Expected TableNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_dynamodb.py::test_create_backup
#[tokio::test]
async fn test_moto_create_backup() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-backup-create";
    create_hash_table(&client, table_name).await;

    let resp = client
        .create_backup()
        .table_name(table_name)
        .backup_name("my-backup")
        .send()
        .await
        .unwrap();

    let details = resp.backup_details().unwrap();
    assert!(details.backup_arn().contains(table_name));
    assert_eq!(details.backup_name(), "my-backup");
    assert!(details.backup_size_bytes() >= Some(0));
    assert_eq!(
        details.backup_status(),
        &aws_sdk_dynamodb::types::BackupStatus::Available
    );
    assert_eq!(
        details.backup_type(),
        &aws_sdk_dynamodb::types::BackupType::User
    );
    // backup_creation_date_time is always present (not Option)
}

// Ported from moto: test_dynamodb.py::test_create_backup_for_non_existent_table_raises_error
#[tokio::test]
async fn test_moto_create_backup_nonexistent_table() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-backup-noexist";

    let err = client
        .create_backup()
        .table_name(table_name)
        .backup_name("backup")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("TableNotFoundException"),
        "Expected TableNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_dynamodb.py::test_create_multiple_backups_with_same_name
#[tokio::test]
async fn test_moto_create_multiple_backups_same_name() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-backup-multi";
    create_hash_table(&client, table_name).await;

    let mut backup_arns = Vec::new();
    for _ in 0..4 {
        let resp = client
            .create_backup()
            .table_name(table_name)
            .backup_name("same-name")
            .send()
            .await
            .unwrap();
        let details = resp.backup_details().unwrap();
        assert_eq!(details.backup_name(), "same-name");
        let arn = details.backup_arn().to_string();
        assert!(!backup_arns.contains(&arn));
        backup_arns.push(arn);
    }
}

// Ported from moto: test_dynamodb.py::test_describe_backup
#[tokio::test]
async fn test_moto_describe_backup() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-backup-desc";
    let create_resp = client
        .create_table()
        .table_name(table_name)
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let table_arn = create_resp
        .table_description()
        .unwrap()
        .table_arn()
        .unwrap();

    let backup_resp = client
        .create_backup()
        .table_name(table_name)
        .backup_name("my-backup")
        .send()
        .await
        .unwrap();
    let backup_arn = backup_resp
        .backup_details()
        .unwrap()
        .backup_arn()
        .to_string();

    let desc = client
        .describe_backup()
        .backup_arn(&backup_arn)
        .send()
        .await
        .unwrap();
    let description = desc.backup_description().unwrap();

    let details = description.backup_details().unwrap();
    assert!(details.backup_arn().contains(table_name));
    assert_eq!(details.backup_name(), "my-backup");
    assert_eq!(
        details.backup_status(),
        &aws_sdk_dynamodb::types::BackupStatus::Available
    );
    assert_eq!(
        details.backup_type(),
        &aws_sdk_dynamodb::types::BackupType::User
    );

    let source = description.source_table_details().unwrap();
    assert_eq!(source.table_name(), table_name);
    assert_eq!(source.table_arn(), Some(table_arn));
    assert!(!source.key_schema().is_empty());
    assert!(source.provisioned_throughput().is_some());
}

// Ported from moto: test_dynamodb.py::test_describe_backup_for_non_existent_backup_raises_error
#[tokio::test]
async fn test_moto_describe_backup_nonexistent() {
    let client = make_dynamodb_client().await;
    let non_existent_arn =
        "arn:aws:dynamodb:us-east-1:123456789012:table/table-name/backup/01623095754481-2cfcd6f9";

    let err = client
        .describe_backup()
        .backup_arn(non_existent_arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("BackupNotFoundException"),
        "Expected BackupNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_dynamodb.py::test_list_backups_for_non_existent_table
#[tokio::test]
async fn test_moto_list_backups_nonexistent_table() {
    let client = make_dynamodb_client().await;

    let resp = client
        .list_backups()
        .table_name("nonexistent-table")
        .send()
        .await
        .unwrap();

    assert!(resp.backup_summaries().is_empty());
}

// Ported from moto: test_dynamodb.py::test_list_backups
#[tokio::test]
async fn test_moto_list_backups() {
    let client = make_dynamodb_client().await;

    // Create two tables with two backups each
    for table_name in &["moto-lb-t1", "moto-lb-t2"] {
        create_hash_table(&client, table_name).await;
        for backup_name in &["backup-1", "backup-2"] {
            client
                .create_backup()
                .table_name(*table_name)
                .backup_name(*backup_name)
                .send()
                .await
                .unwrap();
        }
    }

    // List backups for specific table
    for table_name in &["moto-lb-t1", "moto-lb-t2"] {
        let resp = client
            .list_backups()
            .table_name(*table_name)
            .send()
            .await
            .unwrap();
        let summaries = resp.backup_summaries();
        assert_eq!(summaries.len(), 2);
        for summary in summaries {
            assert_eq!(summary.table_name(), Some(*table_name));
            assert!(summary.table_arn().unwrap().contains(table_name));
            assert!(summary.backup_arn().is_some());
            assert!(summary.backup_creation_date_time().is_some());
        }
    }
}

// Ported from moto: test_dynamodb.py::test_delete_backup
#[tokio::test]
async fn test_moto_delete_backup() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-backup-del";
    create_hash_table(&client, table_name).await;

    // Create 2 backups
    for name in &["backup-1", "backup-2"] {
        client
            .create_backup()
            .table_name(table_name)
            .backup_name(*name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_backups()
        .table_name(table_name)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.backup_summaries().len(), 2);

    // Delete first backup
    let backup_to_delete = resp.backup_summaries()[0].backup_arn().unwrap().to_string();
    let del_resp = client
        .delete_backup()
        .backup_arn(&backup_to_delete)
        .send()
        .await
        .unwrap();

    let desc = del_resp.backup_description().unwrap();
    assert!(desc.backup_details().is_some());
    assert!(desc.source_table_details().is_some());
    let details = desc.backup_details().unwrap();
    assert_eq!(details.backup_arn(), backup_to_delete);
    assert_eq!(
        details.backup_status(),
        &aws_sdk_dynamodb::types::BackupStatus::Deleted
    );

    // Verify only 1 backup remains
    let resp = client
        .list_backups()
        .table_name(table_name)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.backup_summaries().len(), 1);
}

// Ported from moto: test_dynamodb.py::test_delete_non_existent_backup_raises_error
#[tokio::test]
async fn test_moto_delete_nonexistent_backup() {
    let client = make_dynamodb_client().await;
    let non_existent_arn =
        "arn:aws:dynamodb:us-east-1:123456789012:table/table-name/backup/01623095754481-2cfcd6f9";

    let err = client
        .delete_backup()
        .backup_arn(non_existent_arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("BackupNotFoundException"),
        "Expected BackupNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_dynamodb.py::test_restore_table_from_backup
#[tokio::test]
async fn test_moto_restore_table_from_backup() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-restore-backup";

    let create_resp = client
        .create_table()
        .table_name(table_name)
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let source_table_arn = create_resp
        .table_description()
        .unwrap()
        .table_arn()
        .unwrap()
        .to_string();

    // Put 5 items
    for i in 0..5 {
        client
            .put_item()
            .table_name(table_name)
            .item("id", AttributeValue::S(format!("item {i}")))
            .send()
            .await
            .unwrap();
    }

    // Create backup
    let backup_resp = client
        .create_backup()
        .table_name(table_name)
        .backup_name("backup")
        .send()
        .await
        .unwrap();
    let backup_arn = backup_resp
        .backup_details()
        .unwrap()
        .backup_arn()
        .to_string();

    // Restore
    let restored_table_name = "moto-restore-backup-restored";
    let restored = client
        .restore_table_from_backup()
        .target_table_name(restored_table_name)
        .backup_arn(&backup_arn)
        .send()
        .await
        .unwrap();

    let desc = restored.table_description().unwrap();
    assert_eq!(desc.table_name(), Some(restored_table_name));
    assert!(desc.table_arn().unwrap().contains(restored_table_name));
    assert_eq!(desc.item_count(), Some(5));

    // Check RestoreSummary
    let summary = desc.restore_summary().unwrap();
    assert_eq!(summary.source_backup_arn(), Some(backup_arn.as_str()));
    assert_eq!(summary.source_table_arn(), Some(source_table_arn.as_str()));
    // restore_date_time is always present (not Option)
    assert!(!summary.restore_in_progress());
}

// Ported from moto: test_dynamodb.py::test_restore_table_from_non_existent_backup_raises_error
#[tokio::test]
async fn test_moto_restore_from_nonexistent_backup() {
    let client = make_dynamodb_client().await;
    let non_existent_arn =
        "arn:aws:dynamodb:us-east-1:123456789012:table/table-name/backup/01623095754481-2cfcd6f9";

    let err = client
        .restore_table_from_backup()
        .target_table_name("from-backup")
        .backup_arn(non_existent_arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("BackupNotFoundException"),
        "Expected BackupNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_dynamodb.py::test_restore_table_from_backup_raises_error_when_table_already_exists
#[tokio::test]
async fn test_moto_restore_from_backup_table_exists() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-restore-exists";
    create_hash_table(&client, table_name).await;

    let backup_resp = client
        .create_backup()
        .table_name(table_name)
        .backup_name("backup")
        .send()
        .await
        .unwrap();
    let backup_arn = backup_resp
        .backup_details()
        .unwrap()
        .backup_arn()
        .to_string();

    let err = client
        .restore_table_from_backup()
        .target_table_name(table_name) // already exists
        .backup_arn(&backup_arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("TableAlreadyExistsException"),
        "Expected TableAlreadyExistsException, got: {err_str}"
    );
}

// Ported from moto: test_dynamodb.py::test_restore_table_to_point_in_time
#[tokio::test]
async fn test_moto_restore_table_to_point_in_time() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-pitr-restore";

    let create_resp = client
        .create_table()
        .table_name(table_name)
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let source_table_arn = create_resp
        .table_description()
        .unwrap()
        .table_arn()
        .unwrap()
        .to_string();

    // Put 5 items
    for i in 0..5 {
        client
            .put_item()
            .table_name(table_name)
            .item("id", AttributeValue::S(format!("item {i}")))
            .send()
            .await
            .unwrap();
    }

    // Restore
    let restored_table_name = "moto-pitr-restore-target";
    let restored = client
        .restore_table_to_point_in_time()
        .source_table_name(table_name)
        .target_table_name(restored_table_name)
        .send()
        .await
        .unwrap();

    let desc = restored.table_description().unwrap();
    assert_eq!(desc.table_name(), Some(restored_table_name));
    assert!(desc.table_arn().unwrap().contains(restored_table_name));
    assert_eq!(desc.item_count(), Some(5));

    // Check RestoreSummary
    let summary = desc.restore_summary().unwrap();
    assert_eq!(summary.source_table_arn(), Some(source_table_arn.as_str()));
    // restore_date_time is always present (not Option)
    assert!(!summary.restore_in_progress());
}

// Ported from moto: test_dynamodb.py::test_restore_table_to_point_in_time_raises_error_when_source_not_exist
#[tokio::test]
async fn test_moto_restore_pitr_source_not_found() {
    let client = make_dynamodb_client().await;

    let err = client
        .restore_table_to_point_in_time()
        .source_table_name("nonexistent-source")
        .target_table_name("target-table")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("SourceTableNotFoundException"),
        "Expected SourceTableNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_dynamodb.py::test_restore_table_to_point_in_time_raises_error_when_dest_exist
#[tokio::test]
async fn test_moto_restore_pitr_dest_exists() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-pitr-dest-exists-src";
    let restored_table_name = "moto-pitr-dest-exists-dst";
    create_hash_table(&client, table_name).await;
    create_hash_table(&client, restored_table_name).await;

    let err = client
        .restore_table_to_point_in_time()
        .source_table_name(table_name)
        .target_table_name(restored_table_name)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("TableAlreadyExistsException"),
        "Expected TableAlreadyExistsException, got: {err_str}"
    );
}

// Ported from moto: test_dynamodb.py::test_source_and_restored_table_items_are_not_linked
#[tokio::test]
async fn test_moto_source_and_restored_items_not_linked() {
    let client = make_dynamodb_client().await;
    let source_table = "moto-unlinked-src";
    create_hash_table(&client, source_table).await;

    // Add 5 items to source
    let mut original_ids = Vec::new();
    for i in 0..5 {
        let id = format!("orig-{i}");
        client
            .put_item()
            .table_name(source_table)
            .item("pk", AttributeValue::S(id.clone()))
            .send()
            .await
            .unwrap();
        original_ids.push(id);
    }

    // Create backup
    let backup_arn = client
        .create_backup()
        .table_name(source_table)
        .backup_name("backup")
        .send()
        .await
        .unwrap()
        .backup_details()
        .unwrap()
        .backup_arn()
        .to_string();

    // Add 5 more items to source after backup
    let mut post_backup_ids = Vec::new();
    for i in 0..5 {
        let id = format!("post-{i}");
        client
            .put_item()
            .table_name(source_table)
            .item("pk", AttributeValue::S(id.clone()))
            .send()
            .await
            .unwrap();
        post_backup_ids.push(id);
    }

    // Restore from backup
    let restored_table = "moto-unlinked-restored";
    client
        .restore_table_from_backup()
        .target_table_name(restored_table)
        .backup_arn(&backup_arn)
        .send()
        .await
        .unwrap();

    // Add 5 more items to restored table
    let mut post_restore_ids = Vec::new();
    for i in 0..5 {
        let id = format!("restore-{i}");
        client
            .put_item()
            .table_name(restored_table)
            .item("pk", AttributeValue::S(id.clone()))
            .send()
            .await
            .unwrap();
        post_restore_ids.push(id);
    }

    // Source should have original + post_backup = 10 items
    let source_scan = client.scan().table_name(source_table).send().await.unwrap();
    assert_eq!(source_scan.count(), 10);

    // Restored should have original + post_restore = 10 items
    let restored_scan = client
        .scan()
        .table_name(restored_table)
        .send()
        .await
        .unwrap();
    assert_eq!(restored_scan.count(), 10);

    // Verify source items are correct
    let source_ids: Vec<String> = source_scan
        .items()
        .iter()
        .map(|item| item.get("pk").unwrap().as_s().unwrap().to_string())
        .collect();
    for id in &original_ids {
        assert!(source_ids.contains(id));
    }
    for id in &post_backup_ids {
        assert!(source_ids.contains(id));
    }

    // Verify restored items are correct
    let restored_ids: Vec<String> = restored_scan
        .items()
        .iter()
        .map(|item| item.get("pk").unwrap().as_s().unwrap().to_string())
        .collect();
    for id in &original_ids {
        assert!(restored_ids.contains(id));
    }
    for id in &post_restore_ids {
        assert!(restored_ids.contains(id));
    }
}

// Ported from moto: test_dynamodb.py::test_describe_endpoints
#[tokio::test]
async fn test_moto_describe_endpoints() {
    let client = make_dynamodb_client().await;

    let resp = client.describe_endpoints().send().await.unwrap();
    let endpoints = resp.endpoints();
    assert_eq!(endpoints.len(), 1);
    assert_eq!(endpoints[0].address(), "dynamodb.us-east-1.amazonaws.com");
    assert_eq!(endpoints[0].cache_period_in_minutes(), 1440);
}

// Ported from moto: test_dynamodb_update_table.py::test_update_table_throughput
#[tokio::test]
async fn test_moto_update_table_throughput() {
    let client = make_dynamodb_client().await;
    let table_name = "moto-ut-throughput";

    client
        .create_table()
        .table_name(table_name)
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Update throughput
    client
        .update_table()
        .table_name(table_name)
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(6)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Verify
    let desc = client
        .describe_table()
        .table_name(table_name)
        .send()
        .await
        .unwrap();
    let pt = desc.table().unwrap().provisioned_throughput().unwrap();
    assert_eq!(pt.read_capacity_units(), Some(5));
    assert_eq!(pt.write_capacity_units(), Some(6));
}

// Ported from moto: test_dynamodb_transact.py::test_valid_transact_get_items (multi-table)
#[tokio::test]
async fn test_moto_transact_get_items_multi_table() {
    let client = make_dynamodb_client().await;

    // Create table1 with hash+range key
    let table1 = "moto-tgi-multi-t1";
    create_hash_range_table(&client, table1).await;
    client
        .put_item()
        .table_name(table1)
        .item("pk", AttributeValue::S("1".to_string()))
        .item("sk", AttributeValue::S("1".to_string()))
        .send()
        .await
        .unwrap();
    client
        .put_item()
        .table_name(table1)
        .item("pk", AttributeValue::S("1".to_string()))
        .item("sk", AttributeValue::S("2".to_string()))
        .send()
        .await
        .unwrap();

    // Create table2
    let table2 = "moto-tgi-multi-t2";
    create_hash_range_table(&client, table2).await;
    client
        .put_item()
        .table_name(table2)
        .item("pk", AttributeValue::S("1".to_string()))
        .item("sk", AttributeValue::S("1".to_string()))
        .send()
        .await
        .unwrap();

    // TransactGetItems across two tables, with one non-existent key
    let resp = client
        .transact_get_items()
        .transact_items(
            aws_sdk_dynamodb::types::TransactGetItem::builder()
                .get(
                    aws_sdk_dynamodb::types::Get::builder()
                        .table_name(table1)
                        .key("pk", AttributeValue::S("1".to_string()))
                        .key("sk", AttributeValue::S("1".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .transact_items(
            aws_sdk_dynamodb::types::TransactGetItem::builder()
                .get(
                    aws_sdk_dynamodb::types::Get::builder()
                        .table_name(table1)
                        .key("pk", AttributeValue::S("non_exists_key".to_string()))
                        .key("sk", AttributeValue::S("2".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let responses = resp.responses();
    assert_eq!(responses.len(), 2);
    // First item found
    let item0 = responses[0].item().unwrap();
    assert_eq!(item0.get("pk").unwrap().as_s().unwrap(), "1");
    assert_eq!(item0.get("sk").unwrap().as_s().unwrap(), "1");
    // Second item not found
    assert!(responses[1].item().is_none());

    // Get items from both tables
    let resp2 = client
        .transact_get_items()
        .transact_items(
            aws_sdk_dynamodb::types::TransactGetItem::builder()
                .get(
                    aws_sdk_dynamodb::types::Get::builder()
                        .table_name(table1)
                        .key("pk", AttributeValue::S("1".to_string()))
                        .key("sk", AttributeValue::S("1".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .transact_items(
            aws_sdk_dynamodb::types::TransactGetItem::builder()
                .get(
                    aws_sdk_dynamodb::types::Get::builder()
                        .table_name(table1)
                        .key("pk", AttributeValue::S("1".to_string()))
                        .key("sk", AttributeValue::S("2".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .transact_items(
            aws_sdk_dynamodb::types::TransactGetItem::builder()
                .get(
                    aws_sdk_dynamodb::types::Get::builder()
                        .table_name(table2)
                        .key("pk", AttributeValue::S("1".to_string()))
                        .key("sk", AttributeValue::S("1".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let responses2 = resp2.responses();
    assert_eq!(responses2.len(), 3);
    assert_eq!(
        responses2[0]
            .item()
            .unwrap()
            .get("sk")
            .unwrap()
            .as_s()
            .unwrap(),
        "1"
    );
    assert_eq!(
        responses2[1]
            .item()
            .unwrap()
            .get("sk")
            .unwrap()
            .as_s()
            .unwrap(),
        "2"
    );
    assert_eq!(
        responses2[2]
            .item()
            .unwrap()
            .get("pk")
            .unwrap()
            .as_s()
            .unwrap(),
        "1"
    );
}

// ============================================================================
// Tests derived from AWS documentation: DynamoDB
// Source: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/
// ============================================================================

// --- Test 1: UpdateItem ReturnValues=ALL_NEW ---
// AWS docs: ReturnValues=ALL_NEW returns the item as it appears after the update.
#[tokio::test]
async fn test_update_item_return_values_all_new() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "doc-ui-allnew").await;

    client
        .put_item()
        .table_name("doc-ui-allnew")
        .item("pk", AttributeValue::S("k1".into()))
        .item("status", AttributeValue::S("pending".into()))
        .send()
        .await
        .unwrap();

    let resp = client
        .update_item()
        .table_name("doc-ui-allnew")
        .key("pk", AttributeValue::S("k1".into()))
        .update_expression("SET #s = :v")
        .expression_attribute_names("#s", "status")
        .expression_attribute_values(":v", AttributeValue::S("active".into()))
        .return_values(aws_sdk_dynamodb::types::ReturnValue::AllNew)
        .send()
        .await
        .unwrap();

    let attrs = resp.attributes().expect("ALL_NEW should return attributes");
    assert_eq!(attrs.get("pk").unwrap().as_s().unwrap(), "k1");
    assert_eq!(attrs.get("status").unwrap().as_s().unwrap(), "active");
}

// --- Test 2: DeleteItem ReturnValues=ALL_OLD ---
// AWS docs: ReturnValues=ALL_OLD returns the item as it appeared before the delete.
#[tokio::test]
async fn test_delete_item_return_values_all_old() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "doc-di-allold").await;

    client
        .put_item()
        .table_name("doc-di-allold")
        .item("pk", AttributeValue::S("to-delete".into()))
        .item("value", AttributeValue::S("goodbye".into()))
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_item()
        .table_name("doc-di-allold")
        .key("pk", AttributeValue::S("to-delete".into()))
        .return_values(aws_sdk_dynamodb::types::ReturnValue::AllOld)
        .send()
        .await
        .unwrap();

    let attrs = resp
        .attributes()
        .expect("ALL_OLD should return deleted item");
    assert_eq!(attrs.get("pk").unwrap().as_s().unwrap(), "to-delete");
    assert_eq!(attrs.get("value").unwrap().as_s().unwrap(), "goodbye");

    // Verify the item is gone
    let get_resp = client
        .get_item()
        .table_name("doc-di-allold")
        .key("pk", AttributeValue::S("to-delete".into()))
        .send()
        .await
        .unwrap();
    assert!(get_resp.item().is_none(), "item should be deleted");
}

// --- Test 3: ExpressionAttributeNames (#placeholder for reserved words) ---
// AWS docs: ExpressionAttributeNames maps #name tokens to actual attribute names,
// allowing use of reserved words as attribute names.
#[tokio::test]
async fn test_expression_attribute_names_in_update() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "doc-ean-tbl").await;

    client
        .put_item()
        .table_name("doc-ean-tbl")
        .item("pk", AttributeValue::S("item1".into()))
        // "name" is a DynamoDB reserved word
        .item("name", AttributeValue::S("original".into()))
        .send()
        .await
        .unwrap();

    // Use #n to refer to the "name" attribute (reserved word)
    client
        .update_item()
        .table_name("doc-ean-tbl")
        .key("pk", AttributeValue::S("item1".into()))
        .update_expression("SET #n = :v")
        .expression_attribute_names("#n", "name")
        .expression_attribute_values(":v", AttributeValue::S("updated".into()))
        .send()
        .await
        .unwrap();

    let get_resp = client
        .get_item()
        .table_name("doc-ean-tbl")
        .key("pk", AttributeValue::S("item1".into()))
        .send()
        .await
        .unwrap();
    let item = get_resp.item().unwrap();
    assert_eq!(item.get("name").unwrap().as_s().unwrap(), "updated");
}

// --- Test 4: Query Count and ScannedCount ---
// AWS docs: Query response always includes Count (items returned) and ScannedCount
// (items evaluated before FilterExpression). Without a filter they are equal.
#[tokio::test]
async fn test_query_count_and_scanned_count() {
    let client = make_dynamodb_client().await;
    create_hash_range_table(&client, "doc-qcsc-tbl").await;

    for i in 0..4u8 {
        client
            .put_item()
            .table_name("doc-qcsc-tbl")
            .item("pk", AttributeValue::S("pk1".into()))
            .item("sk", AttributeValue::S(format!("sk{i}")))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .query()
        .table_name("doc-qcsc-tbl")
        .key_condition_expression("pk = :pk")
        .expression_attribute_values(":pk", AttributeValue::S("pk1".into()))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.count(), 4, "count should be 4");
    assert_eq!(
        resp.scanned_count(),
        4,
        "scanned_count should equal count when no filter"
    );
}

// --- Tests 5–10: Unimplemented features documented with #[ignore] ---

// --- Test 5: PutItem with ConditionExpression ---
// AWS docs: attribute_not_exists(pk) prevents overwriting an existing item.
// ConditionalCheckFailedException is returned when the condition is not met.
#[tokio::test]
async fn test_put_item_condition_attribute_not_exists() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "doc-pi-cond").await;

    // First put succeeds (item doesn't exist yet)
    client
        .put_item()
        .table_name("doc-pi-cond")
        .item("pk", AttributeValue::S("unique-key".into()))
        .item("data", AttributeValue::S("first".into()))
        .condition_expression("attribute_not_exists(pk)")
        .send()
        .await
        .expect("first put with attribute_not_exists should succeed");

    // Second put on same key should fail
    let err = client
        .put_item()
        .table_name("doc-pi-cond")
        .item("pk", AttributeValue::S("unique-key".into()))
        .item("data", AttributeValue::S("second".into()))
        .condition_expression("attribute_not_exists(pk)")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConditionalCheckFailedException"),
        "Expected ConditionalCheckFailedException, got: {err_str}"
    );
}

// --- Test 6: UpdateItem with ConditionExpression ---
// AWS docs: ConditionExpression can require an attribute to equal a specific value.
// If the condition fails, ConditionalCheckFailedException is returned.
#[tokio::test]
async fn test_update_item_condition_expression() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "doc-ui-cond").await;

    client
        .put_item()
        .table_name("doc-ui-cond")
        .item("pk", AttributeValue::S("k1".into()))
        .item("version", AttributeValue::N("1".into()))
        .send()
        .await
        .unwrap();

    // Update should fail because version is 1, not 99
    let err = client
        .update_item()
        .table_name("doc-ui-cond")
        .key("pk", AttributeValue::S("k1".into()))
        .update_expression("SET version = :newver")
        .condition_expression("version = :expected")
        .expression_attribute_values(":newver", AttributeValue::N("2".into()))
        .expression_attribute_values(":expected", AttributeValue::N("99".into()))
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConditionalCheckFailedException"),
        "Expected ConditionalCheckFailedException, got: {err_str}"
    );
}

// --- Test 7: Query with FilterExpression ---
// AWS docs: FilterExpression is applied after Query reads items matching KeyConditionExpression.
// ScannedCount reflects items read; Count reflects items after filtering.
#[tokio::test]
async fn test_query_filter_expression() {
    let client = make_dynamodb_client().await;
    create_hash_range_table(&client, "doc-qfe-tbl").await;

    // Put 3 items: two with type=A, one with type=B
    for (sk, typ) in [("sk1", "A"), ("sk2", "A"), ("sk3", "B")] {
        client
            .put_item()
            .table_name("doc-qfe-tbl")
            .item("pk", AttributeValue::S("pk1".into()))
            .item("sk", AttributeValue::S(sk.into()))
            .item("typ", AttributeValue::S(typ.into()))
            .send()
            .await
            .unwrap();
    }

    // Query with filter: only return items where typ = "A"
    let resp = client
        .query()
        .table_name("doc-qfe-tbl")
        .key_condition_expression("pk = :pk")
        .filter_expression("typ = :t")
        .expression_attribute_values(":pk", AttributeValue::S("pk1".into()))
        .expression_attribute_values(":t", AttributeValue::S("A".into()))
        .send()
        .await
        .unwrap();

    // ScannedCount = 3 (all items with pk=pk1 were read)
    assert_eq!(
        resp.scanned_count(),
        3,
        "scanned_count should be 3 (all read)"
    );
    // Count = 2 (only type=A items pass the filter)
    assert_eq!(resp.count(), 2, "count should be 2 after filter");
    for item in resp.items() {
        assert_eq!(item.get("typ").unwrap().as_s().unwrap(), "A");
    }
}

// --- Test 8: Scan with FilterExpression ---
// AWS docs: FilterExpression is applied after Scan reads items.
// ScannedCount = items evaluated; Count = items after filter.
#[tokio::test]
async fn test_scan_filter_expression() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "doc-sfe-tbl").await;

    for (pk, active) in [("a", "true"), ("b", "true"), ("c", "false")] {
        client
            .put_item()
            .table_name("doc-sfe-tbl")
            .item("pk", AttributeValue::S(pk.into()))
            .item("active", AttributeValue::S(active.into()))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .scan()
        .table_name("doc-sfe-tbl")
        .filter_expression("active = :v")
        .expression_attribute_values(":v", AttributeValue::S("true".into()))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.scanned_count(), 3, "all 3 items should be scanned");
    assert_eq!(resp.count(), 2, "only 2 active items should be returned");
    for item in resp.items() {
        assert_eq!(item.get("active").unwrap().as_s().unwrap(), "true");
    }
}

// --- Test 9: UpdateItem ADD numeric ---
// AWS docs: ADD <attr> <value> increments a numeric attribute by value.
// If the attribute does not exist, ADD creates it with the given value.
#[tokio::test]
async fn test_update_item_add_numeric() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "doc-ui-add").await;

    client
        .put_item()
        .table_name("doc-ui-add")
        .item("pk", AttributeValue::S("counter".into()))
        .item("count", AttributeValue::N("10".into()))
        .send()
        .await
        .unwrap();

    // ADD 5 to count
    client
        .update_item()
        .table_name("doc-ui-add")
        .key("pk", AttributeValue::S("counter".into()))
        .update_expression("ADD #c :delta")
        .expression_attribute_names("#c", "count")
        .expression_attribute_values(":delta", AttributeValue::N("5".into()))
        .send()
        .await
        .unwrap();

    let resp = client
        .get_item()
        .table_name("doc-ui-add")
        .key("pk", AttributeValue::S("counter".into()))
        .send()
        .await
        .unwrap();
    let count = resp.item().unwrap().get("count").unwrap().as_n().unwrap();
    assert_eq!(count, "15", "10 + 5 should equal 15");
}

// --- Test 10: ListTables pagination ---
// AWS docs: ListTables returns at most 100 tables per call.
// Use Limit and ExclusiveStartTableName to paginate.
#[tokio::test]
async fn test_list_tables_pagination() {
    let client = make_dynamodb_client().await;

    // Create 5 tables
    for i in 0..5u8 {
        create_hash_table(&client, &format!("doc-pag-tbl{i:02}")).await;
    }

    // First page: limit=2
    let page1 = client.list_tables().limit(2).send().await.unwrap();

    assert_eq!(page1.table_names().len(), 2);
    assert!(
        page1.last_evaluated_table_name().is_some(),
        "last_evaluated_table_name should be set when there are more tables"
    );

    // Second page
    let page2 = client
        .list_tables()
        .limit(2)
        .exclusive_start_table_name(page1.last_evaluated_table_name().unwrap())
        .send()
        .await
        .unwrap();

    assert_eq!(page2.table_names().len(), 2);
}

// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================

// Covers FIX(terraform-e2e): For PAY_PER_REQUEST tables, AWS still returns
// ProvisionedThroughput with zeros. The terraform provider expects this field.
#[tokio::test]
async fn test_fix_pay_per_request_provisioned_throughput_zeros() {
    let client = make_dynamodb_client().await;
    let table_name = "fix-ppr-pt-zeros";

    client
        .create_table()
        .table_name(table_name)
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("pk")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("pk")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_table()
        .table_name(table_name)
        .send()
        .await
        .unwrap();

    let table = desc.table().expect("should have table description");

    // PAY_PER_REQUEST tables must still include ProvisionedThroughput with zeros
    let pt = table
        .provisioned_throughput()
        .expect("ProvisionedThroughput must be present even for PAY_PER_REQUEST");
    assert_eq!(
        pt.read_capacity_units(),
        Some(0),
        "ReadCapacityUnits should be 0 for PAY_PER_REQUEST"
    );
    assert_eq!(
        pt.write_capacity_units(),
        Some(0),
        "WriteCapacityUnits should be 0 for PAY_PER_REQUEST"
    );
}

// Covers FIX(terraform-e2e): TableId must be deterministic — random UUID per call
// caused the terraform waiter to see a different response each time.
#[tokio::test]
async fn test_fix_table_id_deterministic() {
    let client = make_dynamodb_client().await;
    let table_name = "fix-table-id-det";

    create_hash_table(&client, table_name).await;

    let desc1 = client
        .describe_table()
        .table_name(table_name)
        .send()
        .await
        .unwrap();
    let id1 = desc1
        .table()
        .unwrap()
        .table_id()
        .expect("TableId must be present")
        .to_string();

    let desc2 = client
        .describe_table()
        .table_name(table_name)
        .send()
        .await
        .unwrap();
    let id2 = desc2
        .table()
        .unwrap()
        .table_id()
        .expect("TableId must be present")
        .to_string();

    assert_eq!(
        id1, id2,
        "TableId must be deterministic across multiple DescribeTable calls"
    );
    assert!(!id1.is_empty(), "TableId must not be empty");
}

// Covers FIX(terraform-e2e): DeletionProtectionEnabled field must be present
// in DescribeTable — terraform reads this field on every DescribeTable.
#[tokio::test]
async fn test_fix_deletion_protection_enabled_present() {
    let client = make_dynamodb_client().await;
    let table_name = "fix-del-prot";

    create_hash_table(&client, table_name).await;

    let desc = client
        .describe_table()
        .table_name(table_name)
        .send()
        .await
        .unwrap();

    let table = desc.table().expect("should have table description");

    // DeletionProtectionEnabled must be present (defaults to false)
    assert_eq!(
        table.deletion_protection_enabled(),
        Some(false),
        "DeletionProtectionEnabled must be present and default to false"
    );
}

// Covers FIX(terraform-e2e): newer terraform-provider-aws (v6.13+) waiters poll
// WarmThroughput.Status and treat None/absent as '' which causes
// "unexpected state '', wanted target 'ACTIVE'".
#[tokio::test]
async fn test_fix_warm_throughput_present() {
    let client = make_dynamodb_client().await;
    let table_name = "fix-warm-throughput";

    create_hash_table(&client, table_name).await;

    let desc = client
        .describe_table()
        .table_name(table_name)
        .send()
        .await
        .unwrap();

    let table = desc.table().expect("should have table description");

    // WarmThroughput must be present with status ACTIVE
    let wt = table
        .warm_throughput()
        .expect("WarmThroughput must be present in DescribeTable response");
    assert_eq!(
        wt.status().map(|s| s.as_str()),
        Some("ACTIVE"),
        "WarmThroughput.Status must be ACTIVE for an active table"
    );
}

// ===========================================================================
// Global Tables (v1 API)
// ===========================================================================

#[tokio::test]
async fn test_create_and_describe_global_table() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "GlobalTest").await;

    let result = client
        .create_global_table()
        .global_table_name("GlobalTest")
        .replication_group(
            aws_sdk_dynamodb::types::Replica::builder()
                .region_name("us-east-1")
                .build(),
        )
        .replication_group(
            aws_sdk_dynamodb::types::Replica::builder()
                .region_name("eu-west-1")
                .build(),
        )
        .send()
        .await
        .expect("create_global_table should succeed");

    let desc = result.global_table_description().unwrap();
    assert_eq!(desc.global_table_name(), Some("GlobalTest"));
    assert_eq!(
        desc.global_table_status(),
        Some(&aws_sdk_dynamodb::types::GlobalTableStatus::Active)
    );
    assert_eq!(desc.replication_group().len(), 2);

    // Describe
    let describe = client
        .describe_global_table()
        .global_table_name("GlobalTest")
        .send()
        .await
        .expect("describe_global_table should succeed");

    let desc = describe.global_table_description().unwrap();
    assert_eq!(desc.global_table_name(), Some("GlobalTest"));
}

#[tokio::test]
async fn test_update_global_table() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "GlobalUpdate").await;

    client
        .create_global_table()
        .global_table_name("GlobalUpdate")
        .replication_group(
            aws_sdk_dynamodb::types::Replica::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .update_global_table()
        .global_table_name("GlobalUpdate")
        .replica_updates(
            aws_sdk_dynamodb::types::ReplicaUpdate::builder()
                .create(
                    aws_sdk_dynamodb::types::CreateReplicaAction::builder()
                        .region_name("ap-southeast-1")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("update_global_table should succeed");

    let desc = result.global_table_description().unwrap();
    assert_eq!(desc.replication_group().len(), 2);
}

#[tokio::test]
async fn test_list_global_tables() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "GlobalList").await;

    client
        .create_global_table()
        .global_table_name("GlobalList")
        .replication_group(
            aws_sdk_dynamodb::types::Replica::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .list_global_tables()
        .send()
        .await
        .expect("list_global_tables should succeed");

    assert!(!result.global_tables().is_empty());
}

#[tokio::test]
async fn test_describe_global_table_settings() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "GlobalSettings").await;

    client
        .create_global_table()
        .global_table_name("GlobalSettings")
        .replication_group(
            aws_sdk_dynamodb::types::Replica::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .describe_global_table_settings()
        .global_table_name("GlobalSettings")
        .send()
        .await
        .expect("describe_global_table_settings should succeed");

    assert_eq!(result.global_table_name(), Some("GlobalSettings"));
    assert!(!result.replica_settings().is_empty());
}

#[tokio::test]
async fn test_update_global_table_settings() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "GlobalSettingsUpdate").await;

    client
        .create_global_table()
        .global_table_name("GlobalSettingsUpdate")
        .replication_group(
            aws_sdk_dynamodb::types::Replica::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .update_global_table_settings()
        .global_table_name("GlobalSettingsUpdate")
        .send()
        .await
        .expect("update_global_table_settings should succeed");

    assert_eq!(result.global_table_name(), Some("GlobalSettingsUpdate"));
}

#[tokio::test]
async fn test_global_table_not_found() {
    let client = make_dynamodb_client().await;

    let err = client
        .describe_global_table()
        .global_table_name("NonExistent")
        .send()
        .await;

    assert!(err.is_err());
}

// ===========================================================================
// Kinesis Streaming Destinations
// ===========================================================================

#[tokio::test]
async fn test_enable_and_describe_kinesis_streaming() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "KinesisTable").await;

    let stream_arn = "arn:aws:kinesis:us-east-1:123456789012:stream/test-stream";

    let result = client
        .enable_kinesis_streaming_destination()
        .table_name("KinesisTable")
        .stream_arn(stream_arn)
        .send()
        .await
        .expect("enable_kinesis_streaming_destination should succeed");

    assert_eq!(result.table_name(), Some("KinesisTable"));
    assert_eq!(result.stream_arn(), Some(stream_arn));
    assert_eq!(
        result.destination_status(),
        Some(&aws_sdk_dynamodb::types::DestinationStatus::Active)
    );

    // Describe
    let desc = client
        .describe_kinesis_streaming_destination()
        .table_name("KinesisTable")
        .send()
        .await
        .expect("describe should succeed");

    assert_eq!(desc.table_name(), Some("KinesisTable"));
    assert_eq!(desc.kinesis_data_stream_destinations().len(), 1);
    assert_eq!(
        desc.kinesis_data_stream_destinations()[0].stream_arn(),
        Some(stream_arn)
    );
}

#[tokio::test]
async fn test_disable_kinesis_streaming() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "KinesisDisable").await;

    let stream_arn = "arn:aws:kinesis:us-east-1:123456789012:stream/test-stream2";

    client
        .enable_kinesis_streaming_destination()
        .table_name("KinesisDisable")
        .stream_arn(stream_arn)
        .send()
        .await
        .unwrap();

    let result = client
        .disable_kinesis_streaming_destination()
        .table_name("KinesisDisable")
        .stream_arn(stream_arn)
        .send()
        .await
        .expect("disable should succeed");

    assert_eq!(
        result.destination_status(),
        Some(&aws_sdk_dynamodb::types::DestinationStatus::Disabled)
    );
}

#[tokio::test]
async fn test_update_kinesis_streaming() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "KinesisUpdate").await;

    let stream_arn = "arn:aws:kinesis:us-east-1:123456789012:stream/test-stream3";

    client
        .enable_kinesis_streaming_destination()
        .table_name("KinesisUpdate")
        .stream_arn(stream_arn)
        .send()
        .await
        .unwrap();

    let result = client
        .update_kinesis_streaming_destination()
        .table_name("KinesisUpdate")
        .stream_arn(stream_arn)
        .update_kinesis_streaming_configuration(
            aws_sdk_dynamodb::types::UpdateKinesisStreamingConfiguration::builder()
                .approximate_creation_date_time_precision(
                    aws_sdk_dynamodb::types::ApproximateCreationDateTimePrecision::Millisecond,
                )
                .build(),
        )
        .send()
        .await
        .expect("update should succeed");

    assert_eq!(result.table_name(), Some("KinesisUpdate"));
    assert_eq!(result.stream_arn(), Some(stream_arn));
}

// ===========================================================================
// Contributor Insights
// ===========================================================================

#[tokio::test]
async fn test_update_and_describe_contributor_insights() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "ContribTable").await;

    let update = client
        .update_contributor_insights()
        .table_name("ContribTable")
        .contributor_insights_action(aws_sdk_dynamodb::types::ContributorInsightsAction::Enable)
        .send()
        .await
        .expect("update_contributor_insights should succeed");

    assert_eq!(update.table_name(), Some("ContribTable"));
    assert_eq!(
        update.contributor_insights_status(),
        Some(&aws_sdk_dynamodb::types::ContributorInsightsStatus::Enabled)
    );

    // Describe
    let desc = client
        .describe_contributor_insights()
        .table_name("ContribTable")
        .send()
        .await
        .expect("describe should succeed");

    assert_eq!(desc.table_name(), Some("ContribTable"));
    assert_eq!(
        desc.contributor_insights_status(),
        Some(&aws_sdk_dynamodb::types::ContributorInsightsStatus::Enabled)
    );
}

#[tokio::test]
async fn test_list_contributor_insights() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "ContribList").await;

    client
        .update_contributor_insights()
        .table_name("ContribList")
        .contributor_insights_action(aws_sdk_dynamodb::types::ContributorInsightsAction::Enable)
        .send()
        .await
        .unwrap();

    let result = client
        .list_contributor_insights()
        .table_name("ContribList")
        .send()
        .await
        .expect("list should succeed");

    assert!(!result.contributor_insights_summaries().is_empty());
    assert_eq!(
        result.contributor_insights_summaries()[0].table_name(),
        Some("ContribList")
    );
}

#[tokio::test]
async fn test_disable_contributor_insights() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "ContribDisable").await;

    client
        .update_contributor_insights()
        .table_name("ContribDisable")
        .contributor_insights_action(aws_sdk_dynamodb::types::ContributorInsightsAction::Enable)
        .send()
        .await
        .unwrap();

    let update = client
        .update_contributor_insights()
        .table_name("ContribDisable")
        .contributor_insights_action(aws_sdk_dynamodb::types::ContributorInsightsAction::Disable)
        .send()
        .await
        .expect("disable should succeed");

    assert_eq!(
        update.contributor_insights_status(),
        Some(&aws_sdk_dynamodb::types::ContributorInsightsStatus::Disabled)
    );
}

// ===========================================================================
// DescribeLimits
// ===========================================================================

#[tokio::test]
async fn test_describe_limits() {
    let client = make_dynamodb_client().await;

    let result = client
        .describe_limits()
        .send()
        .await
        .expect("describe_limits should succeed");

    assert_eq!(result.account_max_read_capacity_units(), Some(80000));
    assert_eq!(result.account_max_write_capacity_units(), Some(80000));
    assert_eq!(result.table_max_read_capacity_units(), Some(40000));
    assert_eq!(result.table_max_write_capacity_units(), Some(40000));
}

// ===========================================================================
// ListImports
// ===========================================================================

#[tokio::test]
async fn test_list_imports() {
    let client = make_dynamodb_client().await;

    let result = client
        .list_imports()
        .send()
        .await
        .expect("list_imports should succeed");

    assert!(result.import_summary_list().is_empty());
}

// ===========================================================================
// ExportTableToPointInTime
// ===========================================================================

#[tokio::test]
async fn test_export_table_to_point_in_time() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "ExportTable").await;

    let table_arn = "arn:aws:dynamodb:us-east-1:123456789012:table/ExportTable";

    let result = client
        .export_table_to_point_in_time()
        .table_arn(table_arn)
        .s3_bucket("my-export-bucket")
        .s3_prefix("exports/")
        .send()
        .await
        .expect("export should succeed");

    let desc = result.export_description().unwrap();
    assert!(desc.export_arn().is_some());
    assert_eq!(desc.table_arn(), Some(table_arn));
    assert_eq!(
        desc.export_status(),
        Some(&aws_sdk_dynamodb::types::ExportStatus::InProgress)
    );
}

// ===========================================================================
// DescribeTableReplicaAutoScaling / UpdateTableReplicaAutoScaling
// ===========================================================================

#[tokio::test]
async fn test_describe_table_replica_auto_scaling() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "AutoScaleTable").await;

    let result = client
        .describe_table_replica_auto_scaling()
        .table_name("AutoScaleTable")
        .send()
        .await
        .expect("describe should succeed");

    let desc = result.table_auto_scaling_description().unwrap();
    assert_eq!(desc.table_name(), Some("AutoScaleTable"));
    assert_eq!(
        desc.table_status(),
        Some(&aws_sdk_dynamodb::types::TableStatus::Active)
    );
}

#[tokio::test]
async fn test_update_table_replica_auto_scaling() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "AutoScaleUpdate").await;

    let result = client
        .update_table_replica_auto_scaling()
        .table_name("AutoScaleUpdate")
        .send()
        .await
        .expect("update should succeed");

    let desc = result.table_auto_scaling_description().unwrap();
    assert_eq!(desc.table_name(), Some("AutoScaleUpdate"));
}

#[tokio::test]
async fn test_describe_table_replica_auto_scaling_not_found() {
    let client = make_dynamodb_client().await;

    let err = client
        .describe_table_replica_auto_scaling()
        .table_name("NonExistentTable")
        .send()
        .await;

    assert!(err.is_err());
}

// ─── PartiQL: Arithmetic, list_append, set_add, RETURNING, ORDER BY ──────────

#[tokio::test]
async fn test_partiql_update_arithmetic_add() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-arith-add").await;

    client
        .put_item()
        .table_name("pq-arith-add")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .item(
            "counter",
            aws_sdk_dynamodb::types::AttributeValue::N("10".into()),
        )
        .send()
        .await
        .unwrap();

    // Increment counter by 5
    client
        .execute_statement()
        .statement("UPDATE \"pq-arith-add\" SET counter = counter + 5 WHERE pk = 'k1'")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_item()
        .table_name("pq-arith-add")
        .key(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .send()
        .await
        .unwrap();
    let item = resp.item().unwrap();
    assert_eq!(item.get("counter").unwrap().as_n().unwrap(), "15");
}

#[tokio::test]
async fn test_partiql_update_arithmetic_sub() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-arith-sub").await;

    client
        .put_item()
        .table_name("pq-arith-sub")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .item(
            "score",
            aws_sdk_dynamodb::types::AttributeValue::N("100".into()),
        )
        .send()
        .await
        .unwrap();

    client
        .execute_statement()
        .statement("UPDATE \"pq-arith-sub\" SET score = score - 3 WHERE pk = 'k1'")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_item()
        .table_name("pq-arith-sub")
        .key(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .send()
        .await
        .unwrap();
    let item = resp.item().unwrap();
    assert_eq!(item.get("score").unwrap().as_n().unwrap(), "97");
}

#[tokio::test]
async fn test_partiql_update_list_append() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-list-append").await;

    client
        .put_item()
        .table_name("pq-list-append")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .item(
            "tags",
            aws_sdk_dynamodb::types::AttributeValue::L(vec![
                aws_sdk_dynamodb::types::AttributeValue::S("alpha".into()),
            ]),
        )
        .send()
        .await
        .unwrap();

    client
        .execute_statement()
        .statement(
            "UPDATE \"pq-list-append\" SET tags = list_append(tags, ['beta']) WHERE pk = 'k1'",
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .get_item()
        .table_name("pq-list-append")
        .key(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .send()
        .await
        .unwrap();
    let item = resp.item().unwrap();
    let tags = item.get("tags").unwrap().as_l().unwrap();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].as_s().unwrap(), "alpha");
    assert_eq!(tags[1].as_s().unwrap(), "beta");
}

#[tokio::test]
async fn test_partiql_update_set_add() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-set-add").await;

    client
        .put_item()
        .table_name("pq-set-add")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .item(
            "colors",
            aws_sdk_dynamodb::types::AttributeValue::Ss(vec!["red".into(), "blue".into()]),
        )
        .send()
        .await
        .unwrap();

    client
        .execute_statement()
        .statement(
            "UPDATE \"pq-set-add\" SET colors = set_add(colors, <<'green'>>) WHERE pk = 'k1'",
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .get_item()
        .table_name("pq-set-add")
        .key(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .send()
        .await
        .unwrap();
    let item = resp.item().unwrap();
    let mut colors = item.get("colors").unwrap().as_ss().unwrap().to_vec();
    colors.sort();
    assert_eq!(colors, vec!["blue", "green", "red"]);
}

#[tokio::test]
async fn test_partiql_update_set_delete() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-set-del").await;

    client
        .put_item()
        .table_name("pq-set-del")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .item(
            "colors",
            aws_sdk_dynamodb::types::AttributeValue::Ss(vec![
                "red".into(),
                "blue".into(),
                "green".into(),
            ]),
        )
        .send()
        .await
        .unwrap();

    client
        .execute_statement()
        .statement(
            "UPDATE \"pq-set-del\" SET colors = set_delete(colors, <<'blue'>>) WHERE pk = 'k1'",
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .get_item()
        .table_name("pq-set-del")
        .key(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .send()
        .await
        .unwrap();
    let item = resp.item().unwrap();
    let mut colors = item.get("colors").unwrap().as_ss().unwrap().to_vec();
    colors.sort();
    assert_eq!(colors, vec!["green", "red"]);
}

#[tokio::test]
async fn test_partiql_update_returning_all_new() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-ret-new").await;

    client
        .put_item()
        .table_name("pq-ret-new")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .item(
            "val",
            aws_sdk_dynamodb::types::AttributeValue::N("1".into()),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .execute_statement()
        .statement("UPDATE \"pq-ret-new\" SET val = 99 WHERE pk = 'k1' RETURNING ALL NEW *")
        .send()
        .await
        .unwrap();

    let items = resp.items();
    assert_eq!(items.len(), 1, "RETURNING ALL NEW should return one item");
    assert_eq!(items[0].get("val").unwrap().as_n().unwrap(), "99");
}

#[tokio::test]
async fn test_partiql_update_returning_all_old() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-ret-old").await;

    client
        .put_item()
        .table_name("pq-ret-old")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .item(
            "val",
            aws_sdk_dynamodb::types::AttributeValue::N("42".into()),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .execute_statement()
        .statement("UPDATE \"pq-ret-old\" SET val = 99 WHERE pk = 'k1' RETURNING ALL OLD *")
        .send()
        .await
        .unwrap();

    let items = resp.items();
    assert_eq!(items.len(), 1, "RETURNING ALL OLD should return one item");
    // The old value before the update
    assert_eq!(items[0].get("val").unwrap().as_n().unwrap(), "42");
}

#[tokio::test]
async fn test_partiql_delete_returning_all_old() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-del-ret").await;

    client
        .put_item()
        .table_name("pq-del-ret")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .item(
            "data",
            aws_sdk_dynamodb::types::AttributeValue::S("important".into()),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .execute_statement()
        .statement("DELETE FROM \"pq-del-ret\" WHERE pk = 'k1' RETURNING ALL OLD *")
        .send()
        .await
        .unwrap();

    let items = resp.items();
    assert_eq!(
        items.len(),
        1,
        "RETURNING ALL OLD should return the deleted item"
    );
    assert_eq!(items[0].get("data").unwrap().as_s().unwrap(), "important");

    // Confirm the item was actually deleted
    let resp2 = client
        .get_item()
        .table_name("pq-del-ret")
        .key(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .send()
        .await
        .unwrap();
    assert!(resp2.item().is_none());
}

#[tokio::test]
async fn test_partiql_select_order_by_scan() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-order-scan").await;

    for (pk, score) in [("c", "30"), ("a", "10"), ("b", "20")] {
        client
            .put_item()
            .table_name("pq-order-scan")
            .item("pk", aws_sdk_dynamodb::types::AttributeValue::S(pk.into()))
            .item(
                "score",
                aws_sdk_dynamodb::types::AttributeValue::N(score.into()),
            )
            .send()
            .await
            .unwrap();
    }

    // ORDER BY score ASC
    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-order-scan\" ORDER BY score")
        .send()
        .await
        .unwrap();
    let items = resp.items();
    assert_eq!(items.len(), 3);
    let scores: Vec<String> = items
        .iter()
        .map(|i| i.get("score").unwrap().as_n().unwrap().clone())
        .collect();
    assert_eq!(scores, vec!["10", "20", "30"]);

    // ORDER BY score DESC
    let resp2 = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-order-scan\" ORDER BY score DESC")
        .send()
        .await
        .unwrap();
    let items2 = resp2.items();
    let scores2: Vec<String> = items2
        .iter()
        .map(|i| i.get("score").unwrap().as_n().unwrap().clone())
        .collect();
    assert_eq!(scores2, vec!["30", "20", "10"]);
}

#[tokio::test]
async fn test_partiql_batch_select_requires_partition_key() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-batch-val").await;

    client
        .put_item()
        .table_name("pq-batch-val")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .send()
        .await
        .unwrap();

    // SELECT without WHERE — should produce a per-statement error
    let resp = client
        .batch_execute_statement()
        .statements(
            aws_sdk_dynamodb::types::BatchStatementRequest::builder()
                .statement("SELECT * FROM \"pq-batch-val\"")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let responses = resp.responses();
    assert_eq!(responses.len(), 1);
    let err = responses[0].error().unwrap();
    let code = format!("{:?}", err.code());
    assert!(
        code.contains("ValidationException") || code.contains("ValidationError"),
        "expected ValidationException error code, got: {code}"
    );
}

// ---------------------------------------------------------------------------
// GSI / LSI tests
// ---------------------------------------------------------------------------

/// CreateTable with a GSI, then DescribeTable returns the GSI description.
#[tokio::test]
async fn test_create_table_with_gsi_describe_returns_gsi() {
    use aws_sdk_dynamodb::types::{GlobalSecondaryIndex, Projection, ProjectionType};

    let client = make_dynamodb_client().await;

    client
        .create_table()
        .table_name("gsi-desc-tbl")
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
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("gsi_pk")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("gsi_sk")
                .attribute_type(ScalarAttributeType::N)
                .build()
                .unwrap(),
        )
        .global_secondary_indexes(
            GlobalSecondaryIndex::builder()
                .index_name("my-gsi")
                .key_schema(
                    KeySchemaElement::builder()
                        .attribute_name("gsi_pk")
                        .key_type(KeyType::Hash)
                        .build()
                        .unwrap(),
                )
                .key_schema(
                    KeySchemaElement::builder()
                        .attribute_name("gsi_sk")
                        .key_type(KeyType::Range)
                        .build()
                        .unwrap(),
                )
                .projection(
                    Projection::builder()
                        .projection_type(ProjectionType::All)
                        .build(),
                )
                .provisioned_throughput(
                    ProvisionedThroughput::builder()
                        .read_capacity_units(5)
                        .write_capacity_units(5)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_table with GSI");

    let desc = client
        .describe_table()
        .table_name("gsi-desc-tbl")
        .send()
        .await
        .expect("describe_table");
    let table = desc.table().unwrap();
    let gsis = table.global_secondary_indexes();
    assert_eq!(gsis.len(), 1, "should have one GSI");
    assert_eq!(gsis[0].index_name(), Some("my-gsi"));
    assert_eq!(gsis[0].index_status().unwrap().as_str(), "ACTIVE");
    let ks = gsis[0].key_schema();
    assert_eq!(ks.len(), 2);
    assert!(
        gsis[0].index_arn().unwrap().contains("/index/my-gsi"),
        "GSI ARN should contain index name"
    );
}

/// Query a non-existent index name returns an error.
#[tokio::test]
async fn test_query_nonexistent_index_returns_error() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "no-idx-tbl").await;

    let result = client
        .query()
        .table_name("no-idx-tbl")
        .index_name("does-not-exist")
        .key_condition_expression("pk = :pk")
        .expression_attribute_values(":pk", AttributeValue::S("x".into()))
        .send()
        .await;
    assert!(result.is_err(), "querying a non-existent index should fail");
}

// ---------------------------------------------------------------------------
// SET arithmetic edge cases
// ---------------------------------------------------------------------------

/// SET arithmetic with expression attribute names (e.g. `SET #qty = #qty - :dec`).
#[tokio::test]
async fn test_update_item_set_arithmetic_with_expr_names() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "arith-names-tbl").await;

    client
        .put_item()
        .table_name("arith-names-tbl")
        .item("pk", AttributeValue::S("a".into()))
        .item("stock", AttributeValue::N("20".into()))
        .send()
        .await
        .unwrap();

    client
        .update_item()
        .table_name("arith-names-tbl")
        .key("pk", AttributeValue::S("a".into()))
        .update_expression("SET #s = #s - :dec")
        .expression_attribute_names("#s", "stock")
        .expression_attribute_values(":dec", AttributeValue::N("7".into()))
        .send()
        .await
        .unwrap();

    let resp = client
        .get_item()
        .table_name("arith-names-tbl")
        .key("pk", AttributeValue::S("a".into()))
        .send()
        .await
        .unwrap();
    let item = resp.item().unwrap();
    assert_eq!(item.get("stock").unwrap().as_n().unwrap(), "13");
}

// ---------------------------------------------------------------------------
// TransactWriteItems — ConditionExpression coverage
// ---------------------------------------------------------------------------

/// TransactWriteItems with ConditionCheck sub-item type.
#[tokio::test]
async fn test_transact_write_condition_check_sub_item() {
    use aws_sdk_dynamodb::types::{ConditionCheck, TransactWriteItem};

    let client = make_dynamodb_client().await;
    create_hash_table(&client, "txn-cc-tbl").await;

    client
        .put_item()
        .table_name("txn-cc-tbl")
        .item("pk", AttributeValue::S("item1".into()))
        .item("status", AttributeValue::S("active".into()))
        .send()
        .await
        .unwrap();

    // ConditionCheck passes — transaction succeeds and the Put lands.
    client
        .transact_write_items()
        .transact_items(
            TransactWriteItem::builder()
                .condition_check(
                    ConditionCheck::builder()
                        .table_name("txn-cc-tbl")
                        .key("pk", AttributeValue::S("item1".into()))
                        .condition_expression("#s = :expected")
                        .expression_attribute_names("#s", "status")
                        .expression_attribute_values(
                            ":expected",
                            AttributeValue::S("active".into()),
                        )
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .transact_items(
            TransactWriteItem::builder()
                .put(
                    aws_sdk_dynamodb::types::Put::builder()
                        .table_name("txn-cc-tbl")
                        .item("pk", AttributeValue::S("item2".into()))
                        .item("status", AttributeValue::S("new".into()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("transact with passing ConditionCheck should succeed");

    let item2 = client
        .get_item()
        .table_name("txn-cc-tbl")
        .key("pk", AttributeValue::S("item2".into()))
        .send()
        .await
        .unwrap();
    assert!(
        item2.item().is_some(),
        "item2 should exist after passing ConditionCheck"
    );

    // ConditionCheck fails — transaction is cancelled, Put does not land.
    let fail_result = client
        .transact_write_items()
        .transact_items(
            TransactWriteItem::builder()
                .condition_check(
                    ConditionCheck::builder()
                        .table_name("txn-cc-tbl")
                        .key("pk", AttributeValue::S("item1".into()))
                        .condition_expression("#s = :wrong")
                        .expression_attribute_names("#s", "status")
                        .expression_attribute_values(":wrong", AttributeValue::S("deleted".into()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .transact_items(
            TransactWriteItem::builder()
                .put(
                    aws_sdk_dynamodb::types::Put::builder()
                        .table_name("txn-cc-tbl")
                        .item("pk", AttributeValue::S("item3".into()))
                        .item("status", AttributeValue::S("should-not-exist".into()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await;
    assert!(
        fail_result.is_err(),
        "transact with failing ConditionCheck should be cancelled"
    );

    let item3 = client
        .get_item()
        .table_name("txn-cc-tbl")
        .key("pk", AttributeValue::S("item3".into()))
        .send()
        .await
        .unwrap();
    assert!(
        item3.item().is_none(),
        "item3 should not exist after cancelled transaction"
    );
}

/// TransactWriteItems with ConditionExpression on a Put sub-item.
#[tokio::test]
async fn test_transact_write_put_with_condition_expression() {
    use aws_sdk_dynamodb::types::TransactWriteItem;

    let client = make_dynamodb_client().await;
    create_hash_table(&client, "txn-put-cond-tbl").await;

    // Seed an existing item.
    client
        .put_item()
        .table_name("txn-put-cond-tbl")
        .item("pk", AttributeValue::S("existing".into()))
        .item("version", AttributeValue::N("1".into()))
        .send()
        .await
        .unwrap();

    // Put with condition "attribute_not_exists(pk)" — should fail because item exists.
    let result = client
        .transact_write_items()
        .transact_items(
            TransactWriteItem::builder()
                .put(
                    aws_sdk_dynamodb::types::Put::builder()
                        .table_name("txn-put-cond-tbl")
                        .item("pk", AttributeValue::S("existing".into()))
                        .item("version", AttributeValue::N("2".into()))
                        .condition_expression("attribute_not_exists(pk)")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await;
    assert!(
        result.is_err(),
        "Put with failing condition should cancel the transaction"
    );

    // The existing item should be unchanged.
    let item = client
        .get_item()
        .table_name("txn-put-cond-tbl")
        .key("pk", AttributeValue::S("existing".into()))
        .send()
        .await
        .unwrap();
    assert_eq!(
        item.item().unwrap().get("version").unwrap().as_n().unwrap(),
        "1",
        "version should be unchanged after cancelled Put"
    );

    // Put with condition "attribute_not_exists(pk)" on a new key — should succeed.
    client
        .transact_write_items()
        .transact_items(
            TransactWriteItem::builder()
                .put(
                    aws_sdk_dynamodb::types::Put::builder()
                        .table_name("txn-put-cond-tbl")
                        .item("pk", AttributeValue::S("new-item".into()))
                        .item("version", AttributeValue::N("1".into()))
                        .condition_expression("attribute_not_exists(pk)")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("Put with passing condition should succeed");

    let new_item = client
        .get_item()
        .table_name("txn-put-cond-tbl")
        .key("pk", AttributeValue::S("new-item".into()))
        .send()
        .await
        .unwrap();
    assert!(new_item.item().is_some(), "new-item should exist");
}

// ---------------------------------------------------------------------------
// GSI / LSI state view round-trip
// ---------------------------------------------------------------------------

/// Verify that SecondaryIndexView survives a serde round-trip with all fields intact.
#[tokio::test]
async fn test_secondary_index_view_serde_round_trip() {
    use winterbaume_dynamodb::views::{KeySchemaElementView, SecondaryIndexView};

    let idx_view = SecondaryIndexView {
        index_name: "my-gsi".to_string(),
        key_schema: vec![
            KeySchemaElementView {
                attribute_name: "gsi_pk".to_string(),
                key_type: "HASH".to_string(),
            },
            KeySchemaElementView {
                attribute_name: "gsi_sk".to_string(),
                key_type: "RANGE".to_string(),
            },
        ],
        projection_type: "INCLUDE".to_string(),
        non_key_attributes: vec!["extra_a".to_string(), "extra_b".to_string()],
    };

    let json = serde_json::to_string(&idx_view).expect("serialize");
    let restored: SecondaryIndexView = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(restored.index_name, "my-gsi");
    assert_eq!(restored.key_schema.len(), 2);
    assert_eq!(restored.key_schema[0].attribute_name, "gsi_pk");
    assert_eq!(restored.key_schema[0].key_type, "HASH");
    assert_eq!(restored.key_schema[1].attribute_name, "gsi_sk");
    assert_eq!(restored.key_schema[1].key_type, "RANGE");
    assert_eq!(restored.projection_type, "INCLUDE");
    assert_eq!(restored.non_key_attributes, vec!["extra_a", "extra_b"]);
}

/// Verify that a DynamodbStateView with GSI definitions survives serde round-trip
/// and that restoring it into a fresh service preserves the GSI for queries.
#[tokio::test]
async fn test_gsi_survives_snapshot_restore() {
    use winterbaume_core::StatefulService;
    use winterbaume_dynamodb::views::{
        AttributeDefinitionView, DynamodbStateView, KeySchemaElementView,
        ProvisionedThroughputView, SecondaryIndexView, TableStateView,
    };

    // Build a TableStateView with a GSI and one item, restore it, then query.
    let view = DynamodbStateView {
        tables: [(
            "snap-gsi-tbl".to_string(),
            TableStateView {
                name: "snap-gsi-tbl".to_string(),
                arn: "arn:aws:dynamodb:us-east-1:123456789012:table/snap-gsi-tbl".to_string(),
                key_schema: vec![KeySchemaElementView {
                    attribute_name: "pk".to_string(),
                    key_type: "HASH".to_string(),
                }],
                attribute_definitions: vec![
                    AttributeDefinitionView {
                        attribute_name: "pk".to_string(),
                        attribute_type: "S".to_string(),
                    },
                    AttributeDefinitionView {
                        attribute_name: "alt_key".to_string(),
                        attribute_type: "S".to_string(),
                    },
                ],
                billing_mode: "PROVISIONED".to_string(),
                provisioned_throughput: Some(ProvisionedThroughputView {
                    read_capacity_units: 5,
                    write_capacity_units: 5,
                }),
                creation_date_time: "2026-01-01T00:00:00+00:00".to_string(),
                table_status: "ACTIVE".to_string(),
                hash_key_attr: "pk".to_string(),
                hash_key_type: "S".to_string(),
                range_key_attr: None,
                range_key_type: None,
                items: {
                    use winterbaume_dynamodb::types::AttributeValue as AV;
                    let mut item = winterbaume_dynamodb::types::Item::new();
                    item.insert("pk".to_string(), AV::S("item1".to_string()));
                    item.insert("alt_key".to_string(), AV::S("alt1".to_string()));
                    let mut range_map = std::collections::HashMap::new();
                    range_map.insert(String::new(), item);
                    let mut items = std::collections::HashMap::new();
                    // The hash key in the items map is serialize_key_value(S("item1")),
                    // which is just "item1".
                    items.insert("item1".to_string(), range_map);
                    items
                },
                stream_enabled: false,
                stream_view_type: None,
                latest_stream_arn: None,
                latest_stream_label: None,
                global_secondary_index: vec![SecondaryIndexView {
                    index_name: "alt-index".to_string(),
                    key_schema: vec![KeySchemaElementView {
                        attribute_name: "alt_key".to_string(),
                        key_type: "HASH".to_string(),
                    }],
                    projection_type: "ALL".to_string(),
                    non_key_attributes: vec![],
                }],
                local_secondary_index: vec![],
                replica: vec![],
                import_table: None,
                on_demand_throughput: None,
            },
        )]
        .into_iter()
        .collect(),
        tags: Default::default(),
        ttl_configs: Default::default(),
        ..Default::default()
    };

    // Serde round-trip.
    let json = serde_json::to_string(&view).expect("serialize DynamodbStateView");
    let restored: DynamodbStateView =
        serde_json::from_str(&json).expect("deserialize DynamodbStateView");

    // Verify the GSI survived.
    let tv = restored.tables.get("snap-gsi-tbl").unwrap();
    assert_eq!(tv.global_secondary_index.len(), 1);
    assert_eq!(tv.global_secondary_index[0].index_name, "alt-index");

    // Restore into a fresh service and verify GSI query works.
    let svc = DynamoDbService::new();
    svc.restore("123456789012", "us-east-1", restored)
        .await
        .expect("restore");

    let mock = MockAws::builder().with_service(svc).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dynamodb::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_dynamodb::Client::new(&config);

    // DescribeTable should show the GSI.
    let desc = client
        .describe_table()
        .table_name("snap-gsi-tbl")
        .send()
        .await
        .expect("describe after restore");
    let gsis = desc.table().unwrap().global_secondary_indexes();
    assert_eq!(gsis.len(), 1, "GSI should survive snapshot/restore");
    assert_eq!(gsis[0].index_name(), Some("alt-index"));

    // Query via the GSI should find the restored item.
    let result = client
        .query()
        .table_name("snap-gsi-tbl")
        .index_name("alt-index")
        .key_condition_expression("alt_key = :ak")
        .expression_attribute_values(":ak", AttributeValue::S("alt1".into()))
        .send()
        .await
        .expect("GSI query after restore");
    assert_eq!(
        result.count(),
        1,
        "GSI query should find the item after restore"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
//  PartiQL conditional functions — end-to-end coverage of every function
//  documented at https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ql-functions.html
//  Behaviour pinned by the empirical AWS probe on 2026-04-29 (ap-northeast-1).
//  Test names use the `test_partiql_fn_…` prefix.
// ─────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_partiql_fn_size_in_where() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-fn-size").await;

    client
        .put_item()
        .table_name("pq-fn-size")
        .item("pk", AttributeValue::S("k1".into()))
        .item("blob", AttributeValue::S("hello world".into()))
        .send()
        .await
        .unwrap();

    // size("hello world") = 11; > 5 should match.
    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-fn-size\" WHERE pk = 'k1' AND size(blob) > 5")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.items().len(), 1, "size > 5 should match");

    // > 100 should not.
    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-fn-size\" WHERE pk = 'k1' AND size(blob) > 100")
        .send()
        .await
        .unwrap();
    assert!(resp.items().is_empty(), "size > 100 should not match");
}

#[tokio::test]
async fn test_partiql_fn_size_on_string_set() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-fn-size-ss").await;

    client
        .put_item()
        .table_name("pq-fn-size-ss")
        .item("pk", AttributeValue::S("k1".into()))
        .item(
            "tags",
            AttributeValue::Ss(vec!["a".into(), "b".into(), "c".into()]),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-fn-size-ss\" WHERE pk = 'k1' AND size(tags) = 3")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.items().len(), 1, "size of SS with 3 elements");
}

#[tokio::test]
async fn test_partiql_fn_attribute_type_match() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-fn-attrtype").await;

    client
        .put_item()
        .table_name("pq-fn-attrtype")
        .item("pk", AttributeValue::S("k1".into()))
        .item("name", AttributeValue::S("Acme".into()))
        .item("score", AttributeValue::N("42".into()))
        .send()
        .await
        .unwrap();

    let resp = client
        .execute_statement()
        .statement(
            "SELECT * FROM \"pq-fn-attrtype\" WHERE pk = 'k1' AND attribute_type(\"name\", 'S')",
        )
        .send()
        .await
        .unwrap();
    assert_eq!(resp.items().len(), 1, "name is S");

    let resp = client
        .execute_statement()
        .statement(
            "SELECT * FROM \"pq-fn-attrtype\" WHERE pk = 'k1' AND attribute_type(\"name\", 'N')",
        )
        .send()
        .await
        .unwrap();
    assert!(resp.items().is_empty(), "name is not N");

    let resp = client
        .execute_statement()
        .statement(
            "SELECT * FROM \"pq-fn-attrtype\" WHERE pk = 'k1' AND attribute_type(\"score\", 'N')",
        )
        .send()
        .await
        .unwrap();
    assert_eq!(resp.items().len(), 1, "score is N");
}

#[tokio::test]
async fn test_partiql_fn_begins_with_string() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-fn-bw").await;

    client
        .put_item()
        .table_name("pq-fn-bw")
        .item("pk", AttributeValue::S("k1".into()))
        .item("path", AttributeValue::S("user/profile/avatar.png".into()))
        .send()
        .await
        .unwrap();

    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-fn-bw\" WHERE pk = 'k1' AND begins_with(\"path\", 'user/')")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.items().len(), 1, "begins_with prefix match");

    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-fn-bw\" WHERE pk = 'k1' AND begins_with(\"path\", 'admin/')")
        .send()
        .await
        .unwrap();
    assert!(resp.items().is_empty(), "begins_with non-matching prefix");
}

#[tokio::test]
async fn test_partiql_fn_contains_substring_and_set() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-fn-contains").await;

    client
        .put_item()
        .table_name("pq-fn-contains")
        .item("pk", AttributeValue::S("k1".into()))
        .item(
            "title",
            AttributeValue::S("Rust programming language".into()),
        )
        .item(
            "tags",
            AttributeValue::Ss(vec!["rust".into(), "systems".into()]),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .execute_statement()
        .statement(
            "SELECT * FROM \"pq-fn-contains\" WHERE pk = 'k1' AND contains(title, 'programming')",
        )
        .send()
        .await
        .unwrap();
    assert_eq!(resp.items().len(), 1, "contains substring match");

    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-fn-contains\" WHERE pk = 'k1' AND contains(tags, 'rust')")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.items().len(), 1, "contains SS element match");
}

#[tokio::test]
async fn test_partiql_fn_is_missing_distinguishes_present_vs_absent() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-fn-ismissing").await;

    client
        .put_item()
        .table_name("pq-fn-ismissing")
        .item("pk", AttributeValue::S("with-name".into()))
        .item("name", AttributeValue::S("Acme".into()))
        .send()
        .await
        .unwrap();
    client
        .put_item()
        .table_name("pq-fn-ismissing")
        .item("pk", AttributeValue::S("no-name".into()))
        .send()
        .await
        .unwrap();

    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-fn-ismissing\" WHERE pk = 'no-name' AND name IS MISSING")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.items().len(), 1, "IS MISSING matches missing attr");

    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-fn-ismissing\" WHERE pk = 'with-name' AND name IS MISSING")
        .send()
        .await
        .unwrap();
    assert!(
        resp.items().is_empty(),
        "IS MISSING does not match present attr"
    );

    let resp = client
        .execute_statement()
        .statement(
            "SELECT * FROM \"pq-fn-ismissing\" WHERE pk = 'with-name' AND name IS NOT MISSING",
        )
        .send()
        .await
        .unwrap();
    assert_eq!(resp.items().len(), 1, "IS NOT MISSING matches present attr");
}

#[tokio::test]
async fn test_partiql_fn_is_null_distinguishes_null_value_from_missing() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-fn-isnull").await;

    client
        .put_item()
        .table_name("pq-fn-isnull")
        .item("pk", AttributeValue::S("explicit-null".into()))
        .item("name", AttributeValue::Null(true))
        .send()
        .await
        .unwrap();
    client
        .put_item()
        .table_name("pq-fn-isnull")
        .item("pk", AttributeValue::S("missing".into()))
        .send()
        .await
        .unwrap();
    client
        .put_item()
        .table_name("pq-fn-isnull")
        .item("pk", AttributeValue::S("present".into()))
        .item("name", AttributeValue::S("Acme".into()))
        .send()
        .await
        .unwrap();

    // IS NULL matches the explicit {NULL: true} attribute.
    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-fn-isnull\" WHERE pk = 'explicit-null' AND name IS NULL")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.items().len(), 1, "IS NULL matches {{NULL: true}}");

    // IS NULL does NOT match a missing attribute (verified empirically against AWS).
    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-fn-isnull\" WHERE pk = 'missing' AND name IS NULL")
        .send()
        .await
        .unwrap();
    assert!(
        resp.items().is_empty(),
        "IS NULL should NOT match a missing attribute (NULL ≠ MISSING)"
    );

    // IS NOT NULL matches a present non-null attribute.
    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-fn-isnull\" WHERE pk = 'present' AND name IS NOT NULL")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.items().len(), 1, "IS NOT NULL matches non-null attr");
}

#[tokio::test]
async fn test_partiql_fn_exists_rejected_in_execute_statement() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-fn-exists-1").await;

    let err = client
        .execute_statement()
        .statement(
            "EXISTS(SELECT * FROM \"pq-fn-exists-1\" WHERE pk = 'k1' AND name IS NOT MISSING)",
        )
        .send()
        .await
        .unwrap_err();
    let s = format!("{err:?}");
    // AWS returns: "EXISTS can only be used in ExecuteTransaction write requests."
    assert!(
        s.contains("EXISTS") && s.contains("ExecuteTransaction"),
        "expected EXISTS-rejection message, got: {s}"
    );
}

#[tokio::test]
async fn test_partiql_fn_exists_in_transaction_with_condition_met() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-fn-exists-2").await;

    client
        .put_item()
        .table_name("pq-fn-exists-2")
        .item("pk", AttributeValue::S("anchor".into()))
        .item("status", AttributeValue::S("active".into()))
        .send()
        .await
        .unwrap();

    let resp = client
        .execute_transaction()
        .transact_statements(
            aws_sdk_dynamodb::types::ParameterizedStatement::builder()
                .statement(
                    "EXISTS(SELECT * FROM \"pq-fn-exists-2\" WHERE pk = 'anchor' AND status = 'active')",
                )
                .build()
                .unwrap(),
        )
        .transact_statements(
            aws_sdk_dynamodb::types::ParameterizedStatement::builder()
                .statement("INSERT INTO \"pq-fn-exists-2\" VALUE {'pk': 'inserted'}")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("EXISTS condition met, transaction should commit");
    // Both statements executed.
    assert!(!resp.responses().is_empty());

    // Verify the INSERT actually happened.
    let g = client
        .get_item()
        .table_name("pq-fn-exists-2")
        .key("pk", AttributeValue::S("inserted".into()))
        .send()
        .await
        .unwrap();
    assert!(g.item().is_some(), "inserted item should exist");
}

#[tokio::test]
async fn test_partiql_fn_exists_in_transaction_with_condition_unmet_aborts() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-fn-exists-3").await;

    let err = client
        .execute_transaction()
        .transact_statements(
            aws_sdk_dynamodb::types::ParameterizedStatement::builder()
                .statement(
                    "EXISTS(SELECT * FROM \"pq-fn-exists-3\" WHERE pk = 'NOPE' AND status = 'active')",
                )
                .build()
                .unwrap(),
        )
        .transact_statements(
            aws_sdk_dynamodb::types::ParameterizedStatement::builder()
                .statement("INSERT INTO \"pq-fn-exists-3\" VALUE {'pk': 'shouldnt'}")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect_err("EXISTS condition not met, transaction should abort");
    let s = format!("{err:?}");
    assert!(
        s.contains("Transaction") || s.contains("EXISTS"),
        "expected transaction-cancellation error, got: {s}"
    );

    // Verify the INSERT did NOT happen.
    let g = client
        .get_item()
        .table_name("pq-fn-exists-3")
        .key("pk", AttributeValue::S("shouldnt".into()))
        .send()
        .await
        .unwrap();
    assert!(g.item().is_none(), "INSERT should NOT have happened");
}

#[tokio::test]
async fn test_partiql_fn_exists_rejects_pk_only_inner_select() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-fn-exists-4").await;

    // Per AWS empirical findings: EXISTS requires a non-key predicate in
    // the inner SELECT in addition to the full primary key.
    let err = client
        .execute_transaction()
        .transact_statements(
            aws_sdk_dynamodb::types::ParameterizedStatement::builder()
                .statement("EXISTS(SELECT * FROM \"pq-fn-exists-4\" WHERE pk = 'k1')")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect_err("EXISTS without additional condition should fail");
    let s = format!("{err:?}");
    assert!(
        s.contains("additional condition") || s.contains("single item"),
        "expected EXISTS-validation error, got: {s}"
    );
}

// ─── PartiQL: AWS-fidelity rejection of arithmetic / unary-neg in WHERE ──────

#[tokio::test]
async fn test_partiql_where_arithmetic_add_rejected() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-where-arith-add").await;

    let err = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-where-arith-add\" WHERE counter = 5 + 5")
        .send()
        .await
        .expect_err("WHERE arithmetic should be rejected");
    let s = format!("{err:?}");
    assert!(
        s.contains("Unsupported operator in Condition Expression. Operator: +"),
        "expected AWS-shaped + rejection, got: {s}"
    );
}

#[tokio::test]
async fn test_partiql_where_arithmetic_sub_rejected_in_update() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-where-arith-sub").await;

    let err = client
        .execute_statement()
        .statement("UPDATE \"pq-where-arith-sub\" SET v = 1 WHERE pk = 'k1' AND counter = 10 - 1")
        .send()
        .await
        .expect_err("WHERE arithmetic in UPDATE should be rejected");
    let s = format!("{err:?}");
    assert!(
        s.contains("Unsupported operator in Condition Expression. Operator: -"),
        "expected AWS-shaped - rejection, got: {s}"
    );
}

#[tokio::test]
async fn test_partiql_where_unary_neg_path_rejected() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-where-neg").await;

    let err = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-where-neg\" WHERE -counter = 5")
        .send()
        .await
        .expect_err("unary-neg over a path should be rejected");
    let s = format!("{err:?}");
    assert!(
        s.contains(
            "Incorrect number of operands for operator or function; operator or function: -, number of operands: 1"
        ),
        "expected AWS-shaped unary-neg rejection, got: {s}"
    );
}

#[tokio::test]
async fn test_partiql_where_arithmetic_rejected_in_transaction() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-where-arith-txn").await;

    let err = client
        .execute_transaction()
        .transact_statements(
            aws_sdk_dynamodb::types::ParameterizedStatement::builder()
                .statement("SELECT * FROM \"pq-where-arith-txn\" WHERE counter = 1 + 1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect_err("WHERE arithmetic in transaction should be rejected");
    let s = format!("{err:?}");
    assert!(
        s.contains("Unsupported operator in Condition Expression. Operator: +"),
        "expected AWS-shaped + rejection in transaction, got: {s}"
    );
}

#[tokio::test]
async fn test_partiql_where_arithmetic_rejected_in_batch_per_item() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-where-arith-batch").await;

    let resp = client
        .batch_execute_statement()
        .statements(
            aws_sdk_dynamodb::types::BatchStatementRequest::builder()
                .statement("SELECT * FROM \"pq-where-arith-batch\" WHERE counter = 5 + 5")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("batch_execute_statement returns per-item error, not a top-level fault");
    let responses = resp.responses();
    assert_eq!(responses.len(), 1);
    let err = responses[0]
        .error()
        .expect("first batch response should carry an error");
    let msg = err.message().unwrap_or("");
    assert!(
        msg.contains("Unsupported operator in Condition Expression. Operator: +"),
        "expected AWS-shaped + rejection in batch, got: {msg}"
    );
}

#[tokio::test]
async fn test_partiql_batch_exists_message_aligned_with_aws() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "pq-batch-exists-msg").await;

    let resp = client
        .batch_execute_statement()
        .statements(
            aws_sdk_dynamodb::types::BatchStatementRequest::builder()
                .statement(
                    "EXISTS(SELECT * FROM \"pq-batch-exists-msg\" WHERE pk = 'k1' AND name IS NOT MISSING)",
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("batch returns per-item error for top-level EXISTS");
    let responses = resp.responses();
    assert_eq!(responses.len(), 1);
    let err = responses[0]
        .error()
        .expect("EXISTS in batch should be rejected per-item");
    let msg = err.message().unwrap_or("");
    assert_eq!(
        msg, "EXISTS can only be used in ExecuteTransaction write requests.",
        "batch EXISTS rejection wording must match AWS's exact response"
    );
}

/// Regression test for `dynamodb-partiql-sort-key-equality`.
///
/// Before the fix, `SELECT … WHERE pk = ? AND sk = ?` returned the entire
/// partition because the PartiQL planner emitted both equalities as
/// `key_conditions` but the backend only honoured the hash-key one.
#[tokio::test]
async fn test_partiql_select_sort_key_equality() {
    let client = make_dynamodb_client().await;
    create_hash_range_table(&client, "pq-sk-eq").await;

    for sk in &["001", "002", "003", "004", "005"] {
        client
            .put_item()
            .table_name("pq-sk-eq")
            .item("pk", AttributeValue::S("p1".to_string()))
            .item("sk", AttributeValue::S(sk.to_string()))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .execute_statement()
        .statement("SELECT * FROM \"pq-sk-eq\" WHERE pk = ? AND sk = ?")
        .parameters(AttributeValue::S("p1".to_string()))
        .parameters(AttributeValue::S("003".to_string()))
        .send()
        .await
        .expect("partiql with sk equality should succeed");

    let items = resp.items();
    assert_eq!(items.len(), 1, "sk equality must narrow to a single item");
    assert_eq!(items[0].get("sk").unwrap().as_s().unwrap(), "003");
}

// ─── Regression tests for the 2026-04-30 dynamodb-backend TODO sweep ────────

/// Regression test for `dynamodb-lsi-silently-dropped`.
///
/// Before the fix, `CreateTable` accepted `LocalSecondaryIndexes` but
/// `DescribeTable.Table.LocalSecondaryIndexes` came back as `[]`. Now it
/// must round-trip the LSI definition end-to-end.
#[tokio::test]
async fn test_describe_table_returns_local_secondary_indexes() {
    use aws_sdk_dynamodb::types::{LocalSecondaryIndex, Projection, ProjectionType};

    let client = make_dynamodb_client().await;

    client
        .create_table()
        .table_name("lsi-roundtrip")
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("pk")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("sk")
                .key_type(KeyType::Range)
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
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("sk")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("lsi_sk")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .local_secondary_indexes(
            LocalSecondaryIndex::builder()
                .index_name("ByPriority")
                .key_schema(
                    KeySchemaElement::builder()
                        .attribute_name("pk")
                        .key_type(KeyType::Hash)
                        .build()
                        .unwrap(),
                )
                .key_schema(
                    KeySchemaElement::builder()
                        .attribute_name("lsi_sk")
                        .key_type(KeyType::Range)
                        .build()
                        .unwrap(),
                )
                .projection(
                    Projection::builder()
                        .projection_type(ProjectionType::All)
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_table with LSI");

    let desc = client
        .describe_table()
        .table_name("lsi-roundtrip")
        .send()
        .await
        .expect("describe_table");

    let lsis = desc.table().unwrap().local_secondary_indexes().to_vec();
    assert_eq!(
        lsis.len(),
        1,
        "expected 1 LSI in DescribeTable response, got {}",
        lsis.len()
    );
    let lsi = &lsis[0];
    assert_eq!(lsi.index_name().unwrap(), "ByPriority");
    let key_schema = lsi.key_schema();
    assert_eq!(key_schema.len(), 2);
    assert_eq!(key_schema[0].attribute_name(), "pk");
    assert_eq!(key_schema[1].attribute_name(), "lsi_sk");
}

/// Regression for `dynamodb-update-expression-actions` (1):
/// `ADD <attr> :v` against an `SS` attribute used to be silently no-op.
#[tokio::test]
async fn test_update_item_add_string_set() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "ui-add-ss").await;

    client
        .put_item()
        .table_name("ui-add-ss")
        .item("pk", AttributeValue::S("p1".into()))
        .item(
            "tags",
            AttributeValue::Ss(vec!["red".into(), "blue".into()]),
        )
        .send()
        .await
        .unwrap();

    client
        .update_item()
        .table_name("ui-add-ss")
        .key("pk", AttributeValue::S("p1".into()))
        .update_expression("ADD tags :extra")
        .expression_attribute_values(
            ":extra",
            AttributeValue::Ss(vec!["green".into(), "blue".into()]),
        )
        .send()
        .await
        .unwrap();

    let item = client
        .get_item()
        .table_name("ui-add-ss")
        .key("pk", AttributeValue::S("p1".into()))
        .send()
        .await
        .unwrap()
        .item
        .unwrap();
    let mut tags = item.get("tags").unwrap().as_ss().unwrap().clone();
    tags.sort();
    assert_eq!(
        tags,
        vec!["blue".to_string(), "green".to_string(), "red".to_string()],
        "ADD on SS must union members; duplicates are absorbed"
    );
}

/// Regression for `dynamodb-update-expression-actions` (1):
/// `DELETE <attr> :v` against an `SS` attribute used to be silently no-op.
#[tokio::test]
async fn test_update_item_delete_string_set() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "ui-del-ss").await;

    client
        .put_item()
        .table_name("ui-del-ss")
        .item("pk", AttributeValue::S("p1".into()))
        .item(
            "tags",
            AttributeValue::Ss(vec!["red".into(), "blue".into(), "green".into()]),
        )
        .send()
        .await
        .unwrap();

    client
        .update_item()
        .table_name("ui-del-ss")
        .key("pk", AttributeValue::S("p1".into()))
        .update_expression("DELETE tags :rm")
        .expression_attribute_values(":rm", AttributeValue::Ss(vec!["red".into(), "blue".into()]))
        .send()
        .await
        .unwrap();

    let item = client
        .get_item()
        .table_name("ui-del-ss")
        .key("pk", AttributeValue::S("p1".into()))
        .send()
        .await
        .unwrap()
        .item
        .unwrap();
    let tags = item.get("tags").unwrap().as_ss().unwrap().clone();
    assert_eq!(tags, vec!["green".to_string()]);
}

/// Regression for `dynamodb-update-expression-actions` (2):
/// `SET info.city = :v` used to create a literal top-level attribute
/// `"info.city"` instead of updating the nested map field.
#[tokio::test]
async fn test_update_item_set_nested_path() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "ui-nested").await;

    let mut info: std::collections::HashMap<String, AttributeValue> =
        std::collections::HashMap::new();
    info.insert("city".to_string(), AttributeValue::S("Tokyo".into()));
    info.insert("country".to_string(), AttributeValue::S("JP".into()));
    client
        .put_item()
        .table_name("ui-nested")
        .item("pk", AttributeValue::S("p1".into()))
        .item("info", AttributeValue::M(info))
        .send()
        .await
        .unwrap();

    client
        .update_item()
        .table_name("ui-nested")
        .key("pk", AttributeValue::S("p1".into()))
        .update_expression("SET info.city = :c")
        .expression_attribute_values(":c", AttributeValue::S("Osaka".into()))
        .send()
        .await
        .unwrap();

    let item = client
        .get_item()
        .table_name("ui-nested")
        .key("pk", AttributeValue::S("p1".into()))
        .send()
        .await
        .unwrap()
        .item
        .unwrap();
    assert!(
        !item.contains_key("info.city"),
        "must not create literal 'info.city' top-level attribute"
    );
    let info = item.get("info").unwrap().as_m().unwrap();
    assert_eq!(info.get("city").unwrap().as_s().unwrap(), "Osaka");
    assert_eq!(
        info.get("country").unwrap().as_s().unwrap(),
        "JP",
        "siblings must be preserved"
    );
}

/// Regression for `dynamodb-update-expression-actions` (3):
/// `SET #i.#c = :v` with `ExpressionAttributeNames` used to create a
/// literal top-level attribute `"#i.#c"` because aliases weren't
/// resolved per dotted segment.
#[tokio::test]
async fn test_update_item_set_nested_path_with_expr_names() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "ui-nested-aliased").await;

    let mut info: std::collections::HashMap<String, AttributeValue> =
        std::collections::HashMap::new();
    info.insert("city".to_string(), AttributeValue::S("Tokyo".into()));
    client
        .put_item()
        .table_name("ui-nested-aliased")
        .item("pk", AttributeValue::S("p1".into()))
        .item("info", AttributeValue::M(info))
        .send()
        .await
        .unwrap();

    client
        .update_item()
        .table_name("ui-nested-aliased")
        .key("pk", AttributeValue::S("p1".into()))
        .update_expression("SET #i.#c = :v")
        .expression_attribute_names("#i", "info")
        .expression_attribute_names("#c", "city")
        .expression_attribute_values(":v", AttributeValue::S("Kyoto".into()))
        .send()
        .await
        .unwrap();

    let item = client
        .get_item()
        .table_name("ui-nested-aliased")
        .key("pk", AttributeValue::S("p1".into()))
        .send()
        .await
        .unwrap()
        .item
        .unwrap();
    assert!(!item.contains_key("#i.#c"));
    let info = item.get("info").unwrap().as_m().unwrap();
    assert_eq!(info.get("city").unwrap().as_s().unwrap(), "Kyoto");
}

/// Regression for `dynamodb-update-expression-actions` (4):
/// `SET list = list_append(list, :v)` was a silent no-op against the
/// Redis backend; the in-memory backend now uses the shared apply path
/// so both are exercised here.
#[tokio::test]
async fn test_update_item_set_list_append() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "ui-list-append").await;

    client
        .put_item()
        .table_name("ui-list-append")
        .item("pk", AttributeValue::S("p1".into()))
        .item(
            "items",
            AttributeValue::L(vec![AttributeValue::S("a".into())]),
        )
        .send()
        .await
        .unwrap();

    client
        .update_item()
        .table_name("ui-list-append")
        .key("pk", AttributeValue::S("p1".into()))
        .update_expression("SET #l = list_append(#l, :v)")
        .expression_attribute_names("#l", "items")
        .expression_attribute_values(
            ":v",
            AttributeValue::L(vec![
                AttributeValue::S("b".into()),
                AttributeValue::S("c".into()),
            ]),
        )
        .send()
        .await
        .unwrap();

    let item = client
        .get_item()
        .table_name("ui-list-append")
        .key("pk", AttributeValue::S("p1".into()))
        .send()
        .await
        .unwrap()
        .item
        .unwrap();
    let list = item.get("items").unwrap().as_l().unwrap();
    let strs: Vec<&str> = list.iter().map(|v| v.as_s().unwrap().as_str()).collect();
    assert_eq!(strs, vec!["a", "b", "c"]);
}

/// Regression for `dynamodb-update-expression-actions` (5):
/// `SET maybe = if_not_exists(maybe, :d)` used to leave the missing
/// attribute missing.
#[tokio::test]
async fn test_update_item_set_if_not_exists() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "ui-ifnotexists").await;

    client
        .put_item()
        .table_name("ui-ifnotexists")
        .item("pk", AttributeValue::S("p1".into()))
        .send()
        .await
        .unwrap();

    // First call: attribute is absent — sets to "first".
    client
        .update_item()
        .table_name("ui-ifnotexists")
        .key("pk", AttributeValue::S("p1".into()))
        .update_expression("SET maybe = if_not_exists(maybe, :v)")
        .expression_attribute_values(":v", AttributeValue::S("first".into()))
        .send()
        .await
        .unwrap();

    // Second call: attribute already present — must not overwrite.
    client
        .update_item()
        .table_name("ui-ifnotexists")
        .key("pk", AttributeValue::S("p1".into()))
        .update_expression("SET maybe = if_not_exists(maybe, :v)")
        .expression_attribute_values(":v", AttributeValue::S("second".into()))
        .send()
        .await
        .unwrap();

    let item = client
        .get_item()
        .table_name("ui-ifnotexists")
        .key("pk", AttributeValue::S("p1".into()))
        .send()
        .await
        .unwrap()
        .item
        .unwrap();
    assert_eq!(item.get("maybe").unwrap().as_s().unwrap(), "first");
}

/// Regression for `dynamodb-projection-expression-ignored`.
///
/// `GetItem --projection-expression "tags"` used to return the full item
/// including non-projected attributes.
#[tokio::test]
async fn test_get_item_projection_expression() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "ui-proj").await;

    client
        .put_item()
        .table_name("ui-proj")
        .item("pk", AttributeValue::S("p1".into()))
        .item("name", AttributeValue::S("Alice".into()))
        .item(
            "tags",
            AttributeValue::Ss(vec!["red".into(), "blue".into()]),
        )
        .item("hidden", AttributeValue::S("secret".into()))
        .send()
        .await
        .unwrap();

    let resp = client
        .get_item()
        .table_name("ui-proj")
        .key("pk", AttributeValue::S("p1".into()))
        .projection_expression("#t, #n")
        .expression_attribute_names("#t", "tags")
        .expression_attribute_names("#n", "name")
        .send()
        .await
        .unwrap();

    let item = resp.item().unwrap();
    assert!(item.contains_key("tags"));
    assert!(item.contains_key("name"));
    assert!(
        !item.contains_key("hidden"),
        "non-projected attributes must be filtered out"
    );
    assert!(
        !item.contains_key("pk"),
        "key attribute that wasn't projected must be excluded"
    );
}

/// Regression for `dynamodb-projection-expression-ignored` — nested-path
/// projection. `info.city` should return only `info: { city: ... }`,
/// dropping siblings inside `info`.
#[tokio::test]
async fn test_get_item_projection_expression_nested() {
    let client = make_dynamodb_client().await;
    create_hash_table(&client, "ui-proj-nested").await;

    let mut info: std::collections::HashMap<String, AttributeValue> =
        std::collections::HashMap::new();
    info.insert("city".to_string(), AttributeValue::S("Tokyo".into()));
    info.insert("country".to_string(), AttributeValue::S("JP".into()));
    client
        .put_item()
        .table_name("ui-proj-nested")
        .item("pk", AttributeValue::S("p1".into()))
        .item("info", AttributeValue::M(info))
        .item("other", AttributeValue::S("x".into()))
        .send()
        .await
        .unwrap();

    let resp = client
        .get_item()
        .table_name("ui-proj-nested")
        .key("pk", AttributeValue::S("p1".into()))
        .projection_expression("info.city")
        .send()
        .await
        .unwrap();

    let item = resp.item().unwrap();
    assert!(!item.contains_key("other"));
    let info = item.get("info").unwrap().as_m().unwrap();
    assert!(info.contains_key("city"));
    assert!(
        !info.contains_key("country"),
        "siblings inside the nested map must be excluded by projection"
    );
}
