use aws_sdk_resourcegroups::config::BehaviorVersion;
use aws_sdk_resourcegroups::types::{QueryType, ResourceQuery, TagSyncTaskStatus};
use winterbaume_core::MockAws;
use winterbaume_resourcegroups::ResourceGroupsService;

async fn make_client() -> aws_sdk_resourcegroups::Client {
    let mock = MockAws::builder()
        .with_service(ResourceGroupsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_resourcegroups::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_resourcegroups::Client::new(&config)
}

async fn create_test_group(client: &aws_sdk_resourcegroups::Client, name: &str) {
    client
        .create_group()
        .name(name)
        .description("A test group")
        .resource_query(
            ResourceQuery::builder()
                .r#type(QueryType::TagFilters10)
                .query(r#"{"ResourceTypeFilters":["AWS::AllSupported"],"TagFilters":[{"Key":"Stage","Values":["Test"]}]}"#)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_group should succeed");
}

#[tokio::test]
async fn test_create_group() {
    let client = make_client().await;

    let resp = client
        .create_group()
        .name("my-group")
        .description("A test group")
        .resource_query(
            ResourceQuery::builder()
                .r#type(QueryType::TagFilters10)
                .query(r#"{"ResourceTypeFilters":["AWS::AllSupported"],"TagFilters":[{"Key":"Stage","Values":["Test"]}]}"#)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_group should succeed");

    let group = resp.group().expect("should have group");
    assert_eq!(group.name(), "my-group");
    assert!(group.group_arn().contains("arn:aws:resource-groups:"));
}

#[tokio::test]
async fn test_get_group() {
    let client = make_client().await;

    client
        .create_group()
        .name("get-test")
        .resource_query(
            ResourceQuery::builder()
                .r#type(QueryType::TagFilters10)
                .query("{}")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .get_group()
        .group("get-test")
        .send()
        .await
        .expect("get_group should succeed");

    let group = resp.group().expect("should have group");
    assert_eq!(group.name(), "get-test");
}

#[tokio::test]
async fn test_delete_group() {
    let client = make_client().await;

    client
        .create_group()
        .name("delete-me")
        .resource_query(
            ResourceQuery::builder()
                .r#type(QueryType::TagFilters10)
                .query("{}")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_group()
        .group("delete-me")
        .send()
        .await
        .expect("delete_group should succeed");

    let result = client.get_group().group("delete-me").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_groups() {
    let client = make_client().await;

    for name in ["group-a", "group-b"] {
        client
            .create_group()
            .name(name)
            .resource_query(
                ResourceQuery::builder()
                    .r#type(QueryType::TagFilters10)
                    .query("{}")
                    .build()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_groups()
        .send()
        .await
        .expect("list_groups should succeed");

    assert_eq!(resp.group_identifiers().len(), 2);
}

#[tokio::test]
async fn test_get_nonexistent_group() {
    let client = make_client().await;

    let result = client.get_group().group("nonexistent-group").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_groups_empty() {
    let client = make_client().await;

    let resp = client
        .list_groups()
        .send()
        .await
        .expect("list_groups should succeed on empty state");

    assert_eq!(resp.group_identifiers().len(), 0);
}

#[tokio::test]
async fn test_update_group() {
    let client = make_client().await;
    create_test_group(&client, "update-me").await;

    let resp = client
        .update_group()
        .group("update-me")
        .description("Updated description")
        .send()
        .await
        .expect("update_group should succeed");

    let group = resp.group().expect("should have group");
    assert_eq!(group.name(), "update-me");
    assert_eq!(group.description().unwrap_or(""), "Updated description");
}

#[tokio::test]
async fn test_update_group_query() {
    let client = make_client().await;
    create_test_group(&client, "query-update").await;

    let new_query = ResourceQuery::builder()
        .r#type(QueryType::TagFilters10)
        .query(r#"{"ResourceTypeFilters":["AWS::EC2::Instance"]}"#)
        .build()
        .unwrap();

    let resp = client
        .update_group_query()
        .group("query-update")
        .resource_query(new_query)
        .send()
        .await
        .expect("update_group_query should succeed");

    let gq = resp.group_query().expect("should have group_query");
    assert_eq!(gq.group_name(), "query-update");
    assert_eq!(
        gq.resource_query()
            .expect("should have resource_query")
            .query(),
        r#"{"ResourceTypeFilters":["AWS::EC2::Instance"]}"#
    );
}

#[tokio::test]
async fn test_get_group_configuration() {
    let client = make_client().await;
    create_test_group(&client, "config-group").await;

    let resp = client
        .get_group_configuration()
        .group("config-group")
        .send()
        .await
        .expect("get_group_configuration should succeed");

    // Initially no configuration set, so group_configuration should be None
    assert!(resp.group_configuration().is_none());
}

#[tokio::test]
async fn test_put_group_configuration() {
    use aws_sdk_resourcegroups::types::{GroupConfigurationItem, GroupConfigurationParameter};

    let client = make_client().await;
    create_test_group(&client, "put-config-group").await;

    let param = GroupConfigurationParameter::builder()
        .name("allowed-resource-types")
        .values("AWS::EC2::Instance")
        .build()
        .unwrap();

    let config_item = GroupConfigurationItem::builder()
        .r#type("AWS::ResourceGroups::Generic")
        .parameters(param)
        .build()
        .unwrap();

    client
        .put_group_configuration()
        .group("put-config-group")
        .configuration(config_item)
        .send()
        .await
        .expect("put_group_configuration should succeed");

    // Verify config was stored
    let resp = client
        .get_group_configuration()
        .group("put-config-group")
        .send()
        .await
        .expect("get_group_configuration should succeed after put");

    let gc = resp
        .group_configuration()
        .expect("should have group_configuration");
    let items = gc.configuration();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].r#type(), "AWS::ResourceGroups::Generic");
}

#[tokio::test]
async fn test_get_tags() {
    let client = make_client().await;
    create_test_group(&client, "tags-group").await;

    let arn = "arn:aws:resource-groups:us-east-1:123456789012:group/tags-group";

    let resp = client
        .get_tags()
        .arn(arn)
        .send()
        .await
        .expect("get_tags should succeed");

    assert_eq!(resp.arn().unwrap_or(""), arn);
    assert!(
        resp.tags()
            .unwrap_or(&std::collections::HashMap::new())
            .is_empty()
    );
}

#[tokio::test]
async fn test_tag_and_untag() {
    let client = make_client().await;
    create_test_group(&client, "tag-test").await;

    let arn = "arn:aws:resource-groups:us-east-1:123456789012:group/tag-test";

    // Tag the resource
    let resp = client
        .tag()
        .arn(arn)
        .tags("env", "prod")
        .tags("team", "backend")
        .send()
        .await
        .expect("tag should succeed");

    assert_eq!(resp.arn().unwrap_or(""), arn);
    let tags = resp.tags().unwrap();
    assert_eq!(tags.get("env").unwrap(), "prod");
    assert_eq!(tags.get("team").unwrap(), "backend");

    // Verify tags via get_tags
    let get_resp = client
        .get_tags()
        .arn(arn)
        .send()
        .await
        .expect("get_tags should succeed");

    let tags = get_resp.tags().unwrap();
    assert_eq!(tags.get("env").unwrap(), "prod");
    assert_eq!(tags.get("team").unwrap(), "backend");

    // Untag one key
    let untag_resp = client
        .untag()
        .arn(arn)
        .keys("team")
        .send()
        .await
        .expect("untag should succeed");

    assert_eq!(untag_resp.arn().unwrap_or(""), arn);
    assert_eq!(untag_resp.keys(), &["team".to_string()]);

    // Verify only "env" tag remains
    let get_resp2 = client
        .get_tags()
        .arn(arn)
        .send()
        .await
        .expect("get_tags should succeed after untag");

    let tags = get_resp2.tags().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags.get("env").unwrap(), "prod");
    assert!(tags.get("team").is_none());
}

#[tokio::test]
async fn test_start_and_get_tag_sync_task() {
    let client = make_client().await;
    create_test_group(&client, "sync-group").await;

    let resp = client
        .start_tag_sync_task()
        .group("sync-group")
        .tag_key("Environment")
        .tag_value("Production")
        .role_arn("arn:aws:iam::123456789012:role/tag-sync-role")
        .send()
        .await
        .expect("start_tag_sync_task should succeed");

    assert_eq!(resp.group_name().unwrap_or(""), "sync-group");
    assert_eq!(resp.tag_key().unwrap_or(""), "Environment");
    assert_eq!(resp.tag_value().unwrap_or(""), "Production");
    let task_arn = resp.task_arn().expect("should have task_arn");
    assert!(task_arn.contains("tag-sync-task"));

    // GetTagSyncTask
    let get_resp = client
        .get_tag_sync_task()
        .task_arn(task_arn)
        .send()
        .await
        .expect("get_tag_sync_task should succeed");

    assert_eq!(get_resp.task_arn().unwrap_or(""), task_arn);
    assert_eq!(get_resp.group_name().unwrap_or(""), "sync-group");
    assert_eq!(get_resp.tag_key().unwrap_or(""), "Environment");
    assert_eq!(get_resp.status(), Some(&TagSyncTaskStatus::Active));
}

#[tokio::test]
async fn test_list_tag_sync_tasks() {
    let client = make_client().await;
    create_test_group(&client, "list-sync-group").await;

    client
        .start_tag_sync_task()
        .group("list-sync-group")
        .tag_key("Env")
        .tag_value("Dev")
        .role_arn("arn:aws:iam::123456789012:role/role1")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tag_sync_tasks()
        .send()
        .await
        .expect("list_tag_sync_tasks should succeed");

    let tasks = resp.tag_sync_tasks();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].group_name().unwrap_or(""), "list-sync-group");
}

#[tokio::test]
async fn test_cancel_tag_sync_task() {
    let client = make_client().await;
    create_test_group(&client, "cancel-sync-group").await;

    let start_resp = client
        .start_tag_sync_task()
        .group("cancel-sync-group")
        .tag_key("Team")
        .tag_value("Infra")
        .role_arn("arn:aws:iam::123456789012:role/role2")
        .send()
        .await
        .unwrap();

    let task_arn = start_resp.task_arn().unwrap();

    client
        .cancel_tag_sync_task()
        .task_arn(task_arn)
        .send()
        .await
        .expect("cancel_tag_sync_task should succeed");

    // Verify task is cancelled
    let get_resp = client
        .get_tag_sync_task()
        .task_arn(task_arn)
        .send()
        .await
        .expect("get_tag_sync_task should succeed after cancel");

    assert_eq!(get_resp.status().map(|s| s.as_str()), Some("CANCELLED"));
}

#[tokio::test]
async fn test_update_nonexistent_group() {
    let client = make_client().await;

    let result = client
        .update_group()
        .group("nonexistent")
        .description("test")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: ResourceGroups
// ============================================================================

#[tokio::test]
async fn test_create_group_duplicate() {
    let client = make_client().await;
    create_test_group(&client, "dup-group").await;

    let result = client
        .create_group()
        .name("dup-group")
        .resource_query(
            ResourceQuery::builder()
                .r#type(aws_sdk_resourcegroups::types::QueryType::TagFilters10)
                .query("{}")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err(), "Expected error for duplicate group name");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("BadRequestException") || err_str.contains("already exists"),
        "Expected BadRequestException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_group_with_tags() {
    let client = make_client().await;

    let resp = client
        .create_group()
        .name("tagged-group")
        .description("A tagged group")
        .resource_query(
            ResourceQuery::builder()
                .r#type(aws_sdk_resourcegroups::types::QueryType::TagFilters10)
                .query("{}")
                .build()
                .unwrap(),
        )
        .tags("project", "winterbaume")
        .tags("env", "test")
        .send()
        .await
        .expect("create_group with tags should succeed");

    let group = resp.group().expect("should have group");
    let arn = group.group_arn();
    assert!(arn.contains("arn:aws:resource-groups:"));
    assert!(arn.contains("tagged-group"));

    // Verify tags via get_tags
    let tags_resp = client
        .get_tags()
        .arn(arn)
        .send()
        .await
        .expect("get_tags should succeed");

    let empty = std::collections::HashMap::new();
    let tags = tags_resp.tags().unwrap_or(&empty);
    assert_eq!(tags.get("project").map(|s| s.as_str()), Some("winterbaume"));
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("test"));
}

#[tokio::test]
async fn test_delete_nonexistent_group() {
    let client = make_client().await;

    let result = client.delete_group().group("no-such-group").send().await;

    assert!(result.is_err(), "Expected error deleting nonexistent group");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_group_query_not_found() {
    let client = make_client().await;

    let result = client
        .update_group_query()
        .group("ghost-group")
        .resource_query(
            ResourceQuery::builder()
                .r#type(aws_sdk_resourcegroups::types::QueryType::TagFilters10)
                .query("{}")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err(), "Expected error for nonexistent group");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_put_group_configuration_not_found() {
    use aws_sdk_resourcegroups::types::{GroupConfigurationItem, GroupConfigurationParameter};

    let client = make_client().await;

    let param = GroupConfigurationParameter::builder()
        .name("allowed-resource-types")
        .values("AWS::EC2::Instance")
        .build()
        .unwrap();

    let config_item = GroupConfigurationItem::builder()
        .r#type("AWS::ResourceGroups::Generic")
        .parameters(param)
        .build()
        .unwrap();

    let result = client
        .put_group_configuration()
        .group("no-such-group")
        .configuration(config_item)
        .send()
        .await;

    assert!(result.is_err(), "Expected error for nonexistent group");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_group_configuration_not_found() {
    let client = make_client().await;

    let result = client
        .get_group_configuration()
        .group("no-such-group")
        .send()
        .await;

    assert!(result.is_err(), "Expected error for nonexistent group");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_put_group_configuration_multiple_items() {
    use aws_sdk_resourcegroups::types::{GroupConfigurationItem, GroupConfigurationParameter};

    let client = make_client().await;
    create_test_group(&client, "multi-config-group").await;

    let param1a = GroupConfigurationParameter::builder()
        .name("allowed-resource-types")
        .values("AWS::EC2::Instance")
        .values("AWS::S3::Bucket")
        .build()
        .unwrap();
    let item1 = GroupConfigurationItem::builder()
        .r#type("AWS::ResourceGroups::Generic")
        .parameters(param1a)
        .build()
        .unwrap();

    let param2a = GroupConfigurationParameter::builder()
        .name("max-count")
        .values("10")
        .build()
        .unwrap();
    let item2 = GroupConfigurationItem::builder()
        .r#type("AWS::EC2::CapacityReservationPool")
        .parameters(param2a)
        .build()
        .unwrap();

    client
        .put_group_configuration()
        .group("multi-config-group")
        .configuration(item1)
        .configuration(item2)
        .send()
        .await
        .expect("put_group_configuration should succeed");

    let resp = client
        .get_group_configuration()
        .group("multi-config-group")
        .send()
        .await
        .expect("get_group_configuration should succeed");

    let gc = resp
        .group_configuration()
        .expect("should have group_configuration");
    let items = gc.configuration();
    assert_eq!(items.len(), 2, "Expected 2 configuration items");

    let types: Vec<&str> = items.iter().map(|i| i.r#type()).collect();
    assert!(
        types.contains(&"AWS::ResourceGroups::Generic"),
        "Expected AWS::ResourceGroups::Generic type"
    );
    assert!(
        types.contains(&"AWS::EC2::CapacityReservationPool"),
        "Expected AWS::EC2::CapacityReservationPool type"
    );
}

#[tokio::test]
async fn test_get_tags_not_found() {
    let client = make_client().await;

    let result = client
        .get_tags()
        .arn("arn:aws:resource-groups:us-east-1:123456789012:group/nonexistent")
        .send()
        .await;

    assert!(result.is_err(), "Expected error for nonexistent ARN");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_tag_not_found() {
    let client = make_client().await;

    let result = client
        .tag()
        .arn("arn:aws:resource-groups:us-east-1:123456789012:group/ghost-group")
        .tags("key", "value")
        .send()
        .await;

    assert!(result.is_err(), "Expected error for nonexistent ARN");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_untag_not_found() {
    let client = make_client().await;

    let result = client
        .untag()
        .arn("arn:aws:resource-groups:us-east-1:123456789012:group/ghost-group")
        .keys("somekey")
        .send()
        .await;

    assert!(result.is_err(), "Expected error for nonexistent ARN");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_start_tag_sync_task_group_not_found() {
    let client = make_client().await;

    let result = client
        .start_tag_sync_task()
        .group("ghost-group")
        .tag_key("Env")
        .tag_value("Prod")
        .role_arn("arn:aws:iam::123456789012:role/role")
        .send()
        .await;

    assert!(result.is_err(), "Expected error for nonexistent group");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_tag_sync_task_not_found() {
    let client = make_client().await;

    let result = client
        .get_tag_sync_task()
        .task_arn("arn:aws:resource-groups:us-east-1:123456789012:tag-sync-task/nonexistent")
        .send()
        .await;

    assert!(result.is_err(), "Expected error for nonexistent task");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_cancel_tag_sync_task_not_found() {
    let client = make_client().await;

    let result = client
        .cancel_tag_sync_task()
        .task_arn("arn:aws:resource-groups:us-east-1:123456789012:tag-sync-task/nonexistent")
        .send()
        .await;

    assert!(result.is_err(), "Expected error for nonexistent task");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_tag_sync_tasks_filtered_by_group() {
    use aws_sdk_resourcegroups::types::ListTagSyncTasksFilter;

    let client = make_client().await;
    create_test_group(&client, "filter-group-a").await;
    create_test_group(&client, "filter-group-b").await;

    client
        .start_tag_sync_task()
        .group("filter-group-a")
        .tag_key("Env")
        .tag_value("A")
        .role_arn("arn:aws:iam::123456789012:role/role-a")
        .send()
        .await
        .unwrap();

    client
        .start_tag_sync_task()
        .group("filter-group-b")
        .tag_key("Env")
        .tag_value("B")
        .role_arn("arn:aws:iam::123456789012:role/role-b")
        .send()
        .await
        .unwrap();

    // Filter by group name — should return only tasks for filter-group-a
    let filter = ListTagSyncTasksFilter::builder()
        .group_name("filter-group-a")
        .build();

    let resp = client
        .list_tag_sync_tasks()
        .filters(filter)
        .send()
        .await
        .expect("list_tag_sync_tasks with filter should succeed");

    let tasks = resp.tag_sync_tasks();
    assert_eq!(tasks.len(), 1, "Expected only 1 task for filter-group-a");
    assert_eq!(tasks[0].group_name().unwrap_or(""), "filter-group-a");
}

#[tokio::test]
async fn test_list_groups_contains_groups_field() {
    let client = make_client().await;
    create_test_group(&client, "list-field-group").await;

    let resp = client
        .list_groups()
        .send()
        .await
        .expect("list_groups should succeed");

    // Verify GroupIdentifiers field is populated
    let identifiers = resp.group_identifiers();
    assert!(
        !identifiers.is_empty(),
        "GroupIdentifiers should be non-empty"
    );
    let found_name = identifiers
        .iter()
        .any(|gi| gi.group_name().unwrap_or("") == "list-field-group");
    assert!(
        found_name,
        "GroupIdentifiers should contain 'list-field-group'"
    );

    // Verify Groups field is also populated (deprecated in SDK but still returned by winterbaume)
    #[allow(deprecated)]
    let groups = resp.groups();
    #[allow(deprecated)]
    let found_in_groups = groups.iter().any(|g| g.name() == "list-field-group");
    assert!(!groups.is_empty(), "Groups field should be non-empty");
    assert!(
        found_in_groups,
        "Groups field should contain 'list-field-group'"
    );
}

#[tokio::test]
async fn test_group_full_lifecycle() {
    let client = make_client().await;

    // Create
    let create_resp = client
        .create_group()
        .name("lifecycle-group")
        .description("Initial description")
        .resource_query(
            ResourceQuery::builder()
                .r#type(aws_sdk_resourcegroups::types::QueryType::TagFilters10)
                .query(r#"{"ResourceTypeFilters":["AWS::AllSupported"],"TagFilters":[{"Key":"Lifecycle","Values":["Test"]}]}"#)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_group should succeed");

    let arn = create_resp
        .group()
        .expect("should have group")
        .group_arn()
        .to_string();
    assert!(arn.contains("lifecycle-group"));

    // Get
    let get_resp = client
        .get_group()
        .group("lifecycle-group")
        .send()
        .await
        .expect("get_group should succeed");
    assert_eq!(
        get_resp.group().expect("should have group").name(),
        "lifecycle-group"
    );
    assert_eq!(
        get_resp.group().unwrap().description().unwrap_or(""),
        "Initial description"
    );

    // Update description
    let update_resp = client
        .update_group()
        .group("lifecycle-group")
        .description("Updated description")
        .send()
        .await
        .expect("update_group should succeed");
    assert_eq!(
        update_resp
            .group()
            .expect("should have group")
            .description()
            .unwrap_or(""),
        "Updated description"
    );

    // Update query
    let update_query_resp = client
        .update_group_query()
        .group("lifecycle-group")
        .resource_query(
            ResourceQuery::builder()
                .r#type(aws_sdk_resourcegroups::types::QueryType::TagFilters10)
                .query(r#"{"ResourceTypeFilters":["AWS::S3::Bucket"]}"#)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_group_query should succeed");
    assert_eq!(
        update_query_resp
            .group_query()
            .expect("should have group_query")
            .resource_query()
            .expect("should have resource_query")
            .query(),
        r#"{"ResourceTypeFilters":["AWS::S3::Bucket"]}"#
    );

    // Delete
    client
        .delete_group()
        .group("lifecycle-group")
        .send()
        .await
        .expect("delete_group should succeed");

    // Verify gone
    let result = client.get_group().group("lifecycle-group").send().await;
    assert!(
        result.is_err(),
        "Group should no longer exist after deletion"
    );

    // ARN-based get_tags should also fail
    let result2 = client.get_tags().arn(&arn).send().await;
    assert!(
        result2.is_err(),
        "get_tags on deleted group ARN should fail"
    );
}

#[tokio::test]
async fn test_create_group_with_configuration() {
    use aws_sdk_resourcegroups::types::{GroupConfigurationItem, GroupConfigurationParameter};

    let client = make_client().await;

    let param = GroupConfigurationParameter::builder()
        .name("allowed-resource-types")
        .values("AWS::EC2::CapacityReservation")
        .build()
        .unwrap();

    let config_item = GroupConfigurationItem::builder()
        .r#type("AWS::EC2::CapacityReservationPool")
        .parameters(param)
        .build()
        .unwrap();

    let resp = client
        .create_group()
        .name("config-only-group")
        .description("A group with only configuration, no resource query")
        .configuration(config_item)
        .send()
        .await
        .expect("create_group with configuration should succeed");

    let group = resp.group().expect("should have group");
    assert_eq!(group.name(), "config-only-group");
    assert!(group.group_arn().contains("config-only-group"));

    // Verify group is retrievable
    let get_resp = client
        .get_group()
        .group("config-only-group")
        .send()
        .await
        .expect("get_group should succeed");
    assert_eq!(
        get_resp.group().expect("should have group").name(),
        "config-only-group"
    );
}
