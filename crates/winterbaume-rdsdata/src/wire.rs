//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-rdsdata

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restJson protocol.
pub fn serialize_batch_execute_statement_response(
    result: &BatchExecuteStatementResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_begin_transaction_response(result: &BeginTransactionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_commit_transaction_response(result: &CommitTransactionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_execute_sql_response(result: &ExecuteSqlResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_execute_statement_response(result: &ExecuteStatementResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_rollback_transaction_response(
    result: &RollbackTransactionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_execute_statement_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchExecuteStatementRequest, String> {
    let mut input = BatchExecuteStatementRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchExecuteStatementRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchExecuteStatement request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_begin_transaction_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BeginTransactionRequest, String> {
    let mut input = BeginTransactionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BeginTransactionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BeginTransaction request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_commit_transaction_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CommitTransactionRequest, String> {
    let mut input = CommitTransactionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CommitTransactionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CommitTransaction request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_execute_sql_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ExecuteSqlRequest, String> {
    let mut input = ExecuteSqlRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ExecuteSqlRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ExecuteSql request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_execute_statement_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ExecuteStatementRequest, String> {
    let mut input = ExecuteStatementRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ExecuteStatementRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ExecuteStatement request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_rollback_transaction_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RollbackTransactionRequest, String> {
    let mut input = RollbackTransactionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RollbackTransactionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RollbackTransaction request: {err}"))?;
    }
    Ok(input)
}
