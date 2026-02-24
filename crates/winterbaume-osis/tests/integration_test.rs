use aws_sdk_osis::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_osis::OsisService;

async fn make_osis_client() -> aws_sdk_osis::Client {
    let mock = MockAws::builder().with_service(OsisService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_osis::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_osis::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_get_pipeline() {
    let client = make_osis_client().await;

    let resp = client
        .create_pipeline()
        .pipeline_name("my-pipeline")
        .min_units(1)
        .max_units(4)
        .pipeline_configuration_body("version: \"2\"\nlog-pipeline:\n  source:\n    http:\n      path: \"/log/ingest\"\n  sink:\n    - opensearch:\n        hosts: [\"https://search-domain.us-east-1.es.amazonaws.com\"]\n        index: \"logs\"\n")
        .send()
        .await
        .expect("create_pipeline should succeed");

    let pipeline = resp.pipeline().expect("pipeline should be present");
    assert_eq!(pipeline.pipeline_name().unwrap(), "my-pipeline");
    assert_eq!(pipeline.min_units(), 1);
    assert_eq!(pipeline.max_units(), 4);

    let get_resp = client
        .get_pipeline()
        .pipeline_name("my-pipeline")
        .send()
        .await
        .expect("get_pipeline should succeed");

    let got = get_resp.pipeline().expect("pipeline should be present");
    assert_eq!(got.pipeline_name().unwrap(), "my-pipeline");
    assert!(got.pipeline_arn().unwrap().contains("my-pipeline"));
}

#[tokio::test]
async fn test_delete_pipeline() {
    let client = make_osis_client().await;

    client
        .create_pipeline()
        .pipeline_name("del-pipeline")
        .min_units(1)
        .max_units(1)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .unwrap();

    client
        .delete_pipeline()
        .pipeline_name("del-pipeline")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .get_pipeline()
        .pipeline_name("del-pipeline")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_list_pipelines() {
    let client = make_osis_client().await;

    client
        .create_pipeline()
        .pipeline_name("pipe-a")
        .min_units(1)
        .max_units(2)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .unwrap();

    client
        .create_pipeline()
        .pipeline_name("pipe-b")
        .min_units(1)
        .max_units(4)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list_pipelines should succeed");

    assert_eq!(resp.pipelines().len(), 2);
}

#[tokio::test]
async fn test_create_duplicate_pipeline_fails() {
    let client = make_osis_client().await;

    client
        .create_pipeline()
        .pipeline_name("dup-pipeline")
        .min_units(1)
        .max_units(1)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .unwrap();

    let result = client
        .create_pipeline()
        .pipeline_name("dup-pipeline")
        .min_units(1)
        .max_units(1)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await;
    assert!(result.is_err(), "duplicate pipeline should fail");
}

#[tokio::test]
async fn test_get_nonexistent_pipeline_fails() {
    let client = make_osis_client().await;

    let result = client
        .get_pipeline()
        .pipeline_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "get nonexistent pipeline should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_pipeline_fails() {
    let client = make_osis_client().await;

    let result = client
        .delete_pipeline()
        .pipeline_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent pipeline should fail");
}

#[tokio::test]
async fn test_list_pipelines_empty() {
    let client = make_osis_client().await;

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list_pipelines on empty should succeed");

    assert_eq!(resp.pipelines().len(), 0);
}

#[tokio::test]
async fn test_start_pipeline() {
    let client = make_osis_client().await;

    client
        .create_pipeline()
        .pipeline_name("start-pipe")
        .min_units(1)
        .max_units(2)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .unwrap();

    // Stop it first
    client
        .stop_pipeline()
        .pipeline_name("start-pipe")
        .send()
        .await
        .expect("stop_pipeline should succeed");

    // Now start it
    let resp = client
        .start_pipeline()
        .pipeline_name("start-pipe")
        .send()
        .await
        .expect("start_pipeline should succeed");

    let pipeline = resp.pipeline().expect("pipeline should be present");
    assert_eq!(pipeline.status().unwrap().as_str(), "ACTIVE");
}

#[tokio::test]
async fn test_stop_pipeline() {
    let client = make_osis_client().await;

    client
        .create_pipeline()
        .pipeline_name("stop-pipe")
        .min_units(1)
        .max_units(2)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .unwrap();

    let resp = client
        .stop_pipeline()
        .pipeline_name("stop-pipe")
        .send()
        .await
        .expect("stop_pipeline should succeed");

    let pipeline = resp.pipeline().expect("pipeline should be present");
    assert_eq!(pipeline.status().unwrap().as_str(), "STOPPED");
}

#[tokio::test]
async fn test_stop_nonexistent_pipeline_fails() {
    let client = make_osis_client().await;

    let result = client
        .stop_pipeline()
        .pipeline_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "stop nonexistent pipeline should fail");
}

#[tokio::test]
async fn test_update_pipeline() {
    let client = make_osis_client().await;

    client
        .create_pipeline()
        .pipeline_name("update-pipe")
        .min_units(1)
        .max_units(2)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_pipeline()
        .pipeline_name("update-pipe")
        .min_units(2)
        .max_units(8)
        .pipeline_configuration_body("version: \"2\"\nlog-pipeline:\n  source:\n    http:\n      path: \"/updated\"\n  sink:\n    - opensearch:\n        hosts: [\"https://search.example.com\"]\n        index: \"logs\"\n")
        .send()
        .await
        .expect("update_pipeline should succeed");

    let pipeline = resp.pipeline().expect("pipeline should be present");
    assert_eq!(pipeline.min_units(), 2);
    assert_eq!(pipeline.max_units(), 8);
    assert!(
        pipeline
            .pipeline_configuration_body()
            .unwrap()
            .contains("/updated")
    );
}

#[tokio::test]
async fn test_update_nonexistent_pipeline_fails() {
    let client = make_osis_client().await;

    let result = client
        .update_pipeline()
        .pipeline_name("nonexistent")
        .min_units(1)
        .max_units(2)
        .send()
        .await;
    assert!(result.is_err(), "update nonexistent pipeline should fail");
}

#[tokio::test]
async fn test_tag_and_list_tags() {
    let client = make_osis_client().await;

    let create_resp = client
        .create_pipeline()
        .pipeline_name("tag-pipe")
        .min_units(1)
        .max_units(1)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .unwrap();

    let arn = create_resp
        .pipeline()
        .unwrap()
        .pipeline_arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .arn(&arn)
        .tags(
            aws_sdk_osis::types::Tag::builder()
                .key("env")
                .value("prod")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_osis::types::Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags();
    assert_eq!(tags.len(), 2);

    let tag_map: std::collections::HashMap<&str, &str> =
        tags.iter().map(|t| (t.key(), t.value())).collect();
    assert_eq!(tag_map.get("env"), Some(&"prod"));
    assert_eq!(tag_map.get("team"), Some(&"platform"));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_osis_client().await;

    let create_resp = client
        .create_pipeline()
        .pipeline_name("untag-pipe")
        .min_units(1)
        .max_units(1)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .unwrap();

    let arn = create_resp
        .pipeline()
        .unwrap()
        .pipeline_arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .arn(&arn)
        .tags(
            aws_sdk_osis::types::Tag::builder()
                .key("env")
                .value("prod")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_osis::types::Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .arn(&arn)
        .tag_keys("team")
        .send()
        .await
        .expect("untag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = list_resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "env");
    assert_eq!(tags[0].value(), "prod");
}

#[tokio::test]
async fn test_list_tags_nonexistent_resource_fails() {
    let client = make_osis_client().await;

    let result = client
        .list_tags_for_resource()
        .arn("arn:aws:osis:us-east-1:123456789012:pipeline/nonexistent")
        .send()
        .await;
    assert!(
        result.is_err(),
        "list_tags for nonexistent resource should fail"
    );
}

// ============================================================================
// Tests derived from AWS documentation: Amazon OpenSearch Ingestion (OSIS)
// ============================================================================

#[tokio::test]
async fn test_create_pipeline_response_fields() {
    let client = make_osis_client().await;

    let resp = client
        .create_pipeline()
        .pipeline_name("resp-fields-pipe")
        .min_units(1)
        .max_units(2)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .expect("create_pipeline should succeed");

    let pipeline = resp.pipeline().expect("pipeline should be present");
    assert!(
        !pipeline.pipeline_arn().unwrap_or_default().is_empty(),
        "pipeline_arn should be non-empty"
    );
    assert!(
        !pipeline.pipeline_name().unwrap_or_default().is_empty(),
        "pipeline_name should be non-empty"
    );
    assert!(
        !pipeline
            .status()
            .map(|s| s.as_str())
            .unwrap_or_default()
            .is_empty(),
        "status should be non-empty"
    );
    assert!(
        pipeline.created_at().is_some(),
        "created_at should be present"
    );
    assert!(
        pipeline.last_updated_at().is_some(),
        "last_updated_at should be present"
    );
    assert!(
        !pipeline.ingest_endpoint_urls().is_empty(),
        "ingest_endpoint_urls should be non-empty"
    );
}

#[tokio::test]
async fn test_create_pipeline_arn_format() {
    let client = make_osis_client().await;

    let resp = client
        .create_pipeline()
        .pipeline_name("arn-fmt-pipe")
        .min_units(1)
        .max_units(1)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .expect("create_pipeline should succeed");

    let pipeline = resp.pipeline().expect("pipeline should be present");
    let arn = pipeline.pipeline_arn().unwrap_or_default();
    assert!(
        arn.starts_with("arn:aws:osis:"),
        "ARN should start with arn:aws:osis:, got: {arn}"
    );
    assert!(
        arn.contains(":pipeline/arn-fmt-pipe"),
        "ARN should contain :pipeline/<name>, got: {arn}"
    );
}

#[tokio::test]
async fn test_create_pipeline_initial_status() {
    let client = make_osis_client().await;

    let resp = client
        .create_pipeline()
        .pipeline_name("status-pipe")
        .min_units(1)
        .max_units(1)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .expect("create_pipeline should succeed");

    let pipeline = resp.pipeline().expect("pipeline should be present");
    assert_eq!(
        pipeline.status().map(|s| s.as_str()).unwrap_or_default(),
        "ACTIVE",
        "new pipeline should have ACTIVE status"
    );
}

#[tokio::test]
async fn test_create_pipeline_ingest_endpoint_urls() {
    let client = make_osis_client().await;

    let resp = client
        .create_pipeline()
        .pipeline_name("ingest-url-pipe")
        .min_units(1)
        .max_units(1)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .expect("create_pipeline should succeed");

    let pipeline = resp.pipeline().expect("pipeline should be present");
    let urls = pipeline.ingest_endpoint_urls();
    assert!(!urls.is_empty(), "ingest_endpoint_urls should be non-empty");
    assert!(
        urls[0].contains("ingest-url-pipe"),
        "ingest endpoint URL should contain pipeline name, got: {}",
        urls[0]
    );
}

#[tokio::test]
async fn test_get_pipeline_fields_match_create() {
    let client = make_osis_client().await;

    let create_resp = client
        .create_pipeline()
        .pipeline_name("match-fields-pipe")
        .min_units(2)
        .max_units(5)
        .pipeline_configuration_body(
            "version: \"2\"\nmy-pipeline:\n  source:\n    http:\n      path: \"/test\"\n",
        )
        .send()
        .await
        .expect("create_pipeline should succeed");

    let created = create_resp.pipeline().expect("pipeline should be present");
    let create_arn = created.pipeline_arn().unwrap_or_default().to_string();

    let get_resp = client
        .get_pipeline()
        .pipeline_name("match-fields-pipe")
        .send()
        .await
        .expect("get_pipeline should succeed");

    let got = get_resp.pipeline().expect("pipeline should be present");
    assert_eq!(
        got.pipeline_name(),
        created.pipeline_name(),
        "pipeline_name should match"
    );
    assert_eq!(
        got.pipeline_arn().unwrap_or_default(),
        create_arn,
        "pipeline_arn should match"
    );
    assert_eq!(
        got.min_units(),
        created.min_units(),
        "min_units should match"
    );
    assert_eq!(
        got.max_units(),
        created.max_units(),
        "max_units should match"
    );
    assert_eq!(
        got.status().map(|s| s.as_str()),
        created.status().map(|s| s.as_str()),
        "status should match"
    );
    assert_eq!(
        got.pipeline_configuration_body(),
        created.pipeline_configuration_body(),
        "pipeline_configuration_body should match"
    );
}

#[tokio::test]
async fn test_list_pipelines_summary_fields() {
    let client = make_osis_client().await;

    client
        .create_pipeline()
        .pipeline_name("summary-pipe")
        .min_units(2)
        .max_units(4)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list_pipelines should succeed");

    let pipelines = resp.pipelines();
    let summary = pipelines
        .iter()
        .find(|p| p.pipeline_name().unwrap_or_default() == "summary-pipe")
        .expect("summary-pipe should be in list");

    assert!(
        !summary.pipeline_arn().unwrap_or_default().is_empty(),
        "summary pipeline_arn should be non-empty"
    );
    assert!(
        !summary
            .status()
            .map(|s| s.as_str())
            .unwrap_or_default()
            .is_empty(),
        "summary status should be non-empty"
    );
    assert_eq!(
        summary.min_units(),
        Some(2),
        "summary min_units should be 2"
    );
    assert_eq!(
        summary.max_units(),
        Some(4),
        "summary max_units should be 4"
    );
    assert!(
        summary.created_at().is_some(),
        "summary created_at should be present"
    );
    assert!(
        summary.last_updated_at().is_some(),
        "summary last_updated_at should be present"
    );
}

#[tokio::test]
async fn test_start_nonexistent_pipeline_fails() {
    let client = make_osis_client().await;

    let result = client
        .start_pipeline()
        .pipeline_name("nonexistent-start")
        .send()
        .await;

    assert!(result.is_err(), "start nonexistent pipeline should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_untag_nonexistent_resource_fails() {
    let client = make_osis_client().await;

    let result = client
        .untag_resource()
        .arn("arn:aws:osis:us-east-1:123456789012:pipeline/no-such-pipe")
        .tag_keys("env")
        .send()
        .await;

    assert!(result.is_err(), "untag nonexistent resource should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_pipeline_with_tags() {
    let client = make_osis_client().await;

    let create_resp = client
        .create_pipeline()
        .pipeline_name("tagged-at-create-pipe")
        .min_units(1)
        .max_units(1)
        .pipeline_configuration_body("version: \"2\"\n")
        .tags(
            aws_sdk_osis::types::Tag::builder()
                .key("project")
                .value("testing")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_pipeline with tags should succeed");

    let pipeline = create_resp.pipeline().expect("pipeline should be present");
    let arn = pipeline.pipeline_arn().unwrap_or_default().to_string();

    // Tags set at creation time should be retrievable via ListTagsForResource
    let list_resp = client
        .list_tags_for_resource()
        .arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags();
    let tag_map: std::collections::HashMap<&str, &str> =
        tags.iter().map(|t| (t.key(), t.value())).collect();
    assert_eq!(
        tag_map.get("project"),
        Some(&"testing"),
        "tag set at creation time should be present"
    );
}

#[tokio::test]
async fn test_full_pipeline_lifecycle() {
    let client = make_osis_client().await;

    // Create
    let create_resp = client
        .create_pipeline()
        .pipeline_name("lifecycle-pipe")
        .min_units(1)
        .max_units(2)
        .pipeline_configuration_body("version: \"2\"\n")
        .send()
        .await
        .expect("create_pipeline should succeed");
    let pipeline = create_resp.pipeline().expect("pipeline should be present");
    assert_eq!(
        pipeline.pipeline_name().unwrap_or_default(),
        "lifecycle-pipe"
    );
    assert_eq!(
        pipeline.status().map(|s| s.as_str()).unwrap_or_default(),
        "ACTIVE"
    );

    // Get
    let get_resp = client
        .get_pipeline()
        .pipeline_name("lifecycle-pipe")
        .send()
        .await
        .expect("get_pipeline should succeed");
    let got = get_resp.pipeline().expect("pipeline should be present");
    assert_eq!(got.pipeline_name().unwrap_or_default(), "lifecycle-pipe");

    // Update
    let update_resp = client
        .update_pipeline()
        .pipeline_name("lifecycle-pipe")
        .min_units(2)
        .max_units(4)
        .send()
        .await
        .expect("update_pipeline should succeed");
    let updated = update_resp.pipeline().expect("pipeline should be present");
    assert_eq!(updated.min_units(), 2);
    assert_eq!(updated.max_units(), 4);

    // Stop
    let stop_resp = client
        .stop_pipeline()
        .pipeline_name("lifecycle-pipe")
        .send()
        .await
        .expect("stop_pipeline should succeed");
    let stopped = stop_resp.pipeline().expect("pipeline should be present");
    assert_eq!(
        stopped.status().map(|s| s.as_str()).unwrap_or_default(),
        "STOPPED"
    );

    // Start
    let start_resp = client
        .start_pipeline()
        .pipeline_name("lifecycle-pipe")
        .send()
        .await
        .expect("start_pipeline should succeed");
    let started = start_resp.pipeline().expect("pipeline should be present");
    assert_eq!(
        started.status().map(|s| s.as_str()).unwrap_or_default(),
        "ACTIVE"
    );

    // Delete
    client
        .delete_pipeline()
        .pipeline_name("lifecycle-pipe")
        .send()
        .await
        .expect("delete_pipeline should succeed");

    // Verify gone
    let result = client
        .get_pipeline()
        .pipeline_name("lifecycle-pipe")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "expected ResourceNotFoundException after delete, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_pipeline_get_reflects_changes() {
    let client = make_osis_client().await;

    client
        .create_pipeline()
        .pipeline_name("update-reflect-pipe")
        .min_units(1)
        .max_units(2)
        .pipeline_configuration_body("version: \"2\"\noriginal: true\n")
        .send()
        .await
        .unwrap();

    client
        .update_pipeline()
        .pipeline_name("update-reflect-pipe")
        .min_units(3)
        .max_units(6)
        .pipeline_configuration_body("version: \"2\"\nupdated: true\n")
        .send()
        .await
        .expect("update_pipeline should succeed");

    let get_resp = client
        .get_pipeline()
        .pipeline_name("update-reflect-pipe")
        .send()
        .await
        .expect("get_pipeline after update should succeed");

    let got = get_resp.pipeline().expect("pipeline should be present");
    assert_eq!(got.min_units(), 3, "min_units should be updated");
    assert_eq!(got.max_units(), 6, "max_units should be updated");
    assert!(
        got.pipeline_configuration_body()
            .unwrap_or_default()
            .contains("updated: true"),
        "pipeline_configuration_body should be updated"
    );
}
