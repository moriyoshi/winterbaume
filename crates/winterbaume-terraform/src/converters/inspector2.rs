//! Terraform converter for Inspector2 resources.
//!
//! `EnablerTfModel` is generated from `specs/inspector2.toml` as a marker;
//! the resource's only inputs are `account_ids` (Vec<String>) and
//! `resource_types` (Vec<String>), which the spec format does not express.
//! Both are read directly from `instance.attributes`.

use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_inspector2::Inspector2Service;
use winterbaume_inspector2::views::Inspector2StateView;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::extract_region;

pub struct AwsInspector2EnablerConverter {
    service: Arc<Inspector2Service>,
}

impl AwsInspector2EnablerConverter {
    pub fn new(service: Arc<Inspector2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsInspector2EnablerConverter {
    fn resource_type(&self) -> &str {
        "aws_inspector2_enabler"
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

impl AwsInspector2EnablerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let _account_ids = attrs.get("account_ids");

        let resource_types: HashSet<String> = attrs
            .get("resource_types")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let mut enabled_resource_types: HashMap<String, HashSet<String>> = HashMap::new();
        enabled_resource_types.insert(ctx.default_account_id.clone(), resource_types);
        let state_view = Inspector2StateView {
            enabled_resource_types,
            ..Default::default()
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
        if view.enabled_resource_types.is_empty() {
            return Ok(vec![]);
        }
        let resource_types: Vec<String> = view
            .enabled_resource_types
            .values()
            .flat_map(|set| set.iter().cloned())
            .collect();
        let id = format!("{}:{}", ctx.default_account_id, ctx.default_region);
        let attrs = serde_json::json!({
            "id": id,
            "account_ids": [ctx.default_account_id],
            "resource_types": resource_types,
        });
        Ok(vec![ExtractedResource {
            name: id,
            account_id: ctx.default_account_id.clone(),
            region: ctx.default_region.clone(),
            attributes: attrs,
        }])
    }
}
