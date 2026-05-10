//! Terraform converter for Shield resources.
//!
//! `ProtectionTfModel` is generated from `specs/shield.toml`. The
//! protection ID synthesis (UUID when missing), the ARN template, and
//! the conversion from the flat `tags` map to `Vec<TagView>` are wired
//! up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_shield::ShieldService;
use winterbaume_shield::views::{ProtectionView, ShieldStateView, TagView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::shield as shield_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_shield_protection
// ---------------------------------------------------------------------------

/// Converts `aws_shield_protection` Terraform resources to/from Shield state.
pub struct AwsShieldProtectionConverter {
    service: Arc<ShieldService>,
}

impl AwsShieldProtectionConverter {
    pub fn new(service: Arc<ShieldService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsShieldProtectionConverter {
    fn resource_type(&self) -> &str {
        "aws_shield_protection"
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

impl AwsShieldProtectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: shield_gen::ProtectionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_shield_protection", e))?;

        let protection_id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let protection_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:shield::{}:protection/{}",
                ctx.default_account_id, protection_id
            )
        });

        // Convert tags from HashMap<String, String> to Vec<TagView>.
        let tags: Vec<TagView> = model
            .tags
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let protection_view = ProtectionView {
            id: protection_id.clone(),
            name: model.name,
            resource_arn: model.resource_arn,
            protection_arn,
            health_check_ids: vec![],
            tags,
        };

        let state_view = ShieldStateView {
            subscription: None,
            protections: {
                let mut m = HashMap::new();
                m.insert(protection_id, protection_view);
                m
            },
            tags: HashMap::new(),
        };
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
        for prot in view.protections.values() {
            let tags_map: HashMap<String, String> = prot
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": prot.id,
                "arn": prot.protection_arn,
                "name": prot.name,
                "resource_arn": prot.resource_arn,
                "protection_arn": prot.protection_arn,
                "health_check_ids": prot.health_check_ids,
                "tags": tags_map,
            });
            results.push(ExtractedResource {
                name: prot.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
