//! Integration tests for winterbaume S3 service.
//!
//! These tests verify that aws-sdk-s3 operations work end-to-end
//! through the winterbaume mock infrastructure.

use aws_sdk_s3::config::BehaviorVersion;
use aws_sdk_s3::primitives::ByteStream;
use winterbaume_core::MockAws;
use winterbaume_s3::S3Service;

/// Helper to create a configured S3 client backed by winterbaume.
async fn make_s3_client() -> aws_sdk_s3::Client {
    let mock = MockAws::builder().with_service(S3Service::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_s3::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_head_bucket() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("test-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");

    client
        .head_bucket()
        .bucket("test-bucket")
        .send()
        .await
        .expect("head_bucket should succeed");
}

#[tokio::test]
async fn test_delete_bucket() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("del-bucket")
        .send()
        .await
        .unwrap();

    client
        .delete_bucket()
        .bucket("del-bucket")
        .send()
        .await
        .expect("delete_bucket should succeed");

    // head_bucket should fail after delete
    let err = client.head_bucket().bucket("del-bucket").send().await;
    assert!(err.is_err(), "head_bucket should fail after delete");
}

#[tokio::test]
async fn test_put_and_get_object() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("obj-bucket")
        .send()
        .await
        .unwrap();

    let body_content = b"Hello, winterbaume!";
    client
        .put_object()
        .bucket("obj-bucket")
        .key("hello.txt")
        .body(ByteStream::from_static(body_content))
        .content_type("text/plain")
        .send()
        .await
        .expect("put_object should succeed");

    let resp = client
        .get_object()
        .bucket("obj-bucket")
        .key("hello.txt")
        .send()
        .await
        .expect("get_object should succeed");

    let body = resp.body.collect().await.unwrap().into_bytes();
    assert_eq!(body.as_ref(), body_content);
    assert_eq!(resp.content_type.as_deref(), Some("text/plain"));
    assert!(resp.e_tag.is_some(), "ETag should be present");
    assert!(
        resp.content_length.is_some(),
        "Content-Length should be present"
    );
}

#[tokio::test]
async fn test_head_object() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("head-bucket")
        .send()
        .await
        .unwrap();

    client
        .put_object()
        .bucket("head-bucket")
        .key("data.bin")
        .body(ByteStream::from_static(b"binary data"))
        .send()
        .await
        .unwrap();

    let resp = client
        .head_object()
        .bucket("head-bucket")
        .key("data.bin")
        .send()
        .await
        .expect("head_object should succeed");

    assert!(resp.e_tag.is_some());
    assert_eq!(resp.content_length, Some(11)); // "binary data" is 11 bytes
}

#[tokio::test]
async fn test_delete_object() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("delobj-bucket")
        .send()
        .await
        .unwrap();

    client
        .put_object()
        .bucket("delobj-bucket")
        .key("to-delete.txt")
        .body(ByteStream::from_static(b"delete me"))
        .send()
        .await
        .unwrap();

    client
        .delete_object()
        .bucket("delobj-bucket")
        .key("to-delete.txt")
        .send()
        .await
        .expect("delete_object should succeed");

    // get should fail
    let err = client
        .get_object()
        .bucket("delobj-bucket")
        .key("to-delete.txt")
        .send()
        .await;
    assert!(err.is_err(), "get_object should fail after delete");
}

#[tokio::test]
async fn test_list_objects_v2() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("list-bucket")
        .send()
        .await
        .unwrap();

    // Put multiple objects
    for key in &["a.txt", "b.txt", "dir/c.txt", "dir/d.txt", "dir/sub/e.txt"] {
        client
            .put_object()
            .bucket("list-bucket")
            .key(*key)
            .body(ByteStream::from_static(b"content"))
            .send()
            .await
            .unwrap();
    }

    // List all objects
    let resp = client
        .list_objects_v2()
        .bucket("list-bucket")
        .send()
        .await
        .expect("list_objects_v2 should succeed");

    assert_eq!(resp.key_count(), Some(5));
    assert_eq!(resp.contents().len(), 5);

    // List with prefix
    let resp = client
        .list_objects_v2()
        .bucket("list-bucket")
        .prefix("dir/")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.contents().len(), 3);

    // List with delimiter
    let resp = client
        .list_objects_v2()
        .bucket("list-bucket")
        .delimiter("/")
        .send()
        .await
        .unwrap();

    // Should have 2 top-level objects (a.txt, b.txt) and 1 common prefix (dir/)
    assert_eq!(resp.contents().len(), 2);
    assert_eq!(resp.common_prefixes().len(), 1);
    assert_eq!(resp.common_prefixes()[0].prefix().unwrap(), "dir/");

    // List with prefix + delimiter
    let resp = client
        .list_objects_v2()
        .bucket("list-bucket")
        .prefix("dir/")
        .delimiter("/")
        .send()
        .await
        .unwrap();

    // Should have 2 direct objects (dir/c.txt, dir/d.txt) and 1 common prefix (dir/sub/)
    assert_eq!(resp.contents().len(), 2);
    assert_eq!(resp.common_prefixes().len(), 1);
    assert_eq!(resp.common_prefixes()[0].prefix().unwrap(), "dir/sub/");
}

#[tokio::test]
async fn test_list_objects_v2_pagination() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("page-bucket")
        .send()
        .await
        .unwrap();

    for i in 0..5 {
        client
            .put_object()
            .bucket("page-bucket")
            .key(format!("key-{i:02}"))
            .body(ByteStream::from_static(b"x"))
            .send()
            .await
            .unwrap();
    }

    // List with max-keys=2
    let resp = client
        .list_objects_v2()
        .bucket("page-bucket")
        .max_keys(2)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.contents().len(), 2);
    assert!(resp.is_truncated() == Some(true), "should be truncated");
    assert!(
        resp.next_continuation_token().is_some(),
        "should have continuation token"
    );

    // Continue with token
    let resp2 = client
        .list_objects_v2()
        .bucket("page-bucket")
        .max_keys(2)
        .continuation_token(resp.next_continuation_token().unwrap())
        .send()
        .await
        .unwrap();

    assert_eq!(resp2.contents().len(), 2);
    assert!(
        resp2.is_truncated() == Some(true),
        "second page should be truncated"
    );

    // Third page
    let resp3 = client
        .list_objects_v2()
        .bucket("page-bucket")
        .max_keys(2)
        .continuation_token(resp2.next_continuation_token().unwrap())
        .send()
        .await
        .unwrap();

    assert_eq!(resp3.contents().len(), 1);
    assert!(
        resp3.is_truncated() != Some(true),
        "last page should not be truncated"
    );
}

#[tokio::test]
async fn test_overwrite_object() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("overwrite-bucket")
        .send()
        .await
        .unwrap();

    client
        .put_object()
        .bucket("overwrite-bucket")
        .key("file.txt")
        .body(ByteStream::from_static(b"version 1"))
        .send()
        .await
        .unwrap();

    client
        .put_object()
        .bucket("overwrite-bucket")
        .key("file.txt")
        .body(ByteStream::from_static(b"version 2"))
        .send()
        .await
        .unwrap();

    let resp = client
        .get_object()
        .bucket("overwrite-bucket")
        .key("file.txt")
        .send()
        .await
        .unwrap();

    let body = resp.body.collect().await.unwrap().into_bytes();
    assert_eq!(body.as_ref(), b"version 2");
}

#[tokio::test]
async fn test_get_nonexistent_key_fails() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("nokey-bucket")
        .send()
        .await
        .unwrap();

    let err = client
        .get_object()
        .bucket("nokey-bucket")
        .key("nonexistent")
        .send()
        .await;

    assert!(err.is_err(), "get_object should fail for nonexistent key");
}

#[tokio::test]
async fn test_list_buckets() {
    let client = make_s3_client().await;

    for name in &["lb-alpha", "lb-beta", "lb-gamma"] {
        client
            .create_bucket()
            .bucket(*name)
            .send()
            .await
            .expect("create_bucket should succeed");
    }

    let resp = client
        .list_buckets()
        .send()
        .await
        .expect("list_buckets should succeed");

    let buckets = resp.buckets();
    assert_eq!(buckets.len(), 3, "should have 3 buckets");

    let mut names: Vec<&str> = buckets.iter().map(|b| b.name().unwrap()).collect();
    names.sort();
    assert_eq!(names, vec!["lb-alpha", "lb-beta", "lb-gamma"]);
}

#[tokio::test]
async fn test_create_duplicate_bucket_fails() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("dup-bucket")
        .send()
        .await
        .expect("first create_bucket should succeed");

    let err = client.create_bucket().bucket("dup-bucket").send().await;

    assert!(err.is_err(), "creating a duplicate bucket should fail");
}

#[tokio::test]
async fn test_create_bucket_invalid_name() {
    let client = make_s3_client().await;

    let err = client.create_bucket().bucket("ab").send().await;

    assert!(err.is_err(), "bucket name shorter than 3 chars should fail");
}

#[tokio::test]
async fn test_head_nonexistent_bucket() {
    let client = make_s3_client().await;

    let err = client.head_bucket().bucket("no-such-bucket").send().await;

    assert!(
        err.is_err(),
        "head_bucket on nonexistent bucket should fail"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_bucket() {
    let client = make_s3_client().await;

    let err = client.delete_bucket().bucket("no-such-bucket").send().await;

    assert!(
        err.is_err(),
        "delete_bucket on nonexistent bucket should fail"
    );
}

#[tokio::test]
async fn test_delete_bucket_not_empty() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("notempty-bucket")
        .send()
        .await
        .unwrap();

    client
        .put_object()
        .bucket("notempty-bucket")
        .key("file.txt")
        .body(ByteStream::from_static(b"data"))
        .send()
        .await
        .unwrap();

    let err = client
        .delete_bucket()
        .bucket("notempty-bucket")
        .send()
        .await;

    assert!(
        err.is_err(),
        "delete_bucket on non-empty bucket should fail"
    );
}

#[tokio::test]
async fn test_head_nonexistent_object() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("headobj-bucket")
        .send()
        .await
        .unwrap();

    let err = client
        .head_object()
        .bucket("headobj-bucket")
        .key("nonexistent-key")
        .send()
        .await;

    assert!(err.is_err(), "head_object on nonexistent key should fail");
}

#[tokio::test]
async fn test_put_object_to_nonexistent_bucket() {
    let client = make_s3_client().await;

    let err = client
        .put_object()
        .bucket("no-such-bucket")
        .key("file.txt")
        .body(ByteStream::from_static(b"data"))
        .send()
        .await;

    assert!(err.is_err(), "put_object to nonexistent bucket should fail");
}

#[tokio::test]
async fn test_put_and_get_object_with_metadata() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("meta-bucket")
        .send()
        .await
        .unwrap();

    let body_content = b"metadata test body";
    client
        .put_object()
        .bucket("meta-bucket")
        .key("meta.txt")
        .body(ByteStream::from_static(body_content))
        .metadata("key1", "value1")
        .metadata("key2", "value2")
        .send()
        .await
        .expect("put_object with metadata should succeed");

    let resp = client
        .get_object()
        .bucket("meta-bucket")
        .key("meta.txt")
        .send()
        .await
        .expect("get_object should succeed");

    // Check metadata before consuming body
    if let Some(metadata) = resp.metadata() {
        assert_eq!(metadata.get("key1").map(|s| s.as_str()), Some("value1"));
        assert_eq!(metadata.get("key2").map(|s| s.as_str()), Some("value2"));
    }

    // Verify the body is correct
    let body = resp.body.collect().await.unwrap().into_bytes();
    assert_eq!(body.as_ref(), body_content);
}

