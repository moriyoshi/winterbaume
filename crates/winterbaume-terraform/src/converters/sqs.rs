//! Terraform converter for `aws_sqs_queue` resources.
//!
//! Field projection is generated from `specs/sqs.toml` into
//! `crate::generated::sqs`. This module owns only the converter struct,
//! the trait scaffolding, and the inject/extract orchestration that ties
//! the generated projection to the SQS service backend.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_sqs::SqsService;
use winterbaume_sqs::views::SqsStateView;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::sqs as sqs_gen;
use crate::util::{classify_deserialize_error, extract_region};

/// Converts `aws_sqs_queue` Terraform resources to/from SQS service state.
pub struct AwsSqsQueueConverter {
    service: Arc<SqsService>,
}

impl AwsSqsQueueConverter {
    /// Create a new converter backed by the given SQS service.
    pub fn new(service: Arc<SqsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSqsQueueConverter {
    fn resource_type(&self) -> &str {
        "aws_sqs_queue"
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

impl AwsSqsQueueConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sqs_gen::SqsQueueTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_sqs_queue", e))?;

        let view = model.into_state_view(ctx, &region);
        let mut state_view = SqsStateView::default();
        state_view.queues.insert(view.name.clone(), view);
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
        for queue in view.queues.values() {
            let model = sqs_gen::SqsQueueTfModel::from(queue);
            let attributes = serde_json::to_value(&model)
                .expect("infallible: SqsQueueTfModel serializes to JSON");
            results.push(ExtractedResource {
                name: queue.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes,
            });
        }
        Ok(results)
    }
}
