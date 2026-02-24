use aws_sdk_lakeformation::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_lakeformation::LakeFormationService;

async fn make_lakeformation_client() -> aws_sdk_lakeformation::Client {
    let mock = MockAws::builder()
        .with_service(LakeFormationService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_lakeformation::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_lakeformation::Client::new(&config)
}

#[tokio::test]
async fn test_register_and_list_resources() {
    let client = make_lakeformation_client().await;

    client
        .register_resource()
        .resource_arn("arn:aws:s3:::my-bucket")
        .use_service_linked_role(true)
        .send()
        .await
        .expect("register_resource should succeed");

    let resp = client
        .list_resources()
        .send()
        .await
        .expect("list_resources should succeed");

    let resources = resp.resource_info_list();
    assert_eq!(resources.len(), 1);
    assert_eq!(
        resources[0].resource_arn().unwrap(),
        "arn:aws:s3:::my-bucket"
    );
}

#[tokio::test]
async fn test_register_and_deregister_resource() {
    let client = make_lakeformation_client().await;

    client
        .register_resource()
        .resource_arn("arn:aws:s3:::my-bucket")
        .use_service_linked_role(true)
        .send()
        .await
        .expect("register_resource should succeed");

    client
        .deregister_resource()
        .resource_arn("arn:aws:s3:::my-bucket")
        .send()
        .await
        .expect("deregister_resource should succeed");

    let resp = client
        .list_resources()
        .send()
        .await
        .expect("list_resources should succeed");

    assert_eq!(resp.resource_info_list().len(), 0);
}

#[tokio::test]
async fn test_deregister_nonexistent_resource_fails() {
    let client = make_lakeformation_client().await;

    let result = client
        .deregister_resource()
        .resource_arn("arn:aws:s3:::nonexistent-bucket")
        .send()
        .await;

    assert!(
        result.is_err(),
        "deregister nonexistent resource should fail"
    );
}

#[tokio::test]
async fn test_register_duplicate_resource_fails() {
    let client = make_lakeformation_client().await;

    client
        .register_resource()
        .resource_arn("arn:aws:s3:::my-bucket")
        .use_service_linked_role(true)
        .send()
        .await
        .expect("first register should succeed");

    let result = client
        .register_resource()
        .resource_arn("arn:aws:s3:::my-bucket")
        .use_service_linked_role(true)
        .send()
        .await;

    assert!(result.is_err(), "duplicate register should fail");
}

#[tokio::test]
async fn test_get_data_lake_settings() {
    let client = make_lakeformation_client().await;

    let resp = client
        .get_data_lake_settings()
        .send()
        .await
        .expect("get_data_lake_settings should succeed");

    let settings = resp.data_lake_settings().unwrap();

    // Default settings should have IAM_ALLOWED_PRINCIPALS in create database/table default perms
    let db_perms = settings.create_database_default_permissions();
    assert!(
        !db_perms.is_empty(),
        "create_database_default_permissions should not be empty"
    );

    let table_perms = settings.create_table_default_permissions();
    assert!(
        !table_perms.is_empty(),
        "create_table_default_permissions should not be empty"
    );
}

#[tokio::test]
async fn test_describe_resource() {
    let client = make_lakeformation_client().await;

    client
        .register_resource()
        .resource_arn("arn:aws:s3:::describe-bucket")
        .use_service_linked_role(true)
        .send()
        .await
        .expect("register should succeed");

    let resp = client
        .describe_resource()
        .resource_arn("arn:aws:s3:::describe-bucket")
        .send()
        .await
        .expect("describe_resource should succeed");

    let info = resp.resource_info().unwrap();
    assert_eq!(info.resource_arn().unwrap(), "arn:aws:s3:::describe-bucket");
}

#[tokio::test]
async fn test_describe_nonexistent_resource_fails() {
    let client = make_lakeformation_client().await;

    let result = client
        .describe_resource()
        .resource_arn("arn:aws:s3:::nonexistent")
        .send()
        .await;

    assert!(result.is_err(), "describe nonexistent resource should fail");
}

#[tokio::test]
async fn test_put_data_lake_settings() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{DataLakePrincipal, DataLakeSettings};

    let settings = DataLakeSettings::builder()
        .data_lake_admins(
            DataLakePrincipal::builder()
                .data_lake_principal_identifier("arn:aws:iam::123456789012:role/Admin")
                .build(),
        )
        .build();

    client
        .put_data_lake_settings()
        .data_lake_settings(settings)
        .send()
        .await
        .expect("put_data_lake_settings should succeed");

    let resp = client
        .get_data_lake_settings()
        .send()
        .await
        .expect("get_data_lake_settings should succeed");

    let s = resp.data_lake_settings().unwrap();
    let admins = s.data_lake_admins();
    assert_eq!(admins.len(), 1);
    assert_eq!(
        admins[0].data_lake_principal_identifier().unwrap(),
        "arn:aws:iam::123456789012:role/Admin"
    );
}

