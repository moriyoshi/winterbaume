use std::sync::{Arc, Mutex};

use duckdb::Connection;
use winterbaume_sqlengine_duckdb::{DuckDbAthenaQueryBackend, DuckDbRedshiftQueryBackend};

// ---- Athena ----

async fn make_athena_client() -> aws_sdk_athena::Client {
    use aws_sdk_athena::config::BehaviorVersion;
    use winterbaume_athena::AthenaService;
    use winterbaume_core::MockAws;

    let conn = Arc::new(Mutex::new(
        Connection::open_in_memory().expect("open in-memory DuckDB"),
    ));
    let mock = MockAws::builder()
        .with_service(AthenaService::with_query_backend(Arc::new(
            DuckDbAthenaQueryBackend::new(conn),
        )))
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_athena::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_athena::Client::new(&config)
}

#[tokio::test]
async fn test_athena_select_literal() {
    let client = make_athena_client().await;

    let start_resp = client
        .start_query_execution()
        .query_string("SELECT 1 AS n")
        .send()
        .await
        .expect("start_query_execution should succeed");

    let id = start_resp
        .query_execution_id()
        .expect("should have query_execution_id");

    let results = client
        .get_query_results()
        .query_execution_id(id)
        .send()
        .await
        .expect("get_query_results should succeed");

    let result_set = results.result_set().expect("should have result_set");
    let rows = result_set.rows();
    // Athena prepends a header row with column names.
    assert!(rows.len() >= 2, "expected header + data row");
    let header = &rows[0];
    assert_eq!(header.data()[0].var_char_value(), Some("n"));
    let data_row = &rows[1];
    assert_eq!(data_row.data()[0].var_char_value(), Some("1"));
}

#[tokio::test]
async fn test_athena_failed_query() {
    let client = make_athena_client().await;

    let start_resp = client
        .start_query_execution()
        .query_string("SELECT * FROM nonexistent_table_xyz")
        .send()
        .await
        .expect("start_query_execution should succeed");

    let id = start_resp
        .query_execution_id()
        .expect("should have query_execution_id");

    let get_resp = client
        .get_query_execution()
        .query_execution_id(id)
        .send()
        .await
        .expect("get_query_execution should succeed");

    let status = get_resp
        .query_execution()
        .and_then(|qe| qe.status())
        .and_then(|s| s.state())
        .map(|s| s.as_str());
    assert_eq!(status, Some("FAILED"));
}

// ---- Redshift Data ----

async fn make_redshiftdata_client() -> aws_sdk_redshiftdata::Client {
    use aws_sdk_redshiftdata::config::BehaviorVersion;
    use winterbaume_core::MockAws;
    use winterbaume_redshiftdata::RedshiftDataService;

    let conn = Arc::new(Mutex::new(
        Connection::open_in_memory().expect("open in-memory DuckDB"),
    ));
    let mock = MockAws::builder()
        .with_service(RedshiftDataService::with_query_backend(Arc::new(
            DuckDbRedshiftQueryBackend::new(conn),
        )))
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
async fn test_redshift_select_literal() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT 42 AS answer")
        .database("dev")
        .cluster_identifier("my-cluster")
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
    assert_eq!(col_metadata.len(), 1);
    assert_eq!(col_metadata[0].name(), Some("answer"));

    let records = result.records();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0][0].as_long_value(), Ok(&42_i64));
}

#[tokio::test]
async fn test_redshift_select_string() {
    let client = make_redshiftdata_client().await;

    let exec_resp = client
        .execute_statement()
        .sql("SELECT 'hello' AS greeting")
        .database("dev")
        .cluster_identifier("my-cluster")
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

    let records = result.records();
    assert_eq!(records.len(), 1);
    assert_eq!(
        records[0][0].as_string_value().map(|s| s.as_str()),
        Ok("hello")
    );
}
