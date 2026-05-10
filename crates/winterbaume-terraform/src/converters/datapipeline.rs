//! Terraform converter for Data Pipeline resources.
//!
//! `PipelineTfModel` is generated from `specs/datapipeline.toml`. The
//! identifier synthesis (`df-<uuid>` when missing), the unique-id default,
//! the structured `tags` projection (Vec<PipelineTagView>), and the
//! `created_at` / `status` constants are wired up here.

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
use crate::generated::datapipeline as datapipeline_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: datapipeline_gen::PipelineTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_datapipeline_pipeline", e))?;

        let description = model.description.unwrap_or_default();
        let pipeline_id = model
            .id
            .unwrap_or_else(|| format!("df-{}", uuid::Uuid::new_v4().to_string().replace('-', "")));
        let unique_id = model
            .unique_id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        // `tags` on the StateView is a Vec<PipelineTagView>, which the codegen
        // does not support. Read the tag map straight off the TF attributes.
        let tags = if let Some(obj) = instance.attributes.get("tags").and_then(|v| v.as_object()) {
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
            name: model.name.clone(),
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
