use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;
use crate::wire::{
    GatewayRouteSpec, RouteSpec, VirtualGatewaySpec, VirtualNodeSpec, VirtualRouterSpec,
    VirtualServiceSpec,
};

#[derive(Debug, Default)]
pub struct AppMeshState {
    pub meshes: HashMap<String, Mesh>,
    /// key: (mesh_name, virtual_router_name)
    pub virtual_routers: HashMap<(String, String), VirtualRouter>,
    /// key: (mesh_name, virtual_node_name)
    pub virtual_nodes: HashMap<(String, String), VirtualNode>,
    /// key: (mesh_name, virtual_gateway_name)
    pub virtual_gateways: HashMap<(String, String), VirtualGateway>,
    /// key: (mesh_name, virtual_service_name)
    pub virtual_services: HashMap<(String, String), VirtualService>,
    /// key: (mesh_name, virtual_router_name, route_name)
    pub routes: HashMap<(String, String, String), Route>,
    /// key: (mesh_name, virtual_gateway_name, gateway_route_name)
    pub gateway_routes: HashMap<(String, String, String), GatewayRoute>,
    /// tags by ARN
    pub tags: HashMap<String, Vec<(String, String)>>,
}

#[derive(Debug, thiserror::Error)]
pub enum AppMeshError {
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Conflict(String),
}

fn not_found(msg: &str) -> AppMeshError {
    AppMeshError::NotFound(msg.to_string())
}

fn conflict(msg: &str) -> AppMeshError {
    AppMeshError::Conflict(msg.to_string())
}

impl AppMeshState {
    // =========== Mesh ===========

    pub fn create_mesh(
        &mut self,
        mesh_name: &str,
        account_id: &str,
        region: &str,
        egress_filter_type: &str,
        tags: Vec<(String, String)>,
    ) -> Result<&Mesh, AppMeshError> {
        if self.meshes.contains_key(mesh_name) {
            return Err(conflict(&format!(
                "Mesh with name {mesh_name} already exists"
            )));
        }

        let now = Utc::now();
        let arn = format!("arn:aws:appmesh:{region}:{account_id}:mesh/{mesh_name}");
        let uid = uuid::Uuid::new_v4().to_string();

        let mesh = Mesh {
            mesh_name: mesh_name.to_string(),
            arn: arn.clone(),
            uid,
            status: "ACTIVE".to_string(),
            version: 1,
            created_at: now,
            last_updated_at: now,
            mesh_owner: account_id.to_string(),
            resource_owner: account_id.to_string(),
            egress_filter_type: egress_filter_type.to_string(),
            tags: tags.clone(),
        };

        self.meshes.insert(mesh_name.to_string(), mesh);
        self.tags.insert(arn, tags);
        Ok(self.meshes.get(mesh_name).unwrap())
    }

    pub fn describe_mesh(&self, mesh_name: &str) -> Result<&Mesh, AppMeshError> {
        self.meshes
            .get(mesh_name)
            .ok_or_else(|| not_found(&format!("Mesh with name {mesh_name} is not found")))
    }

    pub fn delete_mesh(&mut self, mesh_name: &str) -> Result<Mesh, AppMeshError> {
        let mesh = self
            .meshes
            .remove(mesh_name)
            .ok_or_else(|| not_found(&format!("Mesh with name {mesh_name} is not found")))?;
        self.tags.remove(&mesh.arn);
        Ok(mesh)
    }

    pub fn list_meshes(&self) -> Vec<&Mesh> {
        self.meshes.values().collect()
    }

    pub fn update_mesh(
        &mut self,
        mesh_name: &str,
        egress_filter_type: &str,
    ) -> Result<&Mesh, AppMeshError> {
        let mesh = self
            .meshes
            .get_mut(mesh_name)
            .ok_or_else(|| not_found(&format!("Mesh with name {mesh_name} is not found")))?;
        mesh.egress_filter_type = egress_filter_type.to_string();
        mesh.version += 1;
        mesh.last_updated_at = Utc::now();
        Ok(mesh)
    }

    // =========== VirtualRouter ===========

