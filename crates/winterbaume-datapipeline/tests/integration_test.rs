use aws_sdk_datapipeline::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_datapipeline::DataPipelineService;

async fn make_datapipeline_client() -> aws_sdk_datapipeline::Client {
    let mock = MockAws::builder()
        .with_service(DataPipelineService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_datapipeline::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_datapipeline::Client::new(&config)
}

#[tokio::test]
async fn test_create_pipeline() {
    let client = make_datapipeline_client().await;

    let resp = client
        .create_pipeline()
        .name("test-pipeline")
        .unique_id("test-unique-1")
        .send()
        .await
        .expect("create_pipeline should succeed");

    assert!(!resp.pipeline_id().is_empty());
}

#[tokio::test]
async fn test_list_pipelines() {
    let client = make_datapipeline_client().await;

    client
        .create_pipeline()
        .name("pipeline-a")
        .unique_id("uid-a")
        .send()
        .await
        .unwrap();

    client
        .create_pipeline()
        .name("pipeline-b")
        .unique_id("uid-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list_pipelines should succeed");

    assert_eq!(resp.pipeline_id_list().len(), 2);
}

#[tokio::test]
async fn test_describe_pipelines() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("desc-pipeline")
        .unique_id("uid-desc")
        .description("A test pipeline")
        .send()
        .await
        .unwrap();

    let pipeline_id = create_resp.pipeline_id().to_string();

    let resp = client
        .describe_pipelines()
        .pipeline_ids(&pipeline_id)
        .send()
        .await
        .expect("describe_pipelines should succeed");

    let descriptions = resp.pipeline_description_list();
    assert_eq!(descriptions.len(), 1);
    assert_eq!(descriptions[0].pipeline_id(), pipeline_id);
    assert_eq!(descriptions[0].name(), "desc-pipeline");
}

#[tokio::test]
async fn test_delete_pipeline() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("del-pipeline")
        .unique_id("uid-del")
        .send()
        .await
        .unwrap();

    let pipeline_id = create_resp.pipeline_id().to_string();

    client
        .delete_pipeline()
        .pipeline_id(&pipeline_id)
        .send()
        .await
        .expect("delete_pipeline should succeed");

    // Describing the deleted pipeline should fail
    let result = client
        .describe_pipelines()
        .pipeline_ids(&pipeline_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_and_get_pipeline_definition() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("def-pipeline")
        .unique_id("uid-def")
        .send()
        .await
        .unwrap();

    let pipeline_id = create_resp.pipeline_id().to_string();

    let field = aws_sdk_datapipeline::types::Field::builder()
        .key("type")
        .string_value("Schedule")
        .build()
        .unwrap();

    let pipeline_object = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("Schedule")
        .name("my-schedule")
        .fields(field)
        .build()
        .unwrap();

    let put_resp = client
        .put_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .pipeline_objects(pipeline_object)
        .send()
        .await
        .expect("put_pipeline_definition should succeed");

    assert!(!put_resp.errored());

    let get_resp = client
        .get_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .send()
        .await
        .expect("get_pipeline_definition should succeed");

    let objects = get_resp.pipeline_objects();
    assert_eq!(objects.len(), 1);
    assert_eq!(objects[0].id(), "Schedule");
    assert_eq!(objects[0].name(), "my-schedule");
}

