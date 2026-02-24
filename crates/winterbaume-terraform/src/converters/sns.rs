//! Terraform converters for SNS resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_sns::SnsService;
use winterbaume_sns::views::{SnsStateView, SubscriptionView, TopicView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_sns_topic
// ---------------------------------------------------------------------------

/// Converts `aws_sns_topic` Terraform resources to/from SNS topic state.
pub struct AwsSnsTopicConverter {
    service: Arc<SnsService>,
}

impl AwsSnsTopicConverter {
    pub fn new(service: Arc<SnsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSnsTopicConverter {
    fn resource_type(&self) -> &str {
        "aws_sns_topic"
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

impl AwsSnsTopicConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_sns_topic")?;
        let region = extract_region(attrs, &ctx.default_region);

        let arn = optional_str(attrs, "arn")
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| {
                format!("arn:aws:sns:{}:{}:{}", region, ctx.default_account_id, name)
            });

        let _tags_all = attrs.get("tags_all");
        let _tracing_config = optional_str(attrs, "tracing_config");
        let _archive_policy = optional_str(attrs, "archive_policy");
        let _signature_version = optional_str(attrs, "signature_version");
        let _name_prefix = optional_str(attrs, "name_prefix");
        let _http_success_feedback_sample_rate =
            optional_str(attrs, "http_success_feedback_sample_rate");
        let _sqs_success_feedback_sample_rate =
            optional_str(attrs, "sqs_success_feedback_sample_rate");
        let _lambda_success_feedback_sample_rate =
            optional_str(attrs, "lambda_success_feedback_sample_rate");
        let _firehose_success_feedback_sample_rate =
            optional_str(attrs, "firehose_success_feedback_sample_rate");
        let _application_success_feedback_sample_rate =
            optional_str(attrs, "application_success_feedback_sample_rate");
        let _fifo_throughput_scope = optional_str(attrs, "fifo_throughput_scope");

        let mut attributes: HashMap<String, String> = HashMap::new();
        if let Some(display_name) = optional_str(attrs, "display_name") {
            if !display_name.is_empty() {
                attributes.insert("DisplayName".to_string(), display_name);
            }
        }
        if let Some(policy) = optional_str(attrs, "policy") {
            if !policy.is_empty() {
                attributes.insert("Policy".to_string(), policy);
            }
        }
        if let Some(delivery_policy) = optional_str(attrs, "delivery_policy") {
            if !delivery_policy.is_empty() {
                attributes.insert("DeliveryPolicy".to_string(), delivery_policy);
            }
        }
        if let Some(kms_master_key_id) = optional_str(attrs, "kms_master_key_id") {
            if !kms_master_key_id.is_empty() {
                attributes.insert("KmsMasterKeyId".to_string(), kms_master_key_id);
            }
        }
        // FIFO topic attributes
        if let Some(fifo_topic) = attrs.get("fifo_topic").and_then(|v| v.as_bool()) {
            attributes.insert("FifoTopic".to_string(), fifo_topic.to_string());
        }
        if let Some(content_based_dedup) = attrs
            .get("content_based_deduplication")
            .and_then(|v| v.as_bool())
        {
            attributes.insert(
                "ContentBasedDeduplication".to_string(),
                content_based_dedup.to_string(),
            );
        }
        // Feedback role ARNs
        let feedback_attrs = [
            ("sqs_success_feedback_role_arn", "SQSSuccessFeedbackRoleArn"),
            ("sqs_failure_feedback_role_arn", "SQSFailureFeedbackRoleArn"),
            (
                "sqs_success_feedback_sample_rate",
                "SQSSuccessFeedbackSampleRate",
            ),
            (
                "lambda_success_feedback_role_arn",
                "LambdaSuccessFeedbackRoleArn",
            ),
            (
                "lambda_failure_feedback_role_arn",
                "LambdaFailureFeedbackRoleArn",
            ),
            (
                "lambda_success_feedback_sample_rate",
                "LambdaSuccessFeedbackSampleRate",
            ),
            (
                "http_success_feedback_role_arn",
                "HTTPSuccessFeedbackRoleArn",
            ),
            (
                "http_failure_feedback_role_arn",
                "HTTPFailureFeedbackRoleArn",
            ),
            (
                "http_success_feedback_sample_rate",
                "HTTPSuccessFeedbackSampleRate",
            ),
            (
                "firehose_success_feedback_role_arn",
                "FirehoseSuccessFeedbackRoleArn",
            ),
            (
                "firehose_failure_feedback_role_arn",
                "FirehoseFailureFeedbackRoleArn",
            ),
            (
                "firehose_success_feedback_sample_rate",
                "FirehoseSuccessFeedbackSampleRate",
            ),
            (
                "application_success_feedback_role_arn",
                "ApplicationSuccessFeedbackRoleArn",
            ),
            (
                "application_failure_feedback_role_arn",
                "ApplicationFailureFeedbackRoleArn",
            ),
            (
                "application_success_feedback_sample_rate",
                "ApplicationSuccessFeedbackSampleRate",
            ),
        ];
        for (tf_key, sns_key) in &feedback_attrs {
            if let Some(val) = optional_str(attrs, tf_key).filter(|v| !v.is_empty()) {
                attributes.insert(sns_key.to_string(), val);
            }
        }

        let topic_view = TopicView {
            arn: arn.clone(),
            name: name.to_string(),
            attributes,
            tags: extract_tags(attrs),
            permissions: HashMap::new(),
            data_protection_policy: None,
        };

        let mut state_view = SnsStateView {
            ..Default::default()
        };
        state_view.topics.insert(arn, topic_view);
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
        for topic in view.topics.values() {
            let get_attr =
                |key: &str| -> String { topic.attributes.get(key).cloned().unwrap_or_default() };
            let fifo_topic = get_attr("FifoTopic") == "true";
            let content_based_dedup = get_attr("ContentBasedDeduplication") == "true";
            let owner = topic.arn.split(':').nth(4).unwrap_or("").to_string();
            let attrs = serde_json::json!({
                "id": topic.arn,
                "arn": topic.arn,
                "name": topic.name,
                "owner": owner,
                "display_name": get_attr("DisplayName"),
                "policy": get_attr("Policy"),
                "delivery_policy": get_attr("DeliveryPolicy"),
                "kms_master_key_id": get_attr("KmsMasterKeyId"),
                "fifo_topic": fifo_topic,
                "content_based_deduplication": content_based_dedup,
                "sqs_success_feedback_role_arn": get_attr("SQSSuccessFeedbackRoleArn"),
                "sqs_failure_feedback_role_arn": get_attr("SQSFailureFeedbackRoleArn"),
                "sqs_success_feedback_sample_rate": get_attr("SQSSuccessFeedbackSampleRate"),
                "lambda_success_feedback_role_arn": get_attr("LambdaSuccessFeedbackRoleArn"),
                "lambda_failure_feedback_role_arn": get_attr("LambdaFailureFeedbackRoleArn"),
                "lambda_success_feedback_sample_rate": get_attr("LambdaSuccessFeedbackSampleRate"),
                "http_success_feedback_role_arn": get_attr("HTTPSuccessFeedbackRoleArn"),
                "http_failure_feedback_role_arn": get_attr("HTTPFailureFeedbackRoleArn"),
                "http_success_feedback_sample_rate": get_attr("HTTPSuccessFeedbackSampleRate"),
                "firehose_success_feedback_role_arn": get_attr("FirehoseSuccessFeedbackRoleArn"),
                "firehose_failure_feedback_role_arn": get_attr("FirehoseFailureFeedbackRoleArn"),
                "firehose_success_feedback_sample_rate": get_attr("FirehoseSuccessFeedbackSampleRate"),
                "application_success_feedback_role_arn": get_attr("ApplicationSuccessFeedbackRoleArn"),
                "application_failure_feedback_role_arn": get_attr("ApplicationFailureFeedbackRoleArn"),
                "application_success_feedback_sample_rate": get_attr("ApplicationSuccessFeedbackSampleRate"),
                "tags": topic.tags,
            });
            results.push(ExtractedResource {
                name: topic.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sns_topic_subscription
// ---------------------------------------------------------------------------

/// Converts `aws_sns_topic_subscription` Terraform resources to/from SNS state.
pub struct AwsSnsTopicSubscriptionConverter {
    service: Arc<SnsService>,
}

impl AwsSnsTopicSubscriptionConverter {
    pub fn new(service: Arc<SnsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSnsTopicSubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_sns_topic_subscription"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_sns_topic"]
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

impl AwsSnsTopicSubscriptionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let topic_arn = require_str(attrs, "topic_arn", "aws_sns_topic_subscription")?;
        let protocol = require_str(attrs, "protocol", "aws_sns_topic_subscription")?;
        let endpoint = require_str(attrs, "endpoint", "aws_sns_topic_subscription")?;
        let region = extract_region(attrs, &ctx.default_region);

        let arn = optional_str(attrs, "arn")
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:sns:{}:{}:{}:{}",
                    region,
                    ctx.default_account_id,
                    topic_arn.rsplit(':').next().unwrap_or("topic"),
                    uuid::Uuid::new_v4()
                )
            });

        let _tags_all = attrs.get("tags_all");
        let _confirmation_timeout_in_minutes = attrs.get("confirmation_timeout_in_minutes");
        let _delivery_policy = optional_str(attrs, "delivery_policy");
        let _endpoint_auto_confirms = attrs.get("endpoint_auto_confirms");
        let _filter_policy_scope = optional_str(attrs, "filter_policy_scope");

        let mut sub_attributes: HashMap<String, String> = HashMap::new();
        if let Some(raw_msg) = optional_str(attrs, "raw_message_delivery") {
            sub_attributes.insert("RawMessageDelivery".to_string(), raw_msg);
        }
        if let Some(filter_policy) = optional_str(attrs, "filter_policy") {
            if !filter_policy.is_empty() {
                sub_attributes.insert("FilterPolicy".to_string(), filter_policy);
            }
        }

        let sub_view = SubscriptionView {
            arn: arn.clone(),
            topic_arn: topic_arn.to_string(),
            protocol: protocol.to_string(),
            endpoint: endpoint.to_string(),
            confirmed: true,
            owner: ctx.default_account_id.clone(),
            attributes: sub_attributes,
        };

        let mut state_view = SnsStateView {
            ..Default::default()
        };
        state_view.subscriptions.insert(arn, sub_view);
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
        for sub in view.subscriptions.values() {
            let attrs = serde_json::json!({
                "id": sub.arn,
                "arn": sub.arn,
                "topic_arn": sub.topic_arn,
                "protocol": sub.protocol,
                "endpoint": sub.endpoint,
                "confirmed": sub.confirmed,
                "owner": sub.owner,
                "raw_message_delivery": sub.attributes.get("RawMessageDelivery").cloned().unwrap_or_else(|| "false".to_string()),
                "filter_policy": sub.attributes.get("FilterPolicy").cloned().unwrap_or_default(),
                "tags_all": {},
                "confirmation_was_authenticated": sub.confirmed,
                "subscription_role_arn": sub.attributes.get("SubscriptionRoleArn").cloned().unwrap_or_default(),
            });
            results.push(ExtractedResource {
                name: sub.arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
