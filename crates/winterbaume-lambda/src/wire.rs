//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-lambda

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
pub fn serialize_add_layer_version_permission_response(
    result: &AddLayerVersionPermissionResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_add_permission_response(result: &AddPermissionResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_checkpoint_durable_execution_response(
    result: &CheckpointDurableExecutionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_alias_response(result: &AliasConfiguration) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_capacity_provider_response(
    result: &CreateCapacityProviderResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_code_signing_config_response(
    result: &CreateCodeSigningConfigResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_event_source_mapping_response(
    result: &EventSourceMappingConfiguration,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_function_response(result: &FunctionConfiguration) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_function_url_config_response(
    result: &CreateFunctionUrlConfigResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_alias_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_capacity_provider_response(
    result: &DeleteCapacityProviderResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_code_signing_config_response(
    result: &DeleteCodeSigningConfigResponse,
) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_event_source_mapping_response(
    result: &EventSourceMappingConfiguration,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_function_response(result: &DeleteFunctionResponse) -> MockResponse {
    let status = result.status_code.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_function_code_signing_config_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_function_concurrency_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_function_event_invoke_config_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_function_url_config_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_layer_version_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_provisioned_concurrency_config_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_get_account_settings_response(
    result: &GetAccountSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_alias_response(result: &AliasConfiguration) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_capacity_provider_response(
    result: &GetCapacityProviderResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_code_signing_config_response(
    result: &GetCodeSigningConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_durable_execution_response(
    result: &GetDurableExecutionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_durable_execution_history_response(
    result: &GetDurableExecutionHistoryResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_durable_execution_state_response(
    result: &GetDurableExecutionStateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_event_source_mapping_response(
    result: &EventSourceMappingConfiguration,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_function_response(result: &GetFunctionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_function_code_signing_config_response(
    result: &GetFunctionCodeSigningConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_function_concurrency_response(
    result: &GetFunctionConcurrencyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_function_configuration_response(
    result: &FunctionConfiguration,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_function_event_invoke_config_response(
    result: &FunctionEventInvokeConfig,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_function_recursion_config_response(
    result: &GetFunctionRecursionConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_function_scaling_config_response(
    result: &GetFunctionScalingConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_function_url_config_response(
    result: &GetFunctionUrlConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_layer_version_response(result: &GetLayerVersionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_layer_version_by_arn_response(
    result: &GetLayerVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_layer_version_policy_response(
    result: &GetLayerVersionPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_policy_response(result: &GetPolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_provisioned_concurrency_config_response(
    result: &GetProvisionedConcurrencyConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_runtime_management_config_response(
    result: &GetRuntimeManagementConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_invoke_response(result: &InvocationResponse) -> MockResponse {
    let status = result.status_code.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "x-amz-durable-execution-arn": set by caller from durable_execution_arn field
    // Header "x-amz-executed-version": set by caller from executed_version field
    // Header "x-amz-function-error": set by caller from function_error field
    // Header "x-amz-log-result": set by caller from log_result field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_invoke_async_response(result: &InvokeAsyncResponse) -> MockResponse {
    let status = result.status.unwrap_or(202) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_invoke_with_response_stream_response(
    result: &InvokeWithResponseStreamResponse,
) -> MockResponse {
    let status = result.status_code.unwrap_or(200) as u16;
    let body = match &result.event_stream {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    let resp = MockResponse::rest_json(status, body);
    // Header "x-amz-executed-version": set by caller from executed_version field
    // Header "content-type": set by caller from response_stream_content_type field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_list_aliases_response(result: &ListAliasesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_capacity_providers_response(
    result: &ListCapacityProvidersResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_code_signing_configs_response(
    result: &ListCodeSigningConfigsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_durable_executions_by_function_response(
    result: &ListDurableExecutionsByFunctionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_event_source_mappings_response(
    result: &ListEventSourceMappingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_function_event_invoke_configs_response(
    result: &ListFunctionEventInvokeConfigsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_function_url_configs_response(
    result: &ListFunctionUrlConfigsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_function_versions_by_capacity_provider_response(
    result: &ListFunctionVersionsByCapacityProviderResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_functions_response(result: &ListFunctionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_functions_by_code_signing_config_response(
    result: &ListFunctionsByCodeSigningConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_layer_versions_response(result: &ListLayerVersionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_layers_response(result: &ListLayersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_provisioned_concurrency_configs_response(
    result: &ListProvisionedConcurrencyConfigsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_response(result: &ListTagsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_versions_by_function_response(
    result: &ListVersionsByFunctionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_publish_layer_version_response(
    result: &PublishLayerVersionResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_publish_version_response(result: &FunctionConfiguration) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_function_code_signing_config_response(
    result: &PutFunctionCodeSigningConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_function_concurrency_response(result: &Concurrency) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_function_event_invoke_config_response(
    result: &FunctionEventInvokeConfig,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_function_recursion_config_response(
    result: &PutFunctionRecursionConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_function_scaling_config_response(
    result: &PutFunctionScalingConfigResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_provisioned_concurrency_config_response(
    result: &PutProvisionedConcurrencyConfigResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_runtime_management_config_response(
    result: &PutRuntimeManagementConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_remove_layer_version_permission_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_remove_permission_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_send_durable_execution_callback_failure_response(
    result: &SendDurableExecutionCallbackFailureResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_send_durable_execution_callback_heartbeat_response(
    result: &SendDurableExecutionCallbackHeartbeatResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_send_durable_execution_callback_success_response(
    result: &SendDurableExecutionCallbackSuccessResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_durable_execution_response(
    result: &StopDurableExecutionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_alias_response(result: &AliasConfiguration) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_capacity_provider_response(
    result: &UpdateCapacityProviderResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_code_signing_config_response(
    result: &UpdateCodeSigningConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_event_source_mapping_response(
    result: &EventSourceMappingConfiguration,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_function_code_response(result: &FunctionConfiguration) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_function_configuration_response(
    result: &FunctionConfiguration,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_function_event_invoke_config_response(
    result: &FunctionEventInvokeConfig,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_function_url_config_response(
    result: &UpdateFunctionUrlConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_layer_version_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddLayerVersionPermissionRequest, String> {
    let mut input = AddLayerVersionPermissionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddLayerVersionPermissionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AddLayerVersionPermission request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "LayerName" => {
                input.layer_name = value.to_string();
            }
            "VersionNumber" => {
                input.version_number = value
                    .parse::<i64>()
                    .map_err(|err| format!("failed to parse long: {err}"))?;
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("RevisionId") {
        input.revision_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddPermissionRequest, String> {
    let mut input = AddPermissionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddPermissionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AddPermission request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_checkpoint_durable_execution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CheckpointDurableExecutionRequest, String> {
    let mut input = CheckpointDurableExecutionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CheckpointDurableExecutionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CheckpointDurableExecution request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "DurableExecutionArn" => {
                input.durable_execution_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAliasRequest, String> {
    let mut input = CreateAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_capacity_provider_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCapacityProviderRequest, String> {
    let mut input = CreateCapacityProviderRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCapacityProviderRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateCapacityProvider request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_code_signing_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCodeSigningConfigRequest, String> {
    let mut input = CreateCodeSigningConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCodeSigningConfigRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateCodeSigningConfig request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_event_source_mapping_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateEventSourceMappingRequest, String> {
    let mut input = CreateEventSourceMappingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateEventSourceMappingRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateEventSourceMapping request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFunctionRequest, String> {
    let mut input = CreateFunctionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFunctionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFunction request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_function_url_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFunctionUrlConfigRequest, String> {
    let mut input = CreateFunctionUrlConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFunctionUrlConfigRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateFunctionUrlConfig request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAliasRequest, String> {
    let mut input = DeleteAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_capacity_provider_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCapacityProviderRequest, String> {
    let mut input = DeleteCapacityProviderRequest::default();
    for (name, value) in labels {
        match *name {
            "CapacityProviderName" => {
                input.capacity_provider_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_code_signing_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCodeSigningConfigRequest, String> {
    let mut input = DeleteCodeSigningConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "CodeSigningConfigArn" => {
                input.code_signing_config_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_event_source_mapping_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteEventSourceMappingRequest, String> {
    let mut input = DeleteEventSourceMappingRequest::default();
    for (name, value) in labels {
        match *name {
            "UUID" => {
                input.u_u_i_d = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFunctionRequest, String> {
    let mut input = DeleteFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_function_code_signing_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFunctionCodeSigningConfigRequest, String> {
    let mut input = DeleteFunctionCodeSigningConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_function_concurrency_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFunctionConcurrencyRequest, String> {
    let mut input = DeleteFunctionConcurrencyRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_function_event_invoke_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFunctionEventInvokeConfigRequest, String> {
    let mut input = DeleteFunctionEventInvokeConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_function_url_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFunctionUrlConfigRequest, String> {
    let mut input = DeleteFunctionUrlConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_layer_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteLayerVersionRequest, String> {
    let mut input = DeleteLayerVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "LayerName" => {
                input.layer_name = value.to_string();
            }
            "VersionNumber" => {
                input.version_number = value
                    .parse::<i64>()
                    .map_err(|err| format!("failed to parse long: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_provisioned_concurrency_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteProvisionedConcurrencyConfigRequest, String> {
    let mut input = DeleteProvisionedConcurrencyConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_account_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccountSettingsRequest, String> {
    let input = GetAccountSettingsRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAliasRequest, String> {
    let mut input = GetAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_capacity_provider_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCapacityProviderRequest, String> {
    let mut input = GetCapacityProviderRequest::default();
    for (name, value) in labels {
        match *name {
            "CapacityProviderName" => {
                input.capacity_provider_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_code_signing_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCodeSigningConfigRequest, String> {
    let mut input = GetCodeSigningConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "CodeSigningConfigArn" => {
                input.code_signing_config_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_durable_execution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDurableExecutionRequest, String> {
    let mut input = GetDurableExecutionRequest::default();
    for (name, value) in labels {
        match *name {
            "DurableExecutionArn" => {
                input.durable_execution_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_durable_execution_history_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDurableExecutionHistoryRequest, String> {
    let mut input = GetDurableExecutionHistoryRequest::default();
    for (name, value) in labels {
        match *name {
            "DurableExecutionArn" => {
                input.durable_execution_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("IncludeExecutionData") {
        input.include_execution_data = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("ReverseOrder") {
        input.reverse_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_durable_execution_state_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDurableExecutionStateRequest, String> {
    let mut input = GetDurableExecutionStateRequest::default();
    for (name, value) in labels {
        match *name {
            "DurableExecutionArn" => {
                input.durable_execution_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("CheckpointToken") {
        input.checkpoint_token = value.to_string();
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_event_source_mapping_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEventSourceMappingRequest, String> {
    let mut input = GetEventSourceMappingRequest::default();
    for (name, value) in labels {
        match *name {
            "UUID" => {
                input.u_u_i_d = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFunctionRequest, String> {
    let mut input = GetFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_function_code_signing_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFunctionCodeSigningConfigRequest, String> {
    let mut input = GetFunctionCodeSigningConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_function_concurrency_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFunctionConcurrencyRequest, String> {
    let mut input = GetFunctionConcurrencyRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_function_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFunctionConfigurationRequest, String> {
    let mut input = GetFunctionConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_function_event_invoke_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFunctionEventInvokeConfigRequest, String> {
    let mut input = GetFunctionEventInvokeConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_function_recursion_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFunctionRecursionConfigRequest, String> {
    let mut input = GetFunctionRecursionConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_function_scaling_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFunctionScalingConfigRequest, String> {
    let mut input = GetFunctionScalingConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_function_url_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFunctionUrlConfigRequest, String> {
    let mut input = GetFunctionUrlConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_layer_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetLayerVersionRequest, String> {
    let mut input = GetLayerVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "LayerName" => {
                input.layer_name = value.to_string();
            }
            "VersionNumber" => {
                input.version_number = value
                    .parse::<i64>()
                    .map_err(|err| format!("failed to parse long: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_layer_version_by_arn_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetLayerVersionByArnRequest, String> {
    let mut input = GetLayerVersionByArnRequest::default();
    if let Some(value) = query.get("Arn") {
        input.arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_layer_version_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetLayerVersionPolicyRequest, String> {
    let mut input = GetLayerVersionPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "LayerName" => {
                input.layer_name = value.to_string();
            }
            "VersionNumber" => {
                input.version_number = value
                    .parse::<i64>()
                    .map_err(|err| format!("failed to parse long: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPolicyRequest, String> {
    let mut input = GetPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_provisioned_concurrency_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetProvisionedConcurrencyConfigRequest, String> {
    let mut input = GetProvisionedConcurrencyConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_runtime_management_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRuntimeManagementConfigRequest, String> {
    let mut input = GetRuntimeManagementConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_invoke_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<InvocationRequest, String> {
    let mut input = InvocationRequest::default();
    if !request.body.is_empty() {
        input.payload = Some(request.body.clone());
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("X-Amz-Client-Context")
        .and_then(|value| value.to_str().ok())
    {
        input.client_context = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("X-Amz-Durable-Execution-Name")
        .and_then(|value| value.to_str().ok())
    {
        input.durable_execution_name = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("X-Amz-Invocation-Type")
        .and_then(|value| value.to_str().ok())
    {
        input.invocation_type = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("X-Amz-Log-Type")
        .and_then(|value| value.to_str().ok())
    {
        input.log_type = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("X-Amz-Tenant-Id")
        .and_then(|value| value.to_str().ok())
    {
        input.tenant_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_invoke_async_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<InvokeAsyncRequest, String> {
    let mut input = InvokeAsyncRequest::default();
    if !request.body.is_empty() {
        input.invoke_args = request.body.clone();
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_invoke_with_response_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<InvokeWithResponseStreamRequest, String> {
    let mut input = InvokeWithResponseStreamRequest::default();
    if !request.body.is_empty() {
        input.payload = Some(request.body.clone());
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("X-Amz-Client-Context")
        .and_then(|value| value.to_str().ok())
    {
        input.client_context = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("X-Amz-Invocation-Type")
        .and_then(|value| value.to_str().ok())
    {
        input.invocation_type = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("X-Amz-Log-Type")
        .and_then(|value| value.to_str().ok())
    {
        input.log_type = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("X-Amz-Tenant-Id")
        .and_then(|value| value.to_str().ok())
    {
        input.tenant_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_aliases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAliasesRequest, String> {
    let mut input = ListAliasesRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("FunctionVersion") {
        input.function_version = Some(value.to_string());
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_capacity_providers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCapacityProvidersRequest, String> {
    let mut input = ListCapacityProvidersRequest::default();
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("State") {
        input.state = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_code_signing_configs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCodeSigningConfigsRequest, String> {
    let mut input = ListCodeSigningConfigsRequest::default();
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_durable_executions_by_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDurableExecutionsByFunctionRequest, String> {
    let mut input = ListDurableExecutionsByFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("DurableExecutionName") {
        input.durable_execution_name = Some(value.to_string());
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    if let Some(value) = query.get("ReverseOrder") {
        input.reverse_order = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("StartedAfter") {
        input.started_after = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("StartedBefore") {
        input.started_before = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("Statuses") {
        input.statuses = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_event_source_mappings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEventSourceMappingsRequest, String> {
    let mut input = ListEventSourceMappingsRequest::default();
    if let Some(value) = query.get("EventSourceArn") {
        input.event_source_arn = Some(value.to_string());
    }
    if let Some(value) = query.get("FunctionName") {
        input.function_name = Some(value.to_string());
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_function_event_invoke_configs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFunctionEventInvokeConfigsRequest, String> {
    let mut input = ListFunctionEventInvokeConfigsRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_function_url_configs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFunctionUrlConfigsRequest, String> {
    let mut input = ListFunctionUrlConfigsRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_function_versions_by_capacity_provider_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFunctionVersionsByCapacityProviderRequest, String> {
    let mut input = ListFunctionVersionsByCapacityProviderRequest::default();
    for (name, value) in labels {
        match *name {
            "CapacityProviderName" => {
                input.capacity_provider_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_functions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFunctionsRequest, String> {
    let mut input = ListFunctionsRequest::default();
    if let Some(value) = query.get("FunctionVersion") {
        input.function_version = Some(value.to_string());
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MasterRegion") {
        input.master_region = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_functions_by_code_signing_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFunctionsByCodeSigningConfigRequest, String> {
    let mut input = ListFunctionsByCodeSigningConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "CodeSigningConfigArn" => {
                input.code_signing_config_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_layer_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListLayerVersionsRequest, String> {
    let mut input = ListLayerVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "LayerName" => {
                input.layer_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("CompatibleArchitecture") {
        input.compatible_architecture = Some(value.to_string());
    }
    if let Some(value) = query.get("CompatibleRuntime") {
        input.compatible_runtime = Some(value.to_string());
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_layers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListLayersRequest, String> {
    let mut input = ListLayersRequest::default();
    if let Some(value) = query.get("CompatibleArchitecture") {
        input.compatible_architecture = Some(value.to_string());
    }
    if let Some(value) = query.get("CompatibleRuntime") {
        input.compatible_runtime = Some(value.to_string());
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_provisioned_concurrency_configs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListProvisionedConcurrencyConfigsRequest, String> {
    let mut input = ListProvisionedConcurrencyConfigsRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsRequest, String> {
    let mut input = ListTagsRequest::default();
    for (name, value) in labels {
        match *name {
            "Resource" => {
                input.resource = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_versions_by_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVersionsByFunctionRequest, String> {
    let mut input = ListVersionsByFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_publish_layer_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PublishLayerVersionRequest, String> {
    let mut input = PublishLayerVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PublishLayerVersionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PublishLayerVersion request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "LayerName" => {
                input.layer_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_publish_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PublishVersionRequest, String> {
    let mut input = PublishVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PublishVersionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PublishVersion request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_function_code_signing_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutFunctionCodeSigningConfigRequest, String> {
    let mut input = PutFunctionCodeSigningConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutFunctionCodeSigningConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutFunctionCodeSigningConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_function_concurrency_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutFunctionConcurrencyRequest, String> {
    let mut input = PutFunctionConcurrencyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutFunctionConcurrencyRequest>(&request.body).map_err(
            |err| format!("failed to deserialize PutFunctionConcurrency request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_function_event_invoke_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutFunctionEventInvokeConfigRequest, String> {
    let mut input = PutFunctionEventInvokeConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutFunctionEventInvokeConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutFunctionEventInvokeConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_function_recursion_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutFunctionRecursionConfigRequest, String> {
    let mut input = PutFunctionRecursionConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutFunctionRecursionConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutFunctionRecursionConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_function_scaling_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutFunctionScalingConfigRequest, String> {
    let mut input = PutFunctionScalingConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutFunctionScalingConfigRequest>(&request.body).map_err(
            |err| format!("failed to deserialize PutFunctionScalingConfig request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_provisioned_concurrency_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutProvisionedConcurrencyConfigRequest, String> {
    let mut input = PutProvisionedConcurrencyConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutProvisionedConcurrencyConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutProvisionedConcurrencyConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_runtime_management_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutRuntimeManagementConfigRequest, String> {
    let mut input = PutRuntimeManagementConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutRuntimeManagementConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutRuntimeManagementConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_layer_version_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveLayerVersionPermissionRequest, String> {
    let mut input = RemoveLayerVersionPermissionRequest::default();
    for (name, value) in labels {
        match *name {
            "LayerName" => {
                input.layer_name = value.to_string();
            }
            "StatementId" => {
                input.statement_id = value.to_string();
            }
            "VersionNumber" => {
                input.version_number = value
                    .parse::<i64>()
                    .map_err(|err| format!("failed to parse long: {err}"))?;
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("RevisionId") {
        input.revision_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemovePermissionRequest, String> {
    let mut input = RemovePermissionRequest::default();
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            "StatementId" => {
                input.statement_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    if let Some(value) = query.get("RevisionId") {
        input.revision_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_send_durable_execution_callback_failure_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SendDurableExecutionCallbackFailureRequest, String> {
    let mut input = SendDurableExecutionCallbackFailureRequest::default();
    if !request.body.is_empty() {
        let payload = serde_json::from_slice::<ErrorObject>(&request.body).map_err(|err| {
            format!("failed to deserialize SendDurableExecutionCallbackFailure payload: {err}")
        })?;
        input.error = Some(payload);
    }
    for (name, value) in labels {
        match *name {
            "CallbackId" => {
                input.callback_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_send_durable_execution_callback_heartbeat_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SendDurableExecutionCallbackHeartbeatRequest, String> {
    let mut input = SendDurableExecutionCallbackHeartbeatRequest::default();
    for (name, value) in labels {
        match *name {
            "CallbackId" => {
                input.callback_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_send_durable_execution_callback_success_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SendDurableExecutionCallbackSuccessRequest, String> {
    let mut input = SendDurableExecutionCallbackSuccessRequest::default();
    if !request.body.is_empty() {
        input.result = Some(request.body.clone());
    }
    for (name, value) in labels {
        match *name {
            "CallbackId" => {
                input.callback_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_durable_execution_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopDurableExecutionRequest, String> {
    let mut input = StopDurableExecutionRequest::default();
    if !request.body.is_empty() {
        let payload = serde_json::from_slice::<ErrorObject>(&request.body)
            .map_err(|err| format!("failed to deserialize StopDurableExecution payload: {err}"))?;
        input.error = Some(payload);
    }
    for (name, value) in labels {
        match *name {
            "DurableExecutionArn" => {
                input.durable_execution_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceRequest, String> {
    let mut input = TagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Resource" => {
                input.resource = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "Resource" => {
                input.resource = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("tagKeys") {
        input.tag_keys = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAliasRequest, String> {
    let mut input = UpdateAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_capacity_provider_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCapacityProviderRequest, String> {
    let mut input = UpdateCapacityProviderRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCapacityProviderRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateCapacityProvider request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "CapacityProviderName" => {
                input.capacity_provider_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_code_signing_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCodeSigningConfigRequest, String> {
    let mut input = UpdateCodeSigningConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCodeSigningConfigRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateCodeSigningConfig request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "CodeSigningConfigArn" => {
                input.code_signing_config_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_event_source_mapping_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateEventSourceMappingRequest, String> {
    let mut input = UpdateEventSourceMappingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateEventSourceMappingRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateEventSourceMapping request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "UUID" => {
                input.u_u_i_d = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_function_code_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFunctionCodeRequest, String> {
    let mut input = UpdateFunctionCodeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFunctionCodeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFunctionCode request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_function_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFunctionConfigurationRequest, String> {
    let mut input = UpdateFunctionConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFunctionConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateFunctionConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_function_event_invoke_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFunctionEventInvokeConfigRequest, String> {
    let mut input = UpdateFunctionEventInvokeConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFunctionEventInvokeConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateFunctionEventInvokeConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_function_url_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFunctionUrlConfigRequest, String> {
    let mut input = UpdateFunctionUrlConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFunctionUrlConfigRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateFunctionUrlConfig request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "FunctionName" => {
                input.function_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("Qualifier") {
        input.qualifier = Some(value.to_string());
    }
    Ok(input)
}
