use bytes::Bytes;
use serde_json::{Value, json};
use winterbaume_core::{MockRequest, MockService};
use winterbaume_ivs::IvsService;

fn post_request(path: &str, body: Value) -> MockRequest {
    let uri = format!("https://ivs.us-east-1.amazonaws.com{path}");
    MockRequest {
        method: "POST".to_string(),
        uri,
        headers: http::HeaderMap::new(),
        body: Bytes::from(body.to_string()),
    }
}

async fn create_channel(service: &IvsService, name: &str) -> Value {
    let req = post_request(
        "/CreateChannel",
        json!({ "name": name, "latencyMode": "LOW", "type": "STANDARD" }),
    );
    let resp = service.handle(req).await;
    assert_eq!(resp.status, 200, "CreateChannel should return 200");
    serde_json::from_slice(&resp.body).unwrap()
}

#[tokio::test]
async fn test_create_and_get_channel() {
    let service = IvsService::new();

    let create_resp = create_channel(&service, "test-channel").await;
    let channel = &create_resp["channel"];
    assert_eq!(channel["name"], "test-channel");
    assert_eq!(channel["latencyMode"], "LOW");
    assert_eq!(channel["type"], "STANDARD");
    assert_eq!(channel["authorized"], false);
    let arn = channel["arn"].as_str().unwrap();
    assert!(arn.starts_with("arn:aws:ivs:us-east-1:123456789012:channel/"));

    // GetChannel
    let get_req = post_request("/GetChannel", json!({ "arn": arn }));
    let get_resp = service.handle(get_req).await;
    assert_eq!(get_resp.status, 200);
    let get_body: Value = serde_json::from_slice(&get_resp.body).unwrap();
    assert_eq!(get_body["channel"]["name"], "test-channel");
    assert_eq!(get_body["channel"]["arn"], arn);
}

#[tokio::test]
async fn test_delete_channel() {
    let service = IvsService::new();

    let create_resp = create_channel(&service, "to-delete").await;
    let arn = create_resp["channel"]["arn"].as_str().unwrap().to_string();

    // Delete
    let del_req = post_request("/DeleteChannel", json!({ "arn": arn }));
    let del_resp = service.handle(del_req).await;
    assert_eq!(del_resp.status, 204);

    // Get after delete should fail
    let get_req = post_request("/GetChannel", json!({ "arn": arn }));
    let get_resp = service.handle(get_req).await;
    assert_eq!(get_resp.status, 404);
}

#[tokio::test]
async fn test_list_channels() {
    let service = IvsService::new();

    create_channel(&service, "channel-a").await;
    create_channel(&service, "channel-b").await;

    let list_req = post_request("/ListChannels", json!({}));
    let list_resp = service.handle(list_req).await;
    assert_eq!(list_resp.status, 200);
    let list_body: Value = serde_json::from_slice(&list_resp.body).unwrap();
    let channels = list_body["channels"].as_array().unwrap();
    assert_eq!(channels.len(), 2);
}