    pub fn create_virtual_router(
        &mut self,
        mesh_name: &str,
        virtual_router_name: &str,
        account_id: &str,
        region: &str,
        spec: VirtualRouterSpec,
        tags: Vec<(String, String)>,
    ) -> Result<&VirtualRouter, AppMeshError> {
        if !self.meshes.contains_key(mesh_name) {
            return Err(not_found(&format!(
                "Mesh with name {mesh_name} is not found"
            )));
        }
        let key = (mesh_name.to_string(), virtual_router_name.to_string());
        if self.virtual_routers.contains_key(&key) {
            return Err(conflict(&format!(
                "VirtualRouter with name {virtual_router_name} already exists"
            )));
        }
        let now = Utc::now();
        let arn = format!(
            "arn:aws:appmesh:{region}:{account_id}:mesh/{mesh_name}/virtualRouter/{virtual_router_name}"
        );
        let uid = uuid::Uuid::new_v4().to_string();
        let vr = VirtualRouter {
            mesh_name: mesh_name.to_string(),
            virtual_router_name: virtual_router_name.to_string(),
            arn: arn.clone(),
            uid,
            status: "ACTIVE".to_string(),
            version: 1,
            created_at: now,
            last_updated_at: now,
            mesh_owner: account_id.to_string(),
            resource_owner: account_id.to_string(),
            spec,
            tags: tags.clone(),
        };
        self.virtual_routers.insert(key.clone(), vr);
        self.tags.insert(arn, tags);
        Ok(self.virtual_routers.get(&key).unwrap())
    }

    pub fn describe_virtual_router(
        &self,
        mesh_name: &str,
        virtual_router_name: &str,
    ) -> Result<&VirtualRouter, AppMeshError> {
        let key = (mesh_name.to_string(), virtual_router_name.to_string());
        self.virtual_routers.get(&key).ok_or_else(|| {
            not_found(&format!(
                "VirtualRouter with name {virtual_router_name} is not found"
            ))
        })
    }

    pub fn delete_virtual_router(
        &mut self,
        mesh_name: &str,
        virtual_router_name: &str,
    ) -> Result<VirtualRouter, AppMeshError> {
        let key = (mesh_name.to_string(), virtual_router_name.to_string());
        let vr = self.virtual_routers.remove(&key).ok_or_else(|| {
            not_found(&format!(
                "VirtualRouter with name {virtual_router_name} is not found"
            ))
        })?;
        self.tags.remove(&vr.arn);
        Ok(vr)
    }

    pub fn list_virtual_routers(&self, mesh_name: &str) -> Vec<&VirtualRouter> {
        self.virtual_routers
            .values()
            .filter(|vr| vr.mesh_name == mesh_name)
            .collect()
    }

    pub fn update_virtual_router(
        &mut self,
        mesh_name: &str,
        virtual_router_name: &str,
        spec: VirtualRouterSpec,
    ) -> Result<&VirtualRouter, AppMeshError> {
        let key = (mesh_name.to_string(), virtual_router_name.to_string());
        let vr = self.virtual_routers.get_mut(&key).ok_or_else(|| {
            not_found(&format!(
                "VirtualRouter with name {virtual_router_name} is not found"
            ))
        })?;
        vr.spec = spec;
        vr.version += 1;
        vr.last_updated_at = Utc::now();
        Ok(vr)
    }

    // =========== VirtualNode ===========

    pub fn create_virtual_node(
        &mut self,
        mesh_name: &str,
        virtual_node_name: &str,
        account_id: &str,
        region: &str,
        spec: VirtualNodeSpec,
        tags: Vec<(String, String)>,
    ) -> Result<&VirtualNode, AppMeshError> {
        if !self.meshes.contains_key(mesh_name) {
            return Err(not_found(&format!(
                "Mesh with name {mesh_name} is not found"
            )));
        }
        let key = (mesh_name.to_string(), virtual_node_name.to_string());
        if self.virtual_nodes.contains_key(&key) {
            return Err(conflict(&format!(
                "VirtualNode with name {virtual_node_name} already exists"
            )));
        }
        let now = Utc::now();
        let arn = format!(
            "arn:aws:appmesh:{region}:{account_id}:mesh/{mesh_name}/virtualNode/{virtual_node_name}"
        );
        let uid = uuid::Uuid::new_v4().to_string();
        let vn = VirtualNode {
            mesh_name: mesh_name.to_string(),
            virtual_node_name: virtual_node_name.to_string(),
            arn: arn.clone(),
            uid,
            status: "ACTIVE".to_string(),
            version: 1,
            created_at: now,
            last_updated_at: now,
            mesh_owner: account_id.to_string(),
            resource_owner: account_id.to_string(),
            spec,
            tags: tags.clone(),
        };
        self.virtual_nodes.insert(key.clone(), vn);
        self.tags.insert(arn, tags);
        Ok(self.virtual_nodes.get(&key).unwrap())
    }

