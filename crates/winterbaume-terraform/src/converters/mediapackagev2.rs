//! Terraform converter for MediaPackage V2 resources.
//!
//! `ChannelGroupTfModel` is generated from `specs/mediapackagev2.toml`.
//! The ARN template and the `created_at` / `modified_at` fallbacks
//! (current time) are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_mediapackagev2::MediaPackageV2Service;
use winterbaume_mediapackagev2::views::{ChannelGroupView, MediaPackageV2StateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::mediapackagev2 as mediapackagev2_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_media_packagev2_channel_group
// ---------------------------------------------------------------------------

/// Converts `aws_media_packagev2_channel_group` Terraform resources to/from MediaPackageV2 state.
pub struct AwsMediaPackageV2ChannelGroupConverter {
    service: Arc<MediaPackageV2Service>,
}

impl AwsMediaPackageV2ChannelGroupConverter {
    pub fn new(service: Arc<MediaPackageV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMediaPackageV2ChannelGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_media_packagev2_channel_group"
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

impl AwsMediaPackageV2ChannelGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: mediapackagev2_gen::ChannelGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_media_packagev2_channel_group", e))?;

        let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:mediapackagev2:{}:{}:channelGroup/{}",
                region, ctx.default_account_id, model.name
            )
        });

        let cg_view = ChannelGroupView {
            channel_group_name: model.name.clone(),
            arn,
            egress_domain: model.egress_domain.unwrap_or_default(),
            description: model.description.unwrap_or_default(),
            tags: model.tags,
            created_at: model.created_at.unwrap_or_else(|| now.clone()),
            modified_at: model.modified_at.unwrap_or(now),
            e_tag: model.e_tag.unwrap_or_default(),
            channels: HashMap::new(),
        };

        let mut state_view = MediaPackageV2StateView {
            channel_groups: HashMap::new(),
        };
        state_view.channel_groups.insert(model.name, cg_view);
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
        for cg in view.channel_groups.values() {
            let attrs = serde_json::json!({
                "id": cg.channel_group_name,
                "name": cg.channel_group_name,
                "arn": cg.arn,
                "egress_domain": cg.egress_domain,
                "description": cg.description,
                "tags": cg.tags,
                "created_at": cg.created_at,
                "modified_at": cg.modified_at,
                "e_tag": cg.e_tag,
            });
            results.push(ExtractedResource {
                name: cg.channel_group_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