#[tokio::test]
async fn test_create_delete_then_list_empty() {
    let service = IvsService::new();

    let create_resp = create_channel(&service, "ephemeral-channel").await;
    let arn = create_resp["channel"]["arn"].as_str().unwrap().to_string();

    // List should show 1
    let list_req = post_request("/ListChannels", json!({}));
    let list_resp = service.handle(list_req).await;
    let list_body: Value = serde_json::from_slice(&list_resp.body).unwrap();
    assert_eq!(list_body["channels"].as_array().unwrap().len(), 1);

    // Delete it
    let del_req = post_request("/DeleteChannel", json!({ "arn": arn }));
    let del_resp = service.handle(del_req).await;
    assert_eq!(del_resp.status, 204);

    // List should now be empty
    let list_req = post_request("/ListChannels", json!({}));
    let list_resp = service.handle(list_req).await;
    let list_body: Value = serde_json::from_slice(&list_resp.body).unwrap();
    assert_eq!(list_body["channels"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_get_nonexistent_channel() {
    let service = IvsService::new();

    let get_req = post_request(
        "/GetChannel",
        json!({ "arn": "arn:aws:ivs:us-east-1:123456789012:channel/nonexistent" }),
    );
    let get_resp = service.handle(get_req).await;
    assert_eq!(get_resp.status, 404);
}

// ============================================================================
// Tests derived from AWS documentation: IVS (Interactive Video Service) — SDK client
// ============================================================================

use aws_sdk_ivs::config::BehaviorVersion;
use aws_sdk_ivs::types::{ChannelLatencyMode, ChannelType};
use winterbaume_core::MockAws;

async fn make_ivs_client() -> aws_sdk_ivs::Client {
    let mock = MockAws::builder().with_service(IvsService::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ivs::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_ivs::Client::new(&config)
}

#[tokio::test]
async fn sdk_test_create_channel_returns_expected_fields() {
    let client = make_ivs_client().await;
    let resp = client
        .create_channel()
        .name("sdk-channel")
        .latency_mode(ChannelLatencyMode::LowLatency)
        .r#type(ChannelType::StandardChannelType)
        .send()
        .await
        .expect("create_channel should succeed");
    let channel = resp.channel().expect("response must contain channel");
    let arn = channel.arn().expect("channel must have arn");
    assert!(arn.starts_with("arn:aws:ivs:us-east-1:123456789012:channel/"));
    assert_eq!(channel.name(), Some("sdk-channel"));
    assert!(!channel.authorized());
    assert_eq!(
        channel.latency_mode(),
        Some(&ChannelLatencyMode::LowLatency)
    );
}

#[tokio::test]
async fn sdk_test_get_channel_by_arn() {
    let client = make_ivs_client().await;
    let create_resp = client
        .create_channel()
        .name("get-me")
        .send()
        .await
        .expect("create_channel should succeed");
    let arn = create_resp
        .channel()
        .and_then(|c| c.arn())
        .expect("created channel must have arn")
        .to_string();

    let get_resp = client
        .get_channel()
        .arn(&arn)
        .send()
        .await
        .expect("get_channel should succeed");
    let channel = get_resp.channel().expect("get_channel must return channel");
    assert_eq!(channel.arn(), Some(arn.as_str()));
    assert_eq!(channel.name(), Some("get-me"));
}

#[tokio::test]
async fn sdk_test_get_nonexistent_channel_is_error() {
    let client = make_ivs_client().await;
    let result = client
        .get_channel()
        .arn("arn:aws:ivs:us-east-1:123456789012:channel/doesnotexist")
        .send()
        .await;
    assert!(
        result.is_err(),
        "get_channel on missing channel should fail"
    );
}

#[tokio::test]
async fn sdk_test_update_channel_name_and_latency() {
    let client = make_ivs_client().await;
    let create_resp = client
        .create_channel()
        .name("original-name")
        .latency_mode(ChannelLatencyMode::LowLatency)
        .send()
        .await
        .expect("create_channel should succeed");
    let arn = create_resp
        .channel()
        .and_then(|c| c.arn())
        .expect("created channel must have arn")
        .to_string();

    let update_resp = client
        .update_channel()
        .arn(&arn)
        .name("updated-name")
        .latency_mode(ChannelLatencyMode::NormalLatency)
        .send()
        .await
        .expect("update_channel should succeed");
    let channel = update_resp
        .channel()
        .expect("update_channel must return channel");
    assert_eq!(channel.name(), Some("updated-name"));
    assert_eq!(
        channel.latency_mode(),
        Some(&ChannelLatencyMode::NormalLatency)
    );
}

#[tokio::test]
async fn sdk_test_update_nonexistent_channel_is_error() {
    let client = make_ivs_client().await;
    let result = client
        .update_channel()
        .arn("arn:aws:ivs:us-east-1:123456789012:channel/ghost")
        .name("new-name")
        .send()
        .await;
    assert!(
        result.is_err(),
        "update_channel on missing channel should fail"
    );
}

#[tokio::test]
async fn sdk_test_list_channels_returns_all_created() {
    let client = make_ivs_client().await;
    for name in ["list-ch-1", "list-ch-2", "list-ch-3"] {
        client
            .create_channel()
            .name(name)
            .send()
            .await
            .expect("create_channel should succeed");
    }
    let list_resp = client
        .list_channels()
        .send()
        .await
        .expect("list_channels should succeed");
    assert_eq!(list_resp.channels().len(), 3);
}

#[tokio::test]
async fn sdk_test_batch_get_channel_returns_found_and_errors() {
    let client = make_ivs_client().await;
    let create_resp = client
        .create_channel()
        .name("batch-ch")
        .send()
        .await
        .expect("create_channel should succeed");
    let existing_arn = create_resp
        .channel()
        .and_then(|c| c.arn())
        .expect("must have arn")
        .to_string();

    let batch_resp = client
        .batch_get_channel()
        .arns(&existing_arn)
        .arns("arn:aws:ivs:us-east-1:123456789012:channel/missing")
        .send()
        .await
        .expect("batch_get_channel should succeed");
    assert_eq!(batch_resp.channels().len(), 1);
    assert_eq!(batch_resp.channels()[0].name(), Some("batch-ch"));
    assert_eq!(batch_resp.errors().len(), 1);
}

#[tokio::test]
async fn sdk_test_delete_channel_then_get_returns_error() {
    let client = make_ivs_client().await;
    let create_resp = client
        .create_channel()
        .name("delete-me")
        .send()
        .await
        .expect("create_channel should succeed");
    let arn = create_resp
        .channel()
        .and_then(|c| c.arn())
        .expect("must have arn")
        .to_string();

    client
        .delete_channel()
        .arn(&arn)
        .send()
        .await
        .expect("delete_channel should succeed");

    let get_result = client.get_channel().arn(&arn).send().await;
    assert!(get_result.is_err(), "get_channel after delete should fail");

    let list_resp = client
        .list_channels()
        .send()
        .await
        .expect("list_channels should succeed");
    assert_eq!(list_resp.channels().len(), 0);
}

// ============================================================================
// Additional tests derived from AWS documentation: IVS API Reference
// ============================================================================

#[tokio::test]
async fn sdk_test_create_channel_duplicate_name() {
    let client = make_ivs_client().await;
    client
        .create_channel()
        .name("dup-channel")
        .send()
        .await
        .expect("first create_channel should succeed");

    let err = client
        .create_channel()
        .name("dup-channel")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException for duplicate name, got: {err_str}"
    );
}

#[tokio::test]
async fn sdk_test_delete_nonexistent_channel() {
    let client = make_ivs_client().await;
    let err = client
        .delete_channel()
        .arn("arn:aws:ivs:us-east-1:123456789012:channel/doesnotexist")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException for missing channel, got: {err_str}"
    );
}

#[tokio::test]
async fn sdk_test_create_channel_with_authorized_field() {
    let client = make_ivs_client().await;
    // authorized is not directly settable via SDK builder for channel create,
    // but we verify the default is false and UpdateChannel can toggle it.
    let create_resp = client
        .create_channel()
        .name("auth-channel")
        .send()
        .await
        .expect("create_channel should succeed");
    let channel = create_resp.channel().expect("must have channel");
    assert!(!channel.authorized(), "Default authorized must be false");

    let arn = channel.arn().expect("must have arn").to_string();

    // Now update authorized to true via UpdateChannel
    let update_resp = client
        .update_channel()
        .arn(&arn)
        .authorized(true)
        .send()
        .await
        .expect("update_channel should succeed");
    let updated = update_resp.channel().expect("update must return channel");
    assert!(
        updated.authorized(),
        "authorized should be true after update"
    );
}

#[tokio::test]
async fn sdk_test_update_channel_type() {
    let client = make_ivs_client().await;
    let create_resp = client
        .create_channel()
        .name("type-channel")
        .r#type(ChannelType::StandardChannelType)
        .send()
        .await
        .expect("create_channel should succeed");
    let arn = create_resp
        .channel()
        .and_then(|c| c.arn())
        .expect("must have arn")
        .to_string();

    let update_resp = client
        .update_channel()
        .arn(&arn)
        .r#type(ChannelType::BasicChannelType)
        .send()
        .await
        .expect("update_channel should succeed");
    let channel = update_resp.channel().expect("update must return channel");
    assert_eq!(
        channel.r#type(),
        Some(&ChannelType::BasicChannelType),
        "Channel type should be updated to BASIC"
    );
}

#[tokio::test]
async fn sdk_test_update_channel_reflects_in_get() {
    let client = make_ivs_client().await;
    let create_resp = client
        .create_channel()
        .name("reflect-channel")
        .latency_mode(ChannelLatencyMode::LowLatency)
        .send()
        .await
        .expect("create_channel should succeed");
    let arn = create_resp
        .channel()
        .and_then(|c| c.arn())
        .expect("must have arn")
        .to_string();

    client
        .update_channel()
        .arn(&arn)
        .name("reflect-channel-updated")
        .latency_mode(ChannelLatencyMode::NormalLatency)
        .send()
        .await
        .expect("update_channel should succeed");

    let get_resp = client
        .get_channel()
        .arn(&arn)
        .send()
        .await
        .expect("get_channel should succeed");
    let channel = get_resp.channel().expect("get must return channel");
    assert_eq!(channel.name(), Some("reflect-channel-updated"));
    assert_eq!(
        channel.latency_mode(),
        Some(&ChannelLatencyMode::NormalLatency)
    );
}

#[tokio::test]
async fn sdk_test_batch_get_channel_all_found() {
    let client = make_ivs_client().await;
    let arn1 = client
        .create_channel()
        .name("batch-all-1")
        .send()
        .await
        .expect("create 1 should succeed")
        .channel()
        .and_then(|c| c.arn())
        .expect("must have arn")
        .to_string();
    let arn2 = client
        .create_channel()
        .name("batch-all-2")
        .send()
        .await
        .expect("create 2 should succeed")
        .channel()
        .and_then(|c| c.arn())
        .expect("must have arn")
        .to_string();

    let batch_resp = client
        .batch_get_channel()
        .arns(&arn1)
        .arns(&arn2)
        .send()
        .await
        .expect("batch_get_channel should succeed");
    assert_eq!(
        batch_resp.channels().len(),
        2,
        "Both channels should be returned"
    );
    assert_eq!(
        batch_resp.errors().len(),
        0,
        "No errors expected when all ARNs exist"
    );
}

#[tokio::test]
async fn sdk_test_batch_get_channel_all_missing() {
    let client = make_ivs_client().await;
    let batch_resp = client
        .batch_get_channel()
        .arns("arn:aws:ivs:us-east-1:123456789012:channel/missing1")
        .arns("arn:aws:ivs:us-east-1:123456789012:channel/missing2")
        .send()
        .await
        .expect("batch_get_channel should succeed (200) even when all ARNs missing");
    assert_eq!(
        batch_resp.channels().len(),
        0,
        "No channels expected when all ARNs missing"
    );
    assert_eq!(
        batch_resp.errors().len(),
        2,
        "Both ARNs should appear in errors"
    );
}

#[tokio::test]
async fn sdk_test_channel_arn_format() {
    let client = make_ivs_client().await;
    let resp = client
        .create_channel()
        .name("arn-check")
        .send()
        .await
        .expect("create_channel should succeed");
    let channel = resp.channel().expect("must have channel");
    let arn = channel.arn().expect("must have arn");
    assert!(
        arn.starts_with("arn:aws:ivs:us-east-1:123456789012:channel/"),
        "ARN must have correct format, got: {arn}"
    );
    let suffix = arn
        .strip_prefix("arn:aws:ivs:us-east-1:123456789012:channel/")
        .unwrap();
    assert!(!suffix.is_empty(), "Channel ID suffix must not be empty");
}

#[tokio::test]
async fn sdk_test_list_channels_after_partial_delete() {
    let client = make_ivs_client().await;
    let arn1 = client
        .create_channel()
        .name("partial-1")
        .send()
        .await
        .expect("create 1 should succeed")
        .channel()
        .and_then(|c| c.arn())
        .expect("must have arn")
        .to_string();
    client
        .create_channel()
        .name("partial-2")
        .send()
        .await
        .expect("create 2 should succeed");
    client
        .create_channel()
        .name("partial-3")
        .send()
        .await
        .expect("create 3 should succeed");

    // Delete only the first one
    client
        .delete_channel()
        .arn(&arn1)
        .send()
        .await
        .expect("delete_channel should succeed");

    let list_resp = client
        .list_channels()
        .send()
        .await
        .expect("list_channels should succeed");
    assert_eq!(
        list_resp.channels().len(),
        2,
        "Should have 2 remaining channels after deleting 1 of 3"
    );
}

// ============================================================================
// Stream key tests
// ============================================================================

#[tokio::test]
async fn sdk_test_create_stream_key() {
    let client = make_ivs_client().await;
    let ch = client
        .create_channel()
        .name("sk-channel")
        .send()
        .await
        .expect("create channel");
    let channel_arn = ch.channel().and_then(|c| c.arn()).expect("arn").to_string();

    let resp = client
        .create_stream_key()
        .channel_arn(&channel_arn)
        .send()
        .await
        .expect("create_stream_key should succeed");
    let sk = resp.stream_key().expect("stream key in response");
    assert!(
        sk.arn()
            .expect("sk arn")
            .starts_with("arn:aws:ivs:us-east-1:123456789012:stream-key/")
    );
    assert_eq!(sk.channel_arn(), Some(channel_arn.as_str()));
    assert!(!sk.value().expect("sk value").is_empty());
}

#[tokio::test]
async fn sdk_test_get_stream_key() {
    let client = make_ivs_client().await;
    let ch = client
        .create_channel()
        .name("sk-get-channel")
        .send()
        .await
        .expect("create ch");
    let channel_arn = ch.channel().and_then(|c| c.arn()).expect("arn").to_string();

    let create_resp = client
        .create_stream_key()
        .channel_arn(&channel_arn)
        .send()
        .await
        .expect("create sk");
    let sk_arn = create_resp
        .stream_key()
        .and_then(|sk| sk.arn())
        .expect("arn")
        .to_string();

    let get_resp = client
        .get_stream_key()
        .arn(&sk_arn)
        .send()
        .await
        .expect("get sk");
    assert_eq!(
        get_resp.stream_key().and_then(|sk| sk.arn()),
        Some(sk_arn.as_str())
    );
}

#[tokio::test]
async fn sdk_test_delete_stream_key() {
    let client = make_ivs_client().await;
    let ch = client
        .create_channel()
        .name("sk-del-channel")
        .send()
        .await
        .expect("create ch");
    let channel_arn = ch.channel().and_then(|c| c.arn()).expect("arn").to_string();

    let sk = client
        .create_stream_key()
        .channel_arn(&channel_arn)
        .send()
        .await
        .expect("create sk");
    let sk_arn = sk
        .stream_key()
        .and_then(|s| s.arn())
        .expect("arn")
        .to_string();

    client
        .delete_stream_key()
        .arn(&sk_arn)
        .send()
        .await
        .expect("delete sk");

    let get_err = client.get_stream_key().arn(&sk_arn).send().await;
    assert!(get_err.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn sdk_test_list_stream_keys() {
    let client = make_ivs_client().await;
    let ch = client
        .create_channel()
        .name("sk-list-channel")
        .send()
        .await
        .expect("create ch");
    let channel_arn = ch.channel().and_then(|c| c.arn()).expect("arn").to_string();

    client
        .create_stream_key()
        .channel_arn(&channel_arn)
        .send()
        .await
        .expect("sk1");
    client
        .create_stream_key()
        .channel_arn(&channel_arn)
        .send()
        .await
        .expect("sk2");

    let list = client
        .list_stream_keys()
        .channel_arn(&channel_arn)
        .send()
        .await
        .expect("list");
    assert_eq!(list.stream_keys().len(), 2);
}

#[tokio::test]
async fn sdk_test_batch_get_stream_key() {
    let client = make_ivs_client().await;
    let ch = client
        .create_channel()
        .name("sk-batch-channel")
        .send()
        .await
        .expect("create ch");
    let channel_arn = ch.channel().and_then(|c| c.arn()).expect("arn").to_string();

    let sk = client
        .create_stream_key()
        .channel_arn(&channel_arn)
        .send()
        .await
        .expect("create sk");
    let sk_arn = sk
        .stream_key()
        .and_then(|s| s.arn())
        .expect("arn")
        .to_string();

    let batch = client
        .batch_get_stream_key()
        .arns(&sk_arn)
        .arns("arn:aws:ivs:us-east-1:123456789012:stream-key/nonexistent")
        .send()
        .await
        .expect("batch get");
    assert_eq!(batch.stream_keys().len(), 1);
    assert_eq!(batch.errors().len(), 1);
}

#[tokio::test]
async fn sdk_test_stream_key_not_found_on_missing_channel() {
    let client = make_ivs_client().await;
    let err = client
        .create_stream_key()
        .channel_arn("arn:aws:ivs:us-east-1:123456789012:channel/nonexistent")
        .send()
        .await;
    assert!(err.is_err(), "should fail for missing channel");
}

// ============================================================================
// Recording configuration tests
// ============================================================================

#[tokio::test]
async fn sdk_test_create_recording_configuration() {
    let client = make_ivs_client().await;
    let resp = client
        .create_recording_configuration()
        .name("my-rc")
        .destination_configuration(
            aws_sdk_ivs::types::DestinationConfiguration::builder()
                .s3(aws_sdk_ivs::types::S3DestinationConfiguration::builder()
                    .bucket_name("my-bucket")
                    .build()
                    .expect("s3"))
                .build(),
        )
        .send()
        .await
        .expect("create rc");
    let rc = resp.recording_configuration().expect("rc");
    assert!(
        rc.arn()
            .starts_with("arn:aws:ivs:us-east-1:123456789012:recording-configuration/")
    );
    assert_eq!(rc.name(), Some("my-rc"));
    // state is returned as an enum value
    assert_eq!(rc.state().as_str(), "ACTIVE");
}

#[tokio::test]
async fn sdk_test_get_recording_configuration() {
    let client = make_ivs_client().await;
    let create_resp = client
        .create_recording_configuration()
        .destination_configuration(
            aws_sdk_ivs::types::DestinationConfiguration::builder()
                .s3(aws_sdk_ivs::types::S3DestinationConfiguration::builder()
                    .bucket_name("bucket")
                    .build()
                    .expect("s3"))
                .build(),
        )
        .send()
        .await
        .expect("create rc");
    let arn = create_resp
        .recording_configuration()
        .expect("rc")
        .arn()
        .to_string();

    let get_resp = client
        .get_recording_configuration()
        .arn(&arn)
        .send()
        .await
        .expect("get rc");
    assert_eq!(
        get_resp.recording_configuration().map(|rc| rc.arn()),
        Some(arn.as_str())
    );
}

#[tokio::test]
async fn sdk_test_delete_recording_configuration() {
    let client = make_ivs_client().await;
    let create_resp = client
        .create_recording_configuration()
        .destination_configuration(
            aws_sdk_ivs::types::DestinationConfiguration::builder()
                .s3(aws_sdk_ivs::types::S3DestinationConfiguration::builder()
                    .bucket_name("del-bucket")
                    .build()
                    .expect("s3"))
                .build(),
        )
        .send()
        .await
        .expect("create rc");
    let arn = create_resp
        .recording_configuration()
        .expect("rc")
        .arn()
        .to_string();

    client
        .delete_recording_configuration()
        .arn(&arn)
        .send()
        .await
        .expect("delete rc");

    let err = client.get_recording_configuration().arn(&arn).send().await;
    assert!(err.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn sdk_test_list_recording_configurations() {
    let client = make_ivs_client().await;
    let s3conf = aws_sdk_ivs::types::DestinationConfiguration::builder()
        .s3(aws_sdk_ivs::types::S3DestinationConfiguration::builder()
            .bucket_name("list-bucket")
            .build()
            .expect("s3"))
        .build();
    client
        .create_recording_configuration()
        .destination_configuration(s3conf.clone())
        .send()
        .await
        .expect("rc1");
    client
        .create_recording_configuration()
        .destination_configuration(s3conf)
        .send()
        .await
        .expect("rc2");

    let list = client
        .list_recording_configurations()
        .send()
        .await
        .expect("list");
    assert_eq!(list.recording_configurations().len(), 2);
}

// ============================================================================
// Playback key pair tests
// ============================================================================

const FAKE_PK: &str = "-----BEGIN PUBLIC KEY-----\ntest\n-----END PUBLIC KEY-----\n";

#[tokio::test]
async fn sdk_test_import_playback_key_pair() {
    let client = make_ivs_client().await;
    let resp = client
        .import_playback_key_pair()
        .name("my-keypair")
        .public_key_material(FAKE_PK)
        .send()
        .await
        .expect("import key pair");
    let kp = resp.key_pair().expect("key pair");
    assert!(
        kp.arn()
            .expect("arn")
            .starts_with("arn:aws:ivs:us-east-1:123456789012:playback-key/")
    );
    assert_eq!(kp.name(), Some("my-keypair"));
    assert!(!kp.fingerprint().expect("fingerprint").is_empty());
}

#[tokio::test]
async fn sdk_test_get_playback_key_pair() {
    let client = make_ivs_client().await;
    let create_resp = client
        .import_playback_key_pair()
        .public_key_material(FAKE_PK)
        .send()
        .await
        .expect("import key pair");
    let arn = create_resp
        .key_pair()
        .and_then(|kp| kp.arn())
        .expect("arn")
        .to_string();

    let get_resp = client
        .get_playback_key_pair()
        .arn(&arn)
        .send()
        .await
        .expect("get kp");
    assert_eq!(
        get_resp.key_pair().and_then(|kp| kp.arn()),
        Some(arn.as_str())
    );
}

#[tokio::test]
async fn sdk_test_delete_playback_key_pair() {
    let client = make_ivs_client().await;
    let create_resp = client
        .import_playback_key_pair()
        .public_key_material(FAKE_PK)
        .send()
        .await
        .expect("import kp");
    let arn = create_resp
        .key_pair()
        .and_then(|kp| kp.arn())
        .expect("arn")
        .to_string();

    client
        .delete_playback_key_pair()
        .arn(&arn)
        .send()
        .await
        .expect("delete kp");

    let err = client.get_playback_key_pair().arn(&arn).send().await;
    assert!(err.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn sdk_test_list_playback_key_pairs() {
    let client = make_ivs_client().await;
    client
        .import_playback_key_pair()
        .public_key_material(FAKE_PK)
        .send()
        .await
        .expect("kp1");
    client
        .import_playback_key_pair()
        .public_key_material(FAKE_PK)
        .send()
        .await
        .expect("kp2");

    let list = client.list_playback_key_pairs().send().await.expect("list");
    assert_eq!(list.key_pairs().len(), 2);
}

// ============================================================================
// Playback restriction policy tests
// ============================================================================

#[tokio::test]
async fn sdk_test_create_playback_restriction_policy() {
    let client = make_ivs_client().await;
    let resp = client
        .create_playback_restriction_policy()
        .name("my-policy")
        .allowed_countries("US")
        .allowed_origins("https://example.com")
        .enable_strict_origin_enforcement(true)
        .send()
        .await
        .expect("create policy");
    let policy = resp.playback_restriction_policy().expect("policy");
    assert!(
        policy
            .arn()
            .starts_with("arn:aws:ivs:us-east-1:123456789012:playback-restriction-policy/")
    );
    assert_eq!(policy.name(), Some("my-policy"));
    assert_eq!(policy.allowed_countries(), &["US"]);
    assert_eq!(policy.allowed_origins(), &["https://example.com"]);
    assert_eq!(policy.enable_strict_origin_enforcement(), Some(true));
}

#[tokio::test]
async fn sdk_test_get_playback_restriction_policy() {
    let client = make_ivs_client().await;
    let create_resp = client
        .create_playback_restriction_policy()
        .send()
        .await
        .expect("create policy");
    let arn = create_resp
        .playback_restriction_policy()
        .expect("policy")
        .arn()
        .to_string();

    let get_resp = client
        .get_playback_restriction_policy()
        .arn(&arn)
        .send()
        .await
        .expect("get policy");
    assert_eq!(
        get_resp.playback_restriction_policy().map(|p| p.arn()),
        Some(arn.as_str())
    );
}

#[tokio::test]
async fn sdk_test_update_playback_restriction_policy() {
    let client = make_ivs_client().await;
    let create_resp = client
        .create_playback_restriction_policy()
        .name("orig")
        .allowed_countries("US")
        .send()
        .await
        .expect("create policy");
    let arn = create_resp
        .playback_restriction_policy()
        .expect("policy")
        .arn()
        .to_string();

    let update_resp = client
        .update_playback_restriction_policy()
        .arn(&arn)
        .allowed_countries("CA")
        .allowed_countries("US")
        .send()
        .await
        .expect("update policy");
    let policy = update_resp.playback_restriction_policy().expect("policy");
    assert_eq!(policy.allowed_countries().len(), 2);
}

#[tokio::test]
async fn sdk_test_delete_playback_restriction_policy() {
    let client = make_ivs_client().await;
    let create_resp = client
        .create_playback_restriction_policy()
        .send()
        .await
        .expect("create policy");
    let arn = create_resp
        .playback_restriction_policy()
        .expect("policy")
        .arn()
        .to_string();

    client
        .delete_playback_restriction_policy()
        .arn(&arn)
        .send()
        .await
        .expect("delete policy");

    let err = client
        .get_playback_restriction_policy()
        .arn(&arn)
        .send()
        .await;
    assert!(err.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn sdk_test_list_playback_restriction_policies() {
    let client = make_ivs_client().await;
    client
        .create_playback_restriction_policy()
        .send()
        .await
        .expect("p1");
    client
        .create_playback_restriction_policy()
        .send()
        .await
        .expect("p2");
    client
        .create_playback_restriction_policy()
        .send()
        .await
        .expect("p3");

    let list = client
        .list_playback_restriction_policies()
        .send()
        .await
        .expect("list");
    assert_eq!(list.playback_restriction_policies().len(), 3);
}

// ============================================================================
// Stream operation tests (stateless stubs)
// ============================================================================

#[tokio::test]
async fn sdk_test_get_stream_for_existing_channel() {
    let client = make_ivs_client().await;
    let ch = client
        .create_channel()
        .name("stream-channel")
        .send()
        .await
        .expect("create ch");
    let channel_arn = ch.channel().and_then(|c| c.arn()).expect("arn").to_string();

    let resp = client
        .get_stream()
        .channel_arn(&channel_arn)
        .send()
        .await
        .expect("get stream");
    let stream = resp.stream().expect("stream");
    assert_eq!(stream.channel_arn(), Some(channel_arn.as_str()));
}

#[tokio::test]
async fn sdk_test_list_streams_empty() {
    let client = make_ivs_client().await;
    let list = client.list_streams().send().await.expect("list streams");
    assert_eq!(list.streams().len(), 0);
}

#[tokio::test]
async fn sdk_test_list_stream_sessions() {
    let client = make_ivs_client().await;
    let ch = client
        .create_channel()
        .name("sessions-channel")
        .send()
        .await
        .expect("create ch");
    let channel_arn = ch.channel().and_then(|c| c.arn()).expect("arn").to_string();

    let list = client
        .list_stream_sessions()
        .channel_arn(&channel_arn)
        .send()
        .await
        .expect("list sessions");
    assert_eq!(list.stream_sessions().len(), 0);
}

#[tokio::test]
async fn sdk_test_stop_stream() {
    let client = make_ivs_client().await;
    let ch = client
        .create_channel()
        .name("stop-channel")
        .send()
        .await
        .expect("create ch");
    let channel_arn = ch.channel().and_then(|c| c.arn()).expect("arn").to_string();

    // StopStream on an existing (but offline) channel succeeds
    client
        .stop_stream()
        .channel_arn(&channel_arn)
        .send()
        .await
        .expect("stop stream");
}

// ============================================================================
// Tag operation tests
// ============================================================================

#[tokio::test]
async fn sdk_test_tag_and_list_tags_for_channel() {
    let client = make_ivs_client().await;
    let ch = client
        .create_channel()
        .name("tag-channel")
        .send()
        .await
        .expect("create ch");
    let arn = ch.channel().and_then(|c| c.arn()).expect("arn").to_string();

    let mut tags = std::collections::HashMap::new();
    tags.insert("env".to_string(), "test".to_string());
    tags.insert("project".to_string(), "ivs".to_string());
    client
        .tag_resource()
        .resource_arn(&arn)
        .set_tags(Some(tags))
        .send()
        .await
        .expect("tag resource");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    let returned_tags = list_resp.tags();
    assert_eq!(returned_tags.get("env"), Some(&"test".to_string()));
    assert_eq!(returned_tags.get("project"), Some(&"ivs".to_string()));
}

#[tokio::test]
async fn sdk_test_untag_resource() {
    let client = make_ivs_client().await;
    let ch = client
        .create_channel()
        .name("untag-channel")
        .send()
        .await
        .expect("create ch");
    let arn = ch.channel().and_then(|c| c.arn()).expect("arn").to_string();

    let mut tags = std::collections::HashMap::new();
    tags.insert("key1".to_string(), "val1".to_string());
    tags.insert("key2".to_string(), "val2".to_string());
    client
        .tag_resource()
        .resource_arn(&arn)
        .set_tags(Some(tags))
        .send()
        .await
        .expect("tag");

    // Untag key1
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("key1")
        .send()
        .await
        .expect("untag");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    let returned_tags = list_resp.tags();
    assert!(
        !returned_tags.contains_key("key1"),
        "key1 should be removed"
    );
    assert_eq!(
        returned_tags.get("key2"),
        Some(&"val2".to_string()),
        "key2 should remain"
    );
}

#[tokio::test]
async fn sdk_test_list_tags_empty() {
    let client = make_ivs_client().await;
    let ch = client
        .create_channel()
        .name("no-tags-channel")
        .send()
        .await
        .expect("create ch");
    let arn = ch.channel().and_then(|c| c.arn()).expect("arn").to_string();

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    assert!(list_resp.tags().is_empty());
}

// ============================================================================
// PutMetadata test
// ============================================================================

#[tokio::test]
async fn sdk_test_put_metadata_for_channel() {
    let client = make_ivs_client().await;
    let ch = client
        .create_channel()
        .name("meta-channel")
        .send()
        .await
        .expect("create ch");
    let channel_arn = ch.channel().and_then(|c| c.arn()).expect("arn").to_string();

    client
        .put_metadata()
        .channel_arn(&channel_arn)
        .metadata("{\"event\": \"test\"}")
        .send()
        .await
        .expect("put metadata");
}

// ============================================================================
// BatchStartViewerSessionRevocation test
// ============================================================================

#[tokio::test]
async fn sdk_test_batch_start_viewer_session_revocation() {
    let client = make_ivs_client().await;
    let ch = client
        .create_channel()
        .name("revoke-channel")
        .send()
        .await
        .expect("create ch");
    let channel_arn = ch.channel().and_then(|c| c.arn()).expect("arn").to_string();

    let resp = client
        .batch_start_viewer_session_revocation()
        .viewer_sessions(
            aws_sdk_ivs::types::BatchStartViewerSessionRevocationViewerSession::builder()
                .channel_arn(&channel_arn)
                .viewer_id("viewer-123")
                .build()
                .expect("viewer session"),
        )
        .send()
        .await
        .expect("batch revoke");
    assert_eq!(resp.errors().len(), 0);
}

// ============================================================================
// StartViewerSessionRevocation test
// ============================================================================

#[tokio::test]
async fn sdk_test_start_viewer_session_revocation() {
    let client = make_ivs_client().await;
    let ch = client
        .create_channel()
        .name("revoke1-channel")
        .send()
        .await
        .expect("create ch");
    let channel_arn = ch.channel().and_then(|c| c.arn()).expect("arn").to_string();

    client
        .start_viewer_session_revocation()
        .channel_arn(&channel_arn)
        .viewer_id("viewer-456")
        .send()
        .await
        .expect("start viewer session revocation");
}

// ============================================================================
// State view tests
// ============================================================================

#[tokio::test]
async fn test_state_view_snapshot_restore() {
    use winterbaume_core::StatefulService;
    use winterbaume_ivs::IvsStateView;
    use winterbaume_ivs::views::ChannelView;

    let svc = IvsService::new();

    let mut channels = std::collections::HashMap::new();
    channels.insert(
        "arn:aws:ivs:us-east-1:123456789012:channel/abc123".to_string(),
        ChannelView {
            arn: "arn:aws:ivs:us-east-1:123456789012:channel/abc123".to_string(),
            name: "restored-channel".to_string(),
            latency_mode: "LOW".to_string(),
            channel_type: "STANDARD".to_string(),
            authorized: false,
            tags: std::collections::HashMap::new(),
        },
    );
    let view = IvsStateView {
        channels,
        ..Default::default()
    };

    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore");

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        snapshot
            .channels
            .contains_key("arn:aws:ivs:us-east-1:123456789012:channel/abc123")
    );
    assert_eq!(
        snapshot.channels["arn:aws:ivs:us-east-1:123456789012:channel/abc123"].name,
        "restored-channel"
    );
}

#[tokio::test]
async fn test_state_view_merge_additive() {
    use winterbaume_core::StatefulService;
    use winterbaume_ivs::IvsStateView;
    use winterbaume_ivs::views::ChannelView;

    let svc = IvsService::new();

    let mut channels1 = std::collections::HashMap::new();
    channels1.insert(
        "arn:aws:ivs:us-east-1:123456789012:channel/original".to_string(),
        ChannelView {
            arn: "arn:aws:ivs:us-east-1:123456789012:channel/original".to_string(),
            name: "original".to_string(),
            latency_mode: "LOW".to_string(),
            channel_type: "STANDARD".to_string(),
            authorized: false,
            tags: std::collections::HashMap::new(),
        },
    );
    svc.restore(
        "123456789012",
        "us-east-1",
        IvsStateView {
            channels: channels1,
            ..Default::default()
        },
    )
    .await
    .expect("restore");

    let mut channels2 = std::collections::HashMap::new();
    channels2.insert(
        "arn:aws:ivs:us-east-1:123456789012:channel/merged".to_string(),
        ChannelView {
            arn: "arn:aws:ivs:us-east-1:123456789012:channel/merged".to_string(),
            name: "merged".to_string(),
            latency_mode: "LOW".to_string(),
            channel_type: "STANDARD".to_string(),
            authorized: false,
            tags: std::collections::HashMap::new(),
        },
    );
    svc.merge(
        "123456789012",
        "us-east-1",
        IvsStateView {
            channels: channels2,
            ..Default::default()
        },
    )
    .await
    .expect("merge");

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        snapshot
            .channels
            .contains_key("arn:aws:ivs:us-east-1:123456789012:channel/original"),
        "original should still be present"
    );
    assert!(
        snapshot
            .channels
            .contains_key("arn:aws:ivs:us-east-1:123456789012:channel/merged"),
        "merged should be present"
    );
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = IvsService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .expect("restore");
    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}