#[tokio::test]
async fn test_delete_nonexistent_pipeline_fails() {
    let client = make_datapipeline_client().await;

    let result = client
        .delete_pipeline()
        .pipeline_id("df-nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_creating_pipeline_definition_multiple_objects() {
    // Port of moto test_creating_pipeline_definition
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("mypipeline")
        .unique_id("some-unique-id-multi")
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    // Build 3 pipeline objects like the moto test PIPELINE_OBJECTS
    let default_obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("Default")
        .name("Default")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("workerGroup")
                .string_value("workerGroup")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let schedule_obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("Schedule")
        .name("Schedule")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("startDateTime")
                .string_value("2012-12-12T00:00:00")
                .build()
                .unwrap(),
        )
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("type")
                .string_value("Schedule")
                .build()
                .unwrap(),
        )
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("period")
                .string_value("1 hour")
                .build()
                .unwrap(),
        )
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("endDateTime")
                .string_value("2012-12-21T18:00:00")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let say_hello_obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("SayHello")
        .name("SayHello")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("type")
                .string_value("ShellCommandActivity")
                .build()
                .unwrap(),
        )
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("command")
                .string_value("echo hello")
                .build()
                .unwrap(),
        )
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("parent")
                .ref_value("Default")
                .build()
                .unwrap(),
        )
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("schedule")
                .ref_value("Schedule")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    client
        .put_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .pipeline_objects(default_obj)
        .pipeline_objects(schedule_obj)
        .pipeline_objects(say_hello_obj)
        .send()
        .await
        .expect("put_pipeline_definition should succeed");

    let get_resp = client
        .get_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .send()
        .await
        .expect("get_pipeline_definition should succeed");

    let objects = get_resp.pipeline_objects();
    assert_eq!(objects.len(), 3);

    // Find the "Default" object and verify it
    let default_object = objects.iter().find(|o| o.id() == "Default").unwrap();
    assert_eq!(default_object.name(), "Default");
    assert_eq!(default_object.fields().len(), 1);
    assert_eq!(default_object.fields()[0].key(), "workerGroup");
    assert_eq!(
        default_object.fields()[0].string_value(),
        Some("workerGroup")
    );
}

#[tokio::test]
async fn test_listing_pipelines_with_fields() {
    // Port of moto test_listing_pipelines - verify hasMoreResults and pipeline id/name
    let client = make_datapipeline_client().await;

    let res1 = client
        .create_pipeline()
        .name("mypipeline1")
        .unique_id("some-unique-id1-list")
        .send()
        .await
        .unwrap();
    let res2 = client
        .create_pipeline()
        .name("mypipeline2")
        .unique_id("some-unique-id2-list")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list_pipelines should succeed");

    assert!(!resp.has_more_results());
    let objects = resp.pipeline_id_list();
    assert_eq!(objects.len(), 2);

    let ids: Vec<&str> = objects.iter().map(|p| p.id().unwrap_or("")).collect();
    let names: Vec<&str> = objects.iter().map(|p| p.name().unwrap_or("")).collect();
    assert!(ids.contains(&res1.pipeline_id()));
    assert!(ids.contains(&res2.pipeline_id()));
    assert!(names.contains(&"mypipeline1"));
    assert!(names.contains(&"mypipeline2"));
}

#[tokio::test]
async fn test_pipeline_definition_with_ref_values() {
    // Port of checking refValue fields are stored/returned correctly
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("ref-pipeline")
        .unique_id("uid-ref-val")
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    let obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("SayHello")
        .name("SayHello")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("parent")
                .ref_value("Default")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    client
        .put_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .pipeline_objects(obj)
        .send()
        .await
        .unwrap();

    let get_resp = client
        .get_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .send()
        .await
        .unwrap();

    let objects = get_resp.pipeline_objects();
    assert_eq!(objects.len(), 1);
    assert_eq!(objects[0].fields()[0].key(), "parent");
    assert_eq!(objects[0].fields()[0].ref_value(), Some("Default"));
    assert_eq!(objects[0].fields()[0].string_value(), None);
}

#[tokio::test]
async fn test_describe_pipeline_fields() {
    // Port of moto test_create_pipeline - verify fields in description
    let client = make_datapipeline_client().await;

    let res = client
        .create_pipeline()
        .name("mypipeline")
        .unique_id("some-unique-id-fields")
        .send()
        .await
        .unwrap();
    let pipeline_id = res.pipeline_id().to_string();

    let resp = client
        .describe_pipelines()
        .pipeline_ids(&pipeline_id)
        .send()
        .await
        .unwrap();

    let descriptions = resp.pipeline_description_list();
    assert_eq!(descriptions.len(), 1);
    let desc = &descriptions[0];
    assert_eq!(desc.name(), "mypipeline");
    assert_eq!(desc.pipeline_id(), pipeline_id);

    let fields = desc.fields();
    // Verify key fields are present in the fields list
    let pipeline_state = fields.iter().find(|f| f.key() == "@pipelineState");
    assert!(pipeline_state.is_some());
    assert_eq!(pipeline_state.unwrap().string_value(), Some("PENDING"));

    let unique_id_field = fields.iter().find(|f| f.key() == "uniqueId");
    assert!(unique_id_field.is_some());
    assert_eq!(
        unique_id_field.unwrap().string_value(),
        Some("some-unique-id-fields")
    );
}

