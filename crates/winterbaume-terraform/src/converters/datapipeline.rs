//! Terraform converter for Data Pipeline resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_datapipeline::DataPipelineService;
use winterbaume_datapipeline::views::{DataPipelineStateView, PipelineTagView, PipelineView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_datapipeline_pipeline
// ---------------------------------------------------------------------------

/// Converts `aws_datapipeline_pipeline` Terraform resources to/from Data Pipeline state.
pub struct AwsDatapipelinePipelineConverter {
    service: Arc<DataPipelineService>,
}

impl AwsDatapipelinePipelineConverter {
    pub fn new(service: Arc<DataPipelineService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDatapipelinePipelineConverter {
    fn resource_type(&self) -> &str {
        "aws_datapipeline_pipeline"
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

impl AwsDatapipelinePipelineConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_datapipeline_pipeline")?;
        let description = optional_str(attrs, "description").unwrap_or_default();
        let region = extract_region(attrs, &ctx.default_region);

        let pipeline_id = optional_str(attrs, "id")
            .unwrap_or_else(|| format!("df-{}", uuid::Uuid::new_v4().to_string().replace('-', "")));
        let unique_id =
            optional_str(attrs, "unique_id").unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let tags = if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
            obj.iter()
                .filter_map(|(k, v)| {
                    v.as_str().map(|s| PipelineTagView {
                        key: k.clone(),
                        value: s.to_string(),
                    })
                })
                .collect()
        } else {
            vec![]
        };

        let pipeline_view = PipelineView {
            pipeline_id: pipeline_id.clone(),
            name: name.to_string(),
            description,
            unique_id,
            tags,
            created_at: chrono::Utc::now(),
            status: "PENDING".to_string(),
            pipeline_objects: vec![],
            parameter_objects: vec![],
            parameter_values: vec![],
        };

        let mut state_view = DataPipelineStateView {
            pipelines: HashMap::new(),
        };
        state_view.pipelines.insert(pipeline_id, pipeline_view);
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
            let tags: serde_json::Value = pipeline
                .tags
                .iter()
                .map(|t| (t.key.clone(), serde_json::Value::String(t.value.clone())))
                .collect::<serde_json::Map<String, serde_json::Value>>()
                .into();
            let attrs = serde_json::json!({
                "id": pipeline.pipeline_id,
                "name": pipeline.name,
                "description": pipeline.description,
                "tags": tags,
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
