use aws_sdk_redshiftdata::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_redshiftdata::RedshiftDataService;

async fn make_redshiftdata_client() -> aws_sdk_redshiftdata::Client {
    let mock = MockAws::builder()
        .with_service(RedshiftDataService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_redshiftdata::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_redshiftdata::Client::new(&config)
}

#[tokio::test]
async fn test_execute_and_describe_statement() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT 1")
        .database("dev")
        .cluster_identifier("my-cluster")
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have an Id");
    assert!(!stmt_id.is_empty());

    let desc_resp = client
        .describe_statement()
        .id(stmt_id)
        .send()
        .await
        .expect("describe_statement should succeed");

    assert_eq!(desc_resp.id(), stmt_id);
    assert_eq!(desc_resp.query_string(), Some("SELECT 1"));
}

#[tokio::test]
async fn test_execute_and_cancel_statement() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT * FROM large_table")
        .database("dev")
        .workgroup_name("default")
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have an Id");

    let cancel_resp = client
        .cancel_statement()
        .id(stmt_id)
        .send()
        .await
        .expect("cancel_statement should succeed");

    // Statements are immediately FINISHED by the in-memory backend, so cancel
    // returns false (cannot cancel an already-completed statement).
    assert_eq!(cancel_resp.status(), Some(false));
}

#[tokio::test]
async fn test_list_statements_empty() {
    let client = make_redshiftdata_client().await;

    let resp = client
        .list_statements()
        .send()
        .await
        .expect("list_statements should succeed");

    assert_eq!(resp.statements().len(), 0);
}

#[tokio::test]
async fn test_list_statements_after_execute() {
    let client = make_redshiftdata_client().await;

    client
        .execute_statement()
        .sql("SELECT 1")
        .database("dev")
        .cluster_identifier("cluster-1")
        .send()
        .await
        .expect("first execute should succeed");

    client
        .execute_statement()
        .sql("SELECT 2")
        .database("dev")
        .cluster_identifier("cluster-1")
        .send()
        .await
        .expect("second execute should succeed");

    let resp = client
        .list_statements()
        .send()
        .await
        .expect("list_statements should succeed");

    assert_eq!(resp.statements().len(), 2);
}

#[tokio::test]
async fn test_describe_nonexistent_statement() {
    let client = make_redshiftdata_client().await;

    let result = client
        .describe_statement()
        .id("nonexistent-id-12345")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe of nonexistent statement should fail"
    );
}

#[tokio::test]
async fn test_cancel_nonexistent_statement() {
    let client = make_redshiftdata_client().await;

    let result = client
        .cancel_statement()
        .id("nonexistent-id-67890")
        .send()
        .await;

    assert!(
        result.is_err(),
        "cancel of nonexistent statement should fail"
    );
}

#[tokio::test]
async fn test_execute_statement_missing_sql() {
    let client = make_redshiftdata_client().await;

    let result = client
        .execute_statement()
        .database("dev")
        .cluster_identifier("cluster-1")
        .send()
        .await;

    assert!(result.is_err(), "execute without sql should fail");
}

// --- New moto-ported tests ---

#[tokio::test]
async fn test_cancel_invalid_uuid_raises_validation_exception() {
    let client = make_redshiftdata_client().await;

    let result = client.cancel_statement().id("test").send().await;

    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_validation_exception(),
        "should be ValidationException, got: {:?}",
        service_err
    );
}

#[tokio::test]
async fn test_cancel_valid_uuid_not_found_raises_resource_not_found() {
    let client = make_redshiftdata_client().await;

    let result = client
        .cancel_statement()
        .id("890f1253-595b-4608-a0d1-73f933ccd0a0")
        .send()
        .await;

    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_resource_not_found_exception(),
        "should be ResourceNotFoundException, got: {:?}",
        service_err
    );
}

#[tokio::test]
async fn test_describe_invalid_uuid_raises_validation_exception() {
    let client = make_redshiftdata_client().await;

    let result = client.describe_statement().id("test").send().await;

    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_validation_exception(),
        "should be ValidationException, got: {:?}",
        service_err
    );
}

#[tokio::test]
async fn test_describe_valid_uuid_not_found_raises_resource_not_found() {
    let client = make_redshiftdata_client().await;

    let result = client
        .describe_statement()
        .id("890f1253-595b-4608-a0d1-73f933ccd0a0")
        .send()
        .await;

    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_resource_not_found_exception(),
        "should be ResourceNotFoundException, got: {:?}",
        service_err
    );
}

