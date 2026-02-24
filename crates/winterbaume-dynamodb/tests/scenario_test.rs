//! Smoke tests for winterbaume DynamoDB service — realistic application scenarios.
//!
//! Each test simulates a coherent end-to-end workflow rather than exercising
//! a single API call in isolation.

use aws_sdk_dynamodb::config::BehaviorVersion;
use aws_sdk_dynamodb::types::{
    AttributeDefinition, AttributeValue, KeySchemaElement, KeyType, ProvisionedThroughput,
    ReturnValue, ScalarAttributeType, TransactWriteItem, Update,
};
use winterbaume_core::MockAws;
use winterbaume_dynamodb::DynamoDbService;

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

fn throughput(r: i64, w: i64) -> ProvisionedThroughput {
    ProvisionedThroughput::builder()
        .read_capacity_units(r)
        .write_capacity_units(w)
        .build()
        .unwrap()
}

/// Scenario: user profile CRUD with optimistic locking.
///
/// A user-profile service stores profiles with a `version` counter. Each
/// update must succeed only when the caller's version matches the stored
/// version (optimistic locking via a condition expression). A concurrent
/// update with a stale version should be rejected.
#[tokio::test]
async fn test_user_profile_optimistic_locking() {
    let client = make_dynamodb_client().await;

    // Create the profiles table.
    client
        .create_table()
        .table_name("user-profiles")
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("user_id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("user_id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(throughput(5, 5))
        .send()
        .await
        .expect("create_table user-profiles");

    // Insert initial profile at version 0.
    client
        .put_item()
        .table_name("user-profiles")
        .item("user_id", AttributeValue::S("u-001".to_string()))
        .item("email", AttributeValue::S("alice@example.com".to_string()))
        .item("version", AttributeValue::N("0".to_string()))
        .send()
        .await
        .expect("put_item initial profile");

    // Correct update: version matches — increment version and change email.
    let upd = client
        .update_item()
        .table_name("user-profiles")
        .key("user_id", AttributeValue::S("u-001".to_string()))
        .update_expression("SET email = :new_email, version = :new_ver")
        .condition_expression("version = :expected_ver")
        .expression_attribute_values(
            ":new_email",
            AttributeValue::S("alice2@example.com".to_string()),
        )
        .expression_attribute_values(":new_ver", AttributeValue::N("1".to_string()))
        .expression_attribute_values(":expected_ver", AttributeValue::N("0".to_string()))
        .return_values(ReturnValue::AllNew)
        .send()
        .await
        .expect("update_item with correct version");

    let new_version = upd
        .attributes()
        .and_then(|a| a.get("version"))
        .and_then(|v| v.as_n().ok())
        .expect("updated version");
    assert_eq!(new_version, "1", "version should have been incremented");

    // Stale update: simulate a concurrent writer using the old version — should fail.
    let stale = client
        .update_item()
        .table_name("user-profiles")
        .key("user_id", AttributeValue::S("u-001".to_string()))
        .update_expression("SET email = :stale_email")
        .condition_expression("version = :expected_ver")
        .expression_attribute_values(
            ":stale_email",
            AttributeValue::S("stale@example.com".to_string()),
        )
        .expression_attribute_values(":expected_ver", AttributeValue::N("0".to_string()))
        .send()
        .await;
    assert!(
        stale.is_err(),
        "stale update should fail due to condition check"
    );

    // Final state: email should still reflect the successful update.
    let get = client
        .get_item()
        .table_name("user-profiles")
        .key("user_id", AttributeValue::S("u-001".to_string()))
        .send()
        .await
        .expect("get_item final state");
    let item = get.item().expect("item should exist");
    let email = item
        .get("email")
        .and_then(|v| v.as_s().ok())
        .expect("email attribute");
    assert_eq!(email, "alice2@example.com");
}

/// Scenario: leaderboard with per-game score lookup.
///
/// A gaming service maintains a scores table keyed by (game_id, player_id).
/// It writes 5 player scores across two games, then queries by game hash key
/// to retrieve all scores for one game, and scans with a filter to find
/// high-scoring players across the whole table.
///
/// GSI query support is now implemented — see `test_gsi_query` below for
/// a dedicated GSI test. This test exercises the primary-key query path.
#[tokio::test]
async fn test_leaderboard_query_and_scan() {
    let client = make_dynamodb_client().await;

    // Create table: primary key = (game_id HASH, player_id RANGE).
    client
        .create_table()
        .table_name("scores")
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("game_id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("player_id")
                .key_type(KeyType::Range)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("game_id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("player_id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(throughput(5, 5))
        .send()
        .await
        .expect("create_table scores");

    // Write 5 scores for "galactic-shooter" and 2 for "space-race".
    let entries = [
        ("galactic-shooter", "player-A", 9500),
        ("galactic-shooter", "player-B", 7200),
        ("galactic-shooter", "player-C", 8800),
        ("galactic-shooter", "player-D", 6100),
        ("galactic-shooter", "player-E", 9100),
        ("space-race", "player-A", 3100),
        ("space-race", "player-C", 4200),
    ];
    for (game, player, score) in entries {
        client
            .put_item()
            .table_name("scores")
            .item("game_id", AttributeValue::S(game.to_string()))
            .item("player_id", AttributeValue::S(player.to_string()))
            .item("score", AttributeValue::N(score.to_string()))
            .send()
            .await
            .unwrap_or_else(|e| panic!("put_item {game}/{player}: {e}"));
    }

    // Query all scores for "galactic-shooter" by primary hash key.
    let gal = client
        .query()
        .table_name("scores")
        .key_condition_expression("game_id = :gid")
        .expression_attribute_values(":gid", AttributeValue::S("galactic-shooter".to_string()))
        .send()
        .await
        .expect("query galactic-shooter");
    assert_eq!(
        gal.count(),
        5,
        "galactic-shooter should have 5 score entries"
    );

    // Query with limit to get top-N (sorted by range key, player_id).
    let top3 = client
        .query()
        .table_name("scores")
        .key_condition_expression("game_id = :gid")
        .expression_attribute_values(":gid", AttributeValue::S("galactic-shooter".to_string()))
        .limit(3)
        .send()
        .await
        .expect("query limit 3");
    assert_eq!(top3.count(), 3, "limit 3 should return exactly 3 items");

    // Scan with filter to find players with score > 8000 across all games.
    let elite = client
        .scan()
        .table_name("scores")
        .filter_expression("score > :threshold")
        .expression_attribute_values(":threshold", AttributeValue::N("8000".to_string()))
        .send()
        .await
        .expect("scan elite players");
    // galactic-shooter: player-A(9500), player-C(8800), player-E(9100) = 3
    assert_eq!(
        elite.count(),
        3,
        "3 players should have score > 8000 across all games"
    );
}

/// Scenario: query via a Global Secondary Index.
///
/// An orders table uses `order_id` as the hash key, but we need to query
/// orders by `customer_id`. A GSI with `customer_id` as the hash key and
/// `order_date` as the range key allows this access pattern.
#[tokio::test]
async fn test_gsi_query() {
    use aws_sdk_dynamodb::types::{GlobalSecondaryIndex, Projection, ProjectionType};

    let client = make_dynamodb_client().await;

    client
        .create_table()
        .table_name("orders-gsi")
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("order_id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("order_id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("customer_id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("order_date")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .global_secondary_indexes(
            GlobalSecondaryIndex::builder()
                .index_name("customer-orders-index")
                .key_schema(
                    KeySchemaElement::builder()
                        .attribute_name("customer_id")
                        .key_type(KeyType::Hash)
                        .build()
                        .unwrap(),
                )
                .key_schema(
                    KeySchemaElement::builder()
                        .attribute_name("order_date")
                        .key_type(KeyType::Range)
                        .build()
                        .unwrap(),
                )
                .projection(
                    Projection::builder()
                        .projection_type(ProjectionType::All)
                        .build(),
                )
                .provisioned_throughput(throughput(5, 5))
                .build()
                .unwrap(),
        )
        .provisioned_throughput(throughput(5, 5))
        .send()
        .await
        .expect("create orders-gsi table");

    // Insert orders for two customers.
    let orders = [
        ("ORD-001", "CUST-A", "2026-01-15", "Widget"),
        ("ORD-002", "CUST-B", "2026-01-16", "Gadget"),
        ("ORD-003", "CUST-A", "2026-02-01", "Sprocket"),
        ("ORD-004", "CUST-A", "2026-03-10", "Gizmo"),
        ("ORD-005", "CUST-B", "2026-03-12", "Doohickey"),
    ];
    for (oid, cid, date, product) in orders {
        client
            .put_item()
            .table_name("orders-gsi")
            .item("order_id", AttributeValue::S(oid.to_string()))
            .item("customer_id", AttributeValue::S(cid.to_string()))
            .item("order_date", AttributeValue::S(date.to_string()))
            .item("product", AttributeValue::S(product.to_string()))
            .send()
            .await
            .unwrap_or_else(|e| panic!("put_item {oid}: {e}"));
    }

    // Query GSI for CUST-A orders.
    let cust_a = client
        .query()
        .table_name("orders-gsi")
        .index_name("customer-orders-index")
        .key_condition_expression("customer_id = :cid")
        .expression_attribute_values(":cid", AttributeValue::S("CUST-A".to_string()))
        .send()
        .await
        .expect("query GSI for CUST-A");

    assert_eq!(cust_a.count(), 3, "CUST-A should have 3 orders");

    // Results should be sorted by order_date (range key of the GSI).
    let items = cust_a.items();
    let dates: Vec<&str> = items
        .iter()
        .map(|i| i.get("order_date").unwrap().as_s().unwrap().as_str())
        .collect();
    assert_eq!(
        dates,
        vec!["2026-01-15", "2026-02-01", "2026-03-10"],
        "CUST-A orders should be sorted by order_date ascending"
    );

    // Query GSI for CUST-B orders.
    let cust_b = client
        .query()
        .table_name("orders-gsi")
        .index_name("customer-orders-index")
        .key_condition_expression("customer_id = :cid")
        .expression_attribute_values(":cid", AttributeValue::S("CUST-B".to_string()))
        .send()
        .await
        .expect("query GSI for CUST-B");

    assert_eq!(cust_b.count(), 2, "CUST-B should have 2 orders");
}

/// Scenario: shopping cart with atomic multi-table writes and stock guard.
///
/// A checkout service atomically creates an order record and decrements
/// inventory using `transact_write_items`. A separate guard — a conditional
/// `update_item` — prevents the write when stock is insufficient.
///
/// ConditionExpression inside transact_write_items is now enforced.
/// SET arithmetic (e.g. "quantity = quantity - :dec") is also now supported,
/// though this test still uses `ADD quantity :neg_delta` for the checkout path.
#[tokio::test]
async fn test_shopping_cart_atomic_checkout() {
    let client = make_dynamodb_client().await;

    // Create the inventory table.
    client
        .create_table()
        .table_name("inventory")
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("sku")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("sku")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(throughput(5, 5))
        .send()
        .await
        .expect("create_table inventory");

    // Create the orders table.
    client
        .create_table()
        .table_name("orders")
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("order_id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("order_id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(throughput(5, 5))
        .send()
        .await
        .expect("create_table orders");

    // Seed inventory: 10 units of SKU-WIDGET, 2 units of SKU-GADGET.
    for (sku, qty) in [("SKU-WIDGET", 10i64), ("SKU-GADGET", 2)] {
        client
            .put_item()
            .table_name("inventory")
            .item("sku", AttributeValue::S(sku.to_string()))
            .item("quantity", AttributeValue::N(qty.to_string()))
            .send()
            .await
            .unwrap_or_else(|e| panic!("seed {sku}: {e}"));
    }

    // Successful checkout: atomically decrement widget stock and create order.
    // ADD with a negative delta is used to decrement because SET arithmetic
    // ("quantity = quantity - :dec") is not yet supported.
    client
        .transact_write_items()
        .transact_items(
            TransactWriteItem::builder()
                .update(
                    Update::builder()
                        .table_name("inventory")
                        .key("sku", AttributeValue::S("SKU-WIDGET".to_string()))
                        .update_expression("ADD quantity :neg_dec")
                        .expression_attribute_values(
                            ":neg_dec",
                            AttributeValue::N("-1".to_string()),
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
                        .table_name("orders")
                        .item("order_id", AttributeValue::S("ORD-001".to_string()))
                        .item("sku", AttributeValue::S("SKU-WIDGET".to_string()))
                        .item("qty", AttributeValue::N("1".to_string()))
                        .item("status", AttributeValue::S("confirmed".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("transact_write_items checkout widget");

    // Both effects must have landed atomically.
    let inv = client
        .get_item()
        .table_name("inventory")
        .key("sku", AttributeValue::S("SKU-WIDGET".to_string()))
        .send()
        .await
        .expect("get widget inventory");
    let qty = inv
        .item()
        .and_then(|i| i.get("quantity"))
        .and_then(|v| v.as_n().ok())
        .expect("quantity");
    assert_eq!(
        qty, "9",
        "widget quantity should have been decremented to 9"
    );

    let order = client
        .get_item()
        .table_name("orders")
        .key("order_id", AttributeValue::S("ORD-001".to_string()))
        .send()
        .await
        .expect("get order");
    assert!(order.item().is_some(), "order ORD-001 should exist");

    // Stock guard via conditional update_item: buying 5 gadgets when only 2
    // are in stock must fail with ConditionalCheckFailedException.
    let over_buy = client
        .update_item()
        .table_name("inventory")
        .key("sku", AttributeValue::S("SKU-GADGET".to_string()))
        .update_expression("ADD quantity :neg_dec")
        .condition_expression("quantity >= :required")
        .expression_attribute_values(":neg_dec", AttributeValue::N("-5".to_string()))
        .expression_attribute_values(":required", AttributeValue::N("5".to_string()))
        .send()
        .await;
    assert!(
        over_buy.is_err(),
        "purchasing 5 gadgets with stock=2 should fail the condition check"
    );

    // Gadget inventory should be unchanged (still 2).
    let gadget = client
        .get_item()
        .table_name("inventory")
        .key("sku", AttributeValue::S("SKU-GADGET".to_string()))
        .send()
        .await
        .expect("get gadget inventory");
    let gadget_qty = gadget
        .item()
        .and_then(|i| i.get("quantity"))
        .and_then(|v| v.as_n().ok())
        .expect("gadget quantity");
    assert_eq!(
        gadget_qty, "2",
        "gadget stock should be unchanged after rejected purchase"
    );

    // ConditionExpression inside transact_write_items: try to buy 5 gadgets
    // via a transaction — the condition should fail and cancel the transaction.
    let txn_result = client
        .transact_write_items()
        .transact_items(
            TransactWriteItem::builder()
                .update(
                    Update::builder()
                        .table_name("inventory")
                        .key("sku", AttributeValue::S("SKU-GADGET".to_string()))
                        .update_expression("ADD quantity :neg_dec")
                        .condition_expression("quantity >= :required")
                        .expression_attribute_values(
                            ":neg_dec",
                            AttributeValue::N("-5".to_string()),
                        )
                        .expression_attribute_values(
                            ":required",
                            AttributeValue::N("5".to_string()),
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
                        .table_name("orders")
                        .item("order_id", AttributeValue::S("ORD-002".to_string()))
                        .item("sku", AttributeValue::S("SKU-GADGET".to_string()))
                        .item("qty", AttributeValue::N("5".to_string()))
                        .item("status", AttributeValue::S("confirmed".to_string()))
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await;
    assert!(
        txn_result.is_err(),
        "transact_write with failing ConditionExpression should be cancelled"
    );

    // Gadget stock should still be 2 — transaction was rolled back.
    let gadget2 = client
        .get_item()
        .table_name("inventory")
        .key("sku", AttributeValue::S("SKU-GADGET".to_string()))
        .send()
        .await
        .expect("get gadget after txn cancel");
    let gadget_qty2 = gadget2
        .item()
        .and_then(|i| i.get("quantity"))
        .and_then(|v| v.as_n().ok())
        .expect("gadget quantity after cancel");
    assert_eq!(
        gadget_qty2, "2",
        "gadget stock should be unchanged after cancelled transaction"
    );

    // Order ORD-002 should not exist.
    let no_order = client
        .get_item()
        .table_name("orders")
        .key("order_id", AttributeValue::S("ORD-002".to_string()))
        .send()
        .await
        .expect("get ORD-002");
    assert!(
        no_order.item().is_none(),
        "ORD-002 should not exist after cancelled transaction"
    );
}

/// Scenario: paginated scan with filter for a reporting job.
///
/// A reporting job needs all "active" users born after 1990. The table has
/// 12 items; the job scans with a small page size and a FilterExpression,
/// collecting only matching records across multiple pages.
#[tokio::test]
async fn test_paginated_scan_with_filter() {
    let client = make_dynamodb_client().await;

    client
        .create_table()
        .table_name("users")
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("user_id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("user_id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(throughput(5, 5))
        .send()
        .await
        .expect("create_table users");

    // Insert 12 users: 7 active born after 1990, 5 inactive or older.
    let users: Vec<(&str, &str, i32)> = vec![
        ("u01", "active", 1995),
        ("u02", "inactive", 1995),
        ("u03", "active", 1992),
        ("u04", "active", 1985), // too old
        ("u05", "inactive", 1990),
        ("u06", "active", 1998),
        ("u07", "active", 1991),
        ("u08", "active", 1993),
        ("u09", "inactive", 1996),
        ("u10", "active", 2000),
        ("u11", "active", 1988), // too old
        ("u12", "active", 1997),
    ];
    for (uid, status, birth_year) in &users {
        client
            .put_item()
            .table_name("users")
            .item("user_id", AttributeValue::S(uid.to_string()))
            .item("status", AttributeValue::S(status.to_string()))
            .item("birth_year", AttributeValue::N(birth_year.to_string()))
            .send()
            .await
            .unwrap_or_else(|e| panic!("put_item {uid}: {e}"));
    }

    // Paginated scan: 4 items per page, filter active users born after 1990.
    let mut matched = Vec::new();
    let mut last_key: Option<std::collections::HashMap<String, AttributeValue>> = None;
    loop {
        let mut req = client
            .scan()
            .table_name("users")
            .filter_expression("#s = :active AND birth_year > :year")
            .expression_attribute_names("#s", "status")
            .expression_attribute_values(":active", AttributeValue::S("active".to_string()))
            .expression_attribute_values(":year", AttributeValue::N("1990".to_string()))
            .limit(4);
        if let Some(ref lek) = last_key {
            req = req.set_exclusive_start_key(Some(lek.clone()));
        }
        let page = req.send().await.expect("scan page");
        matched.extend(page.items().to_vec());
        if page.last_evaluated_key().is_none() {
            break;
        }
        last_key = page.last_evaluated_key().cloned();
    }

    // Expected matches: u01(1995), u03(1992), u06(1998), u07(1991), u08(1993), u10(2000), u12(1997) = 7
    assert_eq!(
        matched.len(),
        7,
        "filter should return exactly 7 active users born after 1990"
    );
    for item in &matched {
        let status = item
            .get("status")
            .and_then(|v| v.as_s().ok())
            .expect("status");
        let birth: i32 = item
            .get("birth_year")
            .and_then(|v| v.as_n().ok())
            .and_then(|s| s.parse().ok())
            .expect("birth_year");
        assert_eq!(status, "active");
        assert!(birth > 1990, "birth_year {birth} should be > 1990");
    }
}
