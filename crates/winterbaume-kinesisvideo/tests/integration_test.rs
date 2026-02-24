use aws_sdk_kinesisvideo::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kinesisvideo::KinesisVideoService;

async fn make_kinesis_video_client() -> aws_sdk_kinesisvideo::Client {
    let mock = MockAws::builder()
        .with_service(KinesisVideoService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kinesisvideo::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_kinesisvideo::Client::new(&config)
}

#[tokio::test]
async fn test_create_stream() {
    let client = make_kinesis_video_client().await;

    let resp = client
        .create_stream()
        .stream_name("test-stream")
        .send()
        .await
        .expect("create_stream should succeed");

    let arn = resp.stream_arn().expect("StreamARN should be present");
    assert!(
        arn.contains("test-stream"),
        "ARN should contain stream name"
    );
}

#[tokio::test]
async fn test_create_stream_with_options() {
    let client = make_kinesis_video_client().await;

    let resp = client
        .create_stream()
        .stream_name("my-video-stream")
        .device_name("my-camera")
        .media_type("video/h264")
        .data_retention_in_hours(24)
        .send()
        .await
        .expect("create_stream with options should succeed");

    let arn = resp.stream_arn().expect("StreamARN should be present");
    assert!(arn.contains("my-video-stream"));
}

#[tokio::test]
async fn test_describe_stream() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("describe-me")
        .device_name("camera-1")
        .media_type("video/h264")
        .data_retention_in_hours(48)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("describe-me")
        .send()
        .await
        .expect("describe_stream should succeed");

    let info = resp.stream_info().expect("StreamInfo should be present");
    assert_eq!(info.stream_name().unwrap(), "describe-me");
    assert_eq!(info.device_name().unwrap(), "camera-1");
    assert_eq!(info.media_type().unwrap(), "video/h264");
    assert_eq!(info.data_retention_in_hours().unwrap(), 48);
}

#[tokio::test]
async fn test_describe_nonexistent_stream_fails() {
    let client = make_kinesis_video_client().await;

    let result = client
        .describe_stream()
        .stream_name("no-such-stream")
        .send()
        .await;

    assert!(result.is_err(), "describe nonexistent stream should fail");
}

#[tokio::test]
async fn test_delete_stream() {
    let client = make_kinesis_video_client().await;

    let create_resp = client
        .create_stream()
        .stream_name("delete-me")
        .send()
        .await
        .unwrap();

    let arn = create_resp.stream_arn().unwrap();

    client
        .delete_stream()
        .stream_arn(arn)
        .send()
        .await
        .expect("delete_stream should succeed");

    let result = client
        .describe_stream()
        .stream_name("delete-me")
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_stream_fails() {
    let client = make_kinesis_video_client().await;

    let result = client
        .delete_stream()
        .stream_arn("arn:aws:kinesisvideo:us-east-1:123456789012:stream/no-such/1234567890")
        .send()
        .await;

    assert!(result.is_err(), "delete nonexistent stream should fail");
}

#[tokio::test]
async fn test_list_streams() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("stream-a")
        .send()
        .await
        .unwrap();

    client
        .create_stream()
        .stream_name("stream-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_streams()
        .send()
        .await
        .expect("list_streams should succeed");

    let streams = resp.stream_info_list();
    assert_eq!(streams.len(), 2);
}

#[tokio::test]
async fn test_create_duplicate_stream_fails() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("dup-stream")
        .send()
        .await
        .unwrap();

    let result = client
        .create_stream()
        .stream_name("dup-stream")
        .send()
        .await;

    assert!(result.is_err(), "creating duplicate stream should fail");
}