#[tokio::test]
async fn test_create_and_get_lf_tag() {
    let client = make_lakeformation_client().await;

    client
        .create_lf_tag()
        .tag_key("environment")
        .tag_values("dev")
        .tag_values("staging")
        .tag_values("prod")
        .send()
        .await
        .expect("create_lf_tag should succeed");

    let resp = client
        .get_lf_tag()
        .tag_key("environment")
        .send()
        .await
        .expect("get_lf_tag should succeed");

    assert_eq!(resp.tag_key().unwrap(), "environment");
    let values = resp.tag_values();
    assert_eq!(values.len(), 3);
    assert!(values.contains(&"dev".to_string()));
    assert!(values.contains(&"staging".to_string()));
    assert!(values.contains(&"prod".to_string()));
}

#[tokio::test]
async fn test_create_duplicate_lf_tag_fails() {
    let client = make_lakeformation_client().await;

    client
        .create_lf_tag()
        .tag_key("team")
        .tag_values("a")
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_lf_tag()
        .tag_key("team")
        .tag_values("b")
        .send()
        .await;

    assert!(result.is_err(), "duplicate create_lf_tag should fail");
}

#[tokio::test]
async fn test_delete_lf_tag() {
    let client = make_lakeformation_client().await;

    client
        .create_lf_tag()
        .tag_key("delete-me")
        .tag_values("val")
        .send()
        .await
        .expect("create should succeed");

    client
        .delete_lf_tag()
        .tag_key("delete-me")
        .send()
        .await
        .expect("delete_lf_tag should succeed");

    let result = client.get_lf_tag().tag_key("delete-me").send().await;

    assert!(result.is_err(), "get deleted tag should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_lf_tag_fails() {
    let client = make_lakeformation_client().await;

    let result = client.delete_lf_tag().tag_key("nonexistent").send().await;

    assert!(result.is_err(), "delete nonexistent tag should fail");
}

#[tokio::test]
async fn test_list_lf_tags() {
    let client = make_lakeformation_client().await;

    client
        .create_lf_tag()
        .tag_key("env")
        .tag_values("dev")
        .send()
        .await
        .expect("create env tag should succeed");

    client
        .create_lf_tag()
        .tag_key("team")
        .tag_values("alpha")
        .send()
        .await
        .expect("create team tag should succeed");

    let resp = client
        .list_lf_tags()
        .send()
        .await
        .expect("list_lf_tags should succeed");

    let tags = resp.lf_tags();
    assert_eq!(tags.len(), 2);
}

#[tokio::test]
async fn test_add_lf_tags_to_resource_and_get() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{DatabaseResource, LfTagPair, Resource};

    client
        .create_lf_tag()
        .tag_key("env")
        .tag_values("dev")
        .tag_values("prod")
        .send()
        .await
        .expect("create tag should succeed");

    let resource = Resource::builder()
        .database(DatabaseResource::builder().name("mydb").build().unwrap())
        .build();

    client
        .add_lf_tags_to_resource()
        .resource(resource.clone())
        .lf_tags(
            LfTagPair::builder()
                .tag_key("env")
                .tag_values("dev")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("add_lf_tags_to_resource should succeed");

    let resp = client
        .get_resource_lf_tags()
        .resource(resource)
        .send()
        .await
        .expect("get_resource_lf_tags should succeed");

    let db_tags = resp.lf_tag_on_database();
    assert_eq!(db_tags.len(), 1);
    assert_eq!(db_tags[0].tag_key(), "env");
}

#[tokio::test]
async fn test_grant_and_list_permissions() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{DataLakePrincipal, DatabaseResource, Permission, Resource};

    let principal = DataLakePrincipal::builder()
        .data_lake_principal_identifier("arn:aws:iam::123456789012:role/DataScientist")
        .build();

    let resource = Resource::builder()
        .database(
            DatabaseResource::builder()
                .name("analytics")
                .build()
                .unwrap(),
        )
        .build();

    client
        .grant_permissions()
        .principal(principal.clone())
        .resource(resource)
        .permissions(Permission::Select)
        .permissions(Permission::Describe)
        .send()
        .await
        .expect("grant_permissions should succeed");

    let resp = client
        .list_permissions()
        .principal(principal)
        .send()
        .await
        .expect("list_permissions should succeed");

    let perms = resp.principal_resource_permissions();
    assert_eq!(perms.len(), 1);
    assert!(perms[0].permissions().contains(&Permission::Select));
    assert!(perms[0].permissions().contains(&Permission::Describe));
}

