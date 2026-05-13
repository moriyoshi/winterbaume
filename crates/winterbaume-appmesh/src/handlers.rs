use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{AppMeshError, AppMeshState};
use crate::types::*;
use crate::views::AppMeshStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct AppMeshService {
    pub(crate) state: Arc<BackendState<AppMeshState>>,
    pub(crate) notifier: StateChangeNotifier<AppMeshStateView>,
}

impl AppMeshService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AppMeshService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AppMeshService {
    fn service_name(&self) -> &str {
        "appmesh"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://appmesh\.(.+)\.amazonaws\.com",
            r"https?://appmesh\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<AppMeshState>>;

impl AppMeshService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let (path, query) = split_path_query(&request.uri);
        let method = request.method.as_str();
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&query);

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        if segments.is_empty() || segments[0] != "v20190125" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        let response = match (method, &segments[1..]) {
            // ===== Mesh =====
            ("PUT", ["meshes"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_mesh(&state, &request, labels, &query_map, account_id, &region)
                    .await
            }
            ("GET", ["meshes"]) => self.handle_list_meshes(&state).await,
            ("GET", ["meshes", _mesh_name]) => {
                let labels: &[(&str, &str)] = &[("meshName", segments[2])];
                self.handle_describe_mesh(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["meshes", _mesh_name]) => {
                let labels: &[(&str, &str)] = &[("meshName", segments[2])];
                self.handle_delete_mesh(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["meshes", _mesh_name]) => {
                let labels: &[(&str, &str)] = &[("meshName", segments[2])];
                self.handle_update_mesh(&state, &request, labels, &query_map)
                    .await
            }

            // ===== VirtualRouter =====
            ("PUT", ["meshes", _mesh, "virtualRouters"]) => {
                let labels: &[(&str, &str)] = &[("meshName", segments[2])];
                self.handle_create_virtual_router(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            ("GET", ["meshes", _mesh, "virtualRouters"]) => {
                self.handle_list_virtual_routers(&state, segments[2]).await
            }
            ("GET", ["meshes", _mesh, "virtualRouters", _vr]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualRouterName", segments[4]),
                ];
                self.handle_describe_virtual_router(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["meshes", _mesh, "virtualRouters", _vr]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualRouterName", segments[4]),
                ];
                self.handle_delete_virtual_router(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["meshes", _mesh, "virtualRouters", _vr]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualRouterName", segments[4]),
                ];
                self.handle_update_virtual_router(&state, &request, labels, &query_map)
                    .await
            }

            // ===== VirtualNode =====
            ("PUT", ["meshes", _mesh, "virtualNodes"]) => {
                let labels: &[(&str, &str)] = &[("meshName", segments[2])];
                self.handle_create_virtual_node(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            ("GET", ["meshes", _mesh, "virtualNodes"]) => {
                self.handle_list_virtual_nodes(&state, segments[2]).await
            }
            ("GET", ["meshes", _mesh, "virtualNodes", _vn]) => {
                let labels: &[(&str, &str)] =
                    &[("meshName", segments[2]), ("virtualNodeName", segments[4])];
                self.handle_describe_virtual_node(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["meshes", _mesh, "virtualNodes", _vn]) => {
                let labels: &[(&str, &str)] =
                    &[("meshName", segments[2]), ("virtualNodeName", segments[4])];
                self.handle_delete_virtual_node(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["meshes", _mesh, "virtualNodes", _vn]) => {
                let labels: &[(&str, &str)] =
                    &[("meshName", segments[2]), ("virtualNodeName", segments[4])];
                self.handle_update_virtual_node(&state, &request, labels, &query_map)
                    .await
            }

            // ===== VirtualGateway =====
            ("PUT", ["meshes", _mesh, "virtualGateways"]) => {
                let labels: &[(&str, &str)] = &[("meshName", segments[2])];
                self.handle_create_virtual_gateway(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            ("GET", ["meshes", _mesh, "virtualGateways"]) => {
                self.handle_list_virtual_gateways(&state, segments[2]).await
            }
            ("GET", ["meshes", _mesh, "virtualGateways", _vg]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualGatewayName", segments[4]),
                ];
                self.handle_describe_virtual_gateway(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["meshes", _mesh, "virtualGateways", _vg]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualGatewayName", segments[4]),
                ];
                self.handle_delete_virtual_gateway(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["meshes", _mesh, "virtualGateways", _vg]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualGatewayName", segments[4]),
                ];
                self.handle_update_virtual_gateway(&state, &request, labels, &query_map)
                    .await
            }

            // ===== VirtualService =====
            ("PUT", ["meshes", _mesh, "virtualServices"]) => {
                let labels: &[(&str, &str)] = &[("meshName", segments[2])];
                self.handle_create_virtual_service(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            ("GET", ["meshes", _mesh, "virtualServices"]) => {
                self.handle_list_virtual_services(&state, segments[2]).await
            }
            ("GET", ["meshes", _mesh, "virtualServices", _vs]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualServiceName", segments[4]),
                ];
                self.handle_describe_virtual_service(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["meshes", _mesh, "virtualServices", _vs]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualServiceName", segments[4]),
                ];
                self.handle_delete_virtual_service(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["meshes", _mesh, "virtualServices", _vs]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualServiceName", segments[4]),
                ];
                self.handle_update_virtual_service(&state, &request, labels, &query_map)
                    .await
            }

            // ===== Route (uses singular "virtualRouter") =====
            ("PUT", ["meshes", _mesh, "virtualRouter", _vr, "routes"]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualRouterName", segments[4]),
                ];
                self.handle_create_route(&state, &request, labels, &query_map, account_id, &region)
                    .await
            }
            ("GET", ["meshes", _mesh, "virtualRouter", _vr, "routes"]) => {
                self.handle_list_routes(&state, segments[2], segments[4])
                    .await
            }
            ("GET", ["meshes", _mesh, "virtualRouter", _vr, "routes", _r]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualRouterName", segments[4]),
                    ("routeName", segments[6]),
                ];
                self.handle_describe_route(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["meshes", _mesh, "virtualRouter", _vr, "routes", _r]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualRouterName", segments[4]),
                    ("routeName", segments[6]),
                ];
                self.handle_delete_route(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["meshes", _mesh, "virtualRouter", _vr, "routes", _r]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualRouterName", segments[4]),
                    ("routeName", segments[6]),
                ];
                self.handle_update_route(&state, &request, labels, &query_map)
                    .await
            }

            // ===== GatewayRoute (uses singular "virtualGateway") =====
            ("PUT", ["meshes", _mesh, "virtualGateway", _vg, "gatewayRoutes"]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualGatewayName", segments[4]),
                ];
                self.handle_create_gateway_route(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            ("GET", ["meshes", _mesh, "virtualGateway", _vg, "gatewayRoutes"]) => {
                self.handle_list_gateway_routes(&state, segments[2], segments[4])
                    .await
            }
            ("GET", ["meshes", _mesh, "virtualGateway", _vg, "gatewayRoutes", _gr]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualGatewayName", segments[4]),
                    ("gatewayRouteName", segments[6]),
                ];
                self.handle_describe_gateway_route(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["meshes", _mesh, "virtualGateway", _vg, "gatewayRoutes", _gr]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualGatewayName", segments[4]),
                    ("gatewayRouteName", segments[6]),
                ];
                self.handle_delete_gateway_route(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["meshes", _mesh, "virtualGateway", _vg, "gatewayRoutes", _gr]) => {
                let labels: &[(&str, &str)] = &[
                    ("meshName", segments[2]),
                    ("virtualGatewayName", segments[4]),
                    ("gatewayRouteName", segments[6]),
                ];
                self.handle_update_gateway_route(&state, &request, labels, &query_map)
                    .await
            }

            // ===== Tags =====
            ("PUT", ["tag"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["untag"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_untag_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["tags"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                    .await
            }

            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if matches!(method, "PUT" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    // ===== Mesh handlers =====

    async fn handle_create_mesh(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_mesh_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.mesh_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'meshName'");
        }
        let egress_filter_type = input
            .spec
            .as_ref()
            .and_then(|s| s.egress_filter.as_ref())
            .map(|e| e.r#type.clone())
            .unwrap_or_else(|| "DROP_ALL".to_string());
        let tags = tag_refs_to_pairs(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_mesh(
            &input.mesh_name,
            account_id,
            region,
            &egress_filter_type,
            tags,
        ) {
            Ok(mesh) => serialize_data(&mesh_to_wire_data(mesh)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_describe_mesh(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_mesh_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.describe_mesh(&input.mesh_name) {
            Ok(mesh) => serialize_data(&mesh_to_wire_data(mesh)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_delete_mesh(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_mesh_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_mesh(&input.mesh_name) {
            Ok(mut mesh) => {
                mesh.status = "DELETED".to_string();
                serialize_data(&mesh_to_wire_data(&mesh))
            }
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_list_meshes(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let meshes = state.list_meshes();

        let mesh_refs: Vec<wire::MeshRef> = meshes
            .iter()
            .map(|m| wire::MeshRef {
                mesh_name: Some(m.mesh_name.clone()),
                mesh_owner: Some(m.mesh_owner.clone()),
                resource_owner: Some(m.resource_owner.clone()),
                arn: Some(m.arn.clone()),
                version: Some(m.version),
                created_at: Some(m.created_at.timestamp() as f64),
                last_updated_at: Some(m.last_updated_at.timestamp() as f64),
            })
            .collect();

        wire::serialize_list_meshes_response(&wire::ListMeshesOutput {
            meshes: Some(mesh_refs),
            next_token: None,
        })
    }

    async fn handle_update_mesh(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_mesh_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let egress_filter_type = input
            .spec
            .as_ref()
            .and_then(|s| s.egress_filter.as_ref())
            .map(|e| e.r#type.clone())
            .unwrap_or_else(|| "DROP_ALL".to_string());

        let mut state = state.write().await;
        match state.update_mesh(&input.mesh_name, &egress_filter_type) {
            Ok(mesh) => serialize_data(&mesh_to_wire_data(mesh)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    // ===== VirtualRouter handlers =====

    async fn handle_create_virtual_router(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_virtual_router_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.virtual_router_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'virtualRouterName'");
        }
        let tags = tag_refs_to_pairs(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_virtual_router(
            &input.mesh_name,
            &input.virtual_router_name,
            account_id,
            region,
            input.spec,
            tags,
        ) {
            Ok(vr) => serialize_data(&virtual_router_to_wire_data(vr)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_describe_virtual_router(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_virtual_router_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.describe_virtual_router(&input.mesh_name, &input.virtual_router_name) {
            Ok(vr) => serialize_data(&virtual_router_to_wire_data(vr)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_delete_virtual_router(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_virtual_router_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_virtual_router(&input.mesh_name, &input.virtual_router_name) {
            Ok(mut vr) => {
                vr.status = "DELETED".to_string();
                serialize_data(&virtual_router_to_wire_data(&vr))
            }
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_list_virtual_routers(
        &self,
        state: &SharedState,
        mesh_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let vrs = state.list_virtual_routers(mesh_name);
        let refs: Vec<wire::VirtualRouterRef> = vrs
            .iter()
            .map(|vr| wire::VirtualRouterRef {
                mesh_name: Some(vr.mesh_name.clone()),
                virtual_router_name: Some(vr.virtual_router_name.clone()),
                arn: Some(vr.arn.clone()),
                mesh_owner: Some(vr.mesh_owner.clone()),
                resource_owner: Some(vr.resource_owner.clone()),
                version: Some(vr.version),
                created_at: Some(vr.created_at.timestamp() as f64),
                last_updated_at: Some(vr.last_updated_at.timestamp() as f64),
            })
            .collect();
        wire::serialize_list_virtual_routers_response(&wire::ListVirtualRoutersOutput {
            virtual_routers: Some(refs),
            next_token: None,
        })
    }

    async fn handle_update_virtual_router(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_virtual_router_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.update_virtual_router(&input.mesh_name, &input.virtual_router_name, input.spec)
        {
            Ok(vr) => serialize_data(&virtual_router_to_wire_data(vr)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    // ===== VirtualNode handlers =====

    async fn handle_create_virtual_node(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_virtual_node_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.virtual_node_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'virtualNodeName'");
        }
        let tags = tag_refs_to_pairs(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_virtual_node(
            &input.mesh_name,
            &input.virtual_node_name,
            account_id,
            region,
            input.spec,
            tags,
        ) {
            Ok(vn) => serialize_data(&virtual_node_to_wire_data(vn)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_describe_virtual_node(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_virtual_node_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.describe_virtual_node(&input.mesh_name, &input.virtual_node_name) {
            Ok(vn) => serialize_data(&virtual_node_to_wire_data(vn)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_delete_virtual_node(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_virtual_node_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_virtual_node(&input.mesh_name, &input.virtual_node_name) {
            Ok(mut vn) => {
                vn.status = "DELETED".to_string();
                serialize_data(&virtual_node_to_wire_data(&vn))
            }
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_list_virtual_nodes(
        &self,
        state: &SharedState,
        mesh_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let vns = state.list_virtual_nodes(mesh_name);
        let refs: Vec<wire::VirtualNodeRef> = vns
            .iter()
            .map(|vn| wire::VirtualNodeRef {
                mesh_name: Some(vn.mesh_name.clone()),
                virtual_node_name: Some(vn.virtual_node_name.clone()),
                arn: Some(vn.arn.clone()),
                mesh_owner: Some(vn.mesh_owner.clone()),
                resource_owner: Some(vn.resource_owner.clone()),
                version: Some(vn.version),
                created_at: Some(vn.created_at.timestamp() as f64),
                last_updated_at: Some(vn.last_updated_at.timestamp() as f64),
            })
            .collect();
        wire::serialize_list_virtual_nodes_response(&wire::ListVirtualNodesOutput {
            virtual_nodes: Some(refs),
            next_token: None,
        })
    }

    async fn handle_update_virtual_node(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_virtual_node_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.update_virtual_node(&input.mesh_name, &input.virtual_node_name, input.spec) {
            Ok(vn) => serialize_data(&virtual_node_to_wire_data(vn)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    // ===== VirtualGateway handlers =====

    async fn handle_create_virtual_gateway(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_virtual_gateway_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.virtual_gateway_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'virtualGatewayName'");
        }
        let tags = tag_refs_to_pairs(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_virtual_gateway(
            &input.mesh_name,
            &input.virtual_gateway_name,
            account_id,
            region,
            input.spec,
            tags,
        ) {
            Ok(vg) => serialize_data(&virtual_gateway_to_wire_data(vg)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_describe_virtual_gateway(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_virtual_gateway_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.describe_virtual_gateway(&input.mesh_name, &input.virtual_gateway_name) {
            Ok(vg) => serialize_data(&virtual_gateway_to_wire_data(vg)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_delete_virtual_gateway(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_virtual_gateway_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_virtual_gateway(&input.mesh_name, &input.virtual_gateway_name) {
            Ok(mut vg) => {
                vg.status = "DELETED".to_string();
                serialize_data(&virtual_gateway_to_wire_data(&vg))
            }
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_list_virtual_gateways(
        &self,
        state: &SharedState,
        mesh_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let vgs = state.list_virtual_gateways(mesh_name);
        let refs: Vec<wire::VirtualGatewayRef> = vgs
            .iter()
            .map(|vg| wire::VirtualGatewayRef {
                mesh_name: Some(vg.mesh_name.clone()),
                virtual_gateway_name: Some(vg.virtual_gateway_name.clone()),
                arn: Some(vg.arn.clone()),
                mesh_owner: Some(vg.mesh_owner.clone()),
                resource_owner: Some(vg.resource_owner.clone()),
                version: Some(vg.version),
                created_at: Some(vg.created_at.timestamp() as f64),
                last_updated_at: Some(vg.last_updated_at.timestamp() as f64),
            })
            .collect();
        wire::serialize_list_virtual_gateways_response(&wire::ListVirtualGatewaysOutput {
            virtual_gateways: Some(refs),
            next_token: None,
        })
    }

    async fn handle_update_virtual_gateway(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_virtual_gateway_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.update_virtual_gateway(
            &input.mesh_name,
            &input.virtual_gateway_name,
            input.spec,
        ) {
            Ok(vg) => serialize_data(&virtual_gateway_to_wire_data(vg)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    // ===== VirtualService handlers =====

    async fn handle_create_virtual_service(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_virtual_service_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.virtual_service_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'virtualServiceName'");
        }
        let tags = tag_refs_to_pairs(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_virtual_service(
            &input.mesh_name,
            &input.virtual_service_name,
            account_id,
            region,
            input.spec,
            tags,
        ) {
            Ok(vs) => serialize_data(&virtual_service_to_wire_data(vs)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_describe_virtual_service(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_virtual_service_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.describe_virtual_service(&input.mesh_name, &input.virtual_service_name) {
            Ok(vs) => serialize_data(&virtual_service_to_wire_data(vs)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_delete_virtual_service(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_virtual_service_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_virtual_service(&input.mesh_name, &input.virtual_service_name) {
            Ok(mut vs) => {
                vs.status = "DELETED".to_string();
                serialize_data(&virtual_service_to_wire_data(&vs))
            }
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_list_virtual_services(
        &self,
        state: &SharedState,
        mesh_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let vss = state.list_virtual_services(mesh_name);
        let refs: Vec<wire::VirtualServiceRef> = vss
            .iter()
            .map(|vs| wire::VirtualServiceRef {
                mesh_name: Some(vs.mesh_name.clone()),
                virtual_service_name: Some(vs.virtual_service_name.clone()),
                arn: Some(vs.arn.clone()),
                mesh_owner: Some(vs.mesh_owner.clone()),
                resource_owner: Some(vs.resource_owner.clone()),
                version: Some(vs.version),
                created_at: Some(vs.created_at.timestamp() as f64),
                last_updated_at: Some(vs.last_updated_at.timestamp() as f64),
            })
            .collect();
        wire::serialize_list_virtual_services_response(&wire::ListVirtualServicesOutput {
            virtual_services: Some(refs),
            next_token: None,
        })
    }

    async fn handle_update_virtual_service(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_virtual_service_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.update_virtual_service(
            &input.mesh_name,
            &input.virtual_service_name,
            input.spec,
        ) {
            Ok(vs) => serialize_data(&virtual_service_to_wire_data(vs)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    // ===== Route handlers =====

    async fn handle_create_route(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_route_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.route_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'routeName'");
        }
        let tags = tag_refs_to_pairs(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_route(
            &input.mesh_name,
            &input.virtual_router_name,
            &input.route_name,
            account_id,
            region,
            input.spec,
            tags,
        ) {
            Ok(r) => serialize_data(&route_to_wire_data(r)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_describe_route(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_route_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.describe_route(
            &input.mesh_name,
            &input.virtual_router_name,
            &input.route_name,
        ) {
            Ok(r) => serialize_data(&route_to_wire_data(r)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_delete_route(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_route_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_route(
            &input.mesh_name,
            &input.virtual_router_name,
            &input.route_name,
        ) {
            Ok(mut r) => {
                r.status = "DELETED".to_string();
                serialize_data(&route_to_wire_data(&r))
            }
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_list_routes(
        &self,
        state: &SharedState,
        mesh_name: &str,
        virtual_router_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let routes = state.list_routes(mesh_name, virtual_router_name);
        let refs: Vec<wire::RouteRef> = routes
            .iter()
            .map(|r| wire::RouteRef {
                mesh_name: Some(r.mesh_name.clone()),
                virtual_router_name: Some(r.virtual_router_name.clone()),
                route_name: Some(r.route_name.clone()),
                arn: Some(r.arn.clone()),
                mesh_owner: Some(r.mesh_owner.clone()),
                resource_owner: Some(r.resource_owner.clone()),
                version: Some(r.version),
                created_at: Some(r.created_at.timestamp() as f64),
                last_updated_at: Some(r.last_updated_at.timestamp() as f64),
            })
            .collect();
        wire::serialize_list_routes_response(&wire::ListRoutesOutput {
            routes: Some(refs),
            next_token: None,
        })
    }

    async fn handle_update_route(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_route_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.update_route(
            &input.mesh_name,
            &input.virtual_router_name,
            &input.route_name,
            input.spec,
        ) {
            Ok(r) => serialize_data(&route_to_wire_data(r)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    // ===== GatewayRoute handlers =====

    async fn handle_create_gateway_route(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_gateway_route_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.gateway_route_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'gatewayRouteName'");
        }
        let tags = tag_refs_to_pairs(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_gateway_route(
            &input.mesh_name,
            &input.virtual_gateway_name,
            &input.gateway_route_name,
            account_id,
            region,
            input.spec,
            tags,
        ) {
            Ok(gr) => serialize_data(&gateway_route_to_wire_data(gr)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_describe_gateway_route(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_gateway_route_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.describe_gateway_route(
            &input.mesh_name,
            &input.virtual_gateway_name,
            &input.gateway_route_name,
        ) {
            Ok(gr) => serialize_data(&gateway_route_to_wire_data(gr)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_delete_gateway_route(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_gateway_route_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_gateway_route(
            &input.mesh_name,
            &input.virtual_gateway_name,
            &input.gateway_route_name,
        ) {
            Ok(mut gr) => {
                gr.status = "DELETED".to_string();
                serialize_data(&gateway_route_to_wire_data(&gr))
            }
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_list_gateway_routes(
        &self,
        state: &SharedState,
        mesh_name: &str,
        virtual_gateway_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let grs = state.list_gateway_routes(mesh_name, virtual_gateway_name);
        let refs: Vec<wire::GatewayRouteRef> = grs
            .iter()
            .map(|gr| wire::GatewayRouteRef {
                mesh_name: Some(gr.mesh_name.clone()),
                virtual_gateway_name: Some(gr.virtual_gateway_name.clone()),
                gateway_route_name: Some(gr.gateway_route_name.clone()),
                arn: Some(gr.arn.clone()),
                mesh_owner: Some(gr.mesh_owner.clone()),
                resource_owner: Some(gr.resource_owner.clone()),
                version: Some(gr.version),
                created_at: Some(gr.created_at.timestamp() as f64),
                last_updated_at: Some(gr.last_updated_at.timestamp() as f64),
            })
            .collect();
        wire::serialize_list_gateway_routes_response(&wire::ListGatewayRoutesOutput {
            gateway_routes: Some(refs),
            next_token: None,
        })
    }

    async fn handle_update_gateway_route(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_gateway_route_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.update_gateway_route(
            &input.mesh_name,
            &input.virtual_gateway_name,
            &input.gateway_route_name,
            input.spec,
        ) {
            Ok(gr) => serialize_data(&gateway_route_to_wire_data(gr)),
            Err(e) => appmesh_error_response(&e),
        }
    }

    // ===== Tag handlers =====

    async fn handle_tag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'resourceArn'");
        }
        let tags: Vec<(String, String)> = input
            .tags
            .iter()
            .map(|t| (t.key.clone(), t.value.clone()))
            .collect();
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceOutput {}),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'resourceArn'");
        }
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceOutput {}),
            Err(e) => appmesh_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'resourceArn'");
        }
        let state = state.read().await;
        let tags = state.list_tags_for_resource(&input.resource_arn);
        let tag_refs: Vec<wire::TagRef> = tags
            .iter()
            .map(|(k, v)| wire::TagRef {
                key: k.clone(),
                value: v.clone(),
            })
            .collect();
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceOutput {
            tags: Some(tag_refs),
            next_token: None,
        })
    }
}

// ===== Direct serialization helpers (for @httpPayload operations) =====

fn serialize_data<T: serde::Serialize>(data: &T) -> MockResponse {
    let body = serde_json::to_string(data).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(200, body)
}

// ===== Wire conversion helpers =====

fn build_metadata(
    arn: &str,
    uid: &str,
    version: i64,
    created_at: &chrono::DateTime<chrono::Utc>,
    last_updated_at: &chrono::DateTime<chrono::Utc>,
    mesh_owner: &str,
    resource_owner: &str,
) -> wire::ResourceMetadata {
    wire::ResourceMetadata {
        arn: Some(arn.to_string()),
        uid: Some(uid.to_string()),
        version: Some(version),
        created_at: Some(created_at.timestamp() as f64),
        last_updated_at: Some(last_updated_at.timestamp() as f64),
        mesh_owner: Some(mesh_owner.to_string()),
        resource_owner: Some(resource_owner.to_string()),
    }
}

fn mesh_to_wire_data(mesh: &Mesh) -> wire::MeshData {
    wire::MeshData {
        mesh_name: Some(mesh.mesh_name.clone()),
        spec: Some(wire::MeshSpec {
            egress_filter: Some(wire::EgressFilter {
                r#type: mesh.egress_filter_type.clone(),
            }),
            service_discovery: None,
        }),
        metadata: Some(build_metadata(
            &mesh.arn,
            &mesh.uid,
            mesh.version,
            &mesh.created_at,
            &mesh.last_updated_at,
            &mesh.mesh_owner,
            &mesh.resource_owner,
        )),
        status: Some(wire::MeshStatus {
            status: Some(mesh.status.clone()),
        }),
    }
}

fn virtual_router_to_wire_data(vr: &VirtualRouter) -> wire::VirtualRouterData {
    wire::VirtualRouterData {
        mesh_name: Some(vr.mesh_name.clone()),
        virtual_router_name: Some(vr.virtual_router_name.clone()),
        spec: Some(vr.spec.clone()),
        metadata: Some(build_metadata(
            &vr.arn,
            &vr.uid,
            vr.version,
            &vr.created_at,
            &vr.last_updated_at,
            &vr.mesh_owner,
            &vr.resource_owner,
        )),
        status: Some(wire::VirtualRouterStatus {
            status: Some(vr.status.clone()),
        }),
    }
}

fn virtual_node_to_wire_data(vn: &VirtualNode) -> wire::VirtualNodeData {
    wire::VirtualNodeData {
        mesh_name: Some(vn.mesh_name.clone()),
        virtual_node_name: Some(vn.virtual_node_name.clone()),
        spec: Some(vn.spec.clone()),
        metadata: Some(build_metadata(
            &vn.arn,
            &vn.uid,
            vn.version,
            &vn.created_at,
            &vn.last_updated_at,
            &vn.mesh_owner,
            &vn.resource_owner,
        )),
        status: Some(wire::VirtualNodeStatus {
            status: Some(vn.status.clone()),
        }),
    }
}

fn virtual_gateway_to_wire_data(vg: &VirtualGateway) -> wire::VirtualGatewayData {
    wire::VirtualGatewayData {
        mesh_name: Some(vg.mesh_name.clone()),
        virtual_gateway_name: Some(vg.virtual_gateway_name.clone()),
        spec: Some(vg.spec.clone()),
        metadata: Some(build_metadata(
            &vg.arn,
            &vg.uid,
            vg.version,
            &vg.created_at,
            &vg.last_updated_at,
            &vg.mesh_owner,
            &vg.resource_owner,
        )),
        status: Some(wire::VirtualGatewayStatus {
            status: Some(vg.status.clone()),
        }),
    }
}

fn virtual_service_to_wire_data(vs: &VirtualService) -> wire::VirtualServiceData {
    wire::VirtualServiceData {
        mesh_name: Some(vs.mesh_name.clone()),
        virtual_service_name: Some(vs.virtual_service_name.clone()),
        spec: Some(vs.spec.clone()),
        metadata: Some(build_metadata(
            &vs.arn,
            &vs.uid,
            vs.version,
            &vs.created_at,
            &vs.last_updated_at,
            &vs.mesh_owner,
            &vs.resource_owner,
        )),
        status: Some(wire::VirtualServiceStatus {
            status: Some(vs.status.clone()),
        }),
    }
}

fn route_to_wire_data(r: &Route) -> wire::RouteData {
    wire::RouteData {
        mesh_name: Some(r.mesh_name.clone()),
        virtual_router_name: Some(r.virtual_router_name.clone()),
        route_name: Some(r.route_name.clone()),
        spec: Some(r.spec.clone()),
        metadata: Some(build_metadata(
            &r.arn,
            &r.uid,
            r.version,
            &r.created_at,
            &r.last_updated_at,
            &r.mesh_owner,
            &r.resource_owner,
        )),
        status: Some(wire::RouteStatus {
            status: Some(r.status.clone()),
        }),
    }
}

fn gateway_route_to_wire_data(gr: &GatewayRoute) -> wire::GatewayRouteData {
    wire::GatewayRouteData {
        mesh_name: Some(gr.mesh_name.clone()),
        virtual_gateway_name: Some(gr.virtual_gateway_name.clone()),
        gateway_route_name: Some(gr.gateway_route_name.clone()),
        spec: Some(gr.spec.clone()),
        metadata: Some(build_metadata(
            &gr.arn,
            &gr.uid,
            gr.version,
            &gr.created_at,
            &gr.last_updated_at,
            &gr.mesh_owner,
            &gr.resource_owner,
        )),
        status: Some(wire::GatewayRouteStatus {
            status: Some(gr.status.clone()),
        }),
    }
}

fn tag_refs_to_pairs(tags: Option<&[wire::TagRef]>) -> Vec<(String, String)> {
    tags.map(|arr| {
        arr.iter()
            .map(|t| (t.key.clone(), t.value.clone()))
            .collect()
    })
    .unwrap_or_default()
}

fn split_path_query(uri: &str) -> (String, String) {
    let after_host = if let Some(idx) = uri.find("amazonaws.com") {
        &uri[idx + "amazonaws.com".len()..]
    } else {
        uri
    };
    if let Some(q) = after_host.find('?') {
        (after_host[..q].to_string(), after_host[q + 1..].to_string())
    } else {
        (after_host.to_string(), String::new())
    }
}

fn appmesh_error_response(err: &AppMeshError) -> MockResponse {
    let (status, error_type) = match err {
        AppMeshError::NotFound(_) => (404u16, "NotFoundException"),
        AppMeshError::Conflict(_) => (409u16, "ConflictException"),
    };
    let body = json!({
        "message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
