//! Terraform converters for Kinesis Firehose resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_firehose::FirehoseService;
use winterbaume_firehose::views::{DeliveryStreamView, FirehoseStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::firehose as firehose_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_kinesis_firehose_delivery_stream
// ---------------------------------------------------------------------------

pub struct AwsKinesisFirehoseDeliveryStreamConverter {
    service: Arc<FirehoseService>,
}

impl AwsKinesisFirehoseDeliveryStreamConverter {
    pub fn new(service: Arc<FirehoseService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKinesisFirehoseDeliveryStreamConverter {
    fn resource_type(&self) -> &str {
        "aws_kinesis_firehose_delivery_stream"
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

impl AwsKinesisFirehoseDeliveryStreamConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: firehose_gen::DeliveryStreamTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_kinesis_firehose_delivery_stream", e)
            })?;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:firehose:{}:{}:deliverystream/{}",
                region, ctx.default_account_id, name
            )
        });
        let destination = model.destination.unwrap_or_else(|| "s3".to_string());
        let version_id = model.version_id.unwrap_or_else(|| "1".to_string());

        // server_side_encryption is a nested-block array: [{enabled: bool}]
        let encryption_status = instance
            .attributes
            .get("server_side_encryption")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|obj| obj.get("enabled"))
            .and_then(|v| v.as_bool())
            .map(|enabled| {
                if enabled {
                    "ENABLED".to_string()
                } else {
                    "DISABLED".to_string()
                }
            })
            .unwrap_or_else(|| "DISABLED".to_string());

        let stream_view = DeliveryStreamView {
            name: name.clone(),
            arn,
            status: "ACTIVE".to_string(),
            destination_type: destination,
            destination_id: uuid::Uuid::new_v4().to_string(),
            created_timestamp: Some(Utc::now().to_rfc3339()),
            tags: model.tags,
            encryption_status,
            version_id,
        };

        let mut state_view = FirehoseStateView {
            streams: HashMap::new(),
        };
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
            let attrs = serde_json::json!({
                "id": stream.arn,
                "name": stream.name,
                "arn": stream.arn,
                "destination": stream.destination_type,
                "destination_id": stream.destination_id,
                "version_id": stream.version_id,
                "tags": stream.tags,
                "tags_all": stream.tags,
                "server_side_encryption": [{
                    "enabled": stream.encryption_status == "ENABLED",
                }],
                "kinesis_source_configuration": [],
                "elasticsearch_configuration": [],
                "arn": stream.arn,
                "destination": stream.destination_type,
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
