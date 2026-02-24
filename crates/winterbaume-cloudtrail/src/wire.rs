//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudtrail

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_add_tags_response(result: &AddTagsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_query_response(result: &CancelQueryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_channel_response(result: &CreateChannelResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_dashboard_response(result: &CreateDashboardResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_event_data_store_response(
    result: &CreateEventDataStoreResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_trail_response(result: &CreateTrailResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_channel_response(result: &DeleteChannelResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_dashboard_response(result: &DeleteDashboardResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_event_data_store_response(
    result: &DeleteEventDataStoreResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_resource_policy_response(
    result: &DeleteResourcePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_trail_response(result: &DeleteTrailResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_deregister_organization_delegated_admin_response(
    result: &DeregisterOrganizationDelegatedAdminResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_query_response(result: &DescribeQueryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_trails_response(result: &DescribeTrailsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_federation_response(result: &DisableFederationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_federation_response(result: &EnableFederationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_generate_query_response(result: &GenerateQueryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_channel_response(result: &GetChannelResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_dashboard_response(result: &GetDashboardResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_event_configuration_response(
    result: &GetEventConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_event_data_store_response(result: &GetEventDataStoreResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_event_selectors_response(result: &GetEventSelectorsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_import_response(result: &GetImportResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_insight_selectors_response(
    result: &GetInsightSelectorsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_query_results_response(result: &GetQueryResultsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_policy_response(result: &GetResourcePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_trail_response(result: &GetTrailResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_trail_status_response(result: &GetTrailStatusResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_channels_response(result: &ListChannelsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_dashboards_response(result: &ListDashboardsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_event_data_stores_response(
    result: &ListEventDataStoresResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_import_failures_response(
    result: &ListImportFailuresResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_imports_response(result: &ListImportsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_insights_data_response(result: &ListInsightsDataResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_insights_metric_data_response(
    result: &ListInsightsMetricDataResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_public_keys_response(result: &ListPublicKeysResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_queries_response(result: &ListQueriesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_response(result: &ListTagsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_trails_response(result: &ListTrailsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_lookup_events_response(result: &LookupEventsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_event_configuration_response(
    result: &PutEventConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_event_selectors_response(result: &PutEventSelectorsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_insight_selectors_response(
    result: &PutInsightSelectorsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_organization_delegated_admin_response(
    result: &RegisterOrganizationDelegatedAdminResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_tags_response(result: &RemoveTagsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_restore_event_data_store_response(
    result: &RestoreEventDataStoreResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_search_sample_queries_response(
    result: &SearchSampleQueriesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_dashboard_refresh_response(
    result: &StartDashboardRefreshResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_event_data_store_ingestion_response(
    result: &StartEventDataStoreIngestionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_import_response(result: &StartImportResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_logging_response(result: &StartLoggingResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_query_response(result: &StartQueryResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_event_data_store_ingestion_response(
    result: &StopEventDataStoreIngestionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_import_response(result: &StopImportResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_logging_response(result: &StopLoggingResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_channel_response(result: &UpdateChannelResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_dashboard_response(result: &UpdateDashboardResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_event_data_store_response(
    result: &UpdateEventDataStoreResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_trail_response(result: &UpdateTrailResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_tags_request(body: &[u8]) -> Result<AddTagsRequest, String> {
    if body.is_empty() {
        return Ok(AddTagsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize AddTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_query_request(body: &[u8]) -> Result<CancelQueryRequest, String> {
    if body.is_empty() {
        return Ok(CancelQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_channel_request(body: &[u8]) -> Result<CreateChannelRequest, String> {
    if body.is_empty() {
        return Ok(CreateChannelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateChannel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_dashboard_request(body: &[u8]) -> Result<CreateDashboardRequest, String> {
    if body.is_empty() {
        return Ok(CreateDashboardRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDashboard request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_event_data_store_request(
    body: &[u8],
) -> Result<CreateEventDataStoreRequest, String> {
    if body.is_empty() {
        return Ok(CreateEventDataStoreRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateEventDataStore request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_trail_request(body: &[u8]) -> Result<CreateTrailRequest, String> {
    if body.is_empty() {
        return Ok(CreateTrailRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTrail request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_channel_request(body: &[u8]) -> Result<DeleteChannelRequest, String> {
    if body.is_empty() {
        return Ok(DeleteChannelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteChannel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_dashboard_request(body: &[u8]) -> Result<DeleteDashboardRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDashboardRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDashboard request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_event_data_store_request(
    body: &[u8],
) -> Result<DeleteEventDataStoreRequest, String> {
    if body.is_empty() {
        return Ok(DeleteEventDataStoreRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteEventDataStore request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resource_policy_request(
    body: &[u8],
) -> Result<DeleteResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_trail_request(body: &[u8]) -> Result<DeleteTrailRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTrailRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTrail request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_organization_delegated_admin_request(
    body: &[u8],
) -> Result<DeregisterOrganizationDelegatedAdminRequest, String> {
    if body.is_empty() {
        return Ok(DeregisterOrganizationDelegatedAdminRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DeregisterOrganizationDelegatedAdmin request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_query_request(body: &[u8]) -> Result<DescribeQueryRequest, String> {
    if body.is_empty() {
        return Ok(DescribeQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_trails_request(body: &[u8]) -> Result<DescribeTrailsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTrailsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTrails request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_federation_request(
    body: &[u8],
) -> Result<DisableFederationRequest, String> {
    if body.is_empty() {
        return Ok(DisableFederationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableFederation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_federation_request(
    body: &[u8],
) -> Result<EnableFederationRequest, String> {
    if body.is_empty() {
        return Ok(EnableFederationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableFederation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_generate_query_request(body: &[u8]) -> Result<GenerateQueryRequest, String> {
    if body.is_empty() {
        return Ok(GenerateQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GenerateQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_channel_request(body: &[u8]) -> Result<GetChannelRequest, String> {
    if body.is_empty() {
        return Ok(GetChannelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetChannel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_dashboard_request(body: &[u8]) -> Result<GetDashboardRequest, String> {
    if body.is_empty() {
        return Ok(GetDashboardRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDashboard request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_event_configuration_request(
    body: &[u8],
) -> Result<GetEventConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(GetEventConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetEventConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_event_data_store_request(
    body: &[u8],
) -> Result<GetEventDataStoreRequest, String> {
    if body.is_empty() {
        return Ok(GetEventDataStoreRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetEventDataStore request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_event_selectors_request(
    body: &[u8],
) -> Result<GetEventSelectorsRequest, String> {
    if body.is_empty() {
        return Ok(GetEventSelectorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetEventSelectors request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_import_request(body: &[u8]) -> Result<GetImportRequest, String> {
    if body.is_empty() {
        return Ok(GetImportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetImport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_insight_selectors_request(
    body: &[u8],
) -> Result<GetInsightSelectorsRequest, String> {
    if body.is_empty() {
        return Ok(GetInsightSelectorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetInsightSelectors request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_query_results_request(
    body: &[u8],
) -> Result<GetQueryResultsRequest, String> {
    if body.is_empty() {
        return Ok(GetQueryResultsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetQueryResults request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_resource_policy_request(
    body: &[u8],
) -> Result<GetResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_trail_request(body: &[u8]) -> Result<GetTrailRequest, String> {
    if body.is_empty() {
        return Ok(GetTrailRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetTrail request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_trail_status_request(body: &[u8]) -> Result<GetTrailStatusRequest, String> {
    if body.is_empty() {
        return Ok(GetTrailStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTrailStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_channels_request(body: &[u8]) -> Result<ListChannelsRequest, String> {
    if body.is_empty() {
        return Ok(ListChannelsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListChannels request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_dashboards_request(body: &[u8]) -> Result<ListDashboardsRequest, String> {
    if body.is_empty() {
        return Ok(ListDashboardsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDashboards request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_event_data_stores_request(
    body: &[u8],
) -> Result<ListEventDataStoresRequest, String> {
    if body.is_empty() {
        return Ok(ListEventDataStoresRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListEventDataStores request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_import_failures_request(
    body: &[u8],
) -> Result<ListImportFailuresRequest, String> {
    if body.is_empty() {
        return Ok(ListImportFailuresRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListImportFailures request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_imports_request(body: &[u8]) -> Result<ListImportsRequest, String> {
    if body.is_empty() {
        return Ok(ListImportsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListImports request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_insights_data_request(
    body: &[u8],
) -> Result<ListInsightsDataRequest, String> {
    if body.is_empty() {
        return Ok(ListInsightsDataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListInsightsData request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_insights_metric_data_request(
    body: &[u8],
) -> Result<ListInsightsMetricDataRequest, String> {
    if body.is_empty() {
        return Ok(ListInsightsMetricDataRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListInsightsMetricData request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_public_keys_request(body: &[u8]) -> Result<ListPublicKeysRequest, String> {
    if body.is_empty() {
        return Ok(ListPublicKeysRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPublicKeys request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_queries_request(body: &[u8]) -> Result<ListQueriesRequest, String> {
    if body.is_empty() {
        return Ok(ListQueriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListQueries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_request(body: &[u8]) -> Result<ListTagsRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize ListTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_trails_request(body: &[u8]) -> Result<ListTrailsRequest, String> {
    if body.is_empty() {
        return Ok(ListTrailsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTrails request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_lookup_events_request(body: &[u8]) -> Result<LookupEventsRequest, String> {
    if body.is_empty() {
        return Ok(LookupEventsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize LookupEvents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_event_configuration_request(
    body: &[u8],
) -> Result<PutEventConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(PutEventConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutEventConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_event_selectors_request(
    body: &[u8],
) -> Result<PutEventSelectorsRequest, String> {
    if body.is_empty() {
        return Ok(PutEventSelectorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutEventSelectors request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_insight_selectors_request(
    body: &[u8],
) -> Result<PutInsightSelectorsRequest, String> {
    if body.is_empty() {
        return Ok(PutInsightSelectorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutInsightSelectors request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_resource_policy_request(
    body: &[u8],
) -> Result<PutResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_organization_delegated_admin_request(
    body: &[u8],
) -> Result<RegisterOrganizationDelegatedAdminRequest, String> {
    if body.is_empty() {
        return Ok(RegisterOrganizationDelegatedAdminRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize RegisterOrganizationDelegatedAdmin request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_tags_request(body: &[u8]) -> Result<RemoveTagsRequest, String> {
    if body.is_empty() {
        return Ok(RemoveTagsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_restore_event_data_store_request(
    body: &[u8],
) -> Result<RestoreEventDataStoreRequest, String> {
    if body.is_empty() {
        return Ok(RestoreEventDataStoreRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RestoreEventDataStore request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_search_sample_queries_request(
    body: &[u8],
) -> Result<SearchSampleQueriesRequest, String> {
    if body.is_empty() {
        return Ok(SearchSampleQueriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SearchSampleQueries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_dashboard_refresh_request(
    body: &[u8],
) -> Result<StartDashboardRefreshRequest, String> {
    if body.is_empty() {
        return Ok(StartDashboardRefreshRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartDashboardRefresh request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_event_data_store_ingestion_request(
    body: &[u8],
) -> Result<StartEventDataStoreIngestionRequest, String> {
    if body.is_empty() {
        return Ok(StartEventDataStoreIngestionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartEventDataStoreIngestion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_import_request(body: &[u8]) -> Result<StartImportRequest, String> {
    if body.is_empty() {
        return Ok(StartImportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartImport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_logging_request(body: &[u8]) -> Result<StartLoggingRequest, String> {
    if body.is_empty() {
        return Ok(StartLoggingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartLogging request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_query_request(body: &[u8]) -> Result<StartQueryRequest, String> {
    if body.is_empty() {
        return Ok(StartQueryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartQuery request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_event_data_store_ingestion_request(
    body: &[u8],
) -> Result<StopEventDataStoreIngestionRequest, String> {
    if body.is_empty() {
        return Ok(StopEventDataStoreIngestionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopEventDataStoreIngestion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_import_request(body: &[u8]) -> Result<StopImportRequest, String> {
    if body.is_empty() {
        return Ok(StopImportRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopImport request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_logging_request(body: &[u8]) -> Result<StopLoggingRequest, String> {
    if body.is_empty() {
        return Ok(StopLoggingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopLogging request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_channel_request(body: &[u8]) -> Result<UpdateChannelRequest, String> {
    if body.is_empty() {
        return Ok(UpdateChannelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateChannel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_dashboard_request(body: &[u8]) -> Result<UpdateDashboardRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDashboardRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDashboard request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_event_data_store_request(
    body: &[u8],
) -> Result<UpdateEventDataStoreRequest, String> {
    if body.is_empty() {
        return Ok(UpdateEventDataStoreRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateEventDataStore request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_trail_request(body: &[u8]) -> Result<UpdateTrailRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTrailRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTrail request: {e}"))
}