#[tokio::test]
async fn test_list_objects_v2_start_after() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("startafter-bucket")
        .send()
        .await
        .unwrap();

    for key in &["a.txt", "b.txt", "c.txt", "d.txt"] {
        client
            .put_object()
            .bucket("startafter-bucket")
            .key(*key)
            .body(ByteStream::from_static(b"content"))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_objects_v2()
        .bucket("startafter-bucket")
        .start_after("b.txt")
        .send()
        .await
        .expect("list_objects_v2 with start_after should succeed");

    let keys: Vec<&str> = resp
        .contents()
        .iter()
        .map(|obj| obj.key().unwrap())
        .collect();

    assert_eq!(keys, vec!["c.txt", "d.txt"]);
}

#[tokio::test]
async fn test_list_objects_v2_empty_bucket() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("empty-list-bucket")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_objects_v2()
        .bucket("empty-list-bucket")
        .send()
        .await
        .expect("list_objects_v2 on empty bucket should succeed");

    assert_eq!(resp.key_count(), Some(0));
    assert!(resp.contents().is_empty());
}

#[tokio::test]
async fn test_delete_object_idempotent() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("idempotent-bucket")
        .send()
        .await
        .unwrap();

    // Deleting a nonexistent key should succeed (S3 delete is idempotent)
    client
        .delete_object()
        .bucket("idempotent-bucket")
        .key("nonexistent-key")
        .send()
        .await
        .expect("delete_object on nonexistent key should succeed (idempotent)");
}

#[tokio::test]
async fn test_put_and_get_bucket_lifecycle_configuration() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("lifecycle-bucket")
        .send()
        .await
        .unwrap();

    // Put lifecycle configuration
    let rule = aws_sdk_s3::types::LifecycleRule::builder()
        .id("expire-old-objects")
        .status(aws_sdk_s3::types::ExpirationStatus::Enabled)
        .expiration(
            aws_sdk_s3::types::LifecycleExpiration::builder()
                .days(30)
                .build(),
        )
        .filter(
            aws_sdk_s3::types::LifecycleRuleFilter::builder()
                .prefix("logs/")
                .build(),
        )
        .build()
        .unwrap();

    client
        .put_bucket_lifecycle_configuration()
        .bucket("lifecycle-bucket")
        .lifecycle_configuration(
            aws_sdk_s3::types::BucketLifecycleConfiguration::builder()
                .rules(rule)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_bucket_lifecycle_configuration should succeed");

    // Get the lifecycle configuration back
    let resp = client
        .get_bucket_lifecycle_configuration()
        .bucket("lifecycle-bucket")
        .send()
        .await
        .expect("get_bucket_lifecycle_configuration should succeed");

    assert!(!resp.rules().is_empty());
}

#[tokio::test]
async fn test_get_bucket_lifecycle_configuration_not_found() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("no-lifecycle-bucket")
        .send()
        .await
        .unwrap();

    // Should fail with NoSuchLifecycleConfiguration when no lifecycle is set
    let result = client
        .get_bucket_lifecycle_configuration()
        .bucket("no-lifecycle-bucket")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_and_get_bucket_notification_configuration() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("notification-bucket")
        .send()
        .await
        .unwrap();

    // Get notification configuration before setting - should return empty config
    let resp_empty = client
        .get_bucket_notification_configuration()
        .bucket("notification-bucket")
        .send()
        .await
        .expect("get_bucket_notification_configuration should succeed with empty config");

    // Initially no configurations
    assert!(resp_empty.queue_configurations().is_empty());

    // Put a notification config
    let queue_config = aws_sdk_s3::types::QueueConfiguration::builder()
        .id("notify-on-put")
        .queue_arn("arn:aws:sqs:us-east-1:123456789012:my-queue")
        .events(aws_sdk_s3::types::Event::S3ObjectCreatedPut)
        .build()
        .unwrap();

    client
        .put_bucket_notification_configuration()
        .bucket("notification-bucket")
        .notification_configuration(
            aws_sdk_s3::types::NotificationConfiguration::builder()
                .queue_configurations(queue_config)
                .build(),
        )
        .send()
        .await
        .expect("put_bucket_notification_configuration should succeed");

    // Get it back
    let resp = client
        .get_bucket_notification_configuration()
        .bucket("notification-bucket")
        .send()
        .await
        .expect("get_bucket_notification_configuration should succeed after put");

    assert_eq!(resp.queue_configurations().len(), 1);
}

// ============================================================================
// Tests derived from AWS documentation: Amazon S3 bucket sub-resource operations
// ============================================================================

// -----------------------------------------------------------------------
// Versioning
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_bucket_versioning_default() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("versioning-default-bucket")
        .send()
        .await
        .unwrap();

    // Versioning was never enabled — status element should be absent
    let resp = client
        .get_bucket_versioning()
        .bucket("versioning-default-bucket")
        .send()
        .await
        .expect("get_bucket_versioning should succeed");

    assert!(
        resp.status().is_none(),
        "versioning status should be absent on a new bucket"
    );
}

