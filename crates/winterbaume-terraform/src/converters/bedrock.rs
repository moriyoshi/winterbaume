//! Terraform converters for Bedrock resources.
//!
//! `GuardrailTfModel`, `CustomModelTfModel`, `GuardrailVersionTfModel`,
//! `InferenceProfileTfModel`, and `ProvisionedModelThroughputTfModel`
//! are generated from `specs/bedrock.toml`. The ARN templates, the
//! `READY` / `DRAFT` / `ACTIVE` constants, the `tags_all` merge, the
//! nested-block reads on guardrail (`content_policy_config`,
//! `contextual_grounding_policy_config`,
//! `sensitive_information_policy_config`, `topic_policy_config`,
//! `word_policy_config`), the `training_data_config`,
//! `output_data_config`, and `hyperparameters` reads on custom model,
//! and the `model_source.copy_from` read on inference profile are
//! wired up here. The
//! `aws_bedrock_model_invocation_logging_configuration` resource has
//! no strongly-typed scalar inputs — its sole field is the
//! `logging_config` nested block — so its converter remains
//! hand-written.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_bedrock::BedrockService;
use winterbaume_bedrock::views::BedrockStateView;
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::bedrock as bedrock_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: bedrock_gen::GuardrailTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_bedrock_guardrail", e))?;

        let name = model.name;
        let guardrail_id = model.guardrail_id.unwrap_or_else(|| name.clone());
        let guardrail_arn = model.guardrail_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:bedrock:{}:{}:guardrail/{}",
                region, ctx.default_account_id, guardrail_id
            )
        });
        let status = model.status.unwrap_or_else(|| "READY".to_string());
        let version = model.version.unwrap_or_else(|| "DRAFT".to_string());
        let created_at = model.created_at.unwrap_or_default();
        let updated_at = model.updated_at.unwrap_or_default();
        let blocked_input_messaging = model.blocked_input_messaging.unwrap_or_default();
        let blocked_outputs_messaging = model.blocked_outputs_messaging.unwrap_or_default();

        let attrs = &instance.attributes;
        let mut _tags = model.tags;
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
            name,
            description: model.description,
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
//
// This resource has no strongly-typed scalar inputs. Its sole field
// (`logging_config`) is a nested block carrying further nested blocks
// (`s3_configuration`, `cloudwatch_configuration`). The converter
// reads them straight from instance.attributes and writes directly to
// the `LoggingConfiguration` Rust struct, so it remains hand-written.

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

// ---------------------------------------------------------------------------
// aws_bedrock_custom_model
// ---------------------------------------------------------------------------

/// Converts `aws_bedrock_custom_model` Terraform resources to/from Bedrock state.
pub struct AwsBedrockCustomModelConverter {
    service: Arc<BedrockService>,
}

impl AwsBedrockCustomModelConverter {
    pub fn new(service: Arc<BedrockService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBedrockCustomModelConverter {
    fn resource_type(&self) -> &str {
        "aws_bedrock_custom_model"
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

impl AwsBedrockCustomModelConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: bedrock_gen::CustomModelTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_bedrock_custom_model", e))?;

        let attrs = &instance.attributes;

        // Nested blocks: training_data_config / output_data_config /
        // validation_data_config are HCL lists of one block in TF state. Pull
        // s3_uri out of the first element.
        let training_s3_uri = nested_first(attrs, "training_data_config")
            .and_then(|v| v.get("s3_uri").and_then(|s| s.as_str()))
            .unwrap_or_default()
            .to_string();
        let output_s3_uri = nested_first(attrs, "output_data_config")
            .and_then(|v| v.get("s3_uri").and_then(|s| s.as_str()))
            .unwrap_or_default()
            .to_string();

        // Map-typed `hyperparameters` (HCL map(string)) flows in as a JSON
        // object.
        let mut hyper_parameters: HashMap<String, String> = HashMap::new();
        if let Some(obj) = attrs.get("hyperparameters").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    hyper_parameters.insert(k.clone(), s.to_string());
                }
            }
        }

