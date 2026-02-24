use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::backend::{InMemoryRedshiftQueryBackend, RedshiftQueryBackend};
use crate::model::{
    CancelStatementResponse, ColumnMetadata, DescribeStatementResponse, ExecuteStatementOutput,
    Field, GetStatementResultResponse, ListStatementsResponse, SqlParameter, StatementData,
};
use crate::state::{RedshiftDataError, RedshiftDataState};
use crate::types::{Statement, StatementParameter};
use crate::views::RedshiftDataStateView;
use crate::wire;

pub struct RedshiftDataService {
    pub(crate) query_backend: Arc<dyn RedshiftQueryBackend>,
    pub(crate) state: Arc<BackendState<RedshiftDataState>>,
    pub(crate) notifier: StateChangeNotifier<RedshiftDataStateView>,
}

impl RedshiftDataService {
    pub fn new() -> Self {
        Self {
            query_backend: Arc::new(InMemoryRedshiftQueryBackend),
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Construct with a custom query execution backend.
    pub fn with_query_backend(query_backend: Arc<dyn RedshiftQueryBackend>) -> Self {
        Self {
            query_backend,
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for RedshiftDataService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for RedshiftDataService {
    fn service_name(&self) -> &str {
        "redshift-data"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://redshift-data\.(.+)\.amazonaws\.com",
            r"https?://redshift-data\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl RedshiftDataService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "RedshiftData.ExecuteStatement"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.rsplit('.').next())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        match action.as_str() {
            "ExecuteStatement" => self.handle_execute_statement(&state, body_bytes).await,
            "BatchExecuteStatement" => {
                self.handle_batch_execute_statement(&state, body_bytes)
                    .await
            }
            "DescribeStatement" => self.handle_describe_statement(&state, body_bytes).await,
            "DescribeTable" => self.handle_describe_table(&state, body_bytes).await,
            "CancelStatement" => self.handle_cancel_statement(&state, body_bytes).await,
            "ListStatements" => self.handle_list_statements(&state).await,
            "GetStatementResult" => self.handle_get_statement_result(&state, body_bytes).await,
            "GetStatementResultV2" => {
                self.handle_get_statement_result_v2(&state, body_bytes)
                    .await
            }
            "ListDatabases" => self.handle_list_databases(&state, body_bytes).await,
            "ListSchemas" => self.handle_list_schemas(&state, body_bytes).await,
            "ListTables" => self.handle_list_tables(&state, body_bytes).await,
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for RedshiftData"),
            ),
        }
    }

    async fn handle_execute_statement(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftDataState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_execute_statement_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.sql.is_empty() {
            return json_error_response(400, "ValidationException", "Sql is required");
        }
        let database = match input.database.as_deref() {
            Some(d) => d,
            None => {
                return json_error_response(400, "ValidationException", "Database is required");
            }
        };

        let sql = input.sql.as_str();
        let cluster_identifier = input.cluster_identifier.as_deref();
        let workgroup_name = input.workgroup_name.as_deref();
        let db_user = input.db_user.as_deref();
        let secret_arn = input.secret_arn.as_deref();

        let parameters: Vec<StatementParameter> = input
            .parameters
            .unwrap_or_default()
            .into_iter()
            .map(|p| StatementParameter {
                name: p.name,
                value: p.value,
            })
            .collect();

        let result = self.query_backend.execute_statement(sql.to_string()).await;

        let mut state = state.write().await;
        match state.execute_statement(
            sql,
            database,
            cluster_identifier,
            workgroup_name,
            db_user,
            secret_arn,
            parameters,
            result,
        ) {
            Ok(id) => {
                let output = ExecuteStatementOutput {
                    id: Some(id),
                    created_at: Some(chrono::Utc::now().timestamp() as f64),
                    database: Some(database.to_string()),
                    cluster_identifier: cluster_identifier.map(String::from),
                    workgroup_name: workgroup_name.map(String::from),
                    db_user: db_user.map(String::from),
                    secret_arn: secret_arn.map(String::from),
                    ..Default::default()
                };
                wire::serialize_execute_statement_response(&output)
            }
            Err(e) => redshiftdata_error_response(&e),
        }
    }

    async fn handle_describe_statement(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftDataState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_statement_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "ValidationException", "Id is required");
        }
        let id = input.id.as_str();

        let state = state.read().await;
        match state.describe_statement(id) {
            Ok(stmt) => {
                let resp = statement_to_describe_response(stmt);
                wire::serialize_describe_statement_response(&resp)
            }
            Err(e) => redshiftdata_error_response(&e),
        }
    }

    async fn handle_cancel_statement(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftDataState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_statement_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "ValidationException", "Id is required");
        }
        let id = input.id.as_str();

