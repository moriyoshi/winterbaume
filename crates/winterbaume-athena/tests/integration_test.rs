use aws_sdk_athena::config::BehaviorVersion;
use aws_sdk_athena::types::Tag;
use winterbaume_athena::AthenaService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_athena::Client {
    let mock = MockAws::builder()
        .with_service(AthenaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_athena::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_athena::Client::new(&config)
}

// ---- WorkGroup tests ----

#[tokio::test]
async fn test_create_and_get_work_group() {
    let client = make_client().await;

    client
        .create_work_group()
        .name("my-workgroup")
        .description("Test workgroup")
        .send()
        .await
        .expect("create_work_group should succeed");

    let resp = client
        .get_work_group()
        .work_group("my-workgroup")
        .send()
        .await
        .expect("get_work_group should succeed");

    let wg = resp.work_group().expect("should have work group");
    assert_eq!(wg.name(), "my-workgroup");
    assert_eq!(wg.description(), Some("Test workgroup"));
}

#[tokio::test]
async fn test_list_work_groups() {
    let client = make_client().await;

    for name in ["wg-a", "wg-b"] {
        client.create_work_group().name(name).send().await.unwrap();
    }

    let resp = client
        .list_work_groups()
        .send()
        .await
        .expect("list_work_groups should succeed");

    assert_eq!(resp.work_groups().len(), 2);
}

#[tokio::test]
async fn test_delete_work_group() {
    let client = make_client().await;

    client
        .create_work_group()
        .name("delete-me")
        .send()
        .await
        .unwrap();

    client
        .delete_work_group()
        .work_group("delete-me")
        .send()
        .await
        .expect("delete should succeed");

    let result = client.get_work_group().work_group("delete-me").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_work_group() {
    let client = make_client().await;

    let result = client
        .get_work_group()
        .work_group("nonexistent-wg")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_work_groups_empty() {
    let client = make_client().await;

    let resp = client
        .list_work_groups()
        .send()
        .await
        .expect("list_work_groups should succeed on empty state");

    assert_eq!(resp.work_groups().len(), 0);
}

// ---- CapacityReservation tests ----

#[tokio::test]
async fn test_create_and_get_capacity_reservation() {
    let client = make_client().await;

    client
        .create_capacity_reservation()
        .name("test-cr")
        .target_dpus(24)
        .send()
        .await
        .expect("create_capacity_reservation should succeed");

    let resp = client
        .get_capacity_reservation()
        .name("test-cr")
        .send()
        .await
        .expect("get_capacity_reservation should succeed");

    let cr = resp
        .capacity_reservation()
        .expect("should have capacity reservation");
    assert_eq!(cr.name(), "test-cr");
    assert_eq!(cr.target_dpus(), 24);
    assert_eq!(
        cr.status(),
        &aws_sdk_athena::types::CapacityReservationStatus::Active
    );
}

#[tokio::test]
async fn test_list_capacity_reservations() {
    let client = make_client().await;

    client
        .create_capacity_reservation()
        .name("cr-1")
        .target_dpus(24)
        .send()
        .await
        .unwrap();

    client
        .create_capacity_reservation()
        .name("cr-2")
        .target_dpus(48)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_capacity_reservations()
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(resp.capacity_reservations().len(), 2);
}

#[tokio::test]
async fn test_update_capacity_reservation() {
    let client = make_client().await;

    client
        .create_capacity_reservation()
        .name("update-cr")
        .target_dpus(24)
        .send()
        .await
        .unwrap();

    client
        .update_capacity_reservation()
        .name("update-cr")
        .target_dpus(48)
        .send()
        .await
        .expect("update should succeed");

    let resp = client
        .get_capacity_reservation()
        .name("update-cr")
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.capacity_reservation()
            .expect("should have cr")
            .target_dpus(),
        48
    );
}

#[tokio::test]
async fn test_get_nonexistent_capacity_reservation() {
    let client = make_client().await;

    let result = client
        .get_capacity_reservation()
        .name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- DataCatalog tests ----

#[tokio::test]
async fn test_create_and_get_data_catalog() {
    let client = make_client().await;

    client
        .create_data_catalog()
        .name("test-catalog")
        .r#type(aws_sdk_athena::types::DataCatalogType::Hive)
        .description("Test catalog")
        .send()
        .await
        .expect("create_data_catalog should succeed");

    let resp = client
        .get_data_catalog()
        .name("test-catalog")
        .send()
        .await
        .expect("get_data_catalog should succeed");

    let dc = resp.data_catalog().expect("should have data catalog");
    assert_eq!(dc.name(), "test-catalog");
    assert_eq!(dc.description(), Some("Test catalog"));
}

#[tokio::test]
async fn test_list_data_catalogs() {
    let client = make_client().await;

    client
        .create_data_catalog()
        .name("dc-1")
        .r#type(aws_sdk_athena::types::DataCatalogType::Hive)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_data_catalogs()
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(resp.data_catalogs_summary().len(), 1);
}

#[tokio::test]
async fn test_get_nonexistent_data_catalog() {
    let client = make_client().await;

    let result = client.get_data_catalog().name("nonexistent").send().await;
    assert!(result.is_err());
}

// ---- NamedQuery tests ----

#[tokio::test]
async fn test_create_and_get_named_query() {
    let client = make_client().await;

    let create_resp = client
        .create_named_query()
        .name("test-query")
        .database("default")
        .query_string("SELECT 1")
        .description("A test query")
        .send()
        .await
        .expect("create_named_query should succeed");

    let id = create_resp.named_query_id().expect("should have id");

    let resp = client
        .get_named_query()
        .named_query_id(id)
        .send()
        .await
        .expect("get_named_query should succeed");

    let nq = resp.named_query().expect("should have named query");
    assert_eq!(nq.name(), "test-query");
    assert_eq!(nq.database(), "default");
    assert_eq!(nq.query_string(), "SELECT 1");
}

#[tokio::test]
async fn test_list_named_queries() {
    let client = make_client().await;

    client
        .create_named_query()
        .name("q1")
        .database("db1")
        .query_string("SELECT 1")
        .send()
        .await
        .unwrap();

    client
        .create_named_query()
        .name("q2")
        .database("db2")
        .query_string("SELECT 2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_named_queries()
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(resp.named_query_ids().len(), 2);
}

#[tokio::test]
async fn test_get_nonexistent_named_query() {
    let client = make_client().await;

    let result = client
        .get_named_query()
        .named_query_id("nonexistent-id")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- PreparedStatement tests ----

#[tokio::test]
async fn test_create_and_get_prepared_statement() {
    let client = make_client().await;

    // Create a work group first
    client
        .create_work_group()
        .name("ps-wg")
        .send()
        .await
        .unwrap();

    client
        .create_prepared_statement()
        .statement_name("test-stmt")
        .work_group("ps-wg")
        .query_statement("SELECT ? FROM table1")
        .description("A test prepared statement")
        .send()
        .await
        .expect("create_prepared_statement should succeed");

    let resp = client
        .get_prepared_statement()
        .statement_name("test-stmt")
        .work_group("ps-wg")
        .send()
        .await
        .expect("get_prepared_statement should succeed");

    let ps = resp
        .prepared_statement()
        .expect("should have prepared statement");
    assert_eq!(ps.statement_name(), Some("test-stmt"));
    assert_eq!(ps.query_statement(), Some("SELECT ? FROM table1"));
}

#[tokio::test]
async fn test_get_nonexistent_prepared_statement() {
    let client = make_client().await;

    let result = client
        .get_prepared_statement()
        .statement_name("nonexistent")
        .work_group("primary")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- QueryExecution tests ----

#[tokio::test]
async fn test_start_and_get_query_execution() {
    let client = make_client().await;

    let start_resp = client
        .start_query_execution()
        .query_string("SELECT 1")
        .work_group("primary")
        .send()
        .await
        .expect("start_query_execution should succeed");

    let id = start_resp
        .query_execution_id()
        .expect("should have execution id");

    let resp = client
        .get_query_execution()
        .query_execution_id(id)
        .send()
        .await
        .expect("get_query_execution should succeed");

    let qe = resp.query_execution().expect("should have query execution");
    assert_eq!(qe.query(), Some("SELECT 1"));
    assert_eq!(
        qe.status().unwrap().state(),
        Some(&aws_sdk_athena::types::QueryExecutionState::Succeeded)
    );
}

#[tokio::test]
async fn test_get_query_results() {
    let client = make_client().await;

    let start_resp = client
        .start_query_execution()
        .query_string("SELECT 1")
        .send()
        .await
        .unwrap();

    let id = start_resp.query_execution_id().unwrap();

    let resp = client
        .get_query_results()
        .query_execution_id(id)
        .send()
        .await
        .expect("get_query_results should succeed");

    assert!(resp.result_set().is_some());
}

#[tokio::test]
async fn test_get_query_runtime_statistics() {
    let client = make_client().await;

    let start_resp = client
        .start_query_execution()
        .query_string("SELECT 1")
        .send()
        .await
        .unwrap();

    let id = start_resp.query_execution_id().unwrap();

    let resp = client
        .get_query_runtime_statistics()
        .query_execution_id(id)
        .send()
        .await
        .expect("get_query_runtime_statistics should succeed");

    let stats = resp
        .query_runtime_statistics()
        .expect("should have runtime statistics");
    assert!(stats.timeline().is_some());
}

#[tokio::test]
async fn test_stop_query_execution() {
    let client = make_client().await;

    let start_resp = client
        .start_query_execution()
        .query_string("SELECT 1")
        .send()
        .await
        .unwrap();

    let id = start_resp.query_execution_id().unwrap();

    client
        .stop_query_execution()
        .query_execution_id(id)
        .send()
        .await
        .expect("stop_query_execution should succeed");
}

#[tokio::test]
async fn test_list_query_executions() {
    let client = make_client().await;

    client
        .start_query_execution()
        .query_string("SELECT 1")
        .send()
        .await
        .unwrap();

    client
        .start_query_execution()
        .query_string("SELECT 2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_query_executions()
        .send()
        .await
        .expect("list should succeed");

    assert_eq!(resp.query_execution_ids().len(), 2);
}

#[tokio::test]
async fn test_get_nonexistent_query_execution() {
    let client = make_client().await;

    let result = client
        .get_query_execution()
        .query_execution_id("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Tags test ----

#[tokio::test]
async fn test_list_tags_for_resource() {
    let client = make_client().await;

    let resp = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:athena:us-east-1:123456789012:workgroup/primary")
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    // No tags exist yet, should return empty list
    assert_eq!(resp.tags().len(), 0);
}

// ---- Lifecycle test ----

#[tokio::test]
async fn test_query_execution_lifecycle() {
    let client = make_client().await;

    // Start a query
    let start_resp = client
        .start_query_execution()
        .query_string("SELECT * FROM my_table")
        .work_group("primary")
        .send()
        .await
        .expect("start should succeed");

    let id = start_resp.query_execution_id().unwrap();

    // Get execution details
    let get_resp = client
        .get_query_execution()
        .query_execution_id(id)
        .send()
        .await
        .expect("get should succeed");

    let qe = get_resp.query_execution().unwrap();
    assert_eq!(qe.query(), Some("SELECT * FROM my_table"));
    assert_eq!(qe.work_group(), Some("primary"));

    // Get results
    let results = client
        .get_query_results()
        .query_execution_id(id)
        .send()
        .await
        .expect("get results should succeed");

    assert!(results.result_set().is_some());

    // Get runtime stats
    let stats = client
        .get_query_runtime_statistics()
        .query_execution_id(id)
        .send()
        .await
        .expect("get stats should succeed");

    assert!(stats.query_runtime_statistics().is_some());

    // Verify it appears in list
    let list_resp = client
        .list_query_executions()
        .send()
        .await
        .expect("list should succeed");

    assert!(list_resp.query_execution_ids().contains(&id.to_string()));
}

// ============================================================================
// Ported from moto: test_athena.py
// ============================================================================

// Ported from moto: test_athena.py::test_create_work_group (duplicate error)
#[tokio::test]
async fn test_create_duplicate_work_group_fails() {
    let client = make_client().await;

    let config = aws_sdk_athena::types::WorkGroupConfiguration::builder()
        .result_configuration(
            aws_sdk_athena::types::ResultConfiguration::builder()
                .output_location("s3://bucket-name/prefix/")
                .build(),
        )
        .build();

    client
        .create_work_group()
        .name("athena_workgroup")
        .description("Test work group")
        .configuration(config.clone())
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_work_group()
        .name("athena_workgroup")
        .description("duplicate")
        .configuration(config)
        .send()
        .await;

    assert!(result.is_err(), "duplicate work group should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidRequestException") || err_str.contains("already exists"),
        "Expected InvalidRequestException, got: {err_str}"
    );
}

// Ported from moto: test_athena.py::test_create_data_catalog (duplicate error)
#[tokio::test]
async fn test_create_duplicate_data_catalog_fails() {
    let client = make_client().await;

    client
        .create_data_catalog()
        .name("athena_datacatalog")
        .r#type(aws_sdk_athena::types::DataCatalogType::Glue)
        .description("Test data catalog")
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_data_catalog()
        .name("athena_datacatalog")
        .r#type(aws_sdk_athena::types::DataCatalogType::Glue)
        .description("duplicate")
        .send()
        .await;

    assert!(result.is_err(), "duplicate data catalog should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidRequestException") || err_str.contains("already exists"),
        "Expected InvalidRequestException, got: {err_str}"
    );
}

// Ported from moto: test_athena.py::test_start_query_execution
// Verifies unique execution IDs are returned
#[tokio::test]
async fn test_start_query_unique_ids() {
    let client = make_client().await;

    client
        .create_work_group()
        .name("athena_workgroup")
        .description("Test work group")
        .configuration(
            aws_sdk_athena::types::WorkGroupConfiguration::builder()
                .result_configuration(
                    aws_sdk_athena::types::ResultConfiguration::builder()
                        .output_location("s3://bucket-name/prefix/")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp1 = client
        .start_query_execution()
        .query_string("query1")
        .query_execution_context(
            aws_sdk_athena::types::QueryExecutionContext::builder()
                .database("string")
                .build(),
        )
        .result_configuration(
            aws_sdk_athena::types::ResultConfiguration::builder()
                .output_location("string")
                .build(),
        )
        .work_group("athena_workgroup")
        .send()
        .await
        .unwrap();

    let resp2 = client
        .start_query_execution()
        .query_string("query2")
        .query_execution_context(
            aws_sdk_athena::types::QueryExecutionContext::builder()
                .database("string")
                .build(),
        )
        .result_configuration(
            aws_sdk_athena::types::ResultConfiguration::builder()
                .output_location("string")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let id1 = resp1.query_execution_id().unwrap();
    let id2 = resp2.query_execution_id().unwrap();
    assert_ne!(id1, id2, "Execution IDs should be unique");
}

// Ported from moto: test_athena.py::test_stop_query_execution
// Verifies status changes to CANCELLED after stop
#[tokio::test]
async fn test_stop_query_execution_cancelled_status() {
    let client = make_client().await;

    let exec_id = client
        .start_query_execution()
        .query_string("SELECT stuff")
        .query_execution_context(
            aws_sdk_athena::types::QueryExecutionContext::builder()
                .database("database")
                .build(),
        )
        .result_configuration(
            aws_sdk_athena::types::ResultConfiguration::builder()
                .output_location("s3://bucket-name/prefix/")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .query_execution_id()
        .unwrap()
        .to_string();

    client
        .stop_query_execution()
        .query_execution_id(&exec_id)
        .send()
        .await
        .unwrap();

    let details = client
        .get_query_execution()
        .query_execution_id(&exec_id)
        .send()
        .await
        .unwrap();

    let qe = details.query_execution().unwrap();
    assert_eq!(qe.query_execution_id(), Some(exec_id.as_str()));
    assert_eq!(
        qe.status().unwrap().state(),
        Some(&aws_sdk_athena::types::QueryExecutionState::Cancelled)
    );
}

// Ported from moto: test_athena.py::test_start_execution_with_workgroup
// Verifies workgroup appears in execution details
#[tokio::test]
async fn test_start_execution_with_workgroup() {
    let client = make_client().await;

    client
        .create_work_group()
        .name("myworkgroup")
        .description("Test work group")
        .configuration(
            aws_sdk_athena::types::WorkGroupConfiguration::builder()
                .result_configuration(
                    aws_sdk_athena::types::ResultConfiguration::builder()
                        .output_location("s3://bucket-name/prefix/")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let exec_id = client
        .start_query_execution()
        .query_string("SELECT stuff")
        .query_execution_context(
            aws_sdk_athena::types::QueryExecutionContext::builder()
                .database("database")
                .build(),
        )
        .result_configuration(
            aws_sdk_athena::types::ResultConfiguration::builder()
                .output_location("s3://bucket-name/prefix/")
                .build(),
        )
        .work_group("myworkgroup")
        .send()
        .await
        .unwrap()
        .query_execution_id()
        .unwrap()
        .to_string();

    let details = client
        .get_query_execution()
        .query_execution_id(&exec_id)
        .send()
        .await
        .unwrap();

    let qe = details.query_execution().unwrap();
    assert_eq!(qe.work_group(), Some("myworkgroup"));
}

// Ported from moto: test_athena.py::test_list_tags_for_resource (with data catalog tags)
#[tokio::test]
async fn test_list_tags_for_data_catalog() {
    let client = make_client().await;

    let tag1 = aws_sdk_athena::types::Tag::builder()
        .key("key1")
        .value("value1")
        .build();
    let tag2 = aws_sdk_athena::types::Tag::builder()
        .key("key2")
        .value("value2")
        .build();

    client
        .create_data_catalog()
        .name("athena_datacatalog")
        .r#type(aws_sdk_athena::types::DataCatalogType::Glue)
        .description("Test data catalog")
        .tags(tag1)
        .tags(tag2)
        .send()
        .await
        .unwrap();

    let resource_arn = "arn:aws:athena:us-east-1:123456789012:datacatalog/athena_datacatalog";
    let resp = client
        .list_tags_for_resource()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 2);
    assert!(
        tags.iter()
            .any(|t| t.key() == Some("key1") && t.value() == Some("value1"))
    );
    assert!(
        tags.iter()
            .any(|t| t.key() == Some("key2") && t.value() == Some("value2"))
    );
}

// Ported from moto: test_athena.py::test_list_tags_for_resource_not_found
#[tokio::test]
async fn test_list_tags_for_nonexistent_resource() {
    let client = make_client().await;

    let result = client
        .list_tags_for_resource()
        .resource_arn("NONEXISTENTRESOURCEARN")
        .send()
        .await;

    assert!(
        result.is_err(),
        "list_tags for nonexistent resource should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidRequestException") || err_str.contains("Does Not Exist"),
        "Expected InvalidRequestException or not found, got: {err_str}"
    );
}

// Ported from moto: test_athena.py::test_create_and_get_data_catalog
// Verifies detailed fields including parameters
#[tokio::test]
async fn test_create_and_get_data_catalog_with_params() {
    let client = make_client().await;

    client
        .create_data_catalog()
        .name("athena_datacatalog")
        .r#type(aws_sdk_athena::types::DataCatalogType::Glue)
        .description("Test data catalog")
        .parameters("catalog-id", "AWS Test account ID")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_data_catalog()
        .name("athena_datacatalog")
        .send()
        .await
        .unwrap();

    let dc = resp.data_catalog().unwrap();
    assert_eq!(dc.name(), "athena_datacatalog");
    assert_eq!(dc.description(), Some("Test data catalog"));
    let params = dc.parameters();
    assert!(params.is_some());
    let params = params.unwrap();
    assert_eq!(
        params.get("catalog-id"),
        Some(&"AWS Test account ID".to_string())
    );
}

// Ported from moto: test_athena.py::test_get_query_runtime_statistics_no_execution_id
#[tokio::test]
async fn test_get_query_runtime_statistics_nonexistent() {
    let client = make_client().await;

    let result = client
        .get_query_runtime_statistics()
        .query_execution_id("nonexistent-id-12345")
        .send()
        .await;

    assert!(
        result.is_err(),
        "get_query_runtime_statistics for nonexistent id should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidRequestException") || err_str.contains("was not found"),
        "Expected error about not found, got: {err_str}"
    );
}

// Ported from moto: test_athena.py::test_get_named_query
// Verifies all returned fields
#[tokio::test]
async fn test_get_named_query_fields() {
    let client = make_client().await;

    let create_resp = client
        .create_named_query()
        .name("query-name")
        .database("target_db")
        .query_string("SELECT * FROM tbl1")
        .description("description of this query")
        .send()
        .await
        .unwrap();

    let query_id = create_resp.named_query_id().unwrap();

    let resp = client
        .get_named_query()
        .named_query_id(query_id)
        .send()
        .await
        .unwrap();

    let nq = resp.named_query().unwrap();
    assert_eq!(nq.name(), "query-name");
    assert_eq!(nq.description(), Some("description of this query"));
    assert_eq!(nq.database(), "target_db");
    assert_eq!(nq.query_string(), "SELECT * FROM tbl1");
    assert_eq!(nq.named_query_id(), Some(query_id));
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Athena
// ============================================================================

#[tokio::test]
async fn test_delete_nonexistent_work_group() {
    let client = make_client().await;

    let result = client
        .delete_work_group()
        .work_group("nonexistent-wg")
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent workgroup should fail");
}

#[tokio::test]
async fn test_create_duplicate_capacity_reservation_fails() {
    let client = make_client().await;

    client
        .create_capacity_reservation()
        .name("dup-cr")
        .target_dpus(24)
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_capacity_reservation()
        .name("dup-cr")
        .target_dpus(48)
        .send()
        .await;
    assert!(
        result.is_err(),
        "duplicate capacity reservation should fail"
    );
}

#[tokio::test]
async fn test_update_capacity_reservation_not_found() {
    let client = make_client().await;

    let result = client
        .update_capacity_reservation()
        .name("nonexistent-cr")
        .target_dpus(48)
        .send()
        .await;
    assert!(
        result.is_err(),
        "update nonexistent capacity reservation should fail"
    );
}

#[tokio::test]
async fn test_named_query_workgroup_field() {
    let client = make_client().await;

    client
        .create_work_group()
        .name("named-query-wg")
        .send()
        .await
        .unwrap();

    let create_resp = client
        .create_named_query()
        .name("my-query")
        .work_group("named-query-wg")
        .database("default")
        .query_string("SELECT 1")
        .send()
        .await
        .expect("create_named_query should succeed");

    let query_id = create_resp.named_query_id().unwrap().to_string();
    let get_resp = client
        .get_named_query()
        .named_query_id(&query_id)
        .send()
        .await
        .unwrap();

    let query = get_resp.named_query().unwrap();
    assert_eq!(query.work_group(), Some("named-query-wg"));
}

#[tokio::test]
async fn test_create_duplicate_prepared_statement_fails() {
    let client = make_client().await;

    client
        .create_prepared_statement()
        .statement_name("dup-stmt")
        .work_group("primary")
        .query_statement("SELECT 1")
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_prepared_statement()
        .statement_name("dup-stmt")
        .work_group("primary")
        .query_statement("SELECT 2")
        .send()
        .await;
    assert!(result.is_err(), "duplicate prepared statement should fail");
}

#[tokio::test]
async fn test_query_execution_context_fields() {
    let client = make_client().await;

    let ctx = aws_sdk_athena::types::QueryExecutionContext::builder()
        .database("mydb")
        .catalog("AwsDataCatalog")
        .build();

    let resp = client
        .start_query_execution()
        .query_string("SELECT 1")
        .query_execution_context(ctx)
        .send()
        .await
        .expect("start_query_execution with context should succeed");

    let exec_id = resp.query_execution_id().unwrap().to_string();

    let get_resp = client
        .get_query_execution()
        .query_execution_id(&exec_id)
        .send()
        .await
        .unwrap();

    let exec = get_resp.query_execution().unwrap();
    let exec_ctx = exec.query_execution_context().unwrap();
    assert_eq!(exec_ctx.database(), Some("mydb"));
    assert_eq!(exec_ctx.catalog(), Some("AwsDataCatalog"));
}

// ---- TagResource / UntagResource tests ----

#[tokio::test]
async fn test_tag_and_untag_workgroup() {
    let client = make_client().await;

    // Create a workgroup first
    client
        .create_work_group()
        .name("tag-test-wg")
        .send()
        .await
        .expect("create_work_group should succeed");

    let arn = "arn:aws:athena:us-east-1:123456789012:workgroup/tag-test-wg";

    // Tag it
    client
        .tag_resource()
        .resource_arn(arn)
        .tags(Tag::builder().key("env").value("prod").build())
        .tags(Tag::builder().key("team").value("data").build())
        .send()
        .await
        .expect("tag_resource should succeed");

    // Verify tags
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");
    let tags = tags_resp.tags();
    assert_eq!(tags.len(), 2);

    // Untag one key
    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("team")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify only one tag remains
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();
    let tags = tags_resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("env"));
    assert_eq!(tags[0].value(), Some("prod"));
}

#[tokio::test]
async fn test_tag_resource_overwrites_existing_key() {
    let client = make_client().await;

    client
        .create_work_group()
        .name("tag-overwrite-wg")
        .send()
        .await
        .unwrap();

    let arn = "arn:aws:athena:us-east-1:123456789012:workgroup/tag-overwrite-wg";

    // Set initial tag
    client
        .tag_resource()
        .resource_arn(arn)
        .tags(Tag::builder().key("env").value("dev").build())
        .send()
        .await
        .unwrap();

    // Overwrite with new value
    client
        .tag_resource()
        .resource_arn(arn)
        .tags(Tag::builder().key("env").value("prod").build())
        .send()
        .await
        .unwrap();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .unwrap();
    let tags = tags_resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].value(), Some("prod"));
}

#[tokio::test]
async fn test_tag_nonexistent_resource_fails() {
    let client = make_client().await;

    let arn = "arn:aws:athena:us-east-1:123456789012:workgroup/does-not-exist";

    let result = client
        .tag_resource()
        .resource_arn(arn)
        .tags(Tag::builder().key("k").value("v").build())
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_untag_nonexistent_resource_fails() {
    let client = make_client().await;

    let arn = "arn:aws:athena:us-east-1:123456789012:workgroup/does-not-exist";

    let result = client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("k")
        .send()
        .await;
    assert!(result.is_err());
}
