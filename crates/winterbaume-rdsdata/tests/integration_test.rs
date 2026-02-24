use std::sync::Arc;

use aws_sdk_rdsdata::config::BehaviorVersion;
use winterbaume_core::{BackendState, DEFAULT_ACCOUNT_ID, MockAws};
use winterbaume_rdsdata::types::{ColumnMetadata, QueryResults};
use winterbaume_rdsdata::{RdsDataService, RdsDataState};

const RESOURCE_ARN: &str = "arn:aws:rds:us-east-1:123456789012:cluster:my-cluster";
const SECRET_ARN: &str = "arn:aws:secretsmanager:us-east-1:123456789012:secret:my-secret";

async fn make_client() -> aws_sdk_rdsdata::Client {
    let mock = MockAws::builder()
        .with_service(RdsDataService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rdsdata::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_rdsdata::Client::new(&config)
}

/// Returns a client and a reference to the underlying backend state so tests
/// can pre-populate the result queue before making SDK calls.
async fn make_client_with_state() -> (aws_sdk_rdsdata::Client, Arc<BackendState<RdsDataState>>) {
    let service = RdsDataService::new();
    let backend = service.backend_state();

    let mock = MockAws::builder().with_service(service).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rdsdata::config::Region::new("us-east-1"))
        .load()
        .await;

    (aws_sdk_rdsdata::Client::new(&config), backend)
}

/// Enqueue a result into the backend state for account/region used by tests.
async fn enqueue(backend: &Arc<BackendState<RdsDataState>>, result: QueryResults) {
    let state = backend.get(DEFAULT_ACCOUNT_ID, "us-east-1");
    state.write().await.enqueue_result(result);
}

// ============================================================================
// Tests derived from AWS documentation: RDS Data API
// ============================================================================

#[tokio::test]
async fn test_execute_statement_empty_result() {
    let client = make_client().await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("SELECT 1")
        .send()
        .await
        .expect("execute_statement should succeed with empty queue");

    assert_eq!(resp.number_of_records_updated(), 0);
    assert!(resp.records().is_empty());
    assert!(resp.column_metadata().is_empty());
}

#[tokio::test]
async fn test_execute_statement_missing_resource_arn() {
    let client = make_client().await;

    let result = client
        .execute_statement()
        .secret_arn(SECRET_ARN)
        .sql("SELECT 1")
        .send()
        .await;

    // The SDK may reject a missing required field client-side; either way we
    // should get an error, not a successful response.
    assert!(
        result.is_err(),
        "Expected error when resourceArn is missing"
    );
}

#[tokio::test]
async fn test_execute_statement_missing_secret_arn() {
    let client = make_client().await;

    let result = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .sql("SELECT 1")
        .send()
        .await;

    // The SDK may reject a missing required field client-side; either way we
    // should get an error, not a successful response.
    assert!(result.is_err(), "Expected error when secretArn is missing");
}

#[tokio::test]
async fn test_execute_statement_missing_sql() {
    let client = make_client().await;

    let result = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .send()
        .await;

    // The SDK may reject a missing required field client-side; either way we
    // should get an error.
    assert!(result.is_err(), "Expected error when sql is missing");
}

#[tokio::test]
async fn test_execute_statement_with_queued_result() {
    let (client, backend) = make_client_with_state().await;

    enqueue(
        &backend,
        QueryResults {
            records: vec![vec![
                serde_json::json!({"stringValue": "hello"}),
                serde_json::json!({"longValue": 42}),
            ]],
            column_metadata: vec![],
            number_of_records_updated: 0,
        },
    )
    .await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("SELECT name, id FROM users LIMIT 1")
        .send()
        .await
        .expect("execute_statement should succeed");

    assert_eq!(resp.records().len(), 1, "Expected one row");
    assert_eq!(resp.records()[0].len(), 2, "Expected two columns");
}

