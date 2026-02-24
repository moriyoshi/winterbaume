use aws_sdk_clouddirectory::config::BehaviorVersion;
use winterbaume_clouddirectory::CloudDirectoryService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_clouddirectory::Client {
    let mock = MockAws::builder()
        .with_service(CloudDirectoryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_clouddirectory::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_clouddirectory::Client::new(&config)
}

// ===================== Schema Tests =====================

#[tokio::test]
async fn test_create_and_delete_schema() {
    let client = make_client().await;

    let resp = client
        .create_schema()
        .name("TestSchema")
        .send()
        .await
        .expect("create_schema should succeed");

    let schema_arn = resp.schema_arn().expect("should have schema_arn");
    assert!(schema_arn.contains("schema/development/TestSchema"));

    client
        .delete_schema()
        .schema_arn(schema_arn)
        .send()
        .await
        .expect("delete_schema should succeed");
}

#[tokio::test]
async fn test_publish_schema() {
    let client = make_client().await;

    let create_resp = client
        .create_schema()
        .name("PublishTestSchema")
        .send()
        .await
        .expect("create_schema should succeed");

    let dev_arn = create_resp.schema_arn().unwrap().to_string();

    let publish_resp = client
        .publish_schema()
        .development_schema_arn(&dev_arn)
        .version("1")
        .send()
        .await
        .expect("publish_schema should succeed");

    let published_arn = publish_resp
        .published_schema_arn()
        .expect("should have published_schema_arn");
    assert!(published_arn.contains("schema/published/PublishTestSchema/1"));
}

#[tokio::test]
async fn test_list_development_schema_arns() {
    let client = make_client().await;

    client
        .create_schema()
        .name("DevSchema1")
        .send()
        .await
        .expect("create_schema should succeed");

    client
        .create_schema()
        .name("DevSchema2")
        .send()
        .await
        .expect("create_schema should succeed");

    let resp = client
        .list_development_schema_arns()
        .send()
        .await
        .expect("list_development_schema_arns should succeed");

    let arns = resp.schema_arns();
    assert!(arns.len() >= 2, "Should have at least 2 dev schema ARNs");
}

#[tokio::test]
async fn test_list_published_schema_arns() {
    let client = make_client().await;

    let create_resp = client
        .create_schema()
        .name("ToPublishSchema")
        .send()
        .await
        .unwrap();

    let dev_arn = create_resp.schema_arn().unwrap().to_string();

    client
        .publish_schema()
        .development_schema_arn(&dev_arn)
        .version("2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_published_schema_arns()
        .send()
        .await
        .expect("list_published_schema_arns should succeed");

    let arns = resp.schema_arns();
    assert!(
        !arns.is_empty(),
        "Should have at least 1 published schema ARN"
    );
}

// ===================== Directory Tests =====================

#[tokio::test]
async fn test_create_and_get_directory() {
    let client = make_client().await;

    // First create a schema
    let schema_resp = client
        .create_schema()
        .name("DirTestSchema")
        .send()
        .await
        .unwrap();
    let schema_arn = schema_resp.schema_arn().unwrap().to_string();

    // Create directory
    let create_resp = client
        .create_directory()
        .name("TestDirectory")
        .schema_arn(&schema_arn)
        .send()
        .await
        .expect("create_directory should succeed");

    let dir_arn = create_resp.directory_arn();
    assert!(!dir_arn.is_empty());
    assert_eq!(create_resp.name(), "TestDirectory");

    // Get directory
    let get_resp = client
        .get_directory()
        .directory_arn(dir_arn)
        .send()
        .await
        .expect("get_directory should succeed");

    let dir = get_resp.directory().expect("should have directory");
    assert_eq!(dir.name().unwrap_or_default(), "TestDirectory");
    assert_eq!(dir.state().unwrap().as_str(), "ENABLED");
}

#[tokio::test]
async fn test_list_directories() {
    let client = make_client().await;

    let schema_resp = client
        .create_schema()
        .name("ListDirSchema")
        .send()
        .await
        .unwrap();
    let schema_arn = schema_resp.schema_arn().unwrap().to_string();

    client
        .create_directory()
        .name("ListDir1")
        .schema_arn(&schema_arn)
        .send()
        .await
        .unwrap();

    client
        .create_directory()
        .name("ListDir2")
        .schema_arn(&schema_arn)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_directories()
        .send()
        .await
        .expect("list_directories should succeed");

    let dirs = resp.directories();
    assert!(dirs.len() >= 2, "Should have at least 2 directories");
}

// ===================== Tag Tests =====================

#[tokio::test]
async fn test_tag_and_untag_resource() {
    let client = make_client().await;

    let schema_resp = client
        .create_schema()
        .name("TagTestSchema")
        .send()
        .await
        .unwrap();
    let schema_arn = schema_resp.schema_arn().unwrap().to_string();

    let create_resp = client
        .create_directory()
        .name("TagTestDir")
        .schema_arn(&schema_arn)
        .send()
        .await
        .unwrap();
    let dir_arn = create_resp.directory_arn().to_string();

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&dir_arn)
        .tags(
            aws_sdk_clouddirectory::types::Tag::builder()
                .key("env")
                .value("test")
                .build(),
        )
        .tags(
            aws_sdk_clouddirectory::types::Tag::builder()
                .key("team")
                .value("infra")
                .build(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&dir_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags();
    assert_eq!(tags.len(), 2);

    // Untag one
    client
        .untag_resource()
        .resource_arn(&dir_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify tag removed
    let list_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&dir_arn)
        .send()
        .await
        .unwrap();

    let tags2 = list_resp2.tags();
    assert_eq!(tags2.len(), 1);
    assert_eq!(tags2[0].key().unwrap(), "team");
}

// ===================== Schema Application Tests =====================

#[tokio::test]
async fn test_apply_schema() {
    let client = make_client().await;

    // Create dev schema and publish it
    let schema_resp = client
        .create_schema()
        .name("ApplyTestSchema")
        .send()
        .await
        .unwrap();
    let dev_arn = schema_resp.schema_arn().unwrap().to_string();

    let publish_resp = client
        .publish_schema()
        .development_schema_arn(&dev_arn)
        .version("1")
        .send()
        .await
        .unwrap();
    let published_arn = publish_resp.published_schema_arn().unwrap().to_string();

    // Create directory with dev schema
    let create_resp = client
        .create_directory()
        .name("ApplyDir")
        .schema_arn(&dev_arn)
        .send()
        .await
        .unwrap();
    let dir_arn = create_resp.directory_arn().to_string();

    // Apply the published schema
    let apply_resp = client
        .apply_schema()
        .directory_arn(&dir_arn)
        .published_schema_arn(&published_arn)
        .send()
        .await
        .expect("apply_schema should succeed");

    assert_eq!(apply_resp.directory_arn().unwrap(), &dir_arn);
    assert_eq!(apply_resp.applied_schema_arn().unwrap(), &published_arn);
}
