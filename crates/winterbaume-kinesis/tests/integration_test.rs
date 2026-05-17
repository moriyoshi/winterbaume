use aws_sdk_kinesis::config::BehaviorVersion;
use aws_sdk_kinesis::primitives::Blob;
use winterbaume_core::MockAws;
use winterbaume_kinesis::KinesisService;

async fn make_kinesis_client() -> aws_sdk_kinesis::Client {
    let mock = MockAws::builder()
        .with_service(KinesisService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kinesis::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_kinesis::Client::new(&config)
}

#[tokio::test]
async fn test_create_stream() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("test-stream")
        .shard_count(1)
        .send()
        .await
        .expect("create_stream should succeed");
}

#[tokio::test]
async fn test_describe_stream() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("desc-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("desc-stream")
        .send()
        .await
        .expect("describe_stream should succeed");

    let desc = resp
        .stream_description()
        .expect("should have stream description");
    assert_eq!(desc.stream_name(), "desc-stream");
    assert_eq!(
        desc.stream_status(),
        &aws_sdk_kinesis::types::StreamStatus::Active
    );
    assert_eq!(desc.shards().len(), 2);
}

#[tokio::test]
async fn test_list_streams() {
    let client = make_kinesis_client().await;

    for name in ["stream-a", "stream-b", "stream-c"] {
        client
            .create_stream()
            .stream_name(name)
            .shard_count(1)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_streams()
        .send()
        .await
        .expect("list_streams should succeed");

    assert_eq!(resp.stream_names().len(), 3);
}

#[tokio::test]
async fn test_delete_stream() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("del-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    client
        .delete_stream()
        .stream_name("del-stream")
        .send()
        .await
        .expect("delete_stream should succeed");

    let result = client
        .describe_stream()
        .stream_name("del-stream")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_record() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("put-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let resp = client
        .put_record()
        .stream_name("put-stream")
        .data(Blob::new(b"hello world".to_vec()))
        .partition_key("key1")
        .send()
        .await
        .expect("put_record should succeed");

    assert!(!resp.sequence_number().is_empty());
    assert!(!resp.shard_id().is_empty());
}

#[tokio::test]
async fn test_put_records() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("putmulti-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let resp = client
        .put_records()
        .stream_name("putmulti-stream")
        .records(
            aws_sdk_kinesis::types::PutRecordsRequestEntry::builder()
                .data(Blob::new(b"record1".to_vec()))
                .partition_key("key1")
                .build()
                .unwrap(),
        )
        .records(
            aws_sdk_kinesis::types::PutRecordsRequestEntry::builder()
                .data(Blob::new(b"record2".to_vec()))
                .partition_key("key2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_records should succeed");

    assert_eq!(resp.failed_record_count(), Some(0));
    assert_eq!(resp.records().len(), 2);
}

#[tokio::test]
async fn test_create_duplicate_stream_fails() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("dup-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let result = client
        .create_stream()
        .stream_name("dup-stream")
        .shard_count(1)
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_record_to_nonexistent_stream_fails() {
    let client = make_kinesis_client().await;

    let result = client
        .put_record()
        .stream_name("nonexistent-stream")
        .data(Blob::new(b"data".to_vec()))
        .partition_key("key1")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_shard_iterator_and_records() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("getrecords-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    // Put a record
    client
        .put_record()
        .stream_name("getrecords-stream")
        .data(Blob::new(b"hello kinesis".to_vec()))
        .partition_key("pk1")
        .send()
        .await
        .expect("put_record should succeed");

    // Describe the stream to get the shard ID
    let desc = client
        .describe_stream()
        .stream_name("getrecords-stream")
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
        .to_string();

    // Get a shard iterator
    let iter_resp = client
        .get_shard_iterator()
        .stream_name("getrecords-stream")
        .shard_id(&shard_id)
        .shard_iterator_type(aws_sdk_kinesis::types::ShardIteratorType::TrimHorizon)
        .send()
        .await
        .expect("get_shard_iterator should succeed");

    let shard_iterator = iter_resp
        .shard_iterator()
        .expect("should have shard iterator");

    // Get records using that iterator
    let records_resp = client
        .get_records()
        .shard_iterator(shard_iterator)
        .send()
        .await
        .expect("get_records should succeed");

    let records = records_resp.records();
    assert!(!records.is_empty(), "should have at least one record");
    assert_eq!(records[0].partition_key(), "pk1");
}

#[tokio::test]
async fn test_add_and_list_tags_for_stream() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("tag-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    client
        .add_tags_to_stream()
        .stream_name("tag-stream")
        .tags("env", "prod")
        .tags("team", "backend")
        .send()
        .await
        .expect("add_tags_to_stream should succeed");

    let resp = client
        .list_tags_for_stream()
        .stream_name("tag-stream")
        .send()
        .await
        .expect("list_tags_for_stream should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 2);
}

#[tokio::test]
async fn test_remove_tags_from_stream() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("rmtag-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    client
        .add_tags_to_stream()
        .stream_name("rmtag-stream")
        .tags("env", "prod")
        .tags("team", "backend")
        .send()
        .await
        .unwrap();

    client
        .remove_tags_from_stream()
        .stream_name("rmtag-stream")
        .tag_keys("env")
        .send()
        .await
        .expect("remove_tags_from_stream should succeed");

    let resp = client
        .list_tags_for_stream()
        .stream_name("rmtag-stream")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "team");
}

#[tokio::test]
async fn test_register_and_describe_stream_consumer() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("consumer-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let stream_arn = {
        let desc = client
            .describe_stream()
            .stream_name("consumer-stream")
            .send()
            .await
            .unwrap();
        desc.stream_description().unwrap().stream_arn().to_string()
    };

    let reg_resp = client
        .register_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_name("my-consumer")
        .send()
        .await
        .expect("register_stream_consumer should succeed");

    let consumer = reg_resp.consumer().unwrap();
    assert_eq!(consumer.consumer_name(), "my-consumer");
    assert_eq!(
        consumer.consumer_status(),
        &aws_sdk_kinesis::types::ConsumerStatus::Active
    );

    // Describe the consumer
    let desc_resp = client
        .describe_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_name("my-consumer")
        .send()
        .await
        .expect("describe_stream_consumer should succeed");

    let desc = desc_resp.consumer_description().unwrap();
    assert_eq!(desc.consumer_name(), "my-consumer");
}

#[tokio::test]
async fn test_list_stream_consumers() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("list-consumer-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let stream_arn = {
        let desc = client
            .describe_stream()
            .stream_name("list-consumer-stream")
            .send()
            .await
            .unwrap();
        desc.stream_description().unwrap().stream_arn().to_string()
    };

    client
        .register_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_name("consumer-a")
        .send()
        .await
        .unwrap();

    client
        .register_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_name("consumer-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_stream_consumers()
        .stream_arn(&stream_arn)
        .send()
        .await
        .expect("list_stream_consumers should succeed");

    assert_eq!(resp.consumers().len(), 2);
}

#[tokio::test]
async fn test_deregister_stream_consumer() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("dereg-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let stream_arn = {
        let desc = client
            .describe_stream()
            .stream_name("dereg-stream")
            .send()
            .await
            .unwrap();
        desc.stream_description().unwrap().stream_arn().to_string()
    };

    let reg_resp = client
        .register_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_name("temp-consumer")
        .send()
        .await
        .unwrap();

    let consumer_arn = reg_resp.consumer().unwrap().consumer_arn().to_string();

    client
        .deregister_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_arn(&consumer_arn)
        .send()
        .await
        .expect("deregister_stream_consumer should succeed");

    let resp = client
        .list_stream_consumers()
        .stream_arn(&stream_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.consumers().len(), 0);
}

#[tokio::test]
async fn test_increase_stream_retention_period() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("retention-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    client
        .increase_stream_retention_period()
        .stream_name("retention-stream")
        .retention_period_hours(48)
        .send()
        .await
        .expect("increase_stream_retention_period should succeed");

    let desc = client
        .describe_stream()
        .stream_name("retention-stream")
        .send()
        .await
        .unwrap();

    assert_eq!(
        desc.stream_description().unwrap().retention_period_hours(),
        48
    );
}

#[tokio::test]
async fn test_decrease_stream_retention_period() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("dec-retention-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    // First increase
    client
        .increase_stream_retention_period()
        .stream_name("dec-retention-stream")
        .retention_period_hours(72)
        .send()
        .await
        .unwrap();

    // Then decrease
    client
        .decrease_stream_retention_period()
        .stream_name("dec-retention-stream")
        .retention_period_hours(48)
        .send()
        .await
        .expect("decrease_stream_retention_period should succeed");

    let desc = client
        .describe_stream()
        .stream_name("dec-retention-stream")
        .send()
        .await
        .unwrap();

    assert_eq!(
        desc.stream_description().unwrap().retention_period_hours(),
        48
    );
}

#[tokio::test]
async fn test_start_and_stop_stream_encryption() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("enc-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    client
        .start_stream_encryption()
        .stream_name("enc-stream")
        .encryption_type(aws_sdk_kinesis::types::EncryptionType::Kms)
        .key_id("arn:aws:kms:us-east-1:123456789012:key/test-key")
        .send()
        .await
        .expect("start_stream_encryption should succeed");

    client
        .stop_stream_encryption()
        .stream_name("enc-stream")
        .encryption_type(aws_sdk_kinesis::types::EncryptionType::Kms)
        .key_id("arn:aws:kms:us-east-1:123456789012:key/test-key")
        .send()
        .await
        .expect("stop_stream_encryption should succeed");
}

#[tokio::test]
async fn test_enable_and_disable_enhanced_monitoring() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("monitor-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let enable_resp = client
        .enable_enhanced_monitoring()
        .stream_name("monitor-stream")
        .shard_level_metrics(aws_sdk_kinesis::types::MetricsName::IncomingBytes)
        .shard_level_metrics(aws_sdk_kinesis::types::MetricsName::OutgoingBytes)
        .send()
        .await
        .expect("enable_enhanced_monitoring should succeed");

    // current = metrics before the change (empty), desired = metrics after (2)
    assert_eq!(enable_resp.current_shard_level_metrics().len(), 0);
    assert_eq!(enable_resp.desired_shard_level_metrics().len(), 2);

    let disable_resp = client
        .disable_enhanced_monitoring()
        .stream_name("monitor-stream")
        .shard_level_metrics(aws_sdk_kinesis::types::MetricsName::IncomingBytes)
        .send()
        .await
        .expect("disable_enhanced_monitoring should succeed");

    // current = metrics before the change (2), desired = metrics after (1)
    assert_eq!(disable_resp.current_shard_level_metrics().len(), 2);
    assert_eq!(disable_resp.desired_shard_level_metrics().len(), 1);
}

#[tokio::test]
async fn test_describe_stream_summary() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("summary-stream")
        .shard_count(3)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream_summary()
        .stream_name("summary-stream")
        .send()
        .await
        .expect("describe_stream_summary should succeed");

    let summary = resp.stream_description_summary().unwrap();
    assert_eq!(summary.stream_name(), "summary-stream");
    assert_eq!(summary.open_shard_count(), 3);
}

#[tokio::test]
async fn test_describe_limits() {
    let client = make_kinesis_client().await;

    let resp = client
        .describe_limits()
        .send()
        .await
        .expect("describe_limits should succeed");

    assert_eq!(resp.shard_limit(), 6000);
    assert_eq!(resp.on_demand_stream_count_limit(), 50);
}

#[tokio::test]
async fn test_list_shards() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("shard-stream")
        .shard_count(4)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_shards()
        .stream_name("shard-stream")
        .send()
        .await
        .expect("list_shards should succeed");

    assert_eq!(resp.shards().len(), 4);
}

#[tokio::test]
async fn test_update_shard_count() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("updshard-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_shard_count()
        .stream_name("updshard-stream")
        .target_shard_count(4)
        .scaling_type(aws_sdk_kinesis::types::ScalingType::UniformScaling)
        .send()
        .await
        .expect("update_shard_count should succeed");

    assert_eq!(resp.current_shard_count(), Some(2));
    assert_eq!(resp.target_shard_count(), Some(4));
}

#[tokio::test]
async fn test_merge_shards() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("merge-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    client
        .merge_shards()
        .stream_name("merge-stream")
        .shard_to_merge("shardId-000000000000")
        .adjacent_shard_to_merge("shardId-000000000001")
        .send()
        .await
        .expect("merge_shards should succeed");

    let shards = client
        .list_shards()
        .stream_name("merge-stream")
        .send()
        .await
        .unwrap();

    assert_eq!(shards.shards().len(), 1);
}

#[tokio::test]
async fn test_split_shard() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("split-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    client
        .split_shard()
        .stream_name("split-stream")
        .shard_to_split("shardId-000000000000")
        .new_starting_hash_key("170141183460469231731687303715884105728")
        .send()
        .await
        .expect("split_shard should succeed");

    let shards = client
        .list_shards()
        .stream_name("split-stream")
        .send()
        .await
        .unwrap();

    assert_eq!(shards.shards().len(), 2);
}

#[tokio::test]
async fn test_put_and_get_resource_policy() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("policy-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let stream_arn = {
        let desc = client
            .describe_stream()
            .stream_name("policy-stream")
            .send()
            .await
            .unwrap();
        desc.stream_description().unwrap().stream_arn().to_string()
    };

    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;

    client
        .put_resource_policy()
        .resource_arn(&stream_arn)
        .policy(policy)
        .send()
        .await
        .expect("put_resource_policy should succeed");

    let resp = client
        .get_resource_policy()
        .resource_arn(&stream_arn)
        .send()
        .await
        .expect("get_resource_policy should succeed");

    assert_eq!(resp.policy(), policy);
}