#[tokio::test]
async fn test_execute_statement_column_metadata() {
    let (client, backend) = make_client_with_state().await;

    enqueue(
        &backend,
        QueryResults {
            records: vec![],
            column_metadata: vec![
                ColumnMetadata {
                    name: "id".to_string(),
                    type_name: "INT".to_string(),
                    label: "id".to_string(),
                },
                ColumnMetadata {
                    name: "name".to_string(),
                    type_name: "VARCHAR".to_string(),
                    label: "name".to_string(),
                },
            ],
            number_of_records_updated: 0,
        },
    )
    .await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("SELECT id, name FROM users WHERE 1=0")
        .send()
        .await
        .expect("execute_statement should succeed");

    let meta = resp.column_metadata();
    assert_eq!(meta.len(), 2, "Expected two column metadata entries");

    // First column: id
    assert_eq!(meta[0].name(), Some("id"));
    assert_eq!(meta[0].type_name(), Some("INT"));
    assert_eq!(meta[0].label(), Some("id"));

    // Second column: name
    assert_eq!(meta[1].name(), Some("name"));
    assert_eq!(meta[1].type_name(), Some("VARCHAR"));
    assert_eq!(meta[1].label(), Some("name"));
}

#[tokio::test]
async fn test_execute_statement_dml_update_count() {
    let (client, backend) = make_client_with_state().await;

    enqueue(
        &backend,
        QueryResults {
            records: vec![],
            column_metadata: vec![],
            number_of_records_updated: 3,
        },
    )
    .await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("UPDATE users SET active = true WHERE region = 'us-east-1'")
        .send()
        .await
        .expect("execute_statement should succeed");

    assert_eq!(
        resp.number_of_records_updated(),
        3,
        "Expected numberOfRecordsUpdated to be 3"
    );
    assert!(resp.records().is_empty(), "Expected no records for DML");
}

#[tokio::test]
async fn test_execute_statement_multiple_queued_results() {
    let (client, backend) = make_client_with_state().await;

    // Enqueue two distinct results
    enqueue(
        &backend,
        QueryResults {
            records: vec![vec![serde_json::json!({"longValue": 1})]],
            column_metadata: vec![],
            number_of_records_updated: 0,
        },
    )
    .await;
    enqueue(
        &backend,
        QueryResults {
            records: vec![vec![serde_json::json!({"longValue": 2})]],
            column_metadata: vec![],
            number_of_records_updated: 0,
        },
    )
    .await;

    // First call should consume first result
    let resp1 = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("SELECT 1")
        .send()
        .await
        .expect("first execute_statement should succeed");
    assert_eq!(resp1.records().len(), 1);

    // Second call should consume second result
    let resp2 = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("SELECT 2")
        .send()
        .await
        .expect("second execute_statement should succeed");
    assert_eq!(resp2.records().len(), 1);

    // Third call: queue is now empty -> empty result
    let resp3 = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("SELECT 3")
        .send()
        .await
        .expect("third execute_statement should succeed with empty queue");
    assert_eq!(resp3.number_of_records_updated(), 0);
    assert!(resp3.records().is_empty());
}

#[tokio::test]
async fn test_execute_statement_multi_row_result() {
    let (client, backend) = make_client_with_state().await;

    // Simulate a 3-row SELECT result
    enqueue(
        &backend,
        QueryResults {
            records: vec![
                vec![
                    serde_json::json!({"longValue": 1}),
                    serde_json::json!({"stringValue": "Alice"}),
                ],
                vec![
                    serde_json::json!({"longValue": 2}),
                    serde_json::json!({"stringValue": "Bob"}),
                ],
                vec![
                    serde_json::json!({"longValue": 3}),
                    serde_json::json!({"stringValue": "Carol"}),
                ],
            ],
            column_metadata: vec![
                ColumnMetadata {
                    name: "id".to_string(),
                    type_name: "INT".to_string(),
                    label: "id".to_string(),
                },
                ColumnMetadata {
                    name: "name".to_string(),
                    type_name: "VARCHAR".to_string(),
                    label: "name".to_string(),
                },
            ],
            number_of_records_updated: 0,
        },
    )
    .await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("SELECT id, name FROM users")
        .send()
        .await
        .expect("execute_statement should succeed");

    assert_eq!(resp.records().len(), 3, "Expected 3 rows");
    assert_eq!(resp.column_metadata().len(), 2, "Expected 2 columns");
    assert_eq!(resp.number_of_records_updated(), 0);
}

#[tokio::test]
async fn test_execute_statement_empty_string_resource_arn() {
    // Passing empty string explicitly for resourceArn triggers BadRequestException
    // from the handler (resourceArn.is_empty() check in state.rs).
    // Note: the SDK sends the empty string in the JSON body.
    let client = make_client().await;

    let result = client
        .execute_statement()
        .resource_arn("")
        .secret_arn(SECRET_ARN)
        .sql("SELECT 1")
        .send()
        .await;

    let err_str = format!("{:?}", result);
    assert!(
        err_str.contains("BadRequestException") || err_str.contains("bad") || result.is_err(),
        "Expected BadRequestException for empty resourceArn, got: {err_str}"
    );
}

