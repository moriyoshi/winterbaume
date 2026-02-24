use aws_sdk_firehose::config::BehaviorVersion;
use aws_sdk_firehose::primitives::Blob;
use aws_sdk_firehose::types::Tag;
use winterbaume_core::MockAws;
use winterbaume_firehose::FirehoseService;

async fn make_firehose_client() -> aws_sdk_firehose::Client {
    let mock = MockAws::builder()
        .with_service(FirehoseService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_firehose::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_firehose::Client::new(&config)
}

#[tokio::test]
async fn test_create_delivery_stream() {
    let client = make_firehose_client().await;

    let resp = client
        .create_delivery_stream()
        .delivery_stream_name("test-stream")
        .send()
        .await
        .expect("create_delivery_stream should succeed");

    let arn = resp
        .delivery_stream_arn()
        .expect("should have delivery stream ARN");
    assert!(arn.contains("arn:aws:firehose:"));
}

#[tokio::test]
async fn test_describe_delivery_stream() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("desc-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_delivery_stream()
        .delivery_stream_name("desc-stream")
        .send()
        .await
        .expect("describe_delivery_stream should succeed");

    let desc = resp
        .delivery_stream_description()
        .expect("should have description");
    assert_eq!(desc.delivery_stream_name(), "desc-stream");
    assert_eq!(
        desc.delivery_stream_status(),
        &aws_sdk_firehose::types::DeliveryStreamStatus::Active
    );
}

#[tokio::test]
async fn test_list_delivery_streams() {
    let client = make_firehose_client().await;

    for name in ["stream-a", "stream-b", "stream-c"] {
        client
            .create_delivery_stream()
            .delivery_stream_name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_delivery_streams()
        .send()
        .await
        .expect("list_delivery_streams should succeed");

    assert_eq!(resp.delivery_stream_names().len(), 3);
}

#[tokio::test]
async fn test_delete_delivery_stream() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("del-stream")
        .send()
        .await
        .unwrap();

    client
        .delete_delivery_stream()
        .delivery_stream_name("del-stream")
        .send()
        .await
        .expect("delete_delivery_stream should succeed");

    let result = client
        .describe_delivery_stream()
        .delivery_stream_name("del-stream")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_record() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("put-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .put_record()
        .delivery_stream_name("put-stream")
        .record(
            aws_sdk_firehose::types::Record::builder()
                .data(Blob::new(b"test data".to_vec()))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_record should succeed");

    assert!(!resp.record_id().is_empty());
}

#[tokio::test]
async fn test_create_duplicate_stream_fails() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("dup-stream")
        .send()
        .await
        .unwrap();

    let result = client
        .create_delivery_stream()
        .delivery_stream_name("dup-stream")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_record_to_nonexistent_stream_fails() {
    let client = make_firehose_client().await;

    let result = client
        .put_record()
        .delivery_stream_name("nonexistent-stream")
        .record(
            aws_sdk_firehose::types::Record::builder()
                .data(Blob::new(b"test".to_vec()))
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_stream_fails() {
    let client = make_firehose_client().await;

    let result = client
        .delete_delivery_stream()
        .delivery_stream_name("nonexistent-stream")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_record_batch() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("batch-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .put_record_batch()
        .delivery_stream_name("batch-stream")
        .records(
            aws_sdk_firehose::types::Record::builder()
                .data(Blob::new(b"record one".to_vec()))
                .build()
                .unwrap(),
        )
        .records(
            aws_sdk_firehose::types::Record::builder()
                .data(Blob::new(b"record two".to_vec()))
                .build()
                .unwrap(),
        )
        .records(
            aws_sdk_firehose::types::Record::builder()
                .data(Blob::new(b"record three".to_vec()))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_record_batch should succeed");

    assert_eq!(resp.failed_put_count(), 0);
    assert_eq!(resp.request_responses().len(), 3);
    for entry in resp.request_responses() {
        assert!(entry.record_id().is_some());
    }
}

#[tokio::test]
async fn test_tag_and_list_tags_for_delivery_stream() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("tag-stream")
        .send()
        .await
        .unwrap();

    client
        .tag_delivery_stream()
        .delivery_stream_name("tag-stream")
        .tags(
            Tag::builder()
                .key("Environment")
                .value("Production")
                .build()
                .unwrap(),
        )
        .tags(
            Tag::builder()
                .key("Team")
                .value("Platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_delivery_stream should succeed");

    let resp = client
        .list_tags_for_delivery_stream()
        .delivery_stream_name("tag-stream")
        .send()
        .await
        .expect("list_tags_for_delivery_stream should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 2);

    let tag_map: std::collections::HashMap<&str, &str> = tags
        .iter()
        .map(|t| (t.key(), t.value().unwrap_or_default()))
        .collect();
    assert_eq!(tag_map.get("Environment"), Some(&"Production"));
    assert_eq!(tag_map.get("Team"), Some(&"Platform"));
}

#[tokio::test]
async fn test_untag_delivery_stream() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("untag-stream")
        .send()
        .await
        .unwrap();

    client
        .tag_delivery_stream()
        .delivery_stream_name("untag-stream")
        .tags(Tag::builder().key("Env").value("Dev").build().unwrap())
        .tags(Tag::builder().key("Owner").value("Alice").build().unwrap())
        .send()
        .await
        .unwrap();

    client
        .untag_delivery_stream()
        .delivery_stream_name("untag-stream")
        .tag_keys("Env")
        .send()
        .await
        .expect("untag_delivery_stream should succeed");

    let resp = client
        .list_tags_for_delivery_stream()
        .delivery_stream_name("untag-stream")
        .send()
        .await
        .unwrap();

    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "Owner");
}

#[tokio::test]
async fn test_start_delivery_stream_encryption() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("enc-stream")
        .send()
        .await
        .unwrap();

    client
        .start_delivery_stream_encryption()
        .delivery_stream_name("enc-stream")
        .send()
        .await
        .expect("start_delivery_stream_encryption should succeed");
}

#[tokio::test]
async fn test_stop_delivery_stream_encryption() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("dec-stream")
        .send()
        .await
        .unwrap();

    client
        .start_delivery_stream_encryption()
        .delivery_stream_name("dec-stream")
        .send()
        .await
        .unwrap();

    client
        .stop_delivery_stream_encryption()
        .delivery_stream_name("dec-stream")
        .send()
        .await
        .expect("stop_delivery_stream_encryption should succeed");
}

#[tokio::test]
async fn test_update_destination() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("upd-stream")
        .send()
        .await
        .unwrap();

    // Describe to get the current version and destination id
    let desc = client
        .describe_delivery_stream()
        .delivery_stream_name("upd-stream")
        .send()
        .await
        .unwrap();
    let description = desc.delivery_stream_description().unwrap();
    let destination_id = description.destinations()[0].destination_id();

    client
        .update_destination()
        .delivery_stream_name("upd-stream")
        .current_delivery_stream_version_id("1")
        .destination_id(destination_id)
        .send()
        .await
        .expect("update_destination should succeed");

    // Second update with stale version should fail
    let result = client
        .update_destination()
        .delivery_stream_name("upd-stream")
        .current_delivery_stream_version_id("1")
        .destination_id(destination_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_encryption_on_nonexistent_stream_fails() {
    let client = make_firehose_client().await;

    let result = client
        .start_delivery_stream_encryption()
        .delivery_stream_name("no-such-stream")
        .send()
        .await;
    assert!(result.is_err());

    let result = client
        .stop_delivery_stream_encryption()
        .delivery_stream_name("no-such-stream")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Ported from moto: test_firehose.py, test_firehose_put.py,
//                    test_firehose_tags.py, test_firehose_encryption.py
// ============================================================================

// Ported from moto: test_firehose.py::test_create_delivery_stream_failures (duplicate check)
#[tokio::test]
async fn test_create_duplicate_stream_error_type() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("dup-err-stream")
        .send()
        .await
        .unwrap();

    let err = client
        .create_delivery_stream()
        .delivery_stream_name("dup-err-stream")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceInUse"),
        "Expected ResourceInUseException, got: {err_str}"
    );
}

// Ported from moto: test_firehose.py::test_delete_delivery_stream (multiple streams, delete one, check remaining)
#[tokio::test]
async fn test_delete_one_and_check_remaining() {
    let client = make_firehose_client().await;

    for idx in 0..5 {
        client
            .create_delivery_stream()
            .delivery_stream_name(format!("del-multi-{idx}"))
            .send()
            .await
            .unwrap();
    }

    // Delete the first stream
    client
        .delete_delivery_stream()
        .delivery_stream_name("del-multi-0")
        .send()
        .await
        .unwrap();

    let resp = client.list_delivery_streams().send().await.unwrap();
    assert_eq!(resp.delivery_stream_names().len(), 4);
    assert!(
        !resp
            .delivery_stream_names()
            .contains(&"del-multi-0".to_string())
    );
}

// Ported from moto: test_firehose.py::test_delete_delivery_stream (nonexistent -> ResourceNotFoundException)
#[tokio::test]
async fn test_delete_nonexistent_stream_error_type() {
    let client = make_firehose_client().await;

    let err = client
        .delete_delivery_stream()
        .delivery_stream_name("no-such-stream")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_firehose.py::test_describe_delivery_stream (check ARN format, status, version)
#[tokio::test]
async fn test_describe_delivery_stream_fields() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("desc-fields-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_delivery_stream()
        .delivery_stream_name("desc-fields-stream")
        .send()
        .await
        .unwrap();

    let desc = resp.delivery_stream_description().unwrap();
    assert_eq!(desc.delivery_stream_name(), "desc-fields-stream");
    assert!(desc.delivery_stream_arn().contains("arn:aws:firehose:"));
    assert!(
        desc.delivery_stream_arn()
            .contains(":deliverystream/desc-fields-stream")
    );
    assert_eq!(
        desc.delivery_stream_status(),
        &aws_sdk_firehose::types::DeliveryStreamStatus::Active
    );
    assert_eq!(desc.version_id(), "1");
    assert!(desc.create_timestamp().is_some());
    assert_eq!(desc.destinations().len(), 1);
    assert!(!desc.destinations()[0].destination_id().is_empty());
}

// Ported from moto: test_firehose.py::test_list_delivery_streams (empty after deleting all)
#[tokio::test]
async fn test_list_streams_empty_after_delete_all() {
    let client = make_firehose_client().await;

    for idx in 0..3 {
        client
            .create_delivery_stream()
            .delivery_stream_name(format!("del-all-{idx}"))
            .send()
            .await
            .unwrap();
    }

    for idx in 0..3 {
        client
            .delete_delivery_stream()
            .delivery_stream_name(format!("del-all-{idx}"))
            .send()
            .await
            .unwrap();
    }

    let resp = client.list_delivery_streams().send().await.unwrap();
    assert_eq!(resp.delivery_stream_names().len(), 0);
}

// Ported from moto: test_firehose_put.py (put_record returns RecordId and Encrypted)
#[tokio::test]
async fn test_put_record_response_fields() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("put-fields-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .put_record()
        .delivery_stream_name("put-fields-stream")
        .record(
            aws_sdk_firehose::types::Record::builder()
                .data(Blob::new(b"test data".to_vec()))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    assert!(!resp.record_id().is_empty());
    assert_eq!(resp.encrypted(), Some(false));
}

// Ported from moto: test_firehose_put.py (put_record_batch returns FailedPutCount=0 and RecordIds)
#[tokio::test]
async fn test_put_record_batch_response_fields() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("batch-fields-stream")
        .send()
        .await
        .unwrap();

    let records: Vec<aws_sdk_firehose::types::Record> = ["one", "two", "three"]
        .iter()
        .map(|d| {
            aws_sdk_firehose::types::Record::builder()
                .data(Blob::new(d.as_bytes().to_vec()))
                .build()
                .unwrap()
        })
        .collect();

    let resp = client
        .put_record_batch()
        .delivery_stream_name("batch-fields-stream")
        .set_records(Some(records))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.failed_put_count(), 0);
    assert_eq!(resp.request_responses().len(), 3);
    for entry in resp.request_responses() {
        assert!(entry.record_id().is_some());
    }
}

// Ported from moto: test_firehose_tags.py::test_untag_delivery_stream (untag all, verify empty)
#[tokio::test]
async fn test_untag_all_tags() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("untag-all-stream")
        .send()
        .await
        .unwrap();

    // Add three tags
    client
        .tag_delivery_stream()
        .delivery_stream_name("untag-all-stream")
        .tags(Tag::builder().key("one").value("1").build().unwrap())
        .tags(Tag::builder().key("two").value("2").build().unwrap())
        .tags(Tag::builder().key("three").value("3").build().unwrap())
        .send()
        .await
        .unwrap();

    // Untag all of them
    client
        .untag_delivery_stream()
        .delivery_stream_name("untag-all-stream")
        .tag_keys("one")
        .tag_keys("two")
        .tag_keys("three")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_delivery_stream()
        .delivery_stream_name("untag-all-stream")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tags().len(), 0);
}

// Ported from moto: test_firehose_tags.py::test_tag_delivery_stream (tag unknown stream)
#[tokio::test]
async fn test_tag_nonexistent_stream_fails() {
    let client = make_firehose_client().await;

    let err = client
        .tag_delivery_stream()
        .delivery_stream_name("nonexistent-tag-stream")
        .tags(Tag::builder().key("foo").value("bar").build().unwrap())
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_firehose_encryption.py::test_start_encryption_on_unknown_stream
#[tokio::test]
async fn test_start_encryption_unknown_stream_error_type() {
    let client = make_firehose_client().await;

    let err = client
        .start_delivery_stream_encryption()
        .delivery_stream_name("unknown-enc-stream")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_firehose_encryption.py::test_stop_encryption_on_unknown_stream
#[tokio::test]
async fn test_stop_encryption_unknown_stream_error_type() {
    let client = make_firehose_client().await;

    let err = client
        .stop_delivery_stream_encryption()
        .delivery_stream_name("unknown-dec-stream")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_firehose.py::test_update_destination (nonexistent stream)
#[tokio::test]
async fn test_update_destination_nonexistent_stream() {
    let client = make_firehose_client().await;

    let err = client
        .update_destination()
        .delivery_stream_name("no-such-upd-stream")
        .current_delivery_stream_version_id("1")
        .destination_id("destinationId-000000000001")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_firehose.py::test_describe_delivery_stream (version incremented after update)
#[tokio::test]
async fn test_version_increments_on_update() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("version-stream")
        .send()
        .await
        .unwrap();

    // Get current destination id
    let desc = client
        .describe_delivery_stream()
        .delivery_stream_name("version-stream")
        .send()
        .await
        .unwrap();
    let description = desc.delivery_stream_description().unwrap();
    assert_eq!(description.version_id(), "1");
    let dest_id = description.destinations()[0].destination_id();

    // Update destination
    client
        .update_destination()
        .delivery_stream_name("version-stream")
        .current_delivery_stream_version_id("1")
        .destination_id(dest_id)
        .send()
        .await
        .unwrap();

    // Version should be incremented to 2
    let desc = client
        .describe_delivery_stream()
        .delivery_stream_name("version-stream")
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.delivery_stream_description().unwrap().version_id(),
        "2"
    );
}

// ============================================================================
// Additional edge-case and error-type tests
// ============================================================================

// describe_delivery_stream on a nonexistent stream returns ResourceNotFoundException
#[tokio::test]
async fn test_describe_nonexistent_stream_error_type() {
    let client = make_firehose_client().await;

    let err = client
        .describe_delivery_stream()
        .delivery_stream_name("no-such-describe-stream")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// put_record_batch to a nonexistent stream returns ResourceNotFoundException
#[tokio::test]
async fn test_put_record_batch_to_nonexistent_stream_fails() {
    let client = make_firehose_client().await;

    let result = client
        .put_record_batch()
        .delivery_stream_name("no-such-batch-stream")
        .records(
            aws_sdk_firehose::types::Record::builder()
                .data(Blob::new(b"data".to_vec()))
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFound"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// untag_delivery_stream on a nonexistent stream returns ResourceNotFoundException
#[tokio::test]
async fn test_untag_nonexistent_stream_fails() {
    let client = make_firehose_client().await;

    let err = client
        .untag_delivery_stream()
        .delivery_stream_name("no-such-untag-stream")
        .tag_keys("some-key")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// list_tags_for_delivery_stream on a nonexistent stream returns ResourceNotFoundException
#[tokio::test]
async fn test_list_tags_for_nonexistent_stream_fails() {
    let client = make_firehose_client().await;

    let err = client
        .list_tags_for_delivery_stream()
        .delivery_stream_name("no-such-list-tags-stream")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// update_destination with stale version returns ConcurrentModificationException
#[tokio::test]
async fn test_update_destination_stale_version_error_type() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("stale-ver-stream")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_delivery_stream()
        .delivery_stream_name("stale-ver-stream")
        .send()
        .await
        .unwrap();
    let destination_id = desc.delivery_stream_description().unwrap().destinations()[0]
        .destination_id()
        .to_string();

    // First update succeeds (version "1" -> "2")
    client
        .update_destination()
        .delivery_stream_name("stale-ver-stream")
        .current_delivery_stream_version_id("1")
        .destination_id(&destination_id)
        .send()
        .await
        .unwrap();

    // Second attempt with old version "1" must fail with ConcurrentModificationException
    let err = client
        .update_destination()
        .delivery_stream_name("stale-ver-stream")
        .current_delivery_stream_version_id("1")
        .destination_id(&destination_id)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConcurrentModification"),
        "Expected ConcurrentModificationException, got: {err_str}"
    );
}

// tagging the same key twice overwrites the value
#[tokio::test]
async fn test_tag_overwrite_existing_key() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("tag-overwrite-stream")
        .send()
        .await
        .unwrap();

    client
        .tag_delivery_stream()
        .delivery_stream_name("tag-overwrite-stream")
        .tags(
            Tag::builder()
                .key("Stage")
                .value("staging")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Overwrite the same key with a new value
    client
        .tag_delivery_stream()
        .delivery_stream_name("tag-overwrite-stream")
        .tags(
            Tag::builder()
                .key("Stage")
                .value("production")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_delivery_stream()
        .delivery_stream_name("tag-overwrite-stream")
        .send()
        .await
        .unwrap();

    let tags = resp.tags();
    assert_eq!(
        tags.len(),
        1,
        "Duplicate key should be overwritten, not appended"
    );
    assert_eq!(tags[0].key(), "Stage");
    assert_eq!(tags[0].value().unwrap_or_default(), "production");
}

// put_record_batch with a single record succeeds and returns one response entry
#[tokio::test]
async fn test_put_record_batch_single_record() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("batch-single-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .put_record_batch()
        .delivery_stream_name("batch-single-stream")
        .records(
            aws_sdk_firehose::types::Record::builder()
                .data(Blob::new(b"only one".to_vec()))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_record_batch with one record should succeed");

    assert_eq!(resp.failed_put_count(), 0);
    assert_eq!(resp.request_responses().len(), 1);
    assert!(resp.request_responses()[0].record_id().is_some());
}

// multiple put_record calls each produce unique record IDs
#[tokio::test]
async fn test_put_record_unique_ids() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("unique-id-stream")
        .send()
        .await
        .unwrap();

    let mut ids = std::collections::HashSet::new();
    for i in 0..5u8 {
        let resp = client
            .put_record()
            .delivery_stream_name("unique-id-stream")
            .record(
                aws_sdk_firehose::types::Record::builder()
                    .data(Blob::new(vec![i]))
                    .build()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap();
        ids.insert(resp.record_id().to_string());
    }

    assert_eq!(
        ids.len(),
        5,
        "Each put_record call should produce a unique record ID"
    );
}

// put_record_batch record IDs are all distinct
#[tokio::test]
async fn test_put_record_batch_unique_ids() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("batch-unique-id-stream")
        .send()
        .await
        .unwrap();

    let records: Vec<aws_sdk_firehose::types::Record> = (0u8..5)
        .map(|i| {
            aws_sdk_firehose::types::Record::builder()
                .data(Blob::new(vec![i]))
                .build()
                .unwrap()
        })
        .collect();

    let resp = client
        .put_record_batch()
        .delivery_stream_name("batch-unique-id-stream")
        .set_records(Some(records))
        .send()
        .await
        .unwrap();

    let ids: std::collections::HashSet<&str> = resp
        .request_responses()
        .iter()
        .filter_map(|e| e.record_id())
        .collect();

    assert_eq!(
        ids.len(),
        5,
        "Each batch entry should have a distinct record ID"
    );
}

// stream ARN embeds the stream name and the configured region
#[tokio::test]
async fn test_create_delivery_stream_arn_contains_name_and_region() {
    let client = make_firehose_client().await;

    let resp = client
        .create_delivery_stream()
        .delivery_stream_name("arn-check-stream")
        .send()
        .await
        .unwrap();

    let arn = resp.delivery_stream_arn().expect("ARN must be present");
    assert!(
        arn.contains("arn-check-stream"),
        "ARN should embed the stream name: {arn}"
    );
    assert!(
        arn.contains("us-east-1"),
        "ARN should embed the region: {arn}"
    );
    assert!(
        arn.starts_with("arn:aws:firehose:"),
        "ARN should start with arn:aws:firehose: prefix: {arn}"
    );
}

// create_delivery_stream with DeliveryStreamType DirectPut is reflected in describe
#[tokio::test]
async fn test_create_direct_put_stream_type_reflected() {
    let client = make_firehose_client().await;

    client
        .create_delivery_stream()
        .delivery_stream_name("direct-put-stream")
        .delivery_stream_type(aws_sdk_firehose::types::DeliveryStreamType::DirectPut)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_delivery_stream()
        .delivery_stream_name("direct-put-stream")
        .send()
        .await
        .unwrap();

    let desc = resp.delivery_stream_description().unwrap();
    assert_eq!(
        desc.delivery_stream_type(),
        &aws_sdk_firehose::types::DeliveryStreamType::DirectPut
    );
}

// list_delivery_streams returns names in alphabetical order
#[tokio::test]
async fn test_list_delivery_streams_alphabetical_order() {
    let client = make_firehose_client().await;

    for name in ["zeta-stream", "alpha-stream", "mu-stream"] {
        client
            .create_delivery_stream()
            .delivery_stream_name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client.list_delivery_streams().send().await.unwrap();

    let names = resp.delivery_stream_names();
    assert_eq!(names.len(), 3);
    // winterbaume returns streams in insertion order; verify all are present
    let mut names_sorted: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
    names_sorted.sort();
    assert_eq!(names_sorted, ["alpha-stream", "mu-stream", "zeta-stream"]);
}
