use aws_sdk_mediastoredata::config::BehaviorVersion;
use aws_sdk_mediastoredata::primitives::ByteStream;
use winterbaume_core::MockAws;
use winterbaume_mediastoredata::MediaStoreDataService;

async fn make_client() -> aws_sdk_mediastoredata::Client {
    let mock = MockAws::builder()
        .with_service(MediaStoreDataService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediastoredata::config::Region::new("us-east-1"))
        .endpoint_url("https://data.mediastore.us-east-1.amazonaws.com")
        .load()
        .await;

    aws_sdk_mediastoredata::Client::new(&config)
}

/// Scenario: Media asset ingest, verify, and removal lifecycle
///
/// Models a typical content-delivery workflow: upload several video assets to
/// a container path, confirm they are discoverable via ListItems, retrieve each
/// one by path, then remove them and assert the container is empty.
#[tokio::test]
async fn test_media_asset_ingest_and_removal_lifecycle() {
    let client = make_client().await;

    // Step 1: Upload two video assets into a "live/2024" folder
    for (name, body) in [
        ("live/2024/clip-a.mp4", b"fake-mp4-a" as &[u8]),
        ("live/2024/clip-b.mp4", b"fake-mp4-b"),
    ] {
        client
            .put_object()
            .path(name)
            .content_type("video/mp4")
            .body(ByteStream::from_static(body))
            .send()
            .await
            .unwrap_or_else(|e| panic!("put_object {name} failed: {e}"));
    }

    // Step 2: ListItems at root should expose "live" as a FOLDER
    let root_list = client
        .list_items()
        .send()
        .await
        .expect("list_items at root should succeed");
    let root_items = root_list.items();
    assert_eq!(root_items.len(), 1, "root should contain exactly one item");
    assert_eq!(
        root_items[0].name(),
        Some("live"),
        "folder name should be 'live'"
    );
    assert_eq!(
        root_items[0].r#type().map(|t| t.as_str()),
        Some("FOLDER"),
        "root item should be a FOLDER"
    );

    // Step 3: ListItems under "live/2024" should expose both clips
    let clips_list = client
        .list_items()
        .path("live/2024")
        .send()
        .await
        .expect("list_items under live/2024 should succeed");
    let clips = clips_list.items();
    assert_eq!(clips.len(), 2, "should see both clips under live/2024");
    let clip_names: Vec<Option<&str>> = clips.iter().map(|i| i.name()).collect();
    assert!(clip_names.contains(&Some("clip-a.mp4")));
    assert!(clip_names.contains(&Some("clip-b.mp4")));

    // Step 4: GetObject for each clip — assert content round-trips correctly
    for (short_name, expected_body) in [
        ("clip-a.mp4", b"fake-mp4-a" as &[u8]),
        ("clip-b.mp4", b"fake-mp4-b"),
    ] {
        let path = format!("live/2024/{short_name}");
        let get_resp = client
            .get_object()
            .path(&path)
            .send()
            .await
            .unwrap_or_else(|e| panic!("get_object {path} failed: {e}"));

        assert_eq!(
            get_resp.content_type(),
            Some("video/mp4"),
            "content_type should be video/mp4"
        );
        let body_bytes = get_resp
            .body
            .collect()
            .await
            .expect("should collect body")
            .into_bytes();
        assert_eq!(
            body_bytes.as_ref(),
            expected_body,
            "body should match for {path}"
        );
    }

    // Step 5: Delete both clips
    for name in ["live/2024/clip-a.mp4", "live/2024/clip-b.mp4"] {
        client
            .delete_object()
            .path(name)
            .send()
            .await
            .unwrap_or_else(|e| panic!("delete_object {name} failed: {e}"));
    }

    // Step 6: ListItems at root should now be empty (virtual folder disappears)
    let final_list = client
        .list_items()
        .send()
        .await
        .expect("list_items after deletion should succeed");
    assert_eq!(
        final_list.items().len(),
        0,
        "container should be empty after all objects are deleted"
    );
}

/// Scenario: Concurrent container paths with independent isolation
///
/// Two independent content paths are populated, verified to be independently
/// accessible, and then one is cleared without affecting the other.
#[tokio::test]
async fn test_independent_container_paths_isolation() {
    let client = make_client().await;

    // Step 1: Put objects into two separate virtual folders
    client
        .put_object()
        .path("news/breaking.txt")
        .content_type("text/plain")
        .body(ByteStream::from_static(b"breaking news content"))
        .send()
        .await
        .expect("put news/breaking.txt should succeed");

    client
        .put_object()
        .path("sports/match-report.txt")
        .content_type("text/plain")
        .body(ByteStream::from_static(b"sports match report"))
        .send()
        .await
        .expect("put sports/match-report.txt should succeed");

    // Step 2: ListItems under each path should be isolated
    let news_list = client
        .list_items()
        .path("news")
        .send()
        .await
        .expect("list_items under news should succeed");
    assert_eq!(news_list.items().len(), 1);
    assert_eq!(news_list.items()[0].name(), Some("breaking.txt"));

    let sports_list = client
        .list_items()
        .path("sports")
        .send()
        .await
        .expect("list_items under sports should succeed");
    assert_eq!(sports_list.items().len(), 1);
    assert_eq!(sports_list.items()[0].name(), Some("match-report.txt"));

    // Step 3: Delete the news object and verify sports is unaffected
    client
        .delete_object()
        .path("news/breaking.txt")
        .send()
        .await
        .expect("delete news/breaking.txt should succeed");

    let sports_still = client
        .list_items()
        .path("sports")
        .send()
        .await
        .expect("sports list should still succeed after news deletion");
    assert_eq!(
        sports_still.items().len(),
        1,
        "sports folder should still contain its object"
    );

    // Step 4: Confirm news folder is now empty
    let news_empty = client
        .list_items()
        .path("news")
        .send()
        .await
        .expect("news list should succeed after deletion");
    assert_eq!(
        news_empty.items().len(),
        0,
        "news folder should be empty after deletion"
    );
}