#[tokio::test]
async fn test_execute_statement_with_database_param() {
    // The `database` optional parameter is accepted and passed through.
    // The mock ignores it (no schema-level routing), but the call should succeed.
    let (client, backend) = make_client_with_state().await;

    enqueue(
        &backend,
        QueryResults {
            records: vec![vec![serde_json::json!({"stringValue": "ok"})]],
            column_metadata: vec![],
            number_of_records_updated: 0,
        },
    )
    .await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .database("mydb")
        .sql("SELECT 'ok'")
        .send()
        .await
        .expect("execute_statement with database param should succeed");

    assert_eq!(resp.records().len(), 1);
}

// ============================================================================
// Tests derived from AWS documentation: RDS Data API (extended coverage)
// ============================================================================

#[tokio::test]
async fn test_execute_statement_empty_string_secret_arn() {
    // Passing an empty string for secretArn should trigger the server-side
    // BadRequestException validation in state.rs (secretArn.is_empty() check).
    let client = make_client().await;

    let result = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn("")
        .sql("SELECT 1")
        .send()
        .await;

    assert!(
        result.is_err(),
        "Expected error when secretArn is empty string"
    );
    let err_str = format!("{:?}", result);
    assert!(
        err_str.contains("BadRequestException") || err_str.contains("bad"),
        "Expected BadRequestException for empty secretArn, got: {err_str}"
    );
}

#[tokio::test]
async fn test_execute_statement_empty_string_sql() {
    // Passing an empty string for sql should trigger the server-side
    // BadRequestException validation in state.rs (sql.is_empty() check).
    let client = make_client().await;

    let result = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("")
        .send()
        .await;

    assert!(result.is_err(), "Expected error when sql is empty string");
    let err_str = format!("{:?}", result);
    assert!(
        err_str.contains("BadRequestException") || err_str.contains("bad"),
        "Expected BadRequestException for empty sql, got: {err_str}"
    );
}

#[tokio::test]
async fn test_execute_statement_boolean_value_in_record() {
    // Records may contain booleanValue fields; they should round-trip correctly.
    let (client, backend) = make_client_with_state().await;

    enqueue(
        &backend,
        QueryResults {
            records: vec![vec![
                serde_json::json!({"booleanValue": true}),
                serde_json::json!({"booleanValue": false}),
            ]],
            column_metadata: vec![],
            number_of_records_updated: 0,
        },
    )
    .await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("SELECT active, deleted FROM flags LIMIT 1")
        .send()
        .await
        .expect("execute_statement should succeed");

    assert_eq!(resp.records().len(), 1);
    let row = &resp.records()[0];
    assert_eq!(row.len(), 2);
    assert!(
        row[0].is_boolean_value(),
        "First field should be booleanValue"
    );
    assert_eq!(row[0].as_boolean_value().unwrap(), &true);
    assert!(
        row[1].is_boolean_value(),
        "Second field should be booleanValue"
    );
    assert_eq!(row[1].as_boolean_value().unwrap(), &false);
}

#[tokio::test]
async fn test_execute_statement_null_value_in_record() {
    // Records may contain isNull fields representing SQL NULL values.
    let (client, backend) = make_client_with_state().await;

    enqueue(
        &backend,
        QueryResults {
            records: vec![vec![serde_json::json!({"isNull": true})]],
            column_metadata: vec![],
            number_of_records_updated: 0,
        },
    )
    .await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("SELECT nullable_col FROM t LIMIT 1")
        .send()
        .await
        .expect("execute_statement should succeed");

    assert_eq!(resp.records().len(), 1);
    let row = &resp.records()[0];
    assert_eq!(row.len(), 1);
    assert!(row[0].is_is_null(), "Field should be isNull");
    assert_eq!(row[0].as_is_null().unwrap(), &true);
}

