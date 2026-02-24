//! Serde-compatible view types for App Mesh state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AppMeshService;
use crate::state::AppMeshState;
use crate::types::{
    GatewayRoute, Mesh, Route, VirtualGateway, VirtualNode, VirtualRouter, VirtualService,
};
use crate::wire::{
    GatewayRouteSpec, RouteSpec, VirtualGatewaySpec, VirtualNodeSpec, VirtualRouterSpec,
    VirtualServiceSpec,
};

fn parse_datetime(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

/// Serializable view of the entire App Mesh state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppMeshStateView {
    #[serde(default)]
    pub meshes: HashMap<String, MeshView>,
    #[serde(default)]
    pub virtual_routers: HashMap<String, VirtualRouterView>,
    #[serde(default)]
    pub virtual_nodes: HashMap<String, VirtualNodeView>,
    #[serde(default)]
    pub virtual_gateways: HashMap<String, VirtualGatewayView>,
    #[serde(default)]
    pub virtual_services: HashMap<String, VirtualServiceView>,
    #[serde(default)]
    pub routes: HashMap<String, RouteView>,
    #[serde(default)]
    pub gateway_routes: HashMap<String, GatewayRouteView>,
    #[serde(default)]
    pub tags: HashMap<String, Vec<(String, String)>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshView {
    pub mesh_name: String,
    pub arn: String,
    pub uid: String,
    pub status: String,
    pub version: i64,
    pub created_at: String,
    pub last_updated_at: String,
    pub mesh_owner: String,
    pub resource_owner: String,
    pub egress_filter_type: String,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualRouterView {
    pub mesh_name: String,
    pub virtual_router_name: String,
    pub arn: String,
    pub uid: String,
    pub status: String,
    pub version: i64,
    pub created_at: String,
    pub last_updated_at: String,
    pub mesh_owner: String,
    pub resource_owner: String,
    pub spec: VirtualRouterSpec,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualNodeView {
    pub mesh_name: String,
    pub virtual_node_name: String,
    pub arn: String,
    pub uid: String,
    pub status: String,
    pub version: i64,
    pub created_at: String,
    pub last_updated_at: String,
    pub mesh_owner: String,
    pub resource_owner: String,
    pub spec: VirtualNodeSpec,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualGatewayView {
    pub mesh_name: String,
    pub virtual_gateway_name: String,
    pub arn: String,
    pub uid: String,
    pub status: String,
    pub version: i64,
    pub created_at: String,
    pub last_updated_at: String,
    pub mesh_owner: String,
    pub resource_owner: String,
    pub spec: VirtualGatewaySpec,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualServiceView {
    pub mesh_name: String,
    pub virtual_service_name: String,
    pub arn: String,
    pub uid: String,
    pub status: String,
    pub version: i64,
    pub created_at: String,
    pub last_updated_at: String,
    pub mesh_owner: String,
    pub resource_owner: String,
    pub spec: VirtualServiceSpec,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteView {
    pub mesh_name: String,
    pub virtual_router_name: String,
    pub route_name: String,
    pub arn: String,
    pub uid: String,
    pub status: String,
    pub version: i64,
    pub created_at: String,
    pub last_updated_at: String,
    pub mesh_owner: String,
    pub resource_owner: String,
    pub spec: RouteSpec,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayRouteView {
    pub mesh_name: String,
    pub virtual_gateway_name: String,
    pub gateway_route_name: String,
    pub arn: String,
    pub uid: String,
    pub status: String,
    pub version: i64,
    pub created_at: String,
    pub last_updated_at: String,
    pub mesh_owner: String,
    pub resource_owner: String,
    pub spec: GatewayRouteSpec,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

// ===== From internal state to view =====

impl From<&AppMeshState> for AppMeshStateView {
    fn from(state: &AppMeshState) -> Self {
        AppMeshStateView {
            meshes: state
                .meshes
                .iter()
                .map(|(k, v)| (k.clone(), MeshView::from(v)))
                .collect(),
            virtual_routers: state
                .virtual_routers
                .iter()
                .map(|(k, v)| {
                    let key = format!("{}/{}", k.0, k.1);
                    (key, VirtualRouterView::from(v))
                })
                .collect(),
            virtual_nodes: state
                .virtual_nodes
                .iter()
                .map(|(k, v)| {
                    let key = format!("{}/{}", k.0, k.1);
                    (key, VirtualNodeView::from(v))
                })
                .collect(),
            virtual_gateways: state
                .virtual_gateways
                .iter()
                .map(|(k, v)| {
                    let key = format!("{}/{}", k.0, k.1);
                    (key, VirtualGatewayView::from(v))
                })
                .collect(),
            virtual_services: state
                .virtual_services
                .iter()
                .map(|(k, v)| {
                    let key = format!("{}/{}", k.0, k.1);
                    (key, VirtualServiceView::from(v))
                })
                .collect(),
            routes: state
                .routes
                .iter()
                .map(|(k, v)| {
                    let key = format!("{}/{}/{}", k.0, k.1, k.2);
                    (key, RouteView::from(v))
                })
                .collect(),
            gateway_routes: state
                .gateway_routes
                .iter()
                .map(|(k, v)| {
                    let key = format!("{}/{}/{}", k.0, k.1, k.2);
                    (key, GatewayRouteView::from(v))
                })
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<&Mesh> for MeshView {
    fn from(m: &Mesh) -> Self {
        MeshView {
            mesh_name: m.mesh_name.clone(),
            arn: m.arn.clone(),
            uid: m.uid.clone(),
            status: m.status.clone(),
            version: m.version,
            created_at: m.created_at.to_rfc3339(),
            last_updated_at: m.last_updated_at.to_rfc3339(),
            mesh_owner: m.mesh_owner.clone(),
            resource_owner: m.resource_owner.clone(),
            egress_filter_type: m.egress_filter_type.clone(),
            tags: m.tags.clone(),
        }
    }
}

impl From<&VirtualRouter> for VirtualRouterView {
    fn from(vr: &VirtualRouter) -> Self {
        VirtualRouterView {
            mesh_name: vr.mesh_name.clone(),
            virtual_router_name: vr.virtual_router_name.clone(),
            arn: vr.arn.clone(),
            uid: vr.uid.clone(),
            status: vr.status.clone(),
            version: vr.version,
            created_at: vr.created_at.to_rfc3339(),
            last_updated_at: vr.last_updated_at.to_rfc3339(),
            mesh_owner: vr.mesh_owner.clone(),
            resource_owner: vr.resource_owner.clone(),
            spec: vr.spec.clone(),
            tags: vr.tags.clone(),
        }
    }
}

impl From<&VirtualNode> for VirtualNodeView {
    fn from(vn: &VirtualNode) -> Self {
        VirtualNodeView {
            mesh_name: vn.mesh_name.clone(),
            virtual_node_name: vn.virtual_node_name.clone(),
            arn: vn.arn.clone(),
            uid: vn.uid.clone(),
            status: vn.status.clone(),
            version: vn.version,
            created_at: vn.created_at.to_rfc3339(),
            last_updated_at: vn.last_updated_at.to_rfc3339(),
            mesh_owner: vn.mesh_owner.clone(),
            resource_owner: vn.resource_owner.clone(),
            spec: vn.spec.clone(),
            tags: vn.tags.clone(),
        }
    }
}

impl From<&VirtualGateway> for VirtualGatewayView {
    fn from(vg: &VirtualGateway) -> Self {
        VirtualGatewayView {
            mesh_name: vg.mesh_name.clone(),
            virtual_gateway_name: vg.virtual_gateway_name.clone(),
            arn: vg.arn.clone(),
            uid: vg.uid.clone(),
            status: vg.status.clone(),
            version: vg.version,
            created_at: vg.created_at.to_rfc3339(),
            last_updated_at: vg.last_updated_at.to_rfc3339(),
            mesh_owner: vg.mesh_owner.clone(),
            resource_owner: vg.resource_owner.clone(),
            spec: vg.spec.clone(),
            tags: vg.tags.clone(),
        }
    }
}

impl From<&VirtualService> for VirtualServiceView {
    fn from(vs: &VirtualService) -> Self {
        VirtualServiceView {
            mesh_name: vs.mesh_name.clone(),
            virtual_service_name: vs.virtual_service_name.clone(),
            arn: vs.arn.clone(),
            uid: vs.uid.clone(),
            status: vs.status.clone(),
            version: vs.version,
            created_at: vs.created_at.to_rfc3339(),
            last_updated_at: vs.last_updated_at.to_rfc3339(),
            mesh_owner: vs.mesh_owner.clone(),
            resource_owner: vs.resource_owner.clone(),
            spec: vs.spec.clone(),
            tags: vs.tags.clone(),
        }
    }
}

impl From<&Route> for RouteView {
    fn from(r: &Route) -> Self {
        RouteView {
            mesh_name: r.mesh_name.clone(),
            virtual_router_name: r.virtual_router_name.clone(),
            route_name: r.route_name.clone(),
            arn: r.arn.clone(),
            uid: r.uid.clone(),
            status: r.status.clone(),
            version: r.version,
            created_at: r.created_at.to_rfc3339(),
            last_updated_at: r.last_updated_at.to_rfc3339(),
            mesh_owner: r.mesh_owner.clone(),
            resource_owner: r.resource_owner.clone(),
            spec: r.spec.clone(),
            tags: r.tags.clone(),
        }
    }
}

impl From<&GatewayRoute> for GatewayRouteView {
    fn from(gr: &GatewayRoute) -> Self {
        GatewayRouteView {
            mesh_name: gr.mesh_name.clone(),
            virtual_gateway_name: gr.virtual_gateway_name.clone(),
            gateway_route_name: gr.gateway_route_name.clone(),
            arn: gr.arn.clone(),
            uid: gr.uid.clone(),
            status: gr.status.clone(),
            version: gr.version,
            created_at: gr.created_at.to_rfc3339(),
            last_updated_at: gr.last_updated_at.to_rfc3339(),
            mesh_owner: gr.mesh_owner.clone(),
            resource_owner: gr.resource_owner.clone(),
            spec: gr.spec.clone(),
            tags: gr.tags.clone(),
        }
    }
}

// ===== From view to internal state =====

impl From<AppMeshStateView> for AppMeshState {
    fn from(view: AppMeshStateView) -> Self {
        AppMeshState {
            meshes: view
                .meshes
                .into_values()
                .map(|v| (v.mesh_name.clone(), Mesh::from(v)))
                .collect(),
            virtual_routers: view
                .virtual_routers
                .into_values()
                .map(|v| {
                    let key = (v.mesh_name.clone(), v.virtual_router_name.clone());
                    (key, VirtualRouter::from(v))
                })
                .collect(),
            virtual_nodes: view
                .virtual_nodes
                .into_values()
                .map(|v| {
                    let key = (v.mesh_name.clone(), v.virtual_node_name.clone());
                    (key, VirtualNode::from(v))
                })
                .collect(),
            virtual_gateways: view
                .virtual_gateways
                .into_values()
                .map(|v| {
                    let key = (v.mesh_name.clone(), v.virtual_gateway_name.clone());
                    (key, VirtualGateway::from(v))
                })
                .collect(),
            virtual_services: view
                .virtual_services
                .into_values()
                .map(|v| {
                    let key = (v.mesh_name.clone(), v.virtual_service_name.clone());
                    (key, VirtualService::from(v))
                })
                .collect(),
            routes: view
                .routes
                .into_values()
                .map(|v| {
                    let key = (
                        v.mesh_name.clone(),
                        v.virtual_router_name.clone(),
                        v.route_name.clone(),
                    );
                    (key, Route::from(v))
                })
                .collect(),
            gateway_routes: view
                .gateway_routes
                .into_values()
                .map(|v| {
                    let key = (
                        v.mesh_name.clone(),
                        v.virtual_gateway_name.clone(),
                        v.gateway_route_name.clone(),
                    );
                    (key, GatewayRoute::from(v))
                })
                .collect(),
            tags: view.tags,
        }
    }
}

impl From<MeshView> for Mesh {
    fn from(v: MeshView) -> Self {
        Mesh {
            mesh_name: v.mesh_name,
            arn: v.arn,
            uid: v.uid,
            status: v.status,
            version: v.version,
            created_at: parse_datetime(&v.created_at),
            last_updated_at: parse_datetime(&v.last_updated_at),
            mesh_owner: v.mesh_owner,
            resource_owner: v.resource_owner,
            egress_filter_type: v.egress_filter_type,
            tags: v.tags,
        }
    }
}

impl From<VirtualRouterView> for VirtualRouter {
    fn from(v: VirtualRouterView) -> Self {
        VirtualRouter {
            mesh_name: v.mesh_name,
            virtual_router_name: v.virtual_router_name,
            arn: v.arn,
            uid: v.uid,
            status: v.status,
            version: v.version,
            created_at: parse_datetime(&v.created_at),
            last_updated_at: parse_datetime(&v.last_updated_at),
            mesh_owner: v.mesh_owner,
            resource_owner: v.resource_owner,
            spec: v.spec,
            tags: v.tags,
        }
    }
}

impl From<VirtualNodeView> for VirtualNode {
    fn from(v: VirtualNodeView) -> Self {
        VirtualNode {
            mesh_name: v.mesh_name,
            virtual_node_name: v.virtual_node_name,
            arn: v.arn,
            uid: v.uid,
            status: v.status,
            version: v.version,
            created_at: parse_datetime(&v.created_at),
            last_updated_at: parse_datetime(&v.last_updated_at),
            mesh_owner: v.mesh_owner,
            resource_owner: v.resource_owner,
            spec: v.spec,
            tags: v.tags,
        }
    }
}

impl From<VirtualGatewayView> for VirtualGateway {
    fn from(v: VirtualGatewayView) -> Self {
        VirtualGateway {
            mesh_name: v.mesh_name,
            virtual_gateway_name: v.virtual_gateway_name,
            arn: v.arn,
            uid: v.uid,
            status: v.status,
            version: v.version,
            created_at: parse_datetime(&v.created_at),
            last_updated_at: parse_datetime(&v.last_updated_at),
            mesh_owner: v.mesh_owner,
            resource_owner: v.resource_owner,
            spec: v.spec,
            tags: v.tags,
        }
    }
}

impl From<VirtualServiceView> for VirtualService {
    fn from(v: VirtualServiceView) -> Self {
        VirtualService {
            mesh_name: v.mesh_name,
            virtual_service_name: v.virtual_service_name,
            arn: v.arn,
            uid: v.uid,
            status: v.status,
            version: v.version,
            created_at: parse_datetime(&v.created_at),
            last_updated_at: parse_datetime(&v.last_updated_at),
            mesh_owner: v.mesh_owner,
            resource_owner: v.resource_owner,
            spec: v.spec,
            tags: v.tags,
        }
    }
}

impl From<RouteView> for Route {
    fn from(v: RouteView) -> Self {
        Route {
            mesh_name: v.mesh_name,
            virtual_router_name: v.virtual_router_name,
            route_name: v.route_name,
            arn: v.arn,
            uid: v.uid,
            status: v.status,
            version: v.version,
            created_at: parse_datetime(&v.created_at),
            last_updated_at: parse_datetime(&v.last_updated_at),
            mesh_owner: v.mesh_owner,
            resource_owner: v.resource_owner,
            spec: v.spec,
            tags: v.tags,
        }
    }
}

impl From<GatewayRouteView> for GatewayRoute {
    fn from(v: GatewayRouteView) -> Self {
        GatewayRoute {
            mesh_name: v.mesh_name,
            virtual_gateway_name: v.virtual_gateway_name,
            gateway_route_name: v.gateway_route_name,
            arn: v.arn,
            uid: v.uid,
            status: v.status,
            version: v.version,
            created_at: parse_datetime(&v.created_at),
            last_updated_at: parse_datetime(&v.last_updated_at),
            mesh_owner: v.mesh_owner,
            resource_owner: v.resource_owner,
            spec: v.spec,
            tags: v.tags,
        }
    }
}

// ===== StatefulService implementation =====

impl StatefulService for AppMeshService {
    type StateView = AppMeshStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AppMeshStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = AppMeshState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            let merged = AppMeshState::from(view);
            for (k, v) in merged.meshes {
                guard.meshes.insert(k, v);
            }
            for (k, v) in merged.virtual_routers {
                guard.virtual_routers.insert(k, v);
            }
            for (k, v) in merged.virtual_nodes {
                guard.virtual_nodes.insert(k, v);
            }
            for (k, v) in merged.virtual_gateways {
                guard.virtual_gateways.insert(k, v);
            }
            for (k, v) in merged.virtual_services {
                guard.virtual_services.insert(k, v);
            }
            for (k, v) in merged.routes {
                guard.routes.insert(k, v);
            }
            for (k, v) in merged.gateway_routes {
                guard.gateway_routes.insert(k, v);
            }
            for (k, v) in merged.tags {
                guard.tags.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
