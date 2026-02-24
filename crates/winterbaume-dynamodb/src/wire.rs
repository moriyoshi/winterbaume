//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-dynamodb

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
pub fn serialize_batch_get_item_response(result: &BatchGetItemOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_write_item_response(result: &BatchWriteItemOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_backup_response(result: &CreateBackupOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_global_table_response(result: &CreateGlobalTableOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_table_response(result: &CreateTableOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_backup_response(result: &DeleteBackupOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_item_response(result: &DeleteItemOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_resource_policy_response(
    result: &DeleteResourcePolicyOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_table_response(result: &DeleteTableOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_backup_response(result: &DescribeBackupOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_continuous_backups_response(
    result: &DescribeContinuousBackupsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_contributor_insights_response(
    result: &DescribeContributorInsightsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_endpoints_response(result: &DescribeEndpointsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_export_response(result: &DescribeExportOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_global_table_response(
    result: &DescribeGlobalTableOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_global_table_settings_response(
    result: &DescribeGlobalTableSettingsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_import_response(result: &DescribeImportOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_kinesis_streaming_destination_response(
    result: &DescribeKinesisStreamingDestinationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_limits_response(result: &DescribeLimitsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_table_response(result: &DescribeTableOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_table_replica_auto_scaling_response(
    result: &DescribeTableReplicaAutoScalingOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_time_to_live_response(result: &DescribeTimeToLiveOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_kinesis_streaming_destination_response(
    result: &KinesisStreamingDestinationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_kinesis_streaming_destination_response(
    result: &KinesisStreamingDestinationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_execute_statement_response(result: &ExecuteStatementOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_execute_transaction_response(result: &ExecuteTransactionOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_export_table_to_point_in_time_response(
    result: &ExportTableToPointInTimeOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_item_response(result: &GetItemOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_policy_response(result: &GetResourcePolicyOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_table_response(result: &ImportTableOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_backups_response(result: &ListBackupsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_contributor_insights_response(
    result: &ListContributorInsightsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_exports_response(result: &ListExportsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_global_tables_response(result: &ListGlobalTablesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_imports_response(result: &ListImportsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tables_response(result: &ListTablesOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_of_resource_response(result: &ListTagsOfResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_item_response(result: &PutItemOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_query_response(result: &QueryOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_restore_table_from_backup_response(
    result: &RestoreTableFromBackupOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_restore_table_to_point_in_time_response(
    result: &RestoreTableToPointInTimeOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_scan_response(result: &ScanOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_transact_get_items_response(result: &TransactGetItemsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_transact_write_items_response(result: &TransactWriteItemsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_continuous_backups_response(
    result: &UpdateContinuousBackupsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_contributor_insights_response(
    result: &UpdateContributorInsightsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_global_table_response(result: &UpdateGlobalTableOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_global_table_settings_response(
    result: &UpdateGlobalTableSettingsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_item_response(result: &UpdateItemOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_kinesis_streaming_destination_response(
    result: &UpdateKinesisStreamingDestinationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_table_response(result: &UpdateTableOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_table_replica_auto_scaling_response(
    result: &UpdateTableReplicaAutoScalingOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_time_to_live_response(result: &UpdateTimeToLiveOutput) -> MockResponse {
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
pub fn deserialize_batch_get_item_request(body: &[u8]) -> Result<BatchGetItemInput, String> {
    if body.is_empty() {
        return Ok(BatchGetItemInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchGetItem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_write_item_request(body: &[u8]) -> Result<BatchWriteItemInput, String> {
    if body.is_empty() {
        return Ok(BatchWriteItemInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchWriteItem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_backup_request(body: &[u8]) -> Result<CreateBackupInput, String> {
    if body.is_empty() {
        return Ok(CreateBackupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateBackup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_global_table_request(
    body: &[u8],
) -> Result<CreateGlobalTableInput, String> {
    if body.is_empty() {
        return Ok(CreateGlobalTableInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateGlobalTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_table_request(body: &[u8]) -> Result<CreateTableInput, String> {
    if body.is_empty() {
        return Ok(CreateTableInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_backup_request(body: &[u8]) -> Result<DeleteBackupInput, String> {
    if body.is_empty() {
        return Ok(DeleteBackupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteBackup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_item_request(body: &[u8]) -> Result<DeleteItemInput, String> {
    if body.is_empty() {
        return Ok(DeleteItemInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteItem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resource_policy_request(
    body: &[u8],
) -> Result<DeleteResourcePolicyInput, String> {
    if body.is_empty() {
        return Ok(DeleteResourcePolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_table_request(body: &[u8]) -> Result<DeleteTableInput, String> {
    if body.is_empty() {
        return Ok(DeleteTableInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_backup_request(body: &[u8]) -> Result<DescribeBackupInput, String> {
    if body.is_empty() {
        return Ok(DescribeBackupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeBackup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_continuous_backups_request(
    body: &[u8],
) -> Result<DescribeContinuousBackupsInput, String> {
    if body.is_empty() {
        return Ok(DescribeContinuousBackupsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeContinuousBackups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_contributor_insights_request(
    body: &[u8],
) -> Result<DescribeContributorInsightsInput, String> {
    if body.is_empty() {
        return Ok(DescribeContributorInsightsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeContributorInsights request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_endpoints_request(
    body: &[u8],
) -> Result<DescribeEndpointsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEndpointsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEndpoints request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_export_request(body: &[u8]) -> Result<DescribeExportInput, String> {
    if body.is_empty() {
        return Ok(DescribeExportInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeExport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_global_table_request(
    body: &[u8],
) -> Result<DescribeGlobalTableInput, String> {
    if body.is_empty() {
        return Ok(DescribeGlobalTableInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeGlobalTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_global_table_settings_request(
    body: &[u8],
) -> Result<DescribeGlobalTableSettingsInput, String> {
    if body.is_empty() {
        return Ok(DescribeGlobalTableSettingsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeGlobalTableSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_import_request(body: &[u8]) -> Result<DescribeImportInput, String> {
    if body.is_empty() {
        return Ok(DescribeImportInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeImport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_kinesis_streaming_destination_request(
    body: &[u8],
) -> Result<DescribeKinesisStreamingDestinationInput, String> {
    if body.is_empty() {
        return Ok(DescribeKinesisStreamingDestinationInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeKinesisStreamingDestination request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_limits_request(body: &[u8]) -> Result<DescribeLimitsInput, String> {
    if body.is_empty() {
        return Ok(DescribeLimitsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLimits request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_table_request(body: &[u8]) -> Result<DescribeTableInput, String> {
    if body.is_empty() {
        return Ok(DescribeTableInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_table_replica_auto_scaling_request(
    body: &[u8],
) -> Result<DescribeTableReplicaAutoScalingInput, String> {
    if body.is_empty() {
        return Ok(DescribeTableReplicaAutoScalingInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTableReplicaAutoScaling request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_time_to_live_request(
    body: &[u8],
) -> Result<DescribeTimeToLiveInput, String> {
    if body.is_empty() {
        return Ok(DescribeTimeToLiveInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTimeToLive request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_kinesis_streaming_destination_request(
    body: &[u8],
) -> Result<KinesisStreamingDestinationInput, String> {
    if body.is_empty() {
        return Ok(KinesisStreamingDestinationInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DisableKinesisStreamingDestination request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_kinesis_streaming_destination_request(
    body: &[u8],
) -> Result<KinesisStreamingDestinationInput, String> {
    if body.is_empty() {
        return Ok(KinesisStreamingDestinationInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize EnableKinesisStreamingDestination request: {e}")
    })
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
pub fn deserialize_execute_transaction_request(
    body: &[u8],
) -> Result<ExecuteTransactionInput, String> {
    if body.is_empty() {
        return Ok(ExecuteTransactionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ExecuteTransaction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_export_table_to_point_in_time_request(
    body: &[u8],
) -> Result<ExportTableToPointInTimeInput, String> {
    if body.is_empty() {
        return Ok(ExportTableToPointInTimeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ExportTableToPointInTime request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_item_request(body: &[u8]) -> Result<GetItemInput, String> {
    if body.is_empty() {
        return Ok(GetItemInput::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetItem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_policy_request(
    body: &[u8],
) -> Result<GetResourcePolicyInput, String> {
    if body.is_empty() {
        return Ok(GetResourcePolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_table_request(body: &[u8]) -> Result<ImportTableInput, String> {
    if body.is_empty() {
        return Ok(ImportTableInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_backups_request(body: &[u8]) -> Result<ListBackupsInput, String> {
    if body.is_empty() {
        return Ok(ListBackupsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListBackups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_contributor_insights_request(
    body: &[u8],
) -> Result<ListContributorInsightsInput, String> {
    if body.is_empty() {
        return Ok(ListContributorInsightsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListContributorInsights request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_exports_request(body: &[u8]) -> Result<ListExportsInput, String> {
    if body.is_empty() {
        return Ok(ListExportsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListExports request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_global_tables_request(
    body: &[u8],
) -> Result<ListGlobalTablesInput, String> {
    if body.is_empty() {
        return Ok(ListGlobalTablesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListGlobalTables request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_imports_request(body: &[u8]) -> Result<ListImportsInput, String> {
    if body.is_empty() {
        return Ok(ListImportsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListImports request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tables_request(body: &[u8]) -> Result<ListTablesInput, String> {
    if body.is_empty() {
        return Ok(ListTablesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTables request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_of_resource_request(
    body: &[u8],
) -> Result<ListTagsOfResourceInput, String> {
    if body.is_empty() {
        return Ok(ListTagsOfResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsOfResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_item_request(body: &[u8]) -> Result<PutItemInput, String> {
    if body.is_empty() {
        return Ok(PutItemInput::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize PutItem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_resource_policy_request(
    body: &[u8],
) -> Result<PutResourcePolicyInput, String> {
    if body.is_empty() {
        return Ok(PutResourcePolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_query_request(body: &[u8]) -> Result<QueryInput, String> {
    if body.is_empty() {
        return Ok(QueryInput::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize Query request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_restore_table_from_backup_request(
    body: &[u8],
) -> Result<RestoreTableFromBackupInput, String> {
    if body.is_empty() {
        return Ok(RestoreTableFromBackupInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RestoreTableFromBackup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_restore_table_to_point_in_time_request(
    body: &[u8],
) -> Result<RestoreTableToPointInTimeInput, String> {
    if body.is_empty() {
        return Ok(RestoreTableToPointInTimeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RestoreTableToPointInTime request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_scan_request(body: &[u8]) -> Result<ScanInput, String> {
    if body.is_empty() {
        return Ok(ScanInput::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize Scan request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceInput, String> {
    if body.is_empty() {
        return Ok(TagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_transact_get_items_request(
    body: &[u8],
) -> Result<TransactGetItemsInput, String> {
    if body.is_empty() {
        return Ok(TransactGetItemsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TransactGetItems request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_transact_write_items_request(
    body: &[u8],
) -> Result<TransactWriteItemsInput, String> {
    if body.is_empty() {
        return Ok(TransactWriteItemsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TransactWriteItems request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceInput, String> {
    if body.is_empty() {
        return Ok(UntagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_continuous_backups_request(
    body: &[u8],
) -> Result<UpdateContinuousBackupsInput, String> {
    if body.is_empty() {
        return Ok(UpdateContinuousBackupsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateContinuousBackups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_contributor_insights_request(
    body: &[u8],
) -> Result<UpdateContributorInsightsInput, String> {
    if body.is_empty() {
        return Ok(UpdateContributorInsightsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateContributorInsights request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_global_table_request(
    body: &[u8],
) -> Result<UpdateGlobalTableInput, String> {
    if body.is_empty() {
        return Ok(UpdateGlobalTableInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateGlobalTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_global_table_settings_request(
    body: &[u8],
) -> Result<UpdateGlobalTableSettingsInput, String> {
    if body.is_empty() {
        return Ok(UpdateGlobalTableSettingsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateGlobalTableSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_item_request(body: &[u8]) -> Result<UpdateItemInput, String> {
    if body.is_empty() {
        return Ok(UpdateItemInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateItem request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_kinesis_streaming_destination_request(
    body: &[u8],
) -> Result<UpdateKinesisStreamingDestinationInput, String> {
    if body.is_empty() {
        return Ok(UpdateKinesisStreamingDestinationInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateKinesisStreamingDestination request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_table_request(body: &[u8]) -> Result<UpdateTableInput, String> {
    if body.is_empty() {
        return Ok(UpdateTableInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTable request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_table_replica_auto_scaling_request(
    body: &[u8],
) -> Result<UpdateTableReplicaAutoScalingInput, String> {
    if body.is_empty() {
        return Ok(UpdateTableReplicaAutoScalingInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTableReplicaAutoScaling request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_time_to_live_request(
    body: &[u8],
) -> Result<UpdateTimeToLiveInput, String> {
    if body.is_empty() {
        return Ok(UpdateTimeToLiveInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTimeToLive request: {e}"))
}
