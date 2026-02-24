//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-lakeformation

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
pub fn serialize_add_l_f_tags_to_resource_response(
    result: &AddLFTagsToResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_assume_decorated_role_with_s_a_m_l_response(
    result: &AssumeDecoratedRoleWithSAMLResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_grant_permissions_response(
    result: &BatchGrantPermissionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_revoke_permissions_response(
    result: &BatchRevokePermissionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_transaction_response(result: &CancelTransactionResponse) -> MockResponse {
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
pub fn serialize_create_data_cells_filter_response(
    result: &CreateDataCellsFilterResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_l_f_tag_response(result: &CreateLFTagResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_l_f_tag_expression_response(
    result: &CreateLFTagExpressionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_lake_formation_identity_center_configuration_response(
    result: &CreateLakeFormationIdentityCenterConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_lake_formation_opt_in_response(
    result: &CreateLakeFormationOptInResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_data_cells_filter_response(
    result: &DeleteDataCellsFilterResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_l_f_tag_response(result: &DeleteLFTagResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_l_f_tag_expression_response(
    result: &DeleteLFTagExpressionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_lake_formation_identity_center_configuration_response(
    result: &DeleteLakeFormationIdentityCenterConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_lake_formation_opt_in_response(
    result: &DeleteLakeFormationOptInResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_objects_on_cancel_response(
    result: &DeleteObjectsOnCancelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_deregister_resource_response(result: &DeregisterResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_lake_formation_identity_center_configuration_response(
    result: &DescribeLakeFormationIdentityCenterConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_resource_response(result: &DescribeResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_transaction_response(
    result: &DescribeTransactionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_extend_transaction_response(result: &ExtendTransactionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_data_cells_filter_response(
    result: &GetDataCellsFilterResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_data_lake_principal_response(
    result: &GetDataLakePrincipalResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_data_lake_settings_response(
    result: &GetDataLakeSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_effective_permissions_for_path_response(
    result: &GetEffectivePermissionsForPathResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_l_f_tag_response(result: &GetLFTagResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_l_f_tag_expression_response(
    result: &GetLFTagExpressionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_query_state_response(result: &GetQueryStateResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_query_statistics_response(
    result: &GetQueryStatisticsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_l_f_tags_response(
    result: &GetResourceLFTagsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_table_objects_response(result: &GetTableObjectsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_temporary_data_location_credentials_response(
    result: &GetTemporaryDataLocationCredentialsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_temporary_glue_partition_credentials_response(
    result: &GetTemporaryGluePartitionCredentialsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_temporary_glue_table_credentials_response(
    result: &GetTemporaryGlueTableCredentialsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_work_unit_results_response(
    result: &GetWorkUnitResultsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_work_units_response(result: &GetWorkUnitsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_grant_permissions_response(result: &GrantPermissionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_data_cells_filter_response(
    result: &ListDataCellsFilterResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_l_f_tag_expressions_response(
    result: &ListLFTagExpressionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_l_f_tags_response(result: &ListLFTagsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_lake_formation_opt_ins_response(
    result: &ListLakeFormationOptInsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_permissions_response(result: &ListPermissionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resources_response(result: &ListResourcesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_table_storage_optimizers_response(
    result: &ListTableStorageOptimizersResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_transactions_response(result: &ListTransactionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_data_lake_settings_response(
    result: &PutDataLakeSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_register_resource_response(result: &RegisterResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_l_f_tags_from_resource_response(
    result: &RemoveLFTagsFromResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_revoke_permissions_response(result: &RevokePermissionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_databases_by_l_f_tags_response(
    result: &SearchDatabasesByLFTagsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_tables_by_l_f_tags_response(
    result: &SearchTablesByLFTagsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_query_planning_response(
    result: &StartQueryPlanningResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_transaction_response(result: &StartTransactionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_data_cells_filter_response(
    result: &UpdateDataCellsFilterResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_l_f_tag_response(result: &UpdateLFTagResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_l_f_tag_expression_response(
    result: &UpdateLFTagExpressionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_lake_formation_identity_center_configuration_response(
    result: &UpdateLakeFormationIdentityCenterConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_resource_response(result: &UpdateResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_table_objects_response(
    result: &UpdateTableObjectsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_table_storage_optimizer_response(
    result: &UpdateTableStorageOptimizerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_l_f_tags_to_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddLFTagsToResourceRequest, String> {
    let mut input = AddLFTagsToResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddLFTagsToResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AddLFTagsToResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_assume_decorated_role_with_s_a_m_l_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssumeDecoratedRoleWithSAMLRequest, String> {
    let mut input = AssumeDecoratedRoleWithSAMLRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssumeDecoratedRoleWithSAMLRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssumeDecoratedRoleWithSAML request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_grant_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGrantPermissionsRequest, String> {
    let mut input = BatchGrantPermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGrantPermissionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchGrantPermissions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_revoke_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchRevokePermissionsRequest, String> {
    let mut input = BatchRevokePermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchRevokePermissionsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize BatchRevokePermissions request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_transaction_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelTransactionRequest, String> {
    let mut input = CancelTransactionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CancelTransactionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CancelTransaction request: {err}"))?;
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
pub fn deserialize_create_data_cells_filter_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDataCellsFilterRequest, String> {
    let mut input = CreateDataCellsFilterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDataCellsFilterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDataCellsFilter request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_l_f_tag_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateLFTagRequest, String> {
    let mut input = CreateLFTagRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateLFTagRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateLFTag request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_l_f_tag_expression_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateLFTagExpressionRequest, String> {
    let mut input = CreateLFTagExpressionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateLFTagExpressionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateLFTagExpression request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_lake_formation_identity_center_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateLakeFormationIdentityCenterConfigurationRequest, String> {
    let mut input = CreateLakeFormationIdentityCenterConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateLakeFormationIdentityCenterConfigurationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateLakeFormationIdentityCenterConfiguration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_lake_formation_opt_in_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateLakeFormationOptInRequest, String> {
    let mut input = CreateLakeFormationOptInRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateLakeFormationOptInRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateLakeFormationOptIn request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_data_cells_filter_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDataCellsFilterRequest, String> {
    let mut input = DeleteDataCellsFilterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteDataCellsFilterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteDataCellsFilter request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_l_f_tag_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteLFTagRequest, String> {
    let mut input = DeleteLFTagRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteLFTagRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteLFTag request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_l_f_tag_expression_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteLFTagExpressionRequest, String> {
    let mut input = DeleteLFTagExpressionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteLFTagExpressionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteLFTagExpression request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_lake_formation_identity_center_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteLakeFormationIdentityCenterConfigurationRequest, String> {
    let mut input = DeleteLakeFormationIdentityCenterConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteLakeFormationIdentityCenterConfigurationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteLakeFormationIdentityCenterConfiguration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_lake_formation_opt_in_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteLakeFormationOptInRequest, String> {
    let mut input = DeleteLakeFormationOptInRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteLakeFormationOptInRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DeleteLakeFormationOptIn request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_objects_on_cancel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteObjectsOnCancelRequest, String> {
    let mut input = DeleteObjectsOnCancelRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteObjectsOnCancelRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteObjectsOnCancel request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_deregister_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeregisterResourceRequest, String> {
    let mut input = DeregisterResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeregisterResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeregisterResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_lake_formation_identity_center_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeLakeFormationIdentityCenterConfigurationRequest, String> {
    let mut input = DescribeLakeFormationIdentityCenterConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeLakeFormationIdentityCenterConfigurationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeLakeFormationIdentityCenterConfiguration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeResourceRequest, String> {
    let mut input = DescribeResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_transaction_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeTransactionRequest, String> {
    let mut input = DescribeTransactionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeTransactionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeTransaction request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_extend_transaction_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ExtendTransactionRequest, String> {
    let mut input = ExtendTransactionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ExtendTransactionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ExtendTransaction request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_data_cells_filter_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDataCellsFilterRequest, String> {
    let mut input = GetDataCellsFilterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetDataCellsFilterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetDataCellsFilter request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_data_lake_principal_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDataLakePrincipalRequest, String> {
    let input = GetDataLakePrincipalRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_data_lake_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDataLakeSettingsRequest, String> {
    let mut input = GetDataLakeSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetDataLakeSettingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetDataLakeSettings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_effective_permissions_for_path_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEffectivePermissionsForPathRequest, String> {
    let mut input = GetEffectivePermissionsForPathRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetEffectivePermissionsForPathRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize GetEffectivePermissionsForPath request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_l_f_tag_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetLFTagRequest, String> {
    let mut input = GetLFTagRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetLFTagRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetLFTag request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_l_f_tag_expression_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetLFTagExpressionRequest, String> {
    let mut input = GetLFTagExpressionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetLFTagExpressionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetLFTagExpression request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_query_state_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetQueryStateRequest, String> {
    let mut input = GetQueryStateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetQueryStateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetQueryState request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_query_statistics_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetQueryStatisticsRequest, String> {
    let mut input = GetQueryStatisticsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetQueryStatisticsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetQueryStatistics request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_l_f_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourceLFTagsRequest, String> {
    let mut input = GetResourceLFTagsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetResourceLFTagsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetResourceLFTags request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_table_objects_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTableObjectsRequest, String> {
    let mut input = GetTableObjectsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetTableObjectsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetTableObjects request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_temporary_data_location_credentials_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTemporaryDataLocationCredentialsRequest, String> {
    let mut input = GetTemporaryDataLocationCredentialsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetTemporaryDataLocationCredentialsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize GetTemporaryDataLocationCredentials request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_temporary_glue_partition_credentials_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTemporaryGluePartitionCredentialsRequest, String> {
    let mut input = GetTemporaryGluePartitionCredentialsRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<GetTemporaryGluePartitionCredentialsRequest>(&request.body)
                .map_err(|err| {
                    format!(
                        "failed to deserialize GetTemporaryGluePartitionCredentials request: {err}"
                    )
                })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_temporary_glue_table_credentials_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTemporaryGlueTableCredentialsRequest, String> {
    let mut input = GetTemporaryGlueTableCredentialsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetTemporaryGlueTableCredentialsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize GetTemporaryGlueTableCredentials request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_work_unit_results_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetWorkUnitResultsRequest, String> {
    let mut input = GetWorkUnitResultsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetWorkUnitResultsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetWorkUnitResults request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_work_units_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetWorkUnitsRequest, String> {
    let mut input = GetWorkUnitsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetWorkUnitsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetWorkUnits request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_grant_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GrantPermissionsRequest, String> {
    let mut input = GrantPermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GrantPermissionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GrantPermissions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_data_cells_filter_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDataCellsFilterRequest, String> {
    let mut input = ListDataCellsFilterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListDataCellsFilterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListDataCellsFilter request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_l_f_tag_expressions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListLFTagExpressionsRequest, String> {
    let mut input = ListLFTagExpressionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListLFTagExpressionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListLFTagExpressions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_l_f_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListLFTagsRequest, String> {
    let mut input = ListLFTagsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListLFTagsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListLFTags request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_lake_formation_opt_ins_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListLakeFormationOptInsRequest, String> {
    let mut input = ListLakeFormationOptInsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListLakeFormationOptInsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListLakeFormationOptIns request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPermissionsRequest, String> {
    let mut input = ListPermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListPermissionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListPermissions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourcesRequest, String> {
    let mut input = ListResourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListResourcesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListResources request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_table_storage_optimizers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTableStorageOptimizersRequest, String> {
    let mut input = ListTableStorageOptimizersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTableStorageOptimizersRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListTableStorageOptimizers request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_transactions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTransactionsRequest, String> {
    let mut input = ListTransactionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTransactionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTransactions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_data_lake_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutDataLakeSettingsRequest, String> {
    let mut input = PutDataLakeSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutDataLakeSettingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutDataLakeSettings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_register_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterResourceRequest, String> {
    let mut input = RegisterResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RegisterResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_l_f_tags_from_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveLFTagsFromResourceRequest, String> {
    let mut input = RemoveLFTagsFromResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RemoveLFTagsFromResourceRequest>(&request.body).map_err(
            |err| format!("failed to deserialize RemoveLFTagsFromResource request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_revoke_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RevokePermissionsRequest, String> {
    let mut input = RevokePermissionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RevokePermissionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RevokePermissions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_databases_by_l_f_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchDatabasesByLFTagsRequest, String> {
    let mut input = SearchDatabasesByLFTagsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchDatabasesByLFTagsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize SearchDatabasesByLFTags request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_tables_by_l_f_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchTablesByLFTagsRequest, String> {
    let mut input = SearchTablesByLFTagsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchTablesByLFTagsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchTablesByLFTags request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_query_planning_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartQueryPlanningRequest, String> {
    let mut input = StartQueryPlanningRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartQueryPlanningRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartQueryPlanning request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_transaction_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartTransactionRequest, String> {
    let mut input = StartTransactionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartTransactionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartTransaction request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_data_cells_filter_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDataCellsFilterRequest, String> {
    let mut input = UpdateDataCellsFilterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDataCellsFilterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDataCellsFilter request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_l_f_tag_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateLFTagRequest, String> {
    let mut input = UpdateLFTagRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateLFTagRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateLFTag request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_l_f_tag_expression_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateLFTagExpressionRequest, String> {
    let mut input = UpdateLFTagExpressionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateLFTagExpressionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateLFTagExpression request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_lake_formation_identity_center_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateLakeFormationIdentityCenterConfigurationRequest, String> {
    let mut input = UpdateLakeFormationIdentityCenterConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateLakeFormationIdentityCenterConfigurationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateLakeFormationIdentityCenterConfiguration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateResourceRequest, String> {
    let mut input = UpdateResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_table_objects_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTableObjectsRequest, String> {
    let mut input = UpdateTableObjectsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTableObjectsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateTableObjects request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_table_storage_optimizer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTableStorageOptimizerRequest, String> {
    let mut input = UpdateTableStorageOptimizerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTableStorageOptimizerRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateTableStorageOptimizer request: {err}")
            })?;
    }
    Ok(input)
}