#[tokio::test]
async fn test_list_streams_empty() {
    let client = make_kinesis_video_client().await;

    let resp = client
        .list_streams()
        .send()
        .await
        .expect("list_streams on empty state should succeed");

    let streams = resp.stream_info_list();
    assert_eq!(streams.len(), 0);
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Kinesis Video Streams
// ============================================================================

#[tokio::test]
async fn test_describe_stream_by_arn() {
    let client = make_kinesis_video_client().await;

    let create_resp = client
        .create_stream()
        .stream_name("arn-lookup-stream")
        .send()
        .await
        .unwrap();

    let arn = create_resp.stream_arn().unwrap().to_string();

    let resp = client
        .describe_stream()
        .stream_arn(&arn)
        .send()
        .await
        .expect("describe_stream by ARN should succeed");

    let info = resp.stream_info().expect("StreamInfo should be present");
    assert_eq!(info.stream_name().unwrap(), "arn-lookup-stream");
    assert_eq!(info.stream_arn().unwrap(), arn);
}

#[tokio::test]
async fn test_describe_stream_no_args_fails() {
    let client = make_kinesis_video_client().await;

    // Neither StreamName nor StreamARN — should return an error
    let result = client.describe_stream().send().await;

    assert!(result.is_err(), "describe_stream with no args should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidArgumentException") || err_str.contains("InvalidArgument"),
        "Expected InvalidArgumentException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_stream_lifecycle() {
    let client = make_kinesis_video_client().await;

    // Create
    let create_resp = client
        .create_stream()
        .stream_name("lifecycle-stream")
        .data_retention_in_hours(12)
        .send()
        .await
        .expect("create_stream should succeed");

    let arn = create_resp.stream_arn().unwrap().to_string();

    // Describe
    let desc_resp = client
        .describe_stream()
        .stream_name("lifecycle-stream")
        .send()
        .await
        .expect("describe_stream should succeed");
    let info = desc_resp.stream_info().unwrap();
    assert_eq!(info.stream_name().unwrap(), "lifecycle-stream");
    assert_eq!(info.data_retention_in_hours().unwrap(), 12);

    // Delete
    client
        .delete_stream()
        .stream_arn(&arn)
        .send()
        .await
        .expect("delete_stream should succeed");

    // Verify gone
    let after_delete = client
        .describe_stream()
        .stream_name("lifecycle-stream")
        .send()
        .await;
    assert!(after_delete.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_describe_stream_status_active() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("status-check-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("status-check-stream")
        .send()
        .await
        .unwrap();

    let info = resp.stream_info().unwrap();
    // Status should be ACTIVE immediately after creation
    assert_eq!(info.status().unwrap().as_str(), "ACTIVE");
}

#[tokio::test]
async fn test_describe_stream_version_present() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("version-check-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("version-check-stream")
        .send()
        .await
        .unwrap();

    let info = resp.stream_info().unwrap();
    let version = info.version().unwrap_or_default();
    assert!(!version.is_empty(), "Version should be non-empty");
}

#[tokio::test]
async fn test_describe_stream_creation_time_present() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("creation-time-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("creation-time-stream")
        .send()
        .await
        .unwrap();

    let info = resp.stream_info().unwrap();
    assert!(
        info.creation_time().is_some(),
        "CreationTime should be present"
    );
}

#[tokio::test]
async fn test_describe_stream_default_kms_key() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("default-kms-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("default-kms-stream")
        .send()
        .await
        .unwrap();

    let info = resp.stream_info().unwrap();
    assert_eq!(
        info.kms_key_id().unwrap_or_default(),
        "aws/kinesisvideo",
        "Default KMS key should be aws/kinesisvideo"
    );
}

#[tokio::test]
async fn test_create_stream_with_kms_key() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("custom-kms-stream")
        .kms_key_id("arn:aws:kms:us-east-1:123456789012:key/my-key-id")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("custom-kms-stream")
        .send()
        .await
        .unwrap();

    let info = resp.stream_info().unwrap();
    assert_eq!(
        info.kms_key_id().unwrap_or_default(),
        "arn:aws:kms:us-east-1:123456789012:key/my-key-id"
    );
}

#[tokio::test]
async fn test_list_streams_after_delete() {
    let client = make_kinesis_video_client().await;

    let resp_a = client
        .create_stream()
        .stream_name("keep-stream")
        .send()
        .await
        .unwrap();

    let resp_b = client
        .create_stream()
        .stream_name("remove-stream")
        .send()
        .await
        .unwrap();

    let arn_b = resp_b.stream_arn().unwrap().to_string();
    let _ = resp_a; // keep_stream stays

    client
        .delete_stream()
        .stream_arn(&arn_b)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_streams()
        .send()
        .await
        .expect("list_streams should succeed after partial delete");

    let streams = list_resp.stream_info_list();
    assert_eq!(
        streams.len(),
        1,
        "Only one stream should remain after deleting one"
    );
    assert_eq!(streams[0].stream_name().unwrap(), "keep-stream");
}

#[tokio::test]
async fn test_describe_stream_arn_format() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("arn-format-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("arn-format-stream")
        .send()
        .await
        .unwrap();

    let info = resp.stream_info().unwrap();
    let arn = info.stream_arn().unwrap_or_default();
    assert!(
        arn.contains("kinesisvideo"),
        "ARN should contain 'kinesisvideo', got: {arn}"
    );
    assert!(
        arn.contains("arn-format-stream"),
        "ARN should contain stream name, got: {arn}"
    );
}

#[tokio::test]
async fn test_create_stream_default_data_retention() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("default-retention-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stream()
        .stream_name("default-retention-stream")
        .send()
        .await
        .unwrap();

    let info = resp.stream_info().unwrap();
    // DataRetentionInHours defaults to 0 when not specified
    assert_eq!(
        info.data_retention_in_hours().unwrap_or(-1),
        0,
        "Default DataRetentionInHours should be 0"
    );
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Kinesis Video Streams (2026-03-28)
// ============================================================================

#[tokio::test]
async fn test_delete_stream_with_current_version() {
    let client = make_kinesis_video_client().await;

    let create_resp = client
        .create_stream()
        .stream_name("versioned-delete-stream")
        .send()
        .await
        .unwrap();

    let arn = create_resp.stream_arn().unwrap().to_string();

    // Get the current version from DescribeStream
    let desc_resp = client
        .describe_stream()
        .stream_name("versioned-delete-stream")
        .send()
        .await
        .unwrap();
    let version = desc_resp
        .stream_info()
        .unwrap()
        .version()
        .unwrap()
        .to_string();

    // Delete with the correct current version — should succeed
    client
        .delete_stream()
        .stream_arn(&arn)
        .current_version(&version)
        .send()
        .await
        .expect("delete_stream with correct CurrentVersion should succeed");

    // Verify the stream is gone
    let after = client
        .describe_stream()
        .stream_name("versioned-delete-stream")
        .send()
        .await;
    assert!(
        after.is_err(),
        "stream should be gone after versioned delete"
    );
}

#[tokio::test]
async fn test_delete_stream_version_mismatch() {
    let client = make_kinesis_video_client().await;

    let create_resp = client
        .create_stream()
        .stream_name("mismatch-version-stream")
        .send()
        .await
        .unwrap();

    let arn = create_resp.stream_arn().unwrap().to_string();

    // Delete with a wrong version — should return VersionMismatchException
    let err = client
        .delete_stream()
        .stream_arn(&arn)
        .current_version("wrong-version-value")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("VersionMismatchException"),
        "Expected VersionMismatchException for wrong CurrentVersion, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_stream_resource_not_found_error_type() {
    let client = make_kinesis_video_client().await;

    let err = client
        .describe_stream()
        .stream_name("nonexistent-stream-xyz")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException for describe on nonexistent stream, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_stream_resource_not_found_error_type() {
    let client = make_kinesis_video_client().await;

    let err = client
        .delete_stream()
        .stream_arn("arn:aws:kinesisvideo:us-east-1:123456789012:stream/nonexistent/9999999999")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException for delete on nonexistent stream, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_stream_duplicate_error_type() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("dup-error-type-stream")
        .send()
        .await
        .unwrap();

    let err = client
        .create_stream()
        .stream_name("dup-error-type-stream")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceInUseException"),
        "Expected ResourceInUseException for duplicate stream name, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_streams_items_have_arn_and_name() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("list-fields-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_streams()
        .send()
        .await
        .expect("list_streams should succeed");

    let streams = resp.stream_info_list();
    assert!(!streams.is_empty(), "should have at least one stream");

    let stream = &streams[0];
    let name = stream.stream_name().unwrap_or_default();
    let arn = stream.stream_arn().unwrap_or_default();
    assert!(!name.is_empty(), "StreamName in list should be non-empty");
    assert!(!arn.is_empty(), "StreamARN in list should be non-empty");
    assert!(
        arn.contains(name),
        "StreamARN should contain stream name, arn={arn}, name={name}"
    );
}

// ============================================================================
// UpdateStream
// ============================================================================

#[tokio::test]
async fn test_update_stream() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("update-stream-test")
        .device_name("old-camera")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_stream()
        .stream_name("update-stream-test")
        .send()
        .await
        .unwrap();
    let version = desc.stream_info().unwrap().version().unwrap().to_string();

    client
        .update_stream()
        .stream_name("update-stream-test")
        .current_version(&version)
        .device_name("new-camera")
        .media_type("video/h265")
        .send()
        .await
        .expect("update_stream should succeed");

    let desc2 = client
        .describe_stream()
        .stream_name("update-stream-test")
        .send()
        .await
        .unwrap();
    let info = desc2.stream_info().unwrap();
    assert_eq!(info.device_name().unwrap(), "new-camera");
    assert_eq!(info.media_type().unwrap(), "video/h265");
}

#[tokio::test]
async fn test_update_stream_version_mismatch() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("update-ver-mismatch-stream")
        .send()
        .await
        .unwrap();

    let err = client
        .update_stream()
        .stream_name("update-ver-mismatch-stream")
        .current_version("wrong-version")
        .device_name("cam")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("VersionMismatchException"),
        "Expected VersionMismatchException, got: {err_str}"
    );
}

// ============================================================================
// UpdateDataRetention
// ============================================================================

#[tokio::test]
async fn test_update_data_retention_increase() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("retention-increase-stream")
        .data_retention_in_hours(10)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_stream()
        .stream_name("retention-increase-stream")
        .send()
        .await
        .unwrap();
    let version = desc.stream_info().unwrap().version().unwrap().to_string();

    client
        .update_data_retention()
        .stream_name("retention-increase-stream")
        .current_version(&version)
        .operation(aws_sdk_kinesisvideo::types::UpdateDataRetentionOperation::IncreaseDataRetention)
        .data_retention_change_in_hours(5)
        .send()
        .await
        .expect("update_data_retention INCREASE should succeed");

    let desc2 = client
        .describe_stream()
        .stream_name("retention-increase-stream")
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc2
            .stream_info()
            .unwrap()
            .data_retention_in_hours()
            .unwrap(),
        15
    );
}