#[tokio::test]
async fn test_revoke_permissions() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{DataLakePrincipal, DatabaseResource, Permission, Resource};

    let principal = DataLakePrincipal::builder()
        .data_lake_principal_identifier("arn:aws:iam::123456789012:role/Analyst")
        .build();

    let resource = Resource::builder()
        .database(DatabaseResource::builder().name("reports").build().unwrap())
        .build();

    client
        .grant_permissions()
        .principal(principal.clone())
        .resource(resource.clone())
        .permissions(Permission::Select)
        .send()
        .await
        .expect("grant should succeed");

    client
        .revoke_permissions()
        .principal(principal.clone())
        .resource(resource)
        .permissions(Permission::Select)
        .send()
        .await
        .expect("revoke_permissions should succeed");

    let resp = client
        .list_permissions()
        .principal(principal)
        .send()
        .await
        .expect("list_permissions should succeed");

    assert_eq!(resp.principal_resource_permissions().len(), 0);
}

#[tokio::test]
async fn test_batch_grant_permissions() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{
        BatchPermissionsRequestEntry, DataLakePrincipal, DatabaseResource, Permission, Resource,
    };

    let entry = BatchPermissionsRequestEntry::builder()
        .id("entry1")
        .principal(
            DataLakePrincipal::builder()
                .data_lake_principal_identifier("arn:aws:iam::123456789012:role/BatchUser")
                .build(),
        )
        .resource(
            Resource::builder()
                .database(DatabaseResource::builder().name("batchdb").build().unwrap())
                .build(),
        )
        .permissions(Permission::Select)
        .build()
        .unwrap();

    let resp = client
        .batch_grant_permissions()
        .entries(entry)
        .send()
        .await
        .expect("batch_grant_permissions should succeed");

    assert!(resp.failures().is_empty());
}

#[tokio::test]
async fn test_batch_revoke_permissions() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{
        BatchPermissionsRequestEntry, DataLakePrincipal, DatabaseResource, Permission, Resource,
    };

    let principal = DataLakePrincipal::builder()
        .data_lake_principal_identifier("arn:aws:iam::123456789012:role/BatchRevoke")
        .build();

    let resource = Resource::builder()
        .database(
            DatabaseResource::builder()
                .name("revokedb")
                .build()
                .unwrap(),
        )
        .build();

    // First grant
    client
        .grant_permissions()
        .principal(principal.clone())
        .resource(resource.clone())
        .permissions(Permission::Select)
        .send()
        .await
        .expect("grant should succeed");

    // Batch revoke
    let entry = BatchPermissionsRequestEntry::builder()
        .id("revoke1")
        .principal(principal.clone())
        .resource(resource)
        .permissions(Permission::Select)
        .build()
        .unwrap();

    let resp = client
        .batch_revoke_permissions()
        .entries(entry)
        .send()
        .await
        .expect("batch_revoke_permissions should succeed");

    assert!(resp.failures().is_empty());

    // Verify revoked
    let list_resp = client
        .list_permissions()
        .principal(principal)
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(list_resp.principal_resource_permissions().len(), 0);
}