#[tokio::test]
async fn test_delete_resource_policy() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("delpol-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let stream_arn = {
        let desc = client
            .describe_stream()
            .stream_name("delpol-stream")
            .send()
            .await
            .unwrap();
        desc.stream_description().unwrap().stream_arn().to_string()
    };

    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;

    client
        .put_resource_policy()
        .resource_arn(&stream_arn)
        .policy(policy)
        .send()
        .await
        .unwrap();

    client
        .delete_resource_policy()
        .resource_arn(&stream_arn)
        .send()
        .await
        .expect("delete_resource_policy should succeed");

    let resp = client
        .get_resource_policy()
        .resource_arn(&stream_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.policy(), "{}");
}

#[tokio::test]
async fn test_update_stream_mode() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("mode-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let stream_arn = {
        let desc = client
            .describe_stream()
            .stream_name("mode-stream")
            .send()
            .await
            .unwrap();
        desc.stream_description().unwrap().stream_arn().to_string()
    };

    client
        .update_stream_mode()
        .stream_arn(&stream_arn)
        .stream_mode_details(
            aws_sdk_kinesis::types::StreamModeDetails::builder()
                .stream_mode(aws_sdk_kinesis::types::StreamMode::OnDemand)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_stream_mode should succeed");

    let summary = client
        .describe_stream_summary()
        .stream_name("mode-stream")
        .send()
        .await
        .unwrap();

    let mode_details = summary
        .stream_description_summary()
        .unwrap()
        .stream_mode_details()
        .unwrap();
    assert_eq!(
        mode_details.stream_mode(),
        &aws_sdk_kinesis::types::StreamMode::OnDemand
    );
}

