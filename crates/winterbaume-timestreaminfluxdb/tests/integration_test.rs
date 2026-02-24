//! Integration tests for winterbaume Timestream for InfluxDB service.

use aws_sdk_timestreaminfluxdb::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_timestreaminfluxdb::TimestreamInfluxDbService;

async fn make_client() -> aws_sdk_timestreaminfluxdb::Client {
    let mock = MockAws::builder()
        .with_service(TimestreamInfluxDbService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_timestreaminfluxdb::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_timestreaminfluxdb::Client::new(&config)
}

async fn create_test_instance(
    client: &aws_sdk_timestreaminfluxdb::Client,
    name: &str,
) -> aws_sdk_timestreaminfluxdb::operation::create_db_instance::CreateDbInstanceOutput {
    client
        .create_db_instance()
        .name(name)
        .password("TestPassword123!")
        .db_instance_type(aws_sdk_timestreaminfluxdb::types::DbInstanceType::DbInfluxMedium)
        .allocated_storage(20)
        .vpc_subnet_ids("subnet-11111111")
        .vpc_subnet_ids("subnet-22222222")
        .vpc_security_group_ids("sg-11111111")
        .send()
        .await
        .expect("create_db_instance should succeed")
}

#[tokio::test]
async fn test_create_db_instance() {
    let client = make_client().await;

    let resp = create_test_instance(&client, "my-influxdb").await;

    assert_eq!(resp.name(), "my-influxdb");
    assert!(!resp.id().is_empty());
    assert!(resp.arn().contains("db-instance/"));
    assert!(resp.arn().contains("timestream-influxdb"));
    assert!(resp.endpoint().is_some());
    assert_eq!(resp.allocated_storage(), Some(20));
    assert!(!resp.vpc_subnet_ids().is_empty());
}

#[tokio::test]
async fn test_create_and_get_db_instance() {
    let client = make_client().await;

    let create_resp = create_test_instance(&client, "get-test-db").await;
    let instance_id = create_resp.id().to_string();

    let get_resp = client
        .get_db_instance()
        .identifier(&instance_id)
        .send()
        .await
        .expect("get_db_instance should succeed");

    assert_eq!(get_resp.id(), instance_id);
    assert_eq!(get_resp.name(), "get-test-db");
    assert_eq!(get_resp.arn(), create_resp.arn());
}

#[tokio::test]
async fn test_delete_db_instance() {
    let client = make_client().await;

    let create_resp = create_test_instance(&client, "del-test-db").await;
    let instance_id = create_resp.id().to_string();

    let delete_resp = client
        .delete_db_instance()
        .identifier(&instance_id)
        .send()
        .await
        .expect("delete_db_instance should succeed");

    assert_eq!(delete_resp.name(), "del-test-db");
    assert_eq!(
        delete_resp.status(),
        Some(&aws_sdk_timestreaminfluxdb::types::Status::Deleting)
    );

    // Verify it's gone
    let list_resp = client
        .list_db_instances()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(list_resp.items().len(), 0);
}

#[tokio::test]
async fn test_list_db_instances() {
    let client = make_client().await;

    for i in 0..3 {
        create_test_instance(&client, &format!("list-db-{i}")).await;
    }

    let resp = client
        .list_db_instances()
        .send()
        .await
        .expect("list_db_instances should succeed");

    assert_eq!(resp.items().len(), 3);
}

#[tokio::test]
async fn test_get_nonexistent_db_instance() {
    let client = make_client().await;

    let result = client
        .get_db_instance()
        .identifier("nonexistent123")
        .send()
        .await;

    assert!(result.is_err(), "get of nonexistent instance should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_db_instance() {
    let client = make_client().await;

    let result = client
        .delete_db_instance()
        .identifier("nonexistent123")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete of nonexistent instance should fail"
    );
}

#[tokio::test]
async fn test_create_duplicate_name() {
    let client = make_client().await;

    create_test_instance(&client, "dup-db").await;

    let result = client
        .create_db_instance()
        .name("dup-db")
        .password("TestPassword123!")
        .db_instance_type(aws_sdk_timestreaminfluxdb::types::DbInstanceType::DbInfluxMedium)
        .allocated_storage(20)
        .vpc_subnet_ids("subnet-11111111")
        .vpc_security_group_ids("sg-11111111")
        .send()
        .await;

    assert!(
        result.is_err(),
        "creating a duplicate-named instance should fail"
    );
}

// --- DbCluster tests ---

#[tokio::test]
async fn test_create_db_cluster() {
    let client = make_client().await;

    let resp = client
        .create_db_cluster()
        .name("my-cluster")
        .password("TestPassword123!")
        .db_instance_type(aws_sdk_timestreaminfluxdb::types::DbInstanceType::DbInfluxMedium)
        .allocated_storage(20)
        .vpc_subnet_ids("subnet-11111111")
        .vpc_subnet_ids("subnet-22222222")
        .vpc_security_group_ids("sg-11111111")
        .send()
        .await
        .expect("create_db_cluster should succeed");

    assert!(resp.db_cluster_id().is_some());
    assert!(resp.db_cluster_status().is_some());
}

#[tokio::test]
async fn test_create_and_get_db_cluster() {
    let client = make_client().await;

    let create_resp = client
        .create_db_cluster()
        .name("get-cluster")
        .password("TestPassword123!")
        .db_instance_type(aws_sdk_timestreaminfluxdb::types::DbInstanceType::DbInfluxMedium)
        .allocated_storage(20)
        .vpc_subnet_ids("subnet-11111111")
        .vpc_security_group_ids("sg-11111111")
        .send()
        .await
        .expect("create_db_cluster should succeed");

    let cluster_id = create_resp.db_cluster_id().unwrap().to_string();

    let get_resp = client
        .get_db_cluster()
        .db_cluster_id(&cluster_id)
        .send()
        .await
        .expect("get_db_cluster should succeed");

    assert_eq!(get_resp.id(), cluster_id);
    assert_eq!(get_resp.name(), "get-cluster");
    assert!(get_resp.arn().contains("db-cluster/"));
}

#[tokio::test]
async fn test_list_db_clusters() {
    let client = make_client().await;

    for i in 0..3 {
        client
            .create_db_cluster()
            .name(format!("list-cluster-{i}"))
            .password("TestPassword123!")
            .db_instance_type(aws_sdk_timestreaminfluxdb::types::DbInstanceType::DbInfluxMedium)
            .allocated_storage(20)
            .vpc_subnet_ids("subnet-11111111")
            .vpc_security_group_ids("sg-11111111")
            .send()
            .await
            .expect("create_db_cluster should succeed");
    }

    let resp = client
        .list_db_clusters()
        .send()
        .await
        .expect("list_db_clusters should succeed");

    assert_eq!(resp.items().len(), 3);
}

// --- DbParameterGroup tests ---

#[tokio::test]
async fn test_create_db_parameter_group() {
    let client = make_client().await;

    let resp = client
        .create_db_parameter_group()
        .name("my-param-group")
        .description("Test parameter group")
        .send()
        .await
        .expect("create_db_parameter_group should succeed");

    assert!(!resp.id().is_empty());
    assert_eq!(resp.name(), "my-param-group");
    assert!(resp.arn().contains("db-parameter-group/"));
    assert_eq!(resp.description(), Some("Test parameter group"));
}

#[tokio::test]
async fn test_create_and_get_db_parameter_group() {
    let client = make_client().await;

    let create_resp = client
        .create_db_parameter_group()
        .name("get-pg")
        .description("Test desc")
        .send()
        .await
        .expect("create_db_parameter_group should succeed");

    let pg_id = create_resp.id().to_string();

    let get_resp = client
        .get_db_parameter_group()
        .identifier(&pg_id)
        .send()
        .await
        .expect("get_db_parameter_group should succeed");

    assert_eq!(get_resp.id(), pg_id);
    assert_eq!(get_resp.name(), "get-pg");
    assert_eq!(get_resp.description(), Some("Test desc"));
}

#[tokio::test]
async fn test_list_db_parameter_groups() {
    let client = make_client().await;

    for i in 0..2 {
        client
            .create_db_parameter_group()
            .name(format!("list-pg-{i}"))
            .send()
            .await
            .expect("create_db_parameter_group should succeed");
    }

    let resp = client
        .list_db_parameter_groups()
        .send()
        .await
        .expect("list_db_parameter_groups should succeed");

    assert_eq!(resp.items().len(), 2);
}

// --- Tag operation tests ---

#[tokio::test]
async fn test_tag_and_list_tags_for_db_instance() {
    let client = make_client().await;

    let create_resp = create_test_instance(&client, "tag-test-db").await;
    let arn = create_resp.arn().to_string();

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "test")
        .tags("team", "backend")
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("test"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("backend"));
}

