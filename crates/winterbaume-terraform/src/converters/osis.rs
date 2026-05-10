//! Terraform converter for OSIS (OpenSearch Ingestion) resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_osis::OsisService;
use winterbaume_osis::views::{OsisStateView, PipelineView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::osis as osis_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_osis_pipeline
// ---------------------------------------------------------------------------

/// Converts `aws_osis_pipeline` Terraform resources to/from OSIS state.
pub struct AwsOsisPipelineConverter {
    service: Arc<OsisService>,
}

impl AwsOsisPipelineConverter {
    pub fn new(service: Arc<OsisService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOsisPipelineConverter {
    fn resource_type(&self) -> &str {
        "aws_osis_pipeline"
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

impl AwsOsisPipelineConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: osis_gen::PipelineTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_osis_pipeline", e))?;

        let pipeline_name = model.pipeline_name.clone();
        let pipeline_arn = model.pipeline_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:osis:{}:{}:pipeline/{}",
                region, ctx.default_account_id, pipeline_name
            )
        });
        let pipeline_configuration_body = model.pipeline_configuration_body.unwrap_or_default();
        let min_units = model.min_units as i32;
        let max_units = model.max_units as i32;

        // JSON-blob and array-of-strings fields stay as raw attributes.
        let attrs = &instance.attributes;
        let ingest_endpoint_urls = attrs
            .get("ingest_endpoint_urls")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let buffer_options = attrs.get("buffer_options").cloned();
        let encryption_at_rest_options = attrs.get("encryption_at_rest_options").cloned();
        let log_publishing_options = attrs.get("log_publishing_options").cloned();
        let vpc_options = attrs.get("vpc_options").cloned();

        let now = chrono::Utc::now().to_rfc3339();
        let pipeline_view = PipelineView {
            pipeline_name: pipeline_name.clone(),
            pipeline_arn,
            min_units,
            max_units,
            status: "ACTIVE".to_string(),
            pipeline_configuration_body,
            created_at: now.clone(),
            last_updated_at: now,
            ingest_endpoint_urls,
            tags: model.tags,
            buffer_options,
            encryption_at_rest_options,
            log_publishing_options,
            vpc_options,
        };

        let mut state_view = OsisStateView {
            pipelines: HashMap::new(),
        };
        state_view.pipelines.insert(pipeline_name, pipeline_view);
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
                "id": pipeline.pipeline_name,
                "pipeline_name": pipeline.pipeline_name,
                "pipeline_arn": pipeline.pipeline_arn,
                "min_units": pipeline.min_units,
                "max_units": pipeline.max_units,
                "pipeline_configuration_body": pipeline.pipeline_configuration_body,
                "ingest_endpoint_urls": pipeline.ingest_endpoint_urls,
                "tags": pipeline.tags,
                "buffer_options": pipeline.buffer_options,
                "encryption_at_rest_options": pipeline.encryption_at_rest_options,
                "log_publishing_options": pipeline.log_publishing_options,
                "vpc_options": pipeline.vpc_options,
            });
            results.push(ExtractedResource {
                name: pipeline.pipeline_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
