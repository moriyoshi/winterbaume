//! End-to-end scenario tests for winterbaume-keyspaces.
//!
//! Each test exercises a multi-operation use-case workflow rather than individual
//! API operations. See integration_test.rs for per-operation coverage.

use aws_sdk_keyspaces::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_keyspaces::KeyspacesService;

async fn make_client() -> aws_sdk_keyspaces::Client {
    let mock = MockAws::builder()
        .with_service(KeyspacesService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_keyspaces::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_keyspaces::Client::new(&config)
}

/// Scenario: Full table lifecycle — create keyspace, create table with schema,
/// update table capacity, verify update, then clean up.
#[tokio::test]
async fn test_table_lifecycle() {
    // Scenario: create a keyspace and a table, update the table's capacity mode,
    // verify changes are reflected, restore table from snapshot, then delete everything.

    let client = make_client().await;

    // Step 1: create keyspace
    client
        .create_keyspace()
        .keyspace_name("scenario_ks")
        .send()
        .await
        .expect("create keyspace should succeed");

    // Step 2: create table with a schema
    let schema = aws_sdk_keyspaces::types::SchemaDefinition::builder()
        .all_columns(
            aws_sdk_keyspaces::types::ColumnDefinition::builder()
                .name("user_id")
                .r#type("uuid")
                .build()
                .unwrap(),
        )
        .all_columns(
            aws_sdk_keyspaces::types::ColumnDefinition::builder()
                .name("created_at")
                .r#type("timestamp")
                .build()
                .unwrap(),
        )
        .partition_keys(
            aws_sdk_keyspaces::types::PartitionKey::builder()
                .name("user_id")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let create_resp = client
        .create_table()
        .keyspace_name("scenario_ks")
        .table_name("users")
        .schema_definition(schema)
        .capacity_specification(
            aws_sdk_keyspaces::types::CapacitySpecification::builder()
                .throughput_mode(aws_sdk_keyspaces::types::ThroughputMode::PayPerRequest)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create table should succeed");
    assert!(create_resp.resource_arn().contains("users"));

    // Step 3: update table — switch to provisioned capacity and add a column
    let update_resp = client
        .update_table()
        .keyspace_name("scenario_ks")
        .table_name("users")
        .capacity_specification(
            aws_sdk_keyspaces::types::CapacitySpecification::builder()
                .throughput_mode(aws_sdk_keyspaces::types::ThroughputMode::Provisioned)
                .read_capacity_units(10)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .add_columns(
            aws_sdk_keyspaces::types::ColumnDefinition::builder()
                .name("email")
                .r#type("text")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update table should succeed");
    assert!(update_resp.resource_arn().contains("users"));

    // Step 4: verify updated state via GetTable
    let get_resp = client
        .get_table()
        .keyspace_name("scenario_ks")
        .table_name("users")
        .send()
        .await
        .expect("get table should succeed");
    let cap = get_resp
        .capacity_specification()
        .expect("capacity_specification should be set");
    assert_eq!(
        cap.throughput_mode().as_str(),
        "PROVISIONED",
        "throughput mode should be PROVISIONED after update"
    );
    assert_eq!(
        cap.read_capacity_units(),
        Some(10),
        "read capacity should be 10"
    );

    // Step 5: restore table (copy) to a new name
    let restore_resp = client
        .restore_table()
        .source_keyspace_name("scenario_ks")
        .source_table_name("users")
        .target_keyspace_name("scenario_ks")
        .target_table_name("users_backup")
        .send()
        .await
        .expect("restore table should succeed");
    assert!(restore_resp.restored_table_arn().contains("users_backup"));

    // Step 6: list tables — both should appear
    let list_resp = client
        .list_tables()
        .keyspace_name("scenario_ks")
        .send()
        .await
        .expect("list tables should succeed");
    let table_names: Vec<_> = list_resp
        .tables()
        .iter()
        .map(|t| t.table_name().to_string())
        .collect();
    assert!(
        table_names.contains(&"users".to_string()),
        "original table should be listed"
    );
    assert!(
        table_names.contains(&"users_backup".to_string()),
        "backup table should be listed"
    );

    // Step 7: clean up
    client
        .delete_table()
        .keyspace_name("scenario_ks")
        .table_name("users_backup")
        .send()
        .await
        .expect("delete backup table should succeed");
    client
        .delete_table()
        .keyspace_name("scenario_ks")
        .table_name("users")
        .send()
        .await
        .expect("delete original table should succeed");
    client
        .delete_keyspace()
        .keyspace_name("scenario_ks")
        .send()
        .await
        .expect("delete keyspace should succeed");
}

/// Scenario: Tag management lifecycle — create resource, tag it, verify tags,
/// add more tags, remove one tag, verify final state.
#[tokio::test]
async fn test_tag_management_lifecycle() {
    let client = make_client().await;

    // Step 1: create keyspace with initial tags
    let create_resp = client
        .create_keyspace()
        .keyspace_name("tagged_ks")
        .tags(
            aws_sdk_keyspaces::types::Tag::builder()
                .key("env")
                .value("staging")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create keyspace with tags should succeed");
    let arn = create_resp.resource_arn().to_string();

    // Step 2: verify initial tags
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list initial tags should succeed");
    assert_eq!(tags_resp.tags().len(), 1);
    assert_eq!(tags_resp.tags()[0].key(), "env");

    // Step 3: add more tags
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_keyspaces::types::Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_keyspaces::types::Tag::builder()
                .key("cost-centre")
                .value("infra")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags after adding should succeed");
    assert_eq!(tags_resp.tags().len(), 3, "should have 3 tags now");

    // Step 4: remove one tag
    client
        .untag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_keyspaces::types::Tag::builder()
                .key("env")
                .value("staging")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags after removal should succeed");
    assert_eq!(
        tags_resp.tags().len(),
        2,
        "should have 2 tags after removal"
    );
    let keys: Vec<_> = tags_resp
        .tags()
        .iter()
        .map(|t| t.key().to_string())
        .collect();
    assert!(
        !keys.contains(&"env".to_string()),
        "removed tag should not be present"
    );

    // Step 5: clean up
    client
        .delete_keyspace()
        .keyspace_name("tagged_ks")
        .send()
        .await
        .expect("delete keyspace should succeed");
}

/// Scenario: User-defined type lifecycle — create keyspace, create type,
/// get type, list types, delete type.
#[tokio::test]
async fn test_user_defined_type_lifecycle() {
    let client = make_client().await;

    // Step 1: create keyspace
    client
        .create_keyspace()
        .keyspace_name("udt_ks")
        .send()
        .await
        .expect("create keyspace should succeed");

    // Step 2: create a user-defined type
    let create_resp = client
        .create_type()
        .keyspace_name("udt_ks")
        .type_name("address")
        .field_definitions(
            aws_sdk_keyspaces::types::FieldDefinition::builder()
                .name("street")
                .r#type("text")
                .build()
                .unwrap(),
        )
        .field_definitions(
            aws_sdk_keyspaces::types::FieldDefinition::builder()
                .name("city")
                .r#type("text")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_type should succeed");
    assert_eq!(create_resp.type_name(), "address");

    // Step 3: get the type
    let get_resp = client
        .get_type()
        .keyspace_name("udt_ks")
        .type_name("address")
        .send()
        .await
        .expect("get_type should succeed");
    assert_eq!(get_resp.keyspace_name(), "udt_ks");
    let fields = get_resp.field_definitions();
    assert_eq!(fields.len(), 2, "should have 2 field definitions");

    // Step 4: list types
    let list_resp = client
        .list_types()
        .keyspace_name("udt_ks")
        .send()
        .await
        .expect("list_types should succeed");
    assert!(
        list_resp.types().contains(&"address".to_string()),
        "address type should appear in list"
    );

    // Step 5: delete the type
    let delete_resp = client
        .delete_type()
        .keyspace_name("udt_ks")
        .type_name("address")
        .send()
        .await
        .expect("delete_type should succeed");
    assert_eq!(delete_resp.type_name(), "address");

    // Verify it no longer exists
    let err = client
        .get_type()
        .keyspace_name("udt_ks")
        .type_name("address")
        .send()
        .await
        .expect_err("get_type after delete should fail");
    assert!(
        format!("{err:?}").contains("ResourceNotFound"),
        "should get ResourceNotFoundException"
    );

    // Step 6: clean up
    client
        .delete_keyspace()
        .keyspace_name("udt_ks")
        .send()
        .await
        .expect("delete keyspace should succeed");
}