#[tokio::test]
async fn test_get_statement_result_invalid_uuid_raises_validation_exception() {
    let client = make_redshiftdata_client().await;

    let result = client.get_statement_result().id("test").send().await;

    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_validation_exception(),
        "should be ValidationException, got: {:?}",
        service_err
    );
}

#[tokio::test]
async fn test_get_statement_result_valid_uuid_not_found_raises_resource_not_found() {
    let client = make_redshiftdata_client().await;

    let result = client
        .get_statement_result()
        .id("890f1253-595b-4608-a0d1-73f933ccd0a0")
        .send()
        .await;

    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_resource_not_found_exception(),
        "should be ResourceNotFoundException, got: {:?}",
        service_err
    );
}

#[tokio::test]
async fn test_execute_and_describe_with_parameters() {
    let client = make_redshiftdata_client().await;

    use aws_sdk_redshiftdata::types::SqlParameter;

    let exec_resp = client
        .execute_statement()
        .sql("sql")
        .database("database")
        .cluster_identifier("cluster_identifier")
        .db_user("db_user")
        .secret_arn("secret_arn")
        .parameters(
            SqlParameter::builder()
                .name("name")
                .value("value")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have id");

    let desc = client
        .describe_statement()
        .id(stmt_id)
        .send()
        .await
        .expect("describe_statement should succeed");

    assert_eq!(desc.cluster_identifier(), Some("cluster_identifier"));
    assert_eq!(desc.database(), Some("database"));
    assert_eq!(desc.db_user(), Some("db_user"));
    assert_eq!(desc.secret_arn(), Some("secret_arn"));
    assert_eq!(desc.query_string(), Some("sql"));
    assert_eq!(desc.status().map(|s| s.as_str()), Some("FINISHED"));
    let params = desc.query_parameters();
    assert_eq!(params.len(), 1);
    assert_eq!(params[0].name(), "name");
    assert_eq!(params[0].value(), "value");
}

#[tokio::test]
async fn test_execute_and_get_statement_result_mock_data() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("sql")
        .database("database")
        .cluster_identifier("cluster_identifier")
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have id");

    let result = client
        .get_statement_result()
        .id(stmt_id)
        .send()
        .await
        .expect("get_statement_result should succeed");

    let col_metadata = result.column_metadata();
    assert_eq!(col_metadata.len(), 3);
    assert_eq!(col_metadata[0].name(), Some("Number"));
    assert_eq!(col_metadata[1].name(), Some("Street"));
    assert_eq!(col_metadata[2].name(), Some("City"));

    let records = result.records();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0][0].as_long_value(), Ok(&10_i64));
    assert_eq!(
        records[1][1].as_string_value().map(|s| s.as_str()),
        Ok("Beta st")
    );
    assert_eq!(
        records[2][2].as_string_value().map(|s| s.as_str()),
        Ok("Seattle")
    );
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Redshift Data API
// ============================================================================

#[tokio::test]
async fn test_execute_statement_missing_database() {
    let client = make_redshiftdata_client().await;

    // Database is required when authenticating via temporary credentials
    let result = client
        .execute_statement()
        .sql("SELECT 1")
        .cluster_identifier("my-cluster")
        .send()
        .await;

    assert!(result.is_err(), "execute without database should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ValidationException"),
        "Expected ValidationException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_execute_statement_response_fields() {
    let client = make_redshiftdata_client().await;

    let resp = client
        .execute_statement()
        .sql("SELECT 1")
        .database("mydb")
        .cluster_identifier("mycluster")
        .send()
        .await
        .expect("execute_statement should succeed");

    // Id must be present and non-empty
    let id = resp.id().expect("response must have Id");
    assert!(!id.is_empty(), "Id must be non-empty");

    // CreatedAt must be present
    assert!(resp.created_at().is_some(), "response must have CreatedAt");

    // Database must be echoed back
    assert_eq!(resp.database(), Some("mydb"));

    // ClusterIdentifier must be echoed back
    assert_eq!(resp.cluster_identifier(), Some("mycluster"));
}

#[tokio::test]
async fn test_execute_statement_workgroup_response_fields() {
    let client = make_redshiftdata_client().await;

    let resp = client
        .execute_statement()
        .sql("SELECT 2")
        .database("svrdb")
        .workgroup_name("my-workgroup")
        .send()
        .await
        .expect("execute_statement with workgroup should succeed");

    assert_eq!(resp.workgroup_name(), Some("my-workgroup"));
    assert_eq!(resp.database(), Some("svrdb"));
    // cluster_identifier should be absent when using workgroup
    assert!(
        resp.cluster_identifier().is_none(),
        "cluster_identifier should be absent for workgroup connections"
    );
}

