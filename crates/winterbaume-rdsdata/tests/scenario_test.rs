/// Scenario: RDS Data transaction lifecycle
///
/// Covers the canonical usage pattern for the RDS Data API:
/// 1. Begin a transaction.
/// 2. Execute one or more SQL statements within the transaction.
/// 3. Either commit or roll back the transaction.
///
/// This validates that the mock correctly tracks transaction state across all
/// three (or more) operations, and that post-commit/rollback state is
/// consistent.
use aws_sdk_rdsdata::config::BehaviorVersion;
use winterbaume_core::{BackendState, DEFAULT_ACCOUNT_ID, MockAws};
use winterbaume_rdsdata::types::QueryResults;
use winterbaume_rdsdata::{RdsDataService, RdsDataState};

const RESOURCE_ARN: &str = "arn:aws:rds:us-east-1:123456789012:cluster:my-cluster";
const SECRET_ARN: &str = "arn:aws:secretsmanager:us-east-1:123456789012:secret:my-secret";

async fn make_client_with_state() -> (
    aws_sdk_rdsdata::Client,
    std::sync::Arc<BackendState<RdsDataState>>,
) {
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

async fn enqueue(backend: &std::sync::Arc<BackendState<RdsDataState>>, result: QueryResults) {
    let state = backend.get(DEFAULT_ACCOUNT_ID, "us-east-1");
    state.write().await.enqueue_result(result);
}

/// Scenario: begin → execute (SELECT) → commit
///
/// A client opens a transaction, issues a SELECT within that transaction,
/// then commits. After commit the transaction must no longer exist.
#[tokio::test]
async fn test_transaction_select_then_commit() {
    let (client, backend) = make_client_with_state().await;

    // Pre-queue the SELECT result that will be consumed by execute_statement.
    enqueue(
        &backend,
        QueryResults {
            records: vec![vec![serde_json::json!({"longValue": 42})]],
            column_metadata: vec![],
            number_of_records_updated: 0,
        },
    )
    .await;

    // Step 1: begin transaction.
    let begin_resp = client
        .begin_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .send()
        .await
        .expect("begin_transaction should succeed");

    let tx_id = begin_resp
        .transaction_id()
        .expect("transactionId must be present")
        .to_string();

    // Step 2: execute a SELECT inside the transaction.
    let exec_resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(&tx_id)
        .sql("SELECT answer FROM universe LIMIT 1")
        .send()
        .await
        .expect("execute_statement within transaction should succeed");

    assert_eq!(exec_resp.records().len(), 1, "expected one row from SELECT");

    // Step 3: commit the transaction.
    let commit_resp = client
        .commit_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(&tx_id)
        .send()
        .await
        .expect("commit_transaction should succeed");

    assert_eq!(
        commit_resp.transaction_status(),
        Some("Transaction Committed"),
        "commit status must be 'Transaction Committed'"
    );

    // Step 4: verify the transaction no longer exists — a second commit must fail.
    let double_commit = client
        .commit_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(&tx_id)
        .send()
        .await;

    assert!(
        double_commit.is_err(),
        "committing an already-committed transaction must fail"
    );
    let err_str = format!("{:?}", double_commit);
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "expected NotFoundException after double commit, got: {err_str}"
    );
}

