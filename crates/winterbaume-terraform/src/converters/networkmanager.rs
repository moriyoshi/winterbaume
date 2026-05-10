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
    DeviceView, GlobalNetworkView, NetworkManagerStateView, SiteView,
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