        let custom_model_name = model.custom_model_name.clone();
        let custom_model_arn = model.custom_model_arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:bedrock:{}:{}:custom-model/{}",
                region, ctx.default_account_id, custom_model_name
            )
        });
        let job_name = model.job_name.clone();
        let job_arn = model.job_arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:bedrock:{}:{}:model-customization-job/{}",
                region, ctx.default_account_id, job_name
            )
        });
        let customization_type = model
            .customization_type
            .clone()
            .unwrap_or_else(|| "FINE_TUNING".to_string());
        let creation_time = Utc::now().to_rfc3339();

        let cm = winterbaume_bedrock::types::CustomModel {
            model_arn: custom_model_arn.clone(),
            model_name: custom_model_name.clone(),
            base_model_arn: model.base_model_identifier.clone(),
            customization_type: customization_type.clone(),
            creation_time: creation_time.clone(),
            job_arn: job_arn.clone(),
            job_name: job_name.clone(),
            training_data_config: winterbaume_bedrock::types::TrainingDataConfig {
                s3_uri: training_s3_uri,
            },
            output_data_config: winterbaume_bedrock::types::OutputDataConfig {
                s3_uri: output_s3_uri,
            },
            hyper_parameters: hyper_parameters.clone(),
        };

        // Also create the matching customization job so that downstream
        // describe-job calls succeed.
        let cj = winterbaume_bedrock::types::ModelCustomizationJob {
            job_arn: job_arn.clone(),
            job_name: job_name.clone(),
            base_model_identifier: model.base_model_identifier,
            custom_model_name: custom_model_name.clone(),
            customization_type,
            role_arn: model.role_arn,
            status: model
                .job_status
                .clone()
                .unwrap_or_else(|| "Completed".to_string()),
            creation_time: creation_time.clone(),
            last_modified_time: creation_time,
            training_data_config: cm.training_data_config.clone(),
            output_data_config: cm.output_data_config.clone(),
            hyper_parameters,
        };

        let mut state_view = BedrockStateView::default();
        state_view.custom_models.insert(custom_model_arn, cm);
        state_view.customization_jobs.insert(job_arn, cj);
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
        for cm in view.custom_models.values() {
            let hyper: serde_json::Map<String, serde_json::Value> = cm
                .hyper_parameters
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            let attrs = serde_json::json!({
                "id": cm.model_arn,
                "custom_model_arn": cm.model_arn,
                "custom_model_name": cm.model_name,
                "base_model_identifier": cm.base_model_arn,
                "customization_type": cm.customization_type,
                "job_arn": cm.job_arn,
                "job_name": cm.job_name,
                "job_status": "Completed",
                "role_arn": "",
                "hyperparameters": hyper,
                "training_data_config": [{"s3_uri": cm.training_data_config.s3_uri}],
                "output_data_config": [{"s3_uri": cm.output_data_config.s3_uri}],
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: cm.model_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_bedrock_guardrail_version
// ---------------------------------------------------------------------------
//
// State gap: the Bedrock service stores a single `version` field on
// `Guardrail` and has no per-version snapshot collection. This converter
// looks the parent guardrail up by ARN and writes the resolved version
// string back into that field. The version published to AWS for a
// guardrail is monotonic and managed server-side; absent a richer state
// model we synthesise "1" when the TF state does not carry one yet.

/// Converts `aws_bedrock_guardrail_version` Terraform resources to/from Bedrock state.
pub struct AwsBedrockGuardrailVersionConverter {
    service: Arc<BedrockService>,
}

impl AwsBedrockGuardrailVersionConverter {
    pub fn new(service: Arc<BedrockService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBedrockGuardrailVersionConverter {
    fn resource_type(&self) -> &str {
        "aws_bedrock_guardrail_version"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_bedrock_guardrail"]
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

impl AwsBedrockGuardrailVersionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: bedrock_gen::GuardrailVersionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_bedrock_guardrail_version", e))?;

        let guardrail_arn = model.guardrail_arn;
        let version = model.version.unwrap_or_else(|| "1".to_string());

        // Look up the parent guardrail, struct-update it with the new
        // version, and merge it back. If the guardrail isn't present we
        // record a warning rather than fail outright; the dependency edge
        // in depends_on_types() drives the usual injection order.
        let mut warnings = vec![];
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut state_view = BedrockStateView::default();
        match snapshot
            .guardrails
            .values()
            .find(|g| g.guardrail_arn == guardrail_arn)
        {
            Some(existing) => {
                let updated = winterbaume_bedrock::types::Guardrail {
                    version: version.clone(),
                    description: model.description.or_else(|| existing.description.clone()),
                    updated_at: Utc::now().to_rfc3339(),
                    ..existing.clone()
                };
                state_view
                    .guardrails
                    .insert(updated.guardrail_id.clone(), updated);
                self.service
                    .merge(&ctx.default_account_id, &region, state_view)
                    .await?;
            }
            None => {
                warnings.push(format!(
                    "aws_bedrock_guardrail_version: parent guardrail {guardrail_arn} not found in Bedrock state; skipping version bump"
                ));
            }
        }

        Ok(ConversionResult { region, warnings })
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
            // Only emit a guardrail_version row when the guardrail has been
            // bumped past the implicit DRAFT version.
            if g.version == "DRAFT" {
                continue;
            }
            let attrs = serde_json::json!({
                "id": format!("{}:{}", g.guardrail_arn, g.version),
                "guardrail_arn": g.guardrail_arn,
                "version": g.version,
                "description": g.description,
                "skip_destroy": false,
            });
            results.push(ExtractedResource {
                name: format!("{}-v{}", g.name, g.version),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_bedrock_inference_profile
// ---------------------------------------------------------------------------

/// Converts `aws_bedrock_inference_profile` Terraform resources to/from Bedrock state.
pub struct AwsBedrockInferenceProfileConverter {
    service: Arc<BedrockService>,
}

impl AwsBedrockInferenceProfileConverter {
    pub fn new(service: Arc<BedrockService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBedrockInferenceProfileConverter {
    fn resource_type(&self) -> &str {
        "aws_bedrock_inference_profile"
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

impl AwsBedrockInferenceProfileConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: bedrock_gen::InferenceProfileTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_bedrock_inference_profile", e))?;

        let attrs = &instance.attributes;
        // model_source { copy_from = "<model-arn>" } -> InferenceProfile.model_arn.
        let model_source_arn = nested_first(attrs, "model_source")
            .and_then(|v| v.get("copy_from").and_then(|s| s.as_str()))
            .unwrap_or_default()
            .to_string();

        let name = model.name.clone();
        let profile_id = model.id.unwrap_or_else(|| format!("{region}.{name}"));
        let profile_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:bedrock:{}:{}:application-inference-profile/{}",
                region, ctx.default_account_id, profile_id
            )
        });
        let status = model.status.unwrap_or_else(|| "ACTIVE".to_string());
        let profile_type = model
            .profile_type
            .unwrap_or_else(|| "APPLICATION".to_string());
        let now = Utc::now().to_rfc3339();
        let created_at = model.created_at.unwrap_or_else(|| now.clone());
        let updated_at = model.updated_at.unwrap_or(now);

        let profile = winterbaume_bedrock::types::InferenceProfile {
            inference_profile_arn: profile_arn.clone(),
            inference_profile_id: profile_id,
            inference_profile_name: name,
            description: model.description,
            status,
            profile_type,
            model_arn: model_source_arn,
            created_at,
            updated_at,
        };

        let mut state_view = BedrockStateView::default();
        state_view.inference_profiles.insert(profile_arn, profile);
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
        for p in view.inference_profiles.values() {
            let attrs = serde_json::json!({
                "id": p.inference_profile_id,
                "arn": p.inference_profile_arn,
                "name": p.inference_profile_name,
                "description": p.description,
                "status": p.status,
                "type": p.profile_type,
                "models": [{"model_arn": p.model_arn}],
                "model_source": [{"copy_from": p.model_arn}],
                "created_at": p.created_at,
                "updated_at": p.updated_at,
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: p.inference_profile_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_bedrock_provisioned_model_throughput
// ---------------------------------------------------------------------------

/// Converts `aws_bedrock_provisioned_model_throughput` Terraform resources to/from Bedrock state.
pub struct AwsBedrockProvisionedModelThroughputConverter {
    service: Arc<BedrockService>,
}

impl AwsBedrockProvisionedModelThroughputConverter {
    pub fn new(service: Arc<BedrockService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBedrockProvisionedModelThroughputConverter {
    fn resource_type(&self) -> &str {
        "aws_bedrock_provisioned_model_throughput"
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

impl AwsBedrockProvisionedModelThroughputConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: bedrock_gen::ProvisionedModelThroughputTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_bedrock_provisioned_model_throughput", e)
            })?;

        let provisioned_model_arn = model.provisioned_model_arn.clone().unwrap_or_else(|| {
            let id = uuid::Uuid::new_v4().to_string();
            format!(
                "arn:aws:bedrock:{}:{}:provisioned-model/{}",
                region, ctx.default_account_id, id
            )
        });
        let now = Utc::now().to_rfc3339();

        let throughput = winterbaume_bedrock::types::ProvisionedModelThroughput {
            provisioned_model_arn: provisioned_model_arn.clone(),
            provisioned_model_name: model.provisioned_model_name,
            model_arn: model.model_arn,
            model_units: model.model_units as i32,
            status: "InService".to_string(),
            commitment_duration: model.commitment_duration,
            creation_time: now.clone(),
            last_modified_time: now,
        };

        let mut state_view = BedrockStateView::default();
        state_view
            .provisioned_models
            .insert(provisioned_model_arn, throughput);
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
        for p in view.provisioned_models.values() {
            let attrs = serde_json::json!({
                "id": p.provisioned_model_arn,
                "provisioned_model_arn": p.provisioned_model_arn,
                "provisioned_model_name": p.provisioned_model_name,
                "model_arn": p.model_arn,
                "model_units": p.model_units,
                "commitment_duration": p.commitment_duration,
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: p.provisioned_model_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Read the first element of a Terraform single-instance nested block,
/// which arrives as a JSON list of one object.
fn nested_first<'a>(attrs: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    attrs
        .get(key)
        .and_then(|v| v.as_array())
        .and_then(|a| a.first())
}