#[tokio::test]
#[allow(clippy::approx_constant)]
async fn test_execute_statement_double_value_in_record() {
    // Records may contain doubleValue fields for floating-point columns.
    let (client, backend) = make_client_with_state().await;

    enqueue(
        &backend,
        QueryResults {
            records: vec![vec![serde_json::json!({"doubleValue": 3.14})]],
            column_metadata: vec![],
            number_of_records_updated: 0,
        },
    )
    .await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("SELECT pi FROM constants LIMIT 1")
        .send()
        .await
        .expect("execute_statement should succeed");

    assert_eq!(resp.records().len(), 1);
    let row = &resp.records()[0];
    assert_eq!(row.len(), 1);
    assert!(row[0].is_double_value(), "Field should be doubleValue");
    let v = row[0].as_double_value().unwrap();
    assert!((*v - 3.14_f64).abs() < 1e-9, "Expected 3.14, got {v}");
}

#[tokio::test]
async fn test_execute_statement_mixed_field_types_in_row() {
    // A single row may contain fields of different types: string, long, boolean, isNull.
    let (client, backend) = make_client_with_state().await;

    enqueue(
        &backend,
        QueryResults {
            records: vec![vec![
                serde_json::json!({"stringValue": "hello"}),
                serde_json::json!({"longValue": 42}),
                serde_json::json!({"booleanValue": true}),
                serde_json::json!({"isNull": true}),
            ]],
            column_metadata: vec![
                ColumnMetadata {
                    name: "name".to_string(),
                    type_name: "VARCHAR".to_string(),
                    label: "name".to_string(),
                },
                ColumnMetadata {
                    name: "age".to_string(),
                    type_name: "INT".to_string(),
                    label: "age".to_string(),
                },
                ColumnMetadata {
                    name: "active".to_string(),
                    type_name: "BOOLEAN".to_string(),
                    label: "active".to_string(),
                },
                ColumnMetadata {
                    name: "deleted_at".to_string(),
                    type_name: "TIMESTAMP".to_string(),
                    label: "deleted_at".to_string(),
                },
            ],
            number_of_records_updated: 0,
        },
    )
    .await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("SELECT name, age, active, deleted_at FROM users LIMIT 1")
        .send()
        .await
        .expect("execute_statement should succeed");

    assert_eq!(resp.records().len(), 1);
    let row = &resp.records()[0];
    assert_eq!(row.len(), 4, "Expected 4 fields in the row");

    // Field 0: stringValue
    assert!(row[0].is_string_value());
    assert_eq!(row[0].as_string_value().unwrap(), "hello");

    // Field 1: longValue
    assert!(row[1].is_long_value());
    assert_eq!(row[1].as_long_value().unwrap(), &42_i64);

    // Field 2: booleanValue
    assert!(row[2].is_boolean_value());
    assert_eq!(row[2].as_boolean_value().unwrap(), &true);

    // Field 3: isNull
    assert!(row[3].is_is_null());

    // Verify column metadata lines up
    let meta = resp.column_metadata();
    assert_eq!(meta.len(), 4);
    assert_eq!(meta[0].name(), Some("name"));
    assert_eq!(meta[1].name(), Some("age"));
    assert_eq!(meta[2].name(), Some("active"));
    assert_eq!(meta[3].name(), Some("deleted_at"));
}

#[tokio::test]
async fn test_execute_statement_with_schema_param() {
    // The `schema` optional parameter is accepted by the handler (silently ignored
    // per AWS docs: "Currently, the schema parameter isn't supported").
    let (client, backend) = make_client_with_state().await;

    enqueue(
        &backend,
        QueryResults {
            records: vec![vec![serde_json::json!({"stringValue": "ok"})]],
            column_metadata: vec![],
            number_of_records_updated: 0,
        },
    )
    .await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .database("mydb")
        .schema("public")
        .sql("SELECT 'ok'")
        .send()
        .await
        .expect("execute_statement with schema param should succeed");

    assert_eq!(resp.records().len(), 1);
}

#[tokio::test]
async fn test_execute_statement_with_transaction_id_param() {
    // The `transactionId` optional parameter is accepted by the handler.
    let (client, backend) = make_client_with_state().await;

    enqueue(
        &backend,
        QueryResults {
            records: vec![],
            column_metadata: vec![],
            number_of_records_updated: 1,
        },
    )
    .await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id("fake-transaction-id-12345")
        .sql("INSERT INTO t VALUES (1)")
        .send()
        .await
        .expect("execute_statement with transactionId should succeed");

    assert_eq!(resp.number_of_records_updated(), 1);
}