#[tokio::test]
async fn test_put_get_bucket_versioning_enabled() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("versioning-enabled-bucket")
        .send()
        .await
        .unwrap();

    client
        .put_bucket_versioning()
        .bucket("versioning-enabled-bucket")
        .versioning_configuration(
            aws_sdk_s3::types::VersioningConfiguration::builder()
                .status(aws_sdk_s3::types::BucketVersioningStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .expect("put_bucket_versioning Enabled should succeed");

    let resp = client
        .get_bucket_versioning()
        .bucket("versioning-enabled-bucket")
        .send()
        .await
        .expect("get_bucket_versioning should succeed");

    assert_eq!(
        resp.status(),
        Some(&aws_sdk_s3::types::BucketVersioningStatus::Enabled)
    );
}

#[tokio::test]
async fn test_put_get_bucket_versioning_suspended() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("versioning-suspended-bucket")
        .send()
        .await
        .unwrap();

    // First enable, then suspend
    client
        .put_bucket_versioning()
        .bucket("versioning-suspended-bucket")
        .versioning_configuration(
            aws_sdk_s3::types::VersioningConfiguration::builder()
                .status(aws_sdk_s3::types::BucketVersioningStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .put_bucket_versioning()
        .bucket("versioning-suspended-bucket")
        .versioning_configuration(
            aws_sdk_s3::types::VersioningConfiguration::builder()
                .status(aws_sdk_s3::types::BucketVersioningStatus::Suspended)
                .build(),
        )
        .send()
        .await
        .expect("put_bucket_versioning Suspended should succeed");

    let resp = client
        .get_bucket_versioning()
        .bucket("versioning-suspended-bucket")
        .send()
        .await
        .expect("get_bucket_versioning should succeed");

    assert_eq!(
        resp.status(),
        Some(&aws_sdk_s3::types::BucketVersioningStatus::Suspended)
    );
}

// -----------------------------------------------------------------------
// Object versioning (functional)
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_put_object_versioning_enabled_returns_version_id() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("ver-put-returns-vid")
        .send()
        .await
        .unwrap();

    client
        .put_bucket_versioning()
        .bucket("ver-put-returns-vid")
        .versioning_configuration(
            aws_sdk_s3::types::VersioningConfiguration::builder()
                .status(aws_sdk_s3::types::BucketVersioningStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .put_object()
        .bucket("ver-put-returns-vid")
        .key("mykey")
        .body(ByteStream::from_static(b"hello"))
        .send()
        .await
        .expect("put_object should succeed");

    let vid = resp.version_id().expect("version_id should be set");
    assert!(!vid.is_empty(), "version_id must not be empty");
    assert_ne!(
        vid, "null",
        "version_id must not be 'null' when versioning enabled"
    );
}

#[tokio::test]
async fn test_put_object_versioning_two_versions() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("ver-two-versions")
        .send()
        .await
        .unwrap();

    client
        .put_bucket_versioning()
        .bucket("ver-two-versions")
        .versioning_configuration(
            aws_sdk_s3::types::VersioningConfiguration::builder()
                .status(aws_sdk_s3::types::BucketVersioningStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let v1 = client
        .put_object()
        .bucket("ver-two-versions")
        .key("mykey")
        .body(ByteStream::from_static(b"version-1"))
        .send()
        .await
        .unwrap();
    let vid1 = v1.version_id().unwrap().to_string();

    let _v2 = client
        .put_object()
        .bucket("ver-two-versions")
        .key("mykey")
        .body(ByteStream::from_static(b"version-2"))
        .send()
        .await
        .unwrap();

    // GetObject without versionId should return v2.
    let current = client
        .get_object()
        .bucket("ver-two-versions")
        .key("mykey")
        .send()
        .await
        .expect("get latest should succeed");
    let body = current.body.collect().await.unwrap().into_bytes();
    assert_eq!(body.as_ref(), b"version-2");

    // GetObject with v1's versionId should return v1.
    let mut old = client
        .get_object()
        .bucket("ver-two-versions")
        .key("mykey")
        .version_id(vid1.clone())
        .send()
        .await
        .expect("get by versionId should succeed");
    let returned_vid1 = old.version_id().unwrap().to_string();
    let old_body = old.body.collect().await.unwrap().into_bytes();
    assert_eq!(old_body.as_ref(), b"version-1");
    assert_eq!(returned_vid1, vid1.as_str());
}

#[tokio::test]
async fn test_delete_object_versioning_creates_delete_marker() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("ver-delete-marker")
        .send()
        .await
        .unwrap();

    client
        .put_bucket_versioning()
        .bucket("ver-delete-marker")
        .versioning_configuration(
            aws_sdk_s3::types::VersioningConfiguration::builder()
                .status(aws_sdk_s3::types::BucketVersioningStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .put_object()
        .bucket("ver-delete-marker")
        .key("mykey")
        .body(ByteStream::from_static(b"data"))
        .send()
        .await
        .unwrap();

    // Delete without versionId — should create delete marker, not permanently delete.
    let del_resp = client
        .delete_object()
        .bucket("ver-delete-marker")
        .key("mykey")
        .send()
        .await
        .expect("delete_object should succeed");

    assert!(
        del_resp.delete_marker().unwrap_or(false),
        "response must indicate a delete marker was created"
    );
    assert!(
        del_resp.version_id().is_some(),
        "delete marker version_id must be present"
    );

    // GetObject on a key with a delete marker should return NoSuchKey.
    let err = client
        .get_object()
        .bucket("ver-delete-marker")
        .key("mykey")
        .send()
        .await
        .expect_err("get after delete marker should fail");
    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_no_such_key(),
        "expected NoSuchKey after delete marker"
    );

    // ListObjectVersions should show both the version and the delete marker.
    let list = client
        .list_object_versions()
        .bucket("ver-delete-marker")
        .send()
        .await
        .unwrap();

    let versions = list.versions();
    assert_eq!(versions.len(), 1, "should have 1 object version");
    assert!(
        !versions[0].is_latest().unwrap_or(true),
        "version should no longer be latest"
    );

    let markers = list.delete_markers();
    assert_eq!(markers.len(), 1, "should have 1 delete marker");
    assert!(
        markers[0].is_latest().unwrap_or(false),
        "delete marker should be latest"
    );
}

#[tokio::test]
async fn test_delete_specific_version() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("ver-delete-specific")
        .send()
        .await
        .unwrap();

    client
        .put_bucket_versioning()
        .bucket("ver-delete-specific")
        .versioning_configuration(
            aws_sdk_s3::types::VersioningConfiguration::builder()
                .status(aws_sdk_s3::types::BucketVersioningStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let v1_resp = client
        .put_object()
        .bucket("ver-delete-specific")
        .key("mykey")
        .body(ByteStream::from_static(b"v1"))
        .send()
        .await
        .unwrap();
    let vid1 = v1_resp.version_id().unwrap().to_string();

    client
        .put_object()
        .bucket("ver-delete-specific")
        .key("mykey")
        .body(ByteStream::from_static(b"v2"))
        .send()
        .await
        .unwrap();

    // Permanently delete v1 by its version_id — v2 should still be accessible.
    client
        .delete_object()
        .bucket("ver-delete-specific")
        .key("mykey")
        .version_id(vid1.clone())
        .send()
        .await
        .expect("delete specific version should succeed");

    // Current (v2) should still be accessible.
    let current = client
        .get_object()
        .bucket("ver-delete-specific")
        .key("mykey")
        .send()
        .await
        .expect("v2 should still be accessible");
    let body = current.body.collect().await.unwrap().into_bytes();
    assert_eq!(body.as_ref(), b"v2");

    // v1 should be gone.
    client
        .get_object()
        .bucket("ver-delete-specific")
        .key("mykey")
        .version_id(vid1)
        .send()
        .await
        .expect_err("deleted version should not be accessible");
}

#[tokio::test]
async fn test_list_object_versions_with_history() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("ver-list-versions")
        .send()
        .await
        .unwrap();

    client
        .put_bucket_versioning()
        .bucket("ver-list-versions")
        .versioning_configuration(
            aws_sdk_s3::types::VersioningConfiguration::builder()
                .status(aws_sdk_s3::types::BucketVersioningStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Put 3 versions of "a" and 1 version of "b".
    for i in 0..3u8 {
        client
            .put_object()
            .bucket("ver-list-versions")
            .key("a")
            .body(ByteStream::from_static(b"data"))
            .send()
            .await
            .unwrap();
        let _ = i;
    }
    client
        .put_object()
        .bucket("ver-list-versions")
        .key("b")
        .body(ByteStream::from_static(b"bdata"))
        .send()
        .await
        .unwrap();

    let list = client
        .list_object_versions()
        .bucket("ver-list-versions")
        .send()
        .await
        .unwrap();

    let versions = list.versions();
    let a_versions: Vec<_> = versions
        .iter()
        .filter(|v| v.key().unwrap_or("") == "a")
        .collect();
    let b_versions: Vec<_> = versions
        .iter()
        .filter(|v| v.key().unwrap_or("") == "b")
        .collect();

    assert_eq!(a_versions.len(), 3, "should have 3 versions of key 'a'");
    assert_eq!(b_versions.len(), 1, "should have 1 version of key 'b'");

    // Exactly one version of "a" should be is_latest.
    let latest_a = a_versions
        .iter()
        .filter(|v| v.is_latest().unwrap_or(false))
        .count();
    assert_eq!(
        latest_a, 1,
        "exactly one version of 'a' should be is_latest"
    );
    assert!(
        b_versions[0].is_latest().unwrap_or(false),
        "the single version of 'b' should be is_latest"
    );
}

#[tokio::test]
async fn test_versioning_suspended_stores_null_version() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("ver-suspended-null")
        .send()
        .await
        .unwrap();

    // Enable and then suspend versioning.
    client
        .put_bucket_versioning()
        .bucket("ver-suspended-null")
        .versioning_configuration(
            aws_sdk_s3::types::VersioningConfiguration::builder()
                .status(aws_sdk_s3::types::BucketVersioningStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .put_bucket_versioning()
        .bucket("ver-suspended-null")
        .versioning_configuration(
            aws_sdk_s3::types::VersioningConfiguration::builder()
                .status(aws_sdk_s3::types::BucketVersioningStatus::Suspended)
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Two PutObject calls with versioning suspended; each uses version_id = "null".
    client
        .put_object()
        .bucket("ver-suspended-null")
        .key("mykey")
        .body(ByteStream::from_static(b"first"))
        .send()
        .await
        .unwrap();

    client
        .put_object()
        .bucket("ver-suspended-null")
        .key("mykey")
        .body(ByteStream::from_static(b"second"))
        .send()
        .await
        .unwrap();

    // The second write should have overwritten the first.
    let mut resp = client
        .get_object()
        .bucket("ver-suspended-null")
        .key("mykey")
        .send()
        .await
        .unwrap();
    let no_version_id = resp.version_id().is_none();
    let body = resp.body.collect().await.unwrap().into_bytes();
    assert_eq!(body.as_ref(), b"second");

    // version_id header should be absent (null version).
    assert!(
        no_version_id,
        "version_id must be absent for suspended versioning"
    );
}

// -----------------------------------------------------------------------
// ACL
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_bucket_acl_default() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("acl-default-bucket")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_bucket_acl()
        .bucket("acl-default-bucket")
        .send()
        .await
        .expect("get_bucket_acl should succeed");

    let grants = resp.grants();
    assert!(
        !grants.is_empty(),
        "default ACL should have at least one grant"
    );
    assert_eq!(
        grants[0].permission(),
        Some(&aws_sdk_s3::types::Permission::FullControl)
    );
    assert!(resp.owner().is_some(), "ACL should have an owner");
}

#[tokio::test]
async fn test_put_bucket_acl() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("acl-put-bucket")
        .send()
        .await
        .unwrap();

    // Put a canned ACL
    client
        .put_bucket_acl()
        .bucket("acl-put-bucket")
        .acl(aws_sdk_s3::types::BucketCannedAcl::Private)
        .send()
        .await
        .expect("put_bucket_acl should succeed");
}

// -----------------------------------------------------------------------
// Policy
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_bucket_policy_not_found() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("policy-nf-bucket")
        .send()
        .await
        .unwrap();

    let err = client
        .get_bucket_policy()
        .bucket("policy-nf-bucket")
        .send()
        .await;

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("NoSuchBucketPolicy"),
        "Expected NoSuchBucketPolicy, got: {err_str}"
    );
}

#[tokio::test]
async fn test_put_get_delete_bucket_policy() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("policy-crud-bucket")
        .send()
        .await
        .unwrap();

    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;

    client
        .put_bucket_policy()
        .bucket("policy-crud-bucket")
        .policy(policy)
        .send()
        .await
        .expect("put_bucket_policy should succeed");

    let resp = client
        .get_bucket_policy()
        .bucket("policy-crud-bucket")
        .send()
        .await
        .expect("get_bucket_policy should succeed");

    assert!(resp.policy().is_some(), "policy should be returned");

    client
        .delete_bucket_policy()
        .bucket("policy-crud-bucket")
        .send()
        .await
        .expect("delete_bucket_policy should succeed");

    let err = client
        .get_bucket_policy()
        .bucket("policy-crud-bucket")
        .send()
        .await;
    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("NoSuchBucketPolicy"),
        "After delete, expected NoSuchBucketPolicy, got: {err_str}"
    );
}

// -----------------------------------------------------------------------
// Tagging
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_bucket_tagging_not_found() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("tagging-nf-bucket")
        .send()
        .await
        .unwrap();

    let err = client
        .get_bucket_tagging()
        .bucket("tagging-nf-bucket")
        .send()
        .await;

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("NoSuchTagSet"),
        "Expected NoSuchTagSet, got: {err_str}"
    );
}

#[tokio::test]
async fn test_put_get_delete_bucket_tagging() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("tagging-crud-bucket")
        .send()
        .await
        .unwrap();

    client
        .put_bucket_tagging()
        .bucket("tagging-crud-bucket")
        .tagging(
            aws_sdk_s3::types::Tagging::builder()
                .tag_set(
                    aws_sdk_s3::types::Tag::builder()
                        .key("env")
                        .value("test")
                        .build()
                        .unwrap(),
                )
                .tag_set(
                    aws_sdk_s3::types::Tag::builder()
                        .key("team")
                        .value("platform")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_bucket_tagging should succeed");

    let resp = client
        .get_bucket_tagging()
        .bucket("tagging-crud-bucket")
        .send()
        .await
        .expect("get_bucket_tagging should succeed");

    assert_eq!(resp.tag_set().len(), 2, "should have 2 tags");

    let mut keys: Vec<&str> = resp.tag_set().iter().map(|t| t.key()).collect();
    keys.sort();
    assert_eq!(keys, vec!["env", "team"]);

    client
        .delete_bucket_tagging()
        .bucket("tagging-crud-bucket")
        .send()
        .await
        .expect("delete_bucket_tagging should succeed");

    let err = client
        .get_bucket_tagging()
        .bucket("tagging-crud-bucket")
        .send()
        .await;
    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("NoSuchTagSet"),
        "After delete, expected NoSuchTagSet, got: {err_str}"
    );
}

// -----------------------------------------------------------------------
// Accelerate Configuration
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_bucket_accelerate_configuration_default() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("accelerate-default-bucket")
        .send()
        .await
        .unwrap();

    // Default is no acceleration — should return an empty config (no error)
    let resp = client
        .get_bucket_accelerate_configuration()
        .bucket("accelerate-default-bucket")
        .send()
        .await
        .expect("get_bucket_accelerate_configuration should succeed on new bucket");

    // Status should be absent (no acceleration configured)
    assert!(
        resp.status().is_none(),
        "accelerate status should be absent on a new bucket"
    );
}

#[tokio::test]
async fn test_put_bucket_accelerate_configuration() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("accelerate-put-bucket")
        .send()
        .await
        .unwrap();

    client
        .put_bucket_accelerate_configuration()
        .bucket("accelerate-put-bucket")
        .accelerate_configuration(
            aws_sdk_s3::types::AccelerateConfiguration::builder()
                .status(aws_sdk_s3::types::BucketAccelerateStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .expect("put_bucket_accelerate_configuration should succeed");
}

// -----------------------------------------------------------------------
// Request Payment
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_bucket_request_payment_default() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("reqpayment-default-bucket")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_bucket_request_payment()
        .bucket("reqpayment-default-bucket")
        .send()
        .await
        .expect("get_bucket_request_payment should succeed");

    assert_eq!(
        resp.payer(),
        Some(&aws_sdk_s3::types::Payer::BucketOwner),
        "default payer should be BucketOwner"
    );
}

#[tokio::test]
async fn test_put_bucket_request_payment() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("reqpayment-put-bucket")
        .send()
        .await
        .unwrap();

    client
        .put_bucket_request_payment()
        .bucket("reqpayment-put-bucket")
        .request_payment_configuration(
            aws_sdk_s3::types::RequestPaymentConfiguration::builder()
                .payer(aws_sdk_s3::types::Payer::BucketOwner)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_bucket_request_payment should succeed");
}

// -----------------------------------------------------------------------
// CORS
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_bucket_cors_not_found() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("cors-nf-bucket")
        .send()
        .await
        .unwrap();

    let err = client
        .get_bucket_cors()
        .bucket("cors-nf-bucket")
        .send()
        .await;

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("NoSuchCORSConfiguration"),
        "Expected NoSuchCORSConfiguration, got: {err_str}"
    );
}

