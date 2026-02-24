//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-mediaconnect

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
pub fn serialize_add_bridge_outputs_response(result: &AddBridgeOutputsResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_add_bridge_sources_response(result: &AddBridgeSourcesResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_add_flow_media_streams_response(
    result: &AddFlowMediaStreamsResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_add_flow_outputs_response(result: &AddFlowOutputsResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_add_flow_sources_response(result: &AddFlowSourcesResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_add_flow_vpc_interfaces_response(
    result: &AddFlowVpcInterfacesResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_router_input_response(
    result: &BatchGetRouterInputResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_router_network_interface_response(
    result: &BatchGetRouterNetworkInterfaceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_router_output_response(
    result: &BatchGetRouterOutputResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_bridge_response(result: &CreateBridgeResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_flow_response(result: &CreateFlowResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_gateway_response(result: &CreateGatewayResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_router_input_response(result: &CreateRouterInputResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_router_network_interface_response(
    result: &CreateRouterNetworkInterfaceResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_router_output_response(
    result: &CreateRouterOutputResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_bridge_response(result: &DeleteBridgeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_flow_response(result: &DeleteFlowResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_gateway_response(result: &DeleteGatewayResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_router_input_response(result: &DeleteRouterInputResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_router_network_interface_response(
    result: &DeleteRouterNetworkInterfaceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_router_output_response(
    result: &DeleteRouterOutputResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_deregister_gateway_instance_response(
    result: &DeregisterGatewayInstanceResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_bridge_response(result: &DescribeBridgeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_flow_response(result: &DescribeFlowResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_flow_source_metadata_response(
    result: &DescribeFlowSourceMetadataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_flow_source_thumbnail_response(
    result: &DescribeFlowSourceThumbnailResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_gateway_response(result: &DescribeGatewayResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_gateway_instance_response(
    result: &DescribeGatewayInstanceResponse,
) -> MockResponse {
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
pub fn serialize_get_router_input_response(result: &GetRouterInputResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_router_input_source_metadata_response(
    result: &GetRouterInputSourceMetadataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_router_input_thumbnail_response(
    result: &GetRouterInputThumbnailResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_router_network_interface_response(
    result: &GetRouterNetworkInterfaceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_router_output_response(result: &GetRouterOutputResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_grant_flow_entitlements_response(
    result: &GrantFlowEntitlementsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_bridges_response(result: &ListBridgesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_entitlements_response(result: &ListEntitlementsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_flows_response(result: &ListFlowsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_gateway_instances_response(
    result: &ListGatewayInstancesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_gateways_response(result: &ListGatewaysResponse) -> MockResponse {
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
pub fn serialize_list_router_inputs_response(result: &ListRouterInputsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_router_network_interfaces_response(
    result: &ListRouterNetworkInterfacesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_router_outputs_response(result: &ListRouterOutputsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_global_resource_response(
    result: &ListTagsForGlobalResourceResponse,
) -> MockResponse {
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
pub fn serialize_purchase_offering_response(result: &PurchaseOfferingResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_bridge_output_response(
    result: &RemoveBridgeOutputResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_bridge_source_response(
    result: &RemoveBridgeSourceResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_flow_media_stream_response(
    result: &RemoveFlowMediaStreamResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_flow_output_response(result: &RemoveFlowOutputResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_flow_source_response(result: &RemoveFlowSourceResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_flow_vpc_interface_response(
    result: &RemoveFlowVpcInterfaceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_restart_router_input_response(
    result: &RestartRouterInputResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_restart_router_output_response(
    result: &RestartRouterOutputResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_revoke_flow_entitlement_response(
    result: &RevokeFlowEntitlementResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_flow_response(result: &StartFlowResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_router_input_response(result: &StartRouterInputResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_router_output_response(result: &StartRouterOutputResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_flow_response(result: &StopFlowResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_router_input_response(result: &StopRouterInputResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_router_output_response(result: &StopRouterOutputResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_tag_global_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_take_router_input_response(result: &TakeRouterInputResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_untag_global_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_bridge_response(result: &UpdateBridgeResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_bridge_output_response(
    result: &UpdateBridgeOutputResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_bridge_source_response(
    result: &UpdateBridgeSourceResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_bridge_state_response(result: &UpdateBridgeStateResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_flow_response(result: &UpdateFlowResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_flow_entitlement_response(
    result: &UpdateFlowEntitlementResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_flow_media_stream_response(
    result: &UpdateFlowMediaStreamResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_flow_output_response(result: &UpdateFlowOutputResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_flow_source_response(result: &UpdateFlowSourceResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_gateway_instance_response(
    result: &UpdateGatewayInstanceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_router_input_response(result: &UpdateRouterInputResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_router_network_interface_response(
    result: &UpdateRouterNetworkInterfaceResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_router_output_response(
    result: &UpdateRouterOutputResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_bridge_outputs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddBridgeOutputsRequest, String> {
    let mut input = AddBridgeOutputsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddBridgeOutputsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AddBridgeOutputs request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "BridgeArn" => {
                input.bridge_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_bridge_sources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddBridgeSourcesRequest, String> {
    let mut input = AddBridgeSourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddBridgeSourcesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AddBridgeSources request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "BridgeArn" => {
                input.bridge_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_flow_media_streams_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddFlowMediaStreamsRequest, String> {
    let mut input = AddFlowMediaStreamsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddFlowMediaStreamsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AddFlowMediaStreams request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_flow_outputs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddFlowOutputsRequest, String> {
    let mut input = AddFlowOutputsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddFlowOutputsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AddFlowOutputs request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_flow_sources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddFlowSourcesRequest, String> {
    let mut input = AddFlowSourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddFlowSourcesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AddFlowSources request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_flow_vpc_interfaces_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddFlowVpcInterfacesRequest, String> {
    let mut input = AddFlowVpcInterfacesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddFlowVpcInterfacesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AddFlowVpcInterfaces request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_router_input_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetRouterInputRequest, String> {
    let mut input = BatchGetRouterInputRequest::default();
    if let Some(value) = query.get("arns") {
        input.arns = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_router_network_interface_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetRouterNetworkInterfaceRequest, String> {
    let mut input = BatchGetRouterNetworkInterfaceRequest::default();
    if let Some(value) = query.get("arns") {
        input.arns = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_router_output_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetRouterOutputRequest, String> {
    let mut input = BatchGetRouterOutputRequest::default();
    if let Some(value) = query.get("arns") {
        input.arns = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_bridge_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBridgeRequest, String> {
    let mut input = CreateBridgeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBridgeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBridge request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFlowRequest, String> {
    let mut input = CreateFlowRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFlowRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFlow request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateGatewayRequest, String> {
    let mut input = CreateGatewayRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateGatewayRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateGateway request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_router_input_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRouterInputRequest, String> {
    let mut input = CreateRouterInputRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRouterInputRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateRouterInput request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_router_network_interface_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRouterNetworkInterfaceRequest, String> {
    let mut input = CreateRouterNetworkInterfaceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRouterNetworkInterfaceRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateRouterNetworkInterface request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_router_output_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRouterOutputRequest, String> {
    let mut input = CreateRouterOutputRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRouterOutputRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateRouterOutput request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_bridge_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBridgeRequest, String> {
    let mut input = DeleteBridgeRequest::default();
    for (name, value) in labels {
        match *name {
            "BridgeArn" => {
                input.bridge_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFlowRequest, String> {
    let mut input = DeleteFlowRequest::default();
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteGatewayRequest, String> {
    let mut input = DeleteGatewayRequest::default();
    for (name, value) in labels {
        match *name {
            "GatewayArn" => {
                input.gateway_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_router_input_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRouterInputRequest, String> {
    let mut input = DeleteRouterInputRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_router_network_interface_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRouterNetworkInterfaceRequest, String> {
    let mut input = DeleteRouterNetworkInterfaceRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_router_output_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRouterOutputRequest, String> {
    let mut input = DeleteRouterOutputRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_deregister_gateway_instance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeregisterGatewayInstanceRequest, String> {
    let mut input = DeregisterGatewayInstanceRequest::default();
    for (name, value) in labels {
        match *name {
            "GatewayInstanceArn" => {
                input.gateway_instance_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("force") {
        input.force = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_bridge_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBridgeRequest, String> {
    let mut input = DescribeBridgeRequest::default();
    for (name, value) in labels {
        match *name {
            "BridgeArn" => {
                input.bridge_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeFlowRequest, String> {
    let mut input = DescribeFlowRequest::default();
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_flow_source_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeFlowSourceMetadataRequest, String> {
    let mut input = DescribeFlowSourceMetadataRequest::default();
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_flow_source_thumbnail_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeFlowSourceThumbnailRequest, String> {
    let mut input = DescribeFlowSourceThumbnailRequest::default();
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeGatewayRequest, String> {
    let mut input = DescribeGatewayRequest::default();
    for (name, value) in labels {
        match *name {
            "GatewayArn" => {
                input.gateway_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_gateway_instance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeGatewayInstanceRequest, String> {
    let mut input = DescribeGatewayInstanceRequest::default();
    for (name, value) in labels {
        match *name {
            "GatewayInstanceArn" => {
                input.gateway_instance_arn = value.to_string();
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
            "OfferingArn" => {
                input.offering_arn = value.to_string();
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
            "ReservationArn" => {
                input.reservation_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_router_input_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRouterInputRequest, String> {
    let mut input = GetRouterInputRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_router_input_source_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRouterInputSourceMetadataRequest, String> {
    let mut input = GetRouterInputSourceMetadataRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_router_input_thumbnail_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRouterInputThumbnailRequest, String> {
    let mut input = GetRouterInputThumbnailRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_router_network_interface_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRouterNetworkInterfaceRequest, String> {
    let mut input = GetRouterNetworkInterfaceRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_router_output_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRouterOutputRequest, String> {
    let mut input = GetRouterOutputRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_grant_flow_entitlements_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GrantFlowEntitlementsRequest, String> {
    let mut input = GrantFlowEntitlementsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GrantFlowEntitlementsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GrantFlowEntitlements request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_bridges_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBridgesRequest, String> {
    let mut input = ListBridgesRequest::default();
    if let Some(value) = query.get("filterArn") {
        input.filter_arn = Some(value.to_string());
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
pub fn deserialize_list_entitlements_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEntitlementsRequest, String> {
    let mut input = ListEntitlementsRequest::default();
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
pub fn deserialize_list_flows_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFlowsRequest, String> {
    let mut input = ListFlowsRequest::default();
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
pub fn deserialize_list_gateway_instances_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGatewayInstancesRequest, String> {
    let mut input = ListGatewayInstancesRequest::default();
    if let Some(value) = query.get("filterArn") {
        input.filter_arn = Some(value.to_string());
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
pub fn deserialize_list_gateways_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGatewaysRequest, String> {
    let mut input = ListGatewaysRequest::default();
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
pub fn deserialize_list_reservations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListReservationsRequest, String> {
    let mut input = ListReservationsRequest::default();
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
pub fn deserialize_list_router_inputs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRouterInputsRequest, String> {
    let mut input = ListRouterInputsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListRouterInputsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListRouterInputs request: {err}"))?;
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
pub fn deserialize_list_router_network_interfaces_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRouterNetworkInterfacesRequest, String> {
    let mut input = ListRouterNetworkInterfacesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListRouterNetworkInterfacesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListRouterNetworkInterfaces request: {err}")
            })?;
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
pub fn deserialize_list_router_outputs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRouterOutputsRequest, String> {
    let mut input = ListRouterOutputsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListRouterOutputsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListRouterOutputs request: {err}"))?;
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
pub fn deserialize_list_tags_for_global_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForGlobalResourceRequest, String> {
    let mut input = ListTagsForGlobalResourceRequest::default();
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
            "OfferingArn" => {
                input.offering_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_bridge_output_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveBridgeOutputRequest, String> {
    let mut input = RemoveBridgeOutputRequest::default();
    for (name, value) in labels {
        match *name {
            "BridgeArn" => {
                input.bridge_arn = value.to_string();
            }
            "OutputName" => {
                input.output_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_bridge_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveBridgeSourceRequest, String> {
    let mut input = RemoveBridgeSourceRequest::default();
    for (name, value) in labels {
        match *name {
            "BridgeArn" => {
                input.bridge_arn = value.to_string();
            }
            "SourceName" => {
                input.source_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_flow_media_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveFlowMediaStreamRequest, String> {
    let mut input = RemoveFlowMediaStreamRequest::default();
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            "MediaStreamName" => {
                input.media_stream_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_flow_output_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveFlowOutputRequest, String> {
    let mut input = RemoveFlowOutputRequest::default();
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            "OutputArn" => {
                input.output_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_flow_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveFlowSourceRequest, String> {
    let mut input = RemoveFlowSourceRequest::default();
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            "SourceArn" => {
                input.source_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_flow_vpc_interface_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveFlowVpcInterfaceRequest, String> {
    let mut input = RemoveFlowVpcInterfaceRequest::default();
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            "VpcInterfaceName" => {
                input.vpc_interface_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_restart_router_input_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RestartRouterInputRequest, String> {
    let mut input = RestartRouterInputRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_restart_router_output_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RestartRouterOutputRequest, String> {
    let mut input = RestartRouterOutputRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_revoke_flow_entitlement_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RevokeFlowEntitlementRequest, String> {
    let mut input = RevokeFlowEntitlementRequest::default();
    for (name, value) in labels {
        match *name {
            "EntitlementArn" => {
                input.entitlement_arn = value.to_string();
            }
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartFlowRequest, String> {
    let mut input = StartFlowRequest::default();
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_router_input_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartRouterInputRequest, String> {
    let mut input = StartRouterInputRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_router_output_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartRouterOutputRequest, String> {
    let mut input = StartRouterOutputRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopFlowRequest, String> {
    let mut input = StopFlowRequest::default();
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_router_input_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopRouterInputRequest, String> {
    let mut input = StopRouterInputRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_router_output_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopRouterOutputRequest, String> {
    let mut input = StopRouterOutputRequest::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_global_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagGlobalResourceRequest, String> {
    let mut input = TagGlobalResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagGlobalResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TagGlobalResource request: {err}"))?;
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
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_take_router_input_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TakeRouterInputRequest, String> {
    let mut input = TakeRouterInputRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TakeRouterInputRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TakeRouterInput request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "RouterOutputArn" => {
                input.router_output_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_global_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagGlobalResourceRequest, String> {
    let mut input = UntagGlobalResourceRequest::default();
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
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
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
pub fn deserialize_update_bridge_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBridgeRequest, String> {
    let mut input = UpdateBridgeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBridgeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBridge request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "BridgeArn" => {
                input.bridge_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_bridge_output_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBridgeOutputRequest, String> {
    let mut input = UpdateBridgeOutputRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBridgeOutputRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBridgeOutput request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "BridgeArn" => {
                input.bridge_arn = value.to_string();
            }
            "OutputName" => {
                input.output_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_bridge_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBridgeSourceRequest, String> {
    let mut input = UpdateBridgeSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBridgeSourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBridgeSource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "BridgeArn" => {
                input.bridge_arn = value.to_string();
            }
            "SourceName" => {
                input.source_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_bridge_state_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBridgeStateRequest, String> {
    let mut input = UpdateBridgeStateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBridgeStateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBridgeState request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "BridgeArn" => {
                input.bridge_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFlowRequest, String> {
    let mut input = UpdateFlowRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFlowRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFlow request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_flow_entitlement_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFlowEntitlementRequest, String> {
    let mut input = UpdateFlowEntitlementRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFlowEntitlementRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFlowEntitlement request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "EntitlementArn" => {
                input.entitlement_arn = value.to_string();
            }
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_flow_media_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFlowMediaStreamRequest, String> {
    let mut input = UpdateFlowMediaStreamRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFlowMediaStreamRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFlowMediaStream request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            "MediaStreamName" => {
                input.media_stream_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_flow_output_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFlowOutputRequest, String> {
    let mut input = UpdateFlowOutputRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFlowOutputRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFlowOutput request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            "OutputArn" => {
                input.output_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_flow_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFlowSourceRequest, String> {
    let mut input = UpdateFlowSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFlowSourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFlowSource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "FlowArn" => {
                input.flow_arn = value.to_string();
            }
            "SourceArn" => {
                input.source_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_gateway_instance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateGatewayInstanceRequest, String> {
    let mut input = UpdateGatewayInstanceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateGatewayInstanceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateGatewayInstance request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GatewayInstanceArn" => {
                input.gateway_instance_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_router_input_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRouterInputRequest, String> {
    let mut input = UpdateRouterInputRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRouterInputRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateRouterInput request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_router_network_interface_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRouterNetworkInterfaceRequest, String> {
    let mut input = UpdateRouterNetworkInterfaceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRouterNetworkInterfaceRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateRouterNetworkInterface request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_router_output_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRouterOutputRequest, String> {
    let mut input = UpdateRouterOutputRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRouterOutputRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateRouterOutput request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
