//! Terraform converter for Pipes resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_pipes::PipesService;
use winterbaume_pipes::views::{PipeView, PipesStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_pipes_pipe
// ---------------------------------------------------------------------------

/// Converts `aws_pipes_pipe` Terraform resources to/from Pipes state.
pub struct AwsPipesPipeConverter {
    service: Arc<PipesService>,
}

impl AwsPipesPipeConverter {
    pub fn new(service: Arc<PipesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsPipesPipeConverter {
    fn resource_type(&self) -> &str {
        "aws_pipes_pipe"
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

impl AwsPipesPipeConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_pipes_pipe")?;
        let source = require_str(attrs, "source", "aws_pipes_pipe")?;
        let target = require_str(attrs, "target", "aws_pipes_pipe")?;
        let description = optional_str(attrs, "description");
        let enrichment = optional_str(attrs, "enrichment");
        let role_arn = optional_str(attrs, "role_arn");
        let desired_state =
            optional_str(attrs, "desired_state").unwrap_or_else(|| "RUNNING".to_string());
        let current_state =
            optional_str(attrs, "current_state").unwrap_or_else(|| "RUNNING".to_string());
        let creation_time = optional_str(attrs, "creation_time").unwrap_or_default();
        let last_modified_time = optional_str(attrs, "last_modified_time").unwrap_or_default();
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:pipes:{}:{}:pipe/{}",
                region, ctx.default_account_id, name
            )
        });
        let mut tags = extract_tags(attrs);
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let enrichment_parameters = attrs.get("enrichment_parameters").cloned();
        let log_configuration = attrs.get("log_configuration").cloned();
        let source_parameters = attrs.get("source_parameters").cloned();
        let target_parameters = attrs.get("target_parameters").cloned();

        let pipe_view = PipeView {
            name: name.to_string(),
            arn,
            source: source.to_string(),
            target: target.to_string(),
            description,
            enrichment,
            role_arn,
            desired_state,
            current_state,
            creation_time,
            last_modified_time,
            tags,
            enrichment_parameters,
            log_configuration,
            source_parameters,
            target_parameters,
        };

        let mut state_view = PipesStateView {
            pipes: HashMap::new(),
        };
        state_view.pipes.insert(name.to_string(), pipe_view);
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
        for pipe in view.pipes.values() {
            let attrs = serde_json::json!({
                "id": pipe.name,
                "name": pipe.name,
                "arn": pipe.arn,
                "source": pipe.source,
                "target": pipe.target,
                "description": pipe.description,
                "enrichment": pipe.enrichment,
                "role_arn": pipe.role_arn,
                "desired_state": pipe.desired_state,
                "current_state": pipe.current_state,
                "creation_time": pipe.creation_time,
                "last_modified_time": pipe.last_modified_time,
                "tags": pipe.tags,
                "tags_all": pipe.tags,
                "enrichment_parameters": pipe.enrichment_parameters,
                "log_configuration": pipe.log_configuration,
                "source_parameters": pipe.source_parameters,
                "target_parameters": pipe.target_parameters,
            });
            results.push(ExtractedResource {
                name: pipe.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