#[tokio::test]
async fn test_put_get_delete_bucket_cors() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("cors-crud-bucket")
        .send()
        .await
        .unwrap();

    let cors_rule = aws_sdk_s3::types::CorsRule::builder()
        .allowed_methods("GET")
        .allowed_methods("PUT")
        .allowed_origins("*")
        .build()
        .unwrap();

    client
        .put_bucket_cors()
        .bucket("cors-crud-bucket")
        .cors_configuration(
            aws_sdk_s3::types::CorsConfiguration::builder()
                .cors_rules(cors_rule)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_bucket_cors should succeed");

    let resp = client
        .get_bucket_cors()
        .bucket("cors-crud-bucket")
        .send()
        .await
        .expect("get_bucket_cors should succeed");

    assert!(
        !resp.cors_rules().is_empty(),
        "CORS rules should be returned"
    );

    client
        .delete_bucket_cors()
        .bucket("cors-crud-bucket")
        .send()
        .await
        .expect("delete_bucket_cors should succeed");

    let err = client
        .get_bucket_cors()
        .bucket("cors-crud-bucket")
        .send()
        .await;
    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("NoSuchCORSConfiguration"),
        "After delete, expected NoSuchCORSConfiguration, got: {err_str}"
    );
}

// -----------------------------------------------------------------------
// Encryption
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_bucket_encryption_default() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("encryption-default-bucket")
        .send()
        .await
        .unwrap();

    // AWS returns a default AES256 rule even before any encryption is configured (since 2023)
    let resp = client
        .get_bucket_encryption()
        .bucket("encryption-default-bucket")
        .send()
        .await
        .expect("get_bucket_encryption should succeed on new bucket");

    let rules = resp
        .server_side_encryption_configuration()
        .map(|c| c.rules())
        .unwrap_or_default();

    assert!(
        !rules.is_empty(),
        "default encryption should have at least one rule"
    );
    let default_sse = rules[0]
        .apply_server_side_encryption_by_default()
        .expect("Rule should have default encryption");
    assert_eq!(
        default_sse.sse_algorithm(),
        &aws_sdk_s3::types::ServerSideEncryption::Aes256,
        "default SSE algorithm should be AES256"
    );
}

#[tokio::test]
async fn test_put_get_delete_bucket_encryption() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("encryption-crud-bucket")
        .send()
        .await
        .unwrap();

    let rule = aws_sdk_s3::types::ServerSideEncryptionRule::builder()
        .apply_server_side_encryption_by_default(
            aws_sdk_s3::types::ServerSideEncryptionByDefault::builder()
                .sse_algorithm(aws_sdk_s3::types::ServerSideEncryption::Aes256)
                .build()
                .unwrap(),
        )
        .build();

    client
        .put_bucket_encryption()
        .bucket("encryption-crud-bucket")
        .server_side_encryption_configuration(
            aws_sdk_s3::types::ServerSideEncryptionConfiguration::builder()
                .rules(rule)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_bucket_encryption should succeed");

    let resp = client
        .get_bucket_encryption()
        .bucket("encryption-crud-bucket")
        .send()
        .await
        .expect("get_bucket_encryption should succeed after put");

    assert!(
        resp.server_side_encryption_configuration().is_some(),
        "encryption configuration should be returned"
    );

    client
        .delete_bucket_encryption()
        .bucket("encryption-crud-bucket")
        .send()
        .await
        .expect("delete_bucket_encryption should succeed");

    // After delete, the default AES256 rule should be returned again
    let resp2 = client
        .get_bucket_encryption()
        .bucket("encryption-crud-bucket")
        .send()
        .await
        .expect("get_bucket_encryption should succeed after delete (default config)");

    let rules2 = resp2
        .server_side_encryption_configuration()
        .map(|c| c.rules())
        .unwrap_or_default();
    assert!(
        !rules2.is_empty(),
        "default encryption config should be returned after delete"
    );
}

// -----------------------------------------------------------------------
// Object Lock
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_bucket_object_lock_configuration_not_found() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("objectlock-bucket")
        .send()
        .await
        .unwrap();

    // Object lock is not configured — should return ObjectLockConfigurationNotFoundError
    let err = client
        .get_object_lock_configuration()
        .bucket("objectlock-bucket")
        .send()
        .await;

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ObjectLockConfigurationNotFoundError")
            || err_str.contains("ObjectLockConfigurationNotFound"),
        "Expected ObjectLockConfigurationNotFoundError, got: {err_str}"
    );
}

// -----------------------------------------------------------------------
// Logging
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_bucket_logging_default() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("logging-default-bucket")
        .send()
        .await
        .unwrap();

    // Default: no logging configured — should return empty BucketLoggingStatus
    let resp = client
        .get_bucket_logging()
        .bucket("logging-default-bucket")
        .send()
        .await
        .expect("get_bucket_logging should succeed");

    // logging_enabled() should be None when logging is not configured
    assert!(
        resp.logging_enabled().is_none(),
        "logging_enabled should be None on a new bucket"
    );
}

#[tokio::test]
async fn test_put_bucket_logging() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("logging-put-bucket")
        .send()
        .await
        .unwrap();

    // Disable logging (put with empty logging enabled config — common pattern)
    client
        .put_bucket_logging()
        .bucket("logging-put-bucket")
        .bucket_logging_status(aws_sdk_s3::types::BucketLoggingStatus::builder().build())
        .send()
        .await
        .expect("put_bucket_logging should succeed");
}

// -----------------------------------------------------------------------
// Replication
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_bucket_replication_not_found() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("replication-bucket")
        .send()
        .await
        .unwrap();

    let err = client
        .get_bucket_replication()
        .bucket("replication-bucket")
        .send()
        .await;

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ReplicationConfigurationNotFoundError")
            || err_str.contains("ReplicationConfigurationNotFound"),
        "Expected ReplicationConfigurationNotFoundError, got: {err_str}"
    );
}

// -----------------------------------------------------------------------
// Website
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_bucket_website_not_found() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("website-nf-bucket")
        .send()
        .await
        .unwrap();

    let err = client
        .get_bucket_website()
        .bucket("website-nf-bucket")
        .send()
        .await;

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("NoSuchWebsiteConfiguration"),
        "Expected NoSuchWebsiteConfiguration, got: {err_str}"
    );
}

#[tokio::test]
async fn test_put_get_delete_bucket_website() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("website-crud-bucket")
        .send()
        .await
        .unwrap();

    client
        .put_bucket_website()
        .bucket("website-crud-bucket")
        .website_configuration(
            aws_sdk_s3::types::WebsiteConfiguration::builder()
                .index_document(
                    aws_sdk_s3::types::IndexDocument::builder()
                        .suffix("index.html")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("put_bucket_website should succeed");

    client
        .get_bucket_website()
        .bucket("website-crud-bucket")
        .send()
        .await
        .expect("get_bucket_website should succeed after put");

    client
        .delete_bucket_website()
        .bucket("website-crud-bucket")
        .send()
        .await
        .expect("delete_bucket_website should succeed");

    let err = client
        .get_bucket_website()
        .bucket("website-crud-bucket")
        .send()
        .await;
    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("NoSuchWebsiteConfiguration"),
        "After delete, expected NoSuchWebsiteConfiguration, got: {err_str}"
    );
}

// -----------------------------------------------------------------------
// Policy Status
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_bucket_policy_status() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("policystatus-bucket")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_bucket_policy_status()
        .bucket("policystatus-bucket")
        .send()
        .await
        .expect("get_bucket_policy_status should succeed");

    let is_public = resp.policy_status().and_then(|ps| ps.is_public);
    assert_eq!(
        is_public,
        Some(false),
        "bucket should not be public by default"
    );
}

// -----------------------------------------------------------------------
// Ownership Controls
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_bucket_ownership_controls_default() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("ownership-default-bucket")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_bucket_ownership_controls()
        .bucket("ownership-default-bucket")
        .send()
        .await
        .expect("get_bucket_ownership_controls should succeed");

    let rules = resp
        .ownership_controls()
        .map(|oc| oc.rules())
        .unwrap_or_default();

    assert!(
        !rules.is_empty(),
        "ownership controls should have a default rule"
    );
    assert_eq!(
        rules[0].object_ownership(),
        &aws_sdk_s3::types::ObjectOwnership::BucketOwnerEnforced,
        "default object ownership should be BucketOwnerEnforced"
    );
}

#[tokio::test]
async fn test_put_get_delete_bucket_ownership_controls() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("ownership-crud-bucket")
        .send()
        .await
        .unwrap();

    client
        .put_bucket_ownership_controls()
        .bucket("ownership-crud-bucket")
        .ownership_controls(
            aws_sdk_s3::types::OwnershipControls::builder()
                .rules(
                    aws_sdk_s3::types::OwnershipControlsRule::builder()
                        .object_ownership(aws_sdk_s3::types::ObjectOwnership::BucketOwnerPreferred)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_bucket_ownership_controls should succeed");

    let resp = client
        .get_bucket_ownership_controls()
        .bucket("ownership-crud-bucket")
        .send()
        .await
        .expect("get_bucket_ownership_controls should succeed after put");

    let rules = resp
        .ownership_controls()
        .map(|oc| oc.rules())
        .unwrap_or_default();
    assert_eq!(
        rules[0].object_ownership(),
        &aws_sdk_s3::types::ObjectOwnership::BucketOwnerPreferred,
    );

    client
        .delete_bucket_ownership_controls()
        .bucket("ownership-crud-bucket")
        .send()
        .await
        .expect("delete_bucket_ownership_controls should succeed");

    // After delete, default BucketOwnerEnforced should be returned again
    let resp2 = client
        .get_bucket_ownership_controls()
        .bucket("ownership-crud-bucket")
        .send()
        .await
        .expect("get_bucket_ownership_controls should succeed after delete");

    let rules2 = resp2
        .ownership_controls()
        .map(|oc| oc.rules())
        .unwrap_or_default();
    assert_eq!(
        rules2[0].object_ownership(),
        &aws_sdk_s3::types::ObjectOwnership::BucketOwnerEnforced,
        "after delete, should revert to BucketOwnerEnforced default"
    );
}

// -----------------------------------------------------------------------
// Public Access Block
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_get_public_access_block_not_found() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("pab-nf-bucket")
        .send()
        .await
        .unwrap();

    let err = client
        .get_public_access_block()
        .bucket("pab-nf-bucket")
        .send()
        .await;

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("NoSuchPublicAccessBlockConfiguration"),
        "Expected NoSuchPublicAccessBlockConfiguration, got: {err_str}"
    );
}

