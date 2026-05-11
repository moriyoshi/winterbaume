//! Terraform converter for `aws_kinesis_stream` resources.
//!
//! `StreamTfModel` is generated from `specs/kinesis.toml`. The ARN
//! template (with `id` fallback), the nested `stream_mode_details`
//! block, the synthesised provisioned-mode shard list, the
//! `encryption_type` default, and the `created_timestamp` constant are
//! wired up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_kinesis::KinesisService;
use winterbaume_kinesis::views::{KinesisStateView, ShardView, StreamView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::kinesis as kinesis_gen;
use crate::util::{classify_deserialize_error, extract_region};

/// Converts `aws_kinesis_stream` Terraform resources to/from Kinesis state.
pub struct AwsKinesisStreamConverter {
    service: Arc<KinesisService>,
}

impl AwsKinesisStreamConverter {
    pub fn new(service: Arc<KinesisService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKinesisStreamConverter {
    fn resource_type(&self) -> &str {
        "aws_kinesis_stream"
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

impl AwsKinesisStreamConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: kinesis_gen::StreamTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_kinesis_stream", e))?;

        let name = model.name.clone();
        let arn = model.arn.clone().or(model.id.clone()).unwrap_or_else(|| {
            format!(
                "arn:aws:kinesis:{}:{}:stream/{}",
                region, ctx.default_account_id, name
            )
        });

        // Stream mode: ON_DEMAND or PROVISIONED (default PROVISIONED)
        let stream_mode = instance
            .attributes
            .get("stream_mode_details")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("stream_mode"))
            .and_then(|v| v.as_str())
            .unwrap_or("PROVISIONED")
            .to_string();

        let shard_count = model.shard_count as usize;
        let retention_period_hours = model.retention_period as u32;

        let encryption_type = model.encryption_type.unwrap_or_else(|| "NONE".to_string());
        let key_id = model.kms_key_id;

        // Build fake shards for PROVISIONED mode
        let shards = if stream_mode == "PROVISIONED" {
            (0..shard_count)
                .map(|i| {
                    let range = u128::MAX / shard_count as u128;
                    ShardView {
                        shard_id: format!("shardId-{:012}", i),
                        starting_hash_key: (range * i as u128).to_string(),
                        ending_hash_key: if i + 1 == shard_count {
                            u128::MAX.to_string()
                        } else {
                            (range * (i + 1) as u128 - 1).to_string()
                        },
                        parent_shard_id: None,
                        adjacent_parent_shard_id: None,
                        closed: false,
                    }
                })
                .collect()
        } else {
            vec![]
        };

        let stream_view = StreamView {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            stream_mode,
            shards,
            retention_period_hours,
            encryption_type,
            key_id,
            tags: model.tags,
            consumers: vec![],
            created_timestamp: Some(Utc::now().to_rfc3339()),
            account_id: ctx.default_account_id.clone(),
            enhanced_monitoring: vec![],
            resource_policy: None,
            max_record_size_in_ki_b: None,
        };

        let mut state_view = KinesisStateView::default();
        state_view.streams.insert(name, stream_view);
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
        for stream in view.streams.values() {
            let shard_count = stream.shards.iter().filter(|s| !s.closed).count() as i64;
            let attrs = serde_json::json!({
                "id": stream.arn,
                "name": stream.name,
                "arn": stream.arn,
                "shard_count": shard_count,
                "retention_period": stream.retention_period_hours,
                "encryption_type": stream.encryption_type,
                "kms_key_id": stream.key_id,
                "tags": stream.tags,
            });
            results.push(ExtractedResource {
                name: stream.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_kinesis_resource_policy — mutates StreamView.resource_policy on an
// existing stream; the view-layer merge only inserts streams, so there is no
// path to attach a policy to a stream we did not create here. Inject is a
// no-op with a warning.
// ---------------------------------------------------------------------------

pub struct AwsKinesisResourcePolicyConverter {
    #[allow(dead_code)]
    service: Arc<KinesisService>,
}

impl AwsKinesisResourcePolicyConverter {
    pub fn new(service: Arc<KinesisService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKinesisResourcePolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_kinesis_resource_policy"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: kinesis_gen::ResourcePolicyTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_kinesis_resource_policy", e))?;
            let warn_msg = "stream resource policies attach to an existing StreamView; the \
                            view-layer merge has no per-stream policy patch path — inject is \
                            a no-op"
                .to_string();
            eprintln!("warning: aws_kinesis_resource_policy: {warn_msg}");
            Ok(ConversionResult {
                region,
                warnings: vec![format!("aws_kinesis_resource_policy: {warn_msg}")],
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

// ---------------------------------------------------------------------------
// aws_kinesis_stream_consumer — populates StreamView.consumers on an
// existing stream. Same view-layer limitation as resource_policy: warning-only.
// ---------------------------------------------------------------------------

pub struct AwsKinesisStreamConsumerConverter {
    #[allow(dead_code)]
    service: Arc<KinesisService>,
}

impl AwsKinesisStreamConsumerConverter {
    pub fn new(service: Arc<KinesisService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKinesisStreamConsumerConverter {
    fn resource_type(&self) -> &str {
        "aws_kinesis_stream_consumer"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: kinesis_gen::StreamConsumerTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_kinesis_stream_consumer", e))?;
            let warn_msg = "stream consumers attach to an existing StreamView.consumers; the \
                            view-layer merge has no consumer-list patch path — inject is a \
                            no-op"
                .to_string();
            eprintln!("warning: aws_kinesis_stream_consumer: {warn_msg}");
            Ok(ConversionResult {
                region,
                warnings: vec![format!("aws_kinesis_stream_consumer: {warn_msg}")],
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
