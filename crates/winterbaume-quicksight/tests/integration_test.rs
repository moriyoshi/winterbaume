use aws_sdk_quicksight::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_quicksight::QuickSightService;

async fn make_quicksight_client() -> aws_sdk_quicksight::Client {
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

const ACCOUNT_ID: &str = "123456789012";

// ==== DataSet tests (existing) ====

#[tokio::test]
async fn test_create_and_describe_data_set() {
    let client = make_quicksight_client().await;

    let create_resp = client
        .create_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ds-001")
        .name("Test DataSet")
        .import_mode(aws_sdk_quicksight::types::DataSetImportMode::Spice)
        .physical_table_map(
            "table1",
            aws_sdk_quicksight::types::PhysicalTable::S3Source(
                aws_sdk_quicksight::types::S3Source::builder()
                    .data_source_arn("arn:aws:quicksight:us-east-1:123456789012:datasource/src1")
                    .input_columns(
                        aws_sdk_quicksight::types::InputColumn::builder()
                            .name("col1")
                            .r#type(aws_sdk_quicksight::types::InputColumnDataType::String)
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

    assert_eq!(create_resp.data_set_id(), Some("ds-001"));
    assert!(create_resp.arn().is_some());

    let describe_resp = client
        .describe_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ds-001")
        .send()
        .await
        .expect("describe_data_set should succeed");

    let ds = describe_resp.data_set().expect("should have data set");
    assert_eq!(ds.data_set_id(), Some("ds-001"));
    assert_eq!(ds.name(), Some("Test DataSet"));
}

#[tokio::test]
async fn test_list_data_sets() {
    let client = make_quicksight_client().await;

    for (id, name) in [("ds-a", "DS A"), ("ds-b", "DS B"), ("ds-c", "DS C")] {
        client
            .create_data_set()
            .aws_account_id(ACCOUNT_ID)
            .data_set_id(id)
            .name(name)
            .import_mode(aws_sdk_quicksight::types::DataSetImportMode::DirectQuery)
            .physical_table_map(
                "table1",
                aws_sdk_quicksight::types::PhysicalTable::S3Source(
                    aws_sdk_quicksight::types::S3Source::builder()
                        .data_source_arn(
                            "arn:aws:quicksight:us-east-1:123456789012:datasource/src1",
                        )
                        .input_columns(
                            aws_sdk_quicksight::types::InputColumn::builder()
                                .name("col1")
                                .r#type(aws_sdk_quicksight::types::InputColumnDataType::String)
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                ),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_data_sets()
        .aws_account_id(ACCOUNT_ID)
        .send()
        .await
        .expect("list_data_sets should succeed");

    assert_eq!(resp.data_set_summaries().len(), 3);
}

#[tokio::test]
async fn test_delete_data_set() {
    let client = make_quicksight_client().await;

    client
        .create_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ds-del")
        .name("To Delete")
        .import_mode(aws_sdk_quicksight::types::DataSetImportMode::DirectQuery)
        .physical_table_map(
            "table1",
            aws_sdk_quicksight::types::PhysicalTable::S3Source(
                aws_sdk_quicksight::types::S3Source::builder()
                    .data_source_arn("arn:aws:quicksight:us-east-1:123456789012:datasource/src1")
                    .input_columns(
                        aws_sdk_quicksight::types::InputColumn::builder()
                            .name("col1")
                            .r#type(aws_sdk_quicksight::types::InputColumnDataType::String)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            ),
        )
        .send()
        .await
        .unwrap();

    let del_resp = client
        .delete_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ds-del")
        .send()
        .await
        .expect("delete_data_set should succeed");

    assert_eq!(del_resp.data_set_id(), Some("ds-del"));

    let result = client
        .describe_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ds-del")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_data_set() {
    let client = make_quicksight_client().await;

    client
        .create_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ds-upd")
        .name("Original Name")
        .import_mode(aws_sdk_quicksight::types::DataSetImportMode::DirectQuery)
        .physical_table_map(
            "table1",
            aws_sdk_quicksight::types::PhysicalTable::S3Source(
                aws_sdk_quicksight::types::S3Source::builder()
                    .data_source_arn("arn:aws:quicksight:us-east-1:123456789012:datasource/src1")
                    .input_columns(
                        aws_sdk_quicksight::types::InputColumn::builder()
                            .name("col1")
                            .r#type(aws_sdk_quicksight::types::InputColumnDataType::String)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            ),
        )
        .send()
        .await
        .unwrap();

    let upd_resp = client
        .update_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ds-upd")
        .name("Updated Name")
        .import_mode(aws_sdk_quicksight::types::DataSetImportMode::Spice)
        .physical_table_map(
            "table1",
            aws_sdk_quicksight::types::PhysicalTable::S3Source(
                aws_sdk_quicksight::types::S3Source::builder()
                    .data_source_arn("arn:aws:quicksight:us-east-1:123456789012:datasource/src1")
                    .input_columns(
                        aws_sdk_quicksight::types::InputColumn::builder()
                            .name("col1")
                            .r#type(aws_sdk_quicksight::types::InputColumnDataType::String)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            ),
        )
        .send()
        .await
        .expect("update_data_set should succeed");

    assert_eq!(upd_resp.data_set_id(), Some("ds-upd"));

    let describe_resp = client
        .describe_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ds-upd")
        .send()
        .await
        .expect("describe should succeed after update");

    let ds = describe_resp.data_set().expect("should have data set");
    assert_eq!(ds.name(), Some("Updated Name"));
}

#[tokio::test]
async fn test_describe_nonexistent_data_set_fails() {
    let client = make_quicksight_client().await;

    let result = client
        .describe_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("nonexistent-ds")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_data_set_fails() {
    let client = make_quicksight_client().await;

    let result = client
        .delete_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("nonexistent-ds")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_duplicate_data_set_fails() {
    let client = make_quicksight_client().await;

    client
        .create_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ds-dup")
        .name("First")
        .import_mode(aws_sdk_quicksight::types::DataSetImportMode::DirectQuery)
        .physical_table_map(
            "table1",
            aws_sdk_quicksight::types::PhysicalTable::S3Source(
                aws_sdk_quicksight::types::S3Source::builder()
                    .data_source_arn("arn:aws:quicksight:us-east-1:123456789012:datasource/src1")
                    .input_columns(
                        aws_sdk_quicksight::types::InputColumn::builder()
                            .name("col1")
                            .r#type(aws_sdk_quicksight::types::InputColumnDataType::String)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            ),
        )
        .send()
        .await
        .unwrap();

    let result = client
        .create_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ds-dup")
        .name("Second")
        .import_mode(aws_sdk_quicksight::types::DataSetImportMode::DirectQuery)
        .physical_table_map(
            "table1",
            aws_sdk_quicksight::types::PhysicalTable::S3Source(
                aws_sdk_quicksight::types::S3Source::builder()
                    .data_source_arn("arn:aws:quicksight:us-east-1:123456789012:datasource/src1")
                    .input_columns(
                        aws_sdk_quicksight::types::InputColumn::builder()
                            .name("col1")
                            .r#type(aws_sdk_quicksight::types::InputColumnDataType::String)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            ),
        )
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_data_sets_empty() {
    let client = make_quicksight_client().await;

    let resp = client
        .list_data_sets()
        .aws_account_id(ACCOUNT_ID)
        .send()
        .await
        .expect("list_data_sets should succeed");

    assert!(resp.data_set_summaries().is_empty());
}

// ==== DataSource tests ====

#[tokio::test]
async fn test_create_and_describe_data_source() {
    let client = make_quicksight_client().await;

    let create_resp = client
        .create_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("dsrc-001")
        .name("Test DataSource")
        .r#type(aws_sdk_quicksight::types::DataSourceType::S3)
        .send()
        .await
        .expect("create_data_source should succeed");

    assert_eq!(create_resp.data_source_id(), Some("dsrc-001"));
    assert!(create_resp.arn().is_some());

    let describe_resp = client
        .describe_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("dsrc-001")
        .send()
        .await
        .expect("describe_data_source should succeed");

    let ds = describe_resp
        .data_source()
        .expect("should have data source");
    assert_eq!(ds.data_source_id(), Some("dsrc-001"));
    assert_eq!(ds.name(), Some("Test DataSource"));
}

#[tokio::test]
async fn test_update_data_source() {
    let client = make_quicksight_client().await;

    client
        .create_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("dsrc-upd")
        .name("Original")
        .r#type(aws_sdk_quicksight::types::DataSourceType::S3)
        .send()
        .await
        .unwrap();

    let upd_resp = client
        .update_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("dsrc-upd")
        .name("Updated")
        .send()
        .await
        .expect("update_data_source should succeed");

    assert_eq!(upd_resp.data_source_id(), Some("dsrc-upd"));

    let describe_resp = client
        .describe_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("dsrc-upd")
        .send()
        .await
        .unwrap();

    let ds = describe_resp.data_source().unwrap();
    assert_eq!(ds.name(), Some("Updated"));
}

#[tokio::test]
async fn test_delete_data_source() {
    let client = make_quicksight_client().await;

    client
        .create_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("dsrc-del")
        .name("To Delete")
        .r#type(aws_sdk_quicksight::types::DataSourceType::S3)
        .send()
        .await
        .unwrap();

    let del_resp = client
        .delete_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("dsrc-del")
        .send()
        .await
        .expect("delete_data_source should succeed");

    assert_eq!(del_resp.data_source_id(), Some("dsrc-del"));

    let result = client
        .describe_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("dsrc-del")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_data_sources() {
    let client = make_quicksight_client().await;

    client
        .create_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("dsrc-a")
        .name("DS A")
        .r#type(aws_sdk_quicksight::types::DataSourceType::S3)
        .send()
        .await
        .unwrap();

    client
        .create_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("dsrc-b")
        .name("DS B")
        .r#type(aws_sdk_quicksight::types::DataSourceType::S3)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_data_sources()
        .aws_account_id(ACCOUNT_ID)
        .send()
        .await
        .expect("list_data_sources should succeed");

    assert_eq!(resp.data_sources().len(), 2);
}

// ==== Dashboard tests ====

#[tokio::test]
async fn test_create_and_describe_dashboard() {
    let client = make_quicksight_client().await;

    let create_resp = client
        .create_dashboard()
        .aws_account_id(ACCOUNT_ID)
        .dashboard_id("dash-001")
        .name("Test Dashboard")
        .source_entity(
            aws_sdk_quicksight::types::DashboardSourceEntity::builder()
                .source_template(
                    aws_sdk_quicksight::types::DashboardSourceTemplate::builder()
                        .arn("arn:aws:quicksight:us-east-1:123456789012:template/tmpl1")
                        .data_set_references(
                            aws_sdk_quicksight::types::DataSetReference::builder()
                                .data_set_arn(
                                    "arn:aws:quicksight:us-east-1:123456789012:dataset/ds1",
                                )
                                .data_set_placeholder("ds1")
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("create_dashboard should succeed");

    assert_eq!(create_resp.dashboard_id(), Some("dash-001"));
    assert!(create_resp.arn().is_some());

    let describe_resp = client
        .describe_dashboard()
        .aws_account_id(ACCOUNT_ID)
        .dashboard_id("dash-001")
        .send()
        .await
        .expect("describe_dashboard should succeed");

    let db = describe_resp.dashboard().expect("should have dashboard");
    assert_eq!(db.dashboard_id(), Some("dash-001"));
    assert_eq!(db.name(), Some("Test Dashboard"));
}

#[tokio::test]
async fn test_list_dashboards() {
    let client = make_quicksight_client().await;

    for (id, name) in [("dash-a", "A"), ("dash-b", "B")] {
        client
            .create_dashboard()
            .aws_account_id(ACCOUNT_ID)
            .dashboard_id(id)
            .name(name)
            .source_entity(
                aws_sdk_quicksight::types::DashboardSourceEntity::builder()
                    .source_template(
                        aws_sdk_quicksight::types::DashboardSourceTemplate::builder()
                            .arn("arn:aws:quicksight:us-east-1:123456789012:template/tmpl1")
                            .data_set_references(
                                aws_sdk_quicksight::types::DataSetReference::builder()
                                    .data_set_arn(
                                        "arn:aws:quicksight:us-east-1:123456789012:dataset/ds1",
                                    )
                                    .data_set_placeholder("ds1")
                                    .build()
                                    .unwrap(),
                            )
                            .build()
                            .unwrap(),
                    )
                    .build(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_dashboards()
        .aws_account_id(ACCOUNT_ID)
        .send()
        .await
        .expect("list_dashboards should succeed");

    assert_eq!(resp.dashboard_summary_list().len(), 2);
}

// ==== Group tests ====

#[tokio::test]
async fn test_create_and_describe_group() {
    let client = make_quicksight_client().await;

    let create_resp = client
        .create_group()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("test-group")
        .description("A test group")
        .send()
        .await
        .expect("create_group should succeed");

    let group = create_resp.group().expect("should have group");
    assert_eq!(group.group_name(), Some("test-group"));
    assert_eq!(group.description(), Some("A test group"));

    let describe_resp = client
        .describe_group()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("test-group")
        .send()
        .await
        .expect("describe_group should succeed");

    let group = describe_resp.group().expect("should have group");
    assert_eq!(group.group_name(), Some("test-group"));
}

#[tokio::test]
async fn test_update_group() {
    let client = make_quicksight_client().await;

    client
        .create_group()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("grp-upd")
        .description("Original")
        .send()
        .await
        .unwrap();

    let upd_resp = client
        .update_group()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("grp-upd")
        .description("Updated")
        .send()
        .await
        .expect("update_group should succeed");

    let group = upd_resp.group().expect("should have group");
    assert_eq!(group.description(), Some("Updated"));
}

#[tokio::test]
async fn test_delete_group() {
    let client = make_quicksight_client().await;

    client
        .create_group()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("grp-del")
        .send()
        .await
        .unwrap();

    client
        .delete_group()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("grp-del")
        .send()
        .await
        .expect("delete_group should succeed");

    let result = client
        .describe_group()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("grp-del")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_groups() {
    let client = make_quicksight_client().await;

    for name in ["grp-a", "grp-b"] {
        client
            .create_group()
            .aws_account_id(ACCOUNT_ID)
            .namespace("default")
            .group_name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_groups()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .send()
        .await
        .expect("list_groups should succeed");

    assert_eq!(resp.group_list().len(), 2);
}

#[tokio::test]
async fn test_search_groups() {
    let client = make_quicksight_client().await;

    for name in ["alpha-group", "beta-group", "alpha-team"] {
        client
            .create_group()
            .aws_account_id(ACCOUNT_ID)
            .namespace("default")
            .group_name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .search_groups()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .filters(
            aws_sdk_quicksight::types::GroupSearchFilter::builder()
                .name(aws_sdk_quicksight::types::GroupFilterAttribute::GroupName)
                .operator(aws_sdk_quicksight::types::GroupFilterOperator::StartsWith)
                .value("alpha")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("search_groups should succeed");

    assert_eq!(resp.group_list().len(), 2);
}

// ==== Group Membership tests ====

#[tokio::test]
async fn test_create_and_describe_group_membership() {
    let client = make_quicksight_client().await;

    client
        .create_group()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("membership-grp")
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_group_membership()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("membership-grp")
        .member_name("user1")
        .send()
        .await
        .expect("create_group_membership should succeed");

    let member = create_resp.group_member().expect("should have member");
    assert_eq!(member.member_name(), Some("user1"));

    let describe_resp = client
        .describe_group_membership()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("membership-grp")
        .member_name("user1")
        .send()
        .await
        .expect("describe_group_membership should succeed");

    let member = describe_resp.group_member().expect("should have member");
    assert_eq!(member.member_name(), Some("user1"));
}

#[tokio::test]
async fn test_list_group_memberships() {
    let client = make_quicksight_client().await;

    client
        .create_group()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("list-members-grp")
        .send()
        .await
        .unwrap();

    for member in ["user1", "user2", "user3"] {
        client
            .create_group_membership()
            .aws_account_id(ACCOUNT_ID)
            .namespace("default")
            .group_name("list-members-grp")
            .member_name(member)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_group_memberships()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("list-members-grp")
        .send()
        .await
        .expect("list_group_memberships should succeed");

    assert_eq!(resp.group_member_list().len(), 3);
}

// ==== User tests ====

#[tokio::test]
async fn test_register_and_describe_user() {
    let client = make_quicksight_client().await;

    let reg_resp = client
        .register_user()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .email("test@example.com")
        .identity_type(aws_sdk_quicksight::types::IdentityType::Iam)
        .user_role(aws_sdk_quicksight::types::UserRole::Reader)
        .user_name("testuser")
        .send()
        .await
        .expect("register_user should succeed");

    let user = reg_resp.user().expect("should have user");
    assert_eq!(user.user_name(), Some("testuser"));
    assert_eq!(user.email(), Some("test@example.com"));

    let describe_resp = client
        .describe_user()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .user_name("testuser")
        .send()
        .await
        .expect("describe_user should succeed");

    let user = describe_resp.user().expect("should have user");
    assert_eq!(user.user_name(), Some("testuser"));
}

#[tokio::test]
async fn test_update_user() {
    let client = make_quicksight_client().await;

    client
        .register_user()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .email("upd@example.com")
        .identity_type(aws_sdk_quicksight::types::IdentityType::Iam)
        .user_role(aws_sdk_quicksight::types::UserRole::Reader)
        .user_name("upduser")
        .send()
        .await
        .unwrap();

    let upd_resp = client
        .update_user()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .user_name("upduser")
        .email("updated@example.com")
        .role(aws_sdk_quicksight::types::UserRole::Author)
        .send()
        .await
        .expect("update_user should succeed");

    let user = upd_resp.user().expect("should have user");
    assert_eq!(user.email(), Some("updated@example.com"));
}

#[tokio::test]
async fn test_delete_user() {
    let client = make_quicksight_client().await;

    client
        .register_user()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .email("del@example.com")
        .identity_type(aws_sdk_quicksight::types::IdentityType::Iam)
        .user_role(aws_sdk_quicksight::types::UserRole::Reader)
        .user_name("deluser")
        .send()
        .await
        .unwrap();

    client
        .delete_user()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .user_name("deluser")
        .send()
        .await
        .expect("delete_user should succeed");

    let result = client
        .describe_user()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .user_name("deluser")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_users() {
    let client = make_quicksight_client().await;

    for (name, email) in [("ua", "ua@example.com"), ("ub", "ub@example.com")] {
        client
            .register_user()
            .aws_account_id(ACCOUNT_ID)
            .namespace("default")
            .email(email)
            .identity_type(aws_sdk_quicksight::types::IdentityType::Iam)
            .user_role(aws_sdk_quicksight::types::UserRole::Reader)
            .user_name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_users()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .send()
        .await
        .expect("list_users should succeed");

    assert_eq!(resp.user_list().len(), 2);
}

#[tokio::test]
async fn test_list_user_groups() {
    let client = make_quicksight_client().await;

    // Create user
    client
        .register_user()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .email("ug@example.com")
        .identity_type(aws_sdk_quicksight::types::IdentityType::Iam)
        .user_role(aws_sdk_quicksight::types::UserRole::Reader)
        .user_name("uguser")
        .send()
        .await
        .unwrap();

    // Create two groups and add user to both
    for grp in ["ug-grp1", "ug-grp2"] {
        client
            .create_group()
            .aws_account_id(ACCOUNT_ID)
            .namespace("default")
            .group_name(grp)
            .send()
            .await
            .unwrap();

        client
            .create_group_membership()
            .aws_account_id(ACCOUNT_ID)
            .namespace("default")
            .group_name(grp)
            .member_name("uguser")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_user_groups()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .user_name("uguser")
        .send()
        .await
        .expect("list_user_groups should succeed");

    assert_eq!(resp.group_list().len(), 2);
}

// ==== Ingestion test ====

#[tokio::test]
async fn test_create_ingestion() {
    let client = make_quicksight_client().await;

    // First create a data set
    client
        .create_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ds-ing")
        .name("Ingestion DS")
        .import_mode(aws_sdk_quicksight::types::DataSetImportMode::Spice)
        .physical_table_map(
            "table1",
            aws_sdk_quicksight::types::PhysicalTable::S3Source(
                aws_sdk_quicksight::types::S3Source::builder()
                    .data_source_arn("arn:aws:quicksight:us-east-1:123456789012:datasource/src1")
                    .input_columns(
                        aws_sdk_quicksight::types::InputColumn::builder()
                            .name("col1")
                            .r#type(aws_sdk_quicksight::types::InputColumnDataType::String)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            ),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .create_ingestion()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ds-ing")
        .ingestion_id("ing-001")
        .send()
        .await
        .expect("create_ingestion should succeed");

    assert_eq!(resp.ingestion_id(), Some("ing-001"));
    assert!(resp.arn().is_some());
}

// ==== Account Settings tests ====

#[tokio::test]
async fn test_describe_account_settings() {
    let client = make_quicksight_client().await;

    let resp = client
        .describe_account_settings()
        .aws_account_id(ACCOUNT_ID)
        .send()
        .await
        .expect("describe_account_settings should succeed");

    assert!(resp.account_settings().is_some());
}

#[tokio::test]
async fn test_update_account_settings() {
    let client = make_quicksight_client().await;

    client
        .update_account_settings()
        .aws_account_id(ACCOUNT_ID)
        .default_namespace("custom-ns")
        .send()
        .await
        .expect("update_account_settings should succeed");

    let resp = client
        .describe_account_settings()
        .aws_account_id(ACCOUNT_ID)
        .send()
        .await
        .unwrap();

    let settings = resp.account_settings().unwrap();
    assert_eq!(settings.default_namespace(), Some("custom-ns"));
}

// ==== Public Sharing Settings test ====

#[tokio::test]
async fn test_update_public_sharing_settings() {
    let client = make_quicksight_client().await;

    client
        .update_public_sharing_settings()
        .aws_account_id(ACCOUNT_ID)
        .public_sharing_enabled(true)
        .send()
        .await
        .expect("update_public_sharing_settings should succeed");
}

// ==== Tag tests ====

#[tokio::test]
async fn test_tag_list_untag_resource() {
    let client = make_quicksight_client().await;

    let resource_arn = "arn:aws:quicksight:us-east-1:123456789012:dataset/ds-tag-test";

    // Tag
    client
        .tag_resource()
        .resource_arn(resource_arn)
        .tags(
            aws_sdk_quicksight::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_quicksight::types::Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // List
    let resp = client
        .list_tags_for_resource()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    assert_eq!(resp.tags().len(), 2);

    // Untag
    client
        .untag_resource()
        .resource_arn(resource_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(resource_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "team");
}

// ==== Error path tests ====

#[tokio::test]
async fn test_describe_nonexistent_group_fails() {
    let client = make_quicksight_client().await;

    let result = client
        .describe_group()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_nonexistent_user_fails() {
    let client = make_quicksight_client().await;

    let result = client
        .describe_user()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .user_name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_nonexistent_dashboard_fails() {
    let client = make_quicksight_client().await;

    let result = client
        .describe_dashboard()
        .aws_account_id(ACCOUNT_ID)
        .dashboard_id("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_nonexistent_data_source_fails() {
    let client = make_quicksight_client().await;

    let result = client
        .describe_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ==== Additional moto-ported tests ====

// From moto test_quicksight_groups.py: test_search_groups__check_exceptions
// Searching with a StartsWith filter on a value with no matches returns an empty list.
#[tokio::test]
async fn test_search_groups_no_matches() {
    let client = make_quicksight_client().await;

    // Create some groups
    for name in ["alpha-group", "beta-group"] {
        client
            .create_group()
            .aws_account_id(ACCOUNT_ID)
            .namespace("default")
            .group_name(name)
            .send()
            .await
            .unwrap();
    }

    let result = client
        .search_groups()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .filters(
            aws_sdk_quicksight::types::GroupSearchFilter::builder()
                .name(aws_sdk_quicksight::types::GroupFilterAttribute::GroupName)
                .operator(aws_sdk_quicksight::types::GroupFilterOperator::StartsWith)
                .value("nonexistent")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("search_groups with no matches should succeed");

    assert_eq!(result.group_list().len(), 0);
}

// From moto test_quicksight_groups.py: test_search_groups with StartsWith operator
#[tokio::test]
async fn test_search_groups_starts_with() {
    let client = make_quicksight_client().await;

    for name in ["alpha-group", "alpha-team", "beta-group", "gamma-squad"] {
        client
            .create_group()
            .aws_account_id(ACCOUNT_ID)
            .namespace("default")
            .group_name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .search_groups()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .filters(
            aws_sdk_quicksight::types::GroupSearchFilter::builder()
                .name(aws_sdk_quicksight::types::GroupFilterAttribute::GroupName)
                .operator(aws_sdk_quicksight::types::GroupFilterOperator::StartsWith)
                .value("alpha")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("search_groups StartsWith should succeed");

    assert_eq!(resp.group_list().len(), 2);
}

// From moto test_quicksight_users.py: test_register_user__quicksight
// Test registering a user with QUICKSIGHT identity type
#[tokio::test]
async fn test_register_user_quicksight_identity_type() {
    let client = make_quicksight_client().await;

    let reg_resp = client
        .register_user()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .email("quicksight-user@example.com")
        .identity_type(aws_sdk_quicksight::types::IdentityType::Quicksight)
        .user_role(aws_sdk_quicksight::types::UserRole::Reader)
        .user_name("qs-reader")
        .send()
        .await
        .expect("register_user with Quicksight identity should succeed");

    let user = reg_resp.user().expect("should have user");
    assert_eq!(user.user_name(), Some("qs-reader"));
    assert_eq!(user.email(), Some("quicksight-user@example.com"));
    assert_eq!(
        user.identity_type(),
        Some(&aws_sdk_quicksight::types::IdentityType::Quicksight)
    );
    assert_eq!(
        user.role(),
        Some(&aws_sdk_quicksight::types::UserRole::Reader)
    );
    assert!(!user.active());
}

// From moto test_quicksight_users.py: test_list_users__initial
#[tokio::test]
async fn test_list_users_empty() {
    let client = make_quicksight_client().await;

    let resp = client
        .list_users()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .send()
        .await
        .expect("list_users on empty namespace should succeed");

    assert_eq!(resp.user_list().len(), 0);
}

// From moto test_quicksight_groups.py: test_list_groups__initial
#[tokio::test]
async fn test_list_groups_empty() {
    let client = make_quicksight_client().await;

    let resp = client
        .list_groups()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .send()
        .await
        .expect("list_groups on empty namespace should succeed");

    assert_eq!(resp.group_list().len(), 0);
}

// From moto test_quicksight_users.py: test_delete_user__quicksight
// After deleting a user, describe_user should return an error.
#[tokio::test]
async fn test_delete_user_then_describe_fails() {
    let client = make_quicksight_client().await;

    client
        .register_user()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .email("del2@example.com")
        .identity_type(aws_sdk_quicksight::types::IdentityType::Quicksight)
        .user_role(aws_sdk_quicksight::types::UserRole::Reader)
        .user_name("del2user")
        .send()
        .await
        .unwrap();

    client
        .delete_user()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .user_name("del2user")
        .send()
        .await
        .expect("delete_user should succeed");

    let result = client
        .describe_user()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .user_name("del2user")
        .send()
        .await;

    assert!(result.is_err());
}

// From moto test_quicksight_tagging.py: test_tag_data_source
// Tags can be added to data sources and listed back.
#[tokio::test]
async fn test_tag_data_source() {
    let client = make_quicksight_client().await;

    client
        .create_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("tagged-dsrc-001")
        .name("Tagged DataSource")
        .r#type(aws_sdk_quicksight::types::DataSourceType::S3)
        .send()
        .await
        .unwrap();

    let arn = format!("arn:aws:quicksight:us-east-1:{ACCOUNT_ID}:datasource/tagged-dsrc-001");

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_quicksight::types::Tag::builder()
                .key("env")
                .value("prod")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    assert!(
        resp.tags()
            .iter()
            .any(|t| t.key() == "env" && t.value() == "prod")
    );
}

// From moto test_quicksight_tagging.py: test_untag_data_source
// After untagging, the tag should not appear in list_tags_for_resource.
#[tokio::test]
async fn test_untag_data_source() {
    let client = make_quicksight_client().await;

    client
        .create_data_source()
        .aws_account_id(ACCOUNT_ID)
        .data_source_id("untag-dsrc-001")
        .name("Untag DataSource")
        .r#type(aws_sdk_quicksight::types::DataSourceType::S3)
        .send()
        .await
        .unwrap();

    let arn = format!("arn:aws:quicksight:us-east-1:{ACCOUNT_ID}:datasource/untag-dsrc-001");

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_quicksight::types::Tag::builder()
                .key("env")
                .value("staging")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    assert!(!resp.tags().iter().any(|t| t.key() == "env"));
}

// ==== Analysis tests ====

#[tokio::test]
async fn test_create_and_describe_analysis() {
    let client = make_quicksight_client().await;

    let resp = client
        .create_analysis()
        .aws_account_id(ACCOUNT_ID)
        .analysis_id("analysis-001")
        .name("Test Analysis")
        .send()
        .await
        .expect("create_analysis should succeed");

    assert_eq!(resp.analysis_id(), Some("analysis-001"));
    assert!(resp.arn().is_some());

    let desc = client
        .describe_analysis()
        .aws_account_id(ACCOUNT_ID)
        .analysis_id("analysis-001")
        .send()
        .await
        .expect("describe_analysis should succeed");

    let a = desc.analysis().expect("should have analysis");
    assert_eq!(a.analysis_id(), Some("analysis-001"));
    assert_eq!(a.name(), Some("Test Analysis"));
}

#[tokio::test]
async fn test_list_analyses() {
    let client = make_quicksight_client().await;

    for (id, name) in [("al-a", "Analysis A"), ("al-b", "Analysis B")] {
        client
            .create_analysis()
            .aws_account_id(ACCOUNT_ID)
            .analysis_id(id)
            .name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_analyses()
        .aws_account_id(ACCOUNT_ID)
        .send()
        .await
        .expect("list_analyses should succeed");

    let ids: Vec<_> = resp
        .analysis_summary_list()
        .iter()
        .filter_map(|a| a.analysis_id())
        .collect();
    assert!(ids.contains(&"al-a"));
    assert!(ids.contains(&"al-b"));
}

#[tokio::test]
async fn test_update_analysis() {
    let client = make_quicksight_client().await;

    client
        .create_analysis()
        .aws_account_id(ACCOUNT_ID)
        .analysis_id("al-upd-001")
        .name("Original Name")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_analysis()
        .aws_account_id(ACCOUNT_ID)
        .analysis_id("al-upd-001")
        .name("Updated Name")
        .send()
        .await
        .expect("update_analysis should succeed");

    assert_eq!(resp.analysis_id(), Some("al-upd-001"));
}

#[tokio::test]
async fn test_delete_analysis() {
    let client = make_quicksight_client().await;

    client
        .create_analysis()
        .aws_account_id(ACCOUNT_ID)
        .analysis_id("al-del-001")
        .name("To Delete")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_analysis()
        .aws_account_id(ACCOUNT_ID)
        .analysis_id("al-del-001")
        .send()
        .await
        .expect("delete_analysis should succeed");

    assert_eq!(resp.analysis_id(), Some("al-del-001"));

    client
        .describe_analysis()
        .aws_account_id(ACCOUNT_ID)
        .analysis_id("al-del-001")
        .send()
        .await
        .expect_err("describe after delete should fail");
}

// ==== Dashboard delete/update tests ====

#[tokio::test]
async fn test_delete_dashboard() {
    let client = make_quicksight_client().await;

    client
        .create_dashboard()
        .aws_account_id(ACCOUNT_ID)
        .dashboard_id("dash-del-001")
        .name("Dashboard To Delete")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_dashboard()
        .aws_account_id(ACCOUNT_ID)
        .dashboard_id("dash-del-001")
        .send()
        .await
        .expect("delete_dashboard should succeed");

    assert_eq!(resp.dashboard_id(), Some("dash-del-001"));

    client
        .describe_dashboard()
        .aws_account_id(ACCOUNT_ID)
        .dashboard_id("dash-del-001")
        .send()
        .await
        .expect_err("describe after delete should fail");
}

#[tokio::test]
async fn test_update_dashboard() {
    let client = make_quicksight_client().await;

    client
        .create_dashboard()
        .aws_account_id(ACCOUNT_ID)
        .dashboard_id("dash-upd-001")
        .name("Original Dashboard")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_dashboard()
        .aws_account_id(ACCOUNT_ID)
        .dashboard_id("dash-upd-001")
        .name("Updated Dashboard")
        .send()
        .await
        .expect("update_dashboard should succeed");

    assert_eq!(resp.dashboard_id(), Some("dash-upd-001"));
}

// ==== GroupMembership delete test ====

#[tokio::test]
async fn test_delete_group_membership() {
    let client = make_quicksight_client().await;

    client
        .create_group()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("grp-mem-del")
        .send()
        .await
        .unwrap();

    client
        .create_group_membership()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("grp-mem-del")
        .member_name("alice")
        .send()
        .await
        .unwrap();

    client
        .delete_group_membership()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("grp-mem-del")
        .member_name("alice")
        .send()
        .await
        .expect("delete_group_membership should succeed");

    let resp = client
        .list_group_memberships()
        .aws_account_id(ACCOUNT_ID)
        .namespace("default")
        .group_name("grp-mem-del")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.group_member_list().len(), 0);
}

// ==== Folder tests ====

#[tokio::test]
async fn test_create_and_describe_folder() {
    let client = make_quicksight_client().await;

    let resp = client
        .create_folder()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("folder-001")
        .name("Test Folder")
        .folder_type(aws_sdk_quicksight::types::FolderType::Shared)
        .send()
        .await
        .expect("create_folder should succeed");

    assert_eq!(resp.folder_id(), Some("folder-001"));

    let desc = client
        .describe_folder()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("folder-001")
        .send()
        .await
        .expect("describe_folder should succeed");

    let f = desc.folder().expect("should have folder");
    assert_eq!(f.folder_id(), Some("folder-001"));
    assert_eq!(f.name(), Some("Test Folder"));
}

#[tokio::test]
async fn test_list_folders() {
    let client = make_quicksight_client().await;

    for (id, name) in [("fld-a", "Folder A"), ("fld-b", "Folder B")] {
        client
            .create_folder()
            .aws_account_id(ACCOUNT_ID)
            .folder_id(id)
            .name(name)
            .folder_type(aws_sdk_quicksight::types::FolderType::Shared)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_folders()
        .aws_account_id(ACCOUNT_ID)
        .send()
        .await
        .expect("list_folders should succeed");

    let ids: Vec<_> = resp
        .folder_summary_list()
        .iter()
        .filter_map(|f| f.folder_id())
        .collect();
    assert!(ids.contains(&"fld-a"));
    assert!(ids.contains(&"fld-b"));
}

#[tokio::test]
async fn test_delete_folder() {
    let client = make_quicksight_client().await;

    client
        .create_folder()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("fld-del-001")
        .name("Folder To Delete")
        .folder_type(aws_sdk_quicksight::types::FolderType::Shared)
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_folder()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("fld-del-001")
        .send()
        .await
        .expect("delete_folder should succeed");

    assert_eq!(resp.folder_id(), Some("fld-del-001"));

    client
        .describe_folder()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("fld-del-001")
        .send()
        .await
        .expect_err("describe after delete should fail");
}

#[tokio::test]
async fn test_folder_membership_lifecycle() {
    let client = make_quicksight_client().await;

    client
        .create_folder()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("fld-mem-001")
        .name("Folder with Members")
        .folder_type(aws_sdk_quicksight::types::FolderType::Shared)
        .send()
        .await
        .unwrap();

    client
        .create_folder_membership()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("fld-mem-001")
        .member_type(aws_sdk_quicksight::types::MemberType::Dataset)
        .member_id("ds-member-001")
        .send()
        .await
        .expect("create_folder_membership should succeed");

    let list_resp = client
        .list_folder_members()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("fld-mem-001")
        .send()
        .await
        .expect("list_folder_members should succeed");

    assert_eq!(list_resp.folder_member_list().len(), 1);

    client
        .delete_folder_membership()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("fld-mem-001")
        .member_type(aws_sdk_quicksight::types::MemberType::Dataset)
        .member_id("ds-member-001")
        .send()
        .await
        .expect("delete_folder_membership should succeed");

    let list_after = client
        .list_folder_members()
        .aws_account_id(ACCOUNT_ID)
        .folder_id("fld-mem-001")
        .send()
        .await
        .unwrap();

    assert_eq!(list_after.folder_member_list().len(), 0);
}

// ==== Namespace tests ====

#[tokio::test]
async fn test_create_and_describe_namespace() {
    let client = make_quicksight_client().await;

    let resp = client
        .create_namespace()
        .aws_account_id(ACCOUNT_ID)
        .namespace("test-ns-001")
        .identity_store(aws_sdk_quicksight::types::IdentityStore::Quicksight)
        .send()
        .await
        .expect("create_namespace should succeed");

    assert_eq!(resp.name(), Some("test-ns-001"));

    let desc = client
        .describe_namespace()
        .aws_account_id(ACCOUNT_ID)
        .namespace("test-ns-001")
        .send()
        .await
        .expect("describe_namespace should succeed");

    let ns = desc.namespace().expect("should have namespace");
    assert_eq!(ns.name(), Some("test-ns-001"));
}

#[tokio::test]
async fn test_list_and_delete_namespace() {
    let client = make_quicksight_client().await;

    client
        .create_namespace()
        .aws_account_id(ACCOUNT_ID)
        .namespace("ns-list-001")
        .identity_store(aws_sdk_quicksight::types::IdentityStore::Quicksight)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_namespaces()
        .aws_account_id(ACCOUNT_ID)
        .send()
        .await
        .expect("list_namespaces should succeed");

    let names: Vec<_> = list_resp
        .namespaces()
        .iter()
        .filter_map(|ns| ns.name())
        .collect();
    assert!(names.contains(&"ns-list-001"));

    client
        .delete_namespace()
        .aws_account_id(ACCOUNT_ID)
        .namespace("ns-list-001")
        .send()
        .await
        .expect("delete_namespace should succeed");

    client
        .describe_namespace()
        .aws_account_id(ACCOUNT_ID)
        .namespace("ns-list-001")
        .send()
        .await
        .expect_err("describe after delete should fail");
}

// ==== Template tests ====

#[tokio::test]
async fn test_create_and_describe_template() {
    let client = make_quicksight_client().await;

    let resp = client
        .create_template()
        .aws_account_id(ACCOUNT_ID)
        .template_id("tmpl-001")
        .name("Test Template")
        .send()
        .await
        .expect("create_template should succeed");

    assert_eq!(resp.template_id(), Some("tmpl-001"));
    assert!(resp.arn().is_some());

    let desc = client
        .describe_template()
        .aws_account_id(ACCOUNT_ID)
        .template_id("tmpl-001")
        .send()
        .await
        .expect("describe_template should succeed");

    let t = desc.template().expect("should have template");
    assert_eq!(t.template_id(), Some("tmpl-001"));
    assert_eq!(t.name(), Some("Test Template"));
}

#[tokio::test]
async fn test_list_templates() {
    let client = make_quicksight_client().await;

    for (id, name) in [("tmpl-la", "Template A"), ("tmpl-lb", "Template B")] {
        client
            .create_template()
            .aws_account_id(ACCOUNT_ID)
            .template_id(id)
            .name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_templates()
        .aws_account_id(ACCOUNT_ID)
        .send()
        .await
        .expect("list_templates should succeed");

    let ids: Vec<_> = resp
        .template_summary_list()
        .iter()
        .filter_map(|t| t.template_id())
        .collect();
    assert!(ids.contains(&"tmpl-la"));
    assert!(ids.contains(&"tmpl-lb"));
}

#[tokio::test]
async fn test_update_and_delete_template() {
    let client = make_quicksight_client().await;

    client
        .create_template()
        .aws_account_id(ACCOUNT_ID)
        .template_id("tmpl-ud-001")
        .name("Original Template")
        .send()
        .await
        .unwrap();

    let upd = client
        .update_template()
        .aws_account_id(ACCOUNT_ID)
        .template_id("tmpl-ud-001")
        .name("Updated Template")
        .send()
        .await
        .expect("update_template should succeed");

    assert_eq!(upd.template_id(), Some("tmpl-ud-001"));

    client
        .delete_template()
        .aws_account_id(ACCOUNT_ID)
        .template_id("tmpl-ud-001")
        .send()
        .await
        .expect("delete_template should succeed");

    client
        .describe_template()
        .aws_account_id(ACCOUNT_ID)
        .template_id("tmpl-ud-001")
        .send()
        .await
        .expect_err("describe after delete should fail");
}

// ==== Theme tests ====

#[tokio::test]
async fn test_create_and_describe_theme() {
    let client = make_quicksight_client().await;

    let resp = client
        .create_theme()
        .aws_account_id(ACCOUNT_ID)
        .theme_id("theme-001")
        .name("Test Theme")
        .base_theme_id("MIDNIGHT")
        .send()
        .await
        .expect("create_theme should succeed");

    assert_eq!(resp.theme_id(), Some("theme-001"));
    assert!(resp.arn().is_some());

    let desc = client
        .describe_theme()
        .aws_account_id(ACCOUNT_ID)
        .theme_id("theme-001")
        .send()
        .await
        .expect("describe_theme should succeed");

    let t = desc.theme().expect("should have theme");
    assert_eq!(t.theme_id(), Some("theme-001"));
    assert_eq!(t.name(), Some("Test Theme"));
}

#[tokio::test]
async fn test_list_themes() {
    let client = make_quicksight_client().await;

    for (id, name) in [("theme-la", "Theme A"), ("theme-lb", "Theme B")] {
        client
            .create_theme()
            .aws_account_id(ACCOUNT_ID)
            .theme_id(id)
            .name(name)
            .base_theme_id("MIDNIGHT")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_themes()
        .aws_account_id(ACCOUNT_ID)
        .send()
        .await
        .expect("list_themes should succeed");

    let ids: Vec<_> = resp
        .theme_summary_list()
        .iter()
        .filter_map(|t| t.theme_id())
        .collect();
    assert!(ids.contains(&"theme-la"));
    assert!(ids.contains(&"theme-lb"));
}

#[tokio::test]
async fn test_update_and_delete_theme() {
    let client = make_quicksight_client().await;

    client
        .create_theme()
        .aws_account_id(ACCOUNT_ID)
        .theme_id("theme-ud-001")
        .name("Original Theme")
        .base_theme_id("MIDNIGHT")
        .send()
        .await
        .unwrap();

    let upd = client
        .update_theme()
        .aws_account_id(ACCOUNT_ID)
        .theme_id("theme-ud-001")
        .name("Updated Theme")
        .base_theme_id("MIDNIGHT")
        .send()
        .await
        .expect("update_theme should succeed");

    assert_eq!(upd.theme_id(), Some("theme-ud-001"));

    client
        .delete_theme()
        .aws_account_id(ACCOUNT_ID)
        .theme_id("theme-ud-001")
        .send()
        .await
        .expect("delete_theme should succeed");

    client
        .describe_theme()
        .aws_account_id(ACCOUNT_ID)
        .theme_id("theme-ud-001")
        .send()
        .await
        .expect_err("describe after delete should fail");
}

// ==== Ingestion describe/list tests ====

#[tokio::test]
async fn test_describe_and_list_ingestions() {
    let client = make_quicksight_client().await;

    client
        .create_data_set()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ing-ds-001")
        .name("Ingestion Dataset")
        .import_mode(aws_sdk_quicksight::types::DataSetImportMode::Spice)
        .physical_table_map(
            "t1",
            aws_sdk_quicksight::types::PhysicalTable::S3Source(
                aws_sdk_quicksight::types::S3Source::builder()
                    .data_source_arn("arn:aws:quicksight:us-east-1:123456789012:datasource/src1")
                    .input_columns(
                        aws_sdk_quicksight::types::InputColumn::builder()
                            .name("col1")
                            .r#type(aws_sdk_quicksight::types::InputColumnDataType::String)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            ),
        )
        .send()
        .await
        .unwrap();

    client
        .create_ingestion()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ing-ds-001")
        .ingestion_id("ing-001")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_ingestion()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ing-ds-001")
        .ingestion_id("ing-001")
        .send()
        .await
        .expect("describe_ingestion should succeed");

    let ing = desc.ingestion().expect("should have ingestion");
    assert_eq!(ing.ingestion_id(), Some("ing-001"));

    let list = client
        .list_ingestions()
        .aws_account_id(ACCOUNT_ID)
        .data_set_id("ing-ds-001")
        .send()
        .await
        .expect("list_ingestions should succeed");

    assert_eq!(list.ingestions().len(), 1);
}
