//! Terraform converter for Glacier resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_glacier::GlacierService;
use winterbaume_glacier::views::{GlacierStateView, VaultView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::glacier as glacier_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_glacier_vault
// ---------------------------------------------------------------------------

/// Converts `aws_glacier_vault` Terraform resources to/from Glacier state.
pub struct AwsGlacierVaultConverter {
    service: Arc<GlacierService>,
}

impl AwsGlacierVaultConverter {
    pub fn new(service: Arc<GlacierService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlacierVaultConverter {
    fn resource_type(&self) -> &str {
        "aws_glacier_vault"
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

impl AwsGlacierVaultConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glacier_gen::VaultTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_glacier_vault", e))?;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:glacier:{}:{}:vaults/{}",
                region, ctx.default_account_id, name
            )
        });

        // Parse notification block straight from raw attributes — TF schema
        // exposes it as an array-of-objects which doesn't fit the spec's
        // limited type vocabulary.
        let notification_config = instance.attributes.get("notification").map(|v| {
            let obj = v.as_array().and_then(|a| a.first()).unwrap_or(v);
            let sns_topic = obj
                .get("sns_topic")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let events = obj
                .get("events")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();
            winterbaume_glacier::views::VaultNotificationConfigView { sns_topic, events }
        });

        let vault_view = VaultView {
            vault_name: name.clone(),
            arn,
            created_at: chrono::Utc::now().to_rfc3339(),
            archives: HashMap::new(),
            jobs: HashMap::new(),
            tags: model.tags,
            access_policy: model.access_policy,
            notification_config,
            vault_lock: None,
            multipart_uploads: HashMap::new(),
        };

        let mut state_view = GlacierStateView {
            vaults: HashMap::new(),
            data_retrieval_policy: None,
            provisioned_capacity: vec![],
        };
        state_view.vaults.insert(name, vault_view);
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
        for vault in view.vaults.values() {
            let mut attrs = serde_json::json!({
                "id": vault.vault_name,
                "name": vault.vault_name,
                "arn": vault.arn,
                "tags": vault.tags,
                "tags_all": vault.tags,
            });
            if let Some(policy) = &vault.access_policy {
                attrs["access_policy"] = serde_json::Value::String(policy.clone());
            }
            if let Some(nc) = &vault.notification_config {
                attrs["notification"] = serde_json::json!([{
                    "sns_topic": nc.sns_topic,
                    "events": nc.events,
                }]);
            }
            results.push(ExtractedResource {
                name: vault.vault_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
