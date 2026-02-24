//! Terraform converter for `aws_sqs_queue` resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_sqs::SqsService;
use winterbaume_sqs::views::{QueueStateView, SqsStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{
    extract_region, extract_tags, optional_bool, optional_i64, optional_str, require_str,
};

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
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_sqs_queue")?;
        let region = extract_region(attrs, &ctx.default_region);

        let url = optional_str(attrs, "url")
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| {
                format!(
                    "https://sqs.{}.amazonaws.com/{}/{}",
                    region, ctx.default_account_id, name
                )
            });
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!("arn:aws:sqs:{}:{}:{}", region, ctx.default_account_id, name)
        });

        let fifo = optional_bool(attrs, "fifo_queue").unwrap_or(false);

        let queue_view = QueueStateView {
            name: name.to_string(),
            url,
            arn,
            region: region.clone(),
            account_id: ctx.default_account_id.clone(),
            created_timestamp: None,
            last_modified_timestamp: None,
            visibility_timeout: optional_i64(attrs, "visibility_timeout_seconds").unwrap_or(30)
                as u32,
            delay_seconds: optional_i64(attrs, "delay_seconds").unwrap_or(0) as u32,
            maximum_message_size: optional_i64(attrs, "max_message_size").unwrap_or(262144) as u32,
            message_retention_period: optional_i64(attrs, "message_retention_seconds")
                .unwrap_or(345600) as u32,
            receive_wait_time_seconds: optional_i64(attrs, "receive_wait_time_seconds").unwrap_or(0)
                as u32,
            fifo_queue: fifo,
            content_based_deduplication: optional_bool(attrs, "content_based_deduplication")
                .unwrap_or(false),
            tags: extract_tags(attrs),
            redrive_policy: optional_str(attrs, "redrive_policy"),
            policy: optional_str(attrs, "policy"),
        };

        let mut state_view = SqsStateView::default();
        state_view.queues.insert(name.to_string(), queue_view);
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
            let attrs = serde_json::json!({
                "id": queue.url,
                "name": queue.name,
                "arn": queue.arn,
                "url": queue.url,
                "visibility_timeout_seconds": queue.visibility_timeout,
                "delay_seconds": queue.delay_seconds,
                "max_message_size": queue.maximum_message_size,
                "message_retention_seconds": queue.message_retention_period,
                "receive_wait_time_seconds": queue.receive_wait_time_seconds,
                "fifo_queue": queue.fifo_queue,
                "content_based_deduplication": queue.content_based_deduplication,
                "tags": queue.tags,
                "tags_all": queue.tags,
            });
            results.push(ExtractedResource {
                name: queue.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
