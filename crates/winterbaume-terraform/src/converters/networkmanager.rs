//! Terraform converters for Network Manager resources.
//!
//! `GlobalNetworkTfModel`, `SiteTfModel`, and `DeviceTfModel` are
//! generated from `specs/networkmanager.toml`. The ARN templates,
//! the `state = "AVAILABLE"` default, the empty-description default,
//! the synthetic `created_at = "1970-01-01T00:00:00Z"` fallback, and
//! the location / aws_location raw blobs are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_networkmanager::NetworkManagerService;
use winterbaume_networkmanager::views::{
    AttachmentView, ConnectPeerView, ConnectionView, CoreNetworkView,
    CustomerGatewayAssociationView, DeviceView, GlobalNetworkView, LinkAssociationView, LinkView,
    NetworkManagerStateView, SiteView, TransitGatewayConnectPeerAssociationView,
    TransitGatewayRegistrationView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::networkmanager as networkmanager_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_networkmanager_global_network
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_global_network` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerGlobalNetworkConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerGlobalNetworkConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerGlobalNetworkConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_global_network"
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

impl AwsNetworkmanagerGlobalNetworkConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::GlobalNetworkTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_networkmanager_global_network", e))?;

        let id = model.id.unwrap_or_default();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:networkmanager::{}:global-network/{}",
                ctx.default_account_id, id
            )
        });
        let description = model.description.unwrap_or_default();
        let state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());
        let created_at = model
            .created_at
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());

        let gn_view = GlobalNetworkView {
            global_network_id: id.clone(),
            global_network_arn: arn,
            description,
            created_at,
            state,
            tags: model.tags,
        };

        let mut state_view = NetworkManagerStateView {
            global_networks: HashMap::new(),
            core_networks: HashMap::new(),
            sites: HashMap::new(),
            links: HashMap::new(),
            devices: HashMap::new(),
            ..Default::default()
        };
        state_view.global_networks.insert(id, gn_view);
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
        for gn in view.global_networks.values() {
            let attrs = serde_json::json!({
                "id": gn.global_network_id,
                "arn": gn.global_network_arn,
                "description": gn.description,
                "state": gn.state,
                "created_at": gn.created_at,
                "tags": gn.tags,
            });
            results.push(ExtractedResource {
                name: gn.global_network_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_site
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_site` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerSiteConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerSiteConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerSiteConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_site"
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

impl AwsNetworkmanagerSiteConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::SiteTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_networkmanager_site", e))?;

        let global_network_id = model.global_network_id;
        let id = model.id.unwrap_or_default();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:networkmanager::{}:site/{}/{}",
                ctx.default_account_id, global_network_id, id
            )
        });
        let description = model.description.unwrap_or_default();
        let state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());
        let created_at = model
            .created_at
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());

        let location = attrs.get("location").cloned();
        let site_view = SiteView {
            site_id: id.clone(),
            site_arn: arn,
            global_network_id,
            description,
            created_at,
            state,
            tags: model.tags,
            location,
        };

        let mut state_view = NetworkManagerStateView {
            global_networks: HashMap::new(),
            core_networks: HashMap::new(),
            sites: HashMap::new(),
            links: HashMap::new(),
            devices: HashMap::new(),
            ..Default::default()
        };
        state_view.sites.insert(id, site_view);
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
        for site in view.sites.values() {
            let attrs = serde_json::json!({
                "id": site.site_id,
                "arn": site.site_arn,
                "global_network_id": site.global_network_id,
                "description": site.description,
                "state": site.state,
                "created_at": site.created_at,
                "tags": site.tags,
                "location": site.location,
            });
            results.push(ExtractedResource {
                name: site.site_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_device
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_device` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerDeviceConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerDeviceConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerDeviceConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_device"
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

impl AwsNetworkmanagerDeviceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::DeviceTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_networkmanager_device", e))?;

        let global_network_id = model.global_network_id;
        let id = model.id.unwrap_or_default();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:networkmanager::{}:device/{}/{}",
                ctx.default_account_id, global_network_id, id
            )
        });
        let description = model.description.unwrap_or_default();
        let site_id = model.site_id;
        let device_model = model.model;
        let serial_number = model.serial_number;
        let device_type = model.device_type;
        let vendor = model.vendor;
        let state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());
        let created_at = model
            .created_at
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());

        let location = attrs.get("location").cloned();
        let aws_location = attrs.get("aws_location").cloned();
        let device_view = DeviceView {
            device_id: id.clone(),
            device_arn: arn,
            global_network_id,
            description,
            site_id,
            model: device_model,
            serial_number,
            device_type,
            vendor,
            created_at,
            state,
            tags: model.tags,
            location,
            aws_location,
        };

        let mut state_view = NetworkManagerStateView {
            global_networks: HashMap::new(),
            core_networks: HashMap::new(),
            sites: HashMap::new(),
            links: HashMap::new(),
            devices: HashMap::new(),
            ..Default::default()
        };
        state_view.devices.insert(id, device_view);
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
        for dev in view.devices.values() {
            let attrs = serde_json::json!({
                "id": dev.device_id,
                "arn": dev.device_arn,
                "global_network_id": dev.global_network_id,
                "description": dev.description,
                "site_id": dev.site_id,
                "model": dev.model,
                "serial_number": dev.serial_number,
                "type": dev.device_type,
                "vendor": dev.vendor,
                "state": dev.state,
                "created_at": dev.created_at,
                "tags": dev.tags,
                "location": dev.location,
                "aws_location": dev.aws_location,
            });
            results.push(ExtractedResource {
                name: dev.device_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Helpers shared by the wave-2 converters below.
// ---------------------------------------------------------------------------

fn now_rfc3339() -> String {
    "1970-01-01T00:00:00Z".to_string()
}

// ---------------------------------------------------------------------------
// aws_networkmanager_core_network
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_core_network` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerCoreNetworkConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerCoreNetworkConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerCoreNetworkConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_core_network"
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

impl AwsNetworkmanagerCoreNetworkConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::CoreNetworkTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_networkmanager_core_network", e))?;

        let global_network_id = model.global_network_id;
        let id = model.id.unwrap_or_default();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:networkmanager::{}:core-network/{}",
                ctx.default_account_id, id
            )
        });
        let description = model.description.unwrap_or_default();
        let state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());
        let created_at = model.created_at.unwrap_or_else(now_rfc3339);

        let view = CoreNetworkView {
            core_network_id: id.clone(),
            core_network_arn: arn,
            global_network_id,
            description,
            created_at,
            state,
            tags: model.tags,
        };

        let mut state_view = NetworkManagerStateView::default();
        state_view.core_networks.insert(id, view);
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
        for cn in view.core_networks.values() {
            let attrs = serde_json::json!({
                "id": cn.core_network_id,
                "arn": cn.core_network_arn,
                "global_network_id": cn.global_network_id,
                "description": cn.description,
                "state": cn.state,
                "created_at": cn.created_at,
                "tags": cn.tags,
            });
            results.push(ExtractedResource {
                name: cn.core_network_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_link
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_link` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerLinkConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerLinkConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerLinkConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_link"
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

impl AwsNetworkmanagerLinkConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::LinkTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_networkmanager_link", e))?;

        let global_network_id = model.global_network_id;
        let site_id = model.site_id.unwrap_or_default();
        let id = model.id.unwrap_or_default();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:networkmanager::{}:link/{}/{}",
                ctx.default_account_id, global_network_id, id
            )
        });
        let description = model.description.unwrap_or_default();
        let provider = model.provider_name.unwrap_or_default();
        let link_type = model.link_type.unwrap_or_default();
        let state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());
        let created_at = model.created_at.unwrap_or_else(now_rfc3339);

        let bandwidth = attrs.get("bandwidth").and_then(|v| v.as_array().cloned());
        let (download, upload) = if let Some(arr) = bandwidth.as_ref().and_then(|a| a.first()) {
            let dl = arr
                .get("download_speed")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;
            let ul = arr
                .get("upload_speed")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;
            (dl, ul)
        } else {
            (0, 0)
        };

        let link_view = LinkView {
            link_id: id.clone(),
            link_arn: arn,
            global_network_id,
            site_id,
            description,
            provider,
            link_type,
            bandwidth_download_speed: download,
            bandwidth_upload_speed: upload,
            created_at,
            state,
            tags: model.tags,
        };

        let mut state_view = NetworkManagerStateView::default();
        state_view.links.insert(id, link_view);
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
        for link in view.links.values() {
            let attrs = serde_json::json!({
                "id": link.link_id,
                "arn": link.link_arn,
                "global_network_id": link.global_network_id,
                "site_id": link.site_id,
                "description": link.description,
                "provider_name": link.provider,
                "type": link.link_type,
                "state": link.state,
                "created_at": link.created_at,
                "tags": link.tags,
            });
            results.push(ExtractedResource {
                name: link.link_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_connection
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_connection` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerConnectionConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerConnectionConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerConnectionConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_connection"
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

impl AwsNetworkmanagerConnectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::ConnectionTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_networkmanager_connection", e))?;

        let global_network_id = model.global_network_id;
        let id = model.id.unwrap_or_default();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:networkmanager::{}:connection/{}/{}",
                ctx.default_account_id, global_network_id, id
            )
        });
        let description = model.description.unwrap_or_default();
        let state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());
        let created_at = model.created_at.unwrap_or_else(now_rfc3339);

        let view = ConnectionView {
            connection_id: id.clone(),
            connection_arn: arn,
            global_network_id,
            device_id: model.device_id.unwrap_or_default(),
            connected_device_id: model.connected_device_id.unwrap_or_default(),
            link_id: model.link_id,
            connected_link_id: model.connected_link_id,
            description,
            created_at,
            state,
            tags: model.tags,
        };

        let mut state_view = NetworkManagerStateView::default();
        state_view.connections.insert(id, view);
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
        for c in view.connections.values() {
            let attrs = serde_json::json!({
                "id": c.connection_id,
                "arn": c.connection_arn,
                "global_network_id": c.global_network_id,
                "device_id": c.device_id,
                "connected_device_id": c.connected_device_id,
                "link_id": c.link_id,
                "connected_link_id": c.connected_link_id,
                "description": c.description,
                "state": c.state,
                "created_at": c.created_at,
                "tags": c.tags,
            });
            results.push(ExtractedResource {
                name: c.connection_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Internal helper: build a fresh-write attachment converter body. Several
// attachment types (connect, dx_gateway, site_to_site_vpn, vpc, transit
// gateway route table) share the same AttachmentView slot in state and
// differ only in `attachment_type`, the synthetic `resource_arn` source,
// and a few HCL field names.
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn build_attachment_view(
    attachment_id: String,
    attachment_type: &str,
    core_network_id: String,
    core_network_arn: String,
    owner_account_id: String,
    resource_arn: String,
    edge_location: Option<String>,
    state: String,
    created_at: String,
    tags: HashMap<String, String>,
    segment_name: Option<String>,
    subnet_arns: Vec<String>,
) -> AttachmentView {
    AttachmentView {
        attachment_id,
        attachment_type: attachment_type.to_string(),
        core_network_id,
        core_network_arn,
        owner_account_id,
        resource_arn,
        edge_location,
        state,
        created_at: created_at.clone(),
        updated_at: created_at,
        tags,
        subnet_arns,
        segment_name,
    }
}

fn core_network_arn(account_id: &str, core_network_id: &str) -> String {
    format!(
        "arn:aws:networkmanager::{}:core-network/{}",
        account_id, core_network_id
    )
}

// ---------------------------------------------------------------------------
// aws_networkmanager_connect_attachment
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_connect_attachment` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerConnectAttachmentConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerConnectAttachmentConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerConnectAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_connect_attachment"
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

impl AwsNetworkmanagerConnectAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::ConnectAttachmentTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_networkmanager_connect_attachment", e)
            })?;

        let id = model.id.unwrap_or_default();
        let owner = model
            .owner_account_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let cn_arn = core_network_arn(&ctx.default_account_id, &model.core_network_id);
        let resource_arn = model.resource_arn.unwrap_or_default();
        let state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());

        let view = build_attachment_view(
            id.clone(),
            "CONNECT",
            model.core_network_id,
            cn_arn,
            owner,
            resource_arn,
            model.edge_location,
            state,
            now_rfc3339(),
            model.tags,
            model.segment_name,
            vec![],
        );

        let mut state_view = NetworkManagerStateView::default();
        state_view.attachments.insert(id, view);
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
        for a in view
            .attachments
            .values()
            .filter(|a| a.attachment_type == "CONNECT")
        {
            let attrs = serde_json::json!({
                "id": a.attachment_id,
                "arn": a.attachment_id,
                "core_network_id": a.core_network_id,
                "owner_account_id": a.owner_account_id,
                "resource_arn": a.resource_arn,
                "edge_location": a.edge_location,
                "state": a.state,
                "segment_name": a.segment_name,
                "tags": a.tags,
            });
            results.push(ExtractedResource {
                name: a.attachment_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_vpc_attachment
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_vpc_attachment` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerVpcAttachmentConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerVpcAttachmentConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerVpcAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_vpc_attachment"
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

impl AwsNetworkmanagerVpcAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::VpcAttachmentTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_networkmanager_vpc_attachment", e))?;

        let id = model.id.unwrap_or_default();
        let owner = model
            .owner_account_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let cn_arn = core_network_arn(&ctx.default_account_id, &model.core_network_id);
        let resource_arn = model.vpc_arn.unwrap_or_default();
        let state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());

        let subnet_arns: Vec<String> = attrs
            .get("subnet_arns")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let view = build_attachment_view(
            id.clone(),
            "VPC",
            model.core_network_id,
            cn_arn,
            owner,
            resource_arn,
            model.edge_location,
            state,
            now_rfc3339(),
            model.tags,
            model.segment_name,
            subnet_arns,
        );

        let mut state_view = NetworkManagerStateView::default();
        state_view.attachments.insert(id, view);
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
        for a in view
            .attachments
            .values()
            .filter(|a| a.attachment_type == "VPC")
        {
            let attrs = serde_json::json!({
                "id": a.attachment_id,
                "arn": a.attachment_id,
                "core_network_id": a.core_network_id,
                "vpc_arn": a.resource_arn,
                "owner_account_id": a.owner_account_id,
                "edge_location": a.edge_location,
                "state": a.state,
                "segment_name": a.segment_name,
                "subnet_arns": a.subnet_arns,
                "tags": a.tags,
            });
            results.push(ExtractedResource {
                name: a.attachment_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_site_to_site_vpn_attachment
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_site_to_site_vpn_attachment` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerSiteToSiteVpnAttachmentConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerSiteToSiteVpnAttachmentConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerSiteToSiteVpnAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_site_to_site_vpn_attachment"
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

impl AwsNetworkmanagerSiteToSiteVpnAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::SiteToSiteVpnAttachmentTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_networkmanager_site_to_site_vpn_attachment", e)
            })?;

        let id = model.id.unwrap_or_default();
        let owner = model
            .owner_account_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let cn_arn = core_network_arn(&ctx.default_account_id, &model.core_network_id);
        let resource_arn = model.vpn_connection_arn.unwrap_or_default();
        let state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());

        let view = build_attachment_view(
            id.clone(),
            "SITE_TO_SITE_VPN",
            model.core_network_id,
            cn_arn,
            owner,
            resource_arn,
            model.edge_location,
            state,
            now_rfc3339(),
            model.tags,
            model.segment_name,
            vec![],
        );

        let mut state_view = NetworkManagerStateView::default();
        state_view.attachments.insert(id, view);
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
        for a in view
            .attachments
            .values()
            .filter(|a| a.attachment_type == "SITE_TO_SITE_VPN")
        {
            let attrs = serde_json::json!({
                "id": a.attachment_id,
                "arn": a.attachment_id,
                "core_network_id": a.core_network_id,
                "vpn_connection_arn": a.resource_arn,
                "owner_account_id": a.owner_account_id,
                "edge_location": a.edge_location,
                "state": a.state,
                "segment_name": a.segment_name,
                "tags": a.tags,
            });
            results.push(ExtractedResource {
                name: a.attachment_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_dx_gateway_attachment
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_dx_gateway_attachment` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerDxGatewayAttachmentConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerDxGatewayAttachmentConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerDxGatewayAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_dx_gateway_attachment"
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

impl AwsNetworkmanagerDxGatewayAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::DxGatewayAttachmentTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_networkmanager_dx_gateway_attachment", e)
            })?;

        let id = model.id.unwrap_or_default();
        let owner = model
            .owner_account_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let cn_arn = core_network_arn(&ctx.default_account_id, &model.core_network_id);
        let resource_arn = model.direct_connect_gateway_arn.unwrap_or_default();
        let state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());

        let view = build_attachment_view(
            id.clone(),
            "DIRECT_CONNECT_GATEWAY",
            model.core_network_id,
            cn_arn,
            owner,
            resource_arn,
            model.edge_locations,
            state,
            now_rfc3339(),
            model.tags,
            model.segment_name,
            vec![],
        );

        let mut state_view = NetworkManagerStateView::default();
        state_view.attachments.insert(id, view);
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
        for a in view
            .attachments
            .values()
            .filter(|a| a.attachment_type == "DIRECT_CONNECT_GATEWAY")
        {
            let attrs = serde_json::json!({
                "id": a.attachment_id,
                "arn": a.attachment_id,
                "core_network_id": a.core_network_id,
                "direct_connect_gateway_arn": a.resource_arn,
                "owner_account_id": a.owner_account_id,
                "edge_locations": a.edge_location,
                "state": a.state,
                "segment_name": a.segment_name,
                "tags": a.tags,
            });
            results.push(ExtractedResource {
                name: a.attachment_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_transit_gateway_route_table_attachment
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_transit_gateway_route_table_attachment` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerTransitGatewayRouteTableAttachmentConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerTransitGatewayRouteTableAttachmentConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerTransitGatewayRouteTableAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_transit_gateway_route_table_attachment"
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

impl AwsNetworkmanagerTransitGatewayRouteTableAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::TransitGatewayRouteTableAttachmentTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error(
                    "aws_networkmanager_transit_gateway_route_table_attachment",
                    e,
                )
            })?;

        let id = model.id.unwrap_or_default();
        let owner = model
            .owner_account_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let core_network_id = model.core_network_id.unwrap_or_default();
        let cn_arn = core_network_arn(&ctx.default_account_id, &core_network_id);
        let resource_arn = model.transit_gateway_route_table_arn.unwrap_or_default();
        let state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());

        let view = build_attachment_view(
            id.clone(),
            "TRANSIT_GATEWAY_ROUTE_TABLE",
            core_network_id,
            cn_arn,
            owner,
            resource_arn,
            model.edge_location,
            state,
            now_rfc3339(),
            model.tags,
            model.segment_name,
            vec![],
        );

        let mut state_view = NetworkManagerStateView::default();
        state_view.attachments.insert(id, view);
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
        for a in view
            .attachments
            .values()
            .filter(|a| a.attachment_type == "TRANSIT_GATEWAY_ROUTE_TABLE")
        {
            let attrs = serde_json::json!({
                "id": a.attachment_id,
                "arn": a.attachment_id,
                "core_network_id": a.core_network_id,
                "transit_gateway_route_table_arn": a.resource_arn,
                "owner_account_id": a.owner_account_id,
                "edge_location": a.edge_location,
                "state": a.state,
                "segment_name": a.segment_name,
                "tags": a.tags,
            });
            results.push(ExtractedResource {
                name: a.attachment_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_attachment_accepter
// (snapshot+mutate: flips the state of an existing attachment to AVAILABLE.)
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_attachment_accepter` Terraform resources by
/// flipping the `state` of the referenced attachment in Network Manager state.
pub struct AwsNetworkmanagerAttachmentAccepterConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerAttachmentAccepterConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerAttachmentAccepterConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_attachment_accepter"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // accepter is a state-flip on an attachment that is already extracted
        // by its underlying attachment-type converter; nothing of our own to
        // emit on extract.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsNetworkmanagerAttachmentAccepterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::AttachmentAccepterTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_networkmanager_attachment_accepter", e)
            })?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(attachment) = snapshot.attachments.get_mut(&model.attachment_id) {
            attachment.state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());
        } else {
            warnings.push(format!(
                "aws_networkmanager_attachment_accepter: attachment '{}' not found in state; accept is a no-op",
                model.attachment_id
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_connect_peer
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_connect_peer` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerConnectPeerConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerConnectPeerConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerConnectPeerConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_connect_peer"
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

impl AwsNetworkmanagerConnectPeerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::ConnectPeerTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_networkmanager_connect_peer", e))?;

        let id = model.id.unwrap_or_default();
        let state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());
        let created_at = model.created_at.unwrap_or_else(now_rfc3339);

        let inside_cidr_blocks: Vec<String> = attrs
            .get("inside_cidr_blocks")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let view = ConnectPeerView {
            connect_peer_id: id.clone(),
            connect_attachment_id: model.connect_attachment_id,
            core_network_id: model.core_network_id,
            edge_location: model.edge_location,
            peer_address: model.peer_address.unwrap_or_default(),
            core_network_address: model.core_network_address,
            inside_cidr_blocks,
            created_at,
            state,
            tags: model.tags,
        };

        let mut state_view = NetworkManagerStateView::default();
        state_view.connect_peers.insert(id, view);
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
        for cp in view.connect_peers.values() {
            let attrs = serde_json::json!({
                "id": cp.connect_peer_id,
                "arn": cp.connect_peer_id,
                "connect_attachment_id": cp.connect_attachment_id,
                "core_network_id": cp.core_network_id,
                "edge_location": cp.edge_location,
                "peer_address": cp.peer_address,
                "core_network_address": cp.core_network_address,
                "state": cp.state,
                "created_at": cp.created_at,
                "tags": cp.tags,
            });
            results.push(ExtractedResource {
                name: cp.connect_peer_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_transit_gateway_registration
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_transit_gateway_registration` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerTransitGatewayRegistrationConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerTransitGatewayRegistrationConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerTransitGatewayRegistrationConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_transit_gateway_registration"
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

impl AwsNetworkmanagerTransitGatewayRegistrationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::TransitGatewayRegistrationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_networkmanager_transit_gateway_registration", e)
            })?;

        let global_network_id = model.global_network_id;
        let transit_gateway_arn = model.transit_gateway_arn.unwrap_or_default();
        let state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());

        let key = format!("{}::{}", global_network_id, transit_gateway_arn);

        let view = TransitGatewayRegistrationView {
            global_network_id,
            transit_gateway_arn,
            state,
            state_message: String::new(),
        };

        let mut state_view = NetworkManagerStateView::default();
        state_view.transit_gateway_registrations.insert(key, view);
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
        for (key, r) in &view.transit_gateway_registrations {
            let attrs = serde_json::json!({
                "global_network_id": r.global_network_id,
                "transit_gateway_arn": r.transit_gateway_arn,
                "state": r.state,
            });
            results.push(ExtractedResource {
                name: key.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_link_association
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_link_association` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerLinkAssociationConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerLinkAssociationConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerLinkAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_link_association"
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

impl AwsNetworkmanagerLinkAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::LinkAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_networkmanager_link_association", e)
            })?;

        let view = LinkAssociationView {
            global_network_id: model.global_network_id,
            device_id: model.device_id.unwrap_or_default(),
            link_id: model.link_id.unwrap_or_default(),
            state: model.state.unwrap_or_else(|| "AVAILABLE".to_string()),
        };

        let mut state_view = NetworkManagerStateView::default();
        state_view.link_associations.push(view);
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
        for la in &view.link_associations {
            let name = format!("{}::{}::{}", la.global_network_id, la.device_id, la.link_id);
            let attrs = serde_json::json!({
                "global_network_id": la.global_network_id,
                "device_id": la.device_id,
                "link_id": la.link_id,
                "state": la.state,
            });
            results.push(ExtractedResource {
                name,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_customer_gateway_association
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_customer_gateway_association` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerCustomerGatewayAssociationConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerCustomerGatewayAssociationConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerCustomerGatewayAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_customer_gateway_association"
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

impl AwsNetworkmanagerCustomerGatewayAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::CustomerGatewayAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_networkmanager_customer_gateway_association", e)
            })?;

        let global_network_id = model.global_network_id;
        let customer_gateway_arn = model.customer_gateway_arn.unwrap_or_default();

        let key = format!("{}::{}", global_network_id, customer_gateway_arn);

        let view = CustomerGatewayAssociationView {
            customer_gateway_arn,
            global_network_id,
            device_id: model.device_id.unwrap_or_default(),
            link_id: model.link_id,
            state: model.state.unwrap_or_else(|| "AVAILABLE".to_string()),
        };

        let mut state_view = NetworkManagerStateView::default();
        state_view.customer_gateway_associations.insert(key, view);
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
        for (key, a) in &view.customer_gateway_associations {
            let attrs = serde_json::json!({
                "global_network_id": a.global_network_id,
                "customer_gateway_arn": a.customer_gateway_arn,
                "device_id": a.device_id,
                "link_id": a.link_id,
                "state": a.state,
            });
            results.push(ExtractedResource {
                name: key.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_transit_gateway_connect_peer_association
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_transit_gateway_connect_peer_association` Terraform resources to/from Network Manager state.
pub struct AwsNetworkmanagerTransitGatewayConnectPeerAssociationConverter {
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerTransitGatewayConnectPeerAssociationConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerTransitGatewayConnectPeerAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_transit_gateway_connect_peer_association"
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

impl AwsNetworkmanagerTransitGatewayConnectPeerAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: networkmanager_gen::TransitGatewayConnectPeerAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error(
                    "aws_networkmanager_transit_gateway_connect_peer_association",
                    e,
                )
            })?;

        let global_network_id = model.global_network_id;
        let transit_gateway_connect_peer_arn =
            model.transit_gateway_connect_peer_arn.unwrap_or_default();
        let key = format!(
            "{}::{}",
            global_network_id, transit_gateway_connect_peer_arn
        );

        let view = TransitGatewayConnectPeerAssociationView {
            transit_gateway_connect_peer_arn,
            global_network_id,
            device_id: model.device_id.unwrap_or_default(),
            link_id: model.link_id,
            state: model.state.unwrap_or_else(|| "AVAILABLE".to_string()),
        };

        let mut state_view = NetworkManagerStateView::default();
        state_view
            .transit_gateway_connect_peer_associations
            .insert(key, view);
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
        for (key, a) in &view.transit_gateway_connect_peer_associations {
            let attrs = serde_json::json!({
                "global_network_id": a.global_network_id,
                "transit_gateway_connect_peer_arn": a.transit_gateway_connect_peer_arn,
                "device_id": a.device_id,
                "link_id": a.link_id,
                "state": a.state,
            });
            results.push(ExtractedResource {
                name: key.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_core_network_policy_attachment
// (warning-only; the core-network policy is not modelled in state. Inject
// parses the TF attributes so they round-trip, but no projection happens.)
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_core_network_policy_attachment` Terraform resources.
/// Warning-only — the core-network policy document has no Network Manager state slot.
pub struct AwsNetworkmanagerCoreNetworkPolicyAttachmentConverter {
    #[allow(dead_code)]
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerCoreNetworkPolicyAttachmentConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerCoreNetworkPolicyAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_core_network_policy_attachment"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsNetworkmanagerCoreNetworkPolicyAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let _model: networkmanager_gen::CoreNetworkPolicyAttachmentTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_networkmanager_core_network_policy_attachment", e)
            })?;
        let msg = "core-network policy attachment is not modelled in state; inject is a no-op";
        eprintln!(
            "warning: aws_networkmanager_core_network_policy_attachment: {}",
            msg
        );
        Ok(ConversionResult {
            region,
            warnings: vec![format!(
                "aws_networkmanager_core_network_policy_attachment: {}",
                msg
            )],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_networkmanager_transit_gateway_peering
// (warning-only; transit-gateway peerings are not modelled as their own
// slot. Inject parses the TF attributes so they round-trip, but no
// projection happens.)
// ---------------------------------------------------------------------------

/// Converts `aws_networkmanager_transit_gateway_peering` Terraform resources.
/// Warning-only — transit-gateway peerings have no Network Manager state slot.
pub struct AwsNetworkmanagerTransitGatewayPeeringConverter {
    #[allow(dead_code)]
    service: Arc<NetworkManagerService>,
}

impl AwsNetworkmanagerTransitGatewayPeeringConverter {
    pub fn new(service: Arc<NetworkManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkmanagerTransitGatewayPeeringConverter {
    fn resource_type(&self) -> &str {
        "aws_networkmanager_transit_gateway_peering"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsNetworkmanagerTransitGatewayPeeringConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let _model: networkmanager_gen::TransitGatewayPeeringTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_networkmanager_transit_gateway_peering", e)
            })?;
        let msg = "transit-gateway peering is not modelled in state; inject is a no-op";
        eprintln!(
            "warning: aws_networkmanager_transit_gateway_peering: {}",
            msg
        );
        Ok(ConversionResult {
            region,
            warnings: vec![format!(
                "aws_networkmanager_transit_gateway_peering: {}",
                msg
            )],
        })
    }
}
