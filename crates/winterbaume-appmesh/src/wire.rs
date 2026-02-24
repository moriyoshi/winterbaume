//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-appmesh

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
pub fn serialize_create_gateway_route_response(result: &CreateGatewayRouteOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.gateway_route).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_mesh_response(result: &CreateMeshOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.mesh).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_route_response(result: &CreateRouteOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.route).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_virtual_gateway_response(
    result: &CreateVirtualGatewayOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_gateway).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_virtual_node_response(result: &CreateVirtualNodeOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_node).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_virtual_router_response(
    result: &CreateVirtualRouterOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_router).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_virtual_service_response(
    result: &CreateVirtualServiceOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_service).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_gateway_route_response(result: &DeleteGatewayRouteOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.gateway_route).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_mesh_response(result: &DeleteMeshOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.mesh).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_route_response(result: &DeleteRouteOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.route).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_virtual_gateway_response(
    result: &DeleteVirtualGatewayOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_gateway).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_virtual_node_response(result: &DeleteVirtualNodeOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_node).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_virtual_router_response(
    result: &DeleteVirtualRouterOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_router).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_virtual_service_response(
    result: &DeleteVirtualServiceOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_service).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_gateway_route_response(
    result: &DescribeGatewayRouteOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.gateway_route).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_mesh_response(result: &DescribeMeshOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.mesh).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_route_response(result: &DescribeRouteOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.route).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_virtual_gateway_response(
    result: &DescribeVirtualGatewayOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_gateway).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_virtual_node_response(
    result: &DescribeVirtualNodeOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_node).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_virtual_router_response(
    result: &DescribeVirtualRouterOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_router).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_virtual_service_response(
    result: &DescribeVirtualServiceOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_service).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_gateway_routes_response(result: &ListGatewayRoutesOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_meshes_response(result: &ListMeshesOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_routes_response(result: &ListRoutesOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_virtual_gateways_response(
    result: &ListVirtualGatewaysOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_virtual_nodes_response(result: &ListVirtualNodesOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_virtual_routers_response(result: &ListVirtualRoutersOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_virtual_services_response(
    result: &ListVirtualServicesOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_gateway_route_response(result: &UpdateGatewayRouteOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.gateway_route).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_mesh_response(result: &UpdateMeshOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.mesh).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_route_response(result: &UpdateRouteOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.route).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_virtual_gateway_response(
    result: &UpdateVirtualGatewayOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_gateway).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_virtual_node_response(result: &UpdateVirtualNodeOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_node).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_virtual_router_response(
    result: &UpdateVirtualRouterOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_router).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_virtual_service_response(
    result: &UpdateVirtualServiceOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.virtual_service).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_gateway_route_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateGatewayRouteInput, String> {
    let mut input = CreateGatewayRouteInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateGatewayRouteInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateGatewayRoute request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualGatewayName" => {
                input.virtual_gateway_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_mesh_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMeshInput, String> {
    let mut input = CreateMeshInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateMeshInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateMesh request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_route_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRouteInput, String> {
    let mut input = CreateRouteInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRouteInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateRoute request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualRouterName" => {
                input.virtual_router_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_virtual_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateVirtualGatewayInput, String> {
    let mut input = CreateVirtualGatewayInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateVirtualGatewayInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateVirtualGateway request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_virtual_node_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateVirtualNodeInput, String> {
    let mut input = CreateVirtualNodeInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateVirtualNodeInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateVirtualNode request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_virtual_router_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateVirtualRouterInput, String> {
    let mut input = CreateVirtualRouterInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateVirtualRouterInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateVirtualRouter request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_virtual_service_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateVirtualServiceInput, String> {
    let mut input = CreateVirtualServiceInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateVirtualServiceInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateVirtualService request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_gateway_route_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteGatewayRouteInput, String> {
    let mut input = DeleteGatewayRouteInput::default();
    for (name, value) in labels {
        match *name {
            "gatewayRouteName" => {
                input.gateway_route_name = value.to_string();
            }
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualGatewayName" => {
                input.virtual_gateway_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_mesh_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMeshInput, String> {
    let mut input = DeleteMeshInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_route_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRouteInput, String> {
    let mut input = DeleteRouteInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "routeName" => {
                input.route_name = value.to_string();
            }
            "virtualRouterName" => {
                input.virtual_router_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_virtual_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVirtualGatewayInput, String> {
    let mut input = DeleteVirtualGatewayInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualGatewayName" => {
                input.virtual_gateway_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_virtual_node_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVirtualNodeInput, String> {
    let mut input = DeleteVirtualNodeInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualNodeName" => {
                input.virtual_node_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_virtual_router_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVirtualRouterInput, String> {
    let mut input = DeleteVirtualRouterInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualRouterName" => {
                input.virtual_router_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_virtual_service_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVirtualServiceInput, String> {
    let mut input = DeleteVirtualServiceInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualServiceName" => {
                input.virtual_service_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_gateway_route_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeGatewayRouteInput, String> {
    let mut input = DescribeGatewayRouteInput::default();
    for (name, value) in labels {
        match *name {
            "gatewayRouteName" => {
                input.gateway_route_name = value.to_string();
            }
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualGatewayName" => {
                input.virtual_gateway_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_mesh_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeMeshInput, String> {
    let mut input = DescribeMeshInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_route_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeRouteInput, String> {
    let mut input = DescribeRouteInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "routeName" => {
                input.route_name = value.to_string();
            }
            "virtualRouterName" => {
                input.virtual_router_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_virtual_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeVirtualGatewayInput, String> {
    let mut input = DescribeVirtualGatewayInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualGatewayName" => {
                input.virtual_gateway_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_virtual_node_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeVirtualNodeInput, String> {
    let mut input = DescribeVirtualNodeInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualNodeName" => {
                input.virtual_node_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_virtual_router_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeVirtualRouterInput, String> {
    let mut input = DescribeVirtualRouterInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualRouterName" => {
                input.virtual_router_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_virtual_service_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeVirtualServiceInput, String> {
    let mut input = DescribeVirtualServiceInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualServiceName" => {
                input.virtual_service_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_gateway_routes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGatewayRoutesInput, String> {
    let mut input = ListGatewayRoutesInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualGatewayName" => {
                input.virtual_gateway_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_meshes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMeshesInput, String> {
    let mut input = ListMeshesInput::default();
    if let Some(value) = query.get("limit") {
        input.limit = Some(
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
pub fn deserialize_list_routes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRoutesInput, String> {
    let mut input = ListRoutesInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualRouterName" => {
                input.virtual_router_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
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
) -> Result<ListTagsForResourceInput, String> {
    let mut input = ListTagsForResourceInput::default();
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_virtual_gateways_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVirtualGatewaysInput, String> {
    let mut input = ListVirtualGatewaysInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_virtual_nodes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVirtualNodesInput, String> {
    let mut input = ListVirtualNodesInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_virtual_routers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVirtualRoutersInput, String> {
    let mut input = ListVirtualRoutersInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_virtual_services_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListVirtualServicesInput, String> {
    let mut input = ListVirtualServicesInput::default();
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceInput, String> {
    let mut input = TagResourceInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceInput>(&request.body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    if let Some(value) = query.get("resourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceInput, String> {
    let mut input = UntagResourceInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UntagResourceInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UntagResource request: {err}"))?;
    }
    if let Some(value) = query.get("resourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_gateway_route_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateGatewayRouteInput, String> {
    let mut input = UpdateGatewayRouteInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateGatewayRouteInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateGatewayRoute request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "gatewayRouteName" => {
                input.gateway_route_name = value.to_string();
            }
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualGatewayName" => {
                input.virtual_gateway_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_mesh_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateMeshInput, String> {
    let mut input = UpdateMeshInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateMeshInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateMesh request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_route_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRouteInput, String> {
    let mut input = UpdateRouteInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRouteInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateRoute request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "routeName" => {
                input.route_name = value.to_string();
            }
            "virtualRouterName" => {
                input.virtual_router_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_virtual_gateway_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateVirtualGatewayInput, String> {
    let mut input = UpdateVirtualGatewayInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateVirtualGatewayInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateVirtualGateway request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualGatewayName" => {
                input.virtual_gateway_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_virtual_node_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateVirtualNodeInput, String> {
    let mut input = UpdateVirtualNodeInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateVirtualNodeInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateVirtualNode request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualNodeName" => {
                input.virtual_node_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_virtual_router_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateVirtualRouterInput, String> {
    let mut input = UpdateVirtualRouterInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateVirtualRouterInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateVirtualRouter request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualRouterName" => {
                input.virtual_router_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_virtual_service_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateVirtualServiceInput, String> {
    let mut input = UpdateVirtualServiceInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateVirtualServiceInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateVirtualService request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "meshName" => {
                input.mesh_name = value.to_string();
            }
            "virtualServiceName" => {
                input.virtual_service_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("meshOwner") {
        input.mesh_owner = Some(value.to_string());
    }
    Ok(input)
}
