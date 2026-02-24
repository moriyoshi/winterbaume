use aws_sdk_mediastoredata::config::BehaviorVersion;
use aws_sdk_mediastoredata::primitives::ByteStream;
use winterbaume_core::MockAws;
use winterbaume_mediastoredata::MediaStoreDataService;

async fn make_mediastoredata_client() -> aws_sdk_mediastoredata::Client {
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

#[tokio::test]
async fn test_put_and_get_object() {
    let client = make_mediastoredata_client().await;

    let body_content = b"hello world";

    client
        .put_object()
        .path("test/file.txt")
        .content_type("text/plain")
        .body(ByteStream::from_static(body_content))
        .send()
        .await
        .expect("put_object should succeed");

    let get_resp = client
        .get_object()
        .path("test/file.txt")
        .send()
        .await
        .expect("get_object should succeed");

    let returned_body = get_resp
        .body
        .collect()
        .await
        .expect("should collect body")
        .into_bytes();

    assert_eq!(returned_body.as_ref(), body_content);
}

#[tokio::test]
async fn test_delete_object() {
    let client = make_mediastoredata_client().await;

    client
        .put_object()
        .path("delete-me.txt")
        .content_type("text/plain")
        .body(ByteStream::from_static(b"data"))
        .send()
        .await
        .unwrap();

    client
        .delete_object()
        .path("delete-me.txt")
        .send()
        .await
        .expect("delete_object should succeed");

    let result = client.get_object().path("delete-me.txt").send().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_object() {
    let client = make_mediastoredata_client().await;

    let body_content = b"describe me";

    client
        .put_object()
        .path("describe/file.bin")
        .content_type("application/octet-stream")
        .body(ByteStream::from_static(body_content))
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_object()
        .path("describe/file.bin")
        .send()
        .await
        .expect("describe_object should succeed");

    assert_eq!(resp.content_length, Some(body_content.len() as i64));
    assert_eq!(
        resp.content_type,
        Some("application/octet-stream".to_string())
    );
    assert!(resp.e_tag.is_some());
}

#[tokio::test]
async fn test_describe_nonexistent_object() {
    let client = make_mediastoredata_client().await;

    let result = client
        .describe_object()
        .path("nonexistent.txt")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_items_empty() {
    let client = make_mediastoredata_client().await;

    let resp = client
        .list_items()
        .send()
        .await
        .expect("list_items should succeed");

    assert!(resp.items().is_empty());
}

#[tokio::test]
async fn test_list_items_with_objects() {
    let client = make_mediastoredata_client().await;

    for name in ["a.txt", "b.txt", "c.txt"] {
        client
            .put_object()
            .path(name)
            .content_type("text/plain")
            .body(ByteStream::from_static(b"data"))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_items()
        .send()
        .await
        .expect("list_items should succeed");

    assert_eq!(resp.items().len(), 3);
}

#[tokio::test]
async fn test_list_items_with_path_prefix() {
    let client = make_mediastoredata_client().await;

    client
        .put_object()
        .path("folder/file1.txt")
        .content_type("text/plain")
        .body(ByteStream::from_static(b"data1"))
        .send()
        .await
        .unwrap();

    client
        .put_object()
        .path("folder/file2.txt")
        .content_type("text/plain")
        .body(ByteStream::from_static(b"data2"))
        .send()
        .await
        .unwrap();

    client
        .put_object()
        .path("other.txt")
        .content_type("text/plain")
        .body(ByteStream::from_static(b"other"))
        .send()
        .await
        .unwrap();

    // List root - should see "folder" (as folder) and "other.txt" (as object)
    let resp = client
        .list_items()
        .send()
        .await
        .expect("list_items should succeed");

    assert_eq!(resp.items().len(), 2);

    // List under "folder" - should see file1.txt and file2.txt
    let resp = client
        .list_items()
        .path("folder")
        .send()
        .await
        .expect("list_items with path should succeed");

    assert_eq!(resp.items().len(), 2);
}

#[tokio::test]
async fn test_put_object_overwrite() {
    let client = make_mediastoredata_client().await;

    client
        .put_object()
        .path("overwrite.txt")
        .content_type("text/plain")
        .body(ByteStream::from_static(b"original"))
        .send()
        .await
        .unwrap();

    client
        .put_object()
        .path("overwrite.txt")
        .content_type("text/plain")
        .body(ByteStream::from_static(b"updated"))
        .send()
        .await
        .unwrap();

    let get_resp = client
        .get_object()
        .path("overwrite.txt")
        .send()
        .await
        .expect("get_object should succeed");

    let returned_body = get_resp
        .body
        .collect()
        .await
        .expect("should collect body")
        .into_bytes();

    assert_eq!(returned_body.as_ref(), b"updated");
}

#[tokio::test]
async fn test_get_nonexistent_object() {
    let client = make_mediastoredata_client().await;

    let result = client.get_object().path("nonexistent.txt").send().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_object_fails() {
    let client = make_mediastoredata_client().await;

    // DeleteObject throws ObjectNotFoundException when the object doesn't exist
    let result = client.delete_object().path("nonexistent.txt").send().await;

    assert!(result.is_err(), "delete of nonexistent object should fail");
}

// --- New moto-ported tests ---

#[tokio::test]
async fn test_put_object_returns_temporal_storage_class() {
    let client = make_mediastoredata_client().await;

    let resp = client
        .put_object()
        .path("foo")
        .body(ByteStream::from_static(b"011001"))
        .send()
        .await
        .expect("put_object should succeed");

    assert_eq!(resp.storage_class().map(|s| s.as_str()), Some("TEMPORAL"));

    // Item should appear in list
    let list_resp = client
        .list_items()
        .send()
        .await
        .expect("list_items should succeed");

    let items = list_resp.items();
    assert!(items.iter().any(|i| i.name() == Some("foo")));
}

#[tokio::test]
async fn test_get_object_not_found_raises_object_not_found_exception() {
    let client = make_mediastoredata_client().await;

    let result = client.get_object().path("foo").send().await;

    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_object_not_found_exception(),
        "should be ObjectNotFoundException"
    );
}

#[tokio::test]
async fn test_delete_object_not_found_raises_object_not_found_exception() {
    let client = make_mediastoredata_client().await;

    let result = client.delete_object().path("foo").send().await;

    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_object_not_found_exception(),
        "should be ObjectNotFoundException"
    );
}

#[tokio::test]
async fn test_delete_object_and_verify_gone_from_list() {
    let client = make_mediastoredata_client().await;

    let object_path = "foo";

    client
        .put_object()
        .path(object_path)
        .body(ByteStream::from_static(b"011001"))
        .send()
        .await
        .expect("put_object should succeed");

    client
        .delete_object()
        .path(object_path)
        .send()
        .await
        .expect("delete_object should succeed");

    let list_resp = client
        .list_items()
        .send()
        .await
        .expect("list_items should succeed");

    assert_eq!(list_resp.items().len(), 0);
}

#[tokio::test]
async fn test_list_items_shows_put_objects() {
    let client = make_mediastoredata_client().await;

    // Initially empty
    let resp = client.list_items().send().await.expect("should succeed");
    assert_eq!(resp.items().len(), 0);

    // Put an object
    client
        .put_object()
        .path("foo")
        .body(ByteStream::from_static(b"011001"))
        .send()
        .await
        .expect("put_object should succeed");

    let resp = client.list_items().send().await.expect("should succeed");
    assert_eq!(resp.items().len(), 1);
    assert!(resp.items().iter().any(|i| i.name() == Some("foo")));
}

// ============================================================================
// Tests derived from AWS documentation: AWS Elemental MediaStore Data Plane
// ============================================================================

#[tokio::test]
async fn test_put_object_response_fields() {
    let client = make_mediastoredata_client().await;

    let resp = client
        .put_object()
        .path("response-fields.bin")
        .content_type("application/octet-stream")
        .body(ByteStream::from_static(b"test data"))
        .send()
        .await
        .expect("put_object should succeed");

    // PutObject must return ETag and ContentSHA256
    assert!(
        resp.e_tag().is_some() && !resp.e_tag().unwrap_or_default().is_empty(),
        "e_tag should be present and non-empty"
    );
    assert!(
        resp.content_sha256().is_some() && !resp.content_sha256().unwrap_or_default().is_empty(),
        "content_sha256 should be present and non-empty"
    );
    // StorageClass must be TEMPORAL
    assert_eq!(
        resp.storage_class().map(|s| s.as_str()),
        Some("TEMPORAL"),
        "storage_class should be TEMPORAL"
    );
}

#[tokio::test]
async fn test_put_object_with_cache_control() {
    let client = make_mediastoredata_client().await;

    let cache_control_value = "max-age=3600, public";

    client
        .put_object()
        .path("cached-object.txt")
        .content_type("text/plain")
        .cache_control(cache_control_value)
        .body(ByteStream::from_static(b"cacheable content"))
        .send()
        .await
        .expect("put_object with cache_control should succeed");

    let get_resp = client
        .get_object()
        .path("cached-object.txt")
        .send()
        .await
        .expect("get_object should succeed");

    assert_eq!(
        get_resp.cache_control(),
        Some(cache_control_value),
        "cache_control should be preserved in get response"
    );
}

#[tokio::test]
async fn test_get_object_content_type_preserved() {
    let client = make_mediastoredata_client().await;

    let expected_content_type = "video/mp4";

    client
        .put_object()
        .path("video.mp4")
        .content_type(expected_content_type)
        .body(ByteStream::from_static(b"fake video bytes"))
        .send()
        .await
        .expect("put_object should succeed");

    let get_resp = client
        .get_object()
        .path("video.mp4")
        .send()
        .await
        .expect("get_object should succeed");

    assert_eq!(
        get_resp.content_type(),
        Some(expected_content_type),
        "content_type should match what was put"
    );
}

#[tokio::test]
async fn test_get_object_header_fields() {
    let client = make_mediastoredata_client().await;

    let body_bytes: &[u8] = b"header field test";

    client
        .put_object()
        .path("header-test.bin")
        .content_type("application/octet-stream")
        .body(ByteStream::from_static(body_bytes))
        .send()
        .await
        .expect("put_object should succeed");

    let get_resp = client
        .get_object()
        .path("header-test.bin")
        .send()
        .await
        .expect("get_object should succeed");

    assert!(
        get_resp.e_tag().is_some() && !get_resp.e_tag().unwrap_or_default().is_empty(),
        "e_tag should be present"
    );
    assert_eq!(
        get_resp.content_length(),
        Some(body_bytes.len() as i64),
        "content_length should match body size"
    );
    assert!(
        get_resp.last_modified().is_some(),
        "last_modified should be present"
    );
}

#[tokio::test]
async fn test_describe_object_last_modified() {
    let client = make_mediastoredata_client().await;

    client
        .put_object()
        .path("timestamped.txt")
        .content_type("text/plain")
        .body(ByteStream::from_static(b"timestamped"))
        .send()
        .await
        .expect("put_object should succeed");

    let resp = client
        .describe_object()
        .path("timestamped.txt")
        .send()
        .await
        .expect("describe_object should succeed");

    assert!(
        resp.last_modified().is_some(),
        "last_modified should be present in describe response"
    );
    assert!(
        resp.e_tag().is_some() && !resp.e_tag().unwrap_or_default().is_empty(),
        "e_tag should be present in describe response"
    );
}

#[tokio::test]
async fn test_describe_object_typed_not_found() {
    let client = make_mediastoredata_client().await;

    let result = client
        .describe_object()
        .path("completely-missing-file.txt")
        .send()
        .await;

    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_object_not_found_exception(),
        "describe of nonexistent object should return ObjectNotFoundException"
    );
}

#[tokio::test]
async fn test_full_lifecycle() {
    let client = make_mediastoredata_client().await;

    // Step 1: put initial object
    client
        .put_object()
        .path("lifecycle/asset.bin")
        .content_type("application/octet-stream")
        .body(ByteStream::from_static(b"version-one"))
        .send()
        .await
        .expect("initial put_object should succeed");

    // Step 2: describe to confirm it exists
    let desc1 = client
        .describe_object()
        .path("lifecycle/asset.bin")
        .send()
        .await
        .expect("describe after put should succeed");
    assert_eq!(
        desc1.content_length(),
        Some(b"version-one".len() as i64),
        "content_length should match initial body"
    );

    // Step 3: overwrite with new content
    client
        .put_object()
        .path("lifecycle/asset.bin")
        .content_type("application/octet-stream")
        .body(ByteStream::from_static(b"version-two-updated"))
        .send()
        .await
        .expect("overwrite put_object should succeed");

    // Step 4: describe again - content_length should reflect new body
    let desc2 = client
        .describe_object()
        .path("lifecycle/asset.bin")
        .send()
        .await
        .expect("describe after overwrite should succeed");
    assert_eq!(
        desc2.content_length(),
        Some(b"version-two-updated".len() as i64),
        "content_length should match updated body"
    );

    // Step 5: delete the object
    client
        .delete_object()
        .path("lifecycle/asset.bin")
        .send()
        .await
        .expect("delete_object should succeed");

    // Step 6: describe should fail with ObjectNotFoundException
    let result = client
        .describe_object()
        .path("lifecycle/asset.bin")
        .send()
        .await;
    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_object_not_found_exception(),
        "describe after delete should return ObjectNotFoundException"
    );
}

#[tokio::test]
async fn test_list_items_object_metadata_fields() {
    let client = make_mediastoredata_client().await;

    let body_content: &[u8] = b"metadata test content";
    let expected_content_type = "image/jpeg";

    client
        .put_object()
        .path("meta.jpg")
        .content_type(expected_content_type)
        .body(ByteStream::from_static(body_content))
        .send()
        .await
        .expect("put_object should succeed");

    let resp = client
        .list_items()
        .send()
        .await
        .expect("list_items should succeed");

    let items = resp.items();
    assert_eq!(items.len(), 1);
    let item = &items[0];

    assert_eq!(item.name(), Some("meta.jpg"), "item name should match");
    assert_eq!(
        item.r#type().map(|t| t.as_str()),
        Some("OBJECT"),
        "item type should be OBJECT"
    );
    assert_eq!(
        item.content_type(),
        Some(expected_content_type),
        "content_type should match"
    );
    assert_eq!(
        item.content_length(),
        Some(body_content.len() as i64),
        "content_length should match body size"
    );
    assert!(
        item.e_tag().is_some() && !item.e_tag().unwrap_or_default().is_empty(),
        "e_tag should be present"
    );
    assert!(
        item.last_modified().is_some(),
        "last_modified should be present"
    );
}

#[tokio::test]
async fn test_list_items_folder_type_has_no_metadata() {
    let client = make_mediastoredata_client().await;

    // Put an object under a subfolder to create a virtual folder
    client
        .put_object()
        .path("videos/clip.mp4")
        .content_type("video/mp4")
        .body(ByteStream::from_static(b"fake mp4"))
        .send()
        .await
        .expect("put_object should succeed");

    // List root - should show "videos" as a FOLDER item
    let resp = client
        .list_items()
        .send()
        .await
        .expect("list_items at root should succeed");

    let items = resp.items();
    assert_eq!(items.len(), 1, "should see exactly one item (the folder)");
    let folder_item = &items[0];

    assert_eq!(
        folder_item.name(),
        Some("videos"),
        "folder name should be 'videos'"
    );
    assert_eq!(
        folder_item.r#type().map(|t| t.as_str()),
        Some("FOLDER"),
        "item type should be FOLDER"
    );
    // Folder items have no content metadata
    assert!(
        folder_item.content_type().is_none(),
        "folder item should have no content_type"
    );
    assert!(
        folder_item.content_length().is_none(),
        "folder item should have no content_length"
    );
    assert!(
        folder_item.e_tag().is_none(),
        "folder item should have no e_tag"
    );
    assert!(
        folder_item.last_modified().is_none(),
        "folder item should have no last_modified"
    );
}

#[tokio::test]
async fn test_list_items_deep_nesting() {
    let client = make_mediastoredata_client().await;

    // Create objects at multiple nesting levels
    client
        .put_object()
        .path("a/b/c/deep.txt")
        .content_type("text/plain")
        .body(ByteStream::from_static(b"deep"))
        .send()
        .await
        .expect("put deep object should succeed");

    client
        .put_object()
        .path("a/b/sibling.txt")
        .content_type("text/plain")
        .body(ByteStream::from_static(b"sibling"))
        .send()
        .await
        .expect("put sibling object should succeed");

    // List root - should see only "a" as FOLDER
    let root_resp = client
        .list_items()
        .send()
        .await
        .expect("list root should succeed");
    assert_eq!(root_resp.items().len(), 1);
    assert_eq!(root_resp.items()[0].name(), Some("a"));
    assert_eq!(
        root_resp.items()[0].r#type().map(|t| t.as_str()),
        Some("FOLDER")
    );

    // List "a" - should see only "b" as FOLDER
    let a_resp = client
        .list_items()
        .path("a")
        .send()
        .await
        .expect("list 'a' should succeed");
    assert_eq!(a_resp.items().len(), 1);
    assert_eq!(a_resp.items()[0].name(), Some("b"));
    assert_eq!(
        a_resp.items()[0].r#type().map(|t| t.as_str()),
        Some("FOLDER")
    );

    // List "a/b" - should see "c" (FOLDER) and "sibling.txt" (OBJECT)
    let ab_resp = client
        .list_items()
        .path("a/b")
        .send()
        .await
        .expect("list 'a/b' should succeed");
    assert_eq!(ab_resp.items().len(), 2);
    let names: Vec<Option<&str>> = ab_resp.items().iter().map(|i| i.name()).collect();
    assert!(names.contains(&Some("c")), "should include folder 'c'");
    assert!(
        names.contains(&Some("sibling.txt")),
        "should include 'sibling.txt'"
    );
}
