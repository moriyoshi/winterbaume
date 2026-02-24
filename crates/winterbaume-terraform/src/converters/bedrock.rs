//! Terraform converters for Bedrock resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_bedrock::BedrockService;
use winterbaume_bedrock::views::BedrockStateView;
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_bedrock_guardrail
// ---------------------------------------------------------------------------

/// Converts `aws_bedrock_guardrail` Terraform resources to/from Bedrock state.
pub struct AwsBedrockGuardrailConverter {
    service: Arc<BedrockService>,
}

impl AwsBedrockGuardrailConverter {
    pub fn new(service: Arc<BedrockService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBedrockGuardrailConverter {
    fn resource_type(&self) -> &str {
        "aws_bedrock_guardrail"
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

impl AwsBedrockGuardrailConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_bedrock_guardrail")?;
        let guardrail_id = optional_str(attrs, "guardrail_id").unwrap_or_else(|| name.to_string());
        let guardrail_arn = optional_str(attrs, "guardrail_arn").unwrap_or_else(|| {
            format!(
                "arn:aws:bedrock:{}:{}:guardrail/{}",
                region, ctx.default_account_id, guardrail_id
            )
        });
        let description = optional_str(attrs, "description");
        let status = optional_str(attrs, "status").unwrap_or_else(|| "READY".to_string());
        let version = optional_str(attrs, "version").unwrap_or_else(|| "DRAFT".to_string());
        let created_at = optional_str(attrs, "created_at").unwrap_or_default();
        let updated_at = optional_str(attrs, "updated_at").unwrap_or_default();
        let blocked_input_messaging =
            optional_str(attrs, "blocked_input_messaging").unwrap_or_default();
        let blocked_outputs_messaging =
            optional_str(attrs, "blocked_outputs_messaging").unwrap_or_default();
        let _kms_key_arn = optional_str(attrs, "kms_key_arn");
        let mut _tags = extract_tags(attrs);
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    _tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let content_policy_config = attrs.get("content_policy_config").cloned();
        let contextual_grounding_policy_config =
            attrs.get("contextual_grounding_policy_config").cloned();
        let sensitive_information_policy_config =
            attrs.get("sensitive_information_policy_config").cloned();
        let topic_policy_config = attrs.get("topic_policy_config").cloned();
        let word_policy_config = attrs.get("word_policy_config").cloned();

        let guardrail = winterbaume_bedrock::types::Guardrail {
            guardrail_id: guardrail_id.clone(),
            guardrail_arn,
            name: name.to_string(),
            description,
            status,
            version,
            created_at,
            updated_at,
            blocked_input_messaging,
            blocked_outputs_messaging,
            content_policy_config,
            contextual_grounding_policy_config,
            sensitive_information_policy_config,
            topic_policy_config,
            word_policy_config,
        };

        let mut state_view = BedrockStateView::default();
        state_view.guardrails.insert(guardrail_id, guardrail);
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
        for g in view.guardrails.values() {
            let attrs = serde_json::json!({
                "id": g.guardrail_id,
                "guardrail_id": g.guardrail_id,
                "guardrail_arn": g.guardrail_arn,
                "name": g.name,
                "description": g.description,
                "status": g.status,
                "version": g.version,
                "created_at": g.created_at,
                "updated_at": g.updated_at,
                "blocked_input_messaging": g.blocked_input_messaging,
                "blocked_outputs_messaging": g.blocked_outputs_messaging,
                "tags_all": {},
                "kms_key_arn": null,
                "content_policy_config": g.content_policy_config,
                "contextual_grounding_policy_config": g.contextual_grounding_policy_config,
                "sensitive_information_policy_config": g.sensitive_information_policy_config,
                "topic_policy_config": g.topic_policy_config,
                "word_policy_config": g.word_policy_config,
            });
            results.push(ExtractedResource {
                name: g.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_bedrock_model_invocation_logging_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_bedrock_model_invocation_logging_configuration` Terraform
/// resources to/from Bedrock state.
pub struct AwsBedrockModelInvocationLoggingConfigurationConverter {
    service: Arc<BedrockService>,
}

impl AwsBedrockModelInvocationLoggingConfigurationConverter {
    pub fn new(service: Arc<BedrockService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBedrockModelInvocationLoggingConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_bedrock_model_invocation_logging_configuration"
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

impl AwsBedrockModelInvocationLoggingConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let logging_config_val = attrs.get("logging_config");

        let mut s3_config = None;
        let mut cloud_watch_config = None;
        let mut text_data_delivery_enabled = None;
        let mut image_data_delivery_enabled = None;
        let mut embedding_data_delivery_enabled = None;

        if let Some(lc) = logging_config_val
            .and_then(|v| v.as_array())
            .and_then(|a| a.first())
        {
            text_data_delivery_enabled = lc
                .get("text_data_delivery_enabled")
                .and_then(|v| v.as_bool());
            image_data_delivery_enabled = lc
                .get("image_data_delivery_enabled")
                .and_then(|v| v.as_bool());
            embedding_data_delivery_enabled = lc
                .get("embedding_data_delivery_enabled")
                .and_then(|v| v.as_bool());

            if let Some(s3) = lc
                .get("s3_configuration")
                .and_then(|v| v.as_array())
                .and_then(|a| a.first())
            {
                let bucket_name = s3
                    .get("bucket_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let key_prefix = s3
                    .get("key_prefix")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                s3_config = Some(winterbaume_bedrock::types::S3LogConfig {
                    bucket_name,
                    key_prefix,
                });
            }
            if let Some(cw) = lc
                .get("cloudwatch_configuration")
                .and_then(|v| v.as_array())
                .and_then(|a| a.first())
            {
                let log_group_name = cw
                    .get("log_group_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let role_arn = cw
                    .get("role_arn")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                cloud_watch_config = Some(winterbaume_bedrock::types::CloudWatchLogConfig {
                    log_group_name,
                    role_arn,
                });
            }
        }

        let config = winterbaume_bedrock::types::LoggingConfiguration {
            text_data_delivery_enabled,
            image_data_delivery_enabled,
            embedding_data_delivery_enabled,
            s3_config,
            cloud_watch_config,
        };

        let state_view = BedrockStateView {
            logging_config: Some(config),
            ..Default::default()
        };
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
        if let Some(config) = &view.logging_config {
            let s3_configuration = config.s3_config.as_ref().map(|s3| {
                serde_json::json!([{
                    "bucket_name": s3.bucket_name,
                    "key_prefix": s3.key_prefix,
                }])
            });
            let cloudwatch_configuration = config.cloud_watch_config.as_ref().map(|cw| {
                serde_json::json!([{
                    "log_group_name": cw.log_group_name,
                    "role_arn": cw.role_arn,
                }])
            });
            let attrs = serde_json::json!({
                "id": ctx.default_account_id,
                "logging_config": [{
                    "text_data_delivery_enabled": config.text_data_delivery_enabled,
                    "image_data_delivery_enabled": config.image_data_delivery_enabled,
                    "embedding_data_delivery_enabled": config.embedding_data_delivery_enabled,
                    "s3_configuration": s3_configuration,
                    "cloudwatch_configuration": cloudwatch_configuration,
                }],
            });
            results.push(ExtractedResource {
                name: "logging".to_string(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