#[tokio::test]
async fn test_execute_statement_with_continue_after_timeout() {
    // The `continueAfterTimeout` boolean parameter is accepted by the handler
    // (recommended for DDL per AWS docs).
    let (client, backend) = make_client_with_state().await;

    enqueue(
        &backend,
        QueryResults {
            records: vec![],
            column_metadata: vec![],
            number_of_records_updated: 0,
        },
    )
    .await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .continue_after_timeout(true)
        .sql("CREATE TABLE IF NOT EXISTS t (id INT PRIMARY KEY)")
        .send()
        .await
        .expect("execute_statement with continueAfterTimeout should succeed");

    assert_eq!(resp.number_of_records_updated(), 0);
}

#[tokio::test]
async fn test_execute_statement_zero_records_updated_default() {
    // When the queue is empty the handler returns a default QueryResults with
    // numberOfRecordsUpdated == 0 and an empty records list.
    let client = make_client().await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("SELECT 42")
        .send()
        .await
        .expect("execute_statement should succeed with empty queue");

    assert_eq!(resp.number_of_records_updated(), 0);
    assert!(resp.records().is_empty());
    assert!(resp.column_metadata().is_empty());
}

#[tokio::test]
async fn test_execute_statement_column_metadata_all_fields() {
    // ColumnMetadata returned in the response should include name, typeName,
    // and label. This test verifies all three fields are correctly populated.
    let (client, backend) = make_client_with_state().await;

    enqueue(
        &backend,
        QueryResults {
            records: vec![],
            column_metadata: vec![ColumnMetadata {
                name: "user_id".to_string(),
                type_name: "BIGINT".to_string(),
                label: "user_id".to_string(),
            }],
            number_of_records_updated: 0,
        },
    )
    .await;

    let resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("SELECT user_id FROM users WHERE 1=0")
        .send()
        .await
        .expect("execute_statement should succeed");

    let meta = resp.column_metadata();
    assert_eq!(meta.len(), 1);
    assert_eq!(meta[0].name(), Some("user_id"), "name should be 'user_id'");
    assert_eq!(
        meta[0].type_name(),
        Some("BIGINT"),
        "typeName should be 'BIGINT'"
    );
    assert_eq!(
        meta[0].label(),
        Some("user_id"),
        "label should be 'user_id'"
    );
}

// ============================================================================
// BeginTransaction tests
// ============================================================================

#[tokio::test]
async fn test_begin_transaction_success() {
    let client = make_client().await;

    let resp = client
        .begin_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .send()
        .await
        .expect("begin_transaction should succeed");

    let tx_id = resp.transaction_id();
    assert!(tx_id.is_some(), "Expected a transactionId in the response");
    assert!(
        !tx_id.unwrap().is_empty(),
        "transactionId should not be empty"
    );
}

#[tokio::test]
async fn test_begin_transaction_missing_resource_arn() {
    let client = make_client().await;

    let result = client
        .begin_transaction()
        .secret_arn(SECRET_ARN)
        .send()
        .await;

    assert!(
        result.is_err(),
        "Expected error when resourceArn is missing"
    );
}

#[tokio::test]
async fn test_begin_transaction_missing_secret_arn() {
    let client = make_client().await;

    let result = client
        .begin_transaction()
        .resource_arn(RESOURCE_ARN)
        .send()
        .await;

    assert!(result.is_err(), "Expected error when secretArn is missing");
}

#[tokio::test]
async fn test_begin_transaction_empty_resource_arn() {
    let client = make_client().await;

    let result = client
        .begin_transaction()
        .resource_arn("")
        .secret_arn(SECRET_ARN)
        .send()
        .await;

    assert!(
        result.is_err(),
        "Expected error when resourceArn is empty string"
    );
}

#[tokio::test]
async fn test_begin_transaction_empty_secret_arn() {
    let client = make_client().await;

    let result = client
        .begin_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn("")
        .send()
        .await;

    assert!(
        result.is_err(),
        "Expected error when secretArn is empty string"
    );
}

#[tokio::test]
async fn test_begin_transaction_with_database_param() {
    // The optional `database` parameter should be accepted without error.
    let client = make_client().await;

    let resp = client
        .begin_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .database("mydb")
        .send()
        .await
        .expect("begin_transaction with database param should succeed");

    assert!(resp.transaction_id().is_some());
}

// ============================================================================
// CommitTransaction tests
// ============================================================================