#[tokio::test]
async fn test_create_pipeline_duplicate_unique_id_returns_error() {
    let client = make_datapipeline_client().await;

    client
        .create_pipeline()
        .name("pipeline-orig")
        .unique_id("dup-uid-1")
        .send()
        .await
        .expect("first create should succeed");

    // A second create with the same unique_id must fail with InvalidRequestException
    let result = client
        .create_pipeline()
        .name("pipeline-dup")
        .unique_id("dup-uid-1")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_pipeline_with_tags_visible_in_describe() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("tagged-pipeline")
        .unique_id("uid-tagged")
        .tags(
            aws_sdk_datapipeline::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_datapipeline::types::Tag::builder()
                .key("team")
                .value("data")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_pipeline with tags should succeed");

    let pipeline_id = create_resp.pipeline_id().to_string();

    let desc_resp = client
        .describe_pipelines()
        .pipeline_ids(&pipeline_id)
        .send()
        .await
        .expect("describe_pipelines should succeed");

    let descriptions = desc_resp.pipeline_description_list();
    assert_eq!(descriptions.len(), 1);
    let tags = descriptions[0].tags();
    assert_eq!(tags.len(), 2);
    let env_tag = tags
        .iter()
        .find(|t| t.key() == "env")
        .expect("env tag missing");
    assert_eq!(env_tag.value(), "test");
    let team_tag = tags
        .iter()
        .find(|t| t.key() == "team")
        .expect("team tag missing");
    assert_eq!(team_tag.value(), "data");
}

#[tokio::test]
async fn test_describe_pipelines_multiple_ids() {
    let client = make_datapipeline_client().await;

    let id1 = client
        .create_pipeline()
        .name("multi-a")
        .unique_id("uid-multi-a")
        .send()
        .await
        .unwrap()
        .pipeline_id()
        .to_string();

    let id2 = client
        .create_pipeline()
        .name("multi-b")
        .unique_id("uid-multi-b")
        .send()
        .await
        .unwrap()
        .pipeline_id()
        .to_string();

    let resp = client
        .describe_pipelines()
        .pipeline_ids(&id1)
        .pipeline_ids(&id2)
        .send()
        .await
        .expect("describe_pipelines with two ids should succeed");

    let descriptions = resp.pipeline_description_list();
    assert_eq!(descriptions.len(), 2);

    let returned_ids: Vec<&str> = descriptions.iter().map(|d| d.pipeline_id()).collect();
    assert!(returned_ids.contains(&id1.as_str()));
    assert!(returned_ids.contains(&id2.as_str()));
}

#[tokio::test]
async fn test_describe_pipelines_nonexistent_fails() {
    let client = make_datapipeline_client().await;

    let result = client
        .describe_pipelines()
        .pipeline_ids("df-doesnotexist")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_pipelines_empty() {
    let client = make_datapipeline_client().await;

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list_pipelines on empty store should succeed");

    assert_eq!(resp.pipeline_id_list().len(), 0);
    assert!(!resp.has_more_results());
}

#[tokio::test]
async fn test_get_pipeline_definition_nonexistent_fails() {
    let client = make_datapipeline_client().await;

    let result = client
        .get_pipeline_definition()
        .pipeline_id("df-nonexistent-def")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_pipeline_definition_empty_before_put() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("empty-def-pipeline")
        .unique_id("uid-empty-def")
        .send()
        .await
        .unwrap();

    let pipeline_id = create_resp.pipeline_id().to_string();

    let get_resp = client
        .get_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .send()
        .await
        .expect("get_pipeline_definition on a fresh pipeline should succeed");

    // No objects have been put yet
    assert_eq!(get_resp.pipeline_objects().len(), 0);
}

#[tokio::test]
async fn test_put_pipeline_definition_nonexistent_fails() {
    let client = make_datapipeline_client().await;

    let field = aws_sdk_datapipeline::types::Field::builder()
        .key("type")
        .string_value("Schedule")
        .build()
        .unwrap();

    let obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("Default")
        .name("Default")
        .fields(field)
        .build()
        .unwrap();

    let result = client
        .put_pipeline_definition()
        .pipeline_id("df-nonexistent-put")
        .pipeline_objects(obj)
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_pipeline_definition_overwrites_previous() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("overwrite-pipeline")
        .unique_id("uid-overwrite")
        .send()
        .await
        .unwrap();

    let pipeline_id = create_resp.pipeline_id().to_string();

    // First put: one object
    let first_obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("ObjA")
        .name("ObjectA")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("type")
                .string_value("ShellCommandActivity")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    client
        .put_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .pipeline_objects(first_obj)
        .send()
        .await
        .expect("first put should succeed");

    // Second put: different object, should replace entirely
    let second_obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("ObjB")
        .name("ObjectB")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("type")
                .string_value("Schedule")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    client
        .put_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .pipeline_objects(second_obj)
        .send()
        .await
        .expect("second put should succeed");

    let get_resp = client
        .get_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .send()
        .await
        .expect("get after overwrite should succeed");

    let objects = get_resp.pipeline_objects();
    assert_eq!(objects.len(), 1);
    assert_eq!(objects[0].id(), "ObjB");
}

#[tokio::test]
async fn test_describe_pipeline_with_description_field() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("described-pipeline")
        .unique_id("uid-with-desc")
        .description("my pipeline description")
        .send()
        .await
        .unwrap();

    let pipeline_id = create_resp.pipeline_id().to_string();

    let resp = client
        .describe_pipelines()
        .pipeline_ids(&pipeline_id)
        .send()
        .await
        .unwrap();

    let desc = &resp.pipeline_description_list()[0];
    assert_eq!(desc.description(), Some("my pipeline description"));

    let description_field = desc.fields().iter().find(|f| f.key() == "description");
    assert!(description_field.is_some());
    assert_eq!(
        description_field.unwrap().string_value(),
        Some("my pipeline description")
    );
}

#[tokio::test]
async fn test_list_pipelines_after_delete_reflects_removal() {
    let client = make_datapipeline_client().await;

    let id1 = client
        .create_pipeline()
        .name("del-list-a")
        .unique_id("uid-del-list-a")
        .send()
        .await
        .unwrap()
        .pipeline_id()
        .to_string();

    client
        .create_pipeline()
        .name("del-list-b")
        .unique_id("uid-del-list-b")
        .send()
        .await
        .unwrap();

    client
        .delete_pipeline()
        .pipeline_id(&id1)
        .send()
        .await
        .expect("delete should succeed");

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list after delete should succeed");

    let listed_ids: Vec<&str> = resp
        .pipeline_id_list()
        .iter()
        .map(|p| p.id().unwrap_or(""))
        .collect();

    assert_eq!(listed_ids.len(), 1);
    assert!(!listed_ids.contains(&id1.as_str()));
}

// --- New operation tests ---

#[tokio::test]
async fn test_activate_pipeline() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("activate-pipeline")
        .unique_id("uid-activate")
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    // Activate should succeed
    client
        .activate_pipeline()
        .pipeline_id(&pipeline_id)
        .send()
        .await
        .expect("activate_pipeline should succeed");

    // Describe should show ACTIVE state in fields
    let resp = client
        .describe_pipelines()
        .pipeline_ids(&pipeline_id)
        .send()
        .await
        .unwrap();
    let fields = resp.pipeline_description_list()[0].fields();
    let state_field = fields.iter().find(|f| f.key() == "@pipelineState");
    assert!(state_field.is_some());
    assert_eq!(state_field.unwrap().string_value(), Some("ACTIVE"));
}

