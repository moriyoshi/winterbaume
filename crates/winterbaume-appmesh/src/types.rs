use chrono::{DateTime, Utc};

use crate::wire::{
    GatewayRouteSpec, RouteSpec, VirtualGatewaySpec, VirtualNodeSpec, VirtualRouterSpec,
    VirtualServiceSpec,
};

#[derive(Debug, Clone)]
pub struct Mesh {
    pub mesh_name: String,
    pub arn: String,
    pub uid: String,
    pub status: String,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub mesh_owner: String,
    pub resource_owner: String,
    /// Egress filter type: "ALLOW_ALL" or "DROP_ALL" (default)
    pub egress_filter_type: String,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct VirtualRouter {
    pub mesh_name: String,
    pub virtual_router_name: String,
    pub arn: String,
    pub uid: String,
    pub status: String,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub mesh_owner: String,
    pub resource_owner: String,
    pub spec: VirtualRouterSpec,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct VirtualNode {
    pub mesh_name: String,
    pub virtual_node_name: String,
    pub arn: String,
    pub uid: String,
    pub status: String,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub mesh_owner: String,
    pub resource_owner: String,
    pub spec: VirtualNodeSpec,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct VirtualGateway {
    pub mesh_name: String,
    pub virtual_gateway_name: String,
    pub arn: String,
    pub uid: String,
    pub status: String,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub mesh_owner: String,
    pub resource_owner: String,
    pub spec: VirtualGatewaySpec,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct VirtualService {
    pub mesh_name: String,
    pub virtual_service_name: String,
    pub arn: String,
    pub uid: String,
    pub status: String,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub mesh_owner: String,
    pub resource_owner: String,
    pub spec: VirtualServiceSpec,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub mesh_name: String,
    pub virtual_router_name: String,
    pub route_name: String,
    pub arn: String,
    pub uid: String,
    pub status: String,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub mesh_owner: String,
    pub resource_owner: String,
    pub spec: RouteSpec,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct GatewayRoute {
    pub mesh_name: String,
    pub virtual_gateway_name: String,
    pub gateway_route_name: String,
    pub arn: String,
    pub uid: String,
    pub status: String,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub mesh_owner: String,
    pub resource_owner: String,
    pub spec: GatewayRouteSpec,
    pub tags: Vec<(String, String)>,
}