        let mut state = state.write().await;
        match state.cancel_statement(id) {
            Ok(status) => {
                let resp = CancelStatementResponse {
                    status: Some(status),
                };
                wire::serialize_cancel_statement_response(&resp)
            }
            Err(e) => redshiftdata_error_response(&e),
        }
    }

    async fn handle_list_statements(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftDataState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let stmts = state.list_statements();
        let entries: Vec<StatementData> = stmts
            .iter()
            .map(|s| statement_to_statement_data(s))
            .collect();

        let resp = ListStatementsResponse {
            statements: Some(entries),
            next_token: None,
        };
        wire::serialize_list_statements_response(&resp)
    }

    async fn handle_get_statement_result(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftDataState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_statement_result_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "ValidationException", "Id is required");
        }
        let id = input.id.as_str();

        let state = state.read().await;
        match state.describe_statement(id) {
            Ok(stmt) => {
                let column_metadata: Vec<ColumnMetadata> = stmt
                    .result_columns
                    .iter()
                    .map(|(name, type_str)| ColumnMetadata {
                        name: Some(name.clone()),
                        type_name: Some(type_str.clone()),
                        ..Default::default()
                    })
                    .collect();

                let records: Vec<Vec<Field>> = stmt
                    .result_data
                    .iter()
                    .map(|row| {
                        row.iter()
                            .zip(stmt.result_columns.iter())
                            .map(|(cell, (_, type_str))| string_to_field(cell, type_str))
                            .collect()
                    })
                    .collect();

                let total = records.len() as i64;
                let resp = GetStatementResultResponse {
                    records: Some(records),
                    column_metadata: Some(column_metadata),
                    total_num_rows: Some(total),
                    next_token: None,
                };
                wire::serialize_get_statement_result_response(&resp)
            }
            Err(e) => redshiftdata_error_response(&e),
        }
    }

    async fn handle_batch_execute_statement(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftDataState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_execute_statement_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.sqls.is_empty() {
            return json_error_response(400, "ValidationException", "Sqls is required");
        }
        let sqls: Vec<String> = input.sqls;

        let database = match input.database.as_deref() {
            Some(d) => d,
            None => {
                return json_error_response(400, "ValidationException", "Database is required");
            }
        };

        let cluster_identifier = input.cluster_identifier.as_deref();
        let workgroup_name = input.workgroup_name.as_deref();
        let db_user = input.db_user.as_deref();
        let secret_arn = input.secret_arn.as_deref();
        let statement_name = input.statement_name.as_deref();

        let result = self.query_backend.batch_execute(sqls.clone()).await;

        let mut state = state.write().await;
        match state.batch_execute_statement(
            sqls.clone(),
            database,
            cluster_identifier,
            workgroup_name,
            db_user,
            secret_arn,
            statement_name,
            result,
        ) {
            Ok(id) => {
                let output = crate::model::BatchExecuteStatementOutput {
                    id: Some(id),
                    created_at: Some(chrono::Utc::now().timestamp() as f64),
                    database: Some(database.to_string()),
                    cluster_identifier: cluster_identifier.map(String::from),
                    workgroup_name: workgroup_name.map(String::from),
                    db_user: db_user.map(String::from),
                    secret_arn: secret_arn.map(String::from),
                    ..Default::default()
                };
                wire::serialize_batch_execute_statement_response(&output)
            }
            Err(e) => redshiftdata_error_response(&e),
        }
    }

    async fn handle_describe_table(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftDataState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let table_name = input.table.as_deref();

        let state = state.read().await;
        let columns = state.describe_table(table_name);
        let column_list: Vec<ColumnMetadata> = columns
            .iter()
            .map(|(name, type_str)| ColumnMetadata {
                name: Some(name.clone()),
                type_name: Some(type_str.clone()),
                ..Default::default()
            })
            .collect();

        let resp = crate::model::DescribeTableResponse {
            column_list: Some(column_list),
            next_token: None,
            table_name: table_name.map(String::from),
        };
        wire::serialize_describe_table_response(&resp)
    }

    async fn handle_get_statement_result_v2(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftDataState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_statement_result_v2_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "ValidationException", "Id is required");
        }
        let id = input.id.as_str();

        let state = state.read().await;
        match state.describe_statement(id) {
            Ok(stmt) => {
                let column_metadata: Vec<ColumnMetadata> = stmt
                    .result_columns
                    .iter()
                    .map(|(name, type_str)| ColumnMetadata {
                        name: Some(name.clone()),
                        type_name: Some(type_str.clone()),
                        ..Default::default()
                    })
                    .collect();

                // V2 format: each record row is serialised as CSV
                let records: Vec<crate::model::QueryRecords> = stmt
                    .result_data
                    .iter()
                    .map(|row| {
                        let csv_line: String = row
                            .iter()
                            .map(|cell| cell.as_deref().unwrap_or(""))
                            .collect::<Vec<&str>>()
                            .join(",");
                        crate::model::QueryRecords {
                            c_s_v_records: Some(csv_line),
                        }
                    })
                    .collect();

                let total = records.len() as i64;
                let resp = crate::model::GetStatementResultV2Response {
                    column_metadata: Some(column_metadata),
                    next_token: None,
                    records: Some(records),
                    result_format: Some("CSV".to_string()),
                    total_num_rows: Some(total),
                };
                wire::serialize_get_statement_result_v2_response(&resp)
            }
            Err(e) => redshiftdata_error_response(&e),
        }
    }

    async fn handle_list_databases(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftDataState>>,
        body: &[u8],
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_databases_request(body) {
            return json_error_response(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let databases = state.list_databases();
        let resp = crate::model::ListDatabasesResponse {
            databases: Some(databases),
            next_token: None,
        };
        wire::serialize_list_databases_response(&resp)
    }

    async fn handle_list_schemas(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftDataState>>,
        body: &[u8],
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_schemas_request(body) {
            return json_error_response(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let schemas = state.list_schemas();
        let resp = crate::model::ListSchemasResponse {
            schemas: Some(schemas),
            next_token: None,
        };
        wire::serialize_list_schemas_response(&resp)
    }

    async fn handle_list_tables(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftDataState>>,
        body: &[u8],
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_tables_request(body) {
            return json_error_response(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let table_names = state.list_tables();
        let tables: Vec<crate::model::TableMember> = table_names
            .iter()
            .map(|name| crate::model::TableMember {
                name: Some(name.clone()),
                ..Default::default()
            })
            .collect();
        let resp = crate::model::ListTablesResponse {
            tables: Some(tables),
            next_token: None,
        };
        wire::serialize_list_tables_response(&resp)
    }
}

/// Convert a state Statement to a wire DescribeStatementResponse.
fn statement_to_describe_response(stmt: &Statement) -> DescribeStatementResponse {
    let query_parameters: Option<Vec<SqlParameter>> = if stmt.parameters.is_empty() {
        None
    } else {
        Some(
            stmt.parameters
                .iter()
                .map(|p| SqlParameter {
                    name: p.name.clone(),
                    value: p.value.clone(),
                })
                .collect(),
        )
    };

    DescribeStatementResponse {
        id: Some(stmt.id.clone()),
        status: Some(stmt.status.as_str().to_string()),
        created_at: Some(stmt.created_at.timestamp() as f64),
        updated_at: Some(stmt.updated_at.timestamp() as f64),
        query_string: Some(stmt.query_string.clone()),
        database: Some(stmt.database.clone()),
        has_result_set: Some(stmt.has_result_set),
        result_rows: Some(stmt.result_rows),
        result_size: Some(stmt.result_size),
        duration: Some(0),
        cluster_identifier: stmt.cluster_identifier.clone(),
        workgroup_name: stmt.workgroup_name.clone(),
        db_user: stmt.db_user.clone(),
        secret_arn: stmt.secret_arn.clone(),
        query_parameters,
        ..Default::default()
    }
}

/// Convert a state Statement to a wire StatementData for list responses.
fn statement_to_statement_data(stmt: &Statement) -> StatementData {
    StatementData {
        id: Some(stmt.id.clone()),
        status: Some(stmt.status.as_str().to_string()),
        created_at: Some(stmt.created_at.timestamp() as f64),
        updated_at: Some(stmt.updated_at.timestamp() as f64),
        query_string: Some(stmt.query_string.clone()),
        statement_name: Some(String::new()),
        is_batch_statement: Some(false),
        secret_arn: stmt.secret_arn.clone(),
        ..Default::default()
    }
}

/// Convert a stored string cell to a typed [`Field`] using the column's
/// declared type string (as returned by the query backend).
fn string_to_field(value: &Option<String>, type_str: &str) -> Field {
    match value {
        None => Field {
            is_null: Some(true),
            ..Default::default()
        },
        Some(s) => {
            let t = type_str.to_ascii_lowercase();
            if t.contains("int") || t.contains("bigint") {
                if let Ok(n) = s.parse::<i64>() {
                    return Field {
                        long_value: Some(n),
                        ..Default::default()
                    };
                }
            }
            if t.contains("float") || t.contains("double") || t.contains("real") {
                if let Ok(f) = s.parse::<f64>() {
                    return Field {
                        double_value: Some(f),
                        ..Default::default()
                    };
                }
            }
            if t.contains("bool") {
                let b = s == "true" || s == "1";
                return Field {
                    boolean_value: Some(b),
                    ..Default::default()
                };
            }
            Field {
                string_value: Some(s.clone()),
                ..Default::default()
            }
        }
    }
}

fn redshiftdata_error_response(err: &RedshiftDataError) -> MockResponse {
    let (status, error_type) = match err {
        RedshiftDataError::SqlRequired
        | RedshiftDataError::SqlsRequired
        | RedshiftDataError::InvalidStatementId => (400, "ValidationException"),
        RedshiftDataError::StatementNotFound => (404, "ResourceNotFoundException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