    pub fn describe_virtual_node(
        &self,
        mesh_name: &str,
        virtual_node_name: &str,
    ) -> Result<&VirtualNode, AppMeshError> {
        let key = (mesh_name.to_string(), virtual_node_name.to_string());
        self.virtual_nodes.get(&key).ok_or_else(|| {
            not_found(&format!(
                "VirtualNode with name {virtual_node_name} is not found"
            ))
        })
    }

    pub fn delete_virtual_node(
        &mut self,
        mesh_name: &str,
        virtual_node_name: &str,
    ) -> Result<VirtualNode, AppMeshError> {
        let key = (mesh_name.to_string(), virtual_node_name.to_string());
        let vn = self.virtual_nodes.remove(&key).ok_or_else(|| {
            not_found(&format!(
                "VirtualNode with name {virtual_node_name} is not found"
            ))
        })?;
        self.tags.remove(&vn.arn);
        Ok(vn)
    }

    pub fn list_virtual_nodes(&self, mesh_name: &str) -> Vec<&VirtualNode> {
        self.virtual_nodes
            .values()
            .filter(|vn| vn.mesh_name == mesh_name)
            .collect()
    }

    pub fn update_virtual_node(
        &mut self,
        mesh_name: &str,
        virtual_node_name: &str,
        spec: VirtualNodeSpec,
    ) -> Result<&VirtualNode, AppMeshError> {
        let key = (mesh_name.to_string(), virtual_node_name.to_string());
        let vn = self.virtual_nodes.get_mut(&key).ok_or_else(|| {
            not_found(&format!(
                "VirtualNode with name {virtual_node_name} is not found"
            ))
        })?;
        vn.spec = spec;
        vn.version += 1;
        vn.last_updated_at = Utc::now();
        Ok(vn)
    }

    // =========== VirtualGateway ===========

    pub fn create_virtual_gateway(
        &mut self,
        mesh_name: &str,
        virtual_gateway_name: &str,
        account_id: &str,
        region: &str,
        spec: VirtualGatewaySpec,
        tags: Vec<(String, String)>,
    ) -> Result<&VirtualGateway, AppMeshError> {
        if !self.meshes.contains_key(mesh_name) {
            return Err(not_found(&format!(
                "Mesh with name {mesh_name} is not found"
            )));
        }
        let key = (mesh_name.to_string(), virtual_gateway_name.to_string());
        if self.virtual_gateways.contains_key(&key) {
            return Err(conflict(&format!(
                "VirtualGateway with name {virtual_gateway_name} already exists"
            )));
        }
        let now = Utc::now();
        let arn = format!(
            "arn:aws:appmesh:{region}:{account_id}:mesh/{mesh_name}/virtualGateway/{virtual_gateway_name}"
        );
        let uid = uuid::Uuid::new_v4().to_string();
        let vg = VirtualGateway {
            mesh_name: mesh_name.to_string(),
            virtual_gateway_name: virtual_gateway_name.to_string(),
            arn: arn.clone(),
            uid,
            status: "ACTIVE".to_string(),
            version: 1,
            created_at: now,
            last_updated_at: now,
            mesh_owner: account_id.to_string(),
            resource_owner: account_id.to_string(),
            spec,
            tags: tags.clone(),
        };
        self.virtual_gateways.insert(key.clone(), vg);
        self.tags.insert(arn, tags);
        Ok(self.virtual_gateways.get(&key).unwrap())
    }

    pub fn describe_virtual_gateway(
        &self,
        mesh_name: &str,
        virtual_gateway_name: &str,
    ) -> Result<&VirtualGateway, AppMeshError> {
        let key = (mesh_name.to_string(), virtual_gateway_name.to_string());
        self.virtual_gateways.get(&key).ok_or_else(|| {
            not_found(&format!(
                "VirtualGateway with name {virtual_gateway_name} is not found"
            ))
        })
    }

    pub fn delete_virtual_gateway(
        &mut self,
        mesh_name: &str,
        virtual_gateway_name: &str,
    ) -> Result<VirtualGateway, AppMeshError> {
        let key = (mesh_name.to_string(), virtual_gateway_name.to_string());
        let vg = self.virtual_gateways.remove(&key).ok_or_else(|| {
            not_found(&format!(
                "VirtualGateway with name {virtual_gateway_name} is not found"
            ))
        })?;
        self.tags.remove(&vg.arn);
        Ok(vg)
    }