#[tokio::test]
async fn test_activate_pipeline_nonexistent_fails() {
    let client = make_datapipeline_client().await;

    let result = client
        .activate_pipeline()
        .pipeline_id("df-nonexistent-activate")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_deactivate_pipeline() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("deactivate-pipeline")
        .unique_id("uid-deactivate")
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    // First activate, then deactivate
    client
        .activate_pipeline()
        .pipeline_id(&pipeline_id)
        .send()
        .await
        .unwrap();

    client
        .deactivate_pipeline()
        .pipeline_id(&pipeline_id)
        .send()
        .await
        .expect("deactivate_pipeline should succeed");

    // Describe should show INACTIVE state
    let resp = client
        .describe_pipelines()
        .pipeline_ids(&pipeline_id)
        .send()
        .await
        .unwrap();
    let fields = resp.pipeline_description_list()[0].fields();
    let state_field = fields.iter().find(|f| f.key() == "@pipelineState");
    assert!(state_field.is_some());
    assert_eq!(state_field.unwrap().string_value(), Some("INACTIVE"));
}

#[tokio::test]
async fn test_deactivate_pipeline_nonexistent_fails() {
    let client = make_datapipeline_client().await;

    let result = client
        .deactivate_pipeline()
        .pipeline_id("df-nonexistent-deactivate")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_add_and_remove_tags() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("tag-pipeline")
        .unique_id("uid-tags")
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    // Add tags
    client
        .add_tags()
        .pipeline_id(&pipeline_id)
        .tags(
            aws_sdk_datapipeline::types::Tag::builder()
                .key("env")
                .value("prod")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_datapipeline::types::Tag::builder()
                .key("team")
                .value("data")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("add_tags should succeed");

    // Verify tags via describe
    let resp = client
        .describe_pipelines()
        .pipeline_ids(&pipeline_id)
        .send()
        .await
        .unwrap();
    let tags = resp.pipeline_description_list()[0].tags();
    assert_eq!(tags.len(), 2);
    let env_tag = tags.iter().find(|t| t.key() == "env").unwrap();
    assert_eq!(env_tag.value(), "prod");

    // Remove one tag
    client
        .remove_tags()
        .pipeline_id(&pipeline_id)
        .tag_keys("env")
        .send()
        .await
        .expect("remove_tags should succeed");

    // Verify the tag was removed
    let resp2 = client
        .describe_pipelines()
        .pipeline_ids(&pipeline_id)
        .send()
        .await
        .unwrap();
    let tags2 = resp2.pipeline_description_list()[0].tags();
    assert_eq!(tags2.len(), 1);
    assert_eq!(tags2[0].key(), "team");
}

#[tokio::test]
async fn test_add_tags_updates_existing_key() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("tag-update-pipeline")
        .unique_id("uid-tag-update")
        .tags(
            aws_sdk_datapipeline::types::Tag::builder()
                .key("env")
                .value("dev")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    // Update an existing tag value
    client
        .add_tags()
        .pipeline_id(&pipeline_id)
        .tags(
            aws_sdk_datapipeline::types::Tag::builder()
                .key("env")
                .value("prod")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("add_tags (update) should succeed");

    let resp = client
        .describe_pipelines()
        .pipeline_ids(&pipeline_id)
        .send()
        .await
        .unwrap();
    let tags = resp.pipeline_description_list()[0].tags();
    // Still just one tag, but with updated value
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].value(), "prod");
}

