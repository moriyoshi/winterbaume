//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-redshift-data

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchExecuteStatementInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "DbUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<SqlParameter>>,
    #[serde(rename = "ResultFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_format: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "SessionKeepAliveSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_keep_alive_seconds: Option<i32>,
    #[serde(rename = "Sqls")]
    #[serde(default)]
    pub sqls: Vec<String>,
    #[serde(rename = "StatementName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_name: Option<String>,
    #[serde(rename = "WithEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_event: Option<bool>,
    #[serde(rename = "WorkgroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqlParameter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchExecuteStatementOutput {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "DbGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_groups: Option<Vec<String>>,
    #[serde(rename = "DbUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "WorkgroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelStatementRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelStatementResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStatementRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStatementResponse {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "DbUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "HasResultSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_result_set: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "QueryParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<Vec<SqlParameter>>,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "RedshiftPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_pid: Option<i64>,
    #[serde(rename = "RedshiftQueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_query_id: Option<i64>,
    #[serde(rename = "ResultFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_format: Option<String>,
    #[serde(rename = "ResultRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_rows: Option<i64>,
    #[serde(rename = "ResultSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_size: Option<i64>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubStatements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_statements: Option<Vec<SubStatementData>>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    #[serde(rename = "WorkgroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubStatementData {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "HasResultSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_result_set: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "RedshiftQueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_query_id: Option<i64>,
    #[serde(rename = "ResultRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_rows: Option<i64>,
    #[serde(rename = "ResultSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_size: Option<i64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTableRequest {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "ConnectedDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_database: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "DbUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(rename = "WorkgroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTableResponse {
    #[serde(rename = "ColumnList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_list: Option<Vec<ColumnMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnMetadata {
    #[serde(rename = "columnDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_default: Option<String>,
    #[serde(rename = "isCaseSensitive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_case_sensitive: Option<bool>,
    #[serde(rename = "isCurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_currency: Option<bool>,
    #[serde(rename = "isSigned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_signed: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<i32>,
    #[serde(rename = "schemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[serde(rename = "tableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "typeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteStatementInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "DbUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<SqlParameter>>,
    #[serde(rename = "ResultFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_format: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "SessionKeepAliveSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_keep_alive_seconds: Option<i32>,
    #[serde(rename = "Sql")]
    #[serde(default)]
    pub sql: String,
    #[serde(rename = "StatementName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_name: Option<String>,
    #[serde(rename = "WithEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_event: Option<bool>,
    #[serde(rename = "WorkgroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteStatementOutput {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "DbGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_groups: Option<Vec<String>>,
    #[serde(rename = "DbUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "WorkgroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStatementResultRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStatementResultResponse {
    #[serde(rename = "ColumnMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_metadata: Option<Vec<ColumnMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Records")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Vec<Field>>>,
    #[serde(rename = "TotalNumRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num_rows: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Field {
    #[serde(rename = "blobValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_value: Option<String>,
    #[serde(rename = "booleanValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    #[serde(rename = "doubleValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    #[serde(rename = "isNull")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_null: Option<bool>,
    #[serde(rename = "longValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_value: Option<i64>,
    #[serde(rename = "stringValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStatementResultV2Request {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStatementResultV2Response {
    #[serde(rename = "ColumnMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_metadata: Option<Vec<ColumnMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Records")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<QueryRecords>>,
    #[serde(rename = "ResultFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_format: Option<String>,
    #[serde(rename = "TotalNumRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num_rows: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryRecords {
    #[serde(rename = "CSVRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_s_v_records: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatabasesRequest {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "DbUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "WorkgroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatabasesResponse {
    #[serde(rename = "Databases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSchemasRequest {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "ConnectedDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_database: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "DbUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_pattern: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "WorkgroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSchemasResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Schemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStatementsRequest {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RoleLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_level: Option<bool>,
    #[serde(rename = "StatementName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "WorkgroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStatementsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Statements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<StatementData>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatementData {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsBatchStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_batch_statement: Option<bool>,
    #[serde(rename = "QueryParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<Vec<SqlParameter>>,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "QueryStrings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_strings: Option<Vec<String>>,
    #[serde(rename = "ResultFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_format: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "StatementName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTablesRequest {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "ConnectedDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_database: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "DbUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_pattern: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "TablePattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_pattern: Option<String>,
    #[serde(rename = "WorkgroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTablesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<TableMember>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableMember {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
