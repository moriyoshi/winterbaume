//! Terraform converter for SSM resources.
//!
//! `ParameterTfModel` is generated from `specs/ssm.toml`. The ARN
//! template, the `type` / `value` / `data_type` defaults, the synthesised
//! initial parameter version, and the constant `tier = "Standard"`
//! extract value are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_ssm::SsmService;
use winterbaume_ssm::views::{ParameterVersionView, ParameterView, SsmStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::ssm as ssm_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_ssm_parameter
// ---------------------------------------------------------------------------

/// Converts `aws_ssm_parameter` Terraform resources to/from SSM state.
pub struct AwsSsmParameterConverter {
    service: Arc<SsmService>,
}

impl AwsSsmParameterConverter {
    pub fn new(service: Arc<SsmService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsmParameterConverter {
    fn resource_type(&self) -> &str {
        "aws_ssm_parameter"
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

impl AwsSsmParameterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssm_gen::ParameterTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_ssm_parameter", e))?;

        let name = model.name.clone();
        let param_type = model.r#type.unwrap_or_else(|| "String".to_string());
        let value = model.value.unwrap_or_default();
        let version = model.version;
        let data_type = model.data_type.unwrap_or_else(|| "text".to_string());

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ssm:{}:{}:parameter{}",
                region, ctx.default_account_id, name
            )
        });

        let param_view = ParameterView {
            name: name.clone(),
            r#type: param_type.clone(),
            value: value.clone(),
            version,
            last_modified_date: None,
            arn,
            data_type,
            tags: model.tags,
        };

        let param_version_view = ParameterVersionView {
            name: name.clone(),
            r#type: param_type,
            value,
            version,
            last_modified_date: None,
            labels: vec![],
        };

        let mut state_view = SsmStateView {
            parameters: HashMap::new(),
            parameter_versions: HashMap::new(),
            documents: HashMap::new(),
            maintenance_windows: HashMap::new(),
            patch_baselines: HashMap::new(),
            default_patch_baselines: HashMap::new(),
            commands: HashMap::new(),
            associations: HashMap::new(),
            resource_tags: HashMap::new(),
            ops_items: HashMap::new(),
            sessions: HashMap::new(),
            activations: HashMap::new(),
            resource_policies: HashMap::new(),
            service_settings: HashMap::new(),
            inventory: HashMap::new(),
            compliance_records: HashMap::new(),
            ops_metadata: HashMap::new(),
            resource_data_syncs: HashMap::new(),
            ..Default::default()
        };
        state_view.parameters.insert(name.clone(), param_view);
        state_view
            .parameter_versions
            .insert(name, vec![param_version_view]);
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
        for param in view.parameters.values() {
            let attrs = serde_json::json!({
                "id": param.name,
                "name": param.name,
                "type": param.r#type,
                "value": param.value,
                "arn": param.arn,
                "version": param.version,
                "last_modified_date": param.last_modified_date,
                "data_type": param.data_type,
                "tags": param.tags,
                "tags_all": param.tags,
                "tier": "Standard",
            });
            results.push(ExtractedResource {
                name: param.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