#[tokio::test]
async fn test_put_get_delete_public_access_block() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("pab-crud-bucket")
        .send()
        .await
        .unwrap();

    client
        .put_public_access_block()
        .bucket("pab-crud-bucket")
        .public_access_block_configuration(
            aws_sdk_s3::types::PublicAccessBlockConfiguration::builder()
                .block_public_acls(true)
                .ignore_public_acls(true)
                .block_public_policy(true)
                .restrict_public_buckets(true)
                .build(),
        )
        .send()
        .await
        .expect("put_public_access_block should succeed");

    let resp = client
        .get_public_access_block()
        .bucket("pab-crud-bucket")
        .send()
        .await
        .expect("get_public_access_block should succeed after put");

    let cfg = resp
        .public_access_block_configuration()
        .expect("public access block config should be present");
    assert_eq!(cfg.block_public_acls(), Some(true));
    assert_eq!(cfg.ignore_public_acls(), Some(true));
    assert_eq!(cfg.block_public_policy(), Some(true));
    assert_eq!(cfg.restrict_public_buckets(), Some(true));

    client
        .delete_public_access_block()
        .bucket("pab-crud-bucket")
        .send()
        .await
        .expect("delete_public_access_block should succeed");

    let err = client
        .get_public_access_block()
        .bucket("pab-crud-bucket")
        .send()
        .await;
    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("NoSuchPublicAccessBlockConfiguration"),
        "After delete, expected NoSuchPublicAccessBlockConfiguration, got: {err_str}"
    );
}

// -----------------------------------------------------------------------
// Lifecycle (DELETE)
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_delete_bucket_lifecycle() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("lifecycle-del-bucket")
        .send()
        .await
        .unwrap();

    // First, put a lifecycle configuration
    let rule = aws_sdk_s3::types::LifecycleRule::builder()
        .id("expire-30days")
        .status(aws_sdk_s3::types::ExpirationStatus::Enabled)
        .expiration(
            aws_sdk_s3::types::LifecycleExpiration::builder()
                .days(30)
                .build(),
        )
        .filter(
            aws_sdk_s3::types::LifecycleRuleFilter::builder()
                .prefix("")
                .build(),
        )
        .build()
        .unwrap();

    client
        .put_bucket_lifecycle_configuration()
        .bucket("lifecycle-del-bucket")
        .lifecycle_configuration(
            aws_sdk_s3::types::BucketLifecycleConfiguration::builder()
                .rules(rule)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Verify it's there
    client
        .get_bucket_lifecycle_configuration()
        .bucket("lifecycle-del-bucket")
        .send()
        .await
        .expect("lifecycle should be present after put");

    // Delete it
    client
        .delete_bucket_lifecycle()
        .bucket("lifecycle-del-bucket")
        .send()
        .await
        .expect("delete_bucket_lifecycle should succeed");

    // Now get should return NoSuchLifecycleConfiguration
    let err = client
        .get_bucket_lifecycle_configuration()
        .bucket("lifecycle-del-bucket")
        .send()
        .await;

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("NoSuchLifecycleConfiguration"),
        "After delete, expected NoSuchLifecycleConfiguration, got: {err_str}"
    );
}

// -----------------------------------------------------------------------
// Full bucket lifecycle with sub-resources
// -----------------------------------------------------------------------

#[tokio::test]
async fn test_bucket_full_lifecycle_with_sub_resources() {
    let client = make_s3_client().await;

    // Create bucket
    client
        .create_bucket()
        .bucket("full-lifecycle-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");

    // Enable versioning
    client
        .put_bucket_versioning()
        .bucket("full-lifecycle-bucket")
        .versioning_configuration(
            aws_sdk_s3::types::VersioningConfiguration::builder()
                .status(aws_sdk_s3::types::BucketVersioningStatus::Enabled)
                .build(),
        )
        .send()
        .await
        .expect("put_bucket_versioning should succeed");

    // Add tags
    client
        .put_bucket_tagging()
        .bucket("full-lifecycle-bucket")
        .tagging(
            aws_sdk_s3::types::Tagging::builder()
                .tag_set(
                    aws_sdk_s3::types::Tag::builder()
                        .key("project")
                        .value("winterbaume")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_bucket_tagging should succeed");

    // Verify versioning
    let versioning = client
        .get_bucket_versioning()
        .bucket("full-lifecycle-bucket")
        .send()
        .await
        .expect("get_bucket_versioning should succeed");
    assert_eq!(
        versioning.status(),
        Some(&aws_sdk_s3::types::BucketVersioningStatus::Enabled)
    );

    // Verify tags
    let tagging = client
        .get_bucket_tagging()
        .bucket("full-lifecycle-bucket")
        .send()
        .await
        .expect("get_bucket_tagging should succeed");
    assert_eq!(tagging.tag_set().len(), 1);
    assert_eq!(tagging.tag_set()[0].key(), "project");

    // Delete bucket (must be empty)
    client
        .delete_bucket()
        .bucket("full-lifecycle-bucket")
        .send()
        .await
        .expect("delete_bucket should succeed");
}

// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================

// Covers FIX(terraform-e2e): All fields below were added for terraform compatibility.
// Terraform's AWS provider queries ~15 sub-resource APIs when reading an S3 bucket.
// Without these fields and their corresponding GET/PUT/DELETE handlers, terraform hangs or errors.
#[tokio::test]
async fn test_terraform_e2e_bucket_sub_resource_reads() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("terraform-compat-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");

    // GetBucketVersioning — returns empty config (no error)
    client
        .get_bucket_versioning()
        .bucket("terraform-compat-bucket")
        .send()
        .await
        .expect("get_bucket_versioning should succeed on fresh bucket");

    // GetBucketAcl — returns default FULL_CONTROL grant
    let acl = client
        .get_bucket_acl()
        .bucket("terraform-compat-bucket")
        .send()
        .await
        .expect("get_bucket_acl should succeed on fresh bucket");
    assert!(
        !acl.grants().is_empty(),
        "ACL should have at least one grant"
    );

    // GetBucketEncryption — returns default AES256 rule
    let enc = client
        .get_bucket_encryption()
        .bucket("terraform-compat-bucket")
        .send()
        .await
        .expect("get_bucket_encryption should succeed on fresh bucket");
    assert!(
        enc.server_side_encryption_configuration().is_some(),
        "encryption config should be returned even on fresh bucket"
    );

    // GetBucketRequestPayment — returns BucketOwner
    let rp = client
        .get_bucket_request_payment()
        .bucket("terraform-compat-bucket")
        .send()
        .await
        .expect("get_bucket_request_payment should succeed on fresh bucket");
    assert_eq!(rp.payer(), Some(&aws_sdk_s3::types::Payer::BucketOwner));

    // GetBucketOwnershipControls — returns BucketOwnerEnforced
    let oc = client
        .get_bucket_ownership_controls()
        .bucket("terraform-compat-bucket")
        .send()
        .await
        .expect("get_bucket_ownership_controls should succeed on fresh bucket");
    assert!(
        oc.ownership_controls().is_some(),
        "ownership controls should be returned even on fresh bucket"
    );

    // GetBucketPolicyStatus — returns IsPublic=false
    let ps = client
        .get_bucket_policy_status()
        .bucket("terraform-compat-bucket")
        .send()
        .await
        .expect("get_bucket_policy_status should succeed on fresh bucket");
    assert_eq!(
        ps.policy_status().and_then(|s| s.is_public),
        Some(false),
        "bucket should not be public by default"
    );

    // GetBucketLogging — returns empty BucketLoggingStatus
    let logging = client
        .get_bucket_logging()
        .bucket("terraform-compat-bucket")
        .send()
        .await
        .expect("get_bucket_logging should succeed on fresh bucket");
    assert!(
        logging.logging_enabled().is_none(),
        "logging should not be enabled on fresh bucket"
    );
}

#[tokio::test]
async fn test_multipart_upload_lifecycle() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("multipart-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");

    let multipart = client
        .create_multipart_upload()
        .bucket("multipart-bucket")
        .key("joined.txt")
        .content_type("text/plain")
        .send()
        .await
        .expect("create_multipart_upload should succeed");
    let upload_id = multipart
        .upload_id()
        .expect("upload id should be returned")
        .to_string();

    let part1 = client
        .upload_part()
        .bucket("multipart-bucket")
        .key("joined.txt")
        .upload_id(&upload_id)
        .part_number(1)
        .body(ByteStream::from_static(b"hello "))
        .send()
        .await
        .expect("upload_part 1 should succeed");
    let part2 = client
        .upload_part()
        .bucket("multipart-bucket")
        .key("joined.txt")
        .upload_id(&upload_id)
        .part_number(2)
        .body(ByteStream::from_static(b"world"))
        .send()
        .await
        .expect("upload_part 2 should succeed");

    let listed_parts = client
        .list_parts()
        .bucket("multipart-bucket")
        .key("joined.txt")
        .upload_id(&upload_id)
        .send()
        .await
        .expect("list_parts should succeed");
    assert_eq!(listed_parts.parts().len(), 2, "two parts should be listed");

    let completed_upload = aws_sdk_s3::types::CompletedMultipartUpload::builder()
        .set_parts(Some(vec![
            aws_sdk_s3::types::CompletedPart::builder()
                .part_number(1)
                .e_tag(part1.e_tag().unwrap_or_default())
                .build(),
            aws_sdk_s3::types::CompletedPart::builder()
                .part_number(2)
                .e_tag(part2.e_tag().unwrap_or_default())
                .build(),
        ]))
        .build();
    client
        .complete_multipart_upload()
        .bucket("multipart-bucket")
        .key("joined.txt")
        .upload_id(&upload_id)
        .multipart_upload(completed_upload)
        .send()
        .await
        .expect("complete_multipart_upload should succeed");

    let object = client
        .get_object()
        .bucket("multipart-bucket")
        .key("joined.txt")
        .send()
        .await
        .expect("get_object should succeed after multipart completion");
    let body = object.body.collect().await.unwrap().into_bytes();
    assert_eq!(body.as_ref(), b"hello world");
}

#[tokio::test]
async fn test_abort_multipart_upload_removes_upload() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("abort-multipart-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");

    let multipart = client
        .create_multipart_upload()
        .bucket("abort-multipart-bucket")
        .key("pending.txt")
        .send()
        .await
        .expect("create_multipart_upload should succeed");
    let upload_id = multipart
        .upload_id()
        .expect("upload id should be returned")
        .to_string();

    let uploads = client
        .list_multipart_uploads()
        .bucket("abort-multipart-bucket")
        .send()
        .await
        .expect("list_multipart_uploads should succeed");
    assert_eq!(
        uploads.uploads().len(),
        1,
        "one multipart upload should exist"
    );

    client
        .abort_multipart_upload()
        .bucket("abort-multipart-bucket")
        .key("pending.txt")
        .upload_id(&upload_id)
        .send()
        .await
        .expect("abort_multipart_upload should succeed");

    let uploads = client
        .list_multipart_uploads()
        .bucket("abort-multipart-bucket")
        .send()
        .await
        .expect("list_multipart_uploads should still succeed");
    assert!(
        uploads.uploads().is_empty(),
        "multipart uploads should be cleared"
    );
}

#[tokio::test]
async fn test_list_object_versions_returns_current_object() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("versions-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");
    client
        .put_object()
        .bucket("versions-bucket")
        .key("doc.txt")
        .body(ByteStream::from_static(b"versioned"))
        .send()
        .await
        .expect("put_object should succeed");

    let versions = client
        .list_object_versions()
        .bucket("versions-bucket")
        .send()
        .await
        .expect("list_object_versions should succeed");
    assert_eq!(
        versions.versions().len(),
        1,
        "one current object version should be returned"
    );
    assert_eq!(
        versions.versions()[0].key().unwrap_or_default(),
        "doc.txt",
        "the returned version should match the uploaded key"
    );
}

// --- Analytics Configuration Tests ---