#[tokio::test]
async fn test_stream_lifecycle() {
    let client = make_kinesis_client().await;

    // Create
    client
        .create_stream()
        .stream_name("lifecycle-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    // Describe
    let desc = client
        .describe_stream()
        .stream_name("lifecycle-stream")
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.stream_description().unwrap().stream_name(),
        "lifecycle-stream"
    );

    // Delete
    client
        .delete_stream()
        .stream_name("lifecycle-stream")
        .send()
        .await
        .unwrap();

    // Verify gone
    let result = client
        .describe_stream()
        .stream_name("lifecycle-stream")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Ported from moto: test_kinesis.py, test_kinesis_boto3.py,
// test_kinesis_encryption.py, test_kinesis_monitoring.py,
// test_kinesis_resource_policy.py, test_kinesis_stream_consumers.py,
// test_kinesis_stream_limits.py
// ============================================================================

async fn get_stream_arn(client: &aws_sdk_kinesis::Client, stream_name: &str) -> String {
    let desc = client
        .describe_stream()
        .stream_name(stream_name)
        .send()
        .await
        .unwrap();
    desc.stream_description().unwrap().stream_arn().to_string()
}

// --- Streams / Shards ---

// Ported from moto: test_kinesis_boto3.py::test_create_shard
#[tokio::test]
async fn test_create_shard_details() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("my-stream")
        .send()
        .await
        .unwrap();
    let desc = resp.stream_description().unwrap();
    assert_eq!(desc.stream_name(), "my-stream");
    assert_eq!(
        desc.stream_arn(),
        "arn:aws:kinesis:us-east-1:123456789012:stream/my-stream"
    );
    assert_eq!(desc.shards().len(), 2);
    assert_eq!(
        desc.stream_status(),
        &aws_sdk_kinesis::types::StreamStatus::Active
    );
    assert!(!desc.has_more_shards());
    assert_eq!(desc.retention_period_hours(), 24);
    // stream_creation_timestamp is a required field, just verify it's non-zero
    let _ = desc.stream_creation_timestamp();
    // EnhancedMonitoring should be a single entry with empty ShardLevelMetrics
    assert_eq!(desc.enhanced_monitoring().len(), 1);
    assert_eq!(desc.enhanced_monitoring()[0].shard_level_metrics().len(), 0);
    // EncryptionType should not be present (NONE)
    assert!(
        desc.encryption_type().is_none()
            || desc.encryption_type() == Some(&aws_sdk_kinesis::types::EncryptionType::None)
    );

    let shards = desc.shards();
    assert_eq!(shards[0].shard_id(), "shardId-000000000000");
    assert!(shards[0].hash_key_range().is_some());
    assert_eq!(shards[0].hash_key_range().unwrap().starting_hash_key(), "0");
    assert!(shards[0].sequence_number_range().is_some());
    assert!(
        !shards[0]
            .sequence_number_range()
            .unwrap()
            .starting_sequence_number()
            .is_empty()
    );
    // Active shard should NOT have ending sequence number
    assert!(
        shards[0]
            .sequence_number_range()
            .unwrap()
            .ending_sequence_number()
            .is_none()
    );
}

// Ported from moto: test_kinesis_boto3.py::test_describe_stream_limit_parameter
#[tokio::test]
async fn test_describe_stream_limit_parameter() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .shard_count(5)
        .send()
        .await
        .unwrap();

    // Without limit
    let resp = client
        .describe_stream()
        .stream_name("my_stream")
        .send()
        .await
        .unwrap();
    let desc = resp.stream_description().unwrap();
    assert_eq!(desc.shards().len(), 5);
    assert!(!desc.has_more_shards());

    // With limit=2
    let resp = client
        .describe_stream()
        .stream_name("my_stream")
        .limit(2)
        .send()
        .await
        .unwrap();
    let desc = resp.stream_description().unwrap();
    assert_eq!(desc.shards().len(), 2);
    assert!(desc.has_more_shards());

    // With limit=5 (exactly matching)
    let resp = client
        .describe_stream()
        .stream_name("my_stream")
        .limit(5)
        .send()
        .await
        .unwrap();
    let desc = resp.stream_description().unwrap();
    assert_eq!(desc.shards().len(), 5);
    assert!(!desc.has_more_shards());

    // With limit=6 (more than available)
    let resp = client
        .describe_stream()
        .stream_name("my_stream")
        .limit(6)
        .send()
        .await
        .unwrap();
    let desc = resp.stream_description().unwrap();
    assert_eq!(desc.shards().len(), 5);
    assert!(!desc.has_more_shards());
}

// Ported from moto: test_kinesis_boto3.py::test_list_shards (with hash key range verification)
#[tokio::test]
async fn test_list_shards_with_hash_ranges() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_shards()
        .stream_name("my_stream")
        .send()
        .await
        .unwrap();
    let shards = resp.shards();
    assert_eq!(shards.len(), 2);

    // Verify IDs
    assert_eq!(shards[0].shard_id(), "shardId-000000000000");
    assert_eq!(shards[1].shard_id(), "shardId-000000000001");

    // Verify hash ranges exist
    for shard in shards {
        assert!(shard.hash_key_range().is_some());
        let hkr = shard.hash_key_range().unwrap();
        assert!(!hkr.starting_hash_key().is_empty());
        assert!(!hkr.ending_hash_key().is_empty());
    }

    // Verify contiguous: shard[0].ending + 1 == shard[1].starting
    let s0_end: u128 = shards[0]
        .hash_key_range()
        .unwrap()
        .ending_hash_key()
        .parse()
        .unwrap();
    let s1_start: u128 = shards[1]
        .hash_key_range()
        .unwrap()
        .starting_hash_key()
        .parse()
        .unwrap();
    assert_eq!(s0_end + 1, s1_start);

    // Verify sequence numbers
    for shard in shards {
        assert!(shard.sequence_number_range().is_some());
        assert!(
            !shard
                .sequence_number_range()
                .unwrap()
                .starting_sequence_number()
                .is_empty()
        );
    }
}

// Ported from moto: test_kinesis_boto3.py::test_list_shards_paging
#[tokio::test]
async fn test_list_shards_paging() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .shard_count(10)
        .send()
        .await
        .unwrap();

    // Get all 10 shards
    let resp = client
        .list_shards()
        .stream_name("my_stream")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.shards().len(), 10);
    assert!(resp.next_token().is_none());

    // Get first 4
    let resp = client
        .list_shards()
        .stream_name("my_stream")
        .max_results(4)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.shards().len(), 4);
    assert_eq!(resp.shards()[0].shard_id(), "shardId-000000000000");
    assert_eq!(resp.shards()[3].shard_id(), "shardId-000000000003");
    assert!(resp.next_token().is_some());
    let token1 = resp.next_token().unwrap().to_string();

    // Get next 4
    let resp = client
        .list_shards()
        .stream_name("my_stream")
        .max_results(4)
        .next_token(&token1)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.shards().len(), 4);
    assert_eq!(resp.shards()[0].shard_id(), "shardId-000000000004");
    assert_eq!(resp.shards()[3].shard_id(), "shardId-000000000007");
    assert!(resp.next_token().is_some());
    let token2 = resp.next_token().unwrap().to_string();

    // Get last 2
    let resp = client
        .list_shards()
        .stream_name("my_stream")
        .max_results(4)
        .next_token(&token2)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.shards().len(), 2);
    assert_eq!(resp.shards()[0].shard_id(), "shardId-000000000008");
    assert_eq!(resp.shards()[1].shard_id(), "shardId-000000000009");
    assert!(resp.next_token().is_none());
}

