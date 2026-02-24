//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-redshift-data

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_batch_execute_statement_response(
    result: &BatchExecuteStatementOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_statement_response(result: &CancelStatementResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_statement_response(result: &DescribeStatementResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_table_response(result: &DescribeTableResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_execute_statement_response(result: &ExecuteStatementOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_statement_result_response(
    result: &GetStatementResultResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_statement_result_v2_response(
    result: &GetStatementResultV2Response,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_databases_response(result: &ListDatabasesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_schemas_response(result: &ListSchemasResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_statements_response(result: &ListStatementsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tables_response(result: &ListTablesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_execute_statement_request(
    body: &[u8],
) -> Result<BatchExecuteStatementInput, String> {
    if body.is_empty() {
        return Ok(BatchExecuteStatementInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchExecuteStatement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_statement_request(body: &[u8]) -> Result<CancelStatementRequest, String> {
    if body.is_empty() {
        return Ok(CancelStatementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelStatement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_statement_request(
    body: &[u8],
) -> Result<DescribeStatementRequest, String> {
    if body.is_empty() {
        return Ok(DescribeStatementRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeStatement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_table_request(body: &[u8]) -> Result<DescribeTableRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTableRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_execute_statement_request(body: &[u8]) -> Result<ExecuteStatementInput, String> {
    if body.is_empty() {
        return Ok(ExecuteStatementInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ExecuteStatement request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_statement_result_request(
    body: &[u8],
) -> Result<GetStatementResultRequest, String> {
    if body.is_empty() {
        return Ok(GetStatementResultRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetStatementResult request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_statement_result_v2_request(
    body: &[u8],
) -> Result<GetStatementResultV2Request, String> {
    if body.is_empty() {
        return Ok(GetStatementResultV2Request::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetStatementResultV2 request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_databases_request(body: &[u8]) -> Result<ListDatabasesRequest, String> {
    if body.is_empty() {
        return Ok(ListDatabasesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDatabases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_schemas_request(body: &[u8]) -> Result<ListSchemasRequest, String> {
    if body.is_empty() {
        return Ok(ListSchemasRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSchemas request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_statements_request(body: &[u8]) -> Result<ListStatementsRequest, String> {
    if body.is_empty() {
        return Ok(ListStatementsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListStatements request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tables_request(body: &[u8]) -> Result<ListTablesRequest, String> {
    if body.is_empty() {
        return Ok(ListTablesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTables request: {e}"))
}