#[tokio::test]
async fn test_describe_statement_timestamps() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT 1")
        .database("dev")
        .cluster_identifier("my-cluster")
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have Id");

    let desc = client
        .describe_statement()
        .id(stmt_id)
        .send()
        .await
        .expect("describe_statement should succeed");

    // Both created_at and updated_at must be present per AWS docs
    assert!(desc.created_at().is_some(), "created_at must be present");
    assert!(desc.updated_at().is_some(), "updated_at must be present");
}

#[tokio::test]
async fn test_describe_statement_has_result_set() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT 1")
        .database("dev")
        .cluster_identifier("my-cluster")
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have Id");

    let desc = client
        .describe_statement()
        .id(stmt_id)
        .send()
        .await
        .expect("describe_statement should succeed");

    // has_result_set field must be present (true for SELECT, per state.rs: has_result_set: true)
    assert!(
        desc.has_result_set().is_some(),
        "has_result_set must be present"
    );
    assert_eq!(
        desc.has_result_set(),
        Some(true),
        "has_result_set must be true for SELECT statements"
    );
}

#[tokio::test]
async fn test_describe_statement_duration() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT 1")
        .database("dev")
        .cluster_identifier("my-cluster")
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have Id");

    let desc = client
        .describe_statement()
        .id(stmt_id)
        .send()
        .await
        .expect("describe_statement should succeed");

    // duration is a plain i64 (0 in mock)
    assert_eq!(desc.duration(), 0_i64, "duration must be 0 in mock");
}

#[tokio::test]
async fn test_describe_statement_status_started() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT 42")
        .database("dev")
        .cluster_identifier("my-cluster")
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have Id");

    let desc = client
        .describe_statement()
        .id(stmt_id)
        .send()
        .await
        .expect("describe_statement should succeed");

    // The in-memory backend completes statements immediately.
    assert_eq!(
        desc.status().map(|s| s.as_str()),
        Some("FINISHED"),
        "Status must be FINISHED after in-memory execution"
    );
}

#[tokio::test]
async fn test_execute_statement_with_workgroup_describe() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT 99")
        .database("wgdb")
        .workgroup_name("test-workgroup")
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have Id");

    let desc = client
        .describe_statement()
        .id(stmt_id)
        .send()
        .await
        .expect("describe_statement should succeed");

    assert_eq!(desc.workgroup_name(), Some("test-workgroup"));
    assert_eq!(desc.database(), Some("wgdb"));
}

#[tokio::test]
async fn test_list_statements_contains_ids() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT 1")
        .database("dev")
        .cluster_identifier("cluster-list-test")
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have Id");

    let list_resp = client
        .list_statements()
        .send()
        .await
        .expect("list_statements should succeed");

    let found = list_resp.statements().iter().any(|s| s.id() == stmt_id);

    assert!(found, "executed statement ID should appear in list");
}

#[tokio::test]
async fn test_list_statements_entry_fields() {
    let client = make_redshiftdata_client().await;

    client
        .execute_statement()
        .sql("SELECT field_test")
        .database("fielddb")
        .cluster_identifier("field-cluster")
        .send()
        .await
        .expect("execute_statement should succeed");

    let list_resp = client
        .list_statements()
        .send()
        .await
        .expect("list_statements should succeed");

    assert!(
        !list_resp.statements().is_empty(),
        "list must have at least one entry"
    );
    let entry = &list_resp.statements()[0];

    // Each StatementData entry must have id and status per AWS docs
    assert!(!entry.id().is_empty(), "entry must have non-empty id");
    assert!(entry.status().is_some(), "entry must have status");
    assert!(entry.created_at().is_some(), "entry must have created_at");
}

#[tokio::test]
async fn test_cancel_already_cancelled_returns_false() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT slow")
        .database("dev")
        .cluster_identifier("cancel-cluster")
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have Id");

    // Statement is immediately FINISHED by the in-memory backend; both cancels
    // return false (cannot cancel an already-completed statement).
    let first_cancel = client
        .cancel_statement()
        .id(stmt_id)
        .send()
        .await
        .expect("first cancel should succeed");
    assert_eq!(first_cancel.status(), Some(false));

    // Second cancel also returns false (idempotent on completed statements).
    let second_cancel = client
        .cancel_statement()
        .id(stmt_id)
        .send()
        .await
        .expect("second cancel should succeed (returns false, not an error)");
    assert_eq!(second_cancel.status(), Some(false));
}

