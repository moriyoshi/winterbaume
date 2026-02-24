//! Terraform converter for SSM resources.

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
use crate::util::{extract_region, extract_tags, optional_i64, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_ssm_parameter")?;
        let _tags_all = attrs.get("tags_all");
        let _allowed_pattern = optional_str(attrs, "allowed_pattern");
        let _overwrite = attrs.get("overwrite");
        let _ = attrs.get("insecure_value");
        let param_type = optional_str(attrs, "type").unwrap_or_else(|| "String".to_string());
        let value = optional_str(attrs, "value").unwrap_or_default();
        let region = extract_region(attrs, &ctx.default_region);
        let version = optional_i64(attrs, "version").unwrap_or(1);
        let data_type = optional_str(attrs, "data_type").unwrap_or_else(|| "text".to_string());

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:ssm:{}:{}:parameter{}",
                region, ctx.default_account_id, name
            )
        });

        let param_view = ParameterView {
            name: name.to_string(),
            r#type: param_type.clone(),
            value: value.clone(),
            version,
            last_modified_date: None,
            arn,
            data_type,
            tags: extract_tags(attrs),
        };

        let param_version_view = ParameterVersionView {
            name: name.to_string(),
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
        state_view.parameters.insert(name.to_string(), param_view);
        state_view
            .parameter_versions
            .insert(name.to_string(), vec![param_version_view]);
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
