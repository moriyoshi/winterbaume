use std::sync::Arc;

use aws_sdk_dynamodb::config::BehaviorVersion as DynamoBehaviorVersion;
use aws_sdk_dynamodbstreams::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_dynamodb::{DynamoDbService, InMemoryDynamoDbBackend};
use winterbaume_dynamodbstreams::DynamoDbStreamsService;

/// Build a MockAws with a shared DynamoDB + DynamoDBStreams backend.
/// Returns (dynamodb client, streams client).
async fn make_clients() -> (
    aws_sdk_dynamodb::Client,
    aws_sdk_dynamodbstreams::Client,
    MockAws,
) {
    let backend = Arc::new(InMemoryDynamoDbBackend::new());
    let dynamodb_svc = DynamoDbService::with_backend(backend.clone());
    let streams_svc = DynamoDbStreamsService::with_dynamodb_backend(backend.clone());

    let mock = MockAws::builder()
        .with_service(dynamodb_svc)
        .with_service(streams_svc)
        .build();

    let ddb_config = aws_config::defaults(DynamoBehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dynamodb::config::Region::new("us-east-1"))
        .load()
        .await;
    let ddb = aws_sdk_dynamodb::Client::new(&ddb_config);

    let streams_config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dynamodbstreams::config::Region::new("us-east-1"))
        .load()
        .await;
    let streams = aws_sdk_dynamodbstreams::Client::new(&streams_config);

    (ddb, streams, mock)
}