#[tokio::test]
async fn test_get_statement_result_total_num_rows() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT * FROM addresses")
        .database("dev")
        .cluster_identifier("result-cluster")
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have Id");

    let result = client
        .get_statement_result()
        .id(stmt_id)
        .send()
        .await
        .expect("get_statement_result should succeed");

    // Mock always returns 3 rows
    assert_eq!(result.total_num_rows(), 3_i64);
}

#[tokio::test]
async fn test_get_statement_result_column_count() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT number, street, city FROM addresses")
        .database("dev")
        .cluster_identifier("col-cluster")
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have Id");

    let result = client
        .get_statement_result()
        .id(stmt_id)
        .send()
        .await
        .expect("get_statement_result should succeed");

    // Mock returns 3 columns: Number, Street, City
    assert_eq!(result.column_metadata().len(), 3);
    let names: Vec<Option<&str>> = result.column_metadata().iter().map(|c| c.name()).collect();
    assert_eq!(names, vec![Some("Number"), Some("Street"), Some("City")]);
}

#[tokio::test]
async fn test_describe_statement_after_cancel() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT 1")
        .database("dev")
        .cluster_identifier("cancel-desc-cluster")
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have Id");

    client
        .cancel_statement()
        .id(stmt_id)
        .send()
        .await
        .expect("cancel should succeed");

    // After cancel, describe should still work and show ABORTED status
    let desc = client
        .describe_statement()
        .id(stmt_id)
        .send()
        .await
        .expect("describe after cancel should succeed");

    // The InMemory backend immediately marks statements as FINISHED, so cancel
    // has no effect and the status remains FINISHED.
    assert_eq!(
        desc.status().map(|s| s.as_str()),
        Some("FINISHED"),
        "Status after cancel on a FINISHED statement must remain FINISHED"
    );
}

// ============================================================================
// Tests for BatchExecuteStatement
// ============================================================================

#[tokio::test]
async fn test_batch_execute_statement_basic() {
    let client = make_redshiftdata_client().await;

    let resp = client
        .batch_execute_statement()
        .sqls("CREATE TABLE t1 (id INT)")
        .sqls("INSERT INTO t1 VALUES (1)")
        .database("dev")
        .cluster_identifier("batch-cluster")
        .send()
        .await
        .expect("batch_execute_statement should succeed");

    let id = resp.id().expect("response must have Id");
    assert!(!id.is_empty(), "Id must be non-empty");
    assert!(resp.created_at().is_some(), "response must have CreatedAt");
    assert_eq!(resp.database(), Some("dev"));
    assert_eq!(resp.cluster_identifier(), Some("batch-cluster"));
}

#[tokio::test]
async fn test_batch_execute_statement_with_workgroup() {
    let client = make_redshiftdata_client().await;

    let resp = client
        .batch_execute_statement()
        .sqls("SELECT 1")
        .sqls("SELECT 2")
        .database("wgdb")
        .workgroup_name("batch-workgroup")
        .send()
        .await
        .expect("batch_execute_statement with workgroup should succeed");

    assert_eq!(resp.workgroup_name(), Some("batch-workgroup"));
    assert_eq!(resp.database(), Some("wgdb"));
    assert!(
        resp.cluster_identifier().is_none(),
        "cluster_identifier should be absent for workgroup connections"
    );
}

#[tokio::test]
async fn test_batch_execute_statement_describe_shows_finished() {
    let client = make_redshiftdata_client().await;

    let resp = client
        .batch_execute_statement()
        .sqls("SELECT 1")
        .database("dev")
        .cluster_identifier("batch-desc-cluster")
        .send()
        .await
        .expect("batch_execute_statement should succeed");

    let stmt_id = resp.id().expect("should have Id");

    let desc = client
        .describe_statement()
        .id(stmt_id)
        .send()
        .await
        .expect("describe after batch execute should succeed");

    assert_eq!(
        desc.status().map(|s| s.as_str()),
        Some("FINISHED"),
        "batch statement should be FINISHED in mock"
    );
}

#[tokio::test]
async fn test_batch_execute_statement_appears_in_list() {
    let client = make_redshiftdata_client().await;

    let resp = client
        .batch_execute_statement()
        .sqls("SELECT 1")
        .database("dev")
        .cluster_identifier("batch-list-cluster")
        .send()
        .await
        .expect("batch_execute_statement should succeed");

    let stmt_id = resp.id().expect("should have Id");

    let list_resp = client
        .list_statements()
        .send()
        .await
        .expect("list_statements should succeed");

    let found = list_resp.statements().iter().any(|s| s.id() == stmt_id);
    assert!(found, "batch-executed statement should appear in list");
}