#[tokio::test]
async fn test_update_data_retention_decrease() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("retention-decrease-stream")
        .data_retention_in_hours(20)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_stream()
        .stream_name("retention-decrease-stream")
        .send()
        .await
        .unwrap();
    let version = desc.stream_info().unwrap().version().unwrap().to_string();

    client
        .update_data_retention()
        .stream_name("retention-decrease-stream")
        .current_version(&version)
        .operation(aws_sdk_kinesisvideo::types::UpdateDataRetentionOperation::DecreaseDataRetention)
        .data_retention_change_in_hours(5)
        .send()
        .await
        .expect("update_data_retention DECREASE should succeed");

    let desc2 = client
        .describe_stream()
        .stream_name("retention-decrease-stream")
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc2
            .stream_info()
            .unwrap()
            .data_retention_in_hours()
            .unwrap(),
        15
    );
}

// ============================================================================
// GetDataEndpoint
// ============================================================================

#[tokio::test]
async fn test_get_data_endpoint() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("endpoint-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_data_endpoint()
        .stream_name("endpoint-stream")
        .api_name(aws_sdk_kinesisvideo::types::ApiName::GetMedia)
        .send()
        .await
        .expect("get_data_endpoint should succeed");

    let endpoint = resp.data_endpoint().unwrap_or_default();
    assert!(!endpoint.is_empty(), "DataEndpoint should not be empty");
    assert!(
        endpoint.contains("kinesisvideo"),
        "Endpoint should contain 'kinesisvideo', got: {endpoint}"
    );
}

