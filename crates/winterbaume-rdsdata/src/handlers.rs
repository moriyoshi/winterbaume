use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, rest_json_error,
};

use crate::state::{RdsDataError, RdsDataState};
use crate::views::RdsDataStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct RdsDataService {
    pub(crate) state: Arc<BackendState<RdsDataState>>,
    pub(crate) notifier: StateChangeNotifier<RdsDataStateView>,
}

impl RdsDataService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Returns the underlying backend state, allowing tests to pre-populate
    /// result queues before making SDK calls.
    pub fn backend_state(&self) -> Arc<BackendState<RdsDataState>> {
        Arc::clone(&self.state)
    }
}

impl Default for RdsDataService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for RdsDataService {
    fn service_name(&self) -> &str {
        "rds-data"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://rds-data\..*\.amazonaws\.com",
            r"https?://rds-data\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl RdsDataService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let query = winterbaume_core::extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(query);
        let labels: &[(&str, &str)] = &[];

        match (request.method.as_str(), path.as_str()) {
            ("POST", "/Execute") => {
                self.handle_execute_statement(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", "/BatchExecute") => {
                self.handle_batch_execute_statement(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", "/BeginTransaction") => {
                self.handle_begin_transaction(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", "/CommitTransaction") => {
                self.handle_commit_transaction(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", "/ExecuteSql") => self.handle_execute_sql(&request, labels, &query_map).await,
            ("POST", "/RollbackTransaction") => {
                self.handle_rollback_transaction(&state, &request, labels, &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_execute_statement(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsDataState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_execute_statement_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.execute_statement(&input.resource_arn, &input.secret_arn, &input.sql) {
            Ok(result) => {
                wire::serialize_execute_statement_response(&query_results_to_model(result))
            }
            Err(e) => rdsdata_error_response(&e),
        }
    }

    async fn handle_batch_execute_statement(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsDataState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_execute_statement_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.resource_arn.is_empty() {
            return rdsdata_error_response(&RdsDataError::ResourceArnRequired);
        }
        if input.secret_arn.is_empty() {
            return rdsdata_error_response(&RdsDataError::SecretArnRequired);
        }
        if input.sql.is_empty() {
            return rdsdata_error_response(&RdsDataError::SqlRequired);
        }

        // Validate transactionId if provided
        if let Some(transaction_id) = input.transaction_id.as_deref() {
            if !transaction_id.is_empty() {
                let state = state.read().await;
                if !state.transactions.contains_key(transaction_id) {
                    return rdsdata_error_response(&RdsDataError::TransactionNotFound {
                        transaction_id: transaction_id.to_string(),
                    });
                }
            }
        }

        // Return empty updateResults — one UpdateResult per param set
        let parameter_sets = input.parameter_sets.as_ref().map(|p| p.len()).unwrap_or(0);

        let update_results: Vec<wire::UpdateResult> = (0..parameter_sets)
            .map(|_| wire::UpdateResult {
                generated_fields: Some(vec![]),
            })
            .collect();

        wire::serialize_batch_execute_statement_response(&wire::BatchExecuteStatementResponse {
            update_results: Some(update_results),
        })
    }

    async fn handle_begin_transaction(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsDataState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_begin_transaction_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.begin_transaction(&input.resource_arn, &input.secret_arn) {
            Ok(transaction_id) => {
                wire::serialize_begin_transaction_response(&wire::BeginTransactionResponse {
                    transaction_id: Some(transaction_id),
                })
            }
            Err(e) => rdsdata_error_response(&e),
        }
    }

    async fn handle_commit_transaction(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsDataState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_commit_transaction_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.commit_transaction(
            &input.resource_arn,
            &input.secret_arn,
            &input.transaction_id,
        ) {
            Ok(()) => {
                wire::serialize_commit_transaction_response(&wire::CommitTransactionResponse {
                    transaction_status: Some("Transaction Committed".to_string()),
                })
            }
            Err(e) => rdsdata_error_response(&e),
        }
    }

    async fn handle_rollback_transaction(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsDataState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_rollback_transaction_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.rollback_transaction(
            &input.resource_arn,
            &input.secret_arn,
            &input.transaction_id,
        ) {
            Ok(()) => {
                wire::serialize_rollback_transaction_response(&wire::RollbackTransactionResponse {
                    transaction_status: Some("Rollback Complete".to_string()),
                })
            }
            Err(e) => rdsdata_error_response(&e),
        }
    }

    async fn handle_execute_sql(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_execute_sql_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.db_cluster_or_instance_arn.is_empty() {
            return rdsdata_error_response(&RdsDataError::DbClusterOrInstanceArnRequired);
        }
        if input.aws_secret_store_arn.is_empty() {
            return rdsdata_error_response(&RdsDataError::AwsSecretStoreArnRequired);
        }
        if input.sql_statements.is_empty() {
            return rdsdata_error_response(&RdsDataError::SqlStatementsRequired);
        }

        // Deprecated operation — return empty results
        wire::serialize_execute_sql_response(&wire::ExecuteSqlResponse {
            sql_statement_results: Some(vec![]),
        })
    }
}

fn query_results_to_model(result: crate::types::QueryResults) -> wire::ExecuteStatementResponse {
    let column_metadata = result
        .column_metadata
        .iter()
        .map(column_metadata_to_model)
        .collect();
    let records = result
        .records
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|field| serde_json::from_value(field).unwrap_or_default())
                .collect()
        })
        .collect();

    wire::ExecuteStatementResponse {
        records: Some(records),
        column_metadata: Some(column_metadata),
        number_of_records_updated: Some(result.number_of_records_updated),
        ..Default::default()
    }
}

fn column_metadata_to_model(metadata: &crate::types::ColumnMetadata) -> wire::ColumnMetadata {
    wire::ColumnMetadata {
        name: Some(metadata.name.clone()),
        type_name: Some(metadata.type_name.clone()),
        label: Some(metadata.label.clone()),
        ..Default::default()
    }
}

fn rdsdata_error_response(err: &RdsDataError) -> MockResponse {
    let (status, error_type) = match err {
        RdsDataError::ResourceArnRequired => (400, "BadRequestException"),
        RdsDataError::SecretArnRequired => (400, "BadRequestException"),
        RdsDataError::SqlRequired => (400, "BadRequestException"),
        RdsDataError::TransactionIdRequired => (400, "BadRequestException"),
        RdsDataError::TransactionNotFound { .. } => (404, "NotFoundException"),
        RdsDataError::DbClusterOrInstanceArnRequired => (400, "BadRequestException"),
        RdsDataError::AwsSecretStoreArnRequired => (400, "BadRequestException"),
        RdsDataError::SqlStatementsRequired => (400, "BadRequestException"),
    };
    let body = json!({
        "message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