// Ported from moto: test_kinesis.py::test_describe_non_existent_stream
#[tokio::test]
async fn test_describe_non_existent_stream() {
    let client = make_kinesis_client().await;

    let err = client
        .describe_stream_summary()
        .stream_name("not-a-stream")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_kinesis.py::test_delete_unknown_stream
#[tokio::test]
async fn test_delete_unknown_stream() {
    let client = make_kinesis_client().await;

    let err = client
        .delete_stream()
        .stream_name("not-a-stream")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// --- Split shard tests ---

// Ported from moto: test_kinesis_boto3.py::test_split_shard (proper shard tracking)
#[tokio::test]
async fn test_split_shard_detailed() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let original = client
        .describe_stream()
        .stream_name("my-stream")
        .send()
        .await
        .unwrap();
    let _original_shards = original.stream_description().unwrap().shards();

    client
        .split_shard()
        .stream_name("my-stream")
        .shard_to_split("shardId-000000000001")
        .new_starting_hash_key("170141183460469231731687303715884105829")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("my-stream")
        .send()
        .await
        .unwrap();
    let desc = resp.stream_description().unwrap();
    let shards = desc.shards();
    assert_eq!(shards.len(), 4);

    // First shard unchanged
    assert_eq!(shards[0].shard_id(), "shardId-000000000000");
    assert!(shards[0].parent_shard_id().is_none());

    // Second shard (parent) now closed
    assert_eq!(shards[1].shard_id(), "shardId-000000000001");
    assert!(shards[1].parent_shard_id().is_none());
    assert!(
        shards[1]
            .sequence_number_range()
            .unwrap()
            .ending_sequence_number()
            .is_some()
    );

    // Third shard (child 1) has parent
    assert_eq!(shards[2].shard_id(), "shardId-000000000002");
    assert_eq!(shards[2].parent_shard_id(), Some("shardId-000000000001"));
    assert!(
        shards[2]
            .sequence_number_range()
            .unwrap()
            .ending_sequence_number()
            .is_none()
    );

    // Fourth shard (child 2) has parent
    assert_eq!(shards[3].shard_id(), "shardId-000000000003");
    assert_eq!(shards[3].parent_shard_id(), Some("shardId-000000000001"));
    assert!(
        shards[3]
            .sequence_number_range()
            .unwrap()
            .ending_sequence_number()
            .is_none()
    );
}

// Ported from moto: test_kinesis_boto3.py::test_split_shard_with_invalid_name
#[tokio::test]
async fn test_split_shard_with_invalid_name() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let err = client
        .split_shard()
        .stream_name("my-stream")
        .shard_to_split("?")
        .new_starting_hash_key("170141183460469231731687303715884105728")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ValidationException"),
        "Expected ValidationException, got: {err_str}"
    );
}

// Ported from moto: test_kinesis_boto3.py::test_split_shard_with_unknown_name
#[tokio::test]
async fn test_split_shard_with_unknown_name() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let err = client
        .split_shard()
        .stream_name("my-stream")
        .shard_to_split("unknown")
        .new_starting_hash_key("170141183460469231731687303715884105728")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_kinesis_boto3.py::test_split_shard_invalid_hashkey
#[tokio::test]
async fn test_split_shard_invalid_hashkey() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let err = client
        .split_shard()
        .stream_name("my-stream")
        .shard_to_split("shardId-000000000001")
        .new_starting_hash_key("sth")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ValidationException"),
        "Expected ValidationException, got: {err_str}"
    );
}

// Ported from moto: test_kinesis_boto3.py::test_split_shard_that_was_split_before
#[tokio::test]
async fn test_split_shard_that_was_split_before() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    client
        .split_shard()
        .stream_name("my-stream")
        .shard_to_split("shardId-000000000001")
        .new_starting_hash_key("170141183460469231731687303715884105829")
        .send()
        .await
        .unwrap();

    // Second split of same shard should fail
    let err = client
        .split_shard()
        .stream_name("my-stream")
        .shard_to_split("shardId-000000000001")
        .new_starting_hash_key("170141183460469231731687303715884105829")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgument"),
        "Expected InvalidArgumentException, got: {err_str}"
    );
}

// --- Merge shard tests ---

// Ported from moto: test_kinesis.py::test_merge_shards (detailed)
#[tokio::test]
async fn test_merge_shards_detailed() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .shard_count(4)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_stream()
        .stream_name("my_stream")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.stream_description().unwrap().shards().len(), 4);

    // Merge first two shards
    client
        .merge_shards()
        .stream_name("my_stream")
        .shard_to_merge("shardId-000000000000")
        .adjacent_shard_to_merge("shardId-000000000001")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_stream()
        .stream_name("my_stream")
        .send()
        .await
        .unwrap();
    let shards = desc.stream_description().unwrap().shards();

    // Old shards still exist, but are closed. A new shard is created.
    assert_eq!(shards.len(), 5);

    // Count active shards (no ending sequence number)
    let active_shards: Vec<_> = shards
        .iter()
        .filter(|s| {
            s.sequence_number_range()
                .unwrap()
                .ending_sequence_number()
                .is_none()
        })
        .collect();
    assert_eq!(active_shards.len(), 3);
}

// Ported from moto: test_kinesis.py::test_merge_shards_invalid_arg
#[tokio::test]
async fn test_merge_shards_invalid_arg() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .shard_count(4)
        .send()
        .await
        .unwrap();

    // Try to merge non-adjacent shards (0 and 2)
    let err = client
        .merge_shards()
        .stream_name("my_stream")
        .shard_to_merge("shardId-000000000000")
        .adjacent_shard_to_merge("shardId-000000000002")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgument"),
        "Expected InvalidArgumentException, got: {err_str}"
    );
}

// --- Update shard count tests ---

// Ported from moto: test_kinesis_boto3.py::test_update_shard_count (parametrized)
#[tokio::test]
async fn test_update_shard_count_scale_up_2_to_4() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_shard_count()
        .stream_name("my-stream")
        .target_shard_count(4)
        .scaling_type(aws_sdk_kinesis::types::ScalingType::UniformScaling)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.stream_name(), Some("my-stream"));
    assert_eq!(resp.current_shard_count(), Some(2));
    assert_eq!(resp.target_shard_count(), Some(4));

    let desc = client
        .describe_stream()
        .stream_name("my-stream")
        .send()
        .await
        .unwrap();
    let stream = desc.stream_description().unwrap();
    assert_eq!(
        stream.stream_status(),
        &aws_sdk_kinesis::types::StreamStatus::Active
    );

    // Count active shards
    let active_shards: Vec<_> = stream
        .shards()
        .iter()
        .filter(|s| {
            s.sequence_number_range()
                .unwrap()
                .ending_sequence_number()
                .is_none()
        })
        .collect();
    assert_eq!(active_shards.len(), 4);

    // Verify open shard count via describe_stream_summary
    let summary = client
        .describe_stream_summary()
        .stream_name("my-stream")
        .send()
        .await
        .unwrap();
    assert_eq!(
        summary
            .stream_description_summary()
            .unwrap()
            .open_shard_count(),
        4
    );
}

// Ported from moto: test_kinesis_boto3.py::test_update_shard_count (scale down)
#[tokio::test]
async fn test_update_shard_count_scale_down_4_to_2() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(4)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_shard_count()
        .stream_name("my-stream")
        .target_shard_count(2)
        .scaling_type(aws_sdk_kinesis::types::ScalingType::UniformScaling)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.current_shard_count(), Some(4));
    assert_eq!(resp.target_shard_count(), Some(2));

    let summary = client
        .describe_stream_summary()
        .stream_name("my-stream")
        .send()
        .await
        .unwrap();
    assert_eq!(
        summary
            .stream_description_summary()
            .unwrap()
            .open_shard_count(),
        2
    );
}

// --- Describe limits ---