/// Scenario: begin → execute (INSERT) → rollback
///
/// A client opens a transaction, issues an INSERT within that transaction,
/// then rolls it back. After rollback the transaction must no longer exist.
#[tokio::test]
async fn test_transaction_insert_then_rollback() {
    let (client, backend) = make_client_with_state().await;

    // Pre-queue the INSERT result (1 row affected).
    enqueue(
        &backend,
        QueryResults {
            records: vec![],
            column_metadata: vec![],
            number_of_records_updated: 1,
        },
    )
    .await;

    // Step 1: begin transaction.
    let begin_resp = client
        .begin_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .database("mydb")
        .send()
        .await
        .expect("begin_transaction should succeed");

    let tx_id = begin_resp
        .transaction_id()
        .expect("transactionId must be present")
        .to_string();

    // Step 2: execute an INSERT inside the transaction.
    let exec_resp = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(&tx_id)
        .sql("INSERT INTO events (name) VALUES (:name)")
        .send()
        .await
        .expect("execute_statement within transaction should succeed");

    assert_eq!(
        exec_resp.number_of_records_updated(),
        1,
        "expected 1 row affected by INSERT"
    );

    // Step 3: roll back the transaction.
    let rollback_resp = client
        .rollback_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(&tx_id)
        .send()
        .await
        .expect("rollback_transaction should succeed");

    assert_eq!(
        rollback_resp.transaction_status(),
        Some("Rollback Complete"),
        "rollback status must be 'Rollback Complete'"
    );

    // Step 4: verify the transaction no longer exists — a commit must now fail.
    let late_commit = client
        .commit_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(&tx_id)
        .send()
        .await;

    assert!(
        late_commit.is_err(),
        "committing a rolled-back transaction must fail"
    );
    let err_str = format!("{:?}", late_commit);
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "expected NotFoundException after rollback, got: {err_str}"
    );
}

/// Scenario: begin → multiple executes → batch execute → commit
///
/// Chains four operations: begin, two sequential execute_statement calls, one
/// batch_execute_statement, and a final commit. Validates that the transaction
/// remains valid across multiple statements.
#[tokio::test]
async fn test_transaction_multiple_statements_then_commit() {
    let (client, backend) = make_client_with_state().await;

    // Queue results for the two execute_statement calls.
    enqueue(
        &backend,
        QueryResults {
            records: vec![],
            column_metadata: vec![],
            number_of_records_updated: 1,
        },
    )
    .await;
    enqueue(
        &backend,
        QueryResults {
            records: vec![],
            column_metadata: vec![],
            number_of_records_updated: 2,
        },
    )
    .await;

    // Step 1: begin.
    let begin_resp = client
        .begin_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .send()
        .await
        .expect("begin_transaction should succeed");

    let tx_id = begin_resp
        .transaction_id()
        .expect("transactionId must be present")
        .to_string();

    // Step 2: first INSERT.
    let exec1 = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(&tx_id)
        .sql("INSERT INTO a VALUES (1)")
        .send()
        .await
        .expect("first execute_statement should succeed");

    assert_eq!(exec1.number_of_records_updated(), 1);

    // Step 3: second INSERT.
    let exec2 = client
        .execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(&tx_id)
        .sql("INSERT INTO b VALUES (2), (3)")
        .send()
        .await
        .expect("second execute_statement should succeed");

    assert_eq!(exec2.number_of_records_updated(), 2);

    // Step 4: batch INSERT with two parameter sets (queue is empty; empty results returned).
    use aws_sdk_rdsdata::types::{Field, SqlParameter};
    let batch_resp = client
        .batch_execute_statement()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(&tx_id)
        .sql("INSERT INTO c (v) VALUES (:v)")
        .parameter_sets(vec![
            SqlParameter::builder()
                .name("v")
                .value(Field::LongValue(10))
                .build(),
        ])
        .parameter_sets(vec![
            SqlParameter::builder()
                .name("v")
                .value(Field::LongValue(20))
                .build(),
        ])
        .send()
        .await
        .expect("batch_execute_statement within transaction should succeed");

    assert_eq!(
        batch_resp.update_results().len(),
        2,
        "expected 2 update results for 2 parameter sets"
    );

    // Step 5: commit.
    let commit_resp = client
        .commit_transaction()
        .resource_arn(RESOURCE_ARN)
        .secret_arn(SECRET_ARN)
        .transaction_id(&tx_id)
        .send()
        .await
        .expect("commit_transaction should succeed");

    assert_eq!(
        commit_resp.transaction_status(),
        Some("Transaction Committed")
    );
}
