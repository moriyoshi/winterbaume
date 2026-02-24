//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-athena

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetNamedQueryInput {
    #[serde(rename = "NamedQueryIds")]
    #[serde(default)]
    pub named_query_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetNamedQueryOutput {
    #[serde(rename = "NamedQueries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_queries: Option<Vec<NamedQuery>>,
    #[serde(rename = "UnprocessedNamedQueryIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_named_query_ids: Option<Vec<UnprocessedNamedQueryId>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NamedQuery {
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NamedQueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query_id: Option<String>,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedNamedQueryId {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "NamedQueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetPreparedStatementInput {
    #[serde(rename = "PreparedStatementNames")]
    #[serde(default)]
    pub prepared_statement_names: Vec<String>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    pub work_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetPreparedStatementOutput {
    #[serde(rename = "PreparedStatements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_statements: Option<Vec<PreparedStatement>>,
    #[serde(rename = "UnprocessedPreparedStatementNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_prepared_statement_names: Option<Vec<UnprocessedPreparedStatementName>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PreparedStatement {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "QueryStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_statement: Option<String>,
    #[serde(rename = "StatementName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_name: Option<String>,
    #[serde(rename = "WorkGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedPreparedStatementName {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "StatementName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetQueryExecutionInput {
    #[serde(rename = "QueryExecutionIds")]
    #[serde(default)]
    pub query_execution_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetQueryExecutionOutput {
    #[serde(rename = "QueryExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_executions: Option<Vec<QueryExecution>>,
    #[serde(rename = "UnprocessedQueryExecutionIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_query_execution_ids: Option<Vec<UnprocessedQueryExecutionId>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryExecution {
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<EngineVersion>,
    #[serde(rename = "ExecutionParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_parameters: Option<Vec<String>>,
    #[serde(rename = "ManagedQueryResultsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_query_results_configuration: Option<ManagedQueryResultsConfiguration>,
    #[serde(rename = "Query")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "QueryExecutionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_context: Option<QueryExecutionContext>,
    #[serde(rename = "QueryExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_id: Option<String>,
    #[serde(rename = "QueryResultsS3AccessGrantsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_results_s3_access_grants_configuration:
        Option<QueryResultsS3AccessGrantsConfiguration>,
    #[serde(rename = "ResultConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_configuration: Option<ResultConfiguration>,
    #[serde(rename = "ResultReuseConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_reuse_configuration: Option<ResultReuseConfiguration>,
    #[serde(rename = "StatementType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_type: Option<String>,
    #[serde(rename = "Statistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<QueryExecutionStatistics>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryExecutionStatus>,
    #[serde(rename = "SubstatementType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substatement_type: Option<String>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EngineVersion {
    #[serde(rename = "EffectiveEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_engine_version: Option<String>,
    #[serde(rename = "SelectedEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_engine_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedQueryResultsConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<ManagedQueryResultsEncryptionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedQueryResultsEncryptionConfiguration {
    #[serde(rename = "KmsKey")]
    #[serde(default)]
    pub kms_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryExecutionContext {
    #[serde(rename = "Catalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryResultsS3AccessGrantsConfiguration {
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    pub authentication_type: String,
    #[serde(rename = "CreateUserLevelPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_user_level_prefix: Option<bool>,
    #[serde(rename = "EnableS3AccessGrants")]
    #[serde(default)]
    pub enable_s3_access_grants: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultConfiguration {
    #[serde(rename = "AclConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_configuration: Option<AclConfiguration>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "OutputLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AclConfiguration {
    #[serde(rename = "S3AclOption")]
    #[serde(default)]
    pub s3_acl_option: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "EncryptionOption")]
    #[serde(default)]
    pub encryption_option: String,
    #[serde(rename = "KmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultReuseConfiguration {
    #[serde(rename = "ResultReuseByAgeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_reuse_by_age_configuration: Option<ResultReuseByAgeConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultReuseByAgeConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "MaxAgeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_in_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryExecutionStatistics {
    #[serde(rename = "DataManifestLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_manifest_location: Option<String>,
    #[serde(rename = "DataScannedInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_scanned_in_bytes: Option<i64>,
    #[serde(rename = "DpuCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpu_count: Option<f64>,
    #[serde(rename = "EngineExecutionTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_execution_time_in_millis: Option<i64>,
    #[serde(rename = "QueryPlanningTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_planning_time_in_millis: Option<i64>,
    #[serde(rename = "QueryQueueTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_queue_time_in_millis: Option<i64>,
    #[serde(rename = "ResultReuseInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_reuse_information: Option<ResultReuseInformation>,
    #[serde(rename = "ServicePreProcessingTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_pre_processing_time_in_millis: Option<i64>,
    #[serde(rename = "ServiceProcessingTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_processing_time_in_millis: Option<i64>,
    #[serde(rename = "TotalExecutionTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_execution_time_in_millis: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultReuseInformation {
    #[serde(rename = "ReusedPreviousResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reused_previous_result: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryExecutionStatus {
    #[serde(rename = "AthenaError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athena_error: Option<AthenaError>,
    #[serde(rename = "CompletionDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateChangeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<String>,
    #[serde(rename = "SubmissionDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AthenaError {
    #[serde(rename = "ErrorCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_category: Option<i32>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ErrorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<i32>,
    #[serde(rename = "Retryable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retryable: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedQueryExecutionId {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "QueryExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelCapacityReservationInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelCapacityReservationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCapacityReservationInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetDpus")]
    #[serde(default)]
    pub target_dpus: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCapacityReservationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataCatalogInput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataCatalogOutput {
    #[serde(rename = "DataCatalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog: Option<DataCatalog>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataCatalog {
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNamedQueryInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    pub query_string: String,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNamedQueryOutput {
    #[serde(rename = "NamedQueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNotebookInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    pub work_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNotebookOutput {
    #[serde(rename = "NotebookId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePreparedStatementInput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "QueryStatement")]
    #[serde(default)]
    pub query_statement: String,
    #[serde(rename = "StatementName")]
    #[serde(default)]
    pub statement_name: String,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    pub work_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePreparedStatementOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePresignedNotebookUrlRequest {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePresignedNotebookUrlResponse {
    #[serde(rename = "AuthToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "AuthTokenExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token_expiration_time: Option<i64>,
    #[serde(rename = "NotebookUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkGroupInput {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<WorkGroupConfiguration>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkGroupConfiguration {
    #[serde(rename = "AdditionalConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_configuration: Option<String>,
    #[serde(rename = "BytesScannedCutoffPerQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_scanned_cutoff_per_query: Option<i64>,
    #[serde(rename = "CustomerContentEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_content_encryption_configuration: Option<CustomerContentEncryptionConfiguration>,
    #[serde(rename = "EnableMinimumEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_minimum_encryption_configuration: Option<bool>,
    #[serde(rename = "EnforceWorkGroupConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_work_group_configuration: Option<bool>,
    #[serde(rename = "EngineConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_configuration: Option<EngineConfiguration>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<EngineVersion>,
    #[serde(rename = "ExecutionRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role: Option<String>,
    #[serde(rename = "IdentityCenterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_configuration: Option<IdentityCenterConfiguration>,
    #[serde(rename = "ManagedQueryResultsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_query_results_configuration: Option<ManagedQueryResultsConfiguration>,
    #[serde(rename = "MonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
    #[serde(rename = "PublishCloudWatchMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_cloud_watch_metrics_enabled: Option<bool>,
    #[serde(rename = "QueryResultsS3AccessGrantsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_results_s3_access_grants_configuration:
        Option<QueryResultsS3AccessGrantsConfiguration>,
    #[serde(rename = "RequesterPaysEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_pays_enabled: Option<bool>,
    #[serde(rename = "ResultConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_configuration: Option<ResultConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomerContentEncryptionConfiguration {
    #[serde(rename = "KmsKey")]
    #[serde(default)]
    pub kms_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EngineConfiguration {
    #[serde(rename = "AdditionalConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_configs: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Classifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifications: Option<Vec<Classification>>,
    #[serde(rename = "CoordinatorDpuSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinator_dpu_size: Option<i32>,
    #[serde(rename = "DefaultExecutorDpuSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_executor_dpu_size: Option<i32>,
    #[serde(rename = "MaxConcurrentDpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_dpus: Option<i32>,
    #[serde(rename = "SparkProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Classification {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityCenterConfiguration {
    #[serde(rename = "EnableIdentityCenter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_identity_center: Option<bool>,
    #[serde(rename = "IdentityCenterInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_instance_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitoringConfiguration {
    #[serde(rename = "CloudWatchLoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_configuration: Option<CloudWatchLoggingConfiguration>,
    #[serde(rename = "ManagedLoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_logging_configuration: Option<ManagedLoggingConfiguration>,
    #[serde(rename = "S3LoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logging_configuration: Option<S3LoggingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLoggingConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "LogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
    #[serde(rename = "LogStreamNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefix: Option<String>,
    #[serde(rename = "LogTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_types: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedLoggingConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "KmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3LoggingConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "KmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    #[serde(rename = "LogLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkGroupOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCapacityReservationInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCapacityReservationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataCatalogInput {
    #[serde(rename = "DeleteCatalogOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_catalog_only: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataCatalogOutput {
    #[serde(rename = "DataCatalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog: Option<DataCatalog>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNamedQueryInput {
    #[serde(rename = "NamedQueryId")]
    #[serde(default)]
    pub named_query_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNamedQueryOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNotebookInput {
    #[serde(rename = "NotebookId")]
    #[serde(default)]
    pub notebook_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNotebookOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePreparedStatementInput {
    #[serde(rename = "StatementName")]
    #[serde(default)]
    pub statement_name: String,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    pub work_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePreparedStatementOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkGroupInput {
    #[serde(rename = "RecursiveDeleteOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_delete_option: Option<bool>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    pub work_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkGroupOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportNotebookInput {
    #[serde(rename = "NotebookId")]
    #[serde(default)]
    pub notebook_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportNotebookOutput {
    #[serde(rename = "NotebookMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_metadata: Option<NotebookMetadata>,
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotebookMetadata {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NotebookId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCalculationExecutionCodeRequest {
    #[serde(rename = "CalculationExecutionId")]
    #[serde(default)]
    pub calculation_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCalculationExecutionCodeResponse {
    #[serde(rename = "CodeBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_block: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCalculationExecutionRequest {
    #[serde(rename = "CalculationExecutionId")]
    #[serde(default)]
    pub calculation_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCalculationExecutionResponse {
    #[serde(rename = "CalculationExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_execution_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<CalculationResult>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "Statistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<CalculationStatistics>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CalculationStatus>,
    #[serde(rename = "WorkingDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CalculationResult {
    #[serde(rename = "ResultS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_s3_uri: Option<String>,
    #[serde(rename = "ResultType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<String>,
    #[serde(rename = "StdErrorS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_error_s3_uri: Option<String>,
    #[serde(rename = "StdOutS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_out_s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CalculationStatistics {
    #[serde(rename = "DpuExecutionInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpu_execution_in_millis: Option<i64>,
    #[serde(rename = "Progress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CalculationStatus {
    #[serde(rename = "CompletionDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateChangeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<String>,
    #[serde(rename = "SubmissionDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCalculationExecutionStatusRequest {
    #[serde(rename = "CalculationExecutionId")]
    #[serde(default)]
    pub calculation_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCalculationExecutionStatusResponse {
    #[serde(rename = "Statistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<CalculationStatistics>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CalculationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCapacityAssignmentConfigurationInput {
    #[serde(rename = "CapacityReservationName")]
    #[serde(default)]
    pub capacity_reservation_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCapacityAssignmentConfigurationOutput {
    #[serde(rename = "CapacityAssignmentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_assignment_configuration: Option<CapacityAssignmentConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityAssignmentConfiguration {
    #[serde(rename = "CapacityAssignments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_assignments: Option<Vec<CapacityAssignment>>,
    #[serde(rename = "CapacityReservationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityAssignment {
    #[serde(rename = "WorkGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCapacityReservationInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCapacityReservationOutput {
    #[serde(rename = "CapacityReservation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation: Option<CapacityReservation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityReservation {
    #[serde(rename = "AllocatedDpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_dpus: Option<i32>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastAllocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_allocation: Option<CapacityAllocation>,
    #[serde(rename = "LastSuccessfulAllocationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_allocation_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetDpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_dpus: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityAllocation {
    #[serde(rename = "RequestCompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_completion_time: Option<f64>,
    #[serde(rename = "RequestTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataCatalogInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataCatalogOutput {
    #[serde(rename = "DataCatalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog: Option<DataCatalog>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDatabaseInput {
    #[serde(rename = "CatalogName")]
    #[serde(default)]
    pub catalog_name: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDatabaseOutput {
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<Database>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Database {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNamedQueryInput {
    #[serde(rename = "NamedQueryId")]
    #[serde(default)]
    pub named_query_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNamedQueryOutput {
    #[serde(rename = "NamedQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query: Option<NamedQuery>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNotebookMetadataInput {
    #[serde(rename = "NotebookId")]
    #[serde(default)]
    pub notebook_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNotebookMetadataOutput {
    #[serde(rename = "NotebookMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_metadata: Option<NotebookMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPreparedStatementInput {
    #[serde(rename = "StatementName")]
    #[serde(default)]
    pub statement_name: String,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    pub work_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPreparedStatementOutput {
    #[serde(rename = "PreparedStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_statement: Option<PreparedStatement>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueryExecutionInput {
    #[serde(rename = "QueryExecutionId")]
    #[serde(default)]
    pub query_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueryExecutionOutput {
    #[serde(rename = "QueryExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution: Option<QueryExecution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueryResultsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueryExecutionId")]
    #[serde(default)]
    pub query_execution_id: String,
    #[serde(rename = "QueryResultType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_result_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueryResultsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResultSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_set: Option<ResultSet>,
    #[serde(rename = "UpdateCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultSet {
    #[serde(rename = "ResultSetMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_set_metadata: Option<ResultSetMetadata>,
    #[serde(rename = "Rows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<Row>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultSetMetadata {
    #[serde(rename = "ColumnInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_info: Option<Vec<ColumnInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnInfo {
    #[serde(rename = "CaseSensitive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
    #[serde(rename = "CatalogName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Nullable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<String>,
    #[serde(rename = "Precision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i32>,
    #[serde(rename = "Scale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<i32>,
    #[serde(rename = "SchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Row {
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Datum>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Datum {
    #[serde(rename = "VarCharValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub var_char_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueryRuntimeStatisticsInput {
    #[serde(rename = "QueryExecutionId")]
    #[serde(default)]
    pub query_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueryRuntimeStatisticsOutput {
    #[serde(rename = "QueryRuntimeStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_runtime_statistics: Option<QueryRuntimeStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryRuntimeStatistics {
    #[serde(rename = "OutputStage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_stage: Option<QueryStage>,
    #[serde(rename = "Rows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<QueryRuntimeStatisticsRows>,
    #[serde(rename = "Timeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<QueryRuntimeStatisticsTimeline>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryStage {
    #[serde(rename = "ExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<i64>,
    #[serde(rename = "InputBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_bytes: Option<i64>,
    #[serde(rename = "InputRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_rows: Option<i64>,
    #[serde(rename = "OutputBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_bytes: Option<i64>,
    #[serde(rename = "OutputRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_rows: Option<i64>,
    #[serde(rename = "QueryStagePlan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_stage_plan: Option<QueryStagePlanNode>,
    #[serde(rename = "StageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<i64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "SubStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_stages: Option<Vec<QueryStage>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryStagePlanNode {
    #[serde(rename = "Children")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<QueryStagePlanNode>>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RemoteSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_sources: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryRuntimeStatisticsRows {
    #[serde(rename = "InputBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_bytes: Option<i64>,
    #[serde(rename = "InputRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_rows: Option<i64>,
    #[serde(rename = "OutputBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_bytes: Option<i64>,
    #[serde(rename = "OutputRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_rows: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryRuntimeStatisticsTimeline {
    #[serde(rename = "EngineExecutionTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_execution_time_in_millis: Option<i64>,
    #[serde(rename = "QueryPlanningTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_planning_time_in_millis: Option<i64>,
    #[serde(rename = "QueryQueueTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_queue_time_in_millis: Option<i64>,
    #[serde(rename = "ServicePreProcessingTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_pre_processing_time_in_millis: Option<i64>,
    #[serde(rename = "ServiceProcessingTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_processing_time_in_millis: Option<i64>,
    #[serde(rename = "TotalExecutionTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_execution_time_in_millis: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceDashboardRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceDashboardResponse {
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionEndpointRequest {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionEndpointResponse {
    #[serde(rename = "AuthToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "AuthTokenExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token_expiration_time: Option<f64>,
    #[serde(rename = "EndpointUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionRequest {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionResponse {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EngineConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_configuration: Option<EngineConfiguration>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "MonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
    #[serde(rename = "NotebookVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_version: Option<String>,
    #[serde(rename = "SessionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_configuration: Option<SessionConfiguration>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "Statistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<SessionStatistics>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SessionStatus>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionConfiguration {
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "ExecutionRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role: Option<String>,
    #[serde(rename = "IdleTimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_seconds: Option<i64>,
    #[serde(rename = "SessionIdleTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_idle_timeout_in_minutes: Option<i32>,
    #[serde(rename = "WorkingDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionStatistics {
    #[serde(rename = "DpuExecutionInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpu_execution_in_millis: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionStatus {
    #[serde(rename = "EndDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<f64>,
    #[serde(rename = "IdleSinceDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_since_date_time: Option<f64>,
    #[serde(rename = "LastModifiedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<f64>,
    #[serde(rename = "StartDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateChangeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionStatusRequest {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionStatusResponse {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SessionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableMetadataInput {
    #[serde(rename = "CatalogName")]
    #[serde(default)]
    pub catalog_name: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableMetadataOutput {
    #[serde(rename = "TableMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_metadata: Option<TableMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableMetadata {
    #[serde(rename = "Columns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Column>>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "LastAccessTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<Column>>,
    #[serde(rename = "TableType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Column {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkGroupInput {
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    pub work_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkGroupOutput {
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<WorkGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkGroup {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<WorkGroupConfiguration>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IdentityCenterApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_application_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportNotebookInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NotebookS3LocationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_s3_location_uri: Option<String>,
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    pub work_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportNotebookOutput {
    #[serde(rename = "NotebookId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationDPUSizesInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationDPUSizesOutput {
    #[serde(rename = "ApplicationDPUSizes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_d_p_u_sizes: Option<Vec<ApplicationDPUSizes>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationDPUSizes {
    #[serde(rename = "ApplicationRuntimeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_runtime_id: Option<String>,
    #[serde(rename = "SupportedDPUSizes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_d_p_u_sizes: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCalculationExecutionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
    #[serde(rename = "StateFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCalculationExecutionsResponse {
    #[serde(rename = "Calculations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculations: Option<Vec<CalculationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CalculationSummary {
    #[serde(rename = "CalculationExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_execution_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CalculationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCapacityReservationsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCapacityReservationsOutput {
    #[serde(rename = "CapacityReservations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservations: Option<Vec<CapacityReservation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataCatalogsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataCatalogsOutput {
    #[serde(rename = "DataCatalogsSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalogs_summary: Option<Vec<DataCatalogSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataCatalogSummary {
    #[serde(rename = "CatalogName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatabasesInput {
    #[serde(rename = "CatalogName")]
    #[serde(default)]
    pub catalog_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatabasesOutput {
    #[serde(rename = "DatabaseList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_list: Option<Vec<Database>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEngineVersionsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEngineVersionsOutput {
    #[serde(rename = "EngineVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_versions: Option<Vec<EngineVersion>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExecutorsRequest {
    #[serde(rename = "ExecutorStateFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executor_state_filter: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExecutorsResponse {
    #[serde(rename = "ExecutorsSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executors_summary: Option<Vec<ExecutorsSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutorsSummary {
    #[serde(rename = "ExecutorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executor_id: Option<String>,
    #[serde(rename = "ExecutorSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executor_size: Option<i64>,
    #[serde(rename = "ExecutorState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executor_state: Option<String>,
    #[serde(rename = "ExecutorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executor_type: Option<String>,
    #[serde(rename = "StartDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<i64>,
    #[serde(rename = "TerminationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_date_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNamedQueriesInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNamedQueriesOutput {
    #[serde(rename = "NamedQueryIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_query_ids: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNotebookMetadataInput {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterDefinition>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    pub work_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterDefinition {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNotebookMetadataOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "NotebookMetadataList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_metadata_list: Option<Vec<NotebookMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNotebookSessionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "NotebookId")]
    #[serde(default)]
    pub notebook_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNotebookSessionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "NotebookSessionsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_sessions_list: Option<Vec<NotebookSessionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotebookSessionSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPreparedStatementsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    pub work_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPreparedStatementsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PreparedStatements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_statements: Option<Vec<PreparedStatementSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PreparedStatementSummary {
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "StatementName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQueryExecutionsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQueryExecutionsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueryExecutionIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSessionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StateFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_filter: Option<String>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    pub work_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSessionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Sessions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<SessionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionSummary {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<EngineVersion>,
    #[serde(rename = "NotebookVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_version: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SessionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTableMetadataInput {
    #[serde(rename = "CatalogName")]
    #[serde(default)]
    pub catalog_name: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTableMetadataOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TableMetadataList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_metadata_list: Option<Vec<TableMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWorkGroupsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWorkGroupsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_groups: Option<Vec<WorkGroupSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkGroupSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<EngineVersion>,
    #[serde(rename = "IdentityCenterApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_application_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutCapacityAssignmentConfigurationInput {
    #[serde(rename = "CapacityAssignments")]
    #[serde(default)]
    pub capacity_assignments: Vec<CapacityAssignment>,
    #[serde(rename = "CapacityReservationName")]
    #[serde(default)]
    pub capacity_reservation_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutCapacityAssignmentConfigurationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCalculationExecutionRequest {
    #[serde(rename = "CalculationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_configuration: Option<CalculationConfiguration>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "CodeBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_block: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CalculationConfiguration {
    #[serde(rename = "CodeBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_block: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCalculationExecutionResponse {
    #[serde(rename = "CalculationExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_execution_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartQueryExecutionInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "EngineConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_configuration: Option<EngineConfiguration>,
    #[serde(rename = "ExecutionParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_parameters: Option<Vec<String>>,
    #[serde(rename = "QueryExecutionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_context: Option<QueryExecutionContext>,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    pub query_string: String,
    #[serde(rename = "ResultConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_configuration: Option<ResultConfiguration>,
    #[serde(rename = "ResultReuseConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_reuse_configuration: Option<ResultReuseConfiguration>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartQueryExecutionOutput {
    #[serde(rename = "QueryExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSessionRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "CopyWorkGroupTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_work_group_tags: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EngineConfiguration")]
    #[serde(default)]
    pub engine_configuration: EngineConfiguration,
    #[serde(rename = "ExecutionRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role: Option<String>,
    #[serde(rename = "MonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
    #[serde(rename = "NotebookVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_version: Option<String>,
    #[serde(rename = "SessionIdleTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_idle_timeout_in_minutes: Option<i32>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    pub work_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSessionResponse {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopCalculationExecutionRequest {
    #[serde(rename = "CalculationExecutionId")]
    #[serde(default)]
    pub calculation_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopCalculationExecutionResponse {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopQueryExecutionInput {
    #[serde(rename = "QueryExecutionId")]
    #[serde(default)]
    pub query_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopQueryExecutionOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateSessionRequest {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateSessionResponse {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCapacityReservationInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "TargetDpus")]
    #[serde(default)]
    pub target_dpus: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCapacityReservationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataCatalogInput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataCatalogOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNamedQueryInput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NamedQueryId")]
    #[serde(default)]
    pub named_query_id: String,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    pub query_string: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNamedQueryOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNotebookInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "NotebookId")]
    #[serde(default)]
    pub notebook_id: String,
    #[serde(rename = "Payload")]
    #[serde(default)]
    pub payload: String,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNotebookMetadataInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NotebookId")]
    #[serde(default)]
    pub notebook_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNotebookMetadataOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNotebookOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePreparedStatementInput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "QueryStatement")]
    #[serde(default)]
    pub query_statement: String,
    #[serde(rename = "StatementName")]
    #[serde(default)]
    pub statement_name: String,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    pub work_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePreparedStatementOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkGroupInput {
    #[serde(rename = "ConfigurationUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_updates: Option<WorkGroupConfigurationUpdates>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    pub work_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkGroupConfigurationUpdates {
    #[serde(rename = "AdditionalConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_configuration: Option<String>,
    #[serde(rename = "BytesScannedCutoffPerQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_scanned_cutoff_per_query: Option<i64>,
    #[serde(rename = "CustomerContentEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_content_encryption_configuration: Option<CustomerContentEncryptionConfiguration>,
    #[serde(rename = "EnableMinimumEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_minimum_encryption_configuration: Option<bool>,
    #[serde(rename = "EnforceWorkGroupConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_work_group_configuration: Option<bool>,
    #[serde(rename = "EngineConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_configuration: Option<EngineConfiguration>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<EngineVersion>,
    #[serde(rename = "ExecutionRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role: Option<String>,
    #[serde(rename = "ManagedQueryResultsConfigurationUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_query_results_configuration_updates:
        Option<ManagedQueryResultsConfigurationUpdates>,
    #[serde(rename = "MonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
    #[serde(rename = "PublishCloudWatchMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_cloud_watch_metrics_enabled: Option<bool>,
    #[serde(rename = "QueryResultsS3AccessGrantsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_results_s3_access_grants_configuration:
        Option<QueryResultsS3AccessGrantsConfiguration>,
    #[serde(rename = "RemoveBytesScannedCutoffPerQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_bytes_scanned_cutoff_per_query: Option<bool>,
    #[serde(rename = "RemoveCustomerContentEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_customer_content_encryption_configuration: Option<bool>,
    #[serde(rename = "RequesterPaysEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_pays_enabled: Option<bool>,
    #[serde(rename = "ResultConfigurationUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_configuration_updates: Option<ResultConfigurationUpdates>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedQueryResultsConfigurationUpdates {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<ManagedQueryResultsEncryptionConfiguration>,
    #[serde(rename = "RemoveEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_encryption_configuration: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultConfigurationUpdates {
    #[serde(rename = "AclConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_configuration: Option<AclConfiguration>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "OutputLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<String>,
    #[serde(rename = "RemoveAclConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_acl_configuration: Option<bool>,
    #[serde(rename = "RemoveEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_encryption_configuration: Option<bool>,
    #[serde(rename = "RemoveExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_expected_bucket_owner: Option<bool>,
    #[serde(rename = "RemoveOutputLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_output_location: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkGroupOutput {}
