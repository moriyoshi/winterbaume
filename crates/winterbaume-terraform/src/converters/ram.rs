//! Terraform converter for RAM resources.
//!
//! `ResourceShareTfModel` is generated from `specs/ram.toml`. The ARN
//! template, the `status = "ACTIVE"` constant, the zero-valued
//! `creation_time`/`last_updated_time` placeholders, and the conversion
//! between the model's `HashMap<String, String>` tag bag and the view's
//! `Vec<TagView>` are wired up here.

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
use crate::generated::ram as ram_gen;
use crate::util::{classify_deserialize_error, extract_account_id, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let account_id = extract_account_id(&instance.attributes, &ctx.default_account_id);
        let model: ram_gen::ResourceShareTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ram_resource_share", e))?;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ram:{}:{}:resource-share/{}",
                region, account_id, model.name
            )
        });

        let tags: Vec<TagView> = model
            .tags
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let rs_view = ResourceShareView {
            resource_share_arn: arn.clone(),
            name: model.name,
            owning_account_id: account_id.clone(),
            allow_external_principals: model.allow_external_principals,
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
