//! Terraform converters for App Mesh resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_appmesh::AppMeshService;
use winterbaume_appmesh::views::{
    AppMeshStateView, GatewayRouteView, MeshView, RouteView, VirtualGatewayView, VirtualNodeView,
    VirtualRouterView, VirtualServiceView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::appmesh as appmesh_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn tags_from_hashmap(tags: &HashMap<String, String>) -> Vec<(String, String)> {
    tags.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}

fn tags_to_json(tags: &[(String, String)]) -> serde_json::Value {
    let map: serde_json::Map<String, serde_json::Value> = tags
        .iter()
        .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
        .collect();
    serde_json::Value::Object(map)
}

// ---------------------------------------------------------------------------
// aws_appmesh_mesh
// ---------------------------------------------------------------------------

pub struct AwsAppmeshMeshConverter {
    service: Arc<AppMeshService>,
}

impl AwsAppmeshMeshConverter {
    pub fn new(service: Arc<AppMeshService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppmeshMeshConverter {
    fn resource_type(&self) -> &str {
        "aws_appmesh_mesh"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsAppmeshMeshConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: appmesh_gen::MeshTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_appmesh_mesh", e))?;

        let mesh_name = model.name;
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:appmesh:{}:{}:mesh/{}",
                region, ctx.default_account_id, mesh_name
            )
        });
        // spec is a nested block — read raw to extract egress_filter.type.
        let egress_filter_type = attrs
            .get("spec")
            .and_then(|s| s.get("egress_filter"))
            .and_then(|e| e.get("type"))
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "DROP_ALL".to_string());
        let mesh_owner = model
            .mesh_owner
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let resource_owner = model
            .resource_owner
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let tags = tags_from_hashmap(&model.tags);

        // version is Option<i64> — read raw rather than via the model.
        let version = attrs.get("version").and_then(|v| v.as_i64()).unwrap_or(1);

        let mesh_view = MeshView {
            mesh_name: mesh_name.clone(),
            arn,
            uid: model.id.unwrap_or_default(),
            status: "ACTIVE".to_string(),
            version,
            created_at: model
                .created_date
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            last_updated_at: model
                .last_updated_date
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            mesh_owner,
            resource_owner,
            egress_filter_type,
            tags,
        };

        let mut state_view = AppMeshStateView::default();
        state_view
            .meshes
            .insert(mesh_view.mesh_name.clone(), mesh_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for mesh in view.meshes.values() {
            let attrs = serde_json::json!({
                "id": mesh.mesh_name,
                "name": mesh.mesh_name,
                "arn": mesh.arn,
                "mesh_owner": mesh.mesh_owner,
                "resource_owner": mesh.resource_owner,
                "created_date": mesh.created_at,
                "last_updated_date": mesh.last_updated_at,
                "spec": {
                    "egress_filter": {
                        "type": mesh.egress_filter_type,
                    }
                },
                "tags": tags_to_json(&mesh.tags),
            });
            results.push(ExtractedResource {
                name: mesh.mesh_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appmesh_virtual_node
// ---------------------------------------------------------------------------

pub struct AwsAppmeshVirtualNodeConverter {
    service: Arc<AppMeshService>,
}

impl AwsAppmeshVirtualNodeConverter {
    pub fn new(service: Arc<AppMeshService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppmeshVirtualNodeConverter {
    fn resource_type(&self) -> &str {
        "aws_appmesh_virtual_node"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsAppmeshVirtualNodeConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: appmesh_gen::VirtualNodeTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_appmesh_virtual_node", e))?;

        let name = model.name;
        let mesh_name = model.mesh_name;
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:appmesh:{}:{}:mesh/{}/virtualNode/{}",
                region, ctx.default_account_id, mesh_name, name
            )
        });
        let mesh_owner = model
            .mesh_owner
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let resource_owner = model
            .resource_owner
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let spec = attrs
            .get("spec")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        let tags = tags_from_hashmap(&model.tags);

        let version = attrs.get("version").and_then(|v| v.as_i64()).unwrap_or(1);

        let vn_view = VirtualNodeView {
            mesh_name: mesh_name.clone(),
            virtual_node_name: name.clone(),
            arn,
            uid: model.id.unwrap_or_default(),
            status: "ACTIVE".to_string(),
            version,
            created_at: model
                .created_date
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            last_updated_at: model
                .last_updated_date
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            mesh_owner,
            resource_owner,
            spec,
            tags,
        };

        let key = format!("{}/{}", vn_view.mesh_name, vn_view.virtual_node_name);
        let mut state_view = AppMeshStateView::default();
        state_view.virtual_nodes.insert(key, vn_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for vn in view.virtual_nodes.values() {
            let spec_json = serde_json::to_value(&vn.spec).unwrap_or_default();
            let attrs = serde_json::json!({
                "id": format!("{}/{}", vn.mesh_name, vn.virtual_node_name),
                "name": vn.virtual_node_name,
                "mesh_name": vn.mesh_name,
                "arn": vn.arn,
                "mesh_owner": vn.mesh_owner,
                "resource_owner": vn.resource_owner,
                "created_date": vn.created_at,
                "last_updated_date": vn.last_updated_at,
                "spec": spec_json,
                "tags": tags_to_json(&vn.tags),
            });
            results.push(ExtractedResource {
                name: format!("{}-{}", vn.mesh_name, vn.virtual_node_name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appmesh_virtual_service
// ---------------------------------------------------------------------------

pub struct AwsAppmeshVirtualServiceConverter {
    service: Arc<AppMeshService>,
}

impl AwsAppmeshVirtualServiceConverter {
    pub fn new(service: Arc<AppMeshService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppmeshVirtualServiceConverter {
    fn resource_type(&self) -> &str {
        "aws_appmesh_virtual_service"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsAppmeshVirtualServiceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: appmesh_gen::VirtualServiceTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_appmesh_virtual_service", e))?;

        let name = model.name;
        let mesh_name = model.mesh_name;
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:appmesh:{}:{}:mesh/{}/virtualService/{}",
                region, ctx.default_account_id, mesh_name, name
            )
        });
        let mesh_owner = model
            .mesh_owner
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let resource_owner = model
            .resource_owner
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let spec = attrs
            .get("spec")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        let tags = tags_from_hashmap(&model.tags);

        let version = attrs.get("version").and_then(|v| v.as_i64()).unwrap_or(1);

        let vs_view = VirtualServiceView {
            mesh_name: mesh_name.clone(),
            virtual_service_name: name.clone(),
            arn,
            uid: model.id.unwrap_or_default(),
            status: "ACTIVE".to_string(),
            version,
            created_at: model
                .created_date
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            last_updated_at: model
                .last_updated_date
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            mesh_owner,
            resource_owner,
            spec,
            tags,
        };

        let key = format!("{}/{}", vs_view.mesh_name, vs_view.virtual_service_name);
        let mut state_view = AppMeshStateView::default();
        state_view.virtual_services.insert(key, vs_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for vs in view.virtual_services.values() {
            let spec_json = serde_json::to_value(&vs.spec).unwrap_or_default();
            let attrs = serde_json::json!({
                "id": format!("{}/{}", vs.mesh_name, vs.virtual_service_name),
                "name": vs.virtual_service_name,
                "mesh_name": vs.mesh_name,
                "arn": vs.arn,
                "mesh_owner": vs.mesh_owner,
                "resource_owner": vs.resource_owner,
                "created_date": vs.created_at,
                "last_updated_date": vs.last_updated_at,
                "spec": spec_json,
                "tags": tags_to_json(&vs.tags),
            });
            results.push(ExtractedResource {
                name: format!("{}-{}", vs.mesh_name, vs.virtual_service_name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appmesh_virtual_router
// ---------------------------------------------------------------------------

pub struct AwsAppmeshVirtualRouterConverter {
    service: Arc<AppMeshService>,
}

impl AwsAppmeshVirtualRouterConverter {
    pub fn new(service: Arc<AppMeshService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppmeshVirtualRouterConverter {
    fn resource_type(&self) -> &str {
        "aws_appmesh_virtual_router"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_appmesh_mesh"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsAppmeshVirtualRouterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: appmesh_gen::VirtualRouterTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_appmesh_virtual_router", e))?;

        let name = model.name;
        let mesh_name = model.mesh_name;
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:appmesh:{}:{}:mesh/{}/virtualRouter/{}",
                region, ctx.default_account_id, mesh_name, name
            )
        });
        let mesh_owner = model
            .mesh_owner
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let resource_owner = model
            .resource_owner
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let spec = attrs
            .get("spec")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        let tags = tags_from_hashmap(&model.tags);
        let version = attrs.get("version").and_then(|v| v.as_i64()).unwrap_or(1);

        let vr_view = VirtualRouterView {
            mesh_name: mesh_name.clone(),
            virtual_router_name: name.clone(),
            arn,
            uid: model.id.unwrap_or_default(),
            status: "ACTIVE".to_string(),
            version,
            created_at: model
                .created_date
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            last_updated_at: model
                .last_updated_date
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            mesh_owner,
            resource_owner,
            spec,
            tags,
        };

        let key = format!("{}/{}", vr_view.mesh_name, vr_view.virtual_router_name);
        let mut state_view = AppMeshStateView::default();
        state_view.virtual_routers.insert(key, vr_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for vr in view.virtual_routers.values() {
            let spec_json = serde_json::to_value(&vr.spec).unwrap_or_default();
            let attrs = serde_json::json!({
                "id": format!("{}/{}", vr.mesh_name, vr.virtual_router_name),
                "name": vr.virtual_router_name,
                "mesh_name": vr.mesh_name,
                "arn": vr.arn,
                "mesh_owner": vr.mesh_owner,
                "resource_owner": vr.resource_owner,
                "created_date": vr.created_at,
                "last_updated_date": vr.last_updated_at,
                "spec": spec_json,
                "tags": tags_to_json(&vr.tags),
            });
            results.push(ExtractedResource {
                name: format!("{}-{}", vr.mesh_name, vr.virtual_router_name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appmesh_virtual_gateway
// ---------------------------------------------------------------------------

pub struct AwsAppmeshVirtualGatewayConverter {
    service: Arc<AppMeshService>,
}

impl AwsAppmeshVirtualGatewayConverter {
    pub fn new(service: Arc<AppMeshService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppmeshVirtualGatewayConverter {
    fn resource_type(&self) -> &str {
        "aws_appmesh_virtual_gateway"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_appmesh_mesh"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsAppmeshVirtualGatewayConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: appmesh_gen::VirtualGatewayTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_appmesh_virtual_gateway", e))?;

        let name = model.name;
        let mesh_name = model.mesh_name;
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:appmesh:{}:{}:mesh/{}/virtualGateway/{}",
                region, ctx.default_account_id, mesh_name, name
            )
        });
        let mesh_owner = model
            .mesh_owner
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let resource_owner = model
            .resource_owner
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let spec = attrs
            .get("spec")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        let tags = tags_from_hashmap(&model.tags);
        let version = attrs.get("version").and_then(|v| v.as_i64()).unwrap_or(1);

        let vg_view = VirtualGatewayView {
            mesh_name: mesh_name.clone(),
            virtual_gateway_name: name.clone(),
            arn,
            uid: model.id.unwrap_or_default(),
            status: "ACTIVE".to_string(),
            version,
            created_at: model
                .created_date
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            last_updated_at: model
                .last_updated_date
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            mesh_owner,
            resource_owner,
            spec,
            tags,
        };

        let key = format!("{}/{}", vg_view.mesh_name, vg_view.virtual_gateway_name);
        let mut state_view = AppMeshStateView::default();
        state_view.virtual_gateways.insert(key, vg_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for vg in view.virtual_gateways.values() {
            let spec_json = serde_json::to_value(&vg.spec).unwrap_or_default();
            let attrs = serde_json::json!({
                "id": format!("{}/{}", vg.mesh_name, vg.virtual_gateway_name),
                "name": vg.virtual_gateway_name,
                "mesh_name": vg.mesh_name,
                "arn": vg.arn,
                "mesh_owner": vg.mesh_owner,
                "resource_owner": vg.resource_owner,
                "created_date": vg.created_at,
                "last_updated_date": vg.last_updated_at,
                "spec": spec_json,
                "tags": tags_to_json(&vg.tags),
            });
            results.push(ExtractedResource {
                name: format!("{}-{}", vg.mesh_name, vg.virtual_gateway_name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appmesh_route
// ---------------------------------------------------------------------------

pub struct AwsAppmeshRouteConverter {
    service: Arc<AppMeshService>,
}

impl AwsAppmeshRouteConverter {
    pub fn new(service: Arc<AppMeshService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppmeshRouteConverter {
    fn resource_type(&self) -> &str {
        "aws_appmesh_route"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_appmesh_virtual_router"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsAppmeshRouteConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: appmesh_gen::RouteTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_appmesh_route", e))?;

        let name = model.name;
        let mesh_name = model.mesh_name;
        let virtual_router_name = model.virtual_router_name;
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:appmesh:{}:{}:mesh/{}/virtualRouter/{}/route/{}",
                region, ctx.default_account_id, mesh_name, virtual_router_name, name
            )
        });
        let mesh_owner = model
            .mesh_owner
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let resource_owner = model
            .resource_owner
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let spec = attrs
            .get("spec")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        let tags = tags_from_hashmap(&model.tags);
        let version = attrs.get("version").and_then(|v| v.as_i64()).unwrap_or(1);

        let route_view = RouteView {
            mesh_name: mesh_name.clone(),
            virtual_router_name: virtual_router_name.clone(),
            route_name: name.clone(),
            arn,
            uid: model.id.unwrap_or_default(),
            status: "ACTIVE".to_string(),
            version,
            created_at: model
                .created_date
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            last_updated_at: model
                .last_updated_date
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            mesh_owner,
            resource_owner,
            spec,
            tags,
        };

        let key = format!(
            "{}/{}/{}",
            route_view.mesh_name, route_view.virtual_router_name, route_view.route_name
        );
        let mut state_view = AppMeshStateView::default();
        state_view.routes.insert(key, route_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for r in view.routes.values() {
            let spec_json = serde_json::to_value(&r.spec).unwrap_or_default();
            let attrs = serde_json::json!({
                "id": format!("{}/{}/{}", r.mesh_name, r.virtual_router_name, r.route_name),
                "name": r.route_name,
                "mesh_name": r.mesh_name,
                "virtual_router_name": r.virtual_router_name,
                "arn": r.arn,
                "mesh_owner": r.mesh_owner,
                "resource_owner": r.resource_owner,
                "created_date": r.created_at,
                "last_updated_date": r.last_updated_at,
                "spec": spec_json,
                "tags": tags_to_json(&r.tags),
            });
            results.push(ExtractedResource {
                name: format!("{}-{}-{}", r.mesh_name, r.virtual_router_name, r.route_name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appmesh_gateway_route
// ---------------------------------------------------------------------------

pub struct AwsAppmeshGatewayRouteConverter {
    service: Arc<AppMeshService>,
}

impl AwsAppmeshGatewayRouteConverter {
    pub fn new(service: Arc<AppMeshService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppmeshGatewayRouteConverter {
    fn resource_type(&self) -> &str {
        "aws_appmesh_gateway_route"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_appmesh_virtual_gateway"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsAppmeshGatewayRouteConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: appmesh_gen::GatewayRouteTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_appmesh_gateway_route", e))?;

        let name = model.name;
        let mesh_name = model.mesh_name;
        let virtual_gateway_name = model.virtual_gateway_name;
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:appmesh:{}:{}:mesh/{}/virtualGateway/{}/gatewayRoute/{}",
                region, ctx.default_account_id, mesh_name, virtual_gateway_name, name
            )
        });
        let mesh_owner = model
            .mesh_owner
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let resource_owner = model
            .resource_owner
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let spec = attrs
            .get("spec")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        let tags = tags_from_hashmap(&model.tags);
        let version = attrs.get("version").and_then(|v| v.as_i64()).unwrap_or(1);

        let gr_view = GatewayRouteView {
            mesh_name: mesh_name.clone(),
            virtual_gateway_name: virtual_gateway_name.clone(),
            gateway_route_name: name.clone(),
            arn,
            uid: model.id.unwrap_or_default(),
            status: "ACTIVE".to_string(),
            version,
            created_at: model
                .created_date
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            last_updated_at: model
                .last_updated_date
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            mesh_owner,
            resource_owner,
            spec,
            tags,
        };

        let key = format!(
            "{}/{}/{}",
            gr_view.mesh_name, gr_view.virtual_gateway_name, gr_view.gateway_route_name
        );
        let mut state_view = AppMeshStateView::default();
        state_view.gateway_routes.insert(key, gr_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for gr in view.gateway_routes.values() {
            let spec_json = serde_json::to_value(&gr.spec).unwrap_or_default();
            let attrs = serde_json::json!({
                "id": format!("{}/{}/{}", gr.mesh_name, gr.virtual_gateway_name, gr.gateway_route_name),
                "name": gr.gateway_route_name,
                "mesh_name": gr.mesh_name,
                "virtual_gateway_name": gr.virtual_gateway_name,
                "arn": gr.arn,
                "mesh_owner": gr.mesh_owner,
                "resource_owner": gr.resource_owner,
                "created_date": gr.created_at,
                "last_updated_date": gr.last_updated_at,
                "spec": spec_json,
                "tags": tags_to_json(&gr.tags),
            });
            results.push(ExtractedResource {
                name: format!(
                    "{}-{}-{}",
                    gr.mesh_name, gr.virtual_gateway_name, gr.gateway_route_name
                ),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