// ============================================================================
// Tests derived from AWS documentation: Timestream for InfluxDB
// ============================================================================

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    let create_resp = create_test_instance(&client, "untag-test-db").await;
    let arn = create_resp.arn().to_string();

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "test")
        .tags("team", "backend")
        .send()
        .await
        .expect("tag_resource should succeed");

    // Untag
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // List tags again
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    assert!(tags.get("env").is_none());
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("backend"));
}

// --- Additional DbInstance tests ---

#[tokio::test]
async fn test_create_db_instance_with_optional_fields() {
    let client = make_client().await;

    let resp = client
        .create_db_instance()
        .name("optional-test-db")
        .password("TestPassword123!")
        .db_instance_type(aws_sdk_timestreaminfluxdb::types::DbInstanceType::DbInfluxMedium)
        .allocated_storage(20)
        .vpc_subnet_ids("subnet-11111111")
        .vpc_security_group_ids("sg-11111111")
        .db_storage_type(aws_sdk_timestreaminfluxdb::types::DbStorageType::InfluxIoIncludedT1)
        .deployment_type(aws_sdk_timestreaminfluxdb::types::DeploymentType::SingleAz)
        .publicly_accessible(true)
        .send()
        .await
        .expect("create_db_instance with optional fields should succeed");

    let instance_id = resp.id().to_string();
    assert!(!instance_id.is_empty());

    let get_resp = client
        .get_db_instance()
        .identifier(&instance_id)
        .send()
        .await
        .expect("get_db_instance should succeed");

    assert_eq!(
        get_resp.db_storage_type(),
        Some(&aws_sdk_timestreaminfluxdb::types::DbStorageType::InfluxIoIncludedT1)
    );
    assert_eq!(
        get_resp.deployment_type(),
        Some(&aws_sdk_timestreaminfluxdb::types::DeploymentType::SingleAz)
    );
    assert_eq!(get_resp.publicly_accessible(), Some(true));
}

