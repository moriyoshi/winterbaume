use aws_sdk_servicecatalogappregistry::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_servicecatalogappregistry::ServiceCatalogAppRegistryService;

async fn make_client() -> aws_sdk_servicecatalogappregistry::Client {
    let mock = MockAws::builder()
        .with_service(ServiceCatalogAppRegistryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_servicecatalogappregistry::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    aws_sdk_servicecatalogappregistry::Client::new(&config)
}

#[tokio::test]
async fn test_create_application() {
    let client = make_client().await;

    let resp = client
        .create_application()
        .name("my-app")
        .description("A test application")
        .send()
        .await
        .expect("create_application should succeed");

    let app = resp.application().expect("should have application");
    assert_eq!(app.name().unwrap(), "my-app");
    assert_eq!(app.description().unwrap(), "A test application");
    assert!(app.id().is_some());
    assert!(app.arn().is_some());
}

#[tokio::test]
async fn test_create_and_get_application() {
    let client = make_client().await;

    let create_resp = client
        .create_application()
        .name("get-test-app")
        .send()
        .await
        .expect("create should succeed");

    let app_id = create_resp.application().unwrap().id().unwrap().to_string();

    let get_resp = client
        .get_application()
        .application(&app_id)
        .send()
        .await
        .expect("get_application should succeed");

    assert_eq!(get_resp.id().unwrap(), app_id);
    assert_eq!(get_resp.name().unwrap(), "get-test-app");
    assert!(get_resp.arn().is_some());
}

#[tokio::test]
async fn test_delete_application() {
    let client = make_client().await;

    let create_resp = client
        .create_application()
        .name("delete-me")
        .send()
        .await
        .unwrap();

    let app_id = create_resp.application().unwrap().id().unwrap().to_string();

    client
        .delete_application()
        .application(&app_id)
        .send()
        .await
        .expect("delete should succeed");

    let result = client.get_application().application(&app_id).send().await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_list_applications() {
    let client = make_client().await;

    client
        .create_application()
        .name("list-app-1")
        .send()
        .await
        .unwrap();

    client
        .create_application()
        .name("list-app-2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");

    assert_eq!(resp.applications().len(), 2);
}

#[tokio::test]
async fn test_create_duplicate_application_fails() {
    let client = make_client().await;

    client
        .create_application()
        .name("dup-app")
        .send()
        .await
        .unwrap();

    let result = client.create_application().name("dup-app").send().await;
    assert!(result.is_err(), "duplicate application should fail");
}

#[tokio::test]
async fn test_get_nonexistent_application_fails() {
    let client = make_client().await;

    let result = client
        .get_application()
        .application("nonexistent-id")
        .send()
        .await;
    assert!(result.is_err(), "get nonexistent application should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_application_fails() {
    let client = make_client().await;

    let result = client
        .delete_application()
        .application("nonexistent-id")
        .send()
        .await;
    assert!(
        result.is_err(),
        "delete nonexistent application should fail"
    );
}

#[tokio::test]
async fn test_create_application_with_tags() {
    let client = make_client().await;

    let resp = client
        .create_application()
        .name("tagged-app")
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("create with tags should succeed");

    let app = resp.application().unwrap();
    assert_eq!(app.name().unwrap(), "tagged-app");

    let app_id = app.id().unwrap().to_string();

    let get_resp = client
        .get_application()
        .application(&app_id)
        .send()
        .await
        .expect("get should succeed");

    let tags = get_resp.tags().unwrap();
    assert_eq!(tags.get("env").unwrap(), "test");
    assert_eq!(tags.get("team").unwrap(), "platform");
}

// ============================================================================
// Tests derived from AWS documentation: AWS Service Catalog AppRegistry
// Source: https://docs.aws.amazon.com/servicecatalog/latest/dg/API_app-registry_CreateApplication.html
// ============================================================================

#[tokio::test]
async fn test_create_application_minimal() {
    // Create with only the required `name` field — no description, no tags.
    let client = make_client().await;

    let resp = client
        .create_application()
        .name("minimal-app")
        .send()
        .await
        .expect("create_application with minimal params should succeed");

    let app = resp
        .application()
        .expect("should have application in response");
    assert_eq!(app.name().unwrap(), "minimal-app");
    assert!(
        app.description().is_none(),
        "description should be absent when not provided"
    );
    assert!(app.id().is_some(), "id should be present");
    assert!(app.arn().is_some(), "arn should be present");
    assert!(
        app.creation_time().is_some(),
        "creation_time should be present"
    );
    assert!(
        app.last_update_time().is_some(),
        "last_update_time should be present"
    );
}

#[tokio::test]
async fn test_get_application_by_name() {
    // GetApplication accepts name or ID; verify name-based lookup returns the same app.
    let client = make_client().await;

    let create_resp = client
        .create_application()
        .name("lookup-by-name-app")
        .send()
        .await
        .expect("create should succeed");

    let created_id = create_resp.application().unwrap().id().unwrap().to_string();

    let get_by_name = client
        .get_application()
        .application("lookup-by-name-app")
        .send()
        .await
        .expect("get_application by name should succeed");

    assert_eq!(
        get_by_name.id().unwrap(),
        created_id,
        "ID returned by name lookup must match ID returned by create"
    );
    assert_eq!(get_by_name.name().unwrap(), "lookup-by-name-app");
}

#[tokio::test]
async fn test_get_application_fields() {
    // Verify all key response fields are populated by GetApplication.
    let client = make_client().await;

    let create_resp = client
        .create_application()
        .name("fields-check-app")
        .description("field verification app")
        .send()
        .await
        .expect("create should succeed");

    let app_id = create_resp.application().unwrap().id().unwrap().to_string();

    let get_resp = client
        .get_application()
        .application(&app_id)
        .send()
        .await
        .expect("get_application should succeed");

    assert!(get_resp.id().is_some(), "id must be present");
    assert!(get_resp.arn().is_some(), "arn must be present");
    assert_eq!(get_resp.name().unwrap(), "fields-check-app");
    assert_eq!(get_resp.description().unwrap(), "field verification app");
    assert!(
        get_resp.creation_time().is_some(),
        "creation_time must be present"
    );
    assert!(
        get_resp.last_update_time().is_some(),
        "last_update_time must be present"
    );
}

#[tokio::test]
async fn test_create_application_arn_format() {
    // Verify the ARN returned by CreateApplication matches the expected format.
    let client = make_client().await;

    let resp = client
        .create_application()
        .name("arn-format-app")
        .send()
        .await
        .expect("create should succeed");

    let app = resp.application().unwrap();
    let arn = app.arn().unwrap();

    assert!(
        arn.starts_with("arn:aws:servicecatalog:"),
        "ARN must start with arn:aws:servicecatalog:, got: {arn}"
    );
    assert!(
        arn.contains("/applications/"),
        "ARN must contain /applications/, got: {arn}"
    );
}

#[tokio::test]
async fn test_delete_application_by_name() {
    // DeleteApplication accepts name or ID; verify name-based deletion works.
    let client = make_client().await;

    client
        .create_application()
        .name("delete-by-name-app")
        .send()
        .await
        .expect("create should succeed");

    client
        .delete_application()
        .application("delete-by-name-app")
        .send()
        .await
        .expect("delete_application by name should succeed");

    let result = client
        .get_application()
        .application("delete-by-name-app")
        .send()
        .await;

    assert!(
        result.is_err(),
        "get after name-based delete should return an error"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_applications_empty() {
    // ListApplications on a fresh mock should return an empty list.
    let client = make_client().await;

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications on empty state should succeed");

    assert_eq!(
        resp.applications().len(),
        0,
        "expected empty applications list on fresh mock"
    );
}

#[tokio::test]
async fn test_list_applications_contains_correct_fields() {
    // Verify ApplicationSummary objects in ListApplications contain the expected fields.
    let client = make_client().await;

    client
        .create_application()
        .name("summary-fields-app")
        .description("summary desc")
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(resp.applications().len(), 1);
    let summary = &resp.applications()[0];
    assert!(summary.id().is_some(), "summary id must be present");
    assert!(summary.arn().is_some(), "summary arn must be present");
    assert_eq!(summary.name().unwrap(), "summary-fields-app");
    assert!(
        summary.creation_time().is_some(),
        "summary creation_time must be present"
    );
    assert!(
        summary.last_update_time().is_some(),
        "summary last_update_time must be present"
    );
}

#[tokio::test]
async fn test_lifecycle_full() {
    // Full create -> get -> delete -> verify-gone lifecycle.
    let client = make_client().await;

    // Create
    let create_resp = client
        .create_application()
        .name("lifecycle-app")
        .description("lifecycle test")
        .send()
        .await
        .expect("create should succeed");

    let app_id = create_resp.application().unwrap().id().unwrap().to_string();
    assert!(!app_id.is_empty(), "app id must not be empty");

    // Get
    let get_resp = client
        .get_application()
        .application(&app_id)
        .send()
        .await
        .expect("get should succeed after create");

    assert_eq!(get_resp.id().unwrap(), app_id);
    assert_eq!(get_resp.name().unwrap(), "lifecycle-app");
    assert_eq!(get_resp.description().unwrap(), "lifecycle test");

    // Delete
    client
        .delete_application()
        .application(&app_id)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client.get_application().application(&app_id).send().await;
    assert!(result.is_err(), "get after delete should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException after delete, got: {err_str}"
    );
}

#[tokio::test]
async fn test_duplicate_name_after_delete() {
    // After deleting an application, the same name should be available for reuse.
    let client = make_client().await;

    client
        .create_application()
        .name("reuse-name-app")
        .send()
        .await
        .expect("first create should succeed");

    client
        .delete_application()
        .application("reuse-name-app")
        .send()
        .await
        .expect("delete should succeed");

    let resp = client
        .create_application()
        .name("reuse-name-app")
        .send()
        .await
        .expect("create with same name after delete should succeed");

    let app = resp.application().unwrap();
    assert_eq!(app.name().unwrap(), "reuse-name-app");
    assert!(app.id().is_some(), "new app should have an id");
}

#[tokio::test]
async fn test_get_application_not_found_error_type() {
    // Verify the error returned for a nonexistent application is ResourceNotFoundException.
    let client = make_client().await;

    let err = client
        .get_application()
        .application("does-not-exist-xyz")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_duplicate_conflict_error_type() {
    // Verify the error returned for a duplicate application name is ConflictException.
    let client = make_client().await;

    client
        .create_application()
        .name("conflict-app")
        .send()
        .await
        .expect("first create should succeed");

    let err = client
        .create_application()
        .name("conflict-app")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException on duplicate name, got: {err_str}"
    );
}

// ============================================================================
// Attribute Group tests
// ============================================================================

#[tokio::test]
async fn test_create_attribute_group() {
    let client = make_client().await;

    let resp = client
        .create_attribute_group()
        .name("my-ag")
        .attributes(r#"{"key":"value"}"#)
        .description("A test attribute group")
        .send()
        .await
        .expect("create_attribute_group should succeed");

    let ag = resp.attribute_group().expect("should have attribute_group");
    assert_eq!(ag.name().unwrap(), "my-ag");
    assert_eq!(ag.description().unwrap(), "A test attribute group");
    assert!(ag.id().is_some());
    assert!(ag.arn().is_some());
}

#[tokio::test]
async fn test_get_attribute_group() {
    let client = make_client().await;

    let create_resp = client
        .create_attribute_group()
        .name("get-ag")
        .attributes(r#"{"foo":"bar"}"#)
        .send()
        .await
        .expect("create should succeed");

    let ag_id = create_resp
        .attribute_group()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let get_resp = client
        .get_attribute_group()
        .attribute_group(&ag_id)
        .send()
        .await
        .expect("get_attribute_group should succeed");

    assert_eq!(get_resp.id().unwrap(), ag_id);
    assert_eq!(get_resp.name().unwrap(), "get-ag");
    assert_eq!(get_resp.attributes().unwrap(), r#"{"foo":"bar"}"#);
}

#[tokio::test]
async fn test_update_attribute_group() {
    let client = make_client().await;

    let create_resp = client
        .create_attribute_group()
        .name("update-ag")
        .attributes(r#"{"old":"data"}"#)
        .send()
        .await
        .expect("create should succeed");

    let ag_id = create_resp
        .attribute_group()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let update_resp = client
        .update_attribute_group()
        .attribute_group(&ag_id)
        .description("updated description")
        .attributes(r#"{"new":"data"}"#)
        .send()
        .await
        .expect("update_attribute_group should succeed");

    let ag = update_resp.attribute_group().unwrap();
    assert_eq!(ag.description().unwrap(), "updated description");
}

#[tokio::test]
async fn test_delete_attribute_group() {
    let client = make_client().await;

    let create_resp = client
        .create_attribute_group()
        .name("delete-ag")
        .attributes(r#"{}"#)
        .send()
        .await
        .expect("create should succeed");

    let ag_id = create_resp
        .attribute_group()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .delete_attribute_group()
        .attribute_group(&ag_id)
        .send()
        .await
        .expect("delete_attribute_group should succeed");

    let result = client
        .get_attribute_group()
        .attribute_group(&ag_id)
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_list_attribute_groups() {
    let client = make_client().await;

    client
        .create_attribute_group()
        .name("ag-list-1")
        .attributes(r#"{}"#)
        .send()
        .await
        .unwrap();
    client
        .create_attribute_group()
        .name("ag-list-2")
        .attributes(r#"{}"#)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_attribute_groups()
        .send()
        .await
        .expect("list_attribute_groups should succeed");

    assert_eq!(resp.attribute_groups().len(), 2);
}

// ============================================================================
// Association tests
// ============================================================================

#[tokio::test]
async fn test_associate_and_disassociate_attribute_group() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("assoc-app")
        .send()
        .await
        .unwrap()
        .application()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let ag = client
        .create_attribute_group()
        .name("assoc-ag")
        .attributes(r#"{}"#)
        .send()
        .await
        .unwrap()
        .attribute_group()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // Associate
    let assoc_resp = client
        .associate_attribute_group()
        .application(&app)
        .attribute_group(&ag)
        .send()
        .await
        .expect("associate_attribute_group should succeed");

    assert!(assoc_resp.application_arn().is_some());
    assert!(assoc_resp.attribute_group_arn().is_some());

    // List
    let list_resp = client
        .list_associated_attribute_groups()
        .application(&app)
        .send()
        .await
        .expect("list_associated_attribute_groups should succeed");

    assert_eq!(list_resp.attribute_groups().len(), 1);

    // Disassociate
    client
        .disassociate_attribute_group()
        .application(&app)
        .attribute_group(&ag)
        .send()
        .await
        .expect("disassociate_attribute_group should succeed");

    let list_after = client
        .list_associated_attribute_groups()
        .application(&app)
        .send()
        .await
        .unwrap();
    assert_eq!(list_after.attribute_groups().len(), 0);
}

#[tokio::test]
async fn test_list_attribute_groups_for_application() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("detail-app")
        .send()
        .await
        .unwrap()
        .application()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let ag = client
        .create_attribute_group()
        .name("detail-ag")
        .attributes(r#"{"x":"y"}"#)
        .send()
        .await
        .unwrap()
        .attribute_group()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    client
        .associate_attribute_group()
        .application(&app)
        .attribute_group(&ag)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_attribute_groups_for_application()
        .application(&app)
        .send()
        .await
        .expect("list_attribute_groups_for_application should succeed");

    assert_eq!(resp.attribute_groups_details().len(), 1);
    assert!(resp.attribute_groups_details()[0].id().is_some());
    assert!(resp.attribute_groups_details()[0].arn().is_some());
}

#[tokio::test]
async fn test_associate_and_list_resources() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("res-app")
        .send()
        .await
        .unwrap()
        .application()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    use aws_sdk_servicecatalogappregistry::types::ResourceType;

    client
        .associate_resource()
        .application(&app)
        .resource_type(ResourceType::CfnStack)
        .resource("my-stack")
        .send()
        .await
        .expect("associate_resource should succeed");

    let list_resp = client
        .list_associated_resources()
        .application(&app)
        .send()
        .await
        .expect("list_associated_resources should succeed");

    assert_eq!(list_resp.resources().len(), 1);
    assert_eq!(list_resp.resources()[0].name().unwrap(), "my-stack");
}

#[tokio::test]
async fn test_get_associated_resource() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("get-res-app")
        .send()
        .await
        .unwrap()
        .application()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    use aws_sdk_servicecatalogappregistry::types::ResourceType;

    client
        .associate_resource()
        .application(&app)
        .resource_type(ResourceType::CfnStack)
        .resource("get-stack")
        .send()
        .await
        .unwrap();

    let get_resp = client
        .get_associated_resource()
        .application(&app)
        .resource_type(ResourceType::CfnStack)
        .resource("get-stack")
        .send()
        .await
        .expect("get_associated_resource should succeed");

    let resource = get_resp.resource().expect("should have resource");
    assert_eq!(resource.name().unwrap(), "get-stack");
}

#[tokio::test]
async fn test_disassociate_resource() {
    let client = make_client().await;

    let app = client
        .create_application()
        .name("disassoc-res-app")
        .send()
        .await
        .unwrap()
        .application()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    use aws_sdk_servicecatalogappregistry::types::ResourceType;

    client
        .associate_resource()
        .application(&app)
        .resource_type(ResourceType::CfnStack)
        .resource("disassoc-stack")
        .send()
        .await
        .unwrap();

    client
        .disassociate_resource()
        .application(&app)
        .resource_type(ResourceType::CfnStack)
        .resource("disassoc-stack")
        .send()
        .await
        .expect("disassociate_resource should succeed");

    let list_resp = client
        .list_associated_resources()
        .application(&app)
        .send()
        .await
        .unwrap();

    assert_eq!(list_resp.resources().len(), 0);
}

// ============================================================================
// Tag tests
// ============================================================================

#[tokio::test]
async fn test_tag_resource_and_list_tags() {
    let client = make_client().await;

    let app_arn = client
        .create_application()
        .name("tag-app")
        .send()
        .await
        .unwrap()
        .application()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_arn(&app_arn)
        .tags("env", "prod")
        .tags("team", "backend")
        .send()
        .await
        .expect("tag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&app_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags().expect("should have tags");
    assert_eq!(
        tags.get("env").map(|s| s.as_str()),
        Some("prod"),
        "env tag should be prod"
    );
    assert_eq!(
        tags.get("team").map(|s| s.as_str()),
        Some("backend"),
        "team tag should be backend"
    );
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    let app_arn = client
        .create_application()
        .name("untag-app")
        .tags("env", "test")
        .tags("team", "ops")
        .send()
        .await
        .unwrap()
        .application()
        .unwrap()
        .arn()
        .unwrap()
        .to_string();

    client
        .untag_resource()
        .resource_arn(&app_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&app_arn)
        .send()
        .await
        .unwrap();

    let tags = list_resp.tags().expect("should have tags");
    assert!(
        !tags.contains_key("env"),
        "env tag should have been removed"
    );
    assert_eq!(
        tags.get("team").map(|s| s.as_str()),
        Some("ops"),
        "team tag should remain"
    );
}

// ============================================================================
// Configuration tests
// ============================================================================

#[tokio::test]
async fn test_put_and_get_configuration() {
    let client = make_client().await;

    use aws_sdk_servicecatalogappregistry::types::{
        AppRegistryConfiguration, TagQueryConfiguration,
    };

    client
        .put_configuration()
        .configuration(
            AppRegistryConfiguration::builder()
                .tag_query_configuration(
                    TagQueryConfiguration::builder()
                        .tag_key("awsApplication")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("put_configuration should succeed");

    let get_resp = client
        .get_configuration()
        .send()
        .await
        .expect("get_configuration should succeed");

    let config = get_resp.configuration().expect("should have configuration");
    let tqc = config
        .tag_query_configuration()
        .expect("should have tagQueryConfiguration");
    assert_eq!(tqc.tag_key().unwrap(), "awsApplication");
}

// ============================================================================
// UpdateApplication test
// ============================================================================

#[tokio::test]
async fn test_update_application() {
    let client = make_client().await;

    let create_resp = client
        .create_application()
        .name("update-app")
        .description("original desc")
        .send()
        .await
        .expect("create should succeed");

    let app_id = create_resp.application().unwrap().id().unwrap().to_string();

    let update_resp = client
        .update_application()
        .application(&app_id)
        .description("updated desc")
        .send()
        .await
        .expect("update_application should succeed");

    let app = update_resp.application().unwrap();
    assert_eq!(app.description().unwrap(), "updated desc");
    assert_eq!(app.name().unwrap(), "update-app");
}

// ============================================================================
// SyncResource test
// ============================================================================

#[tokio::test]
async fn test_sync_resource() {
    let client = make_client().await;

    use aws_sdk_servicecatalogappregistry::types::ResourceType;

    let resp = client
        .sync_resource()
        .resource_type(ResourceType::CfnStack)
        .resource("sync-stack")
        .send()
        .await
        .expect("sync_resource should succeed");

    // resource_arn should be present
    assert!(
        resp.resource_arn().is_some(),
        "resource_arn should be present"
    );
}

// ============================================================================
// Error path tests for new operations
// ============================================================================

#[tokio::test]
async fn test_get_nonexistent_attribute_group_fails() {
    let client = make_client().await;
    let result = client
        .get_attribute_group()
        .attribute_group("does-not-exist")
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(err_str.contains("ResourceNotFoundException"), "{err_str}");
}

#[tokio::test]
async fn test_associate_attribute_group_nonexistent_app_fails() {
    let client = make_client().await;
    let ag = client
        .create_attribute_group()
        .name("orphan-ag")
        .attributes(r#"{}"#)
        .send()
        .await
        .unwrap()
        .attribute_group()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    let result = client
        .associate_attribute_group()
        .application("nonexistent-app")
        .attribute_group(&ag)
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(err_str.contains("ResourceNotFoundException"), "{err_str}");
}

#[tokio::test]
async fn test_tag_resource_nonexistent_arn_fails() {
    let client = make_client().await;
    let result = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:servicecatalog:us-east-1:123456789012:/applications/nonexistent")
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(err_str.contains("ResourceNotFoundException"), "{err_str}");
}