#[tokio::test]
async fn test_put_get_delete_bucket_analytics_configuration() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("analytics-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");

    // Put analytics configuration
    client
        .put_bucket_analytics_configuration()
        .bucket("analytics-bucket")
        .id("my-analytics")
        .analytics_configuration(
            aws_sdk_s3::types::AnalyticsConfiguration::builder()
                .id("my-analytics")
                .storage_class_analysis(aws_sdk_s3::types::StorageClassAnalysis::builder().build())
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_bucket_analytics_configuration should succeed");

    // Get analytics configuration
    let resp = client
        .get_bucket_analytics_configuration()
        .bucket("analytics-bucket")
        .id("my-analytics")
        .send()
        .await
        .expect("get_bucket_analytics_configuration should succeed");
    assert_eq!(
        resp.analytics_configuration().map(|c| c.id()),
        Some("my-analytics"),
        "analytics config id should match"
    );

    // List analytics configurations
    let list_resp = client
        .list_bucket_analytics_configurations()
        .bucket("analytics-bucket")
        .send()
        .await
        .expect("list_bucket_analytics_configurations should succeed");
    assert_eq!(
        list_resp.analytics_configuration_list().len(),
        1,
        "one analytics configuration should be listed"
    );

    // Delete analytics configuration
    client
        .delete_bucket_analytics_configuration()
        .bucket("analytics-bucket")
        .id("my-analytics")
        .send()
        .await
        .expect("delete_bucket_analytics_configuration should succeed");

    // List should be empty after delete
    let list_resp = client
        .list_bucket_analytics_configurations()
        .bucket("analytics-bucket")
        .send()
        .await
        .expect("list_bucket_analytics_configurations should succeed after delete");
    assert_eq!(
        list_resp.analytics_configuration_list().len(),
        0,
        "analytics configuration list should be empty after delete"
    );
}

// --- Intelligent Tiering Tests ---

#[tokio::test]
async fn test_put_get_delete_bucket_intelligent_tiering_configuration() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("itier-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");

    // Put intelligent tiering configuration
    client
        .put_bucket_intelligent_tiering_configuration()
        .bucket("itier-bucket")
        .id("my-tiering")
        .intelligent_tiering_configuration(
            aws_sdk_s3::types::IntelligentTieringConfiguration::builder()
                .id("my-tiering")
                .status(aws_sdk_s3::types::IntelligentTieringStatus::Enabled)
                .tierings(
                    aws_sdk_s3::types::Tiering::builder()
                        .days(90)
                        .access_tier(aws_sdk_s3::types::IntelligentTieringAccessTier::ArchiveAccess)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_bucket_intelligent_tiering_configuration should succeed");

    // Get intelligent tiering configuration
    let resp = client
        .get_bucket_intelligent_tiering_configuration()
        .bucket("itier-bucket")
        .id("my-tiering")
        .send()
        .await
        .expect("get_bucket_intelligent_tiering_configuration should succeed");
    assert_eq!(
        resp.intelligent_tiering_configuration().map(|c| c.id()),
        Some("my-tiering"),
        "intelligent tiering config id should match"
    );

    // List configurations
    let list_resp = client
        .list_bucket_intelligent_tiering_configurations()
        .bucket("itier-bucket")
        .send()
        .await
        .expect("list_bucket_intelligent_tiering_configurations should succeed");
    assert_eq!(
        list_resp.intelligent_tiering_configuration_list().len(),
        1,
        "one intelligent tiering configuration should be listed"
    );

    // Delete
    client
        .delete_bucket_intelligent_tiering_configuration()
        .bucket("itier-bucket")
        .id("my-tiering")
        .send()
        .await
        .expect("delete_bucket_intelligent_tiering_configuration should succeed");

    // List should now be empty
    let list_resp = client
        .list_bucket_intelligent_tiering_configurations()
        .bucket("itier-bucket")
        .send()
        .await
        .expect("list should succeed after delete");
    assert_eq!(
        list_resp.intelligent_tiering_configuration_list().len(),
        0,
        "intelligent tiering list should be empty after delete"
    );
}

// --- Metrics Configuration Tests ---

#[tokio::test]
async fn test_put_get_delete_bucket_metrics_configuration() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("metrics-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");

    // Put metrics configuration
    client
        .put_bucket_metrics_configuration()
        .bucket("metrics-bucket")
        .id("my-metrics")
        .metrics_configuration(
            aws_sdk_s3::types::MetricsConfiguration::builder()
                .id("my-metrics")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_bucket_metrics_configuration should succeed");

    // Get metrics configuration
    let resp = client
        .get_bucket_metrics_configuration()
        .bucket("metrics-bucket")
        .id("my-metrics")
        .send()
        .await
        .expect("get_bucket_metrics_configuration should succeed");
    assert_eq!(
        resp.metrics_configuration().map(|c| c.id()),
        Some("my-metrics"),
        "metrics config id should match"
    );

    // List metrics configurations
    let list_resp = client
        .list_bucket_metrics_configurations()
        .bucket("metrics-bucket")
        .send()
        .await
        .expect("list_bucket_metrics_configurations should succeed");
    assert_eq!(
        list_resp.metrics_configuration_list().len(),
        1,
        "one metrics configuration should be listed"
    );

    // Delete metrics configuration
    client
        .delete_bucket_metrics_configuration()
        .bucket("metrics-bucket")
        .id("my-metrics")
        .send()
        .await
        .expect("delete_bucket_metrics_configuration should succeed");

    // List should be empty after delete
    let list_resp = client
        .list_bucket_metrics_configurations()
        .bucket("metrics-bucket")
        .send()
        .await
        .expect("list should succeed after delete");
    assert_eq!(
        list_resp.metrics_configuration_list().len(),
        0,
        "metrics configuration list should be empty after delete"
    );
}

// --- Inventory Delete Test ---

#[tokio::test]
async fn test_delete_bucket_inventory_configuration() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("inv-del-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");

    // Put inventory configuration
    client
        .put_bucket_inventory_configuration()
        .bucket("inv-del-bucket")
        .id("inv-to-delete")
        .inventory_configuration(
            aws_sdk_s3::types::InventoryConfiguration::builder()
                .id("inv-to-delete")
                .is_enabled(true)
                .included_object_versions(aws_sdk_s3::types::InventoryIncludedObjectVersions::All)
                .destination(
                    aws_sdk_s3::types::InventoryDestination::builder()
                        .s3_bucket_destination(
                            aws_sdk_s3::types::InventoryS3BucketDestination::builder()
                                .bucket("arn:aws:s3:::inv-del-bucket")
                                .format(aws_sdk_s3::types::InventoryFormat::Csv)
                                .build()
                                .unwrap(),
                        )
                        .build(),
                )
                .schedule(
                    aws_sdk_s3::types::InventorySchedule::builder()
                        .frequency(aws_sdk_s3::types::InventoryFrequency::Daily)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_bucket_inventory_configuration should succeed");

    // Delete the inventory configuration
    client
        .delete_bucket_inventory_configuration()
        .bucket("inv-del-bucket")
        .id("inv-to-delete")
        .send()
        .await
        .expect("delete_bucket_inventory_configuration should succeed");

    // List should be empty
    let list_resp = client
        .list_bucket_inventory_configurations()
        .bucket("inv-del-bucket")
        .send()
        .await
        .expect("list_bucket_inventory_configurations should succeed");
    assert!(
        list_resp.inventory_configuration_list().is_empty(),
        "inventory list should be empty after delete"
    );
}

// Note: ListDirectoryBuckets uses a different endpoint (s3express-control.*.amazonaws.com)
// and cannot be tested via the standard S3 client URL patterns without additional
// URL pattern registration. The handler is implemented but not tested via SDK here.

// --- Create Session Test ---

#[tokio::test]
async fn test_create_session() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("session-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");

    let resp = client
        .create_session()
        .bucket("session-bucket")
        .send()
        .await
        .expect("create_session should succeed");
    assert!(
        resp.credentials().is_some(),
        "credentials should be returned"
    );
}

// --- Get Object Torrent Test ---

#[tokio::test]
async fn test_get_object_torrent() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("torrent-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");

    client
        .put_object()
        .bucket("torrent-bucket")
        .key("file.bin")
        .body(aws_sdk_s3::primitives::ByteStream::from_static(b"data"))
        .send()
        .await
        .expect("put_object should succeed");

    client
        .get_object_torrent()
        .bucket("torrent-bucket")
        .key("file.bin")
        .send()
        .await
        .expect("get_object_torrent should succeed");
}

// --- Rename Object Test ---

#[tokio::test]
async fn test_rename_object() {
    let client = make_s3_client().await;

    client
        .create_bucket()
        .bucket("rename-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");

    client
        .put_object()
        .bucket("rename-bucket")
        .key("original.txt")
        .body(aws_sdk_s3::primitives::ByteStream::from_static(b"hello"))
        .send()
        .await
        .expect("put_object should succeed");

    client
        .rename_object()
        .bucket("rename-bucket")
        .key("renamed.txt")
        .rename_source("/rename-bucket/original.txt")
        .send()
        .await
        .expect("rename_object should succeed");

    // Renamed key should now exist
    client
        .head_object()
        .bucket("rename-bucket")
        .key("renamed.txt")
        .send()
        .await
        .expect("head_object for renamed key should succeed");

    // Original key should be gone
    let err = client
        .head_object()
        .bucket("rename-bucket")
        .key("original.txt")
        .send()
        .await;
    assert!(err.is_err(), "original key should not exist after rename");
}

// ---------------------------------------------------------------------------
// S3 Control dispatch (FIX(terraform-e2e)) — requests arriving with /v20180820/ paths
// ---------------------------------------------------------------------------

/// Test that S3Control tag-management requests routed through S3Service are handled
/// correctly. Terraform's AWS provider calls S3Control ListTagsForResource using the
/// "s3" credential-scope service name, so those requests land in S3Service via the
/// /v20180820/ path prefix guard in dispatch().
#[tokio::test]
async fn test_s3control_dispatch_list_tags_for_resource() {
    use winterbaume_core::{MockRequest, MockService};
    use winterbaume_s3::S3Service;

    let svc = S3Service::new();
    // Simulate a GET /v20180820/tags/{arn} request (ListTagsForResource)
    let request = MockRequest {
        method: "GET".to_string(),
        uri: "https://s3.us-east-1.amazonaws.com/v20180820/tags/arn%3Aaws%3As3%3Aus-east-1%3A123456789012%3Aaccesspoint%2Fmy-ap".to_string(),
        headers: http::HeaderMap::new(),
        body: bytes::Bytes::new(),
    };
    let response = svc.handle(request).await;
    assert_eq!(
        response.status, 200,
        "ListTagsForResource should return 200"
    );
    let body = String::from_utf8_lossy(&response.body);
    assert!(
        body.contains("Tags"),
        "ListTagsForResource response should contain Tags key"
    );
}