#[tokio::test]
async fn test_get_data_endpoint_nonexistent_stream() {
    let client = make_kinesis_video_client().await;

    let err = client
        .get_data_endpoint()
        .stream_name("nonexistent-endpoint-stream")
        .api_name(aws_sdk_kinesisvideo::types::ApiName::GetMedia)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// Stream tags: TagStream / UntagStream / ListTagsForStream
// ============================================================================

#[tokio::test]
async fn test_tag_stream() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("tag-stream-test")
        .send()
        .await
        .unwrap();

    client
        .tag_stream()
        .stream_name("tag-stream-test")
        .tags("env", "prod")
        .tags("team", "video")
        .send()
        .await
        .expect("tag_stream should succeed");

    let list_resp = client
        .list_tags_for_stream()
        .stream_name("tag-stream-test")
        .send()
        .await
        .expect("list_tags_for_stream should succeed");

    let tags = list_resp.tags().cloned().unwrap_or_default();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("video"));
}

#[tokio::test]
async fn test_untag_stream() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("untag-stream-test")
        .send()
        .await
        .unwrap();

    client
        .tag_stream()
        .stream_name("untag-stream-test")
        .tags("key1", "val1")
        .tags("key2", "val2")
        .send()
        .await
        .unwrap();

    client
        .untag_stream()
        .stream_name("untag-stream-test")
        .tag_key_list("key1")
        .send()
        .await
        .expect("untag_stream should succeed");

    let list_resp = client
        .list_tags_for_stream()
        .stream_name("untag-stream-test")
        .send()
        .await
        .unwrap();
    let tags = list_resp.tags().cloned().unwrap_or_default();
    assert!(!tags.contains_key("key1"), "key1 should be removed");
    assert_eq!(tags.get("key2").map(|s| s.as_str()), Some("val2"));
}

#[tokio::test]
async fn test_list_tags_for_stream_empty() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("no-tags-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_stream()
        .stream_name("no-tags-stream")
        .send()
        .await
        .expect("list_tags_for_stream on stream with no tags should succeed");

    assert!(
        resp.tags().map(|t| t.is_empty()).unwrap_or(true),
        "Should have no tags"
    );
}

// ============================================================================
// Signaling channels: Create / Describe / Delete / List / Update
// ============================================================================

#[tokio::test]
async fn test_create_signaling_channel() {
    let client = make_kinesis_video_client().await;

    let resp = client
        .create_signaling_channel()
        .channel_name("test-channel")
        .send()
        .await
        .expect("create_signaling_channel should succeed");

    let arn = resp.channel_arn().unwrap();
    assert!(
        arn.contains("test-channel"),
        "ARN should contain channel name"
    );
}

