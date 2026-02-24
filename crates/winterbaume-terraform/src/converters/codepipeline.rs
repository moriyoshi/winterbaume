//! Terraform converters for CodePipeline resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_codepipeline::CodePipelineService;
use winterbaume_codepipeline::views::{CodePipelineStateView, PipelineView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_codepipeline
// ---------------------------------------------------------------------------

pub struct AwsCodepipelinePipelineConverter {
    service: Arc<CodePipelineService>,
}

impl AwsCodepipelinePipelineConverter {
    pub fn new(service: Arc<CodePipelineService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCodepipelinePipelineConverter {
    fn resource_type(&self) -> &str {
        "aws_codepipeline"
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

impl AwsCodepipelinePipelineConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_codepipeline")?;
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:codepipeline:{}:{}:{}",
                region, ctx.default_account_id, name
            )
        });
        let role_arn = optional_str(attrs, "role_arn").unwrap_or_default();
        let stages = attrs
            .get("stage")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));

        let mut tags: HashMap<String, String> = HashMap::new();
        if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.insert(k.clone(), s.to_string());
                }
            }
        }
        let _tags_all = attrs.get("tags_all");
        let execution_mode = optional_str(attrs, "execution_mode");
        let pipeline_type = optional_str(attrs, "pipeline_type");
        let artifact_store = attrs.get("artifact_store").cloned();
        let trigger = attrs.get("trigger").cloned();
        let variable = attrs.get("variable").cloned();

        let now = Utc::now().to_rfc3339();
        let pipeline_view = PipelineView {
            name: name.to_string(),
            arn,
            role_arn,
            stages,
            version: 1,
            created: now.clone(),
            updated: now,
            tags,
            execution_mode,
            pipeline_type,
            artifact_store,
            trigger,
            variable,
            ..Default::default()
        };

        let mut state_view = CodePipelineStateView {
            pipelines: HashMap::new(),
            ..Default::default()
        };
        state_view.pipelines.insert(name.to_string(), pipeline_view);
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
        for pipeline in view.pipelines.values() {
            let attrs = serde_json::json!({
                "id": pipeline.name,
                "name": pipeline.name,
                "arn": pipeline.arn,
                "role_arn": pipeline.role_arn,
                "tags": pipeline.tags,
                "tags_all": pipeline.tags,
                "execution_mode": pipeline.execution_mode.as_deref().unwrap_or("QUEUED"),
                "pipeline_type": pipeline.pipeline_type.as_deref().unwrap_or("V2"),
                "artifact_store": pipeline.artifact_store,
                "stage": pipeline.stages,
                "trigger": pipeline.trigger,
                "variable": pipeline.variable,
            });
            results.push(ExtractedResource {
                name: pipeline.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