#[tokio::test]
async fn test_add_tags_nonexistent_pipeline_fails() {
    let client = make_datapipeline_client().await;
    let result = client
        .add_tags()
        .pipeline_id("df-no-such")
        .tags(
            aws_sdk_datapipeline::types::Tag::builder()
                .key("k")
                .value("v")
                .build()
                .unwrap(),
        )
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_objects() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("obj-pipeline")
        .unique_id("uid-obj")
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    // Put a definition with two objects
    let obj_a = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("ObjA")
        .name("ObjectA")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("type")
                .string_value("Schedule")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    let obj_b = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("ObjB")
        .name("ObjectB")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("type")
                .string_value("ShellCommandActivity")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    client
        .put_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .pipeline_objects(obj_a)
        .pipeline_objects(obj_b)
        .send()
        .await
        .unwrap();

    // Describe a specific object
    let resp = client
        .describe_objects()
        .pipeline_id(&pipeline_id)
        .object_ids("ObjA")
        .send()
        .await
        .expect("describe_objects should succeed");

    let objects = resp.pipeline_objects();
    assert_eq!(objects.len(), 1);
    assert_eq!(objects[0].id(), "ObjA");
    assert_eq!(objects[0].name(), "ObjectA");
}

#[tokio::test]
async fn test_describe_objects_nonexistent_pipeline_fails() {
    let client = make_datapipeline_client().await;
    let result = client
        .describe_objects()
        .pipeline_id("df-no-such")
        .object_ids("ObjA")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_query_objects() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("query-pipeline")
        .unique_id("uid-query")
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    let obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("MyObj")
        .name("MyObject")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("workerGroup")
                .string_value("myWorkerGroup")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    client
        .put_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .pipeline_objects(obj)
        .send()
        .await
        .unwrap();

    let resp = client
        .query_objects()
        .pipeline_id(&pipeline_id)
        .sphere("COMPONENT")
        .send()
        .await
        .expect("query_objects should succeed");

    // COMPONENT sphere should return all objects
    let ids = resp.ids();
    assert!(!ids.is_empty());
    assert!(ids.contains(&"MyObj".to_string()));
}

