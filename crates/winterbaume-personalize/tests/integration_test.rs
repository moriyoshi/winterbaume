use aws_sdk_personalize::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_personalize::PersonalizeService;

async fn make_client() -> aws_sdk_personalize::Client {
    let mock = MockAws::builder()
        .with_service(PersonalizeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_personalize::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_personalize::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_dataset_group() {
    let client = make_client().await;

    let create_resp = client
        .create_dataset_group()
        .name("my-dataset-group")
        .send()
        .await
        .expect("create_dataset_group should succeed");

    let arn = create_resp
        .dataset_group_arn()
        .expect("should have dataset_group_arn");
    assert!(arn.contains("my-dataset-group"));

    let describe_resp = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .expect("describe_dataset_group should succeed");

    let dg = describe_resp
        .dataset_group()
        .expect("should have dataset_group");
    assert_eq!(dg.name().unwrap(), "my-dataset-group");
    assert_eq!(dg.status().unwrap(), "ACTIVE");
}

#[tokio::test]
async fn test_create_dataset_group_with_domain() {
    let client = make_client().await;

    let create_resp = client
        .create_dataset_group()
        .name("domain-group")
        .domain(aws_sdk_personalize::types::Domain::Ecommerce)
        .send()
        .await
        .expect("create_dataset_group with domain should succeed");

    let arn = create_resp.dataset_group_arn().unwrap();

    let describe_resp = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .expect("describe should succeed");

    let dg = describe_resp.dataset_group().unwrap();
    assert_eq!(
        dg.domain(),
        Some(&aws_sdk_personalize::types::Domain::Ecommerce)
    );
}

#[tokio::test]
async fn test_list_dataset_groups() {
    let client = make_client().await;

    for name in ["group-a", "group-b"] {
        client
            .create_dataset_group()
            .name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_dataset_groups()
        .send()
        .await
        .expect("list_dataset_groups should succeed");

    assert_eq!(resp.dataset_groups().len(), 2);
}

#[tokio::test]
async fn test_delete_dataset_group() {
    let client = make_client().await;

    let create_resp = client
        .create_dataset_group()
        .name("delete-me")
        .send()
        .await
        .unwrap();

    let arn = create_resp.dataset_group_arn().unwrap();

    client
        .delete_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_duplicate_dataset_group() {
    let client = make_client().await;

    client
        .create_dataset_group()
        .name("unique-name")
        .send()
        .await
        .unwrap();

    let result = client
        .create_dataset_group()
        .name("unique-name")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Personalize
// ============================================================================

#[tokio::test]
async fn test_describe_dataset_group_not_found() {
    let client = make_client().await;

    let err = client
        .describe_dataset_group()
        .dataset_group_arn("arn:aws:personalize:us-east-1:123456789012:dataset-group/nonexistent")
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
async fn test_delete_dataset_group_not_found() {
    let client = make_client().await;

    let err = client
        .delete_dataset_group()
        .dataset_group_arn("arn:aws:personalize:us-east-1:123456789012:dataset-group/nonexistent")
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
async fn test_dataset_group_arn_format() {
    let client = make_client().await;

    let resp = client
        .create_dataset_group()
        .name("arn-format-test")
        .send()
        .await
        .unwrap();

    let arn = resp.dataset_group_arn().unwrap();
    assert!(
        arn.starts_with("arn:aws:personalize:"),
        "ARN should start with arn:aws:personalize:, got: {arn}"
    );
    assert!(
        arn.contains(":dataset-group/"),
        "ARN should contain ':dataset-group/', got: {arn}"
    );
    assert!(
        arn.contains("arn-format-test"),
        "ARN should contain the dataset group name, got: {arn}"
    );
}

#[tokio::test]
async fn test_dataset_group_timestamps() {
    let client = make_client().await;

    let create_resp = client
        .create_dataset_group()
        .name("timestamps-test")
        .send()
        .await
        .unwrap();

    let arn = create_resp.dataset_group_arn().unwrap();

    let describe_resp = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .unwrap();

    let dg = describe_resp.dataset_group().unwrap();
    assert!(
        dg.creation_date_time().is_some(),
        "creationDateTime should be present"
    );
    assert!(
        dg.last_updated_date_time().is_some(),
        "lastUpdatedDateTime should be present"
    );
}

#[tokio::test]
async fn test_dataset_group_with_role_arn() {
    let client = make_client().await;

    let role_arn = "arn:aws:iam::123456789012:role/PersonalizeRole";

    let create_resp = client
        .create_dataset_group()
        .name("role-arn-test")
        .role_arn(role_arn)
        .send()
        .await
        .unwrap();

    let arn = create_resp.dataset_group_arn().unwrap();

    let describe_resp = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .unwrap();

    let dg = describe_resp.dataset_group().unwrap();
    assert_eq!(
        dg.role_arn(),
        Some(role_arn),
        "roleArn should be stored and returned"
    );
}

#[tokio::test]
async fn test_list_dataset_groups_empty() {
    let client = make_client().await;

    let resp = client.list_dataset_groups().send().await.unwrap();
    assert_eq!(
        resp.dataset_groups().len(),
        0,
        "Fresh state should have no dataset groups"
    );
}

#[tokio::test]
async fn test_dataset_group_lifecycle() {
    let client = make_client().await;

    // Create
    let create_resp = client
        .create_dataset_group()
        .name("lifecycle-group")
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp.dataset_group_arn().unwrap().to_string();

    // Describe
    let describe_resp = client
        .describe_dataset_group()
        .dataset_group_arn(&arn)
        .send()
        .await
        .expect("describe should succeed");

    let dg = describe_resp.dataset_group().unwrap();
    assert_eq!(dg.name().unwrap(), "lifecycle-group");
    assert_eq!(dg.status().unwrap(), "ACTIVE");

    // Delete
    client
        .delete_dataset_group()
        .dataset_group_arn(&arn)
        .send()
        .await
        .expect("delete should succeed");

    // Describe after delete should fail
    let err = client
        .describe_dataset_group()
        .dataset_group_arn(&arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException after delete, got: {err_str}"
    );
}

// ============================================================================
// Schema tests
// ============================================================================

const SAMPLE_SCHEMA: &str = r#"{
  "type": "record",
  "name": "Interactions",
  "namespace": "com.amazonaws.personalize.schema",
  "fields": [
    {"name": "USER_ID", "type": "string"},
    {"name": "ITEM_ID", "type": "string"},
    {"name": "TIMESTAMP", "type": "long"}
  ],
  "version": "1.0"
}"#;

async fn create_test_schema(client: &aws_sdk_personalize::Client, name: &str) -> String {
    client
        .create_schema()
        .name(name)
        .schema(SAMPLE_SCHEMA)
        .send()
        .await
        .unwrap()
        .schema_arn()
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn test_create_and_describe_schema() {
    let client = make_client().await;

    let arn = create_test_schema(&client, "my-schema").await;
    assert!(arn.contains("my-schema"), "ARN should contain schema name");

    let describe_resp = client
        .describe_schema()
        .schema_arn(&arn)
        .send()
        .await
        .expect("describe_schema should succeed");

    let schema = describe_resp.schema().expect("should have schema");
    assert_eq!(schema.name().unwrap(), "my-schema");
    assert!(
        schema.schema().is_some(),
        "schema body should be present in response"
    );
}

#[tokio::test]
async fn test_create_schema_with_domain() {
    let client = make_client().await;

    let resp = client
        .create_schema()
        .name("vod-schema")
        .schema(SAMPLE_SCHEMA)
        .domain(aws_sdk_personalize::types::Domain::VideoOnDemand)
        .send()
        .await
        .expect("create_schema with domain should succeed");

    let arn = resp.schema_arn().unwrap();

    let describe_resp = client
        .describe_schema()
        .schema_arn(arn)
        .send()
        .await
        .unwrap();

    let schema = describe_resp.schema().unwrap();
    assert_eq!(
        schema.domain(),
        Some(&aws_sdk_personalize::types::Domain::VideoOnDemand),
        "domain should be VIDEO_ON_DEMAND"
    );
}

#[tokio::test]
async fn test_list_schemas() {
    let client = make_client().await;

    for name in ["schema-x", "schema-y"] {
        create_test_schema(&client, name).await;
    }

    let resp = client
        .list_schemas()
        .send()
        .await
        .expect("list_schemas should succeed");

    assert_eq!(resp.schemas().len(), 2, "Should list 2 schemas");
}

#[tokio::test]
async fn test_list_schemas_empty() {
    let client = make_client().await;

    let resp = client.list_schemas().send().await.unwrap();
    assert_eq!(
        resp.schemas().len(),
        0,
        "Fresh state should have no schemas"
    );
}

#[tokio::test]
async fn test_delete_schema() {
    let client = make_client().await;

    let arn = create_test_schema(&client, "delete-schema").await;

    client
        .delete_schema()
        .schema_arn(&arn)
        .send()
        .await
        .expect("delete_schema should succeed");

    let err = client
        .describe_schema()
        .schema_arn(&arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException after delete, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_duplicate_schema() {
    let client = make_client().await;

    create_test_schema(&client, "dupe-schema").await;

    let err = client
        .create_schema()
        .name("dupe-schema")
        .schema(SAMPLE_SCHEMA)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceAlreadyExistsException"),
        "Expected ResourceAlreadyExistsException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_schema_not_found() {
    let client = make_client().await;

    let err = client
        .describe_schema()
        .schema_arn("arn:aws:personalize:us-east-1:123456789012:schema/nonexistent")
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
async fn test_delete_schema_not_found() {
    let client = make_client().await;

    let err = client
        .delete_schema()
        .schema_arn("arn:aws:personalize:us-east-1:123456789012:schema/nonexistent")
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
async fn test_schema_arn_format() {
    let client = make_client().await;

    let arn = create_test_schema(&client, "arn-schema").await;

    assert!(
        arn.starts_with("arn:aws:personalize:"),
        "Schema ARN should start with arn:aws:personalize:, got: {arn}"
    );
    assert!(
        arn.contains(":schema/"),
        "Schema ARN should contain ':schema/', got: {arn}"
    );
    assert!(
        arn.contains("arn-schema"),
        "Schema ARN should contain the schema name, got: {arn}"
    );
}

#[tokio::test]
async fn test_schema_content_preserved() {
    let client = make_client().await;

    let arn = create_test_schema(&client, "content-schema").await;

    let describe_resp = client
        .describe_schema()
        .schema_arn(&arn)
        .send()
        .await
        .unwrap();

    let schema = describe_resp.schema().unwrap();
    assert_eq!(
        schema.schema().unwrap(),
        SAMPLE_SCHEMA,
        "Schema body should be preserved verbatim"
    );
}

#[tokio::test]
async fn test_schema_timestamps() {
    let client = make_client().await;

    let arn = create_test_schema(&client, "timestamps-schema").await;

    let describe_resp = client
        .describe_schema()
        .schema_arn(&arn)
        .send()
        .await
        .unwrap();

    let schema = describe_resp.schema().unwrap();
    assert!(
        schema.creation_date_time().is_some(),
        "creationDateTime should be present"
    );
    assert!(
        schema.last_updated_date_time().is_some(),
        "lastUpdatedDateTime should be present"
    );
}

#[tokio::test]
async fn test_schema_lifecycle() {
    let client = make_client().await;

    // Create
    let arn = create_test_schema(&client, "lifecycle-schema").await;

    // Describe
    let describe_resp = client
        .describe_schema()
        .schema_arn(&arn)
        .send()
        .await
        .expect("describe should succeed");

    let schema = describe_resp.schema().unwrap();
    assert_eq!(schema.name().unwrap(), "lifecycle-schema");
    assert!(schema.schema().is_some());

    // Delete
    client
        .delete_schema()
        .schema_arn(&arn)
        .send()
        .await
        .expect("delete should succeed");

    // Describe after delete should fail
    let err = client
        .describe_schema()
        .schema_arn(&arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException after delete, got: {err_str}"
    );
}