// Ported from moto: test_kinesis_boto3.py::test_describe_limits
#[tokio::test]
async fn test_describe_limits_detailed() {
    let client = make_kinesis_client().await;

    let resp = client.describe_limits().send().await.unwrap();

    assert_eq!(resp.shard_limit(), 6000);
    assert_eq!(resp.open_shard_count(), 0);
    assert_eq!(resp.on_demand_stream_count(), 0);
    assert_eq!(resp.on_demand_stream_count_limit(), 50);
}

// --- Tags ---

// Ported from moto: test_kinesis.py::test_add_list_remove_tags
#[tokio::test]
async fn test_add_list_remove_tags_detailed() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let stream_arn = get_stream_arn(&client, "my_stream").await;

    // Add 4 tags by name
    client
        .add_tags_to_stream()
        .stream_name("my_stream")
        .tags("tag1", "val1")
        .tags("tag2", "val2")
        .tags("tag3", "val3")
        .tags("tag4", "val4")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_stream()
        .stream_name("my_stream")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 4);

    // Add tag by ARN
    client
        .add_tags_to_stream()
        .stream_arn(&stream_arn)
        .tags("tag5", "val5")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_stream()
        .stream_arn(&stream_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 5);

    // Remove tags by name
    client
        .remove_tags_from_stream()
        .stream_name("my_stream")
        .tag_keys("tag2")
        .tag_keys("tag3")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_stream()
        .stream_name("my_stream")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 3);

    // Remove tag by ARN
    client
        .remove_tags_from_stream()
        .stream_arn(&stream_arn)
        .tag_keys("tag4")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_stream()
        .stream_name("my_stream")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 2);
}

// --- Encryption ---

// Ported from moto: test_kinesis_encryption.py::test_enable_encryption
#[tokio::test]
async fn test_enable_encryption() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("my-stream")
        .send()
        .await
        .unwrap();
    let desc = resp.stream_description().unwrap();
    assert!(
        desc.encryption_type().is_none()
            || desc.encryption_type() == Some(&aws_sdk_kinesis::types::EncryptionType::None)
    );
    assert!(desc.key_id().is_none());

    client
        .start_stream_encryption()
        .stream_name("my-stream")
        .encryption_type(aws_sdk_kinesis::types::EncryptionType::Kms)
        .key_id("n/a")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("my-stream")
        .send()
        .await
        .unwrap();
    let desc = resp.stream_description().unwrap();
    assert_eq!(
        desc.encryption_type(),
        Some(&aws_sdk_kinesis::types::EncryptionType::Kms)
    );
    assert_eq!(desc.key_id(), Some("n/a"));
}

// Ported from moto: test_kinesis_encryption.py::test_disable_encryption
#[tokio::test]
async fn test_disable_encryption() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    client
        .start_stream_encryption()
        .stream_name("my-stream")
        .encryption_type(aws_sdk_kinesis::types::EncryptionType::Kms)
        .key_id("n/a")
        .send()
        .await
        .unwrap();

    client
        .stop_stream_encryption()
        .stream_name("my-stream")
        .encryption_type(aws_sdk_kinesis::types::EncryptionType::Kms)
        .key_id("n/a")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("my-stream")
        .send()
        .await
        .unwrap();
    let desc = resp.stream_description().unwrap();
    assert!(
        desc.encryption_type().is_none()
            || desc.encryption_type() == Some(&aws_sdk_kinesis::types::EncryptionType::None)
    );
    assert!(desc.key_id().is_none());
}

// Ported from moto: test_kinesis_encryption.py::test_disable_encryption__using_arns
#[tokio::test]
async fn test_disable_encryption_using_arns() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let stream_arn = get_stream_arn(&client, "my-stream").await;

    client
        .start_stream_encryption()
        .stream_arn(&stream_arn)
        .encryption_type(aws_sdk_kinesis::types::EncryptionType::Kms)
        .key_id("n/a")
        .send()
        .await
        .unwrap();

    client
        .stop_stream_encryption()
        .stream_arn(&stream_arn)
        .encryption_type(aws_sdk_kinesis::types::EncryptionType::Kms)
        .key_id("n/a")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("my-stream")
        .send()
        .await
        .unwrap();
    let desc = resp.stream_description().unwrap();
    assert!(
        desc.encryption_type().is_none()
            || desc.encryption_type() == Some(&aws_sdk_kinesis::types::EncryptionType::None)
    );
    assert!(desc.key_id().is_none());
}

// --- Enhanced monitoring ---

// Ported from moto: test_kinesis_monitoring.py::test_enable_enhanced_monitoring_all
#[tokio::test]
async fn test_enable_enhanced_monitoring_all() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .shard_count(4)
        .send()
        .await
        .unwrap();

    let resp = client
        .enable_enhanced_monitoring()
        .stream_name("my_stream")
        .shard_level_metrics(aws_sdk_kinesis::types::MetricsName::All)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.stream_name(), Some("my_stream"));
    assert_eq!(resp.current_shard_level_metrics().len(), 0);
    assert_eq!(resp.desired_shard_level_metrics().len(), 1);
    assert!(resp.stream_arn().is_some());
}

// Ported from moto: test_kinesis_monitoring.py::test_enable_enhanced_monitoring_is_persisted
#[tokio::test]
async fn test_enable_enhanced_monitoring_is_persisted() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .shard_count(4)
        .send()
        .await
        .unwrap();

    client
        .enable_enhanced_monitoring()
        .stream_name("my_stream")
        .shard_level_metrics(aws_sdk_kinesis::types::MetricsName::IncomingBytes)
        .shard_level_metrics(aws_sdk_kinesis::types::MetricsName::OutgoingBytes)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_stream()
        .stream_name("my_stream")
        .send()
        .await
        .unwrap();
    let metrics = desc.stream_description().unwrap().enhanced_monitoring();
    assert_eq!(metrics.len(), 1);
    assert_eq!(metrics[0].shard_level_metrics().len(), 2);
}

// Ported from moto: test_kinesis_monitoring.py::test_enable_enhanced_monitoring_in_steps
#[tokio::test]
async fn test_enable_enhanced_monitoring_in_steps() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .shard_count(4)
        .send()
        .await
        .unwrap();

    // First enable 2 metrics
    client
        .enable_enhanced_monitoring()
        .stream_name("my_stream")
        .shard_level_metrics(aws_sdk_kinesis::types::MetricsName::IncomingBytes)
        .shard_level_metrics(aws_sdk_kinesis::types::MetricsName::OutgoingBytes)
        .send()
        .await
        .unwrap();

    // Then enable a third
    let resp = client
        .enable_enhanced_monitoring()
        .stream_name("my_stream")
        .shard_level_metrics(
            aws_sdk_kinesis::types::MetricsName::WriteProvisionedThroughputExceeded,
        )
        .send()
        .await
        .unwrap();

    // current = before (2 metrics), desired = after (3 metrics)
    assert_eq!(resp.current_shard_level_metrics().len(), 2);
    assert_eq!(resp.desired_shard_level_metrics().len(), 3);

    // Verify via describe
    let desc = client
        .describe_stream()
        .stream_name("my_stream")
        .send()
        .await
        .unwrap();
    let metrics = desc.stream_description().unwrap().enhanced_monitoring()[0].shard_level_metrics();
    assert_eq!(metrics.len(), 3);
}

