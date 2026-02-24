use aws_sdk_codepipeline::config::BehaviorVersion;
use aws_sdk_codepipeline::error::ProvideErrorMetadata;
use aws_sdk_codepipeline::types::{
    ActionCategory, ActionDeclaration, ActionOwner, ActionTypeId, ArtifactStore, ArtifactStoreType,
    PipelineDeclaration, StageDeclaration, Tag,
};
use winterbaume_codepipeline::CodePipelineService;
use winterbaume_core::MockAws;

const ACCOUNT_ID: &str = "123456789012";
const REGION: &str = "us-east-1";
const ROLE_ARN: &str = "arn:aws:iam::123456789012:role/test-role";

async fn make_client() -> aws_sdk_codepipeline::Client {
    let mock = MockAws::builder()
        .with_service(CodePipelineService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codepipeline::config::Region::new(REGION))
        .load()
        .await;

    aws_sdk_codepipeline::Client::new(&config)
}

fn test_pipeline(name: &str) -> PipelineDeclaration {
    PipelineDeclaration::builder()
        .name(name)
        .role_arn(ROLE_ARN)
        .stages(
            StageDeclaration::builder()
                .name("Source")
                .actions(
                    ActionDeclaration::builder()
                        .name("SourceAction")
                        .action_type_id(
                            ActionTypeId::builder()
                                .category(ActionCategory::Source)
                                .owner(ActionOwner::Aws)
                                .provider("S3")
                                .version("1")
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
}

fn full_pipeline(name: &str) -> PipelineDeclaration {
    PipelineDeclaration::builder()
        .name(name)
        .role_arn(ROLE_ARN)
        .artifact_stores(
            "us-east-1",
            ArtifactStore::builder()
                .r#type(ArtifactStoreType::S3)
                .location("codepipeline-us-east-1-123456789012")
                .build()
                .unwrap(),
        )
        .stages(
            StageDeclaration::builder()
                .name("Stage-1")
                .actions(
                    ActionDeclaration::builder()
                        .name("Action-1")
                        .action_type_id(
                            ActionTypeId::builder()
                                .category(ActionCategory::Source)
                                .owner(ActionOwner::Aws)
                                .provider("S3")
                                .version("1")
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .stages(
            StageDeclaration::builder()
                .name("Stage-2")
                .actions(
                    ActionDeclaration::builder()
                        .name("Action-1")
                        .action_type_id(
                            ActionTypeId::builder()
                                .category(ActionCategory::Approval)
                                .owner(ActionOwner::Aws)
                                .provider("Manual")
                                .version("1")
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
}

fn pipeline_arn(name: &str) -> String {
    format!("arn:aws:codepipeline:{REGION}:{ACCOUNT_ID}:{name}")
}

#[tokio::test]
async fn test_create_and_get_pipeline() {
    let client = make_client().await;

    let resp = client
        .create_pipeline()
        .pipeline(test_pipeline("my-pipeline"))
        .send()
        .await
        .expect("create_pipeline should succeed");

    let pipeline = resp.pipeline().expect("should have pipeline");
    assert_eq!(pipeline.name(), "my-pipeline");

    let get_resp = client
        .get_pipeline()
        .name("my-pipeline")
        .send()
        .await
        .expect("get_pipeline should succeed");

    let pipeline = get_resp.pipeline().expect("should have pipeline");
    assert_eq!(pipeline.name(), "my-pipeline");
}

#[tokio::test]
async fn test_list_pipelines() {
    let client = make_client().await;

    for name in ["pipe-a", "pipe-b"] {
        client
            .create_pipeline()
            .pipeline(test_pipeline(name))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(resp.pipelines().len(), 2);
}

#[tokio::test]
async fn test_delete_pipeline() {
    let client = make_client().await;

    client
        .create_pipeline()
        .pipeline(test_pipeline("delete-me"))
        .send()
        .await
        .unwrap();

    client
        .delete_pipeline()
        .name("delete-me")
        .send()
        .await
        .expect("delete should succeed");

    let result = client.get_pipeline().name("delete-me").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_pipeline() {
    let client = make_client().await;

    let result = client
        .get_pipeline()
        .name("nonexistent-pipeline")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_pipelines_empty() {
    let client = make_client().await;

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list_pipelines should succeed on empty state");

    assert_eq!(resp.pipelines().len(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_pipeline_no_error() {
    // Moto: "deleting a not existing pipeline, should raise no exception"
    let client = make_client().await;

    client
        .delete_pipeline()
        .name("nonexistent")
        .send()
        .await
        .expect("delete of nonexistent pipeline should not fail");
}

#[tokio::test]
async fn test_create_pipeline_duplicate_error() {
    let client = make_client().await;

    client
        .create_pipeline()
        .pipeline(full_pipeline("test-pipeline"))
        .send()
        .await
        .unwrap();

    let err = client
        .create_pipeline()
        .pipeline(full_pipeline("test-pipeline"))
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_invalid_structure_exception(),
        "expected InvalidStructureException, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_get_pipeline_not_found_error() {
    let client = make_client().await;

    let err = client
        .get_pipeline()
        .name("not-existing")
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_pipeline_not_found_exception(),
        "expected PipelineNotFoundException, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_update_pipeline() {
    let client = make_client().await;

    client
        .create_pipeline()
        .pipeline(full_pipeline("test-pipeline"))
        .send()
        .await
        .unwrap();

    let update_resp = client
        .update_pipeline()
        .pipeline(full_pipeline("test-pipeline"))
        .send()
        .await
        .expect("update_pipeline should succeed");

    let pipeline = update_resp.pipeline().expect("should have pipeline");
    assert_eq!(pipeline.name(), "test-pipeline");
    assert_eq!(pipeline.version(), Some(2));
}

#[tokio::test]
async fn test_update_pipeline_not_found_error() {
    let client = make_client().await;

    let err = client
        .update_pipeline()
        .pipeline(full_pipeline("not-existing"))
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert_eq!(
        svc_err.code(),
        Some("ResourceNotFoundException"),
        "expected ResourceNotFoundException, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_tag_resource() {
    let client = make_client().await;

    client
        .create_pipeline()
        .pipeline(full_pipeline("test-pipeline"))
        .send()
        .await
        .unwrap();

    client
        .tag_resource()
        .resource_arn(pipeline_arn("test-pipeline"))
        .tags(Tag::builder().key("env").value("prod").build().unwrap())
        .send()
        .await
        .expect("tag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(pipeline_arn("test-pipeline"))
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "env");
    assert_eq!(tags[0].value(), "prod");
}

#[tokio::test]
async fn test_tag_resource_not_found_error() {
    let client = make_client().await;

    let err = client
        .tag_resource()
        .resource_arn(pipeline_arn("not-existing"))
        .tags(Tag::builder().key("k").value("v").build().unwrap())
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_resource_not_found_exception(),
        "expected ResourceNotFoundException, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_list_tags_for_resource() {
    let client = make_client().await;

    // Create pipeline with tags
    client
        .create_pipeline()
        .pipeline(full_pipeline("test-pipeline"))
        .tags(Tag::builder().key("key").value("value").build().unwrap())
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(pipeline_arn("test-pipeline"))
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "key");
    assert_eq!(tags[0].value(), "value");
}

#[tokio::test]
async fn test_list_tags_for_resource_not_found_error() {
    let client = make_client().await;

    let err = client
        .list_tags_for_resource()
        .resource_arn(pipeline_arn("not-existing"))
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_resource_not_found_exception(),
        "expected ResourceNotFoundException, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    // Create pipeline with a tag
    client
        .create_pipeline()
        .pipeline(full_pipeline("test-pipeline"))
        .tags(Tag::builder().key("key").value("value").build().unwrap())
        .send()
        .await
        .unwrap();

    // Remove the tag
    client
        .untag_resource()
        .resource_arn(pipeline_arn("test-pipeline"))
        .tag_keys("key")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(pipeline_arn("test-pipeline"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tags().len(), 0);

    // Removing a nonexistent tag should not error
    client
        .untag_resource()
        .resource_arn(pipeline_arn("test-pipeline"))
        .tag_keys("key")
        .send()
        .await
        .expect("untag of nonexistent tag should not fail");
}

#[tokio::test]
async fn test_untag_resource_not_found_error() {
    let client = make_client().await;

    let err = client
        .untag_resource()
        .resource_arn(pipeline_arn("not-existing"))
        .tag_keys("key")
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_resource_not_found_exception(),
        "expected ResourceNotFoundException, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_create_pipeline_stores_tags() {
    let client = make_client().await;

    // Create pipeline with tags and then verify they can be listed
    client
        .create_pipeline()
        .pipeline(full_pipeline("tagged-pipeline"))
        .tags(Tag::builder().key("owner").value("team-a").build().unwrap())
        .tags(Tag::builder().key("env").value("staging").build().unwrap())
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(pipeline_arn("tagged-pipeline"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tags().len(), 2);
}

// ============================================================================
// Tests derived from AWS documentation: CodePipeline
// ============================================================================

#[tokio::test]
async fn test_get_pipeline_response_fields() {
    let client = make_client().await;

    client
        .create_pipeline()
        .pipeline(full_pipeline("fields-pipeline"))
        .send()
        .await
        .unwrap();

    let resp = client
        .get_pipeline()
        .name("fields-pipeline")
        .send()
        .await
        .expect("get_pipeline should succeed");

    let pipeline = resp.pipeline().expect("should have pipeline");
    assert_eq!(pipeline.name(), "fields-pipeline");
    assert_eq!(pipeline.role_arn(), ROLE_ARN);
    assert!(!pipeline.stages().is_empty(), "stages should not be empty");
    assert_eq!(pipeline.version(), Some(1), "initial version should be 1");
}

#[tokio::test]
async fn test_create_pipeline_initial_version() {
    let client = make_client().await;

    let resp = client
        .create_pipeline()
        .pipeline(test_pipeline("version-pipeline"))
        .send()
        .await
        .expect("create_pipeline should succeed");

    let pipeline = resp.pipeline().expect("should have pipeline in response");
    assert_eq!(pipeline.version(), Some(1), "initial version must be 1");
}

#[tokio::test]
async fn test_update_pipeline_multiple_times() {
    let client = make_client().await;

    client
        .create_pipeline()
        .pipeline(full_pipeline("multi-update-pipeline"))
        .send()
        .await
        .unwrap();

    // First update: version becomes 2
    let resp1 = client
        .update_pipeline()
        .pipeline(full_pipeline("multi-update-pipeline"))
        .send()
        .await
        .expect("first update should succeed");
    assert_eq!(
        resp1.pipeline().and_then(|p| p.version()),
        Some(2),
        "version after first update should be 2"
    );

    // Second update: version becomes 3
    let resp2 = client
        .update_pipeline()
        .pipeline(full_pipeline("multi-update-pipeline"))
        .send()
        .await
        .expect("second update should succeed");
    assert_eq!(
        resp2.pipeline().and_then(|p| p.version()),
        Some(3),
        "version after second update should be 3"
    );
}

#[tokio::test]
async fn test_update_pipeline_role_arn() {
    let client = make_client().await;
    const NEW_ROLE: &str = "arn:aws:iam::123456789012:role/new-role";

    client
        .create_pipeline()
        .pipeline(full_pipeline("role-update-pipeline"))
        .send()
        .await
        .unwrap();

    // Build an updated pipeline declaration with a different role ARN
    use aws_sdk_codepipeline::types::{
        ActionCategory, ActionDeclaration, ActionOwner, ActionTypeId, ArtifactStore,
        ArtifactStoreType, PipelineDeclaration, StageDeclaration,
    };
    let updated_pipeline = PipelineDeclaration::builder()
        .name("role-update-pipeline")
        .role_arn(NEW_ROLE)
        .artifact_stores(
            "us-east-1",
            ArtifactStore::builder()
                .r#type(ArtifactStoreType::S3)
                .location("codepipeline-us-east-1-123456789012")
                .build()
                .unwrap(),
        )
        .stages(
            StageDeclaration::builder()
                .name("Stage-1")
                .actions(
                    ActionDeclaration::builder()
                        .name("Action-1")
                        .action_type_id(
                            ActionTypeId::builder()
                                .category(ActionCategory::Source)
                                .owner(ActionOwner::Aws)
                                .provider("S3")
                                .version("1")
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    client
        .update_pipeline()
        .pipeline(updated_pipeline)
        .send()
        .await
        .expect("update with new role_arn should succeed");

    let get_resp = client
        .get_pipeline()
        .name("role-update-pipeline")
        .send()
        .await
        .unwrap();

    let pipeline = get_resp.pipeline().expect("should have pipeline");
    assert_eq!(
        pipeline.role_arn(),
        NEW_ROLE,
        "role_arn should be updated after UpdatePipeline"
    );
}

#[tokio::test]
async fn test_list_pipelines_contains_fields() {
    let client = make_client().await;

    client
        .create_pipeline()
        .pipeline(test_pipeline("summary-pipeline"))
        .send()
        .await
        .unwrap();

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list_pipelines should succeed");

    let pipelines = resp.pipelines();
    assert_eq!(pipelines.len(), 1);

    let summary = &pipelines[0];
    assert_eq!(
        summary.name(),
        Some("summary-pipeline"),
        "summary should have name"
    );
    assert_eq!(summary.version(), Some(1), "summary should have version 1");
    assert!(
        summary.created().is_some(),
        "summary should have created timestamp"
    );
    assert!(
        summary.updated().is_some(),
        "summary should have updated timestamp"
    );
}

#[tokio::test]
async fn test_create_pipeline_with_artifact_store() {
    let client = make_client().await;

    // full_pipeline() already uses artifactStores — verify it creates and is gettable
    let resp = client
        .create_pipeline()
        .pipeline(full_pipeline("artifact-store-pipeline"))
        .send()
        .await
        .expect("create_pipeline with artifactStores should succeed");

    let pipeline = resp.pipeline().expect("should have pipeline");
    assert_eq!(pipeline.name(), "artifact-store-pipeline");

    let get_resp = client
        .get_pipeline()
        .name("artifact-store-pipeline")
        .send()
        .await
        .expect("get_pipeline should succeed after create with artifactStores");

    assert_eq!(
        get_resp.pipeline().map(|p| p.name()),
        Some("artifact-store-pipeline")
    );
}

#[tokio::test]
async fn test_delete_pipeline_removes_from_list() {
    let client = make_client().await;

    client
        .create_pipeline()
        .pipeline(test_pipeline("delete-list-pipeline"))
        .send()
        .await
        .unwrap();

    // Verify it is in the list before deletion
    let before = client.list_pipelines().send().await.unwrap();
    assert_eq!(before.pipelines().len(), 1);

    client
        .delete_pipeline()
        .name("delete-list-pipeline")
        .send()
        .await
        .expect("delete should succeed");

    let after = client.list_pipelines().send().await.unwrap();
    assert_eq!(
        after.pipelines().len(),
        0,
        "pipeline should be absent from list after deletion"
    );
}

#[tokio::test]
async fn test_tag_resource_multiple_tags() {
    let client = make_client().await;

    client
        .create_pipeline()
        .pipeline(full_pipeline("multi-tag-pipeline"))
        .send()
        .await
        .unwrap();

    client
        .tag_resource()
        .resource_arn(pipeline_arn("multi-tag-pipeline"))
        .tags(Tag::builder().key("k1").value("v1").build().unwrap())
        .tags(Tag::builder().key("k2").value("v2").build().unwrap())
        .tags(Tag::builder().key("k3").value("v3").build().unwrap())
        .send()
        .await
        .expect("tag_resource with multiple tags should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(pipeline_arn("multi-tag-pipeline"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tags().len(), 3, "all three tags should be present");
}

#[tokio::test]
async fn test_untag_resource_multiple_keys() {
    let client = make_client().await;

    client
        .create_pipeline()
        .pipeline(full_pipeline("multi-untag-pipeline"))
        .tags(Tag::builder().key("a").value("1").build().unwrap())
        .tags(Tag::builder().key("b").value("2").build().unwrap())
        .tags(Tag::builder().key("c").value("3").build().unwrap())
        .send()
        .await
        .unwrap();

    // Remove two of the three keys
    client
        .untag_resource()
        .resource_arn(pipeline_arn("multi-untag-pipeline"))
        .tag_keys("a")
        .tag_keys("b")
        .send()
        .await
        .expect("untag_resource with multiple keys should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(pipeline_arn("multi-untag-pipeline"))
        .send()
        .await
        .unwrap();

    let tags = resp.tags();
    assert_eq!(tags.len(), 1, "only one tag should remain");
    assert_eq!(tags[0].key(), "c", "remaining tag key should be 'c'");
    assert_eq!(tags[0].value(), "3", "remaining tag value should be '3'");
}

#[tokio::test]
async fn test_lifecycle_create_update_delete() {
    let client = make_client().await;

    // Create
    client
        .create_pipeline()
        .pipeline(full_pipeline("lifecycle-pipeline"))
        .send()
        .await
        .expect("create should succeed");

    // Get — verify exists at version 1
    let get_resp = client
        .get_pipeline()
        .name("lifecycle-pipeline")
        .send()
        .await
        .expect("get after create should succeed");
    assert_eq!(get_resp.pipeline().and_then(|p| p.version()), Some(1));

    // Update — version becomes 2
    let update_resp = client
        .update_pipeline()
        .pipeline(full_pipeline("lifecycle-pipeline"))
        .send()
        .await
        .expect("update should succeed");
    assert_eq!(update_resp.pipeline().and_then(|p| p.version()), Some(2));

    // Delete
    client
        .delete_pipeline()
        .name("lifecycle-pipeline")
        .send()
        .await
        .expect("delete should succeed");

    // Get after delete — must fail with PipelineNotFoundException
    let err = client
        .get_pipeline()
        .name("lifecycle-pipeline")
        .send()
        .await
        .unwrap_err();
    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_pipeline_not_found_exception(),
        "expected PipelineNotFoundException after delete, got: {svc_err:?}"
    );
}
