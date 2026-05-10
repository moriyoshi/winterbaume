//! Terraform converter for Pipes resources.
//!
//! `PipeTfModel` is generated from `specs/pipes.toml`. The ARN template,
//! the `desired_state` / `current_state` defaults, the `tags_all` merge
//! into `tags`, and the four pass-through JSON parameter blocks
//! (`enrichment_parameters`, `log_configuration`, `source_parameters`,
//! `target_parameters`) are wired up here.

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
use crate::generated::pipes as pipes_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: pipes_gen::PipeTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_pipes_pipe", e))?;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:pipes:{}:{}:pipe/{}",
                region, ctx.default_account_id, name
            )
        });
        let mut tags = model.tags;
        if let Some(obj) = instance
            .attributes
            .get("tags_all")
            .and_then(|v| v.as_object())
        {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let enrichment_parameters = instance.attributes.get("enrichment_parameters").cloned();
        let log_configuration = instance.attributes.get("log_configuration").cloned();
        let source_parameters = instance.attributes.get("source_parameters").cloned();
        let target_parameters = instance.attributes.get("target_parameters").cloned();

        let pipe_view = PipeView {
            name: name.clone(),
            arn,
            source: model.source,
            target: model.target,
            description: model.description,
            enrichment: model.enrichment,
            role_arn: model.role_arn,
            desired_state: model.desired_state.unwrap_or_else(|| "RUNNING".to_string()),
            current_state: model.current_state.unwrap_or_else(|| "RUNNING".to_string()),
            creation_time: model.creation_time.unwrap_or_default(),
            last_modified_time: model.last_modified_time.unwrap_or_default(),
            tags,
            enrichment_parameters,
            log_configuration,
            source_parameters,
            target_parameters,
        };

        let mut state_view = PipesStateView {
            pipes: HashMap::new(),
        };
        state_view.pipes.insert(name, pipe_view);
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