// Ported from moto: test_kinesis_monitoring.py::test_disable_enhanced_monitoring
#[tokio::test]
async fn test_disable_enhanced_monitoring_detailed() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .shard_count(4)
        .send()
        .await
        .unwrap();

    let stream_arn = get_stream_arn(&client, "my_stream").await;

    // Enable 3 metrics via ARN
    client
        .enable_enhanced_monitoring()
        .stream_arn(&stream_arn)
        .shard_level_metrics(aws_sdk_kinesis::types::MetricsName::IncomingBytes)
        .shard_level_metrics(aws_sdk_kinesis::types::MetricsName::OutgoingBytes)
        .shard_level_metrics(
            aws_sdk_kinesis::types::MetricsName::WriteProvisionedThroughputExceeded,
        )
        .send()
        .await
        .unwrap();

    // Disable one
    let resp = client
        .disable_enhanced_monitoring()
        .stream_name("my_stream")
        .shard_level_metrics(aws_sdk_kinesis::types::MetricsName::OutgoingBytes)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.stream_name(), Some("my_stream"));
    assert!(resp.stream_arn().is_some());
    assert_eq!(resp.current_shard_level_metrics().len(), 3);
    assert_eq!(resp.desired_shard_level_metrics().len(), 2);

    // Verify via describe
    let desc = client
        .describe_stream()
        .stream_name("my_stream")
        .send()
        .await
        .unwrap();
    let metrics = desc.stream_description().unwrap().enhanced_monitoring()[0].shard_level_metrics();
    assert_eq!(metrics.len(), 2);
}

// Ported from moto: test_kinesis_monitoring.py::test_disable_enhanced_monitoring_all
#[tokio::test]
async fn test_disable_enhanced_monitoring_all() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .shard_count(4)
        .send()
        .await
        .unwrap();

    client
        .enable_enhanced_monitoring()
        .stream_name("my_stream")
        .shard_level_metrics(aws_sdk_kinesis::types::MetricsName::IncomingBytes)
        .shard_level_metrics(aws_sdk_kinesis::types::MetricsName::OutgoingBytes)
        .shard_level_metrics(
            aws_sdk_kinesis::types::MetricsName::WriteProvisionedThroughputExceeded,
        )
        .send()
        .await
        .unwrap();

    // Disable ALL
    client
        .disable_enhanced_monitoring()
        .stream_name("my_stream")
        .shard_level_metrics(aws_sdk_kinesis::types::MetricsName::All)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_stream()
        .stream_name("my_stream")
        .send()
        .await
        .unwrap();
    let metrics = desc.stream_description().unwrap().enhanced_monitoring()[0].shard_level_metrics();
    assert_eq!(metrics.len(), 0);
}

// --- Retention period ---

// Ported from moto: test_kinesis.py::test_invalid_increase_stream_retention_period
#[tokio::test]
async fn test_invalid_increase_stream_retention_period() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    // Increase to 30
    client
        .increase_stream_retention_period()
        .stream_name("my_stream")
        .retention_period_hours(30)
        .send()
        .await
        .unwrap();

    // Try to increase to 25 (less than current 30) - should fail
    let err = client
        .increase_stream_retention_period()
        .stream_name("my_stream")
        .retention_period_hours(25)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgument"),
        "Expected InvalidArgumentException, got: {err_str}"
    );
}

// Ported from moto: test_kinesis.py::test_invalid_increase_stream_retention_too_low
#[tokio::test]
async fn test_invalid_increase_stream_retention_too_low() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let err = client
        .increase_stream_retention_period()
        .stream_name("my_stream")
        .retention_period_hours(20)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgument"),
        "Expected InvalidArgumentException, got: {err_str}"
    );
}

// Ported from moto: test_kinesis.py::test_invalid_increase_stream_retention_too_high
#[tokio::test]
async fn test_invalid_increase_stream_retention_too_high() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let err = client
        .increase_stream_retention_period()
        .stream_name("my_stream")
        .retention_period_hours(9999)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgument"),
        "Expected InvalidArgumentException, got: {err_str}"
    );
}

// Ported from moto: test_kinesis.py::test_decrease_stream_retention_period_upwards
#[tokio::test]
async fn test_decrease_stream_retention_period_upwards() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("decrease_stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    // Try to decrease to 40 (more than current 24) - should fail
    let err = client
        .decrease_stream_retention_period()
        .stream_name("decrease_stream")
        .retention_period_hours(40)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgument"),
        "Expected InvalidArgumentException, got: {err_str}"
    );
}

// Ported from moto: test_kinesis.py::test_decrease_stream_retention_period_too_low
#[tokio::test]
async fn test_decrease_stream_retention_period_too_low() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("decrease_stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let err = client
        .decrease_stream_retention_period()
        .stream_name("decrease_stream")
        .retention_period_hours(4)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgument"),
        "Expected InvalidArgumentException, got: {err_str}"
    );
}

// Ported from moto: test_kinesis.py::test_decrease_stream_retention_period_too_high
#[tokio::test]
async fn test_decrease_stream_retention_period_too_high() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("decrease_stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let err = client
        .decrease_stream_retention_period()
        .stream_name("decrease_stream")
        .retention_period_hours(9999)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgument"),
        "Expected InvalidArgumentException, got: {err_str}"
    );
}

// --- Stream consumers ---

// Ported from moto: test_kinesis_stream_consumers.py::test_register_stream_consumer (ARN format)
#[tokio::test]
async fn test_register_stream_consumer_arn_format() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(4)
        .send()
        .await
        .unwrap();

    let stream_arn = get_stream_arn(&client, "my-stream").await;

    let resp = client
        .register_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_name("newconsumer")
        .send()
        .await
        .unwrap();

    let consumer = resp.consumer().unwrap();
    assert_eq!(consumer.consumer_name(), "newconsumer");
    assert_eq!(
        consumer.consumer_arn(),
        format!("{}/consumer/newconsumer", stream_arn)
    );
    assert_eq!(
        consumer.consumer_status(),
        &aws_sdk_kinesis::types::ConsumerStatus::Active
    );
    let _ = consumer.consumer_creation_timestamp();

    // Verify via list
    let resp = client
        .list_stream_consumers()
        .stream_arn(&stream_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.consumers().len(), 1);
    assert_eq!(resp.consumers()[0].consumer_name(), "newconsumer");
}

// Ported from moto: test_kinesis_stream_consumers.py::test_describe_stream_consumer_by_name
#[tokio::test]
async fn test_describe_stream_consumer_by_name() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(4)
        .send()
        .await
        .unwrap();

    let stream_arn = get_stream_arn(&client, "my-stream").await;

    client
        .register_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_name("newconsumer")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_name("newconsumer")
        .send()
        .await
        .unwrap();

    let consumer = resp.consumer_description().unwrap();
    assert_eq!(consumer.consumer_name(), "newconsumer");
    assert_eq!(
        consumer.consumer_status(),
        &aws_sdk_kinesis::types::ConsumerStatus::Active
    );
    let _ = consumer.consumer_creation_timestamp();
    assert_eq!(consumer.stream_arn(), stream_arn);
}

// Ported from moto: test_kinesis_stream_consumers.py::test_describe_stream_consumer_by_arn
#[tokio::test]
async fn test_describe_stream_consumer_by_arn() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(4)
        .send()
        .await
        .unwrap();

    let stream_arn = get_stream_arn(&client, "my-stream").await;

    let reg = client
        .register_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_name("newconsumer")
        .send()
        .await
        .unwrap();
    let consumer_arn = reg.consumer().unwrap().consumer_arn().to_string();

    let resp = client
        .describe_stream_consumer()
        .consumer_arn(&consumer_arn)
        .send()
        .await
        .unwrap();

    let consumer = resp.consumer_description().unwrap();
    assert_eq!(consumer.consumer_name(), "newconsumer");
    assert_eq!(consumer.stream_arn(), stream_arn);
}

// Ported from moto: test_kinesis_stream_consumers.py::test_deregister_stream_consumer_by_name
#[tokio::test]
async fn test_deregister_stream_consumer_by_name() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(4)
        .send()
        .await
        .unwrap();

    let stream_arn = get_stream_arn(&client, "my-stream").await;

    client
        .register_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_name("consumer1")
        .send()
        .await
        .unwrap();
    client
        .register_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_name("consumer2")
        .send()
        .await
        .unwrap();

    assert_eq!(
        client
            .list_stream_consumers()
            .stream_arn(&stream_arn)
            .send()
            .await
            .unwrap()
            .consumers()
            .len(),
        2
    );

    client
        .deregister_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_name("consumer1")
        .send()
        .await
        .unwrap();

    assert_eq!(
        client
            .list_stream_consumers()
            .stream_arn(&stream_arn)
            .send()
            .await
            .unwrap()
            .consumers()
            .len(),
        1
    );
}

