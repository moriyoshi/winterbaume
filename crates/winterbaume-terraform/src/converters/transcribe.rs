//! Terraform converters for Transcribe resources.
//!
//! `VocabularyTfModel` and `LanguageModelTfModel` are generated from
//! `specs/transcribe.toml`. The vocabulary `vocabulary_state`
//! constants, the `last_modified_time` (f64) extraction, the
//! `phrases` array (`Vec<String>`), and the language model
//! `input_data_config` nested block are wired up here.
//!
//! Note: Transcribe language models are not directly represented in the
//! TranscribeStateView (no dedicated LanguageModel view type exists yet).
//! For now we store language model metadata as a vocabulary entry with a
//! special naming convention so extraction can distinguish them.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;
use winterbaume_transcribe::TranscribeService;
use winterbaume_transcribe::views::{TranscribeStateView, VocabularyView};

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::transcribe as transcribe_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_transcribe_vocabulary
// ---------------------------------------------------------------------------

pub struct AwsTranscribeVocabularyConverter {
    service: Arc<TranscribeService>,
}

impl AwsTranscribeVocabularyConverter {
    pub fn new(service: Arc<TranscribeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsTranscribeVocabularyConverter {
    fn resource_type(&self) -> &str {
        "aws_transcribe_vocabulary"
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

impl AwsTranscribeVocabularyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: transcribe_gen::VocabularyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_transcribe_vocabulary", e))?;

        let attrs = &instance.attributes;
        let phrases: Option<Vec<String>> = attrs
            .get("phrases")
            .and_then(|v| serde_json::from_value(v.clone()).ok());
        let last_modified_time = attrs
            .get("last_modified_time")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let vocab_view = VocabularyView {
            vocabulary_name: model.vocabulary_name.clone(),
            language_code: model.language_code,
            vocabulary_state: "READY".to_string(),
            last_modified_time,
            phrases,
            vocabulary_file_uri: model.vocabulary_file_uri,
            failure_reason: model.failure_reason,
            download_uri: model.download_uri,
        };

        let mut state_view = TranscribeStateView::default();
        state_view
            .vocabularies
            .insert(vocab_view.vocabulary_name.clone(), vocab_view);
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
        for v in view.vocabularies.values() {
            let attrs = serde_json::json!({
                "id": v.vocabulary_name,
                "vocabulary_name": v.vocabulary_name,
                "language_code": v.language_code,
                "vocabulary_state": v.vocabulary_state,
                "last_modified_time": v.last_modified_time,
                "phrases": v.phrases,
                "vocabulary_file_uri": v.vocabulary_file_uri,
                "failure_reason": v.failure_reason,
                "download_uri": v.download_uri,
            });
            results.push(ExtractedResource {
                name: v.vocabulary_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_transcribe_language_model
// ---------------------------------------------------------------------------

/// Note: Transcribe language models are not directly represented in the
/// TranscribeStateView (no dedicated LanguageModel view type exists yet).
/// This converter provides a stub implementation that maps the Terraform
/// resource into the vocabulary state as a best-effort placeholder until
/// a proper LanguageModel type is added to the service crate.
///
/// For now we store language model metadata as a vocabulary entry with a
/// special naming convention so extraction can distinguish them.
pub struct AwsTranscribeLanguageModelConverter {
    service: Arc<TranscribeService>,
}

impl AwsTranscribeLanguageModelConverter {
    pub fn new(service: Arc<TranscribeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsTranscribeLanguageModelConverter {
    fn resource_type(&self) -> &str {
        "aws_transcribe_language_model"
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

impl AwsTranscribeLanguageModelConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: transcribe_gen::LanguageModelTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_transcribe_language_model", e))?;

        let attrs = &instance.attributes;
        let base_model_name = model
            .base_model_name
            .unwrap_or_else(|| "NarrowBand".to_string());

        // Extract input_data_config nested block (TF schema).
        let input_data_config = attrs
            .get("input_data_config")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first());
        let s3_uri = input_data_config
            .and_then(|cfg| cfg.get("s3_uri"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let data_access_role_arn = input_data_config
            .and_then(|cfg| cfg.get("data_access_role_arn"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let _tuning_data_s3_uri = input_data_config
            .and_then(|cfg| cfg.get("tuning_data_s3_uri"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let last_modified_time = attrs
            .get("last_modified_time")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        // Store as a vocabulary entry with a language-model prefix for identification.
        let key = format!("language-model:{}", model.model_name);

        let vocab_view = VocabularyView {
            vocabulary_name: key.clone(),
            language_code: model.language_code,
            vocabulary_state: format!("COMPLETED|{}", base_model_name),
            last_modified_time,
            phrases: None,
            vocabulary_file_uri: s3_uri,
            failure_reason: model.failure_reason,
            download_uri: data_access_role_arn,
        };

        let mut state_view = TranscribeStateView::default();
        state_view.vocabularies.insert(key, vocab_view);
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
        for v in view.vocabularies.values() {
            if let Some(model_name) = v.vocabulary_name.strip_prefix("language-model:") {
                // vocabulary_state stores "COMPLETED|base_model_name"
                let base_model_name = v
                    .vocabulary_state
                    .split('|')
                    .nth(1)
                    .unwrap_or("NarrowBand")
                    .to_string();
                let attrs = serde_json::json!({
                    "id": model_name,
                    "model_name": model_name,
                    "language_code": v.language_code,
                    "base_model_name": base_model_name,
                    "input_data_config": [{
                        "s3_uri": v.vocabulary_file_uri,
                        "data_access_role_arn": v.download_uri,
                        "tuning_data_s3_uri": serde_json::Value::Null,
                    }],
                    "tags": {},
                    "tags_all": {},
                    "arn": serde_json::Value::Null,
                });
                results.push(ExtractedResource {
                    name: model_name.to_string(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}
