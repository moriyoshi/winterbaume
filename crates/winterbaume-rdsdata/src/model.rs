//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-rdsdata

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchExecuteStatementRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "parameterSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_sets: Option<Vec<Vec<SqlParameter>>>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    pub secret_arn: String,
    #[serde(default)]
    pub sql: String,
    #[serde(rename = "transactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqlParameter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "typeHint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_hint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Field>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Field {
    #[serde(rename = "arrayValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_value: Option<ArrayValue>,
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
pub struct ArrayValue {
    #[serde(rename = "arrayValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_values: Option<Vec<ArrayValue>>,
    #[serde(rename = "booleanValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_values: Option<Vec<bool>>,
    #[serde(rename = "doubleValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_values: Option<Vec<f64>>,
    #[serde(rename = "longValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_values: Option<Vec<i64>>,
    #[serde(rename = "stringValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchExecuteStatementResponse {
    #[serde(rename = "updateResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_results: Option<Vec<UpdateResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResult {
    #[serde(rename = "generatedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_fields: Option<Vec<Field>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BeginTransactionRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    pub secret_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BeginTransactionResponse {
    #[serde(rename = "transactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommitTransactionRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    pub secret_arn: String,
    #[serde(rename = "transactionId")]
    #[serde(default)]
    pub transaction_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommitTransactionResponse {
    #[serde(rename = "transactionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteSqlRequest {
    #[serde(rename = "awsSecretStoreArn")]
    #[serde(default)]
    pub aws_secret_store_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "dbClusterOrInstanceArn")]
    #[serde(default)]
    pub db_cluster_or_instance_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "sqlStatements")]
    #[serde(default)]
    pub sql_statements: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteSqlResponse {
    #[serde(rename = "sqlStatementResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_statement_results: Option<Vec<SqlStatementResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqlStatementResult {
    #[serde(rename = "numberOfRecordsUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_records_updated: Option<i64>,
    #[serde(rename = "resultFrame")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_frame: Option<ResultFrame>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultFrame {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
    #[serde(rename = "resultSetMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_set_metadata: Option<ResultSetMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Record {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Value {
    #[serde(rename = "arrayValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_values: Option<Vec<Value>>,
    #[serde(rename = "bigIntValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_int_value: Option<i64>,
    #[serde(rename = "bitValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_value: Option<bool>,
    #[serde(rename = "blobValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_value: Option<String>,
    #[serde(rename = "doubleValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    #[serde(rename = "intValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_value: Option<i32>,
    #[serde(rename = "isNull")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_null: Option<bool>,
    #[serde(rename = "realValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_value: Option<f32>,
    #[serde(rename = "stringValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
    #[serde(rename = "structValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub struct_value: Option<Box<StructValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StructValue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultSetMetadata {
    #[serde(rename = "columnCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i64>,
    #[serde(rename = "columnMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_metadata: Option<Vec<ColumnMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnMetadata {
    #[serde(rename = "arrayBaseColumnType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_base_column_type: Option<i32>,
    #[serde(rename = "isAutoIncrement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_increment: Option<bool>,
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
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(rename = "typeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteStatementRequest {
    #[serde(rename = "continueAfterTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_after_timeout: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "formatRecordsAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_records_as: Option<String>,
    #[serde(rename = "includeResultMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_result_metadata: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<SqlParameter>>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "resultSetOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_set_options: Option<ResultSetOptions>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    pub secret_arn: String,
    #[serde(default)]
    pub sql: String,
    #[serde(rename = "transactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultSetOptions {
    #[serde(rename = "decimalReturnType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_return_type: Option<String>,
    #[serde(rename = "longReturnType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_return_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteStatementResponse {
    #[serde(rename = "columnMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_metadata: Option<Vec<ColumnMetadata>>,
    #[serde(rename = "formattedRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_records: Option<String>,
    #[serde(rename = "generatedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_fields: Option<Vec<Field>>,
    #[serde(rename = "numberOfRecordsUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_records_updated: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Vec<Field>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RollbackTransactionRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    pub secret_arn: String,
    #[serde(rename = "transactionId")]
    #[serde(default)]
    pub transaction_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RollbackTransactionResponse {
    #[serde(rename = "transactionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status: Option<String>,
}