// Ported from moto: test_kinesis_stream_consumers.py::test_deregister_stream_consumer_by_arn
#[tokio::test]
async fn test_deregister_stream_consumer_by_arn() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(4)
        .send()
        .await
        .unwrap();

    let stream_arn = get_stream_arn(&client, "my-stream").await;

    let resp = client
        .register_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_name("consumer1")
        .send()
        .await
        .unwrap();
    let consumer1_arn = resp.consumer().unwrap().consumer_arn().to_string();

    client
        .register_stream_consumer()
        .stream_arn(&stream_arn)
        .consumer_name("consumer2")
        .send()
        .await
        .unwrap();

    assert_eq!(
        client
            .list_stream_consumers()
            .stream_arn(&stream_arn)
            .send()
            .await
            .unwrap()
            .consumers()
            .len(),
        2
    );

    client
        .deregister_stream_consumer()
        .consumer_arn(&consumer1_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(
        client
            .list_stream_consumers()
            .stream_arn(&stream_arn)
            .send()
            .await
            .unwrap()
            .consumers()
            .len(),
        1
    );
}

// --- Stream mode ---

// Ported from moto: test_kinesis.py::test_stream_creation_on_demand
#[tokio::test]
async fn test_stream_creation_on_demand() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .stream_mode_details(
            aws_sdk_kinesis::types::StreamModeDetails::builder()
                .stream_mode(aws_sdk_kinesis::types::StreamMode::OnDemand)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let stream_arn = get_stream_arn(&client, "my_stream").await;

    // AWS starts with 4 shards by default for on-demand
    let resp = client
        .list_shards()
        .stream_arn(&stream_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.shards().len(), 4);
}