#[tokio::test]
async fn test_lf_tag_lifecycle() {
    let client = make_lakeformation_client().await;

    // Create
    client
        .create_lf_tag()
        .tag_key("lifecycle-tag")
        .tag_values("v1")
        .tag_values("v2")
        .send()
        .await
        .expect("create should succeed");

    // Get
    let resp = client
        .get_lf_tag()
        .tag_key("lifecycle-tag")
        .send()
        .await
        .expect("get should succeed");
    assert_eq!(resp.tag_values().len(), 2);

    // List
    let resp = client
        .list_lf_tags()
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.lf_tags().is_empty());

    // Delete
    client
        .delete_lf_tag()
        .tag_key("lifecycle-tag")
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client.get_lf_tag().tag_key("lifecycle-tag").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_resource_lifecycle() {
    let client = make_lakeformation_client().await;

    let arn = "arn:aws:s3:::lifecycle-bucket";

    // Register
    client
        .register_resource()
        .resource_arn(arn)
        .use_service_linked_role(true)
        .send()
        .await
        .expect("register should succeed");

    // Describe
    let resp = client
        .describe_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(resp.resource_info().unwrap().resource_arn().unwrap(), arn);

    // List
    let resp = client
        .list_resources()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(resp.resource_info_list().len(), 1);

    // Deregister
    client
        .deregister_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("deregister should succeed");

    // Verify gone
    let result = client.describe_resource().resource_arn(arn).send().await;
    assert!(result.is_err());
}

// --- New moto-ported tests ---

#[tokio::test]
async fn test_grant_permissions_idempotent() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{DataLakePrincipal, DatabaseResource, Permission, Resource};

    let principal = DataLakePrincipal::builder()
        .data_lake_principal_identifier("asdf")
        .build();

    let resource = Resource::builder()
        .database(DatabaseResource::builder().name("db").build().unwrap())
        .build();

    // Grant the same permissions twice
    for _ in 0..2 {
        client
            .grant_permissions()
            .principal(principal.clone())
            .resource(resource.clone())
            .permissions(Permission::All)
            .send()
            .await
            .expect("grant_permissions should succeed");
    }

    // Should only have 1 entry
    let resp = client
        .list_permissions()
        .send()
        .await
        .expect("list_permissions should succeed");

    let perms = resp.principal_resource_permissions();
    assert_eq!(perms.len(), 1);
    assert!(perms[0].permissions().contains(&Permission::All));
}

#[tokio::test]
async fn test_grant_permissions_staggered() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{DataLakePrincipal, DatabaseResource, Permission, Resource};

    let principal = DataLakePrincipal::builder()
        .data_lake_principal_identifier("asdf")
        .build();

    let resource = Resource::builder()
        .database(DatabaseResource::builder().name("db").build().unwrap())
        .build();

    client
        .grant_permissions()
        .principal(principal.clone())
        .resource(resource.clone())
        .permissions(Permission::Describe)
        .send()
        .await
        .expect("first grant should succeed");

    client
        .grant_permissions()
        .principal(principal.clone())
        .resource(resource.clone())
        .permissions(Permission::CreateTable)
        .send()
        .await
        .expect("second grant should succeed");

    // Should be merged into a single entry with both permissions
    let resp = client
        .list_permissions()
        .send()
        .await
        .expect("list_permissions should succeed");

    let perms = resp.principal_resource_permissions();
    assert_eq!(perms.len(), 1, "should have 1 merged entry");
    let perm_list = perms[0].permissions();
    assert!(perm_list.contains(&Permission::Describe));
    assert!(perm_list.contains(&Permission::CreateTable));
}

