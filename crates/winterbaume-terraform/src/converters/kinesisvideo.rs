//! Terraform converter for Kinesis Video resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_kinesisvideo::KinesisVideoService;
use winterbaume_kinesisvideo::views::{KinesisVideoStateView, StreamView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_i64, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_kinesis_video_stream
// ---------------------------------------------------------------------------

/// Converts `aws_kinesis_video_stream` Terraform resources to/from Kinesis Video state.
pub struct AwsKinesisVideoStreamConverter {
    service: Arc<KinesisVideoService>,
}

impl AwsKinesisVideoStreamConverter {
    pub fn new(service: Arc<KinesisVideoService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKinesisVideoStreamConverter {
    fn resource_type(&self) -> &str {
        "aws_kinesis_video_stream"
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

impl AwsKinesisVideoStreamConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_kinesis_video_stream")?;
        let region = extract_region(attrs, &ctx.default_region);
        let device_name = optional_str(attrs, "device_name");
        let media_type = optional_str(attrs, "media_type");
        let kms_key_id =
            optional_str(attrs, "kms_key_id").unwrap_or_else(|| "aws/kinesisvideo".to_string());
        let data_retention_in_hours =
            optional_i64(attrs, "data_retention_in_hours").unwrap_or(0) as i32;

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:kinesisvideo:{}:{}:stream/{}/0000000000000",
                region, ctx.default_account_id, name
            )
        });

        let stream_view = StreamView {
            stream_name: name.to_string(),
            stream_arn: arn,
            device_name,
            media_type,
            kms_key_id,
            version: optional_str(attrs, "version").unwrap_or_else(|| "1".to_string()),
            status: optional_str(attrs, "status").unwrap_or_else(|| "ACTIVE".to_string()),
            creation_time: optional_str(attrs, "creation_time")
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            data_retention_in_hours,
            tags: extract_tags(attrs),
            image_generation_config: None,
            notification_config: None,
            storage_config: None,
            edge_config: None,
        };

        let mut state_view = KinesisVideoStateView {
            streams: HashMap::new(),
            channels: HashMap::new(),
        };
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
            let attrs = serde_json::json!({
                "id": stream.stream_arn,
                "name": stream.stream_name,
                "arn": stream.stream_arn,
                "device_name": stream.device_name,
                "media_type": stream.media_type,
                "kms_key_id": stream.kms_key_id,
                "version": stream.version,
                "status": stream.status,
                "creation_time": stream.creation_time,
                "data_retention_in_hours": stream.data_retention_in_hours,
                "tags": stream.tags,
            });
            results.push(ExtractedResource {
                name: stream.stream_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