#[tokio::test]
async fn test_s3control_dispatch_put_tags_for_resource() {
    use winterbaume_core::{MockRequest, MockService};
    use winterbaume_s3::S3Service;

    let svc = S3Service::new();
    // Simulate a PUT /v20180820/tags/{arn} request (PutTagsForResource)
    let request = MockRequest {
        method: "PUT".to_string(),
        uri: "https://s3.us-east-1.amazonaws.com/v20180820/tags/arn%3Aaws%3As3%3Aus-east-1%3A123456789012%3Aaccesspoint%2Fmy-ap".to_string(),
        headers: http::HeaderMap::new(),
        body: bytes::Bytes::from(r#"{"Tags":{"Key":"Value"}}"#),
    };
    let response = svc.handle(request).await;
    assert_eq!(response.status, 204, "PutTagsForResource should return 204");
}

#[tokio::test]
async fn test_s3control_dispatch_delete_tags_for_resource() {
    use winterbaume_core::{MockRequest, MockService};
    use winterbaume_s3::S3Service;

    let svc = S3Service::new();
    // Simulate a DELETE /v20180820/tags/{arn} request (DeleteTagsForResource)
    let request = MockRequest {
        method: "DELETE".to_string(),
        uri: "https://s3.us-east-1.amazonaws.com/v20180820/tags/arn%3Aaws%3As3%3Aus-east-1%3A123456789012%3Aaccesspoint%2Fmy-ap".to_string(),
        headers: http::HeaderMap::new(),
        body: bytes::Bytes::new(),
    };
    let response = svc.handle(request).await;
    assert_eq!(
        response.status, 204,
        "DeleteTagsForResource should return 204"
    );
}

// ---------------------------------------------------------------------------
// Regression tests for reported bugs.
// ---------------------------------------------------------------------------

/// Regression for GitHub issue #3: HeadBucket on a missing bucket returned an
/// XML error body, which forced aws-sdk-rust to fall back to the Unhandled
/// error variant instead of resolving HeadBucketError::NotFound. Real S3
/// documents that 4xx responses for HeadBucket carry no body.
#[tokio::test]
async fn test_head_bucket_4xx_has_no_body_and_resolves_typed_not_found() {
    use aws_sdk_s3::operation::head_bucket::HeadBucketError;

    let client = make_s3_client().await;
    let err = client
        .head_bucket()
        .bucket("does-not-exist")
        .send()
        .await
        .expect_err("head_bucket on missing bucket must fail");

    if let aws_sdk_s3::error::SdkError::ServiceError(svc) = &err {
        let raw = svc.raw();
        assert_eq!(raw.status().as_u16(), 404);
        let body_bytes = raw.body().bytes().unwrap_or(&[]);
        assert!(
            body_bytes.is_empty(),
            "HeadBucket 4xx response must carry no body; got {} bytes: {:?}",
            body_bytes.len(),
            std::str::from_utf8(body_bytes).unwrap_or("<non-utf8>")
        );
    } else {
        panic!("expected ServiceError, got {err:?}");
    }

    assert!(
        matches!(err.as_service_error(), Some(HeadBucketError::NotFound(_))),
        "expected typed HeadBucketError::NotFound, got {:?}",
        err.as_service_error()
    );
}

/// Regression for GitHub issue #4: PutObject with `If-None-Match: *` on a key
/// that already exists silently overwrote the object instead of returning
/// 412 PreconditionFailed.
#[tokio::test]
async fn test_put_object_if_none_match_star_rejects_existing_key() {
    let client = make_s3_client().await;
    client
        .create_bucket()
        .bucket("cond-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");

    let first = client
        .put_object()
        .bucket("cond-bucket")
        .key("lock")
        .if_none_match("*")
        .body(ByteStream::from_static(b"v1"))
        .send()
        .await
        .expect("first conditional PUT should succeed");
    let first_etag = first.e_tag.expect("first PUT must return ETag");

    let err = client
        .put_object()
        .bucket("cond-bucket")
        .key("lock")
        .if_none_match("*")
        .body(ByteStream::from_static(b"v2"))
        .send()
        .await
        .expect_err("second conditional PUT must fail");

    let svc_err = err.as_service_error().expect("ServiceError expected");
    let meta = svc_err.meta();
    assert_eq!(meta.code(), Some("PreconditionFailed"));
    if let aws_sdk_s3::error::SdkError::ServiceError(svc) = &err {
        assert_eq!(svc.raw().status().as_u16(), 412);
    } else {
        panic!("expected ServiceError, got {err:?}");
    }

    // The first object must still be intact.
    let head = client
        .head_object()
        .bucket("cond-bucket")
        .key("lock")
        .send()
        .await
        .expect("head_object should succeed after rejected overwrite");
    assert_eq!(head.e_tag.as_deref(), Some(first_etag.as_str()));
}

/// `If-Match: <etag>` succeeds when the stored ETag matches and fails with
/// 412 when it does not.
#[tokio::test]
async fn test_put_object_if_match_enforced() {
    let client = make_s3_client().await;
    client
        .create_bucket()
        .bucket("ifmatch-bucket")
        .send()
        .await
        .unwrap();

    let v1 = client
        .put_object()
        .bucket("ifmatch-bucket")
        .key("k")
        .body(ByteStream::from_static(b"v1"))
        .send()
        .await
        .unwrap();
    let v1_etag = v1.e_tag.unwrap();

    // Matching ETag → overwrite succeeds.
    client
        .put_object()
        .bucket("ifmatch-bucket")
        .key("k")
        .if_match(&v1_etag)
        .body(ByteStream::from_static(b"v2"))
        .send()
        .await
        .expect("If-Match with current ETag must succeed");

    // Stale ETag → 412.
    let err = client
        .put_object()
        .bucket("ifmatch-bucket")
        .key("k")
        .if_match(&v1_etag)
        .body(ByteStream::from_static(b"v3"))
        .send()
        .await
        .expect_err("If-Match with stale ETag must fail");
    let svc_err = err.as_service_error().expect("ServiceError expected");
    assert_eq!(svc_err.meta().code(), Some("PreconditionFailed"));

    // If-Match on a missing key → 412.
    let err = client
        .put_object()
        .bucket("ifmatch-bucket")
        .key("absent")
        .if_match("\"deadbeef\"")
        .body(ByteStream::from_static(b"x"))
        .send()
        .await
        .expect_err("If-Match on missing key must fail");
    let svc_err = err.as_service_error().expect("ServiceError expected");
    assert_eq!(svc_err.meta().code(), Some("PreconditionFailed"));
}

// ---------------------------------------------------------------------------
// Conditional surfaces on the other S3 operations modelled by the HTTP
// Bindings extractor: CompleteMultipartUpload, CopyObject, DeleteObject,
// GetObject, HeadObject.
// ---------------------------------------------------------------------------

/// CompleteMultipartUpload must enforce `If-None-Match: *` on the destination
/// key just like PutObject. A racing finalisation of a multipart upload
/// against an already-existing key must return 412.
#[tokio::test]
async fn test_complete_multipart_upload_if_none_match_star() {
    let client = make_s3_client().await;
    client
        .create_bucket()
        .bucket("mpu-cond")
        .send()
        .await
        .unwrap();

    // Pre-populate the destination key.
    client
        .put_object()
        .bucket("mpu-cond")
        .key("file")
        .body(ByteStream::from_static(b"already-here"))
        .send()
        .await
        .unwrap();

    let create = client
        .create_multipart_upload()
        .bucket("mpu-cond")
        .key("file")
        .send()
        .await
        .unwrap();
    let upload_id = create.upload_id.unwrap();
    let part = client
        .upload_part()
        .bucket("mpu-cond")
        .key("file")
        .upload_id(&upload_id)
        .part_number(1)
        .body(ByteStream::from_static(
            &[0u8; 5 * 1024 * 1024], // minimum part size
        ))
        .send()
        .await
        .unwrap();
    let part_etag = part.e_tag.unwrap();

    let err = client
        .complete_multipart_upload()
        .bucket("mpu-cond")
        .key("file")
        .upload_id(&upload_id)
        .if_none_match("*")
        .multipart_upload(
            aws_sdk_s3::types::CompletedMultipartUpload::builder()
                .parts(
                    aws_sdk_s3::types::CompletedPart::builder()
                        .part_number(1)
                        .e_tag(part_etag)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect_err("CompleteMultipartUpload with If-None-Match: * must fail");
    let svc_err = err.as_service_error().expect("ServiceError expected");
    assert_eq!(svc_err.meta().code(), Some("PreconditionFailed"));
}

/// CopyObject must reject when `x-amz-copy-source-if-match` doesn't match the
/// source ETag (source-side conditional → 412 PreconditionFailed).
#[tokio::test]
async fn test_copy_object_source_if_match_mismatch() {
    let client = make_s3_client().await;
    client
        .create_bucket()
        .bucket("copy-cond")
        .send()
        .await
        .unwrap();
    client
        .put_object()
        .bucket("copy-cond")
        .key("src")
        .body(ByteStream::from_static(b"hello"))
        .send()
        .await
        .unwrap();

    let err = client
        .copy_object()
        .bucket("copy-cond")
        .key("dst")
        .copy_source("copy-cond/src")
        .copy_source_if_match("\"deadbeef\"")
        .send()
        .await
        .expect_err("CopyObject with mismatched x-amz-copy-source-if-match must fail");
    let svc_err = err.as_service_error().expect("ServiceError expected");
    assert_eq!(svc_err.meta().code(), Some("PreconditionFailed"));

    // The destination must not exist after a rejected copy.
    let head = client
        .head_object()
        .bucket("copy-cond")
        .key("dst")
        .send()
        .await;
    assert!(head.is_err(), "rejected copy must not create destination");
}

/// CopyObject must reject when destination `If-None-Match: *` and the
/// destination key already exists.
#[tokio::test]
async fn test_copy_object_destination_if_none_match_star() {
    let client = make_s3_client().await;
    client
        .create_bucket()
        .bucket("copy-cond2")
        .send()
        .await
        .unwrap();
    client
        .put_object()
        .bucket("copy-cond2")
        .key("src")
        .body(ByteStream::from_static(b"hello"))
        .send()
        .await
        .unwrap();
    client
        .put_object()
        .bucket("copy-cond2")
        .key("dst")
        .body(ByteStream::from_static(b"existing"))
        .send()
        .await
        .unwrap();

    let err = client
        .copy_object()
        .bucket("copy-cond2")
        .key("dst")
        .copy_source("copy-cond2/src")
        .if_none_match("*")
        .send()
        .await
        .expect_err("CopyObject with destination If-None-Match: * onto existing key must fail");
    let svc_err = err.as_service_error().expect("ServiceError expected");
    assert_eq!(svc_err.meta().code(), Some("PreconditionFailed"));
}

/// DeleteObject must enforce `If-Match` on the stored ETag.
#[tokio::test]
async fn test_delete_object_if_match_enforced() {
    let client = make_s3_client().await;
    client
        .create_bucket()
        .bucket("del-cond")
        .send()
        .await
        .unwrap();
    let put = client
        .put_object()
        .bucket("del-cond")
        .key("k")
        .body(ByteStream::from_static(b"v1"))
        .send()
        .await
        .unwrap();
    let etag = put.e_tag.unwrap();

    // Mismatch → 412 PreconditionFailed.
    let err = client
        .delete_object()
        .bucket("del-cond")
        .key("k")
        .if_match("\"deadbeef\"")
        .send()
        .await
        .expect_err("conditional delete with stale ETag must fail");
    let svc_err = err.as_service_error().expect("ServiceError expected");
    assert_eq!(svc_err.meta().code(), Some("PreconditionFailed"));

    // Object must still exist after rejected delete.
    client
        .head_object()
        .bucket("del-cond")
        .key("k")
        .send()
        .await
        .expect("object must still exist after rejected conditional delete");

    // Matching ETag → delete succeeds.
    client
        .delete_object()
        .bucket("del-cond")
        .key("k")
        .if_match(&etag)
        .send()
        .await
        .expect("conditional delete with current ETag must succeed");
}

/// GetObject with `If-None-Match` matching the current ETag must return 304
/// Not Modified (cache-validation path). The SDK surfaces 304 as an error.
#[tokio::test]
async fn test_get_object_if_none_match_returns_304() {
    let client = make_s3_client().await;
    client
        .create_bucket()
        .bucket("get-cond")
        .send()
        .await
        .unwrap();
    let put = client
        .put_object()
        .bucket("get-cond")
        .key("k")
        .body(ByteStream::from_static(b"data"))
        .send()
        .await
        .unwrap();
    let etag = put.e_tag.unwrap();

    let err = client
        .get_object()
        .bucket("get-cond")
        .key("k")
        .if_none_match(&etag)
        .send()
        .await
        .expect_err("GET with matching If-None-Match must surface 304");
    if let aws_sdk_s3::error::SdkError::ServiceError(svc) = &err {
        assert_eq!(svc.raw().status().as_u16(), 304);
    } else {
        panic!("expected ServiceError, got {err:?}");
    }
}

/// GetObject with `If-Match` mismatching the current ETag must return 412.
#[tokio::test]
async fn test_get_object_if_match_mismatch_returns_412() {
    let client = make_s3_client().await;
    client
        .create_bucket()
        .bucket("get-cond2")
        .send()
        .await
        .unwrap();
    client
        .put_object()
        .bucket("get-cond2")
        .key("k")
        .body(ByteStream::from_static(b"data"))
        .send()
        .await
        .unwrap();

    let err = client
        .get_object()
        .bucket("get-cond2")
        .key("k")
        .if_match("\"deadbeef\"")
        .send()
        .await
        .expect_err("GET with stale If-Match must fail");
    let svc_err = err.as_service_error().expect("ServiceError expected");
    assert_eq!(svc_err.meta().code(), Some("PreconditionFailed"));
    if let aws_sdk_s3::error::SdkError::ServiceError(svc) = &err {
        assert_eq!(svc.raw().status().as_u16(), 412);
    }
}

/// HeadObject must honour `If-None-Match` (304) and `If-Match` (412), and on
/// 4xx the response body must be empty so aws-sdk-rust resolves the typed
/// `HeadObjectError::NotFound` / Precondition error variant cleanly.
#[tokio::test]
async fn test_head_object_conditional_headers() {
    let client = make_s3_client().await;
    client
        .create_bucket()
        .bucket("head-cond")
        .send()
        .await
        .unwrap();
    let put = client
        .put_object()
        .bucket("head-cond")
        .key("k")
        .body(ByteStream::from_static(b"data"))
        .send()
        .await
        .unwrap();
    let etag = put.e_tag.unwrap();

    // 304 path.
    let err = client
        .head_object()
        .bucket("head-cond")
        .key("k")
        .if_none_match(&etag)
        .send()
        .await
        .expect_err("HEAD with matching If-None-Match must surface 304");
    if let aws_sdk_s3::error::SdkError::ServiceError(svc) = &err {
        assert_eq!(svc.raw().status().as_u16(), 304);
        assert!(
            svc.raw().body().bytes().unwrap_or(&[]).is_empty(),
            "HEAD 304 must carry no body"
        );
    }

    // 412 path with empty body.
    let err = client
        .head_object()
        .bucket("head-cond")
        .key("k")
        .if_match("\"deadbeef\"")
        .send()
        .await
        .expect_err("HEAD with stale If-Match must fail");
    if let aws_sdk_s3::error::SdkError::ServiceError(svc) = &err {
        assert_eq!(svc.raw().status().as_u16(), 412);
        assert!(
            svc.raw().body().bytes().unwrap_or(&[]).is_empty(),
            "HEAD 412 must carry no body (issue #3 regression)"
        );
    }
}

// ---------------------------------------------------------------------------
// BlobBackedService smoke tests
// ---------------------------------------------------------------------------

mod blob_backed {
    use std::collections::HashMap;
    use std::pin::Pin;
    use std::sync::Arc;

    use bytes::Bytes;
    use tokio::io::AsyncReadExt;
    use winterbaume_core::{
        BlobBackedService, BlobExportEntry, BlobSource, BlobStore, BlobVisitor, MemVfs,
        StatefulService, VfsError,
    };
    use winterbaume_s3::S3Service;
    use winterbaume_s3::S3StateView;

    /// Collector that accumulates exported blobs into a HashMap.
    /// Demonstrates that BlobVisitor::visit can borrow &mut self across .await.
    struct BlobCollector {
        blobs: HashMap<String, Bytes>,
    }

    impl BlobCollector {
        fn new() -> Self {
            Self {
                blobs: HashMap::new(),
            }
        }
    }

    impl BlobVisitor for BlobCollector {
        fn visit(
            &mut self,
            batch: Vec<BlobExportEntry>,
        ) -> Pin<Box<dyn std::future::Future<Output = Result<(), VfsError>> + Send + '_>> {
            Box::pin(async move {
                for mut entry in batch {
                    let mut buf = Vec::new();
                    entry
                        .reader
                        .read_to_end(&mut buf)
                        .await
                        .map_err(VfsError::Io)?;
                    self.blobs.insert(entry.key.clone(), Bytes::from(buf));
                }
                Ok(())
            })
        }
    }

    /// Source that serves blob data from a HashMap.
    struct MapBlobSource {
        data: HashMap<String, Bytes>,
    }

    impl BlobSource for MapBlobSource {
        fn fetch(
            &mut self,
            key: String,
        ) -> Pin<
            Box<
                dyn std::future::Future<
                        Output = Result<Box<dyn tokio::io::AsyncRead + Send + Unpin>, VfsError>,
                    > + Send
                    + '_,
            >,
        > {
            Box::pin(async move {
                let bytes = self
                    .data
                    .get(&key)
                    .cloned()
                    .ok_or(VfsError::NotFound(key))?;
                Ok(Box::new(std::io::Cursor::new(bytes))
                    as Box<dyn tokio::io::AsyncRead + Send + Unpin>)
            })
        }
    }

    fn make_service_with_shared_vfs() -> (S3Service, BlobStore) {
        let vfs = Arc::new(MemVfs::new());
        let svc = S3Service::with_vfs(vfs.clone());
        // Use the scoped namespace matching the account/region used by tests.
        let blobs = BlobStore::new(vfs, "s3/111111111111/us-east-1");
        (svc, blobs)
    }

    fn seed_view(blob_key: &str) -> S3StateView {
        use winterbaume_s3::views::{BucketStateView, ObjectView};
        let mut objects = HashMap::new();
        objects.insert(
            "hello.txt".to_string(),
            ObjectView {
                key: "hello.txt".to_string(),
                blob_key: blob_key.to_string(),
                content_length: 5,
                content_type: "text/plain".to_string(),
                etag: "\"abc\"".to_string(),
                last_modified: Some("2026-01-01T00:00:00Z".to_string()),
                storage_class: "STANDARD".to_string(),
                metadata: vec![],
                tags: HashMap::new(),
                acl: None,
                legal_hold_status: None,
                retention_mode: None,
                retain_until_date: None,
                version_id: "null".to_string(),
                blob_version_id: String::new(),
            },
        );
        let mut buckets = HashMap::new();
        buckets.insert(
            "test-bucket".to_string(),
            BucketStateView {
                name: "test-bucket".to_string(),
                region: "us-east-1".to_string(),
                objects,
                ..BucketStateView::default()
            },
        );
        S3StateView { buckets }
    }

    #[tokio::test]
    async fn snapshot_with_blobs_exports_blob_data() {
        let (svc, blobs) = make_service_with_shared_vfs();
        let blob_key = "test-bucket/hello.txt";

        // Seed blob data and metadata.
        blobs
            .put(blob_key, Bytes::from_static(b"hello"))
            .await
            .unwrap();
        svc.restore("111111111111", "us-east-1", seed_view(blob_key))
            .await
            .unwrap();

        // Export via snapshot_with_blobs — visitor borrows &mut self across .await.
        let mut collector = BlobCollector::new();
        let view = svc
            .snapshot_with_blobs("111111111111", "us-east-1", &mut collector)
            .await
            .unwrap();

        assert_eq!(view.buckets.len(), 1);
        assert!(
            collector.blobs.contains_key(blob_key),
            "blob should be exported"
        );
        assert_eq!(collector.blobs[blob_key], Bytes::from_static(b"hello"));
    }

    #[tokio::test]
    async fn restore_with_blobs_imports_blob_data() {
        let (svc, blobs) = make_service_with_shared_vfs();
        let blob_key = "test-bucket/hello.txt";
        let view = seed_view(blob_key);

        let mut source = MapBlobSource {
            data: HashMap::from([(blob_key.to_string(), Bytes::from_static(b"hello"))]),
        };

        svc.restore_with_blobs("111111111111", "us-east-1", view, &mut source)
            .await
            .unwrap();

        // Verify the blob landed in the store and metadata is correct.
        let restored = blobs.get(blob_key).await.unwrap();
        assert_eq!(restored, Bytes::from_static(b"hello"));

        let snap = svc.snapshot("111111111111", "us-east-1").await;
        assert_eq!(snap.buckets.len(), 1);
        assert_eq!(
            snap.buckets["test-bucket"].objects["hello.txt"].blob_key,
            blob_key
        );
    }

    #[tokio::test]
    async fn roundtrip_snapshot_restore_with_blobs() {
        let (src_svc, src_blobs) = make_service_with_shared_vfs();
        let blob_key = "test-bucket/greeting.txt";

        // Seed source.
        src_blobs
            .put(blob_key, Bytes::from_static(b"goodbye"))
            .await
            .unwrap();
        src_svc
            .restore("111111111111", "us-east-1", seed_view(blob_key))
            .await
            .unwrap();

        // Export.
        let mut collector = BlobCollector::new();
        let view = src_svc
            .snapshot_with_blobs("111111111111", "us-east-1", &mut collector)
            .await
            .unwrap();

        // Import into a fresh service.
        let (dst_svc, dst_blobs) = make_service_with_shared_vfs();
        let mut source = MapBlobSource {
            data: collector.blobs,
        };
        dst_svc
            .restore_with_blobs("111111111111", "us-east-1", view, &mut source)
            .await
            .unwrap();

        // Verify consistency.
        let dst_view = dst_svc.snapshot("111111111111", "us-east-1").await;
        assert_eq!(dst_view.buckets.len(), 1);
        assert_eq!(
            dst_view.buckets["test-bucket"].objects["hello.txt"].blob_key,
            blob_key
        );
        let blob = dst_blobs.get(blob_key).await.unwrap();
        assert_eq!(blob, Bytes::from_static(b"goodbye"));
    }

    #[tokio::test]
    async fn snapshot_with_blobs_empty_state_exports_nothing() {
        let (svc, _blobs) = make_service_with_shared_vfs();

        let mut collector = BlobCollector::new();
        let view = svc
            .snapshot_with_blobs("111111111111", "us-east-1", &mut collector)
            .await
            .unwrap();

        assert!(view.buckets.is_empty());
        assert!(collector.blobs.is_empty());
    }
}
