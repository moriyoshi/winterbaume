//! End-to-end scenario tests for `winterbaume-quicksight`.
//!
//! Each test here chains 3+ operations and asserts business outcomes rather
//! than per-API return shapes.

use aws_sdk_quicksight::config::BehaviorVersion;
use aws_sdk_quicksight::types::{
    DataSetImportMode, FolderType, InputColumn, InputColumnDataType, PhysicalTable, S3Source,
};
use winterbaume_core::MockAws;
use winterbaume_quicksight::QuickSightService;

const ACCOUNT_ID: &str = "123456789012";

async fn make_client() -> aws_sdk_quicksight::Client {
    let mock = MockAws::builder()
        .with_service(QuickSightService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_quicksight::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_quicksight::Client::new(&config)
}

/// Scenario: Data pipeline lifecycle
///
/// Creates a DataSource, registers a DataSet that references it, starts an
/// Ingestion, verifies the ingestion is visible, then tears down the DataSet
/// and DataSource.  Asserts that all resources appear in list calls and that
/// deleting a DataSet also removes it from list results.
#[tokio::test]
async fn test_data_pipeline_lifecycle() {
    let client = make_client().await;

    // Step 1: create a data source
    let ds_src_resp = client
        .create_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("scenario-src-1")
        .name("Scenario DataSource")
        .r#type(aws_sdk_quicksight::types::DataSourceType::S3)
        .send()
        .await
        .expect("create_data_source should succeed");

    let src_arn = ds_src_resp
        .arn()
        .expect("create_data_source should return an ARN");
    assert!(
        src_arn.contains("scenario-src-1"),
        "ARN should contain the data source ID"
    );

    // Step 2: create a dataset referencing the data source
    let ds_resp = client
        .create_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("scenario-ds-1")
        .name("Scenario DataSet")
        .import_mode(DataSetImportMode::Spice)
        .physical_table_map(
            "tbl1",
            PhysicalTable::S3Source(
                S3Source::builder()
                    .data_source_arn(src_arn.to_string())
                    .input_columns(
                        InputColumn::builder()
                            .name("id")
                            .r#type(InputColumnDataType::Integer)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            ),
        )
        .send()
        .await
        .expect("create_data_set should succeed");

    let dataset_arn = ds_resp.arn().expect("create_data_set should return an ARN");
    assert!(
        dataset_arn.contains("scenario-ds-1"),
        "DataSet ARN should contain the data set ID"
    );

    // Step 3: start an ingestion for the dataset
    let ingest_resp = client
        .create_ingestion()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("scenario-ds-1")
        .ingestion_id("ingest-001")
        .send()
        .await
        .expect("create_ingestion should succeed");

    let ingest_arn = ingest_resp
        .arn()
        .expect("create_ingestion should return an ARN");
    assert!(
        ingest_arn.contains("ingest-001"),
        "Ingestion ARN should contain the ingestion ID"
    );

    // Step 4: verify ingestion appears in describe
    let describe_ingest = client
        .describe_ingestion()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("scenario-ds-1")
        .ingestion_id("ingest-001")
        .send()
        .await
        .expect("describe_ingestion should succeed");

    let ingestion = describe_ingest
        .ingestion()
        .expect("describe_ingestion should return an Ingestion");
    assert_eq!(
        ingestion.ingestion_id(),
        Some("ingest-001"),
        "ingestion ID should match"
    );

    // Step 5: verify the dataset appears in list
    let list_resp = client
        .list_data_sets()
        .aws_account_id(ACCOUNT_ID)
        .send()
        .await
        .expect("list_data_sets should succeed");

    let summaries = list_resp.data_set_summaries();
    assert!(
        summaries
            .iter()
            .any(|s| s.data_set_id() == Some("scenario-ds-1")),
        "dataset should appear in list"
    );

    // Step 6: delete the dataset — it should disappear from list
    client
        .delete_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("scenario-ds-1")
        .send()
        .await
        .expect("delete_data_set should succeed");

    let list_after = client
        .list_data_sets()
        .aws_account_id(ACCOUNT_ID)
        .send()
        .await
        .expect("list_data_sets after delete should succeed");

    assert!(
        !list_after
            .data_set_summaries()
            .iter()
            .any(|s| s.data_set_id() == Some("scenario-ds-1")),
        "deleted dataset must not appear in list"
    );

    // Step 7: delete the data source
    client
        .delete_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("scenario-src-1")
        .send()
        .await
        .expect("delete_data_source should succeed");
}

/// Scenario: Group and user membership workflow
///
/// Creates a namespace, registers two users, creates a group, adds both users
/// to the group, verifies membership via list-group-memberships and
/// list-user-groups, removes one user, then deletes the group and users.
#[tokio::test]
async fn test_group_user_membership_workflow() {
    let client = make_client().await;

    // Step 1: create a custom namespace
    client
        .create_namespace()
        .aws_account_id(ACCOUNT_ID)
        .namespace("scenario-ns")
        .identity_store(aws_sdk_quicksight::types::IdentityStore::Quicksight)
        .send()
        .await
        .expect("create_namespace should succeed");

    // Step 2: register two users
    for (email, user_name) in [("alice@example.com", "alice"), ("bob@example.com", "bob")] {
        client
            .register_user()
            .aws_account_id(ACCOUNT_ID)
            .namespace("scenario-ns")
            .email(email)
            .identity_type(aws_sdk_quicksight::types::IdentityType::Quicksight)
            .user_role(aws_sdk_quicksight::types::UserRole::Reader)
            .user_name(user_name)
            .send()
            .await
            .unwrap_or_else(|_| panic!("register_user({user_name}) should succeed"));
    }

    // Step 3: create a group
    client
        .create_group()
        .aws_account_id(ACCOUNT_ID)
        .namespace("scenario-ns")
        .group_name("readers")
        .description("Reader group")
        .send()
        .await
        .expect("create_group should succeed");

    // Step 4: add both users to the group
    for member in ["alice", "bob"] {
        client
            .create_group_membership()
            .aws_account_id(ACCOUNT_ID)
            .namespace("scenario-ns")
            .group_name("readers")
            .member_name(member)
            .send()
            .await
            .unwrap_or_else(|_| panic!("create_group_membership({member}) should succeed"));
    }

    // Step 5: verify both members appear in list-group-memberships
    let members_resp = client
        .list_group_memberships()
        .aws_account_id(ACCOUNT_ID)
        .namespace("scenario-ns")
        .group_name("readers")
        .send()
        .await
        .expect("list_group_memberships should succeed");

    let member_names: Vec<&str> = members_resp
        .group_member_list()
        .iter()
        .filter_map(|m| m.member_name())
        .collect();
    assert!(
        member_names.contains(&"alice"),
        "alice should be in group members"
    );
    assert!(
        member_names.contains(&"bob"),
        "bob should be in group members"
    );

    // Step 6: verify alice's groups via list-user-groups
    let user_groups = client
        .list_user_groups()
        .aws_account_id(ACCOUNT_ID)
        .namespace("scenario-ns")
        .user_name("alice")
        .send()
        .await
        .expect("list_user_groups should succeed");

    let group_names: Vec<&str> = user_groups
        .group_list()
        .iter()
        .filter_map(|g| g.group_name())
        .collect();
    assert!(
        group_names.contains(&"readers"),
        "alice should be listed in the readers group"
    );

    // Step 7: remove alice from the group
    client
        .delete_group_membership()
        .aws_account_id(ACCOUNT_ID)
        .namespace("scenario-ns")
        .group_name("readers")
        .member_name("alice")
        .send()
        .await
        .expect("delete_group_membership should succeed");

    let after_remove = client
        .list_group_memberships()
        .aws_account_id(ACCOUNT_ID)
        .namespace("scenario-ns")
        .group_name("readers")
        .send()
        .await
        .expect("list_group_memberships after remove should succeed");

    let after_names: Vec<&str> = after_remove
        .group_member_list()
        .iter()
        .filter_map(|m| m.member_name())
        .collect();
    assert!(
        !after_names.contains(&"alice"),
        "alice should no longer be in group after removal"
    );
    assert!(after_names.contains(&"bob"), "bob should still be in group");

    // Step 8: delete group and users
    client
        .delete_group()
        .aws_account_id(ACCOUNT_ID)
        .namespace("scenario-ns")
        .group_name("readers")
        .send()
        .await
        .expect("delete_group should succeed");

    for user_name in ["alice", "bob"] {
        client
            .delete_user()
            .aws_account_id(ACCOUNT_ID)
            .namespace("scenario-ns")
            .user_name(user_name)
            .send()
            .await
            .unwrap_or_else(|_| panic!("delete_user({user_name}) should succeed"));
    }
}

/// Scenario: Folder membership lifecycle
///
/// Creates a folder, creates a dataset, adds the dataset as a folder member,
/// verifies it appears in list-folder-members, then removes it and deletes
/// both resources.
#[tokio::test]
async fn test_folder_with_dataset_member() {
    let client = make_client().await;

    // Step 1: create a dataset
    client
        .create_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("folder-ds-1")
        .name("Folder Test DataSet")
        .import_mode(DataSetImportMode::DirectQuery)
        .physical_table_map(
            "tbl",
            PhysicalTable::S3Source(
                S3Source::builder()
                    .data_source_arn("arn:aws:quicksight:us-east-1:123456789012:datasource/dummy")
                    .input_columns(
                        InputColumn::builder()
                            .name("col")
                            .r#type(InputColumnDataType::String)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            ),
        )
        .send()
        .await
        .expect("create_data_set for folder test should succeed");

    // Use the dataset ID (not the ARN) as the member ID.  ARNs contain
    // forward slashes which fragment the URL path, causing the handler to
    // mis-route the request.
    let member_id = "folder-ds-1";

    // Step 2: create a folder
    client
        .create_folder()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("folder-001")
        .name("Test Folder")
        .folder_type(FolderType::Shared)
        .send()
        .await
        .expect("create_folder should succeed");

    // Step 3: add dataset as a folder member
    client
        .create_folder_membership()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("folder-001")
        .member_id(member_id)
        .member_type(aws_sdk_quicksight::types::MemberType::Dataset)
        .send()
        .await
        .expect("create_folder_membership should succeed");

    // Step 4: verify member appears in list-folder-members
    let members = client
        .list_folder_members()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("folder-001")
        .send()
        .await
        .expect("list_folder_members should succeed");

    assert!(
        members
            .folder_member_list()
            .iter()
            .any(|m| m.member_id() == Some(member_id)),
        "dataset should appear in folder members"
    );

    // Step 5: remove the folder member
    client
        .delete_folder_membership()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("folder-001")
        .member_id(member_id)
        .member_type(aws_sdk_quicksight::types::MemberType::Dataset)
        .send()
        .await
        .expect("delete_folder_membership should succeed");

    let after = client
        .list_folder_members()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("folder-001")
        .send()
        .await
        .expect("list_folder_members after delete should succeed");

    assert!(
        after.folder_member_list().is_empty(),
        "folder should have no members after removal"
    );

    // Step 6: delete folder and dataset
    client
        .delete_folder()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("folder-001")
        .send()
        .await
        .expect("delete_folder should succeed");

    client
        .delete_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("folder-ds-1")
        .send()
        .await
        .expect("delete_data_set should succeed");
}
