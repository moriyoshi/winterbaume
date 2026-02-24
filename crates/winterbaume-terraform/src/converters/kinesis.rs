//! Terraform converter for `aws_kinesis_stream` resources.

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
use crate::util::{extract_region, extract_tags, optional_i64, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_kinesis_stream")?;
        let region = extract_region(attrs, &ctx.default_region);

        let arn = optional_str(attrs, "arn")
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:kinesis:{}:{}:stream/{}",
                    region, ctx.default_account_id, name
                )
            });

        // Stream mode: ON_DEMAND or PROVISIONED (default PROVISIONED)
        let stream_mode = attrs
            .get("stream_mode_details")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("stream_mode"))
            .and_then(|v| v.as_str())
            .unwrap_or("PROVISIONED")
            .to_string();

        let shard_count = optional_i64(attrs, "shard_count").unwrap_or(1) as usize;
        let retention_period_hours = optional_i64(attrs, "retention_period").unwrap_or(24) as u32;

        let encryption_type =
            optional_str(attrs, "encryption_type").unwrap_or_else(|| "NONE".to_string());
        let key_id = optional_str(attrs, "kms_key_id");

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
            name: name.to_string(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            stream_mode,
            shards,
            retention_period_hours,
            encryption_type,
            key_id,
            tags: extract_tags(attrs),
            consumers: vec![],
            created_timestamp: Some(Utc::now().to_rfc3339()),
            account_id: ctx.default_account_id.clone(),
            enhanced_monitoring: vec![],
            resource_policy: None,
            max_record_size_in_ki_b: None,
        };

        let mut state_view = KinesisStateView::default();
        state_view.streams.insert(name.to_string(), stream_view);
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