// ============================================================================
// Tests for DescribeTable
// ============================================================================

#[tokio::test]
async fn test_describe_table_basic() {
    let client = make_redshiftdata_client().await;

    let resp = client
        .describe_table()
        .database("dev")
        .cluster_identifier("desc-table-cluster")
        .table("my_table")
        .send()
        .await
        .expect("describe_table should succeed");

    // The mock returns an empty column list and echoes back the table name
    assert_eq!(resp.table_name(), Some("my_table"));
    assert!(
        resp.column_list().is_empty(),
        "mock returns empty column list"
    );
}

#[tokio::test]
async fn test_describe_table_without_table_name() {
    let client = make_redshiftdata_client().await;

    let resp = client
        .describe_table()
        .database("dev")
        .cluster_identifier("desc-table-cluster-2")
        .send()
        .await
        .expect("describe_table without table name should succeed");

    // When no table is specified, table_name should be None
    assert!(
        resp.table_name().is_none(),
        "table_name should be None when not specified"
    );
}

// ============================================================================
// Tests for ListDatabases
// ============================================================================

#[tokio::test]
async fn test_list_databases() {
    let client = make_redshiftdata_client().await;

    let resp = client
        .list_databases()
        .database("dev")
        .cluster_identifier("list-db-cluster")
        .send()
        .await
        .expect("list_databases should succeed");

    let databases = resp.databases();
    assert_eq!(databases.len(), 2);
    assert!(databases.contains(&"dev".to_string()));
    assert!(databases.contains(&"prod".to_string()));
    assert!(resp.next_token().is_none(), "next_token should be None");
}

// ============================================================================
// Tests for ListSchemas
// ============================================================================

#[tokio::test]
async fn test_list_schemas() {
    let client = make_redshiftdata_client().await;

    let resp = client
        .list_schemas()
        .database("dev")
        .cluster_identifier("list-schema-cluster")
        .send()
        .await
        .expect("list_schemas should succeed");

    let schemas = resp.schemas();
    assert_eq!(schemas.len(), 2);
    assert!(schemas.contains(&"public".to_string()));
    assert!(schemas.contains(&"information_schema".to_string()));
    assert!(resp.next_token().is_none(), "next_token should be None");
}

// ============================================================================
// Tests for ListTables
// ============================================================================

#[tokio::test]
async fn test_list_tables() {
    let client = make_redshiftdata_client().await;

    let resp = client
        .list_tables()
        .database("dev")
        .cluster_identifier("list-tables-cluster")
        .send()
        .await
        .expect("list_tables should succeed");

    // The mock returns an empty table list
    assert!(resp.tables().is_empty(), "mock returns empty table list");
    assert!(resp.next_token().is_none(), "next_token should be None");
}

// ============================================================================
// Tests for GetStatementResultV2
// ============================================================================

#[tokio::test]
async fn test_get_statement_result_v2_basic() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT 1")
        .database("dev")
        .cluster_identifier("v2-cluster")
        .send()
        .await
        .expect("execute_statement should succeed");

    let stmt_id = exec_resp.id().expect("should have Id");

    let result = client
        .get_statement_result_v2()
        .id(stmt_id)
        .send()
        .await
        .expect("get_statement_result_v2 should succeed");

    // The mock always returns the hardcoded three-row result
    assert_eq!(result.total_num_rows(), 3_i64);
    assert_eq!(result.column_metadata().len(), 3);
    assert!(result.next_token().is_none());
}

#[tokio::test]
async fn test_get_statement_result_v2_invalid_uuid() {
    let client = make_redshiftdata_client().await;

    let result = client
        .get_statement_result_v2()
        .id("invalid-not-uuid")
        .send()
        .await;

    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_validation_exception(),
        "should be ValidationException, got: {:?}",
        service_err
    );
}

#[tokio::test]
async fn test_get_statement_result_v2_not_found() {
    let client = make_redshiftdata_client().await;

    let result = client
        .get_statement_result_v2()
        .id("890f1253-595b-4608-a0d1-73f933ccd0a0")
        .send()
        .await;

    let err = result.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_resource_not_found_exception(),
        "should be ResourceNotFoundException, got: {:?}",
        service_err
    );
}
