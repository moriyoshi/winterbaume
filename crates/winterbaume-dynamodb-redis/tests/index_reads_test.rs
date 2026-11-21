//! Secondary-index reads (issue #39) through the Redis backend.
//!
//! These tests need a Redis server. Set `WINTERBAUME_TEST_REDIS_URL` (for
//! example `redis://127.0.0.1:6379/15`) to run them; without it they return
//! early. Each test uses a uniquely named table and deletes it afterwards,
//! so the database does not need to be empty.

use std::collections::HashMap;
use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::config::BehaviorVersion;
use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_sdk_dynamodb::types::{
    AttributeDefinition, AttributeValue, BillingMode, GlobalSecondaryIndex, KeySchemaElement,
    KeyType, LocalSecondaryIndex, Projection, ProjectionType, ScalarAttributeType,
};
use winterbaume_core::MockAws;
use winterbaume_dynamodb::DynamoDbService;
use winterbaume_dynamodb_redis::RedisDynamoDbBackend;

/// A client backed by the Redis backend, or `None` when no Redis server is
/// configured.
async fn client() -> Option<Client> {
    let url = std::env::var("WINTERBAUME_TEST_REDIS_URL").ok()?;
    let backend = RedisDynamoDbBackend::from_connection_info(url.as_str())
        .await
        .expect("connect to WINTERBAUME_TEST_REDIS_URL");
    let mock = MockAws::builder()
        .with_service(DynamoDbService::with_backend(Arc::new(backend)))
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

/// The fixture from the in-memory integration tests: an INCLUDE GSI with a
/// sort key, a KEYS_ONLY GSI without one, and an INCLUDE LSI. `p2/c` lacks
/// the LSI sort key and `p3/d` lacks the GSI partition key, so both indexes
/// are sparse.
async fn create_projection_table(client: &Client, table_name: &str) {
    fn key(name: &str, key_type: KeyType) -> KeySchemaElement {
        KeySchemaElement::builder()
            .attribute_name(name)
            .key_type(key_type)
            .build()
            .unwrap()
    }
    fn attr(name: &str) -> AttributeDefinition {
        AttributeDefinition::builder()
            .attribute_name(name)
            .attribute_type(ScalarAttributeType::S)
            .build()
            .unwrap()
    }
    fn projection(projection_type: ProjectionType, include: &[&str]) -> Projection {
        let mut builder = Projection::builder().projection_type(projection_type);
        for name in include {
            builder = builder.non_key_attributes(*name);
        }
        builder.build()
    }

    client
        .create_table()
        .table_name(table_name)
        .billing_mode(BillingMode::PayPerRequest)
        .key_schema(key("pk", KeyType::Hash))
        .key_schema(key("sk", KeyType::Range))
        .attribute_definitions(attr("pk"))
        .attribute_definitions(attr("sk"))
        .attribute_definitions(attr("gsi_pk"))
        .attribute_definitions(attr("lsi_sk"))
        .global_secondary_indexes(
            GlobalSecondaryIndex::builder()
                .index_name("ByGsiPk")
                .key_schema(key("gsi_pk", KeyType::Hash))
                .key_schema(key("sk", KeyType::Range))
                .projection(projection(ProjectionType::Include, &["wanted"]))
                .build()
                .unwrap(),
        )
        .global_secondary_indexes(
            GlobalSecondaryIndex::builder()
                .index_name("ByGsiKeys")
                .key_schema(key("gsi_pk", KeyType::Hash))
                .projection(projection(ProjectionType::KeysOnly, &[]))
                .build()
                .unwrap(),
        )
        .local_secondary_indexes(
            LocalSecondaryIndex::builder()
                .index_name("ByLsi")
                .key_schema(key("pk", KeyType::Hash))
                .key_schema(key("lsi_sk", KeyType::Range))
                .projection(projection(ProjectionType::Include, &["wanted"]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let rows: [(&str, &str, Option<&str>, Option<&str>); 4] = [
        ("p1", "a", Some("g"), Some("l1")),
        ("p1", "b", Some("g"), Some("l2")),
        ("p2", "c", Some("g"), None),
        ("p3", "d", None, None),
    ];
    for (pk, sk, gsi_pk, lsi_sk) in rows {
        let mut req = client
            .put_item()
            .table_name(table_name)
            .item("pk", AttributeValue::S(pk.into()))
            .item("sk", AttributeValue::S(sk.into()))
            .item("wanted", AttributeValue::S(format!("w-{sk}")))
            .item("secret", AttributeValue::S(format!("s-{sk}")));
        if let Some(v) = gsi_pk {
            req = req.item("gsi_pk", AttributeValue::S(v.into()));
        }
        if let Some(v) = lsi_sk {
            req = req.item("lsi_sk", AttributeValue::S(v.into()));
        }
        req.send().await.unwrap();
    }
}

async fn delete_table(client: &Client, table_name: &str) {
    client
        .delete_table()
        .table_name(table_name)
        .send()
        .await
        .unwrap();
}

fn names(item: &HashMap<String, AttributeValue>) -> Vec<String> {
    let mut names: Vec<String> = item.keys().cloned().collect();
    names.sort();
    names
}

fn sk(item: &HashMap<String, AttributeValue>) -> String {
    item["sk"].as_s().unwrap().clone()
}

/// `Limit` counts index items only, every page carries the index key in
/// `LastEvaluatedKey`, and the projection applies.
#[tokio::test]
async fn scan_index_pages_over_the_sparse_index_only() {
    let Some(client) = client().await else { return };
    let table = unique_table("redis-scan-index");
    create_projection_table(&client, &table).await;

    let mut seen = Vec::new();
    let mut start = None;
    let mut pages = 0;
    loop {
        let resp = client
            .scan()
            .table_name(&table)
            .index_name("ByGsiPk")
            .limit(2)
            .set_exclusive_start_key(start.take())
            .send()
            .await
            .unwrap();
        pages += 1;
        assert!(
            resp.count() > 0,
            "a page must not be emptied by the sparse filter"
        );
        for item in resp.items() {
            assert_eq!(names(item), ["gsi_pk", "pk", "sk", "wanted"]);
            seen.push(sk(item));
        }
        match resp.last_evaluated_key {
            Some(lek) => {
                assert_eq!(names(&lek), ["gsi_pk", "pk", "sk"]);
                start = Some(lek);
            }
            None => break,
        }
        assert!(pages < 5, "pagination did not terminate");
    }
    seen.sort();
    assert_eq!(seen, ["a", "b", "c"]);
    assert_eq!(pages, 2);

    // The base table scan pages too, and keeps every attribute.
    let first = client
        .scan()
        .table_name(&table)
        .limit(3)
        .send()
        .await
        .unwrap();
    assert_eq!(first.count(), 3);
    let lek = first.last_evaluated_key.clone().expect("one item remains");
    assert_eq!(names(&lek), ["pk", "sk"]);
    let second = client
        .scan()
        .table_name(&table)
        .limit(3)
        .set_exclusive_start_key(Some(lek))
        .send()
        .await
        .unwrap();
    assert_eq!(second.count(), 1);
    assert!(second.last_evaluated_key.is_none());
    assert!(
        first
            .items()
            .iter()
            .chain(second.items())
            .all(|i| i.contains_key("secret"))
    );

    delete_table(&client, &table).await;
}

/// Paging a query on a GSI without a sort key (every item shares the index
/// key) visits each item once, and KEYS_ONLY drops non-key attributes.
#[tokio::test]
async fn query_hash_only_gsi_pages_and_projects() {
    let Some(client) = client().await else { return };
    let table = unique_table("redis-query-gsi");
    create_projection_table(&client, &table).await;

    let mut seen = Vec::new();
    let mut start = None;
    for _ in 0..5 {
        let resp = client
            .query()
            .table_name(&table)
            .index_name("ByGsiKeys")
            .key_condition_expression("gsi_pk = :g")
            .expression_attribute_values(":g", AttributeValue::S("g".into()))
            .limit(1)
            .set_exclusive_start_key(start.take())
            .send()
            .await
            .unwrap();
        for item in resp.items() {
            assert_eq!(names(item), ["gsi_pk", "pk", "sk"]);
            seen.push(sk(item));
        }
        match resp.last_evaluated_key {
            Some(lek) => start = Some(lek),
            None => break,
        }
    }
    seen.sort();
    assert_eq!(seen, ["a", "b", "c"]);

    delete_table(&client, &table).await;
}

/// An LSI query skips items without the LSI sort key.
#[tokio::test]
async fn query_lsi_is_sparse() {
    let Some(client) = client().await else { return };
    let table = unique_table("redis-query-lsi");
    create_projection_table(&client, &table).await;

    for (pk, expected) in [("p1", vec!["a", "b"]), ("p2", vec![])] {
        let resp = client
            .query()
            .table_name(&table)
            .index_name("ByLsi")
            .key_condition_expression("pk = :p")
            .expression_attribute_values(":p", AttributeValue::S(pk.into()))
            .send()
            .await
            .unwrap();
        let seen: Vec<String> = resp.items().iter().map(sk).collect();
        assert_eq!(seen, expected, "{pk}");
        for item in resp.items() {
            assert_eq!(names(item), ["lsi_sk", "pk", "sk", "wanted"]);
        }
    }

    delete_table(&client, &table).await;
}

#[tokio::test]
async fn unknown_index_is_a_validation_error() {
    let Some(client) = client().await else { return };
    let table = unique_table("redis-unknown-index");
    create_projection_table(&client, &table).await;

    let err = client
        .scan()
        .table_name(&table)
        .index_name("Missing")
        .send()
        .await
        .unwrap_err();
    assert_eq!(err.code(), Some("ValidationException"), "{err:?}");
    let err = client
        .query()
        .table_name(&table)
        .index_name("Missing")
        .key_condition_expression("pk = :p")
        .expression_attribute_values(":p", AttributeValue::S("p1".into()))
        .send()
        .await
        .unwrap_err();
    assert_eq!(err.code(), Some("ValidationException"), "{err:?}");

    delete_table(&client, &table).await;
}

/// Numeric sort keys order numerically and exactly: two 20-digit keys that
/// are equal as `f64` stay distinct, so paging neither repeats nor skips
/// one of them.
#[tokio::test]
async fn query_orders_numeric_sort_keys_exactly() {
    let Some(client) = client().await else { return };
    let table = unique_table("redis-numeric-sk");
    let key = |name: &str, key_type: KeyType| {
        KeySchemaElement::builder()
            .attribute_name(name)
            .key_type(key_type)
            .build()
            .unwrap()
    };
    let attr = |name: &str, attribute_type: ScalarAttributeType| {
        AttributeDefinition::builder()
            .attribute_name(name)
            .attribute_type(attribute_type)
            .build()
            .unwrap()
    };
    client
        .create_table()
        .table_name(&table)
        .billing_mode(BillingMode::PayPerRequest)
        .key_schema(key("pk", KeyType::Hash))
        .key_schema(key("n", KeyType::Range))
        .attribute_definitions(attr("pk", ScalarAttributeType::S))
        .attribute_definitions(attr("n", ScalarAttributeType::N))
        .send()
        .await
        .unwrap();
    let ascending = [
        "-1.5",
        "9",
        "10",
        "12345678901234567890",
        "12345678901234567891",
    ];
    for n in ascending.iter().rev() {
        client
            .put_item()
            .table_name(&table)
            .item("pk", AttributeValue::S("p".into()))
            .item("n", AttributeValue::N(n.to_string()))
            .send()
            .await
            .unwrap();
    }

    for forward in [true, false] {
        let mut seen: Vec<String> = Vec::new();
        let mut start = None;
        for _ in 0..10 {
            let resp = client
                .query()
                .table_name(&table)
                .key_condition_expression("pk = :p")
                .expression_attribute_values(":p", AttributeValue::S("p".into()))
                .scan_index_forward(forward)
                .limit(1)
                .set_exclusive_start_key(start.take())
                .send()
                .await
                .unwrap();
            seen.extend(resp.items().iter().map(|i| i["n"].as_n().unwrap().clone()));
            match resp.last_evaluated_key {
                Some(lek) => start = Some(lek),
                None => break,
            }
        }
        let mut expected = ascending.to_vec();
        if !forward {
            expected.reverse();
        }
        assert_eq!(seen, expected, "forward={forward}");
    }

    delete_table(&client, &table).await;
}

/// Binary sort keys order by unsigned bytes, not by their base64 text,
/// which puts 0xd0 ("0A==") before 0x68 ("aA==") and 0xff ("/w==") before
/// 0x00 ("AA==").
#[tokio::test]
async fn query_orders_binary_sort_keys_by_bytes() {
    use aws_sdk_dynamodb::primitives::Blob;
    let Some(client) = client().await else { return };
    let table = unique_table("redis-binary-sk");
    let key = |name: &str, key_type: KeyType| {
        KeySchemaElement::builder()
            .attribute_name(name)
            .key_type(key_type)
            .build()
            .unwrap()
    };
    let attr = |name: &str, attribute_type: ScalarAttributeType| {
        AttributeDefinition::builder()
            .attribute_name(name)
            .attribute_type(attribute_type)
            .build()
            .unwrap()
    };
    client
        .create_table()
        .table_name(&table)
        .billing_mode(BillingMode::PayPerRequest)
        .key_schema(key("pk", KeyType::Hash))
        .key_schema(key("b", KeyType::Range))
        .attribute_definitions(attr("pk", ScalarAttributeType::S))
        .attribute_definitions(attr("b", ScalarAttributeType::B))
        .send()
        .await
        .unwrap();
    let ascending: [&[u8]; 6] = [&[0x00], &[0x01], &[0x01, 0x02], &[0x68], &[0xd0], &[0xff]];
    for bytes in ascending.iter().rev() {
        client
            .put_item()
            .table_name(&table)
            .item("pk", AttributeValue::S("p".into()))
            .item("b", AttributeValue::B(Blob::new(*bytes)))
            .send()
            .await
            .unwrap();
    }

    let mut seen: Vec<Vec<u8>> = Vec::new();
    let mut start = None;
    for _ in 0..10 {
        let resp = client
            .query()
            .table_name(&table)
            .key_condition_expression("pk = :p")
            .expression_attribute_values(":p", AttributeValue::S("p".into()))
            .limit(2)
            .set_exclusive_start_key(start.take())
            .send()
            .await
            .unwrap();
        seen.extend(
            resp.items()
                .iter()
                .map(|i| i["b"].as_b().unwrap().as_ref().to_vec()),
        );
        match resp.last_evaluated_key {
            Some(lek) => start = Some(lek),
            None => break,
        }
    }
    let expected: Vec<Vec<u8>> = ascending.iter().map(|b| b.to_vec()).collect();
    assert_eq!(seen, expected);

    delete_table(&client, &table).await;
}
