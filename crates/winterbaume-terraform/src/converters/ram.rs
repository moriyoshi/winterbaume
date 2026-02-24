//! Terraform converter for RAM resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_ram::RamService;
use winterbaume_ram::views::{RamStateView, ResourceShareView, TagView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{
    extract_account_id, extract_region, extract_tags, optional_bool, optional_str, require_str,
};

// ---------------------------------------------------------------------------
// aws_ram_resource_share
// ---------------------------------------------------------------------------

pub struct AwsRamResourceShareConverter {
    service: Arc<RamService>,
}

impl AwsRamResourceShareConverter {
    pub fn new(service: Arc<RamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRamResourceShareConverter {
    fn resource_type(&self) -> &str {
        "aws_ram_resource_share"
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

impl AwsRamResourceShareConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let account_id = extract_account_id(attrs, &ctx.default_account_id);

        let name = require_str(attrs, "name", "aws_ram_resource_share")?;
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:ram:{}:{}:resource-share/{}",
                region, account_id, name
            )
        });
        let allow_external_principals =
            optional_bool(attrs, "allow_external_principals").unwrap_or(false);

        let tags_map = extract_tags(attrs);
        let tags: Vec<TagView> = tags_map
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let rs_view = ResourceShareView {
            resource_share_arn: arn.clone(),
            name: name.to_string(),
            owning_account_id: account_id.clone(),
            allow_external_principals,
            status: "ACTIVE".to_string(),
            creation_time: 0.0,
            last_updated_time: 0.0,
            tags,
        };

        let mut state_view = RamStateView::default();
        state_view.resource_shares.insert(arn, rs_view);
        self.service.merge(&account_id, &region, state_view).await?;

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
        for rs in view.resource_shares.values() {
            let tags_json: serde_json::Map<String, serde_json::Value> = rs
                .tags
                .iter()
                .map(|t| (t.key.clone(), serde_json::Value::String(t.value.clone())))
                .collect();
            let attrs = serde_json::json!({
                "id": rs.resource_share_arn,
                "arn": rs.resource_share_arn,
                "name": rs.name,
                "owning_account_id": rs.owning_account_id,
                "allow_external_principals": rs.allow_external_principals,
                "status": rs.status,
                "creation_time": rs.creation_time,
                "last_updated_time": rs.last_updated_time,
                "tags": tags_json,
            });
            results.push(ExtractedResource {
                name: rs.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
