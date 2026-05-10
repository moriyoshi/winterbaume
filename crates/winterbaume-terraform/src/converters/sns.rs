//! Terraform converters for SNS resources.
//!
//! `TopicTfModel` and `SubscriptionTfModel` are generated from
//! `specs/sns.toml` (see `crate::generated::sns`). The generated structs
//! cover the flat field projection; the AWS-side `TopicView.attributes`
//! / `SubscriptionView.attributes` bags are PascalCase HashMaps that
//! don't fit the thin-projection pattern, so the bag construction lives
//! here.

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
use crate::generated::sns as sns_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sns_gen::TopicTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_sns_topic", e))?;

        let arn = model
            .arn
            .clone()
            .or_else(|| model.id.clone())
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:sns:{}:{}:{}",
                    region, ctx.default_account_id, model.name
                )
            });

        let topic_view = TopicView {
            arn: arn.clone(),
            name: model.name.clone(),
            attributes: build_topic_attributes(&model),
            tags: model.tags.clone(),
            permissions: HashMap::new(),
            data_protection_policy: None,
        };

        let mut state_view = SnsStateView::default();
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
            let attrs = topic_extract_attributes(topic);
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

/// Pack the model's flat fields into the AWS-side PascalCase attributes bag.
/// Empty strings are dropped to match the original hand-rolled converter.
fn build_topic_attributes(model: &sns_gen::TopicTfModel) -> HashMap<String, String> {
    let mut a = HashMap::new();
    insert_non_empty(&mut a, "DisplayName", model.display_name.as_deref());
    insert_non_empty(&mut a, "Policy", model.policy.as_deref());
    insert_non_empty(&mut a, "DeliveryPolicy", model.delivery_policy.as_deref());
    insert_non_empty(&mut a, "KmsMasterKeyId", model.kms_master_key_id.as_deref());
    a.insert("FifoTopic".to_string(), model.fifo_topic.to_string());
    a.insert(
        "ContentBasedDeduplication".to_string(),
        model.content_based_deduplication.to_string(),
    );
    for (key, val) in TOPIC_FEEDBACK_ATTRS {
        let opt = topic_feedback_field(model, key);
        insert_non_empty(&mut a, val, opt);
    }
    a
}

const TOPIC_FEEDBACK_ATTRS: &[(&str, &str)] = &[
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

fn topic_feedback_field<'a>(model: &'a sns_gen::TopicTfModel, key: &str) -> Option<&'a str> {
    let opt = match key {
        "sqs_success_feedback_role_arn" => &model.sqs_success_feedback_role_arn,
        "sqs_failure_feedback_role_arn" => &model.sqs_failure_feedback_role_arn,
        "sqs_success_feedback_sample_rate" => &model.sqs_success_feedback_sample_rate,
        "lambda_success_feedback_role_arn" => &model.lambda_success_feedback_role_arn,
        "lambda_failure_feedback_role_arn" => &model.lambda_failure_feedback_role_arn,
        "lambda_success_feedback_sample_rate" => &model.lambda_success_feedback_sample_rate,
        "http_success_feedback_role_arn" => &model.http_success_feedback_role_arn,
        "http_failure_feedback_role_arn" => &model.http_failure_feedback_role_arn,
        "http_success_feedback_sample_rate" => &model.http_success_feedback_sample_rate,
        "firehose_success_feedback_role_arn" => &model.firehose_success_feedback_role_arn,
        "firehose_failure_feedback_role_arn" => &model.firehose_failure_feedback_role_arn,
        "firehose_success_feedback_sample_rate" => &model.firehose_success_feedback_sample_rate,
        "application_success_feedback_role_arn" => &model.application_success_feedback_role_arn,
        "application_failure_feedback_role_arn" => &model.application_failure_feedback_role_arn,
        "application_success_feedback_sample_rate" => {
            &model.application_success_feedback_sample_rate
        }
        _ => &None,
    };
    opt.as_deref()
}

fn topic_extract_attributes(topic: &TopicView) -> serde_json::Value {
    let get = |k: &str| topic.attributes.get(k).cloned().unwrap_or_default();
    let fifo_topic = get("FifoTopic") == "true";
    let content_based_dedup = get("ContentBasedDeduplication") == "true";
    let owner = topic.arn.split(':').nth(4).unwrap_or("").to_string();
    serde_json::json!({
        "id": topic.arn,
        "arn": topic.arn,
        "name": topic.name,
        "owner": owner,
        "display_name": get("DisplayName"),
        "policy": get("Policy"),
        "delivery_policy": get("DeliveryPolicy"),
        "kms_master_key_id": get("KmsMasterKeyId"),
        "fifo_topic": fifo_topic,
        "content_based_deduplication": content_based_dedup,
        "sqs_success_feedback_role_arn": get("SQSSuccessFeedbackRoleArn"),
        "sqs_failure_feedback_role_arn": get("SQSFailureFeedbackRoleArn"),
        "sqs_success_feedback_sample_rate": get("SQSSuccessFeedbackSampleRate"),
        "lambda_success_feedback_role_arn": get("LambdaSuccessFeedbackRoleArn"),
        "lambda_failure_feedback_role_arn": get("LambdaFailureFeedbackRoleArn"),
        "lambda_success_feedback_sample_rate": get("LambdaSuccessFeedbackSampleRate"),
        "http_success_feedback_role_arn": get("HTTPSuccessFeedbackRoleArn"),
        "http_failure_feedback_role_arn": get("HTTPFailureFeedbackRoleArn"),
        "http_success_feedback_sample_rate": get("HTTPSuccessFeedbackSampleRate"),
        "firehose_success_feedback_role_arn": get("FirehoseSuccessFeedbackRoleArn"),
        "firehose_failure_feedback_role_arn": get("FirehoseFailureFeedbackRoleArn"),
        "firehose_success_feedback_sample_rate": get("FirehoseSuccessFeedbackSampleRate"),
        "application_success_feedback_role_arn": get("ApplicationSuccessFeedbackRoleArn"),
        "application_failure_feedback_role_arn": get("ApplicationFailureFeedbackRoleArn"),
        "application_success_feedback_sample_rate": get("ApplicationSuccessFeedbackSampleRate"),
        "tags": topic.tags,
    })
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: sns_gen::SubscriptionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_sns_topic_subscription", e))?;

        let arn = model
            .arn
            .clone()
            .or_else(|| model.id.clone())
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:sns:{}:{}:{}:{}",
                    region,
                    ctx.default_account_id,
                    model.topic_arn.rsplit(':').next().unwrap_or("topic"),
                    uuid::Uuid::new_v4()
                )
            });

        let mut sub_attributes: HashMap<String, String> = HashMap::new();
        if let Some(raw_msg) = model.raw_message_delivery.clone() {
            sub_attributes.insert("RawMessageDelivery".to_string(), raw_msg);
        }
        insert_non_empty(
            &mut sub_attributes,
            "FilterPolicy",
            model.filter_policy.as_deref(),
        );

        let sub_view = SubscriptionView {
            arn: arn.clone(),
            topic_arn: model.topic_arn.clone(),
            protocol: model.protocol.clone(),
            endpoint: model.endpoint.clone(),
            confirmed: true,
            owner: ctx.default_account_id.clone(),
            attributes: sub_attributes,
        };

        let mut state_view = SnsStateView::default();
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

// ---------------------------------------------------------------------------
// shared helpers
// ---------------------------------------------------------------------------

fn insert_non_empty(map: &mut HashMap<String, String>, key: &str, value: Option<&str>) {
    if let Some(v) = value
        && !v.is_empty()
    {
        map.insert(key.to_string(), v.to_string());
    }
}
