//! Terraform converters for AWS AppFabric resources.
//!
//! `AppBundleTfModel` is generated from `specs/appfabric.toml`. The ARN
//! template (with a UUID-derived bundle id when no ARN is supplied) and
//! the id-from-ARN derivation are wired up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use uuid::Uuid;
use winterbaume_appfabric::AppFabricService;
use winterbaume_appfabric::views::{AppBundleView, AppFabricStateView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::appfabric as appfabric_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_appfabric_app_bundle
// ---------------------------------------------------------------------------

pub struct AwsAppFabricAppBundleConverter {
    service: Arc<AppFabricService>,
}

impl AwsAppFabricAppBundleConverter {
    pub fn new(service: Arc<AppFabricService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppFabricAppBundleConverter {
    fn resource_type(&self) -> &str {
        "aws_appfabric_app_bundle"
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

impl AwsAppFabricAppBundleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: appfabric_gen::AppBundleTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_appfabric_app_bundle", e))?;

        let arn = model.arn.unwrap_or_else(|| {
            let id = Uuid::new_v4().simple().to_string();
            format!(
                "arn:aws:appfabric:{}:{}:appbundle/{}",
                region, ctx.default_account_id, id
            )
        });

        let id = arn
            .rsplit_once('/')
            .map(|(_, id)| id.to_string())
            .unwrap_or_else(|| arn.clone());

        let mut state_view = AppFabricStateView::default();
        state_view.app_bundles.insert(
            id,
            AppBundleView {
                arn,
                customer_managed_key_arn: model.customer_managed_key_arn,
                tags: model.tags,
            },
        );
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

        let mut resources = Vec::new();
        for (id, bundle) in &view.app_bundles {
            let mut attrs = serde_json::Map::new();
            attrs.insert("id".to_string(), serde_json::json!(id));
            attrs.insert("arn".to_string(), serde_json::json!(bundle.arn));
            if let Some(cmk) = &bundle.customer_managed_key_arn {
                attrs.insert(
                    "customer_managed_key_arn".to_string(),
                    serde_json::json!(cmk),
                );
            }
            if !bundle.tags.is_empty() {
                attrs.insert("tags".to_string(), serde_json::json!(bundle.tags));
            }
            resources.push(ExtractedResource {
                name: id.clone(),
                attributes: serde_json::Value::Object(attrs),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
            });
        }

        Ok(resources)
    }
}
