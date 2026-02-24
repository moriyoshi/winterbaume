//! Terraform converters for Network Manager resources.

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
use crate::util::{extract_region, extract_tags, optional_str, require_str};

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

        let id = optional_str(attrs, "id").unwrap_or_default();
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:networkmanager::{}:global-network/{}",
                ctx.default_account_id, id
            )
        });
        let description = optional_str(attrs, "description").unwrap_or_default();
        let state = optional_str(attrs, "state").unwrap_or_else(|| "AVAILABLE".to_string());
        let created_at =
            optional_str(attrs, "created_at").unwrap_or_else(|| "1970-01-01T00:00:00Z".into());

        let gn_view = GlobalNetworkView {
            global_network_id: id.clone(),
            global_network_arn: arn,
            description,
            created_at,
            state,
            tags: extract_tags(attrs),
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
        let global_network_id = require_str(attrs, "global_network_id", "aws_networkmanager_site")?;
        let region = extract_region(attrs, &ctx.default_region);

        let id = optional_str(attrs, "id").unwrap_or_default();
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:networkmanager::{}:site/{}/{}",
                ctx.default_account_id, global_network_id, id
            )
        });
        let description = optional_str(attrs, "description").unwrap_or_default();
        let state = optional_str(attrs, "state").unwrap_or_else(|| "AVAILABLE".to_string());
        let created_at =
            optional_str(attrs, "created_at").unwrap_or_else(|| "1970-01-01T00:00:00Z".into());

        let location = attrs.get("location").cloned();
        let site_view = SiteView {
            site_id: id.clone(),
            site_arn: arn,
            global_network_id: global_network_id.to_string(),
            description,
            created_at,
            state,
            tags: extract_tags(attrs),
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
        let global_network_id =
            require_str(attrs, "global_network_id", "aws_networkmanager_device")?;
        let region = extract_region(attrs, &ctx.default_region);

        let id = optional_str(attrs, "id").unwrap_or_default();
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:networkmanager::{}:device/{}/{}",
                ctx.default_account_id, global_network_id, id
            )
        });
        let description = optional_str(attrs, "description").unwrap_or_default();
        let site_id = optional_str(attrs, "site_id");
        let model = optional_str(attrs, "model");
        let serial_number = optional_str(attrs, "serial_number");
        let device_type = optional_str(attrs, "type");
        let vendor = optional_str(attrs, "vendor");
        let state = optional_str(attrs, "state").unwrap_or_else(|| "AVAILABLE".to_string());
        let created_at =
            optional_str(attrs, "created_at").unwrap_or_else(|| "1970-01-01T00:00:00Z".into());

        let location = attrs.get("location").cloned();
        let aws_location = attrs.get("aws_location").cloned();
        let device_view = DeviceView {
            device_id: id.clone(),
            device_arn: arn,
            global_network_id: global_network_id.to_string(),
            description,
            site_id,
            model,
            serial_number,
            device_type,
            vendor,
            created_at,
            state,
            tags: extract_tags(attrs),
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
