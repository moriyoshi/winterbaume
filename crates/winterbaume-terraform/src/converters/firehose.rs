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
use crate::util::{extract_region, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_kinesis_firehose_delivery_stream")?;
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:firehose:{}:{}:deliverystream/{}",
                region, ctx.default_account_id, name
            )
        });
        let destination = optional_str(attrs, "destination").unwrap_or_else(|| "s3".to_string());

        let _tags_all = attrs.get("tags_all");
        let _extended_s3_configuration = attrs.get("extended_s3_configuration");
        let _http_endpoint_configuration = attrs.get("http_endpoint_configuration");
        let _opensearch_configuration = attrs.get("opensearch_configuration");
        let _opensearchserverless_configuration = attrs.get("opensearchserverless_configuration");
        let _splunk_configuration = attrs.get("splunk_configuration");

        let mut tags: HashMap<String, String> = HashMap::new();
        if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.insert(k.clone(), s.to_string());
                }
            }
        }

        let encryption_status = optional_str(attrs, "server_side_encryption")
            .and_then(|_| {
                attrs
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
            })
            .unwrap_or_else(|| "DISABLED".to_string());

        let version_id = optional_str(attrs, "version_id").unwrap_or_else(|| "1".to_string());

        let stream_view = DeliveryStreamView {
            name: name.to_string(),
            arn,
            status: "ACTIVE".to_string(),
            destination_type: destination,
            destination_id: uuid::Uuid::new_v4().to_string(),
            created_timestamp: Some(Utc::now().to_rfc3339()),
            tags,
            encryption_status,
            version_id,
        };

        let mut state_view = FirehoseStateView {
            streams: HashMap::new(),
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