#[tokio::test]
async fn test_commit_transaction_success() {
    // Begin a transaction, then commit it.
    let client = make_client().await;

    let begin_resp = client
        .begin_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .send()
        .await
        .expect("begin_transaction should succeed");

    let tx_id = begin_resp.transaction_id().unwrap();

    let commit_resp = client
        .commit_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(tx_id)
        .send()
        .await
        .expect("commit_transaction should succeed");

    assert_eq!(
        commit_resp.transaction_status(),
        Some("Transaction Committed")
    );
}

#[tokio::test]
async fn test_commit_transaction_not_found() {
    // Committing a non-existent transaction should return an error.
    let client = make_client().await;

    let result = client
        .commit_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id("nonexistent-txn-id")
        .send()
        .await;

    assert!(result.is_err(), "Expected error for unknown transaction");
    let err_str = format!("{:?}", result);
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_commit_transaction_missing_transaction_id() {
    let client = make_client().await;

    let result = client
        .commit_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .send()
        .await;

    assert!(
        result.is_err(),
        "Expected error when transactionId is missing"
    );
}

#[tokio::test]
async fn test_commit_transaction_double_commit() {
    // Committing the same transaction twice should fail the second time.
    let client = make_client().await;

    let begin_resp = client
        .begin_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .send()
        .await
        .expect("begin_transaction should succeed");

    let tx_id = begin_resp.transaction_id().unwrap().to_string();

    // First commit succeeds
    client
        .commit_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(&tx_id)
        .send()
        .await
        .expect("first commit should succeed");

    // Second commit should fail — transaction already consumed
    let result = client
        .commit_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(&tx_id)
        .send()
        .await;

    assert!(
        result.is_err(),
        "Expected error on second commit of same transaction"
    );
}

// ============================================================================
// RollbackTransaction tests
// ============================================================================

#[tokio::test]
async fn test_rollback_transaction_success() {
    // Begin a transaction, then roll it back.
    let client = make_client().await;

    let begin_resp = client
        .begin_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .send()
        .await
        .expect("begin_transaction should succeed");

    let tx_id = begin_resp.transaction_id().unwrap();

    let rollback_resp = client
        .rollback_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(tx_id)
        .send()
        .await
        .expect("rollback_transaction should succeed");

    assert_eq!(
        rollback_resp.transaction_status(),
        Some("Rollback Complete")
    );
}

#[tokio::test]
async fn test_rollback_transaction_not_found() {
    // Rolling back a non-existent transaction should return an error.
    let client = make_client().await;

    let result = client
        .rollback_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id("nonexistent-txn-id")
        .send()
        .await;

    assert!(result.is_err(), "Expected error for unknown transaction");
    let err_str = format!("{:?}", result);
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_rollback_transaction_missing_transaction_id() {
    let client = make_client().await;

    let result = client
        .rollback_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .send()
        .await;

    assert!(
        result.is_err(),
        "Expected error when transactionId is missing"
    );
}

#[tokio::test]
async fn test_rollback_after_begin_removes_transaction() {
    // After rollback, the transaction should no longer exist.
    let client = make_client().await;

    let begin_resp = client
        .begin_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .send()
        .await
        .expect("begin_transaction should succeed");

    let tx_id = begin_resp.transaction_id().unwrap().to_string();

    // Rollback succeeds
    client
        .rollback_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(&tx_id)
        .send()
        .await
        .expect("rollback should succeed");

    // Second rollback should fail — transaction already consumed
    let result = client
        .rollback_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(&tx_id)
        .send()
        .await;

    assert!(
        result.is_err(),
        "Expected error on second rollback of same transaction"
    );
}

// ============================================================================
// BatchExecuteStatement tests
// ============================================================================

#[tokio::test]
async fn test_batch_execute_statement_success() {
    use aws_sdk_rdsdata::types::SqlParameter;

    let client = make_client().await;

    let resp = client
        .batch_execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("INSERT INTO users (name) VALUES (:name)")
        .parameter_sets(vec![
            SqlParameter::builder()
                .name("name")
                .value(aws_sdk_rdsdata::types::Field::StringValue(
                    "Alice".to_string(),
                ))
                .build(),
        ])
        .parameter_sets(vec![
            SqlParameter::builder()
                .name("name")
                .value(aws_sdk_rdsdata::types::Field::StringValue(
                    "Bob".to_string(),
                ))
                .build(),
        ])
        .send()
        .await
        .expect("batch_execute_statement should succeed");

    // Two parameter sets -> two update results
    assert_eq!(
        resp.update_results().len(),
        2,
        "Expected 2 update results for 2 parameter sets"
    );
}

#[tokio::test]
async fn test_batch_execute_statement_empty_parameter_sets() {
    let client = make_client().await;

    let resp = client
        .batch_execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("INSERT INTO t VALUES (1)")
        .send()
        .await
        .expect("batch_execute_statement with no parameter_sets should succeed");

    // No parameter sets -> empty update results
    assert!(
        resp.update_results().is_empty(),
        "Expected empty update results when no parameter sets"
    );
}

#[tokio::test]
async fn test_batch_execute_statement_missing_resource_arn() {
    let client = make_client().await;

    let result = client
        .batch_execute_statement()
        .secret_arn(SECRET_ARN)
        .sql("INSERT INTO t VALUES (1)")
        .send()
        .await;

    assert!(
        result.is_err(),
        "Expected error when resourceArn is missing"
    );
}

#[tokio::test]
async fn test_batch_execute_statement_missing_secret_arn() {
    let client = make_client().await;

    let result = client
        .batch_execute_statement()
        .resource_arn(RESOURCE_ARN)
        .sql("INSERT INTO t VALUES (1)")
        .send()
        .await;

    assert!(result.is_err(), "Expected error when secretArn is missing");
}

#[tokio::test]
async fn test_batch_execute_statement_missing_sql() {
    let client = make_client().await;

    let result = client
        .batch_execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .send()
        .await;

    assert!(result.is_err(), "Expected error when sql is missing");
}

#[tokio::test]
async fn test_batch_execute_statement_with_transaction_id() {
    // Begin a transaction, then use it in a batch execute.
    let client = make_client().await;

    let begin_resp = client
        .begin_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .send()
        .await
        .expect("begin_transaction should succeed");

    let tx_id = begin_resp.transaction_id().unwrap();

    let resp = client
        .batch_execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("INSERT INTO t VALUES (1)")
        .transaction_id(tx_id)
        .send()
        .await
        .expect("batch_execute_statement with transactionId should succeed");

    assert!(resp.update_results().is_empty());
}

#[tokio::test]
async fn test_batch_execute_statement_invalid_transaction_id() {
    // Using a non-existent transactionId should return NotFoundException.
    let client = make_client().await;

    let result = client
        .batch_execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .sql("INSERT INTO t VALUES (1)")
        .transaction_id("nonexistent-txn-id")
        .send()
        .await;

    assert!(
        result.is_err(),
        "Expected error for unknown transactionId in batch execute"
    );
    let err_str = format!("{:?}", result);
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException, got: {err_str}"
    );
}

// ============================================================================
// ExecuteSql tests (deprecated API)
// ============================================================================

#[tokio::test]
#[allow(deprecated)]
async fn test_execute_sql_success() {
    let client = make_client().await;

    let resp = client
        .execute_sql()
        .db_cluster_or_instance_arn(RESOURCE_ARN)
        .aws_secret_store_arn(SECRET_ARN)
        .sql_statements("SELECT 1")
        .send()
        .await
        .expect("execute_sql should succeed");

    // Deprecated endpoint returns empty results
    assert!(
        resp.sql_statement_results().is_empty(),
        "Expected empty sqlStatementResults"
    );
}

#[tokio::test]
#[allow(deprecated)]
async fn test_execute_sql_missing_db_cluster_arn() {
    let client = make_client().await;

    let result = client
        .execute_sql()
        .aws_secret_store_arn(SECRET_ARN)
        .sql_statements("SELECT 1")
        .send()
        .await;

    assert!(
        result.is_err(),
        "Expected error when dbClusterOrInstanceArn is missing"
    );
}

#[tokio::test]
#[allow(deprecated)]
async fn test_execute_sql_missing_secret_store_arn() {
    let client = make_client().await;

    let result = client
        .execute_sql()
        .db_cluster_or_instance_arn(RESOURCE_ARN)
        .sql_statements("SELECT 1")
        .send()
        .await;

    assert!(
        result.is_err(),
        "Expected error when awsSecretStoreArn is missing"
    );
}

#[tokio::test]
#[allow(deprecated)]
async fn test_execute_sql_missing_sql_statements() {
    let client = make_client().await;

    let result = client
        .execute_sql()
        .db_cluster_or_instance_arn(RESOURCE_ARN)
        .aws_secret_store_arn(SECRET_ARN)
        .send()
        .await;

    assert!(
        result.is_err(),
        "Expected error when sqlStatements is missing"
    );
}
