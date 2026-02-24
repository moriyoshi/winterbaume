use aws_sdk_mediapackage::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediapackage::MediaPackageService;

async fn make_mediapackage_client() -> aws_sdk_mediapackage::Client {
    let mock = MockAws::builder()
        .with_service(MediaPackageService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediapackage::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_mediapackage::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_channel() {
    let client = make_mediapackage_client().await;

    let resp = client
        .create_channel()
        .id("test-channel")
        .description("A test channel")
        .send()
        .await
        .expect("create_channel should succeed");

    assert_eq!(resp.id().unwrap(), "test-channel");
    assert_eq!(resp.description().unwrap(), "A test channel");
    assert!(resp.arn().unwrap().contains("test-channel"));

    let desc = client
        .describe_channel()
        .id("test-channel")
        .send()
        .await
        .expect("describe_channel should succeed");

    assert_eq!(desc.id().unwrap(), "test-channel");
    assert_eq!(desc.description().unwrap(), "A test channel");
}

#[tokio::test]
async fn test_delete_channel() {
    let client = make_mediapackage_client().await;

    client
        .create_channel()
        .id("del-channel")
        .send()
        .await
        .unwrap();

    client
        .delete_channel()
        .id("del-channel")
        .send()
        .await
        .expect("delete_channel should succeed");

    let result = client.describe_channel().id("del-channel").send().await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_list_channels_empty() {
    let client = make_mediapackage_client().await;

    let resp = client
        .list_channels()
        .send()
        .await
        .expect("list_channels should succeed");

    assert_eq!(resp.channels().len(), 0);
}

#[tokio::test]
async fn test_list_channels_with_entries() {
    let client = make_mediapackage_client().await;

    client
        .create_channel()
        .id("ch-1")
        .description("First")
        .send()
        .await
        .unwrap();

    client
        .create_channel()
        .id("ch-2")
        .description("Second")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_channels()
        .send()
        .await
        .expect("list_channels should succeed");

    assert_eq!(resp.channels().len(), 2);
}

#[tokio::test]
async fn test_describe_nonexistent_channel_fails() {
    let client = make_mediapackage_client().await;

    let result = client.describe_channel().id("nonexistent").send().await;
    assert!(result.is_err(), "describe nonexistent channel should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_channel_fails() {
    let client = make_mediapackage_client().await;

    let result = client.delete_channel().id("nonexistent").send().await;
    assert!(result.is_err(), "delete nonexistent channel should fail");
}

#[tokio::test]
async fn test_create_duplicate_channel_fails() {
    let client = make_mediapackage_client().await;

    client
        .create_channel()
        .id("dup-channel")
        .send()
        .await
        .unwrap();

    let result = client.create_channel().id("dup-channel").send().await;
    assert!(result.is_err(), "duplicate channel creation should fail");
}

// --- Origin Endpoint operations ---

#[tokio::test]
async fn test_create_and_describe_origin_endpoint() {
    let client = make_mediapackage_client().await;

    client.create_channel().id("test-ch").send().await.unwrap();

    let resp = client
        .create_origin_endpoint()
        .id("test-ep")
        .channel_id("test-ch")
        .description("Test endpoint")
        .send()
        .await
        .expect("create_origin_endpoint should succeed");

    assert_eq!(resp.id().unwrap_or_default(), "test-ep");
    assert_eq!(resp.channel_id().unwrap_or_default(), "test-ch");
    assert!(resp.arn().unwrap_or_default().contains("origin_endpoints"));

    let desc = client
        .describe_origin_endpoint()
        .id("test-ep")
        .send()
        .await
        .expect("describe should succeed");

    assert_eq!(desc.id().unwrap_or_default(), "test-ep");
    assert_eq!(desc.description().unwrap_or_default(), "Test endpoint");
}

#[tokio::test]
async fn test_delete_origin_endpoint() {
    let client = make_mediapackage_client().await;

    client.create_channel().id("del-ch").send().await.unwrap();
    client
        .create_origin_endpoint()
        .id("del-ep")
        .channel_id("del-ch")
        .send()
        .await
        .unwrap();

    client
        .delete_origin_endpoint()
        .id("del-ep")
        .send()
        .await
        .expect("delete should succeed");

    let result = client.describe_origin_endpoint().id("del-ep").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_origin_endpoints() {
    let client = make_mediapackage_client().await;

    client.create_channel().id("list-ch").send().await.unwrap();
    client
        .create_origin_endpoint()
        .id("ep-1")
        .channel_id("list-ch")
        .send()
        .await
        .unwrap();
    client
        .create_origin_endpoint()
        .id("ep-2")
        .channel_id("list-ch")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_origin_endpoints()
        .channel_id("list-ch")
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(resp.origin_endpoints().len(), 2);
}

#[tokio::test]
async fn test_update_origin_endpoint() {
    let client = make_mediapackage_client().await;

    client.create_channel().id("upd-ch").send().await.unwrap();
    client
        .create_origin_endpoint()
        .id("upd-ep")
        .channel_id("upd-ch")
        .description("original")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_origin_endpoint()
        .id("upd-ep")
        .description("updated")
        .time_delay_seconds(30)
        .send()
        .await
        .expect("update should succeed");

    assert_eq!(resp.description().unwrap_or_default(), "updated");
    assert_eq!(resp.time_delay_seconds(), Some(30));
}

#[tokio::test]
async fn test_create_origin_endpoint_nonexistent_channel() {
    let client = make_mediapackage_client().await;

    let result = client
        .create_origin_endpoint()
        .id("bad-ep")
        .channel_id("nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: AWS Elemental MediaPackage
// ============================================================================

#[tokio::test]
async fn test_create_channel_with_tags() {
    let client = make_mediapackage_client().await;

    let resp = client
        .create_channel()
        .id("tagged-channel")
        .description("Channel with tags")
        .tags("env", "test")
        .tags("project", "winterbaume")
        .send()
        .await
        .expect("create_channel with tags should succeed");

    assert_eq!(resp.id().unwrap_or_default(), "tagged-channel");

    // Tags should be preserved in the response
    let tags = resp.tags();
    assert_eq!(
        tags.and_then(|m| m.get("env")).map(|s: &String| s.as_str()),
        Some("test")
    );
    assert_eq!(
        tags.and_then(|m| m.get("project"))
            .map(|s: &String| s.as_str()),
        Some("winterbaume")
    );

    // Describe should also return the tags
    let desc = client
        .describe_channel()
        .id("tagged-channel")
        .send()
        .await
        .expect("describe_channel should succeed");

    let tags = desc.tags();
    assert_eq!(
        tags.and_then(|m| m.get("env")).map(|s: &String| s.as_str()),
        Some("test")
    );
}

#[tokio::test]
async fn test_channel_arn_format() {
    let client = make_mediapackage_client().await;

    let resp = client
        .create_channel()
        .id("arn-channel")
        .send()
        .await
        .expect("create_channel should succeed");

    let arn = resp.arn().unwrap_or_default();
    assert!(
        arn.starts_with("arn:aws:mediapackage:"),
        "ARN should start with arn:aws:mediapackage:, got: {arn}"
    );
    assert!(
        arn.contains("channels/arn-channel"),
        "ARN should contain channels/arn-channel, got: {arn}"
    );
}

#[tokio::test]
async fn test_list_origin_endpoints_no_filter() {
    let client = make_mediapackage_client().await;

    client.create_channel().id("ch-a").send().await.unwrap();
    client.create_channel().id("ch-b").send().await.unwrap();
    client
        .create_origin_endpoint()
        .id("ep-a")
        .channel_id("ch-a")
        .send()
        .await
        .unwrap();
    client
        .create_origin_endpoint()
        .id("ep-b")
        .channel_id("ch-b")
        .send()
        .await
        .unwrap();

    // List without a channel filter should return all endpoints
    let resp = client
        .list_origin_endpoints()
        .send()
        .await
        .expect("list_origin_endpoints without filter should succeed");

    assert_eq!(
        resp.origin_endpoints().len(),
        2,
        "Expected 2 origin endpoints with no channel filter"
    );
}

#[tokio::test]
async fn test_list_origin_endpoints_empty_channel() {
    let client = make_mediapackage_client().await;

    client.create_channel().id("empty-ch").send().await.unwrap();

    let resp = client
        .list_origin_endpoints()
        .channel_id("empty-ch")
        .send()
        .await
        .expect("list_origin_endpoints on empty channel should succeed");

    assert_eq!(
        resp.origin_endpoints().len(),
        0,
        "Expected 0 endpoints for channel with no endpoints"
    );
}

#[tokio::test]
async fn test_create_duplicate_origin_endpoint_fails() {
    let client = make_mediapackage_client().await;

    client
        .create_channel()
        .id("dup-ep-ch")
        .send()
        .await
        .unwrap();
    client
        .create_origin_endpoint()
        .id("dup-ep")
        .channel_id("dup-ep-ch")
        .send()
        .await
        .unwrap();

    let result = client
        .create_origin_endpoint()
        .id("dup-ep")
        .channel_id("dup-ep-ch")
        .send()
        .await;

    assert!(
        result.is_err(),
        "Creating duplicate origin endpoint should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("UnprocessableEntityException")
            || err_str.contains("unprocessable")
            || err_str.contains("already exists"),
        "Expected conflict error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_nonexistent_origin_endpoint_fails() {
    let client = make_mediapackage_client().await;

    let result = client
        .describe_origin_endpoint()
        .id("nonexistent-ep")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe nonexistent origin endpoint should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected not-found error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_origin_endpoint_fails() {
    let client = make_mediapackage_client().await;

    let result = client
        .delete_origin_endpoint()
        .id("nonexistent-ep")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete nonexistent origin endpoint should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected not-found error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_nonexistent_origin_endpoint_fails() {
    let client = make_mediapackage_client().await;

    let result = client
        .update_origin_endpoint()
        .id("nonexistent-ep")
        .description("should fail")
        .send()
        .await;

    assert!(
        result.is_err(),
        "update nonexistent origin endpoint should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected not-found error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_origin_endpoint_manifest_and_startover() {
    let client = make_mediapackage_client().await;

    client.create_channel().id("mso-ch").send().await.unwrap();
    client
        .create_origin_endpoint()
        .id("mso-ep")
        .channel_id("mso-ch")
        .manifest_name("original-manifest")
        .startover_window_seconds(0)
        .send()
        .await
        .unwrap();

    let resp = client
        .update_origin_endpoint()
        .id("mso-ep")
        .manifest_name("updated-manifest")
        .startover_window_seconds(300)
        .send()
        .await
        .expect("update should succeed");

    assert_eq!(
        resp.manifest_name().unwrap_or_default(),
        "updated-manifest",
        "manifest_name should be updated"
    );
    assert_eq!(
        resp.startover_window_seconds(),
        Some(300),
        "startover_window_seconds should be updated"
    );
}

#[tokio::test]
async fn test_origin_endpoint_url_and_arn_format() {
    let client = make_mediapackage_client().await;

    client.create_channel().id("url-ch").send().await.unwrap();

    let resp = client
        .create_origin_endpoint()
        .id("url-ep")
        .channel_id("url-ch")
        .send()
        .await
        .expect("create_origin_endpoint should succeed");

    let arn = resp.arn().unwrap_or_default();
    assert!(
        arn.starts_with("arn:aws:mediapackage:"),
        "ARN should start with arn:aws:mediapackage:, got: {arn}"
    );
    assert!(
        arn.contains("origin_endpoints/url-ep"),
        "ARN should contain origin_endpoints/url-ep, got: {arn}"
    );

    let url = resp.url().unwrap_or_default();
    assert!(!url.is_empty(), "URL should not be empty");
    assert!(
        url.contains("url-ep"),
        "URL should contain the endpoint id, got: {url}"
    );
}

#[tokio::test]
async fn test_full_lifecycle() {
    let client = make_mediapackage_client().await;

    // Create channel
    let ch = client
        .create_channel()
        .id("lifecycle-ch")
        .description("lifecycle test")
        .send()
        .await
        .expect("create_channel should succeed");
    assert_eq!(ch.id().unwrap_or_default(), "lifecycle-ch");

    // Create origin endpoint
    let ep = client
        .create_origin_endpoint()
        .id("lifecycle-ep")
        .channel_id("lifecycle-ch")
        .description("original description")
        .time_delay_seconds(10)
        .send()
        .await
        .expect("create_origin_endpoint should succeed");
    assert_eq!(ep.id().unwrap_or_default(), "lifecycle-ep");
    assert_eq!(ep.channel_id().unwrap_or_default(), "lifecycle-ch");

    // Describe endpoint
    let desc = client
        .describe_origin_endpoint()
        .id("lifecycle-ep")
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(
        desc.description().unwrap_or_default(),
        "original description"
    );
    assert_eq!(desc.time_delay_seconds(), Some(10));

    // Update endpoint
    let updated = client
        .update_origin_endpoint()
        .id("lifecycle-ep")
        .description("updated description")
        .time_delay_seconds(60)
        .send()
        .await
        .expect("update should succeed");
    assert_eq!(
        updated.description().unwrap_or_default(),
        "updated description"
    );
    assert_eq!(updated.time_delay_seconds(), Some(60));

    // Delete endpoint
    client
        .delete_origin_endpoint()
        .id("lifecycle-ep")
        .send()
        .await
        .expect("delete endpoint should succeed");

    // Verify endpoint is gone
    let result = client
        .describe_origin_endpoint()
        .id("lifecycle-ep")
        .send()
        .await;
    assert!(
        result.is_err(),
        "Endpoint should no longer exist after delete"
    );

    // Delete channel
    client
        .delete_channel()
        .id("lifecycle-ch")
        .send()
        .await
        .expect("delete channel should succeed");

    // Verify channel is gone
    let result = client.describe_channel().id("lifecycle-ch").send().await;
    assert!(
        result.is_err(),
        "Channel should no longer exist after delete"
    );
}