#[tokio::test]
async fn test_create_duplicate_signaling_channel_fails() {
    let client = make_kinesis_video_client().await;

    client
        .create_signaling_channel()
        .channel_name("dup-channel")
        .send()
        .await
        .unwrap();

    let err = client
        .create_signaling_channel()
        .channel_name("dup-channel")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceInUseException"),
        "Expected ResourceInUseException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_signaling_channel() {
    let client = make_kinesis_video_client().await;

    client
        .create_signaling_channel()
        .channel_name("describe-channel")
        .channel_type(aws_sdk_kinesisvideo::types::ChannelType::SingleMaster)
        .single_master_configuration(
            aws_sdk_kinesisvideo::types::SingleMasterConfiguration::builder()
                .message_ttl_seconds(60)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_signaling_channel()
        .channel_name("describe-channel")
        .send()
        .await
        .expect("describe_signaling_channel should succeed");

    let info = resp.channel_info().expect("ChannelInfo should be present");
    assert_eq!(info.channel_name().unwrap(), "describe-channel");
    assert_eq!(info.channel_type().unwrap().as_str(), "SINGLE_MASTER");
    assert_eq!(
        info.single_master_configuration()
            .and_then(|s| s.message_ttl_seconds())
            .unwrap(),
        60
    );
}

#[tokio::test]
async fn test_describe_nonexistent_signaling_channel() {
    let client = make_kinesis_video_client().await;

    let err = client
        .describe_signaling_channel()
        .channel_name("no-such-channel")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_signaling_channel() {
    let client = make_kinesis_video_client().await;

    let create_resp = client
        .create_signaling_channel()
        .channel_name("delete-channel-test")
        .send()
        .await
        .unwrap();
    let arn = create_resp.channel_arn().unwrap().to_string();

    client
        .delete_signaling_channel()
        .channel_arn(&arn)
        .send()
        .await
        .expect("delete_signaling_channel should succeed");

    let after = client
        .describe_signaling_channel()
        .channel_name("delete-channel-test")
        .send()
        .await;
    assert!(after.is_err(), "channel should be gone after deletion");
}

#[tokio::test]
async fn test_list_signaling_channels() {
    let client = make_kinesis_video_client().await;

    client
        .create_signaling_channel()
        .channel_name("list-channel-a")
        .send()
        .await
        .unwrap();

    client
        .create_signaling_channel()
        .channel_name("list-channel-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_signaling_channels()
        .send()
        .await
        .expect("list_signaling_channels should succeed");

    let channels = resp.channel_info_list();
    assert_eq!(channels.len(), 2, "Should have 2 channels");
}

#[tokio::test]
async fn test_update_signaling_channel() {
    let client = make_kinesis_video_client().await;

    client
        .create_signaling_channel()
        .channel_name("update-channel-test")
        .single_master_configuration(
            aws_sdk_kinesisvideo::types::SingleMasterConfiguration::builder()
                .message_ttl_seconds(30)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_signaling_channel()
        .channel_name("update-channel-test")
        .send()
        .await
        .unwrap();
    let version = desc.channel_info().unwrap().version().unwrap().to_string();
    let arn = desc
        .channel_info()
        .unwrap()
        .channel_arn()
        .unwrap()
        .to_string();

    client
        .update_signaling_channel()
        .channel_arn(&arn)
        .current_version(&version)
        .single_master_configuration(
            aws_sdk_kinesisvideo::types::SingleMasterConfiguration::builder()
                .message_ttl_seconds(120)
                .build(),
        )
        .send()
        .await
        .expect("update_signaling_channel should succeed");

    let desc2 = client
        .describe_signaling_channel()
        .channel_name("update-channel-test")
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc2
            .channel_info()
            .unwrap()
            .single_master_configuration()
            .and_then(|s| s.message_ttl_seconds())
            .unwrap(),
        120
    );
}

#[tokio::test]
async fn test_signaling_channel_lifecycle() {
    let client = make_kinesis_video_client().await;

    // Create
    let create_resp = client
        .create_signaling_channel()
        .channel_name("lifecycle-channel")
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp.channel_arn().unwrap().to_string();

    // Describe by ARN
    let desc_resp = client
        .describe_signaling_channel()
        .channel_arn(&arn)
        .send()
        .await
        .expect("describe by ARN should succeed");
    assert_eq!(
        desc_resp.channel_info().unwrap().channel_name().unwrap(),
        "lifecycle-channel"
    );

    // Delete
    client
        .delete_signaling_channel()
        .channel_arn(&arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let after = client
        .describe_signaling_channel()
        .channel_name("lifecycle-channel")
        .send()
        .await;
    assert!(after.is_err(), "channel should be gone");
}

// ============================================================================
// GetSignalingChannelEndpoint
// ============================================================================

#[tokio::test]
async fn test_get_signaling_channel_endpoint() {
    let client = make_kinesis_video_client().await;

    let create_resp = client
        .create_signaling_channel()
        .channel_name("endpoint-channel")
        .send()
        .await
        .unwrap();
    let arn = create_resp.channel_arn().unwrap().to_string();

    let resp = client
        .get_signaling_channel_endpoint()
        .channel_arn(&arn)
        .single_master_channel_endpoint_configuration(
            aws_sdk_kinesisvideo::types::SingleMasterChannelEndpointConfiguration::builder()
                .protocols(aws_sdk_kinesisvideo::types::ChannelProtocol::Wss)
                .role(aws_sdk_kinesisvideo::types::ChannelRole::Master)
                .build(),
        )
        .send()
        .await
        .expect("get_signaling_channel_endpoint should succeed");

    let endpoints = resp.resource_endpoint_list();
    assert!(!endpoints.is_empty(), "Should have at least one endpoint");
    let ep = &endpoints[0];
    assert!(
        ep.resource_endpoint().is_some(),
        "resource_endpoint should be present"
    );
}

// ============================================================================
// Resource tags (channels): TagResource / UntagResource / ListTagsForResource
// ============================================================================

#[tokio::test]
async fn test_tag_resource_channel() {
    let client = make_kinesis_video_client().await;

    let create_resp = client
        .create_signaling_channel()
        .channel_name("tag-resource-channel")
        .send()
        .await
        .unwrap();
    let arn = create_resp.channel_arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_kinesisvideo::types::Tag::builder()
                .key("project")
                .value("test")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags().cloned().unwrap_or_default();
    assert_eq!(tags.get("project").map(|s| s.as_str()), Some("test"));
}

#[tokio::test]
async fn test_untag_resource_channel() {
    let client = make_kinesis_video_client().await;

    let create_resp = client
        .create_signaling_channel()
        .channel_name("untag-resource-channel")
        .send()
        .await
        .unwrap();
    let arn = create_resp.channel_arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_kinesisvideo::types::Tag::builder()
                .key("keep")
                .value("yes")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_kinesisvideo::types::Tag::builder()
                .key("remove")
                .value("no")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_key_list("remove")
        .send()
        .await
        .expect("untag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    let tags = list_resp.tags().cloned().unwrap_or_default();
    assert!(!tags.contains_key("remove"), "remove tag should be gone");
    assert_eq!(tags.get("keep").map(|s| s.as_str()), Some("yes"));
}

#[tokio::test]
async fn test_list_tags_for_resource_nonexistent() {
    let client = make_kinesis_video_client().await;

    let err = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:kinesisvideo:us-east-1:123456789012:channel/no-such/999")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// UpdateImageGenerationConfiguration / DescribeImageGenerationConfiguration
// ============================================================================

#[tokio::test]
async fn test_update_and_describe_image_generation_configuration() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("image-gen-stream")
        .send()
        .await
        .unwrap();

    client
        .update_image_generation_configuration()
        .stream_name("image-gen-stream")
        .image_generation_configuration(
            aws_sdk_kinesisvideo::types::ImageGenerationConfiguration::builder()
                .status(aws_sdk_kinesisvideo::types::ConfigurationStatus::Enabled)
                .image_selector_type(
                    aws_sdk_kinesisvideo::types::ImageSelectorType::ServerTimestamp,
                )
                .destination_config(
                    aws_sdk_kinesisvideo::types::ImageGenerationDestinationConfig::builder()
                        .uri("s3://my-bucket/prefix")
                        .destination_region("us-east-1")
                        .build()
                        .unwrap(),
                )
                .sampling_interval(3000)
                .format(aws_sdk_kinesisvideo::types::Format::Jpeg)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_image_generation_configuration should succeed");

    let resp = client
        .describe_image_generation_configuration()
        .stream_name("image-gen-stream")
        .send()
        .await
        .expect("describe_image_generation_configuration should succeed");

    let config = resp
        .image_generation_configuration()
        .expect("ImageGenerationConfiguration should be present");
    assert_eq!(config.status().as_str(), "ENABLED");
    assert_eq!(config.sampling_interval(), 3000);
}

#[tokio::test]
async fn test_describe_image_generation_configuration_not_set() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("no-image-gen-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_image_generation_configuration()
        .stream_name("no-image-gen-stream")
        .send()
        .await
        .expect("describe_image_generation_configuration should succeed even if not set");

    assert!(
        resp.image_generation_configuration().is_none(),
        "ImageGenerationConfiguration should be None when not configured"
    );
}

// ============================================================================
// UpdateNotificationConfiguration / DescribeNotificationConfiguration
// ============================================================================

#[tokio::test]
async fn test_update_and_describe_notification_configuration() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("notification-stream")
        .send()
        .await
        .unwrap();

    client
        .update_notification_configuration()
        .stream_name("notification-stream")
        .notification_configuration(
            aws_sdk_kinesisvideo::types::NotificationConfiguration::builder()
                .status(aws_sdk_kinesisvideo::types::ConfigurationStatus::Enabled)
                .destination_config(
                    aws_sdk_kinesisvideo::types::NotificationDestinationConfig::builder()
                        .uri("arn:aws:sns:us-east-1:123456789012:my-topic")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_notification_configuration should succeed");

    let resp = client
        .describe_notification_configuration()
        .stream_name("notification-stream")
        .send()
        .await
        .expect("describe_notification_configuration should succeed");

    let config = resp
        .notification_configuration()
        .expect("NotificationConfiguration should be present");
    assert_eq!(config.status().as_str(), "ENABLED");
    assert_eq!(
        config
            .destination_config()
            .map(|dc| dc.uri())
            .unwrap_or_default(),
        "arn:aws:sns:us-east-1:123456789012:my-topic"
    );
}

// ============================================================================
// DescribeStreamStorageConfiguration / UpdateStreamStorageConfiguration
// ============================================================================

#[tokio::test]
async fn test_update_and_describe_stream_storage_configuration() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("storage-config-stream")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_stream()
        .stream_name("storage-config-stream")
        .send()
        .await
        .unwrap();
    let version = desc.stream_info().unwrap().version().unwrap().to_string();

    client
        .update_stream_storage_configuration()
        .stream_name("storage-config-stream")
        .current_version(&version)
        .stream_storage_configuration(
            aws_sdk_kinesisvideo::types::StreamStorageConfiguration::builder()
                .default_storage_tier(aws_sdk_kinesisvideo::types::DefaultStorageTier::Hot)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_stream_storage_configuration should succeed");

    let resp = client
        .describe_stream_storage_configuration()
        .stream_name("storage-config-stream")
        .send()
        .await
        .expect("describe_stream_storage_configuration should succeed");

    assert!(
        resp.stream_name().is_some(),
        "StreamName should be present in response"
    );
}

// ============================================================================
// DescribeMappedResourceConfiguration
// ============================================================================

#[tokio::test]
async fn test_describe_mapped_resource_configuration() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("mapped-resource-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_mapped_resource_configuration()
        .stream_name("mapped-resource-stream")
        .send()
        .await
        .expect("describe_mapped_resource_configuration should succeed");

    // In the mock, the list is always empty
    assert!(
        resp.mapped_resource_configuration_list().is_empty(),
        "MappedResourceConfigurationList should be empty in mock"
    );
}

// ============================================================================
// StartEdgeConfigurationUpdate / DescribeEdgeConfiguration /
// DeleteEdgeConfiguration / ListEdgeAgentConfigurations
// ============================================================================

#[tokio::test]
async fn test_start_edge_configuration_update() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("edge-stream")
        .send()
        .await
        .unwrap();

    let resp = client
        .start_edge_configuration_update()
        .stream_name("edge-stream")
        .edge_config(
            aws_sdk_kinesisvideo::types::EdgeConfig::builder()
                .hub_device_arn(
                    "arn:aws:kinesisvideo:us-east-1:123456789012:device/hub/12345",
                )
                .recorder_config(
                    aws_sdk_kinesisvideo::types::RecorderConfig::builder()
                        .media_source_config(
                            aws_sdk_kinesisvideo::types::MediaSourceConfig::builder()
                                .media_uri_secret_arn(
                                    "arn:aws:secretsmanager:us-east-1:123456789012:secret/rtsp-creds",
                                )
                                .media_uri_type(aws_sdk_kinesisvideo::types::MediaUriType::RtspUri)
                                .build()
                                .unwrap(),
                        )
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("start_edge_configuration_update should succeed");

    assert_eq!(
        resp.sync_status().map(|s| s.as_str()),
        Some("SYNCING"),
        "SyncStatus should be SYNCING"
    );
}

#[tokio::test]
async fn test_describe_edge_configuration() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("describe-edge-stream")
        .send()
        .await
        .unwrap();

    client
        .start_edge_configuration_update()
        .stream_name("describe-edge-stream")
        .edge_config(
            aws_sdk_kinesisvideo::types::EdgeConfig::builder()
                .hub_device_arn("arn:aws:kinesisvideo:us-east-1:123456789012:device/hub/99999")
                .recorder_config(
                    aws_sdk_kinesisvideo::types::RecorderConfig::builder()
                        .media_source_config(
                            aws_sdk_kinesisvideo::types::MediaSourceConfig::builder()
                                .media_uri_secret_arn(
                                    "arn:aws:secretsmanager:us-east-1:123456789012:secret/creds",
                                )
                                .media_uri_type(aws_sdk_kinesisvideo::types::MediaUriType::RtspUri)
                                .build()
                                .unwrap(),
                        )
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_edge_configuration()
        .stream_name("describe-edge-stream")
        .send()
        .await
        .expect("describe_edge_configuration should succeed");

    assert_eq!(resp.stream_name().unwrap(), "describe-edge-stream");
    assert_eq!(resp.sync_status().map(|s| s.as_str()), Some("SYNCING"));
}

#[tokio::test]
async fn test_describe_edge_configuration_not_found() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("no-edge-config-stream")
        .send()
        .await
        .unwrap();

    let err = client
        .describe_edge_configuration()
        .stream_name("no-edge-config-stream")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException when edge config not set, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_edge_configuration() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("delete-edge-stream")
        .send()
        .await
        .unwrap();

    client
        .start_edge_configuration_update()
        .stream_name("delete-edge-stream")
        .edge_config(
            aws_sdk_kinesisvideo::types::EdgeConfig::builder()
                .hub_device_arn("arn:aws:kinesisvideo:us-east-1:123456789012:device/hub/12345")
                .recorder_config(
                    aws_sdk_kinesisvideo::types::RecorderConfig::builder()
                        .media_source_config(
                            aws_sdk_kinesisvideo::types::MediaSourceConfig::builder()
                                .media_uri_secret_arn(
                                    "arn:aws:secretsmanager:us-east-1:123456789012:secret/creds",
                                )
                                .media_uri_type(aws_sdk_kinesisvideo::types::MediaUriType::RtspUri)
                                .build()
                                .unwrap(),
                        )
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_edge_configuration()
        .stream_name("delete-edge-stream")
        .send()
        .await
        .expect("delete_edge_configuration should succeed");

    let after = client
        .describe_edge_configuration()
        .stream_name("delete-edge-stream")
        .send()
        .await;
    assert!(after.is_err(), "edge config should be gone after deletion");
}

#[tokio::test]
async fn test_list_edge_agent_configurations() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("list-edge-stream-1")
        .send()
        .await
        .unwrap();
    client
        .create_stream()
        .stream_name("list-edge-stream-2")
        .send()
        .await
        .unwrap();

    let hub_arn = "arn:aws:kinesisvideo:us-east-1:123456789012:device/hub/12345";

    for name in ["list-edge-stream-1", "list-edge-stream-2"] {
        client
            .start_edge_configuration_update()
            .stream_name(name)
            .edge_config(
                aws_sdk_kinesisvideo::types::EdgeConfig::builder()
                    .hub_device_arn(hub_arn)
                    .recorder_config(
                        aws_sdk_kinesisvideo::types::RecorderConfig::builder()
                            .media_source_config(
                                aws_sdk_kinesisvideo::types::MediaSourceConfig::builder()
                                    .media_uri_secret_arn(
                                        "arn:aws:secretsmanager:us-east-1:123456789012:secret/creds",
                                    )
                                    .media_uri_type(
                                        aws_sdk_kinesisvideo::types::MediaUriType::RtspUri,
                                    )
                                    .build()
                                    .unwrap(),
                            )
                            .build(),
                    )
                    .build()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_edge_agent_configurations()
        .hub_device_arn(hub_arn)
        .send()
        .await
        .expect("list_edge_agent_configurations should succeed");

    let configs = resp.edge_configs();
    assert_eq!(configs.len(), 2, "Should have 2 edge configs");
}

// ============================================================================
// DescribeMediaStorageConfiguration / UpdateMediaStorageConfiguration
// ============================================================================

#[tokio::test]
async fn test_update_and_describe_media_storage_configuration() {
    let client = make_kinesis_video_client().await;

    client
        .create_stream()
        .stream_name("media-storage-stream")
        .send()
        .await
        .unwrap();
    let desc_stream = client
        .describe_stream()
        .stream_name("media-storage-stream")
        .send()
        .await
        .unwrap();
    let stream_arn = desc_stream
        .stream_info()
        .unwrap()
        .stream_arn()
        .unwrap()
        .to_string();

    let create_channel_resp = client
        .create_signaling_channel()
        .channel_name("media-storage-channel")
        .send()
        .await
        .unwrap();
    let channel_arn = create_channel_resp.channel_arn().unwrap().to_string();

    client
        .update_media_storage_configuration()
        .channel_arn(&channel_arn)
        .media_storage_configuration(
            aws_sdk_kinesisvideo::types::MediaStorageConfiguration::builder()
                .status(aws_sdk_kinesisvideo::types::MediaStorageConfigurationStatus::Enabled)
                .stream_arn(&stream_arn)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_media_storage_configuration should succeed");

    let resp = client
        .describe_media_storage_configuration()
        .channel_arn(&channel_arn)
        .send()
        .await
        .expect("describe_media_storage_configuration should succeed");

    let config = resp
        .media_storage_configuration()
        .expect("MediaStorageConfiguration should be present");
    assert_eq!(config.status().as_str(), "ENABLED");
    assert_eq!(config.stream_arn().unwrap(), stream_arn);
}

#[tokio::test]
async fn test_describe_media_storage_configuration_not_set() {
    let client = make_kinesis_video_client().await;

    let create_resp = client
        .create_signaling_channel()
        .channel_name("no-media-storage-channel")
        .send()
        .await
        .unwrap();
    let channel_arn = create_resp.channel_arn().unwrap().to_string();

    let resp = client
        .describe_media_storage_configuration()
        .channel_arn(&channel_arn)
        .send()
        .await
        .expect("describe_media_storage_configuration should succeed even if not set");

    assert!(
        resp.media_storage_configuration().is_none(),
        "MediaStorageConfiguration should be None when not configured"
    );
}

// ============================================================================
// State view tests
// ============================================================================

#[tokio::test]
async fn test_state_change_listener_fires() {
    use winterbaume_core::StatefulService;
    use winterbaume_kinesisvideo::KinesisVideoStateView;

    let svc = winterbaume_kinesisvideo::KinesisVideoService::new();
    let events: std::sync::Arc<std::sync::Mutex<Vec<(String, String)>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let events2 = std::sync::Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore(
        "123456789012",
        "us-east-1",
        KinesisVideoStateView::default(),
    )
    .await
    .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}

#[tokio::test]
async fn test_state_snapshot_contains_created_resources() {
    use winterbaume_core::StatefulService;
    use winterbaume_kinesisvideo::views::StreamView;
    use winterbaume_kinesisvideo::{KinesisVideoService, KinesisVideoStateView};

    let svc = KinesisVideoService::new();

    // Pre-seed via restore
    let mut view = KinesisVideoStateView::default();
    view.streams.insert(
        "snapshot-stream".to_string(),
        StreamView {
            stream_name: "snapshot-stream".to_string(),
            stream_arn: "arn:aws:kinesisvideo:us-east-1:123456789012:stream/snapshot-stream/1"
                .to_string(),
            device_name: None,
            media_type: None,
            kms_key_id: "aws/kinesisvideo".to_string(),
            version: "v1".to_string(),
            status: "ACTIVE".to_string(),
            creation_time: "2024-01-01T00:00:00Z".to_string(),
            data_retention_in_hours: 0,
            tags: Default::default(),
            image_generation_config: None,
            notification_config: None,
            storage_config: None,
            edge_config: None,
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        snapshot.streams.contains_key("snapshot-stream"),
        "snapshot should contain stream"
    );
}

#[tokio::test]
async fn test_state_merge_additive() {
    use winterbaume_core::StatefulService;
    use winterbaume_kinesisvideo::KinesisVideoStateView;
    use winterbaume_kinesisvideo::views::StreamView;

    let svc = winterbaume_kinesisvideo::KinesisVideoService::new();

    // Restore initial state with one stream
    let mut initial = KinesisVideoStateView::default();
    initial.streams.insert(
        "existing-stream".to_string(),
        StreamView {
            stream_name: "existing-stream".to_string(),
            stream_arn: "arn:aws:kinesisvideo:us-east-1:123456789012:stream/existing-stream/1"
                .to_string(),
            device_name: None,
            media_type: None,
            kms_key_id: "aws/kinesisvideo".to_string(),
            version: "v1".to_string(),
            status: "ACTIVE".to_string(),
            creation_time: "2024-01-01T00:00:00Z".to_string(),
            data_retention_in_hours: 0,
            tags: Default::default(),
            image_generation_config: None,
            notification_config: None,
            storage_config: None,
            edge_config: None,
        },
    );
    svc.restore("123456789012", "us-east-1", initial)
        .await
        .unwrap();

    // Merge a second stream
    let mut addition = KinesisVideoStateView::default();
    addition.streams.insert(
        "new-stream".to_string(),
        StreamView {
            stream_name: "new-stream".to_string(),
            stream_arn: "arn:aws:kinesisvideo:us-east-1:123456789012:stream/new-stream/2"
                .to_string(),
            device_name: None,
            media_type: None,
            kms_key_id: "aws/kinesisvideo".to_string(),
            version: "v2".to_string(),
            status: "ACTIVE".to_string(),
            creation_time: "2024-01-02T00:00:00Z".to_string(),
            data_retention_in_hours: 0,
            tags: Default::default(),
            image_generation_config: None,
            notification_config: None,
            storage_config: None,
            edge_config: None,
        },
    );
    svc.merge("123456789012", "us-east-1", addition)
        .await
        .unwrap();

    // Both streams should be present
    let view = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        view.streams.contains_key("existing-stream"),
        "existing-stream should still be there"
    );
    assert!(
        view.streams.contains_key("new-stream"),
        "new-stream should be added"
    );
}
