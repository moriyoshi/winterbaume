//! Terraform converter for Comprehend resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_comprehend::ComprehendService;
use winterbaume_comprehend::views::{ComprehendStateView, EntityRecognizerView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_comprehend_entity_recognizer
// ---------------------------------------------------------------------------

/// Converts `aws_comprehend_entity_recognizer` Terraform resources to/from Comprehend state.
pub struct AwsComprehendEntityRecognizerConverter {
    service: Arc<ComprehendService>,
}

impl AwsComprehendEntityRecognizerConverter {
    pub fn new(service: Arc<ComprehendService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsComprehendEntityRecognizerConverter {
    fn resource_type(&self) -> &str {
        "aws_comprehend_entity_recognizer"
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

impl AwsComprehendEntityRecognizerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_comprehend_entity_recognizer")?;
        let region = extract_region(attrs, &ctx.default_region);
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:comprehend:{}:{}:entity-recognizer/{}",
                region, ctx.default_account_id, name
            )
        });
        let language_code =
            optional_str(attrs, "language_code").unwrap_or_else(|| "en".to_string());
        let data_access_role_arn = optional_str(attrs, "data_access_role_arn").unwrap_or_default();

        // Extract input_data_config S3 URI
        let input_data_config_s3_uri = attrs
            .get("input_data_config")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|obj| obj.get("entity_list"))
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|obj| obj.get("s3_uri"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        // Extract entity types
        let entity_types: Vec<String> = attrs
            .get("input_data_config")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|obj| obj.get("entity_types"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|et| {
                        et.get("type")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                    })
                    .collect()
            })
            .unwrap_or_default();

        let _version_name = optional_str(attrs, "version_name");
        let _model_kms_key_id = optional_str(attrs, "model_kms_key_id");
        let status = optional_str(attrs, "status").unwrap_or_else(|| "TRAINED".to_string());
        let mut tags = extract_tags(attrs);
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let recognizer_view = EntityRecognizerView {
            arn: arn.clone(),
            name: name.to_string(),
            language_code,
            data_access_role_arn,
            input_data_config_s3_uri,
            entity_types,
            status,
            submit_time: 0.0,
            tags,
        };

        let mut state_view = ComprehendStateView {
            document_classifiers: HashMap::new(),
            entity_recognizers: HashMap::new(),
            endpoints: HashMap::new(),
            flywheels: HashMap::new(),
            jobs: HashMap::new(),
            resource_policies: HashMap::new(),
            tags: HashMap::new(),
        };
        state_view.entity_recognizers.insert(arn, recognizer_view);
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
        for recognizer in view.entity_recognizers.values() {
            let entity_types_json: Vec<serde_json::Value> = recognizer
                .entity_types
                .iter()
                .map(|et| serde_json::json!({"type": et}))
                .collect();
            let attrs = serde_json::json!({
                "id": recognizer.arn,
                "arn": recognizer.arn,
                "name": recognizer.name,
                "language_code": recognizer.language_code,
                "data_access_role_arn": recognizer.data_access_role_arn,
                "input_data_config": [{
                    "entity_list": [{"s3_uri": recognizer.input_data_config_s3_uri}],
                    "entity_types": entity_types_json,
                }],
                "entity_types": recognizer.entity_types,
                "status": recognizer.status,
                "submit_time": recognizer.submit_time,
                "tags": recognizer.tags,
                "tags_all": recognizer.tags,
            });
            results.push(ExtractedResource {
                name: recognizer.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