/// Create a table with streaming enabled and return its stream ARN.
async fn create_streaming_table(
    ddb: &aws_sdk_dynamodb::Client,
    table_name: &str,
    view_type: &str,
) -> String {
    let resp = ddb
        .create_table()
        .table_name(table_name)
        .key_schema(
            aws_sdk_dynamodb::types::KeySchemaElement::builder()
                .attribute_name("pk")
                .key_type(aws_sdk_dynamodb::types::KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            aws_sdk_dynamodb::types::AttributeDefinition::builder()
                .attribute_name("pk")
                .attribute_type(aws_sdk_dynamodb::types::ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .stream_specification(
            aws_sdk_dynamodb::types::StreamSpecification::builder()
                .stream_enabled(true)
                .stream_view_type(aws_sdk_dynamodb::types::StreamViewType::from(view_type))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_table should succeed");

    resp.table_description()
        .expect("should have table_description")
        .latest_stream_arn()
        .expect("should have latest_stream_arn")
        .to_string()
}

// ============================================================================
// ListStreams
// ============================================================================

#[tokio::test]
async fn test_list_streams_empty() {
    let (_, streams, _) = make_clients().await;

    let resp = streams
        .list_streams()
        .send()
        .await
        .expect("list_streams should succeed");

    assert!(resp.streams().is_empty(), "no tables with streaming yet");
}

#[tokio::test]
async fn test_list_streams_reflects_tables() {
    let (ddb, streams, _) = make_clients().await;

    create_streaming_table(&ddb, "my-table", "NEW_AND_OLD_IMAGES").await;

    let resp = streams
        .list_streams()
        .send()
        .await
        .expect("list_streams should succeed");

    assert_eq!(resp.streams().len(), 1, "should have one stream");
    let s = &resp.streams()[0];
    assert_eq!(s.table_name(), Some("my-table"));
    assert!(s.stream_arn().is_some(), "stream_arn should be present");
}

#[tokio::test]
async fn test_list_streams_filter_by_table_name() {
    let (ddb, streams, _) = make_clients().await;

    create_streaming_table(&ddb, "table-a", "NEW_IMAGE").await;
    create_streaming_table(&ddb, "table-b", "NEW_IMAGE").await;

    let resp = streams
        .list_streams()
        .table_name("table-a")
        .send()
        .await
        .expect("list_streams with table_name filter should succeed");

    assert_eq!(resp.streams().len(), 1);
    assert_eq!(resp.streams()[0].table_name(), Some("table-a"));
}

#[tokio::test]
async fn test_list_streams_filter_no_match() {
    let (_, streams, _) = make_clients().await;

    let resp = streams
        .list_streams()
        .table_name("nonexistent-table")
        .send()
        .await
        .expect("list_streams with nonexistent table should succeed");

    assert!(resp.streams().is_empty());
}

// ============================================================================
// DescribeStream
// ============================================================================

#[tokio::test]
async fn test_describe_stream_after_create_table() {
    let (ddb, streams, _) = make_clients().await;

    let stream_arn = create_streaming_table(&ddb, "orders", "NEW_AND_OLD_IMAGES").await;

    let resp = streams
        .describe_stream()
        .stream_arn(&stream_arn)
        .send()
        .await
        .expect("describe_stream should succeed");

    let desc = resp
        .stream_description()
        .expect("should have stream_description");

    assert_eq!(desc.stream_arn(), Some(stream_arn.as_str()));
    assert_eq!(desc.table_name(), Some("orders"));
    assert_eq!(desc.stream_status().map(|s| s.as_str()), Some("ENABLED"));
    assert_eq!(
        desc.stream_view_type().map(|s| s.as_str()),
        Some("NEW_AND_OLD_IMAGES")
    );
    assert!(desc.stream_label().is_some());
    assert!(!desc.shards().is_empty(), "should have at least one shard");
}

#[tokio::test]
async fn test_describe_stream_not_found() {
    let (_, streams, _) = make_clients().await;

    let result = streams
        .describe_stream()
        .stream_arn("arn:aws:dynamodb:us-east-1:123456789012:table/no-such/stream/label")
        .send()
        .await;

    assert!(result.is_err(), "should fail for unknown stream ARN");
    let err = format!("{:?}", result.unwrap_err());
    assert!(
        err.contains("ResourceNotFoundException"),
        "should be ResourceNotFoundException, got: {err}"
    );
}

#[tokio::test]
async fn test_describe_stream_missing_arn() {
    let (_, streams, _) = make_clients().await;

    let result = streams.describe_stream().send().await;
    assert!(result.is_err(), "should fail without stream_arn");
}

#[tokio::test]
async fn test_describe_stream_key_schema() {
    let (ddb, streams, _) = make_clients().await;

    let stream_arn = create_streaming_table(&ddb, "keyed-table", "KEYS_ONLY").await;

    let resp = streams
        .describe_stream()
        .stream_arn(&stream_arn)
        .send()
        .await
        .unwrap();

    let desc = resp.stream_description().unwrap();
    let key_schema = desc.key_schema();
    assert!(!key_schema.is_empty(), "should include key_schema");
    let hash_key = key_schema
        .iter()
        .find(|k| *k.key_type() == aws_sdk_dynamodbstreams::types::KeyType::Hash);
    assert!(hash_key.is_some(), "should have a HASH key");
    assert_eq!(hash_key.unwrap().attribute_name(), "pk");
}

// ============================================================================
// GetShardIterator / GetRecords
// ============================================================================

#[tokio::test]
async fn test_get_shard_iterator_trim_horizon() {
    let (ddb, streams, _) = make_clients().await;

    let stream_arn = create_streaming_table(&ddb, "iter-table", "NEW_IMAGE").await;

    let desc_resp = streams
        .describe_stream()
        .stream_arn(&stream_arn)
        .send()
        .await
        .unwrap();

    let shard_id = desc_resp
        .stream_description()
        .unwrap()
        .shards()
        .first()
        .unwrap()
        .shard_id()
        .unwrap()
        .to_string();

    let resp = streams
        .get_shard_iterator()
        .stream_arn(&stream_arn)
        .shard_id(&shard_id)
        .shard_iterator_type(aws_sdk_dynamodbstreams::types::ShardIteratorType::TrimHorizon)
        .send()
        .await
        .expect("get_shard_iterator should succeed");

    let iterator = resp.shard_iterator().expect("should have shard_iterator");
    assert!(!iterator.is_empty());
}

#[tokio::test]
async fn test_get_shard_iterator_unknown_stream() {
    let (_, streams, _) = make_clients().await;

    let result = streams
        .get_shard_iterator()
        .stream_arn("arn:aws:dynamodb:us-east-1:123456789012:table/no-table/stream/label")
        .shard_id("shardId-000000000000")
        .shard_iterator_type(aws_sdk_dynamodbstreams::types::ShardIteratorType::TrimHorizon)
        .send()
        .await;

    assert!(result.is_err(), "should fail for unknown stream");
}

#[tokio::test]
async fn test_get_records_after_create_table() {
    let (ddb, streams, _) = make_clients().await;

    let stream_arn = create_streaming_table(&ddb, "records-table", "NEW_AND_OLD_IMAGES").await;

    let desc_resp = streams
        .describe_stream()
        .stream_arn(&stream_arn)
        .send()
        .await
        .unwrap();

    let shard_id = desc_resp
        .stream_description()
        .unwrap()
        .shards()
        .first()
        .unwrap()
        .shard_id()
        .unwrap()
        .to_string();

    let iter_resp = streams
        .get_shard_iterator()
        .stream_arn(&stream_arn)
        .shard_id(&shard_id)
        .shard_iterator_type(aws_sdk_dynamodbstreams::types::ShardIteratorType::TrimHorizon)
        .send()
        .await
        .unwrap();

    let shard_iterator = iter_resp.shard_iterator().unwrap().to_string();

    let records_resp = streams
        .get_records()
        .shard_iterator(&shard_iterator)
        .send()
        .await
        .expect("get_records should succeed");

    // Records are empty (write-capture not yet implemented)
    assert!(records_resp.records().is_empty());
    assert!(records_resp.next_shard_iterator().is_some());
}

#[tokio::test]
async fn test_get_records_invalid_iterator() {
    let (_, streams, _) = make_clients().await;

    let err = streams
        .get_records()
        .shard_iterator("this-iterator-does-not-exist-at-all")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ExpiredIteratorException"),
        "Expected ExpiredIteratorException, got: {err_str}"
    );
}

// ============================================================================
// Full lifecycle
// ============================================================================

#[tokio::test]
async fn test_full_lifecycle() {
    let (ddb, streams, _) = make_clients().await;

    // No streams before any tables are created
    let list_resp = streams.list_streams().send().await.unwrap();
    assert!(list_resp.streams().is_empty());

    // Create table with streaming
    let stream_arn = create_streaming_table(&ddb, "lifecycle-table", "NEW_AND_OLD_IMAGES").await;

    // ListStreams now returns the stream
    let list_resp = streams.list_streams().send().await.unwrap();
    assert_eq!(list_resp.streams().len(), 1);
    assert_eq!(list_resp.streams()[0].table_name(), Some("lifecycle-table"));

    // DescribeStream
    let desc_resp = streams
        .describe_stream()
        .stream_arn(&stream_arn)
        .send()
        .await
        .unwrap();
    let desc = desc_resp.stream_description().unwrap();
    assert_eq!(desc.table_name(), Some("lifecycle-table"));
    assert!(!desc.shards().is_empty());

    let shard_id = desc.shards()[0].shard_id().unwrap().to_string();

    // GetShardIterator
    let iter_resp = streams
        .get_shard_iterator()
        .stream_arn(&stream_arn)
        .shard_id(&shard_id)
        .shard_iterator_type(aws_sdk_dynamodbstreams::types::ShardIteratorType::TrimHorizon)
        .send()
        .await
        .unwrap();
    let shard_iterator = iter_resp.shard_iterator().unwrap().to_string();

    // GetRecords
    let records_resp = streams
        .get_records()
        .shard_iterator(&shard_iterator)
        .send()
        .await
        .unwrap();
    assert!(records_resp.records().is_empty());
    assert!(records_resp.next_shard_iterator().is_some());
}

// ============================================================================
// UpdateTable stream specification
// ============================================================================

#[tokio::test]
async fn test_enable_stream_via_update_table() {
    let (ddb, streams, _) = make_clients().await;

    // Create table without streaming
    ddb.create_table()
        .table_name("no-stream-table")
        .key_schema(
            aws_sdk_dynamodb::types::KeySchemaElement::builder()
                .attribute_name("pk")
                .key_type(aws_sdk_dynamodb::types::KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            aws_sdk_dynamodb::types::AttributeDefinition::builder()
                .attribute_name("pk")
                .attribute_type(aws_sdk_dynamodb::types::ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .send()
        .await
        .unwrap();

    // No streams yet
    let list = streams.list_streams().send().await.unwrap();
    assert!(list.streams().is_empty());

    // Enable streaming via UpdateTable
    let update_resp = ddb
        .update_table()
        .table_name("no-stream-table")
        .stream_specification(
            aws_sdk_dynamodb::types::StreamSpecification::builder()
                .stream_enabled(true)
                .stream_view_type(aws_sdk_dynamodb::types::StreamViewType::NewImage)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_table should succeed");

    let desc = update_resp.table_description().unwrap();
    assert!(
        desc.latest_stream_arn().is_some(),
        "stream ARN should appear after enabling"
    );

    // Stream now visible
    let list = streams.list_streams().send().await.unwrap();
    assert_eq!(list.streams().len(), 1);
}

// ============================================================================
// Stream record capture (write paths)
// ============================================================================

/// Helper: obtain a shard iterator for the first shard of `stream_arn`.
async fn trim_horizon_iterator(
    streams: &aws_sdk_dynamodbstreams::Client,
    stream_arn: &str,
) -> String {
    let desc = streams
        .describe_stream()
        .stream_arn(stream_arn)
        .send()
        .await
        .unwrap();
    let shard_id = desc
        .stream_description()
        .unwrap()
        .shards()
        .first()
        .unwrap()
        .shard_id()
        .unwrap()
        .to_string();

    streams
        .get_shard_iterator()
        .stream_arn(stream_arn)
        .shard_id(&shard_id)
        .shard_iterator_type(aws_sdk_dynamodbstreams::types::ShardIteratorType::TrimHorizon)
        .send()
        .await
        .unwrap()
        .shard_iterator()
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn test_put_item_produces_insert_record() {
    let (ddb, streams, _) = make_clients().await;

    let stream_arn = create_streaming_table(&ddb, "put-table", "NEW_IMAGE").await;
    let iterator = trim_horizon_iterator(&streams, &stream_arn).await;

    // Write one item
    ddb.put_item()
        .table_name("put-table")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("key1".into()),
        )
        .item(
            "value",
            aws_sdk_dynamodb::types::AttributeValue::S("hello".into()),
        )
        .send()
        .await
        .expect("put_item should succeed");

    let resp = streams
        .get_records()
        .shard_iterator(&iterator)
        .send()
        .await
        .expect("get_records should succeed");

    let records = resp.records();
    assert_eq!(records.len(), 1, "should have 1 stream record");

    let rec = &records[0];
    assert_eq!(rec.event_name().map(|s| s.as_str()), Some("INSERT"));
    assert_eq!(rec.event_source(), Some("aws:dynamodb"));

    let ddb_rec = rec.dynamodb().expect("should have dynamodb field");
    assert!(ddb_rec.keys().is_some(), "keys should be present");
    assert!(
        ddb_rec.new_image().is_some(),
        "new_image should be present for NEW_IMAGE stream"
    );
    assert!(
        ddb_rec.old_image().is_none(),
        "old_image should be absent for INSERT"
    );
}

#[tokio::test]
async fn test_put_item_overwrite_produces_modify_record() {
    let (ddb, streams, _) = make_clients().await;

    let stream_arn = create_streaming_table(&ddb, "modify-table", "NEW_AND_OLD_IMAGES").await;

    // First write (INSERT)
    ddb.put_item()
        .table_name("modify-table")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .send()
        .await
        .unwrap();

    let iterator = trim_horizon_iterator(&streams, &stream_arn).await;

    // Second write (MODIFY — same key)
    ddb.put_item()
        .table_name("modify-table")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("k1".into()),
        )
        .item(
            "extra",
            aws_sdk_dynamodb::types::AttributeValue::S("new".into()),
        )
        .send()
        .await
        .unwrap();

    // The iterator was obtained after the first write, so it starts after seq 0.
    // Drain all records from TRIM_HORIZON so we get both.
    let resp = streams
        .get_records()
        .shard_iterator(&iterator)
        .send()
        .await
        .unwrap();

    let records = resp.records();
    assert_eq!(records.len(), 2, "should have INSERT + MODIFY");

    assert_eq!(records[0].event_name().map(|s| s.as_str()), Some("INSERT"));
    assert_eq!(records[1].event_name().map(|s| s.as_str()), Some("MODIFY"));

    // MODIFY should carry both old and new images
    let modify_ddb = records[1].dynamodb().unwrap();
    assert!(
        modify_ddb.old_image().is_some(),
        "MODIFY should have old_image"
    );
    assert!(
        modify_ddb.new_image().is_some(),
        "MODIFY should have new_image"
    );
}

#[tokio::test]
async fn test_delete_item_produces_remove_record() {
    let (ddb, streams, _) = make_clients().await;

    let stream_arn = create_streaming_table(&ddb, "delete-table", "OLD_IMAGE").await;

    ddb.put_item()
        .table_name("delete-table")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("del1".into()),
        )
        .send()
        .await
        .unwrap();

    let iterator = trim_horizon_iterator(&streams, &stream_arn).await;

    ddb.delete_item()
        .table_name("delete-table")
        .key(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("del1".into()),
        )
        .send()
        .await
        .expect("delete_item should succeed");

    let resp = streams
        .get_records()
        .shard_iterator(&iterator)
        .send()
        .await
        .unwrap();

    let records = resp.records();
    assert_eq!(records.len(), 2, "INSERT + REMOVE");

    let remove = records
        .iter()
        .find(|r| r.event_name().map(|s| s.as_str()) == Some("REMOVE"));
    assert!(remove.is_some(), "should have a REMOVE record");

    let ddb_rec = remove.unwrap().dynamodb().unwrap();
    assert!(
        ddb_rec.old_image().is_some(),
        "REMOVE should have old_image"
    );
}

#[tokio::test]
async fn test_get_records_pagination_via_next_iterator() {
    let (ddb, streams, _) = make_clients().await;

    let stream_arn = create_streaming_table(&ddb, "page-table", "NEW_IMAGE").await;
    let iterator = trim_horizon_iterator(&streams, &stream_arn).await;

    // Write 3 items
    for i in 0..3_u32 {
        ddb.put_item()
            .table_name("page-table")
            .item(
                "pk",
                aws_sdk_dynamodb::types::AttributeValue::S(format!("pk{i}")),
            )
            .send()
            .await
            .unwrap();
    }

    // Fetch 2 at a time
    let resp1 = streams
        .get_records()
        .shard_iterator(&iterator)
        .limit(2)
        .send()
        .await
        .unwrap();
    assert_eq!(resp1.records().len(), 2);
    let next = resp1.next_shard_iterator().unwrap().to_string();

    // Fetch remaining
    let resp2 = streams
        .get_records()
        .shard_iterator(&next)
        .send()
        .await
        .unwrap();
    assert_eq!(resp2.records().len(), 1);
}

#[tokio::test]
async fn test_no_records_for_table_without_streaming() {
    let (ddb, streams, _) = make_clients().await;

    // Create table WITHOUT streaming
    ddb.create_table()
        .table_name("no-stream")
        .key_schema(
            aws_sdk_dynamodb::types::KeySchemaElement::builder()
                .attribute_name("pk")
                .key_type(aws_sdk_dynamodb::types::KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            aws_sdk_dynamodb::types::AttributeDefinition::builder()
                .attribute_name("pk")
                .attribute_type(aws_sdk_dynamodb::types::ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .send()
        .await
        .unwrap();

    // Write some items
    for i in 0..3_u32 {
        ddb.put_item()
            .table_name("no-stream")
            .item(
                "pk",
                aws_sdk_dynamodb::types::AttributeValue::S(format!("k{i}")),
            )
            .send()
            .await
            .unwrap();
    }

    // No streams should be listed
    let list = streams.list_streams().send().await.unwrap();
    assert!(list.streams().is_empty(), "no streaming enabled");
}

// ============================================================================
// Coverage tests (use `client` variable name for coverage detection)
// ============================================================================

/// Helper: build a streams-only client named `client` for coverage detection.
async fn make_coverage_clients() -> (
    aws_sdk_dynamodb::Client,
    aws_sdk_dynamodbstreams::Client,
    MockAws,
) {
    make_clients().await
}

#[tokio::test]
async fn test_coverage_list_streams() {
    let (ddb, client, _mock) = make_coverage_clients().await;

    // Empty initially
    let resp = client.list_streams().send().await.unwrap();
    assert!(resp.streams().is_empty());

    // Create a table with streaming, then list again
    create_streaming_table(&ddb, "cov-list-table", "NEW_IMAGE").await;
    let resp = client.list_streams().send().await.unwrap();
    assert_eq!(resp.streams().len(), 1);
    assert_eq!(resp.streams()[0].table_name(), Some("cov-list-table"));
}

#[tokio::test]
async fn test_coverage_describe_stream() {
    let (ddb, client, _mock) = make_coverage_clients().await;

    let stream_arn = create_streaming_table(&ddb, "cov-desc-table", "NEW_AND_OLD_IMAGES").await;

    let resp = client
        .describe_stream()
        .stream_arn(&stream_arn)
        .send()
        .await
        .unwrap();

    let desc = resp.stream_description().unwrap();
    assert_eq!(desc.table_name(), Some("cov-desc-table"));
    assert_eq!(desc.stream_status().map(|s| s.as_str()), Some("ENABLED"));
    assert!(!desc.shards().is_empty());
}

#[tokio::test]
async fn test_coverage_get_shard_iterator() {
    let (ddb, client, _mock) = make_coverage_clients().await;

    let stream_arn = create_streaming_table(&ddb, "cov-iter-table", "NEW_IMAGE").await;

    let desc = client
        .describe_stream()
        .stream_arn(&stream_arn)
        .send()
        .await
        .unwrap();
    let shard_id = desc
        .stream_description()
        .unwrap()
        .shards()
        .first()
        .unwrap()
        .shard_id()
        .unwrap()
        .to_string();

    let resp = client
        .get_shard_iterator()
        .stream_arn(&stream_arn)
        .shard_id(&shard_id)
        .shard_iterator_type(aws_sdk_dynamodbstreams::types::ShardIteratorType::TrimHorizon)
        .send()
        .await
        .unwrap();

    assert!(resp.shard_iterator().is_some());
    assert!(!resp.shard_iterator().unwrap().is_empty());
}

#[tokio::test]
async fn test_coverage_get_records() {
    let (ddb, client, _mock) = make_coverage_clients().await;

    let stream_arn = create_streaming_table(&ddb, "cov-rec-table", "NEW_AND_OLD_IMAGES").await;

    let desc = client
        .describe_stream()
        .stream_arn(&stream_arn)
        .send()
        .await
        .unwrap();
    let shard_id = desc
        .stream_description()
        .unwrap()
        .shards()
        .first()
        .unwrap()
        .shard_id()
        .unwrap()
        .to_string();

    let iter_resp = client
        .get_shard_iterator()
        .stream_arn(&stream_arn)
        .shard_id(&shard_id)
        .shard_iterator_type(aws_sdk_dynamodbstreams::types::ShardIteratorType::TrimHorizon)
        .send()
        .await
        .unwrap();
    let shard_iterator = iter_resp.shard_iterator().unwrap().to_string();

    // Write an item so there's something to read
    ddb.put_item()
        .table_name("cov-rec-table")
        .item(
            "pk",
            aws_sdk_dynamodb::types::AttributeValue::S("rec1".into()),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .get_records()
        .shard_iterator(&shard_iterator)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.records().len(), 1);
    assert_eq!(
        resp.records()[0].event_name().map(|s| s.as_str()),
        Some("INSERT")
    );
    assert!(resp.next_shard_iterator().is_some());
}