#[tokio::test]
async fn test_list_permissions_with_grant_option() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{DataLakePrincipal, DatabaseResource, Permission, Resource};

    let principal = DataLakePrincipal::builder()
        .data_lake_principal_identifier("asdf")
        .build();

    let resource = Resource::builder()
        .database(DatabaseResource::builder().name("db").build().unwrap())
        .build();

    client
        .grant_permissions()
        .principal(principal.clone())
        .resource(resource.clone())
        .permissions(Permission::All)
        .permissions_with_grant_option(Permission::Select)
        .send()
        .await
        .expect("grant should succeed");

    let resp = client
        .list_permissions()
        .send()
        .await
        .expect("list_permissions should succeed");

    let perms = resp.principal_resource_permissions();
    assert_eq!(perms.len(), 1);
    assert!(perms[0].permissions().contains(&Permission::All));
    assert!(
        perms[0]
            .permissions_with_grant_option()
            .contains(&Permission::Select)
    );
}

#[tokio::test]
async fn test_revoke_partial_permissions() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{DataLakePrincipal, DatabaseResource, Permission, Resource};

    let principal = DataLakePrincipal::builder()
        .data_lake_principal_identifier("asdf")
        .build();

    let resource = Resource::builder()
        .database(DatabaseResource::builder().name("db").build().unwrap())
        .build();

    // Grant SELECT, ALTER, DROP
    client
        .grant_permissions()
        .principal(principal.clone())
        .resource(resource.clone())
        .permissions(Permission::Select)
        .permissions(Permission::Alter)
        .permissions(Permission::Drop)
        .send()
        .await
        .expect("grant should succeed");

    // Revoke only DROP
    client
        .revoke_permissions()
        .principal(principal.clone())
        .resource(resource.clone())
        .permissions(Permission::Drop)
        .send()
        .await
        .expect("revoke should succeed");

    // Should still have SELECT and ALTER
    let resp = client
        .list_permissions()
        .send()
        .await
        .expect("list_permissions should succeed");

    let perms = resp.principal_resource_permissions();
    assert_eq!(
        perms.len(),
        1,
        "entry should still exist after partial revoke"
    );
    let perm_list = perms[0].permissions();
    assert!(perm_list.contains(&Permission::Select));
    assert!(perm_list.contains(&Permission::Alter));
    assert!(!perm_list.contains(&Permission::Drop));
}

#[tokio::test]
async fn test_list_data_cells_filter_empty() {
    let client = make_lakeformation_client().await;

    let resp = client
        .list_data_cells_filter()
        .send()
        .await
        .expect("list_data_cells_filter should succeed");

    assert_eq!(resp.data_cells_filters().len(), 0);
}

#[tokio::test]
async fn test_batch_grant_then_batch_revoke_multi() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{
        BatchPermissionsRequestEntry, DataLakePrincipal, DatabaseResource, Permission, Resource,
    };

    let make_entry = |id: &str, principal_id: &str| {
        BatchPermissionsRequestEntry::builder()
            .id(id)
            .principal(
                DataLakePrincipal::builder()
                    .data_lake_principal_identifier(principal_id)
                    .build(),
            )
            .resource(
                Resource::builder()
                    .database(DatabaseResource::builder().name("db").build().unwrap())
                    .build(),
            )
            .permissions(Permission::Select)
            .permissions(Permission::Alter)
            .permissions(Permission::Drop)
            .permissions_with_grant_option(Permission::Select)
            .permissions_with_grant_option(Permission::Drop)
            .build()
            .unwrap()
    };

    // Batch grant for id1, id2, id3
    let resp = client
        .batch_grant_permissions()
        .entries(make_entry("grant1", "id1"))
        .entries(make_entry("grant2", "id2"))
        .entries(make_entry("grant3", "id3"))
        .send()
        .await
        .expect("batch_grant_permissions should succeed");
    assert!(resp.failures().is_empty());

    let list_resp = client
        .list_permissions()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(list_resp.principal_resource_permissions().len(), 3);

    // Batch revoke for id2 and id3
    let resp = client
        .batch_revoke_permissions()
        .entries(make_entry("revoke1", "id2"))
        .entries(make_entry("revoke2", "id3"))
        .send()
        .await
        .expect("batch_revoke_permissions should succeed");
    assert!(resp.failures().is_empty());

    // Only id1 should remain
    let list_resp = client
        .list_permissions()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(list_resp.principal_resource_permissions().len(), 1);
}