#[tokio::test]
async fn test_create_db_instance_with_tags() {
    let client = make_client().await;

    let resp = client
        .create_db_instance()
        .name("tags-create-db")
        .password("TestPassword123!")
        .db_instance_type(aws_sdk_timestreaminfluxdb::types::DbInstanceType::DbInfluxMedium)
        .allocated_storage(20)
        .vpc_subnet_ids("subnet-11111111")
        .vpc_security_group_ids("sg-11111111")
        .tags("project", "influx-test")
        .tags("owner", "team-alpha")
        .send()
        .await
        .expect("create_db_instance with tags should succeed");

    let arn = resp.arn().to_string();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("project").map(|s| s.as_str()), Some("influx-test"));
    assert_eq!(tags.get("owner").map(|s| s.as_str()), Some("team-alpha"));
}

#[tokio::test]
async fn test_get_db_instance_not_found_error_type() {
    let client = make_client().await;

    let err = client
        .get_db_instance()
        .identifier("doesnotexist999")
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
async fn test_delete_db_instance_not_found_error_type() {
    let client = make_client().await;

    let err = client
        .delete_db_instance()
        .identifier("doesnotexist999")
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
async fn test_db_instance_lifecycle() {
    let client = make_client().await;

    // Create
    let create_resp = create_test_instance(&client, "lifecycle-db").await;
    let instance_id = create_resp.id().to_string();

    // Get
    let get_resp = client
        .get_db_instance()
        .identifier(&instance_id)
        .send()
        .await
        .expect("get should succeed after create");
    assert_eq!(get_resp.name(), "lifecycle-db");

    // Delete
    client
        .delete_db_instance()
        .identifier(&instance_id)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let get_after_delete = client
        .get_db_instance()
        .identifier(&instance_id)
        .send()
        .await;
    assert!(
        get_after_delete.is_err(),
        "get after delete should fail with not-found"
    );
}

#[tokio::test]
async fn test_db_instance_arn_format() {
    let client = make_client().await;

    let resp = create_test_instance(&client, "arn-check-db").await;

    let arn = resp.arn();
    assert!(
        arn.contains("timestream-influxdb"),
        "ARN should contain service name: {arn}"
    );
    assert!(
        arn.contains("db-instance/"),
        "ARN should contain resource type: {arn}"
    );
    assert!(
        arn.starts_with("arn:aws:"),
        "ARN should start with arn:aws:: {arn}"
    );
}

#[tokio::test]
async fn test_list_db_instances_summary_fields() {
    let client = make_client().await;

    create_test_instance(&client, "summary-test-db").await;

    let resp = client
        .list_db_instances()
        .send()
        .await
        .expect("list_db_instances should succeed");

    assert_eq!(resp.items().len(), 1);
    let item = &resp.items()[0];
    assert_eq!(item.name(), "summary-test-db");
    assert!(!item.id().is_empty());
    assert!(item.arn().contains("db-instance/"));
    assert!(item.status().is_some());
    assert!(item.db_instance_type().is_some());
}

// --- Additional DbCluster tests ---

#[tokio::test]
async fn test_create_db_cluster_duplicate_name() {
    let client = make_client().await;

    client
        .create_db_cluster()
        .name("dup-cluster")
        .password("TestPassword123!")
        .db_instance_type(aws_sdk_timestreaminfluxdb::types::DbInstanceType::DbInfluxMedium)
        .allocated_storage(20)
        .vpc_subnet_ids("subnet-11111111")
        .vpc_security_group_ids("sg-11111111")
        .send()
        .await
        .expect("first create_db_cluster should succeed");

    let result = client
        .create_db_cluster()
        .name("dup-cluster")
        .password("TestPassword123!")
        .db_instance_type(aws_sdk_timestreaminfluxdb::types::DbInstanceType::DbInfluxMedium)
        .allocated_storage(20)
        .vpc_subnet_ids("subnet-11111111")
        .vpc_security_group_ids("sg-11111111")
        .send()
        .await;

    assert!(
        result.is_err(),
        "creating a duplicate-named cluster should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_db_cluster_not_found() {
    let client = make_client().await;

    let err = client
        .get_db_cluster()
        .db_cluster_id("nonexistentcluster123")
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
async fn test_db_cluster_arn_format() {
    let client = make_client().await;

    let create_resp = client
        .create_db_cluster()
        .name("arn-cluster")
        .password("TestPassword123!")
        .db_instance_type(aws_sdk_timestreaminfluxdb::types::DbInstanceType::DbInfluxMedium)
        .allocated_storage(20)
        .vpc_subnet_ids("subnet-11111111")
        .vpc_security_group_ids("sg-11111111")
        .send()
        .await
        .expect("create_db_cluster should succeed");

    let cluster_id = create_resp.db_cluster_id().unwrap().to_string();

    let get_resp = client
        .get_db_cluster()
        .db_cluster_id(&cluster_id)
        .send()
        .await
        .expect("get_db_cluster should succeed");

    let arn = get_resp.arn();
    assert!(
        arn.contains("timestream-influxdb"),
        "ARN should contain service name: {arn}"
    );
    assert!(
        arn.contains("db-cluster/"),
        "ARN should contain resource type: {arn}"
    );
    assert!(
        arn.starts_with("arn:aws:"),
        "ARN should start with arn:aws:: {arn}"
    );
}

#[tokio::test]
async fn test_create_db_cluster_with_optional_fields() {
    let client = make_client().await;

    let create_resp = client
        .create_db_cluster()
        .name("optional-cluster")
        .password("TestPassword123!")
        .db_instance_type(aws_sdk_timestreaminfluxdb::types::DbInstanceType::DbInfluxMedium)
        .allocated_storage(20)
        .vpc_subnet_ids("subnet-11111111")
        .vpc_security_group_ids("sg-11111111")
        .network_type(aws_sdk_timestreaminfluxdb::types::NetworkType::Ipv4)
        .db_storage_type(aws_sdk_timestreaminfluxdb::types::DbStorageType::InfluxIoIncludedT1)
        .send()
        .await
        .expect("create_db_cluster with optional fields should succeed");

    let cluster_id = create_resp.db_cluster_id().unwrap().to_string();

    let get_resp = client
        .get_db_cluster()
        .db_cluster_id(&cluster_id)
        .send()
        .await
        .expect("get_db_cluster should succeed");

    assert_eq!(
        get_resp.network_type(),
        Some(&aws_sdk_timestreaminfluxdb::types::NetworkType::Ipv4)
    );
    assert_eq!(
        get_resp.db_storage_type(),
        Some(&aws_sdk_timestreaminfluxdb::types::DbStorageType::InfluxIoIncludedT1)
    );
}

// --- Additional DbParameterGroup tests ---

#[tokio::test]
async fn test_create_db_parameter_group_duplicate_name() {
    let client = make_client().await;

    client
        .create_db_parameter_group()
        .name("dup-param-group")
        .description("First group")
        .send()
        .await
        .expect("first create_db_parameter_group should succeed");

    let result = client
        .create_db_parameter_group()
        .name("dup-param-group")
        .description("Second group attempt")
        .send()
        .await;

    assert!(
        result.is_err(),
        "creating a duplicate-named parameter group should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_db_parameter_group_not_found() {
    let client = make_client().await;

    let err = client
        .get_db_parameter_group()
        .identifier("doesnotexist999")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// --- Tagging on clusters and parameter groups ---

#[tokio::test]
async fn test_tag_and_list_tags_for_cluster() {
    let client = make_client().await;

    let create_resp = client
        .create_db_cluster()
        .name("tag-cluster-test")
        .password("TestPassword123!")
        .db_instance_type(aws_sdk_timestreaminfluxdb::types::DbInstanceType::DbInfluxMedium)
        .allocated_storage(20)
        .vpc_subnet_ids("subnet-11111111")
        .vpc_security_group_ids("sg-11111111")
        .send()
        .await
        .expect("create_db_cluster should succeed");

    let cluster_id = create_resp.db_cluster_id().unwrap().to_string();
    let get_resp = client
        .get_db_cluster()
        .db_cluster_id(&cluster_id)
        .send()
        .await
        .expect("get_db_cluster should succeed");
    let arn = get_resp.arn().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "staging")
        .tags("team", "data")
        .send()
        .await
        .expect("tag_resource on cluster should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource on cluster should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("staging"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("data"));
}

#[tokio::test]
async fn test_tag_and_list_tags_for_parameter_group() {
    let client = make_client().await;

    let create_resp = client
        .create_db_parameter_group()
        .name("tag-pg-test")
        .description("Tagging test parameter group")
        .send()
        .await
        .expect("create_db_parameter_group should succeed");

    let arn = create_resp.arn().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("environment", "prod")
        .send()
        .await
        .expect("tag_resource on parameter group should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource on parameter group should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("environment").map(|s| s.as_str()), Some("prod"));
}

#[tokio::test]
async fn test_list_tags_for_nonexistent_resource() {
    let client = make_client().await;

    let err = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:timestream-influxdb:us-east-1:123456789012:db-instance/nonexistent")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}
