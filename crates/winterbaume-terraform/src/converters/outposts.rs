//! Terraform converters for AWS Outposts resources.
//!
//! `SiteTfModel` and `OutpostTfModel` are generated from
//! `specs/outposts.toml`. The ARN templates, the synthesised
//! `site_id` / `outpost_id`, and the constant `life_cycle_status`
//! are wired up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_outposts::OutpostsService;
use winterbaume_outposts::views::{OutpostView, OutpostsStateView, SiteView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::outposts as outposts_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_outposts_site
// ---------------------------------------------------------------------------

pub struct AwsOutpostsSiteConverter {
    service: Arc<OutpostsService>,
}

impl AwsOutpostsSiteConverter {
    pub fn new(service: Arc<OutpostsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOutpostsSiteConverter {
    fn resource_type(&self) -> &str {
        "aws_outposts_site"
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

impl AwsOutpostsSiteConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: outposts_gen::SiteTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_outposts_site", e))?;

        let site_id = model
            .site_id
            .clone()
            .or_else(|| model.id.clone())
            .unwrap_or_else(|| format!("os-{:017x}", rand_id()));
        let site_arn = model.arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:outposts:{}:{}:site/{}",
                region, ctx.default_account_id, site_id
            )
        });

        let site_view = SiteView {
            site_id: site_id.clone(),
            site_arn,
            account_id: ctx.default_account_id.clone(),
            name: model.name,
            description: model.description,
            notes: None,
            operating_address_country_code: None,
            operating_address_state_or_region: None,
            operating_address_city: None,
            tags: model.tags,
        };

        let mut state_view = OutpostsStateView::default();
        state_view.sites.insert(site_id, site_view);
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
                "site_id": site.site_id,
                "name": site.name,
                "description": site.description,
                "tags": site.tags,
            });
            results.push(ExtractedResource {
                name: site.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_outposts_outpost
// ---------------------------------------------------------------------------

pub struct AwsOutpostsOutpostConverter {
    service: Arc<OutpostsService>,
}

impl AwsOutpostsOutpostConverter {
    pub fn new(service: Arc<OutpostsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOutpostsOutpostConverter {
    fn resource_type(&self) -> &str {
        "aws_outposts_outpost"
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

impl AwsOutpostsOutpostConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: outposts_gen::OutpostTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_outposts_outpost", e))?;

        let outpost_id = model
            .outpost_id
            .clone()
            .or_else(|| model.id.clone())
            .unwrap_or_else(|| format!("op-{:017x}", rand_id()));
        let outpost_arn = model.arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:outposts:{}:{}:outpost/{}",
                region, ctx.default_account_id, outpost_id
            )
        });
        let site_id = model.site_id.clone().unwrap_or_default();
        let site_arn = model.site_arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:outposts:{}:{}:site/{}",
                region, ctx.default_account_id, site_id
            )
        });

        let outpost_view = OutpostView {
            outpost_id: outpost_id.clone(),
            outpost_arn,
            owner_id: ctx.default_account_id.clone(),
            name: model.name,
            description: model.description,
            site_id,
            site_arn,
            availability_zone: model.availability_zone,
            availability_zone_id: model.availability_zone_id,
            life_cycle_status: "ACTIVE".to_string(),
            supported_hardware_type: model.supported_hardware_type,
            tags: model.tags,
        };

        let mut state_view = OutpostsStateView::default();
        state_view.outposts.insert(outpost_id, outpost_view);
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
        for outpost in view.outposts.values() {
            let attrs = serde_json::json!({
                "id": outpost.outpost_id,
                "arn": outpost.outpost_arn,
                "outpost_id": outpost.outpost_id,
                "owner_id": outpost.owner_id,
                "name": outpost.name,
                "description": outpost.description,
                "site_id": outpost.site_id,
                "site_arn": outpost.site_arn,
                "availability_zone": outpost.availability_zone,
                "availability_zone_id": outpost.availability_zone_id,
                "lifecycle_status": outpost.life_cycle_status,
                "supported_hardware_type": outpost.supported_hardware_type,
                "tags": outpost.tags,
            });
            results.push(ExtractedResource {
                name: outpost.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

fn rand_id() -> u128 {
    uuid::Uuid::new_v4().as_u128()
}