    pub fn list_virtual_gateways(&self, mesh_name: &str) -> Vec<&VirtualGateway> {
        self.virtual_gateways
            .values()
            .filter(|vg| vg.mesh_name == mesh_name)
            .collect()
    }

    pub fn update_virtual_gateway(
        &mut self,
        mesh_name: &str,
        virtual_gateway_name: &str,
        spec: VirtualGatewaySpec,
    ) -> Result<&VirtualGateway, AppMeshError> {
        let key = (mesh_name.to_string(), virtual_gateway_name.to_string());
        let vg = self.virtual_gateways.get_mut(&key).ok_or_else(|| {
            not_found(&format!(
                "VirtualGateway with name {virtual_gateway_name} is not found"
            ))
        })?;
        vg.spec = spec;
        vg.version += 1;
        vg.last_updated_at = Utc::now();
        Ok(vg)
    }

    // =========== VirtualService ===========

    pub fn create_virtual_service(
        &mut self,
        mesh_name: &str,
        virtual_service_name: &str,
        account_id: &str,
        region: &str,
        spec: VirtualServiceSpec,
        tags: Vec<(String, String)>,
    ) -> Result<&VirtualService, AppMeshError> {
        if !self.meshes.contains_key(mesh_name) {
            return Err(not_found(&format!(
                "Mesh with name {mesh_name} is not found"
            )));
        }
        let key = (mesh_name.to_string(), virtual_service_name.to_string());
        if self.virtual_services.contains_key(&key) {
            return Err(conflict(&format!(
                "VirtualService with name {virtual_service_name} already exists"
            )));
        }
        let now = Utc::now();
        let arn = format!(
            "arn:aws:appmesh:{region}:{account_id}:mesh/{mesh_name}/virtualService/{virtual_service_name}"
        );
        let uid = uuid::Uuid::new_v4().to_string();
        let vs = VirtualService {
            mesh_name: mesh_name.to_string(),
            virtual_service_name: virtual_service_name.to_string(),
            arn: arn.clone(),
            uid,
            status: "ACTIVE".to_string(),
            version: 1,
            created_at: now,
            last_updated_at: now,
            mesh_owner: account_id.to_string(),
            resource_owner: account_id.to_string(),
            spec,
            tags: tags.clone(),
        };
        self.virtual_services.insert(key.clone(), vs);
        self.tags.insert(arn, tags);
        Ok(self.virtual_services.get(&key).unwrap())
    }

    pub fn describe_virtual_service(
        &self,
        mesh_name: &str,
        virtual_service_name: &str,
    ) -> Result<&VirtualService, AppMeshError> {
        let key = (mesh_name.to_string(), virtual_service_name.to_string());
        self.virtual_services.get(&key).ok_or_else(|| {
            not_found(&format!(
                "VirtualService with name {virtual_service_name} is not found"
            ))
        })
    }

    pub fn delete_virtual_service(
        &mut self,
        mesh_name: &str,
        virtual_service_name: &str,
    ) -> Result<VirtualService, AppMeshError> {
        let key = (mesh_name.to_string(), virtual_service_name.to_string());
        let vs = self.virtual_services.remove(&key).ok_or_else(|| {
            not_found(&format!(
                "VirtualService with name {virtual_service_name} is not found"
            ))
        })?;
        self.tags.remove(&vs.arn);
        Ok(vs)
    }

    pub fn list_virtual_services(&self, mesh_name: &str) -> Vec<&VirtualService> {
        self.virtual_services
            .values()
            .filter(|vs| vs.mesh_name == mesh_name)
            .collect()
    }

    pub fn update_virtual_service(
        &mut self,
        mesh_name: &str,
        virtual_service_name: &str,
        spec: VirtualServiceSpec,
    ) -> Result<&VirtualService, AppMeshError> {
        let key = (mesh_name.to_string(), virtual_service_name.to_string());
        let vs = self.virtual_services.get_mut(&key).ok_or_else(|| {
            not_found(&format!(
                "VirtualService with name {virtual_service_name} is not found"
            ))
        })?;
        vs.spec = spec;
        vs.version += 1;
        vs.last_updated_at = Utc::now();
        Ok(vs)
    }

    // =========== Route ===========