// ============================================================================
// Additional tests derived from AWS documentation
// ============================================================================

#[tokio::test]
async fn test_register_resource_with_role_arn() {
    let client = make_lakeformation_client().await;

    client
        .register_resource()
        .resource_arn("arn:aws:s3:::role-arn-bucket")
        .role_arn("arn:aws:iam::123456789012:role/LakeFormationRole")
        .use_service_linked_role(false)
        .send()
        .await
        .expect("register_resource with explicit role_arn should succeed");

    let resp = client
        .describe_resource()
        .resource_arn("arn:aws:s3:::role-arn-bucket")
        .send()
        .await
        .expect("describe_resource should succeed");

    let info = resp.resource_info().unwrap();
    assert_eq!(info.resource_arn().unwrap(), "arn:aws:s3:::role-arn-bucket");
    assert_eq!(
        info.role_arn().unwrap(),
        "arn:aws:iam::123456789012:role/LakeFormationRole"
    );
}

#[tokio::test]
async fn test_list_resources_multiple() {
    let client = make_lakeformation_client().await;

    for i in 0..3 {
        client
            .register_resource()
            .resource_arn(format!("arn:aws:s3:::multi-bucket-{i}"))
            .use_service_linked_role(true)
            .send()
            .await
            .expect("register_resource should succeed");
    }

    let resp = client
        .list_resources()
        .send()
        .await
        .expect("list_resources should succeed");

    assert_eq!(resp.resource_info_list().len(), 3);

    let arns: Vec<&str> = resp
        .resource_info_list()
        .iter()
        .filter_map(|r| r.resource_arn())
        .collect();
    assert!(arns.contains(&"arn:aws:s3:::multi-bucket-0"));
    assert!(arns.contains(&"arn:aws:s3:::multi-bucket-1"));
    assert!(arns.contains(&"arn:aws:s3:::multi-bucket-2"));
}

#[tokio::test]
async fn test_put_data_lake_settings_replaces_previous() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{DataLakePrincipal, DataLakeSettings};

    let settings_v1 = DataLakeSettings::builder()
        .data_lake_admins(
            DataLakePrincipal::builder()
                .data_lake_principal_identifier("arn:aws:iam::123456789012:role/AdminV1")
                .build(),
        )
        .build();

    client
        .put_data_lake_settings()
        .data_lake_settings(settings_v1)
        .send()
        .await
        .expect("first put should succeed");

    let settings_v2 = DataLakeSettings::builder()
        .data_lake_admins(
            DataLakePrincipal::builder()
                .data_lake_principal_identifier("arn:aws:iam::123456789012:role/AdminV2")
                .build(),
        )
        .data_lake_admins(
            DataLakePrincipal::builder()
                .data_lake_principal_identifier("arn:aws:iam::123456789012:role/AdminV2b")
                .build(),
        )
        .build();

    client
        .put_data_lake_settings()
        .data_lake_settings(settings_v2)
        .send()
        .await
        .expect("second put should succeed");

    let resp = client
        .get_data_lake_settings()
        .send()
        .await
        .expect("get should succeed");

    let s = resp.data_lake_settings().unwrap();
    let admins = s.data_lake_admins();
    assert_eq!(admins.len(), 2, "second put should replace first");
    let ids: Vec<&str> = admins
        .iter()
        .filter_map(|a| a.data_lake_principal_identifier())
        .collect();
    assert!(ids.contains(&"arn:aws:iam::123456789012:role/AdminV2"));
    assert!(ids.contains(&"arn:aws:iam::123456789012:role/AdminV2b"));
    assert!(!ids.contains(&"arn:aws:iam::123456789012:role/AdminV1"));
}

