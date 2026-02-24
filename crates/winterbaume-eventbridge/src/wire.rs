//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-eventbridge

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize void response for awsJson protocol.
pub fn serialize_activate_event_source_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_replay_response(result: &CancelReplayResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_api_destination_response(
    result: &CreateApiDestinationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_archive_response(result: &CreateArchiveResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_connection_response(result: &CreateConnectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_endpoint_response(result: &CreateEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_event_bus_response(result: &CreateEventBusResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_partner_event_source_response(
    result: &CreatePartnerEventSourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_deactivate_event_source_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_deauthorize_connection_response(
    result: &DeauthorizeConnectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_api_destination_response(
    result: &DeleteApiDestinationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_archive_response(result: &DeleteArchiveResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_connection_response(result: &DeleteConnectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_endpoint_response(result: &DeleteEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_event_bus_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_partner_event_source_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_rule_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_api_destination_response(
    result: &DescribeApiDestinationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_archive_response(result: &DescribeArchiveResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_connection_response(result: &DescribeConnectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_endpoint_response(result: &DescribeEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_event_bus_response(result: &DescribeEventBusResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_event_source_response(
    result: &DescribeEventSourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_partner_event_source_response(
    result: &DescribePartnerEventSourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_replay_response(result: &DescribeReplayResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_rule_response(result: &DescribeRuleResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_disable_rule_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_enable_rule_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_api_destinations_response(
    result: &ListApiDestinationsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_archives_response(result: &ListArchivesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_connections_response(result: &ListConnectionsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_endpoints_response(result: &ListEndpointsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_event_buses_response(result: &ListEventBusesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_event_sources_response(result: &ListEventSourcesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_partner_event_source_accounts_response(
    result: &ListPartnerEventSourceAccountsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_partner_event_sources_response(
    result: &ListPartnerEventSourcesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_replays_response(result: &ListReplaysResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_rule_names_by_target_response(
    result: &ListRuleNamesByTargetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_rules_response(result: &ListRulesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_targets_by_rule_response(result: &ListTargetsByRuleResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_events_response(result: &PutEventsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_partner_events_response(result: &PutPartnerEventsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_permission_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_rule_response(result: &PutRuleResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_targets_response(result: &PutTargetsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_remove_permission_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_remove_targets_response(result: &RemoveTargetsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_replay_response(result: &StartReplayResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_test_event_pattern_response(result: &TestEventPatternResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_api_destination_response(
    result: &UpdateApiDestinationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_archive_response(result: &UpdateArchiveResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_connection_response(result: &UpdateConnectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_endpoint_response(result: &UpdateEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_event_bus_response(result: &UpdateEventBusResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_activate_event_source_request(
    body: &[u8],
) -> Result<ActivateEventSourceRequest, String> {
    if body.is_empty() {
        return Ok(ActivateEventSourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ActivateEventSource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_replay_request(body: &[u8]) -> Result<CancelReplayRequest, String> {
    if body.is_empty() {
        return Ok(CancelReplayRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelReplay request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_api_destination_request(
    body: &[u8],
) -> Result<CreateApiDestinationRequest, String> {
    if body.is_empty() {
        return Ok(CreateApiDestinationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateApiDestination request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_archive_request(body: &[u8]) -> Result<CreateArchiveRequest, String> {
    if body.is_empty() {
        return Ok(CreateArchiveRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateArchive request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_connection_request(
    body: &[u8],
) -> Result<CreateConnectionRequest, String> {
    if body.is_empty() {
        return Ok(CreateConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_endpoint_request(body: &[u8]) -> Result<CreateEndpointRequest, String> {
    if body.is_empty() {
        return Ok(CreateEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_event_bus_request(body: &[u8]) -> Result<CreateEventBusRequest, String> {
    if body.is_empty() {
        return Ok(CreateEventBusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateEventBus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_partner_event_source_request(
    body: &[u8],
) -> Result<CreatePartnerEventSourceRequest, String> {
    if body.is_empty() {
        return Ok(CreatePartnerEventSourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePartnerEventSource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deactivate_event_source_request(
    body: &[u8],
) -> Result<DeactivateEventSourceRequest, String> {
    if body.is_empty() {
        return Ok(DeactivateEventSourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeactivateEventSource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deauthorize_connection_request(
    body: &[u8],
) -> Result<DeauthorizeConnectionRequest, String> {
    if body.is_empty() {
        return Ok(DeauthorizeConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeauthorizeConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_api_destination_request(
    body: &[u8],
) -> Result<DeleteApiDestinationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteApiDestinationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteApiDestination request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_archive_request(body: &[u8]) -> Result<DeleteArchiveRequest, String> {
    if body.is_empty() {
        return Ok(DeleteArchiveRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteArchive request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_connection_request(
    body: &[u8],
) -> Result<DeleteConnectionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_endpoint_request(body: &[u8]) -> Result<DeleteEndpointRequest, String> {
    if body.is_empty() {
        return Ok(DeleteEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_event_bus_request(body: &[u8]) -> Result<DeleteEventBusRequest, String> {
    if body.is_empty() {
        return Ok(DeleteEventBusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteEventBus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_partner_event_source_request(
    body: &[u8],
) -> Result<DeletePartnerEventSourceRequest, String> {
    if body.is_empty() {
        return Ok(DeletePartnerEventSourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePartnerEventSource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_rule_request(body: &[u8]) -> Result<DeleteRuleRequest, String> {
    if body.is_empty() {
        return Ok(DeleteRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_api_destination_request(
    body: &[u8],
) -> Result<DescribeApiDestinationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeApiDestinationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeApiDestination request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_archive_request(body: &[u8]) -> Result<DescribeArchiveRequest, String> {
    if body.is_empty() {
        return Ok(DescribeArchiveRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeArchive request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_connection_request(
    body: &[u8],
) -> Result<DescribeConnectionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_endpoint_request(
    body: &[u8],
) -> Result<DescribeEndpointRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_event_bus_request(
    body: &[u8],
) -> Result<DescribeEventBusRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEventBusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEventBus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_event_source_request(
    body: &[u8],
) -> Result<DescribeEventSourceRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEventSourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEventSource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_partner_event_source_request(
    body: &[u8],
) -> Result<DescribePartnerEventSourceRequest, String> {
    if body.is_empty() {
        return Ok(DescribePartnerEventSourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePartnerEventSource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_replay_request(body: &[u8]) -> Result<DescribeReplayRequest, String> {
    if body.is_empty() {
        return Ok(DescribeReplayRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeReplay request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_rule_request(body: &[u8]) -> Result<DescribeRuleRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_rule_request(body: &[u8]) -> Result<DisableRuleRequest, String> {
    if body.is_empty() {
        return Ok(DisableRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_rule_request(body: &[u8]) -> Result<EnableRuleRequest, String> {
    if body.is_empty() {
        return Ok(EnableRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_api_destinations_request(
    body: &[u8],
) -> Result<ListApiDestinationsRequest, String> {
    if body.is_empty() {
        return Ok(ListApiDestinationsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListApiDestinations request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_archives_request(body: &[u8]) -> Result<ListArchivesRequest, String> {
    if body.is_empty() {
        return Ok(ListArchivesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListArchives request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_connections_request(body: &[u8]) -> Result<ListConnectionsRequest, String> {
    if body.is_empty() {
        return Ok(ListConnectionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListConnections request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_endpoints_request(body: &[u8]) -> Result<ListEndpointsRequest, String> {
    if body.is_empty() {
        return Ok(ListEndpointsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListEndpoints request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_event_buses_request(body: &[u8]) -> Result<ListEventBusesRequest, String> {
    if body.is_empty() {
        return Ok(ListEventBusesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListEventBuses request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_event_sources_request(
    body: &[u8],
) -> Result<ListEventSourcesRequest, String> {
    if body.is_empty() {
        return Ok(ListEventSourcesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListEventSources request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_partner_event_source_accounts_request(
    body: &[u8],
) -> Result<ListPartnerEventSourceAccountsRequest, String> {
    if body.is_empty() {
        return Ok(ListPartnerEventSourceAccountsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPartnerEventSourceAccounts request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_partner_event_sources_request(
    body: &[u8],
) -> Result<ListPartnerEventSourcesRequest, String> {
    if body.is_empty() {
        return Ok(ListPartnerEventSourcesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPartnerEventSources request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_replays_request(body: &[u8]) -> Result<ListReplaysRequest, String> {
    if body.is_empty() {
        return Ok(ListReplaysRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListReplays request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_rule_names_by_target_request(
    body: &[u8],
) -> Result<ListRuleNamesByTargetRequest, String> {
    if body.is_empty() {
        return Ok(ListRuleNamesByTargetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRuleNamesByTarget request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_rules_request(body: &[u8]) -> Result<ListRulesRequest, String> {
    if body.is_empty() {
        return Ok(ListRulesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRules request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_targets_by_rule_request(
    body: &[u8],
) -> Result<ListTargetsByRuleRequest, String> {
    if body.is_empty() {
        return Ok(ListTargetsByRuleRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTargetsByRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_events_request(body: &[u8]) -> Result<PutEventsRequest, String> {
    if body.is_empty() {
        return Ok(PutEventsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutEvents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_partner_events_request(
    body: &[u8],
) -> Result<PutPartnerEventsRequest, String> {
    if body.is_empty() {
        return Ok(PutPartnerEventsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutPartnerEvents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_permission_request(body: &[u8]) -> Result<PutPermissionRequest, String> {
    if body.is_empty() {
        return Ok(PutPermissionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutPermission request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_rule_request(body: &[u8]) -> Result<PutRuleRequest, String> {
    if body.is_empty() {
        return Ok(PutRuleRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize PutRule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_targets_request(body: &[u8]) -> Result<PutTargetsRequest, String> {
    if body.is_empty() {
        return Ok(PutTargetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutTargets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_permission_request(
    body: &[u8],
) -> Result<RemovePermissionRequest, String> {
    if body.is_empty() {
        return Ok(RemovePermissionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemovePermission request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_targets_request(body: &[u8]) -> Result<RemoveTargetsRequest, String> {
    if body.is_empty() {
        return Ok(RemoveTargetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveTargets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_replay_request(body: &[u8]) -> Result<StartReplayRequest, String> {
    if body.is_empty() {
        return Ok(StartReplayRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartReplay request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceRequest, String> {
    if body.is_empty() {
        return Ok(TagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_test_event_pattern_request(
    body: &[u8],
) -> Result<TestEventPatternRequest, String> {
    if body.is_empty() {
        return Ok(TestEventPatternRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TestEventPattern request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceRequest, String> {
    if body.is_empty() {
        return Ok(UntagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_api_destination_request(
    body: &[u8],
) -> Result<UpdateApiDestinationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateApiDestinationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateApiDestination request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_archive_request(body: &[u8]) -> Result<UpdateArchiveRequest, String> {
    if body.is_empty() {
        return Ok(UpdateArchiveRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateArchive request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_connection_request(
    body: &[u8],
) -> Result<UpdateConnectionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateConnectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateConnection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_endpoint_request(body: &[u8]) -> Result<UpdateEndpointRequest, String> {
    if body.is_empty() {
        return Ok(UpdateEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_event_bus_request(body: &[u8]) -> Result<UpdateEventBusRequest, String> {
    if body.is_empty() {
        return Ok(UpdateEventBusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateEventBus request: {e}"))
}