    pub fn create_route(
        &mut self,
        mesh_name: &str,
        virtual_router_name: &str,
        route_name: &str,
        account_id: &str,
        region: &str,
        spec: RouteSpec,
        tags: Vec<(String, String)>,
    ) -> Result<&Route, AppMeshError> {
        if !self.meshes.contains_key(mesh_name) {
            return Err(not_found(&format!(
                "Mesh with name {mesh_name} is not found"
            )));
        }
        let vr_key = (mesh_name.to_string(), virtual_router_name.to_string());
        if !self.virtual_routers.contains_key(&vr_key) {
            return Err(not_found(&format!(
                "VirtualRouter with name {virtual_router_name} is not found"
            )));
        }
        let key = (
            mesh_name.to_string(),
            virtual_router_name.to_string(),
            route_name.to_string(),
        );
        if self.routes.contains_key(&key) {
            return Err(conflict(&format!(
                "Route with name {route_name} already exists"
            )));
        }
        let now = Utc::now();
        let arn = format!(
            "arn:aws:appmesh:{region}:{account_id}:mesh/{mesh_name}/virtualRouter/{virtual_router_name}/route/{route_name}"
        );
        let uid = uuid::Uuid::new_v4().to_string();
        let route = Route {
            mesh_name: mesh_name.to_string(),
            virtual_router_name: virtual_router_name.to_string(),
            route_name: route_name.to_string(),
            arn: arn.clone(),
            uid,
            status: "ACTIVE".to_string(),
            version: 1,
            created_at: now,
            last_updated_at: now,
            mesh_owner: account_id.to_string(),
            resource_owner: account_id.to_string(),
            spec,
            tags: tags.clone(),
        };
        self.routes.insert(key.clone(), route);
        self.tags.insert(arn, tags);
        Ok(self.routes.get(&key).unwrap())
    }

    pub fn describe_route(
        &self,
        mesh_name: &str,
        virtual_router_name: &str,
        route_name: &str,
    ) -> Result<&Route, AppMeshError> {
        let key = (
            mesh_name.to_string(),
            virtual_router_name.to_string(),
            route_name.to_string(),
        );
        self.routes
            .get(&key)
            .ok_or_else(|| not_found(&format!("Route with name {route_name} is not found")))
    }

    pub fn delete_route(
        &mut self,
        mesh_name: &str,
        virtual_router_name: &str,
        route_name: &str,
    ) -> Result<Route, AppMeshError> {
        let key = (
            mesh_name.to_string(),
            virtual_router_name.to_string(),
            route_name.to_string(),
        );
        let route = self
            .routes
            .remove(&key)
            .ok_or_else(|| not_found(&format!("Route with name {route_name} is not found")))?;
        self.tags.remove(&route.arn);
        Ok(route)
    }

    pub fn list_routes(&self, mesh_name: &str, virtual_router_name: &str) -> Vec<&Route> {
        self.routes
            .values()
            .filter(|r| r.mesh_name == mesh_name && r.virtual_router_name == virtual_router_name)
            .collect()
    }

    pub fn update_route(
        &mut self,
        mesh_name: &str,
        virtual_router_name: &str,
        route_name: &str,
        spec: RouteSpec,
    ) -> Result<&Route, AppMeshError> {
        let key = (
            mesh_name.to_string(),
            virtual_router_name.to_string(),
            route_name.to_string(),
        );
        let route = self
            .routes
            .get_mut(&key)
            .ok_or_else(|| not_found(&format!("Route with name {route_name} is not found")))?;
        route.spec = spec;
        route.version += 1;
        route.last_updated_at = Utc::now();
        Ok(route)
    }

    // =========== GatewayRoute ===========