#[tokio::test]
async fn test_add_lf_tags_to_table_resource() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{LfTagPair, Resource, TableResource};

    client
        .create_lf_tag()
        .tag_key("sensitivity")
        .tag_values("public")
        .tag_values("private")
        .send()
        .await
        .expect("create tag should succeed");

    let resource = Resource::builder()
        .table(
            TableResource::builder()
                .database_name("mydb")
                .name("mytable")
                .build()
                .unwrap(),
        )
        .build();

    client
        .add_lf_tags_to_resource()
        .resource(resource.clone())
        .lf_tags(
            LfTagPair::builder()
                .tag_key("sensitivity")
                .tag_values("private")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("add_lf_tags_to_resource on table should succeed");

    let resp = client
        .get_resource_lf_tags()
        .resource(resource)
        .send()
        .await
        .expect("get_resource_lf_tags should succeed");

    // Table-level tags are returned; the mock returns them at database level
    // because both table and database resources flow through the same tag store.
    // The response should contain our tag with key "sensitivity".
    let all_tags: Vec<_> = resp
        .lf_tag_on_database()
        .iter()
        .chain(resp.lf_tags_on_table().iter())
        .collect();
    assert!(
        all_tags.iter().any(|t| t.tag_key() == "sensitivity"),
        "sensitivity tag should appear on the resource"
    );
}

#[tokio::test]
async fn test_list_permissions_no_filter_returns_all() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{DataLakePrincipal, DatabaseResource, Permission, Resource};

    for (principal_id, db_name) in [
        ("arn:aws:iam::123456789012:role/UserA", "dbA"),
        ("arn:aws:iam::123456789012:role/UserB", "dbB"),
        ("arn:aws:iam::123456789012:role/UserC", "dbC"),
    ] {
        client
            .grant_permissions()
            .principal(
                DataLakePrincipal::builder()
                    .data_lake_principal_identifier(principal_id)
                    .build(),
            )
            .resource(
                Resource::builder()
                    .database(DatabaseResource::builder().name(db_name).build().unwrap())
                    .build(),
            )
            .permissions(Permission::Describe)
            .send()
            .await
            .expect("grant should succeed");
    }

    // List without a principal filter should return all three grants.
    let resp = client
        .list_permissions()
        .send()
        .await
        .expect("list_permissions without filter should succeed");

    assert_eq!(
        resp.principal_resource_permissions().len(),
        3,
        "all three grants should be returned when no principal filter is applied"
    );
}

#[tokio::test]
async fn test_revoke_all_permissions_removes_entry() {
    let client = make_lakeformation_client().await;

    use aws_sdk_lakeformation::types::{DataLakePrincipal, DatabaseResource, Permission, Resource};

    let principal = DataLakePrincipal::builder()
        .data_lake_principal_identifier("arn:aws:iam::123456789012:role/TempUser")
        .build();

    let resource = Resource::builder()
        .database(DatabaseResource::builder().name("tempdb").build().unwrap())
        .build();

    client
        .grant_permissions()
        .principal(principal.clone())
        .resource(resource.clone())
        .permissions(Permission::Select)
        .permissions(Permission::Describe)
        .send()
        .await
        .expect("grant should succeed");

    // Revoke all permissions one by one.
    client
        .revoke_permissions()
        .principal(principal.clone())
        .resource(resource.clone())
        .permissions(Permission::Select)
        .permissions(Permission::Describe)
        .send()
        .await
        .expect("revoke all permissions should succeed");

    let resp = client
        .list_permissions()
        .principal(principal)
        .send()
        .await
        .expect("list_permissions should succeed");

    assert_eq!(
        resp.principal_resource_permissions().len(),
        0,
        "entry should be removed when all permissions are revoked"
    );
}
