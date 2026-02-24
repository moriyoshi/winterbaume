//! Terraform converter for IVS resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_ivs::IvsService;
use winterbaume_ivs::views::{ChannelView, IvsStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_bool, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_ivs_channel
// ---------------------------------------------------------------------------

/// Converts `aws_ivs_channel` Terraform resources to/from IVS state.
pub struct AwsIvsChannelConverter {
    service: Arc<IvsService>,
}

impl AwsIvsChannelConverter {
    pub fn new(service: Arc<IvsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIvsChannelConverter {
    fn resource_type(&self) -> &str {
        "aws_ivs_channel"
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

impl AwsIvsChannelConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_ivs_channel")?;
        let region = extract_region(attrs, &ctx.default_region);
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:ivs:{}:{}:channel/{}",
                region, ctx.default_account_id, name
            )
        });
        let latency_mode = optional_str(attrs, "latency_mode").unwrap_or_else(|| "LOW".to_string());
        let channel_type = optional_str(attrs, "type").unwrap_or_else(|| "STANDARD".to_string());
        let authorized = optional_bool(attrs, "authorized").unwrap_or(false);
        let tags = extract_tags(attrs);

        let channel_view = ChannelView {
            arn: arn.clone(),
            name: name.to_string(),
            latency_mode,
            channel_type,
            authorized,
            tags,
        };

        let mut state_view = IvsStateView {
            channels: HashMap::new(),
            stream_keys: HashMap::new(),
            recording_configurations: HashMap::new(),
            playback_key_pairs: HashMap::new(),
            playback_restriction_policies: HashMap::new(),
        };
        state_view.channels.insert(arn, channel_view);
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
        for channel in view.channels.values() {
            let attrs = serde_json::json!({
                "id": channel.arn,
                "arn": channel.arn,
                "name": channel.name,
                "latency_mode": channel.latency_mode,
                "type": channel.channel_type,
                "authorized": channel.authorized,
                "tags": channel.tags,
            });
            results.push(ExtractedResource {
                name: channel.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
