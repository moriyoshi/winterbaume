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

/// Scenario: Live-streaming ingest pipeline setup
///
/// A media operator provisions a channel and attaches multiple origin endpoints
/// (HLS and DASH delivery) to distribute live video. The scenario exercises
/// the full channel-to-endpoint lifecycle: create, describe, update, and
/// eventual teardown in the correct dependency order.
#[tokio::test]
async fn test_live_streaming_ingest_pipeline() {
    let client = make_mediapackage_client().await;

    // Step 1 — Create a channel for live ingest
    let channel = client
        .create_channel()
        .id("live-news-channel")
        .description("Live news ingest channel")
        .tags("env", "production")
        .tags("team", "broadcast")
        .send()
        .await
        .expect("creating live channel should succeed");

    assert_eq!(channel.id().unwrap_or_default(), "live-news-channel");
    assert!(
        channel
            .arn()
            .unwrap_or_default()
            .contains("live-news-channel"),
        "channel ARN should reference channel id"
    );

    // Step 2 — Verify channel is discoverable in list
    let list_resp = client
        .list_channels()
        .send()
        .await
        .expect("listing channels should succeed");

    let channel_ids: Vec<&str> = list_resp
        .channels()
        .iter()
        .filter_map(|ch| ch.id())
        .collect();
    assert!(
        channel_ids.contains(&"live-news-channel"),
        "new channel should appear in list"
    );

    // Step 3 — Create first origin endpoint (HLS for standard delivery)
    let hls_ep = client
        .create_origin_endpoint()
        .id("live-news-hls")
        .channel_id("live-news-channel")
        .description("HLS delivery endpoint")
        .manifest_name("index")
        .startover_window_seconds(300)
        .time_delay_seconds(0)
        .tags("format", "hls")
        .send()
        .await
        .expect("creating HLS origin endpoint should succeed");

    assert_eq!(hls_ep.id().unwrap_or_default(), "live-news-hls");
    assert_eq!(hls_ep.channel_id().unwrap_or_default(), "live-news-channel");
    assert!(
        !hls_ep.url().unwrap_or_default().is_empty(),
        "HLS URL should be populated"
    );

    // Step 4 — Create second origin endpoint (DASH for adaptive bitrate)
    let dash_ep = client
        .create_origin_endpoint()
        .id("live-news-dash")
        .channel_id("live-news-channel")
        .description("DASH delivery endpoint")
        .manifest_name("manifest")
        .startover_window_seconds(600)
        .time_delay_seconds(5)
        .tags("format", "dash")
        .send()
        .await
        .expect("creating DASH origin endpoint should succeed");

    assert_eq!(dash_ep.id().unwrap_or_default(), "live-news-dash");

    // Step 5 — Describe the channel and confirm both endpoints are reachable
    let _described_channel = client
        .describe_channel()
        .id("live-news-channel")
        .send()
        .await
        .expect("describing channel should succeed");

    let all_endpoints = client
        .list_origin_endpoints()
        .channel_id("live-news-channel")
        .send()
        .await
        .expect("listing origin endpoints for channel should succeed");

    assert_eq!(
        all_endpoints.origin_endpoints().len(),
        2,
        "both HLS and DASH endpoints should be present"
    );

    // Step 6 — Update the DASH endpoint to extend the DVR window
    let updated_dash = client
        .update_origin_endpoint()
        .id("live-news-dash")
        .description("DASH delivery endpoint — extended DVR")
        .startover_window_seconds(1800)
        .send()
        .await
        .expect("updating DASH endpoint should succeed");

    assert_eq!(
        updated_dash.startover_window_seconds(),
        Some(1800),
        "startover window should be updated"
    );
    assert_eq!(
        updated_dash.description().unwrap_or_default(),
        "DASH delivery endpoint — extended DVR"
    );

    // Step 7 — Teardown: delete endpoints before channel
    client
        .delete_origin_endpoint()
        .id("live-news-hls")
        .send()
        .await
        .expect("deleting HLS endpoint should succeed");

    client
        .delete_origin_endpoint()
        .id("live-news-dash")
        .send()
        .await
        .expect("deleting DASH endpoint should succeed");

    // Step 8 — Confirm endpoints are gone
    let after_delete = client
        .list_origin_endpoints()
        .channel_id("live-news-channel")
        .send()
        .await
        .expect("listing should succeed even with no endpoints");
    assert_eq!(
        after_delete.origin_endpoints().len(),
        0,
        "no endpoints should remain after deletion"
    );

    // Step 9 — Delete the channel and confirm it is gone
    client
        .delete_channel()
        .id("live-news-channel")
        .send()
        .await
        .expect("deleting channel should succeed");

    let result = client
        .describe_channel()
        .id("live-news-channel")
        .send()
        .await;
    assert!(result.is_err(), "channel should not exist after deletion");
}

/// Scenario: Multi-channel content distribution with independent endpoint pools
///
/// An operator runs separate channels for sports and news, each with its own
/// origin endpoints. The scenario verifies that per-channel endpoint filtering
/// correctly isolates resources across channels.
#[tokio::test]
async fn test_multi_channel_isolation() {
    let client = make_mediapackage_client().await;

    // Create two independent channels
    client
        .create_channel()
        .id("sports-ch")
        .description("Sports content")
        .send()
        .await
        .unwrap();

    client
        .create_channel()
        .id("news-ch")
        .description("News content")
        .send()
        .await
        .unwrap();

    // Add endpoints to each channel
    client
        .create_origin_endpoint()
        .id("sports-ep-1")
        .channel_id("sports-ch")
        .send()
        .await
        .unwrap();

    client
        .create_origin_endpoint()
        .id("sports-ep-2")
        .channel_id("sports-ch")
        .send()
        .await
        .unwrap();

    client
        .create_origin_endpoint()
        .id("news-ep-1")
        .channel_id("news-ch")
        .send()
        .await
        .unwrap();

    // Per-channel filtering must not cross-contaminate
    let sports_eps = client
        .list_origin_endpoints()
        .channel_id("sports-ch")
        .send()
        .await
        .expect("listing sports endpoints should succeed");

    let news_eps = client
        .list_origin_endpoints()
        .channel_id("news-ch")
        .send()
        .await
        .expect("listing news endpoints should succeed");

    assert_eq!(
        sports_eps.origin_endpoints().len(),
        2,
        "sports channel should have 2 endpoints"
    );
    assert_eq!(
        news_eps.origin_endpoints().len(),
        1,
        "news channel should have 1 endpoint"
    );

    // Deleting one channel's endpoints should not affect the other
    client
        .delete_origin_endpoint()
        .id("sports-ep-1")
        .send()
        .await
        .unwrap();
    client
        .delete_origin_endpoint()
        .id("sports-ep-2")
        .send()
        .await
        .unwrap();

    let news_eps_after = client
        .list_origin_endpoints()
        .channel_id("news-ch")
        .send()
        .await
        .expect("news channel endpoints should be unaffected");

    assert_eq!(
        news_eps_after.origin_endpoints().len(),
        1,
        "news endpoints should be unaffected by sports channel teardown"
    );

    // Cleanup
    client
        .delete_origin_endpoint()
        .id("news-ep-1")
        .send()
        .await
        .unwrap();
    client
        .delete_channel()
        .id("sports-ch")
        .send()
        .await
        .unwrap();
    client.delete_channel().id("news-ch").send().await.unwrap();
}
