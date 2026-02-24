//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-medialive

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
pub fn serialize_accept_input_device_transfer_response(
    result: &AcceptInputDeviceTransferResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_delete_response(result: &BatchDeleteResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_start_response(result: &BatchStartResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_stop_response(result: &BatchStopResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_update_schedule_response(
    result: &BatchUpdateScheduleResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_input_device_transfer_response(
    result: &CancelInputDeviceTransferResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_claim_device_response(result: &ClaimDeviceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_channel_response(result: &CreateChannelResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_channel_placement_group_response(
    result: &CreateChannelPlacementGroupResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_cloud_watch_alarm_template_response(
    result: &CreateCloudWatchAlarmTemplateResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_cloud_watch_alarm_template_group_response(
    result: &CreateCloudWatchAlarmTemplateGroupResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_cluster_response(result: &CreateClusterResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_event_bridge_rule_template_response(
    result: &CreateEventBridgeRuleTemplateResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_event_bridge_rule_template_group_response(
    result: &CreateEventBridgeRuleTemplateGroupResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_input_response(result: &CreateInputResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_input_security_group_response(
    result: &CreateInputSecurityGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_multiplex_response(result: &CreateMultiplexResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_multiplex_program_response(
    result: &CreateMultiplexProgramResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_network_response(result: &CreateNetworkResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_node_response(result: &CreateNodeResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_node_registration_script_response(
    result: &CreateNodeRegistrationScriptResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_partner_input_response(
    result: &CreatePartnerInputResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_sdi_source_response(result: &CreateSdiSourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_signal_map_response(result: &CreateSignalMapResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_create_tags_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_channel_response(result: &DeleteChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_channel_placement_group_response(
    result: &DeleteChannelPlacementGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_cloud_watch_alarm_template_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_cloud_watch_alarm_template_group_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_cluster_response(result: &DeleteClusterResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_event_bridge_rule_template_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_event_bridge_rule_template_group_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_input_response(result: &DeleteInputResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_input_security_group_response(
    result: &DeleteInputSecurityGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_multiplex_response(result: &DeleteMultiplexResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_multiplex_program_response(
    result: &DeleteMultiplexProgramResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_network_response(result: &DeleteNetworkResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_node_response(result: &DeleteNodeResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_reservation_response(result: &DeleteReservationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_schedule_response(result: &DeleteScheduleResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_sdi_source_response(result: &DeleteSdiSourceResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_signal_map_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_tags_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_account_configuration_response(
    result: &DescribeAccountConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_channel_response(result: &DescribeChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_channel_placement_group_response(
    result: &DescribeChannelPlacementGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_cluster_response(result: &DescribeClusterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_input_response(result: &DescribeInputResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_input_device_response(
    result: &DescribeInputDeviceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_input_device_thumbnail_response(
    result: &DescribeInputDeviceThumbnailResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "content-length": set by caller from content_length field
    // Header "content-type": set by caller from content_type field
    // Header "etag": set by caller from e_tag field
    // Header "last-modified": set by caller from last_modified field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_input_security_group_response(
    result: &DescribeInputSecurityGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_multiplex_response(result: &DescribeMultiplexResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_multiplex_program_response(
    result: &DescribeMultiplexProgramResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_network_response(result: &DescribeNetworkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_node_response(result: &DescribeNodeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_offering_response(result: &DescribeOfferingResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_reservation_response(
    result: &DescribeReservationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_schedule_response(result: &DescribeScheduleResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_sdi_source_response(result: &DescribeSdiSourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_thumbnails_response(result: &DescribeThumbnailsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_cloud_watch_alarm_template_response(
    result: &GetCloudWatchAlarmTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_cloud_watch_alarm_template_group_response(
    result: &GetCloudWatchAlarmTemplateGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_event_bridge_rule_template_response(
    result: &GetEventBridgeRuleTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_event_bridge_rule_template_group_response(
    result: &GetEventBridgeRuleTemplateGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_signal_map_response(result: &GetSignalMapResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_alerts_response(result: &ListAlertsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_channel_placement_groups_response(
    result: &ListChannelPlacementGroupsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_channels_response(result: &ListChannelsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_cloud_watch_alarm_template_groups_response(
    result: &ListCloudWatchAlarmTemplateGroupsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_cloud_watch_alarm_templates_response(
    result: &ListCloudWatchAlarmTemplatesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_cluster_alerts_response(result: &ListClusterAlertsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_clusters_response(result: &ListClustersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_event_bridge_rule_template_groups_response(
    result: &ListEventBridgeRuleTemplateGroupsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_event_bridge_rule_templates_response(
    result: &ListEventBridgeRuleTemplatesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_input_device_transfers_response(
    result: &ListInputDeviceTransfersResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_input_devices_response(result: &ListInputDevicesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_input_security_groups_response(
    result: &ListInputSecurityGroupsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_inputs_response(result: &ListInputsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_multiplex_alerts_response(
    result: &ListMultiplexAlertsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_multiplex_programs_response(
    result: &ListMultiplexProgramsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_multiplexes_response(result: &ListMultiplexesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_networks_response(result: &ListNetworksResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_nodes_response(result: &ListNodesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_offerings_response(result: &ListOfferingsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_reservations_response(result: &ListReservationsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_sdi_sources_response(result: &ListSdiSourcesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_signal_maps_response(result: &ListSignalMapsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_versions_response(result: &ListVersionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_purchase_offering_response(result: &PurchaseOfferingResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_reboot_input_device_response(result: &RebootInputDeviceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_reject_input_device_transfer_response(
    result: &RejectInputDeviceTransferResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_restart_channel_pipelines_response(
    result: &RestartChannelPipelinesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_channel_response(result: &StartChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_delete_monitor_deployment_response(
    result: &StartDeleteMonitorDeploymentResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_input_device_response(result: &StartInputDeviceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_input_device_maintenance_window_response(
    result: &StartInputDeviceMaintenanceWindowResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_monitor_deployment_response(
    result: &StartMonitorDeploymentResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_multiplex_response(result: &StartMultiplexResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_update_signal_map_response(
    result: &StartUpdateSignalMapResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_channel_response(result: &StopChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_input_device_response(result: &StopInputDeviceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_multiplex_response(result: &StopMultiplexResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_transfer_input_device_response(
    result: &TransferInputDeviceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_account_configuration_response(
    result: &UpdateAccountConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_channel_response(result: &UpdateChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_channel_class_response(
    result: &UpdateChannelClassResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_channel_placement_group_response(
    result: &UpdateChannelPlacementGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_cloud_watch_alarm_template_response(
    result: &UpdateCloudWatchAlarmTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_cloud_watch_alarm_template_group_response(
    result: &UpdateCloudWatchAlarmTemplateGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_cluster_response(result: &UpdateClusterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_event_bridge_rule_template_response(
    result: &UpdateEventBridgeRuleTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_event_bridge_rule_template_group_response(
    result: &UpdateEventBridgeRuleTemplateGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_input_response(result: &UpdateInputResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_input_device_response(result: &UpdateInputDeviceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_input_security_group_response(
    result: &UpdateInputSecurityGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_multiplex_response(result: &UpdateMultiplexResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_multiplex_program_response(
    result: &UpdateMultiplexProgramResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_network_response(result: &UpdateNetworkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_node_response(result: &UpdateNodeResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_node_state_response(result: &UpdateNodeStateResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_reservation_response(result: &UpdateReservationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_sdi_source_response(result: &UpdateSdiSourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_accept_input_device_transfer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AcceptInputDeviceTransferRequest, String> {
    let mut input = AcceptInputDeviceTransferRequest::default();
    for (name, value) in labels {
        match *name {
            "InputDeviceId" => {
                input.input_device_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_delete_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchDeleteRequest, String> {
    let mut input = BatchDeleteRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchDeleteRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchDelete request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_start_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchStartRequest, String> {
    let mut input = BatchStartRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchStartRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchStart request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_stop_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchStopRequest, String> {
    let mut input = BatchStopRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchStopRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchStop request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_update_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchUpdateScheduleRequest, String> {
    let mut input = BatchUpdateScheduleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchUpdateScheduleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchUpdateSchedule request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ChannelId" => {
                input.channel_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_input_device_transfer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelInputDeviceTransferRequest, String> {
    let mut input = CancelInputDeviceTransferRequest::default();
    for (name, value) in labels {
        match *name {
            "InputDeviceId" => {
                input.input_device_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_claim_device_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ClaimDeviceRequest, String> {
    let mut input = ClaimDeviceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ClaimDeviceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ClaimDevice request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateChannelRequest, String> {
    let mut input = CreateChannelRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateChannelRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateChannel request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_channel_placement_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateChannelPlacementGroupRequest, String> {
    let mut input = CreateChannelPlacementGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateChannelPlacementGroupRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateChannelPlacementGroup request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_cloud_watch_alarm_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCloudWatchAlarmTemplateRequest, String> {
    let mut input = CreateCloudWatchAlarmTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCloudWatchAlarmTemplateRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateCloudWatchAlarmTemplate request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_cloud_watch_alarm_template_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCloudWatchAlarmTemplateGroupRequest, String> {
    let mut input = CreateCloudWatchAlarmTemplateGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCloudWatchAlarmTemplateGroupRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateCloudWatchAlarmTemplateGroup request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateClusterRequest, String> {
    let mut input = CreateClusterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateClusterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateCluster request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_event_bridge_rule_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateEventBridgeRuleTemplateRequest, String> {
    let mut input = CreateEventBridgeRuleTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateEventBridgeRuleTemplateRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateEventBridgeRuleTemplate request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_event_bridge_rule_template_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateEventBridgeRuleTemplateGroupRequest, String> {
    let mut input = CreateEventBridgeRuleTemplateGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateEventBridgeRuleTemplateGroupRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateEventBridgeRuleTemplateGroup request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_input_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateInputRequest, String> {
    let mut input = CreateInputRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateInputRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateInput request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_input_security_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateInputSecurityGroupRequest, String> {
    let mut input = CreateInputSecurityGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateInputSecurityGroupRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateInputSecurityGroup request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_multiplex_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMultiplexRequest, String> {
    let mut input = CreateMultiplexRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateMultiplexRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateMultiplex request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_multiplex_program_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMultiplexProgramRequest, String> {
    let mut input = CreateMultiplexProgramRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateMultiplexProgramRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateMultiplexProgram request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "MultiplexId" => {
                input.multiplex_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateNetworkRequest, String> {
    let mut input = CreateNetworkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateNetworkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateNetwork request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_node_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateNodeRequest, String> {
    let mut input = CreateNodeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateNodeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateNode request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_node_registration_script_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateNodeRegistrationScriptRequest, String> {
    let mut input = CreateNodeRegistrationScriptRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateNodeRegistrationScriptRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateNodeRegistrationScript request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_partner_input_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePartnerInputRequest, String> {
    let mut input = CreatePartnerInputRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePartnerInputRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreatePartnerInput request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InputId" => {
                input.input_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_sdi_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSdiSourceRequest, String> {
    let mut input = CreateSdiSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSdiSourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateSdiSource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_signal_map_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSignalMapRequest, String> {
    let mut input = CreateSignalMapRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSignalMapRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateSignalMap request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTagsRequest, String> {
    let mut input = CreateTagsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTagsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTags request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteChannelRequest, String> {
    let mut input = DeleteChannelRequest::default();
    for (name, value) in labels {
        match *name {
            "ChannelId" => {
                input.channel_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_channel_placement_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteChannelPlacementGroupRequest, String> {
    let mut input = DeleteChannelPlacementGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "ChannelPlacementGroupId" => {
                input.channel_placement_group_id = value.to_string();
            }
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_cloud_watch_alarm_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCloudWatchAlarmTemplateRequest, String> {
    let mut input = DeleteCloudWatchAlarmTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_cloud_watch_alarm_template_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCloudWatchAlarmTemplateGroupRequest, String> {
    let mut input = DeleteCloudWatchAlarmTemplateGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteClusterRequest, String> {
    let mut input = DeleteClusterRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_event_bridge_rule_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteEventBridgeRuleTemplateRequest, String> {
    let mut input = DeleteEventBridgeRuleTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_event_bridge_rule_template_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteEventBridgeRuleTemplateGroupRequest, String> {
    let mut input = DeleteEventBridgeRuleTemplateGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_input_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteInputRequest, String> {
    let mut input = DeleteInputRequest::default();
    for (name, value) in labels {
        match *name {
            "InputId" => {
                input.input_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_input_security_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteInputSecurityGroupRequest, String> {
    let mut input = DeleteInputSecurityGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "InputSecurityGroupId" => {
                input.input_security_group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_multiplex_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMultiplexRequest, String> {
    let mut input = DeleteMultiplexRequest::default();
    for (name, value) in labels {
        match *name {
            "MultiplexId" => {
                input.multiplex_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_multiplex_program_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMultiplexProgramRequest, String> {
    let mut input = DeleteMultiplexProgramRequest::default();
    for (name, value) in labels {
        match *name {
            "MultiplexId" => {
                input.multiplex_id = value.to_string();
            }
            "ProgramName" => {
                input.program_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteNetworkRequest, String> {
    let mut input = DeleteNetworkRequest::default();
    for (name, value) in labels {
        match *name {
            "NetworkId" => {
                input.network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_node_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteNodeRequest, String> {
    let mut input = DeleteNodeRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            "NodeId" => {
                input.node_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_reservation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteReservationRequest, String> {
    let mut input = DeleteReservationRequest::default();
    for (name, value) in labels {
        match *name {
            "ReservationId" => {
                input.reservation_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteScheduleRequest, String> {
    let mut input = DeleteScheduleRequest::default();
    for (name, value) in labels {
        match *name {
            "ChannelId" => {
                input.channel_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_sdi_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteSdiSourceRequest, String> {
    let mut input = DeleteSdiSourceRequest::default();
    for (name, value) in labels {
        match *name {
            "SdiSourceId" => {
                input.sdi_source_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_signal_map_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteSignalMapRequest, String> {
    let mut input = DeleteSignalMapRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTagsRequest, String> {
    let mut input = DeleteTagsRequest::default();
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
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
pub fn deserialize_describe_account_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAccountConfigurationRequest, String> {
    let input = DescribeAccountConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeChannelRequest, String> {
    let mut input = DescribeChannelRequest::default();
    for (name, value) in labels {
        match *name {
            "ChannelId" => {
                input.channel_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_channel_placement_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeChannelPlacementGroupRequest, String> {
    let mut input = DescribeChannelPlacementGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "ChannelPlacementGroupId" => {
                input.channel_placement_group_id = value.to_string();
            }
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeClusterRequest, String> {
    let mut input = DescribeClusterRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_input_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeInputRequest, String> {
    let mut input = DescribeInputRequest::default();
    for (name, value) in labels {
        match *name {
            "InputId" => {
                input.input_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_input_device_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeInputDeviceRequest, String> {
    let mut input = DescribeInputDeviceRequest::default();
    for (name, value) in labels {
        match *name {
            "InputDeviceId" => {
                input.input_device_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_input_device_thumbnail_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeInputDeviceThumbnailRequest, String> {
    let mut input = DescribeInputDeviceThumbnailRequest::default();
    for (name, value) in labels {
        match *name {
            "InputDeviceId" => {
                input.input_device_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("accept")
        .and_then(|value| value.to_str().ok())
    {
        input.accept = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_input_security_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeInputSecurityGroupRequest, String> {
    let mut input = DescribeInputSecurityGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "InputSecurityGroupId" => {
                input.input_security_group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_multiplex_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeMultiplexRequest, String> {
    let mut input = DescribeMultiplexRequest::default();
    for (name, value) in labels {
        match *name {
            "MultiplexId" => {
                input.multiplex_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_multiplex_program_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeMultiplexProgramRequest, String> {
    let mut input = DescribeMultiplexProgramRequest::default();
    for (name, value) in labels {
        match *name {
            "MultiplexId" => {
                input.multiplex_id = value.to_string();
            }
            "ProgramName" => {
                input.program_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeNetworkRequest, String> {
    let mut input = DescribeNetworkRequest::default();
    for (name, value) in labels {
        match *name {
            "NetworkId" => {
                input.network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_node_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeNodeRequest, String> {
    let mut input = DescribeNodeRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            "NodeId" => {
                input.node_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_offering_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeOfferingRequest, String> {
    let mut input = DescribeOfferingRequest::default();
    for (name, value) in labels {
        match *name {
            "OfferingId" => {
                input.offering_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_reservation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeReservationRequest, String> {
    let mut input = DescribeReservationRequest::default();
    for (name, value) in labels {
        match *name {
            "ReservationId" => {
                input.reservation_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeScheduleRequest, String> {
    let mut input = DescribeScheduleRequest::default();
    for (name, value) in labels {
        match *name {
            "ChannelId" => {
                input.channel_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_sdi_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeSdiSourceRequest, String> {
    let mut input = DescribeSdiSourceRequest::default();
    for (name, value) in labels {
        match *name {
            "SdiSourceId" => {
                input.sdi_source_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_thumbnails_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeThumbnailsRequest, String> {
    let mut input = DescribeThumbnailsRequest::default();
    for (name, value) in labels {
        match *name {
            "ChannelId" => {
                input.channel_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("pipelineId") {
        input.pipeline_id = value.to_string();
    }
    if let Some(value) = query.get("thumbnailType") {
        input.thumbnail_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_cloud_watch_alarm_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCloudWatchAlarmTemplateRequest, String> {
    let mut input = GetCloudWatchAlarmTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_cloud_watch_alarm_template_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCloudWatchAlarmTemplateGroupRequest, String> {
    let mut input = GetCloudWatchAlarmTemplateGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_event_bridge_rule_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEventBridgeRuleTemplateRequest, String> {
    let mut input = GetEventBridgeRuleTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_event_bridge_rule_template_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEventBridgeRuleTemplateGroupRequest, String> {
    let mut input = GetEventBridgeRuleTemplateGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_signal_map_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSignalMapRequest, String> {
    let mut input = GetSignalMapRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_alerts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAlertsRequest, String> {
    let mut input = ListAlertsRequest::default();
    for (name, value) in labels {
        match *name {
            "ChannelId" => {
                input.channel_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("stateFilter") {
        input.state_filter = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_channel_placement_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListChannelPlacementGroupsRequest, String> {
    let mut input = ListChannelPlacementGroupsRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_channels_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListChannelsRequest, String> {
    let mut input = ListChannelsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_cloud_watch_alarm_template_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCloudWatchAlarmTemplateGroupsRequest, String> {
    let mut input = ListCloudWatchAlarmTemplateGroupsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("scope") {
        input.scope = Some(value.to_string());
    }
    if let Some(value) = query.get("signalMapIdentifier") {
        input.signal_map_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_cloud_watch_alarm_templates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCloudWatchAlarmTemplatesRequest, String> {
    let mut input = ListCloudWatchAlarmTemplatesRequest::default();
    if let Some(value) = query.get("groupIdentifier") {
        input.group_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("scope") {
        input.scope = Some(value.to_string());
    }
    if let Some(value) = query.get("signalMapIdentifier") {
        input.signal_map_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_cluster_alerts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListClusterAlertsRequest, String> {
    let mut input = ListClusterAlertsRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("stateFilter") {
        input.state_filter = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_clusters_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListClustersRequest, String> {
    let mut input = ListClustersRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_event_bridge_rule_template_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEventBridgeRuleTemplateGroupsRequest, String> {
    let mut input = ListEventBridgeRuleTemplateGroupsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("signalMapIdentifier") {
        input.signal_map_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_event_bridge_rule_templates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEventBridgeRuleTemplatesRequest, String> {
    let mut input = ListEventBridgeRuleTemplatesRequest::default();
    if let Some(value) = query.get("groupIdentifier") {
        input.group_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("signalMapIdentifier") {
        input.signal_map_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_input_device_transfers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListInputDeviceTransfersRequest, String> {
    let mut input = ListInputDeviceTransfersRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("transferType") {
        input.transfer_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_input_devices_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListInputDevicesRequest, String> {
    let mut input = ListInputDevicesRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_input_security_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListInputSecurityGroupsRequest, String> {
    let mut input = ListInputSecurityGroupsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_inputs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListInputsRequest, String> {
    let mut input = ListInputsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_multiplex_alerts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMultiplexAlertsRequest, String> {
    let mut input = ListMultiplexAlertsRequest::default();
    for (name, value) in labels {
        match *name {
            "MultiplexId" => {
                input.multiplex_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("stateFilter") {
        input.state_filter = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_multiplex_programs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMultiplexProgramsRequest, String> {
    let mut input = ListMultiplexProgramsRequest::default();
    for (name, value) in labels {
        match *name {
            "MultiplexId" => {
                input.multiplex_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_multiplexes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMultiplexesRequest, String> {
    let mut input = ListMultiplexesRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_networks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListNetworksRequest, String> {
    let mut input = ListNetworksRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_nodes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListNodesRequest, String> {
    let mut input = ListNodesRequest::default();
    for (name, value) in labels {
        match *name {
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_offerings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListOfferingsRequest, String> {
    let mut input = ListOfferingsRequest::default();
    if let Some(value) = query.get("channelClass") {
        input.channel_class = Some(value.to_string());
    }
    if let Some(value) = query.get("channelConfiguration") {
        input.channel_configuration = Some(value.to_string());
    }
    if let Some(value) = query.get("codec") {
        input.codec = Some(value.to_string());
    }
    if let Some(value) = query.get("duration") {
        input.duration = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("maximumBitrate") {
        input.maximum_bitrate = Some(value.to_string());
    }
    if let Some(value) = query.get("maximumFramerate") {
        input.maximum_framerate = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resolution") {
        input.resolution = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceType") {
        input.resource_type = Some(value.to_string());
    }
    if let Some(value) = query.get("specialFeature") {
        input.special_feature = Some(value.to_string());
    }
    if let Some(value) = query.get("videoQuality") {
        input.video_quality = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_reservations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListReservationsRequest, String> {
    let mut input = ListReservationsRequest::default();
    if let Some(value) = query.get("channelClass") {
        input.channel_class = Some(value.to_string());
    }
    if let Some(value) = query.get("codec") {
        input.codec = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("maximumBitrate") {
        input.maximum_bitrate = Some(value.to_string());
    }
    if let Some(value) = query.get("maximumFramerate") {
        input.maximum_framerate = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resolution") {
        input.resolution = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceType") {
        input.resource_type = Some(value.to_string());
    }
    if let Some(value) = query.get("specialFeature") {
        input.special_feature = Some(value.to_string());
    }
    if let Some(value) = query.get("videoQuality") {
        input.video_quality = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_sdi_sources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSdiSourcesRequest, String> {
    let mut input = ListSdiSourcesRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_signal_maps_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSignalMapsRequest, String> {
    let mut input = ListSignalMapsRequest::default();
    if let Some(value) = query.get("cloudWatchAlarmTemplateGroupIdentifier") {
        input.cloud_watch_alarm_template_group_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("eventBridgeRuleTemplateGroupIdentifier") {
        input.event_bridge_rule_template_group_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceRequest, String> {
    let mut input = ListTagsForResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVersionsRequest, String> {
    let input = ListVersionsRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_purchase_offering_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PurchaseOfferingRequest, String> {
    let mut input = PurchaseOfferingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PurchaseOfferingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PurchaseOffering request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "OfferingId" => {
                input.offering_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reboot_input_device_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RebootInputDeviceRequest, String> {
    let mut input = RebootInputDeviceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RebootInputDeviceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RebootInputDevice request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InputDeviceId" => {
                input.input_device_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reject_input_device_transfer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RejectInputDeviceTransferRequest, String> {
    let mut input = RejectInputDeviceTransferRequest::default();
    for (name, value) in labels {
        match *name {
            "InputDeviceId" => {
                input.input_device_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_restart_channel_pipelines_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RestartChannelPipelinesRequest, String> {
    let mut input = RestartChannelPipelinesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RestartChannelPipelinesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize RestartChannelPipelines request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "ChannelId" => {
                input.channel_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartChannelRequest, String> {
    let mut input = StartChannelRequest::default();
    for (name, value) in labels {
        match *name {
            "ChannelId" => {
                input.channel_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_delete_monitor_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartDeleteMonitorDeploymentRequest, String> {
    let mut input = StartDeleteMonitorDeploymentRequest::default();
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_input_device_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartInputDeviceRequest, String> {
    let mut input = StartInputDeviceRequest::default();
    for (name, value) in labels {
        match *name {
            "InputDeviceId" => {
                input.input_device_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_input_device_maintenance_window_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartInputDeviceMaintenanceWindowRequest, String> {
    let mut input = StartInputDeviceMaintenanceWindowRequest::default();
    for (name, value) in labels {
        match *name {
            "InputDeviceId" => {
                input.input_device_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_monitor_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartMonitorDeploymentRequest, String> {
    let mut input = StartMonitorDeploymentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartMonitorDeploymentRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartMonitorDeployment request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_multiplex_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartMultiplexRequest, String> {
    let mut input = StartMultiplexRequest::default();
    for (name, value) in labels {
        match *name {
            "MultiplexId" => {
                input.multiplex_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_update_signal_map_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartUpdateSignalMapRequest, String> {
    let mut input = StartUpdateSignalMapRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartUpdateSignalMapRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartUpdateSignalMap request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopChannelRequest, String> {
    let mut input = StopChannelRequest::default();
    for (name, value) in labels {
        match *name {
            "ChannelId" => {
                input.channel_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_input_device_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopInputDeviceRequest, String> {
    let mut input = StopInputDeviceRequest::default();
    for (name, value) in labels {
        match *name {
            "InputDeviceId" => {
                input.input_device_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_multiplex_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopMultiplexRequest, String> {
    let mut input = StopMultiplexRequest::default();
    for (name, value) in labels {
        match *name {
            "MultiplexId" => {
                input.multiplex_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_transfer_input_device_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TransferInputDeviceRequest, String> {
    let mut input = TransferInputDeviceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TransferInputDeviceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TransferInputDevice request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InputDeviceId" => {
                input.input_device_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_account_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAccountConfigurationRequest, String> {
    let mut input = UpdateAccountConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAccountConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateAccountConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateChannelRequest, String> {
    let mut input = UpdateChannelRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateChannelRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateChannel request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ChannelId" => {
                input.channel_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_channel_class_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateChannelClassRequest, String> {
    let mut input = UpdateChannelClassRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateChannelClassRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateChannelClass request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ChannelId" => {
                input.channel_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_channel_placement_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateChannelPlacementGroupRequest, String> {
    let mut input = UpdateChannelPlacementGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateChannelPlacementGroupRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateChannelPlacementGroup request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ChannelPlacementGroupId" => {
                input.channel_placement_group_id = value.to_string();
            }
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_cloud_watch_alarm_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCloudWatchAlarmTemplateRequest, String> {
    let mut input = UpdateCloudWatchAlarmTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCloudWatchAlarmTemplateRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateCloudWatchAlarmTemplate request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_cloud_watch_alarm_template_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCloudWatchAlarmTemplateGroupRequest, String> {
    let mut input = UpdateCloudWatchAlarmTemplateGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCloudWatchAlarmTemplateGroupRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateCloudWatchAlarmTemplateGroup request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_cluster_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateClusterRequest, String> {
    let mut input = UpdateClusterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateClusterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateCluster request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_event_bridge_rule_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateEventBridgeRuleTemplateRequest, String> {
    let mut input = UpdateEventBridgeRuleTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateEventBridgeRuleTemplateRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateEventBridgeRuleTemplate request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_event_bridge_rule_template_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateEventBridgeRuleTemplateGroupRequest, String> {
    let mut input = UpdateEventBridgeRuleTemplateGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateEventBridgeRuleTemplateGroupRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateEventBridgeRuleTemplateGroup request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "Identifier" => {
                input.identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_input_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateInputRequest, String> {
    let mut input = UpdateInputRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateInputRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateInput request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InputId" => {
                input.input_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_input_device_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateInputDeviceRequest, String> {
    let mut input = UpdateInputDeviceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateInputDeviceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateInputDevice request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "InputDeviceId" => {
                input.input_device_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_input_security_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateInputSecurityGroupRequest, String> {
    let mut input = UpdateInputSecurityGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateInputSecurityGroupRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateInputSecurityGroup request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "InputSecurityGroupId" => {
                input.input_security_group_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_multiplex_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateMultiplexRequest, String> {
    let mut input = UpdateMultiplexRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateMultiplexRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateMultiplex request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "MultiplexId" => {
                input.multiplex_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_multiplex_program_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateMultiplexProgramRequest, String> {
    let mut input = UpdateMultiplexProgramRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateMultiplexProgramRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateMultiplexProgram request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "MultiplexId" => {
                input.multiplex_id = value.to_string();
            }
            "ProgramName" => {
                input.program_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_network_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateNetworkRequest, String> {
    let mut input = UpdateNetworkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateNetworkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateNetwork request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "NetworkId" => {
                input.network_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_node_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateNodeRequest, String> {
    let mut input = UpdateNodeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateNodeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateNode request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            "NodeId" => {
                input.node_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_node_state_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateNodeStateRequest, String> {
    let mut input = UpdateNodeStateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateNodeStateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateNodeState request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ClusterId" => {
                input.cluster_id = value.to_string();
            }
            "NodeId" => {
                input.node_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_reservation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateReservationRequest, String> {
    let mut input = UpdateReservationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateReservationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateReservation request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ReservationId" => {
                input.reservation_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_sdi_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSdiSourceRequest, String> {
    let mut input = UpdateSdiSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSdiSourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateSdiSource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "SdiSourceId" => {
                input.sdi_source_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