    pub fn create_gateway_route(
        &mut self,
        mesh_name: &str,
        virtual_gateway_name: &str,
        gateway_route_name: &str,
        account_id: &str,
        region: &str,
        spec: GatewayRouteSpec,
        tags: Vec<(String, String)>,
    ) -> Result<&GatewayRoute, AppMeshError> {
        if !self.meshes.contains_key(mesh_name) {
            return Err(not_found(&format!(
                "Mesh with name {mesh_name} is not found"
            )));
        }
        let vg_key = (mesh_name.to_string(), virtual_gateway_name.to_string());
        if !self.virtual_gateways.contains_key(&vg_key) {
            return Err(not_found(&format!(
                "VirtualGateway with name {virtual_gateway_name} is not found"
            )));
        }
        let key = (
            mesh_name.to_string(),
            virtual_gateway_name.to_string(),
            gateway_route_name.to_string(),
        );
        if self.gateway_routes.contains_key(&key) {
            return Err(conflict(&format!(
                "GatewayRoute with name {gateway_route_name} already exists"
            )));
        }
        let now = Utc::now();
        let arn = format!(
            "arn:aws:appmesh:{region}:{account_id}:mesh/{mesh_name}/virtualGateway/{virtual_gateway_name}/gatewayRoute/{gateway_route_name}"
        );
        let uid = uuid::Uuid::new_v4().to_string();
        let gr = GatewayRoute {
            mesh_name: mesh_name.to_string(),
            virtual_gateway_name: virtual_gateway_name.to_string(),
            gateway_route_name: gateway_route_name.to_string(),
            arn: arn.clone(),
            uid,
            status: "ACTIVE".to_string(),
            version: 1,
            created_at: now,
            last_updated_at: now,
            mesh_owner: account_id.to_string(),
            resource_owner: account_id.to_string(),
            spec,
            tags: tags.clone(),
        };
        self.gateway_routes.insert(key.clone(), gr);
        self.tags.insert(arn, tags);
        Ok(self.gateway_routes.get(&key).unwrap())
    }

    pub fn describe_gateway_route(
        &self,
        mesh_name: &str,
        virtual_gateway_name: &str,
        gateway_route_name: &str,
    ) -> Result<&GatewayRoute, AppMeshError> {
        let key = (
            mesh_name.to_string(),
            virtual_gateway_name.to_string(),
            gateway_route_name.to_string(),
        );
        self.gateway_routes.get(&key).ok_or_else(|| {
            not_found(&format!(
                "GatewayRoute with name {gateway_route_name} is not found"
            ))
        })
    }

    pub fn delete_gateway_route(
        &mut self,
        mesh_name: &str,
        virtual_gateway_name: &str,
        gateway_route_name: &str,
    ) -> Result<GatewayRoute, AppMeshError> {
        let key = (
            mesh_name.to_string(),
            virtual_gateway_name.to_string(),
            gateway_route_name.to_string(),
        );
        let gr = self.gateway_routes.remove(&key).ok_or_else(|| {
            not_found(&format!(
                "GatewayRoute with name {gateway_route_name} is not found"
            ))
        })?;
        self.tags.remove(&gr.arn);
        Ok(gr)
    }

    pub fn list_gateway_routes(
        &self,
        mesh_name: &str,
        virtual_gateway_name: &str,
    ) -> Vec<&GatewayRoute> {
        self.gateway_routes
            .values()
            .filter(|gr| {
                gr.mesh_name == mesh_name && gr.virtual_gateway_name == virtual_gateway_name
            })
            .collect()
    }

    pub fn update_gateway_route(
        &mut self,
        mesh_name: &str,
        virtual_gateway_name: &str,
        gateway_route_name: &str,
        spec: GatewayRouteSpec,
    ) -> Result<&GatewayRoute, AppMeshError> {
        let key = (
            mesh_name.to_string(),
            virtual_gateway_name.to_string(),
            gateway_route_name.to_string(),
        );
        let gr = self.gateway_routes.get_mut(&key).ok_or_else(|| {
            not_found(&format!(
                "GatewayRoute with name {gateway_route_name} is not found"
            ))
        })?;
        gr.spec = spec;
        gr.version += 1;
        gr.last_updated_at = Utc::now();
        Ok(gr)
    }

    // =========== Tags ===========

    pub fn tag_resource(
        &mut self,
        arn: &str,
        new_tags: Vec<(String, String)>,
    ) -> Result<(), AppMeshError> {
        let tags = self.tags.entry(arn.to_string()).or_default();
        for (k, v) in new_tags {
            if let Some(existing) = tags.iter_mut().find(|(ek, _)| *ek == k) {
                existing.1 = v;
            } else {
                tags.push((k, v));
            }
        }
        Ok(())
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: Vec<String>) -> Result<(), AppMeshError> {
        if let Some(tags) = self.tags.get_mut(arn) {
            tags.retain(|(k, _)| !tag_keys.contains(k));
        }
        Ok(())
    }

    pub fn list_tags_for_resource(&self, arn: &str) -> Vec<(String, String)> {
        self.tags.get(arn).cloned().unwrap_or_default()
    }
}