#[tokio::test]
async fn test_query_objects_nonexistent_pipeline_fails() {
    let client = make_datapipeline_client().await;
    let result = client
        .query_objects()
        .pipeline_id("df-no-such")
        .sphere("COMPONENT")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_set_status() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("status-pipeline")
        .unique_id("uid-status")
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    let obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("MyObj")
        .name("MyObject")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("type")
                .string_value("ShellCommandActivity")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    client
        .put_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .pipeline_objects(obj)
        .send()
        .await
        .unwrap();

    // Set status on an object
    client
        .set_status()
        .pipeline_id(&pipeline_id)
        .object_ids("MyObj")
        .status("RERUN")
        .send()
        .await
        .expect("set_status should succeed");

    // Describe objects to verify the @status field was set
    let resp = client
        .describe_objects()
        .pipeline_id(&pipeline_id)
        .object_ids("MyObj")
        .send()
        .await
        .unwrap();
    let objects = resp.pipeline_objects();
    assert_eq!(objects.len(), 1);
    let status_field = objects[0].fields().iter().find(|f| f.key() == "@status");
    assert!(status_field.is_some());
    assert_eq!(status_field.unwrap().string_value(), Some("RERUN"));
}

#[tokio::test]
async fn test_set_status_nonexistent_pipeline_fails() {
    let client = make_datapipeline_client().await;
    let result = client
        .set_status()
        .pipeline_id("df-no-such")
        .object_ids("Obj1")
        .status("RERUN")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_poll_for_task_empty_returns_no_task() {
    let client = make_datapipeline_client().await;

    // No active pipeline with matching worker group - should return empty task object
    let resp = client
        .poll_for_task()
        .worker_group("nonexistent-group")
        .send()
        .await
        .expect("poll_for_task should not error even with no tasks");

    // No task should be returned
    assert!(resp.task_object().is_none());
}

#[tokio::test]
async fn test_poll_for_task_active_pipeline() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("poll-pipeline")
        .unique_id("uid-poll")
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    let obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("Default")
        .name("Default")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("workerGroup")
                .string_value("myGroup")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    client
        .put_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .pipeline_objects(obj)
        .send()
        .await
        .unwrap();

    client
        .activate_pipeline()
        .pipeline_id(&pipeline_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .poll_for_task()
        .worker_group("myGroup")
        .send()
        .await
        .expect("poll_for_task should succeed on active pipeline");

    let task = resp.task_object();
    assert!(task.is_some());
    let task = task.unwrap();
    assert!(task.task_id().map(|t| !t.is_empty()).unwrap_or(false));
    assert_eq!(task.pipeline_id(), Some(pipeline_id.as_str()));
}

#[tokio::test]
async fn test_report_task_progress() {
    let client = make_datapipeline_client().await;

    // First create and activate a pipeline, then poll for a task
    let create_resp = client
        .create_pipeline()
        .name("progress-pipeline")
        .unique_id("uid-progress")
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    let obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("Default")
        .name("Default")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("workerGroup")
                .string_value("progressGroup")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    client
        .put_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .pipeline_objects(obj)
        .send()
        .await
        .unwrap();
    client
        .activate_pipeline()
        .pipeline_id(&pipeline_id)
        .send()
        .await
        .unwrap();

    let poll_resp = client
        .poll_for_task()
        .worker_group("progressGroup")
        .send()
        .await
        .unwrap();

    let task_id = poll_resp
        .task_object()
        .and_then(|t| t.task_id())
        .unwrap_or("")
        .to_string();

    let resp = client
        .report_task_progress()
        .task_id(&task_id)
        .send()
        .await
        .expect("report_task_progress should succeed");

    assert!(!resp.canceled());
}

#[tokio::test]
async fn test_report_task_progress_nonexistent_task_fails() {
    let client = make_datapipeline_client().await;
    let result = client
        .report_task_progress()
        .task_id("nonexistent-task-id")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_report_task_runner_heartbeat() {
    let client = make_datapipeline_client().await;

    let resp = client
        .report_task_runner_heartbeat()
        .taskrunner_id("my-runner-1")
        .send()
        .await
        .expect("report_task_runner_heartbeat should succeed");

    assert!(!resp.terminate());
}

#[tokio::test]
async fn test_set_task_status() {
    let client = make_datapipeline_client().await;

    // Create pipeline, activate it, poll for task, then set task status
    let create_resp = client
        .create_pipeline()
        .name("task-status-pipeline")
        .unique_id("uid-task-status")
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    let obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("Default")
        .name("Default")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("workerGroup")
                .string_value("statusGroup")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    client
        .put_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .pipeline_objects(obj)
        .send()
        .await
        .unwrap();
    client
        .activate_pipeline()
        .pipeline_id(&pipeline_id)
        .send()
        .await
        .unwrap();

    let poll_resp = client
        .poll_for_task()
        .worker_group("statusGroup")
        .send()
        .await
        .unwrap();

    let task_id = poll_resp
        .task_object()
        .and_then(|t| t.task_id())
        .unwrap_or("")
        .to_string();

    client
        .set_task_status()
        .task_id(&task_id)
        .task_status(aws_sdk_datapipeline::types::TaskStatus::Finished)
        .send()
        .await
        .expect("set_task_status should succeed");
}

#[tokio::test]
async fn test_set_task_status_nonexistent_task_fails() {
    let client = make_datapipeline_client().await;
    let result = client
        .set_task_status()
        .task_id("nonexistent-task-id")
        .task_status(aws_sdk_datapipeline::types::TaskStatus::Finished)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_evaluate_expression() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("eval-pipeline")
        .unique_id("uid-eval")
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    let obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("MyObj")
        .name("MyObject")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("type")
                .string_value("Schedule")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    client
        .put_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .pipeline_objects(obj)
        .send()
        .await
        .unwrap();

    let resp = client
        .evaluate_expression()
        .pipeline_id(&pipeline_id)
        .object_id("MyObj")
        .expression("#{type}")
        .send()
        .await
        .expect("evaluate_expression should succeed");

    assert_eq!(resp.evaluated_expression(), "Schedule");
}

#[tokio::test]
async fn test_evaluate_expression_nonexistent_pipeline_fails() {
    let client = make_datapipeline_client().await;
    let result = client
        .evaluate_expression()
        .pipeline_id("df-no-such")
        .object_id("ObjA")
        .expression("#{type}")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_evaluate_expression_nonexistent_object_fails() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("eval-no-obj-pipeline")
        .unique_id("uid-eval-no-obj")
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    let result = client
        .evaluate_expression()
        .pipeline_id(&pipeline_id)
        .object_id("NonExistentObj")
        .expression("#{type}")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_validate_pipeline_definition() {
    let client = make_datapipeline_client().await;

    let create_resp = client
        .create_pipeline()
        .name("validate-pipeline")
        .unique_id("uid-validate")
        .send()
        .await
        .unwrap();
    let pipeline_id = create_resp.pipeline_id().to_string();

    let obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("Default")
        .name("Default")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("workerGroup")
                .string_value("wg")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let resp = client
        .validate_pipeline_definition()
        .pipeline_id(&pipeline_id)
        .pipeline_objects(obj)
        .send()
        .await
        .expect("validate_pipeline_definition should succeed");

    assert!(!resp.errored());
    assert!(resp.validation_errors().is_empty());
    assert!(resp.validation_warnings().is_empty());
}

#[tokio::test]
async fn test_validate_pipeline_definition_nonexistent_pipeline_fails() {
    let client = make_datapipeline_client().await;

    let obj = aws_sdk_datapipeline::types::PipelineObject::builder()
        .id("Default")
        .name("Default")
        .fields(
            aws_sdk_datapipeline::types::Field::builder()
                .key("workerGroup")
                .string_value("wg")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let result = client
        .validate_pipeline_definition()
        .pipeline_id("df-no-such")
        .pipeline_objects(obj)
        .send()
        .await;
    assert!(result.is_err());
}
