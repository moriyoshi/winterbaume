//! Terraform converter for `aws_sqs_queue` resources.
//!
//! `SqsQueueTfModel` is generated from `specs/sqs.toml`. The ARN/URL
//! templates and the constants for the timestamps live in this module.

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

        let arn = model.arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:sqs:{}:{}:{}",
                region, ctx.default_account_id, model.name
            )
        });
        let url = model
            .url
            .clone()
            .or_else(|| model.id.clone())
            .unwrap_or_else(|| {
                format!(
                    "https://sqs.{}.amazonaws.com/{}/{}",
                    region, ctx.default_account_id, model.name
                )
            });

        let queue_view = QueueStateView {
            name: model.name.clone(),
            url,
            arn,
            region: region.clone(),
            account_id: ctx.default_account_id.clone(),
            created_timestamp: None,
            last_modified_timestamp: None,
            visibility_timeout: model.visibility_timeout,
            delay_seconds: model.delay_seconds,
            maximum_message_size: model.maximum_message_size,
            message_retention_period: model.message_retention_period,
            receive_wait_time_seconds: model.receive_wait_time_seconds,
            fifo_queue: model.fifo_queue,
            content_based_deduplication: model.content_based_deduplication,
            tags: model.tags,
            redrive_policy: model.redrive_policy,
            policy: model.policy,
        };

        let mut state_view = SqsStateView::default();
        state_view.queues.insert(model.name, queue_view);
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

// ---------------------------------------------------------------------------
// aws_sqs_queue_policy — modifier-style: writes `policy` onto the queue
// ---------------------------------------------------------------------------

/// Extract the queue name from a queue URL like
/// `https://sqs.<region>.amazonaws.com/<account>/<name>`.
fn queue_name_from_url(url: &str) -> Option<&str> {
    url.rsplit('/').next().filter(|s| !s.is_empty())
}

/// Converts `aws_sqs_queue_policy` Terraform resources to/from SQS state.
pub struct AwsSqsQueuePolicyConverter {
    service: Arc<SqsService>,
}

impl AwsSqsQueuePolicyConverter {
    pub fn new(service: Arc<SqsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSqsQueuePolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_sqs_queue_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_sqs_queue"]
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Extracted alongside `aws_sqs_queue`.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsSqsQueuePolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sqs_gen::SqsQueuePolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_sqs_queue_policy", e))?;

        let queue_name = queue_name_from_url(&model.queue_url)
            .map(str::to_owned)
            .unwrap_or_else(|| model.queue_url.clone());

        let view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        let Some(queue) = view.queues.get(&queue_name).cloned() else {
            let msg = format!(
                "aws_sqs_queue_policy: queue {} not found in state; inject skipped",
                queue_name
            );
            eprintln!("warning: {}", msg);
            warnings.push(msg);
            return Ok(ConversionResult { region, warnings });
        };

        let new_queue = QueueStateView {
            policy: Some(model.policy),
            ..queue
        };

        let mut state_view = SqsStateView::default();
        state_view.queues.insert(queue_name, new_queue);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_sqs_queue_redrive_policy — modifier: writes redrive_policy onto queue
// ---------------------------------------------------------------------------

/// Converts `aws_sqs_queue_redrive_policy` Terraform resources to/from SQS state.
pub struct AwsSqsQueueRedrivePolicyConverter {
    service: Arc<SqsService>,
}

impl AwsSqsQueueRedrivePolicyConverter {
    pub fn new(service: Arc<SqsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSqsQueueRedrivePolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_sqs_queue_redrive_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_sqs_queue"]
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsSqsQueueRedrivePolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sqs_gen::SqsQueueRedrivePolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_sqs_queue_redrive_policy", e))?;

        let queue_name = queue_name_from_url(&model.queue_url)
            .map(str::to_owned)
            .unwrap_or_else(|| model.queue_url.clone());

        let view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        let Some(queue) = view.queues.get(&queue_name).cloned() else {
            let msg = format!(
                "aws_sqs_queue_redrive_policy: queue {} not found in state; inject skipped",
                queue_name
            );
            eprintln!("warning: {}", msg);
            warnings.push(msg);
            return Ok(ConversionResult { region, warnings });
        };

        let new_queue = QueueStateView {
            redrive_policy: Some(model.redrive_policy),
            ..queue
        };

        let mut state_view = SqsStateView::default();
        state_view.queues.insert(queue_name, new_queue);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_sqs_queue_redrive_allow_policy — no state slot (warning-only)
// ---------------------------------------------------------------------------

/// Converts `aws_sqs_queue_redrive_allow_policy` Terraform resources. The
/// `RedriveAllowPolicy` attribute does not have a state slot in
/// `winterbaume_sqs`; this converter validates input and emits a warning.
pub struct AwsSqsQueueRedriveAllowPolicyConverter {
    #[allow(dead_code)]
    service: Arc<SqsService>,
}

impl AwsSqsQueueRedriveAllowPolicyConverter {
    pub fn new(service: Arc<SqsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSqsQueueRedriveAllowPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_sqs_queue_redrive_allow_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_sqs_queue"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let attrs = &instance.attributes;
            let region = extract_region(attrs, &ctx.default_region);
            let _model: sqs_gen::SqsQueueRedriveAllowPolicyTfModel =
                serde_json::from_value(attrs.clone()).map_err(|e| {
                    classify_deserialize_error("aws_sqs_queue_redrive_allow_policy", e)
                })?;
            let msg = "aws_sqs_queue_redrive_allow_policy: no state slot in winterbaume_sqs; inject is a no-op";
            eprintln!("warning: {}", msg);
            Ok(ConversionResult {
                region,
                warnings: vec![msg.to_string()],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}
