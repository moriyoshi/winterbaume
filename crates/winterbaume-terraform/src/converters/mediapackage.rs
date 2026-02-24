//! Terraform converter for MediaPackage resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_mediapackage::MediaPackageService;
use winterbaume_mediapackage::views::{ChannelView, MediaPackageStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_media_package_channel
// ---------------------------------------------------------------------------

/// Converts `aws_media_package_channel` Terraform resources to/from MediaPackage state.
pub struct AwsMediaPackageChannelConverter {
    service: Arc<MediaPackageService>,
}

impl AwsMediaPackageChannelConverter {
    pub fn new(service: Arc<MediaPackageService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMediaPackageChannelConverter {
    fn resource_type(&self) -> &str {
        "aws_media_package_channel"
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

impl AwsMediaPackageChannelConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let channel_id = require_str(attrs, "channel_id", "aws_media_package_channel")?;
        let region = extract_region(attrs, &ctx.default_region);
        let description = optional_str(attrs, "description").unwrap_or_default();

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:mediapackage:{}:{}:channels/{}",
                region, ctx.default_account_id, channel_id
            )
        });

        let channel_view = ChannelView {
            arn,
            id: channel_id.to_string(),
            description,
            tags: extract_tags(attrs),
            created_at: optional_str(attrs, "created_at")
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
        };

        let mut state_view = MediaPackageStateView {
            channels: HashMap::new(),
            origin_endpoints: HashMap::new(),
        };
        state_view
            .channels
            .insert(channel_id.to_string(), channel_view);
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
        for ch in view.channels.values() {
            let attrs = serde_json::json!({
                "id": ch.id,
                "channel_id": ch.id,
                "arn": ch.arn,
                "description": ch.description,
                "tags": ch.tags,
                "created_at": ch.created_at,
            });
            results.push(ExtractedResource {
                name: ch.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