// Ported from moto: test_kinesis.py::test_update_stream_mode
#[tokio::test]
async fn test_update_stream_mode_detailed() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream")
        .stream_mode_details(
            aws_sdk_kinesis::types::StreamModeDetails::builder()
                .stream_mode(aws_sdk_kinesis::types::StreamMode::OnDemand)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let arn = get_stream_arn(&client, "my_stream").await;

    client
        .update_stream_mode()
        .stream_arn(&arn)
        .stream_mode_details(
            aws_sdk_kinesis::types::StreamModeDetails::builder()
                .stream_mode(aws_sdk_kinesis::types::StreamMode::Provisioned)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let summary = client
        .describe_stream_summary()
        .stream_name("my_stream")
        .send()
        .await
        .unwrap();
    assert_eq!(
        summary
            .stream_description_summary()
            .unwrap()
            .stream_mode_details()
            .unwrap()
            .stream_mode(),
        &aws_sdk_kinesis::types::StreamMode::Provisioned
    );
}

// --- Resource policy ---

// Ported from moto: test_kinesis_resource_policy.py::test_get_resource_policy_from_unknown_resource
#[tokio::test]
async fn test_get_resource_policy_from_unknown_resource() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let stream_arn = get_stream_arn(&client, "my-stream").await;
    let bad_arn = format!("{stream_arn}unknown");

    let err = client
        .get_resource_policy()
        .resource_arn(&bad_arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_kinesis_resource_policy.py::test_delete_resource_policy_from_unknown_resource
#[tokio::test]
async fn test_delete_resource_policy_from_unknown_resource() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let stream_arn = get_stream_arn(&client, "my-stream").await;
    let bad_arn = format!("{stream_arn}unknown");

    let err = client
        .delete_resource_policy()
        .resource_arn(&bad_arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_kinesis_resource_policy.py::test_delete_resource_policy_from_resource_without_policy
#[tokio::test]
async fn test_delete_resource_policy_from_resource_without_policy() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let stream_arn = get_stream_arn(&client, "my-stream").await;

    let err = client
        .delete_resource_policy()
        .resource_arn(&stream_arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_kinesis_resource_policy.py::test_delete_resource_policy (get returns "{}" after delete)
#[tokio::test]
async fn test_delete_resource_policy_returns_empty_json() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my-stream")
        .shard_count(2)
        .send()
        .await
        .unwrap();

    let stream_arn = get_stream_arn(&client, "my-stream").await;

    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;
    client
        .put_resource_policy()
        .resource_arn(&stream_arn)
        .policy(policy)
        .send()
        .await
        .unwrap();

    // Verify policy was set
    let resp = client
        .get_resource_policy()
        .resource_arn(&stream_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.policy(), policy);

    // Delete policy
    client
        .delete_resource_policy()
        .resource_arn(&stream_arn)
        .send()
        .await
        .unwrap();

    // After delete, get should return "{}"
    let resp = client
        .get_resource_policy()
        .resource_arn(&stream_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.policy(), "{}");
}

// Ported from moto: test_kinesis.py::test_describe_stream_summary (using ARN)
#[tokio::test]
async fn test_describe_stream_summary_by_arn() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("my_stream_summary")
        .shard_count(5)
        .send()
        .await
        .unwrap();

    let stream_arn = get_stream_arn(&client, "my_stream_summary").await;

    // By name
    let resp = client
        .describe_stream_summary()
        .stream_name("my_stream_summary")
        .send()
        .await
        .unwrap();
    let summary = resp.stream_description_summary().unwrap();
    assert_eq!(summary.stream_name(), "my_stream_summary");
    assert_eq!(summary.open_shard_count(), 5);
    assert_eq!(summary.stream_arn(), stream_arn);

    // By ARN
    let resp = client
        .describe_stream_summary()
        .stream_arn(&stream_arn)
        .send()
        .await
        .unwrap();
    let summary = resp.stream_description_summary().unwrap();
    assert_eq!(summary.stream_name(), "my_stream_summary");
}

// ============================================================================
// Additional tests derived from AWS documentation
// ============================================================================

#[tokio::test]
async fn test_describe_stream_shard_count_single() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("single-shard-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("single-shard-stream")
        .send()
        .await
        .expect("describe_stream should succeed");

    let desc = resp
        .stream_description()
        .expect("should have stream description");
    assert_eq!(desc.shards().len(), 1);
    assert!(!desc.has_more_shards());
}

#[tokio::test]
async fn test_describe_stream_has_arn() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("arn-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("arn-stream")
        .send()
        .await
        .expect("describe_stream should succeed");

    let desc = resp
        .stream_description()
        .expect("should have stream description");
    let arn = desc.stream_arn();
    assert!(
        arn.starts_with("arn:aws:kinesis:"),
        "ARN should start with arn:aws:kinesis:"
    );
    assert!(arn.contains("arn-stream"), "ARN should contain stream name");
}

#[tokio::test]
async fn test_describe_nonexistent_stream_fails() {
    let client = make_kinesis_client().await;

    let result = client
        .describe_stream()
        .stream_name("does-not-exist")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_stream_fails() {
    let client = make_kinesis_client().await;

    let result = client
        .delete_stream()
        .stream_name("never-existed")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_shard_iterator_nonexistent_stream_fails() {
    let client = make_kinesis_client().await;

    let result = client
        .get_shard_iterator()
        .stream_name("no-such-stream")
        .shard_id("shardId-000000000000")
        .shard_iterator_type(aws_sdk_kinesis::types::ShardIteratorType::TrimHorizon)
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_multiple_records_and_get_all() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("multi-get-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let messages = ["alpha", "beta", "gamma", "delta"];
    for msg in &messages {
        client
            .put_record()
            .stream_name("multi-get-stream")
            .data(Blob::new(msg.as_bytes().to_vec()))
            .partition_key("same-key")
            .send()
            .await
            .expect("put_record should succeed");
    }

    let desc = client
        .describe_stream()
        .stream_name("multi-get-stream")
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
        .to_string();

    let iter_resp = client
        .get_shard_iterator()
        .stream_name("multi-get-stream")
        .shard_id(&shard_id)
        .shard_iterator_type(aws_sdk_kinesis::types::ShardIteratorType::TrimHorizon)
        .send()
        .await
        .unwrap();

    let records_resp = client
        .get_records()
        .shard_iterator(iter_resp.shard_iterator().unwrap())
        .send()
        .await
        .expect("get_records should succeed");

    assert_eq!(
        records_resp.records().len(),
        messages.len(),
        "should retrieve all put records"
    );
}

#[tokio::test]
async fn test_put_records_returns_correct_count() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("batch-count-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let entries: Vec<aws_sdk_kinesis::types::PutRecordsRequestEntry> = (0..5)
        .map(|i| {
            aws_sdk_kinesis::types::PutRecordsRequestEntry::builder()
                .data(Blob::new(format!("record-{i}").into_bytes()))
                .partition_key(format!("key-{i}"))
                .build()
                .unwrap()
        })
        .collect();

    let mut builder = client.put_records().stream_name("batch-count-stream");
    for entry in entries {
        builder = builder.records(entry);
    }
    let resp = builder.send().await.expect("put_records should succeed");

    assert_eq!(resp.records().len(), 5);
    assert_eq!(resp.failed_record_count(), Some(0));
}

#[tokio::test]
async fn test_put_records_to_nonexistent_stream_fails() {
    let client = make_kinesis_client().await;

    let result = client
        .put_records()
        .stream_name("ghost-stream")
        .records(
            aws_sdk_kinesis::types::PutRecordsRequestEntry::builder()
                .data(Blob::new(b"data".to_vec()))
                .partition_key("key")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_streams_empty_initially() {
    let client = make_kinesis_client().await;

    let resp = client
        .list_streams()
        .send()
        .await
        .expect("list_streams should succeed on empty state");

    assert_eq!(resp.stream_names().len(), 0);
}

#[tokio::test]
async fn test_list_streams_after_delete() {
    let client = make_kinesis_client().await;

    for name in ["keep-stream", "remove-stream"] {
        client
            .create_stream()
            .stream_name(name)
            .shard_count(1)
            .send()
            .await
            .unwrap();
    }

    client
        .delete_stream()
        .stream_name("remove-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_streams()
        .send()
        .await
        .expect("list_streams should succeed");

    assert_eq!(resp.stream_names().len(), 1);
    assert_eq!(resp.stream_names()[0], "keep-stream");
}

#[tokio::test]
async fn test_get_records_limit_respected() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("limit-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    for i in 0..10 {
        client
            .put_record()
            .stream_name("limit-stream")
            .data(Blob::new(format!("item-{i}").into_bytes()))
            .partition_key("fixed-key")
            .send()
            .await
            .unwrap();
    }

    let desc = client
        .describe_stream()
        .stream_name("limit-stream")
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
        .to_string();

    let iter_resp = client
        .get_shard_iterator()
        .stream_name("limit-stream")
        .shard_id(&shard_id)
        .shard_iterator_type(aws_sdk_kinesis::types::ShardIteratorType::TrimHorizon)
        .send()
        .await
        .unwrap();

    let records_resp = client
        .get_records()
        .shard_iterator(iter_resp.shard_iterator().unwrap())
        .limit(3)
        .send()
        .await
        .expect("get_records with limit should succeed");

    assert_eq!(
        records_resp.records().len(),
        3,
        "limit parameter should constrain returned records"
    );
}

#[tokio::test]
async fn test_put_record_sequence_numbers_are_unique() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("seq-uniq-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let mut sequence_numbers = std::collections::HashSet::new();
    for i in 0..5 {
        let resp = client
            .put_record()
            .stream_name("seq-uniq-stream")
            .data(Blob::new(format!("payload-{i}").into_bytes()))
            .partition_key(format!("pk-{i}"))
            .send()
            .await
            .expect("put_record should succeed");
        sequence_numbers.insert(resp.sequence_number().to_string());
    }

    assert_eq!(
        sequence_numbers.len(),
        5,
        "each record should have a unique sequence number"
    );
}

#[tokio::test]
async fn test_describe_stream_retention_period() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("retention-stream")
        .shard_count(1)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("retention-stream")
        .send()
        .await
        .expect("describe_stream should succeed");

    let desc = resp.stream_description().expect("should have description");
    assert!(
        desc.retention_period_hours() > 0,
        "retention period should be positive"
    );
}

// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================

// Covers FIX(terraform-e2e): DescribeStream returns encryption_type even when NONE
// (handlers.rs:271) — always return encryption_type explicitly (even "NONE") so
// the terraform provider doesn't see a diff between state and refresh.
#[tokio::test]
async fn test_describe_stream_returns_encryption_type() {
    let client = make_kinesis_client().await;

    client
        .create_stream()
        .stream_name("enc-type-stream")
        .shard_count(1)
        .send()
        .await
        .expect("create_stream should succeed");

    let resp = client
        .describe_stream()
        .stream_name("enc-type-stream")
        .send()
        .await
        .expect("describe_stream should succeed");

    let desc = resp
        .stream_description()
        .expect("should have stream description");

    // encryption_type must be explicitly returned (not None), defaulting to NONE
    let enc_type = desc
        .encryption_type()
        .expect("encryption_type should be present even when not encrypted");
    assert_eq!(
        enc_type,
        &aws_sdk_kinesis::types::EncryptionType::None,
        "default encryption_type should be NONE"
    );
}

/// Real AWS Kinesis sequence numbers are monotonic *per shard*, not globally.
/// Regression for the 2026-05-17 fleet-audit finding: a single global counter
/// would make per-shard sequence numbers interleave with other shards' minted
/// values, and a consumer that derives shard ownership from sequence-number
/// ranges would misbehave.
#[tokio::test]
async fn test_put_record_sequence_numbers_are_per_shard() {
    use std::collections::HashMap;

    let client = make_kinesis_client().await;
    client
        .create_stream()
        .stream_name("per-shard-seq")
        .shard_count(3)
        .send()
        .await
        .unwrap();

    // Send 12 records with different partition keys. With 3 shards and
    // hash-based distribution, at least two shards should be hit ( the
    // shard hasher is deterministic so this isn't probabilistic in practice,
    // but the assertion below tolerates any non-degenerate distribution ).
    let mut puts: Vec<(String, u64)> = Vec::new();
    for i in 0..12 {
        let resp = client
            .put_record()
            .stream_name("per-shard-seq")
            .data(Blob::new(format!("rec-{i}").into_bytes()))
            .partition_key(format!("partition-key-{i}"))
            .send()
            .await
            .unwrap();
        let seq: u64 = resp
            .sequence_number()
            .parse()
            .expect("sequence number is numeric");
        puts.push((resp.shard_id().to_string(), seq));
    }

    let mut per_shard: HashMap<String, Vec<u64>> = HashMap::new();
    for (sid, seq) in &puts {
        per_shard.entry(sid.clone()).or_default().push(*seq);
    }

    assert!(
        per_shard.len() >= 2,
        "12 records across 3 shards must hit at least two shards; got {per_shard:?}"
    );

    // Each shard's per-shard sequence stream must start at 1 and increment
    // by 1 in put order.
    for (shard, seqs) in &per_shard {
        for (i, seq) in seqs.iter().enumerate() {
            assert_eq!(
                *seq,
                (i + 1) as u64,
                "sequence numbers in shard {shard} must be 1, 2, 3, ... (got {seqs:?})"
            );
        }
    }
}
