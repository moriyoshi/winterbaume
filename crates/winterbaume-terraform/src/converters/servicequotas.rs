//! Terraform converter for Service Quotas resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_servicequotas::ServiceQuotasService;
use winterbaume_servicequotas::views::{ServiceQuotaEntryView, ServiceQuotasStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_bool, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_servicequotas_service_quota
// ---------------------------------------------------------------------------

/// Converts `aws_servicequotas_service_quota` Terraform resources to/from Service Quotas state.
pub struct AwsServicequotasServiceQuotaConverter {
    service: Arc<ServiceQuotasService>,
}

impl AwsServicequotasServiceQuotaConverter {
    pub fn new(service: Arc<ServiceQuotasService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsServicequotasServiceQuotaConverter {
    fn resource_type(&self) -> &str {
        "aws_servicequotas_service_quota"
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

impl AwsServicequotasServiceQuotaConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let service_code = require_str(attrs, "service_code", "aws_servicequotas_service_quota")?;
        let quota_code = require_str(attrs, "quota_code", "aws_servicequotas_service_quota")?;
        let region = extract_region(attrs, &ctx.default_region);
        let service_name =
            optional_str(attrs, "service_name").unwrap_or_else(|| service_code.to_string());
        let quota_name =
            optional_str(attrs, "quota_name").unwrap_or_else(|| quota_code.to_string());
        let quota_arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:servicequotas:{}:{}:{}/{}",
                region, ctx.default_account_id, service_code, quota_code
            )
        });
        let value = attrs.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let unit = optional_str(attrs, "unit").unwrap_or_else(|| "None".to_string());
        let adjustable = optional_bool(attrs, "adjustable").unwrap_or(false);
        let global_quota = optional_bool(attrs, "global_quota").unwrap_or(false);
        let description = optional_str(attrs, "description").unwrap_or_default();

        let quota_view = ServiceQuotaEntryView {
            service_code: service_code.to_string(),
            service_name,
            quota_code: quota_code.to_string(),
            quota_name,
            quota_arn,
            value,
            unit,
            adjustable,
            global_quota,
            description,
        };

        let key = format!("{service_code}/{quota_code}");
        let mut state_view = ServiceQuotasStateView {
            quotas: HashMap::new(),
            services: HashMap::new(),
        };
        state_view.quotas.insert(key, quota_view);
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
        for quota in view.quotas.values() {
            let id = format!("{}/{}", quota.service_code, quota.quota_code);
            let attrs = serde_json::json!({
                "id": id,
                "service_code": quota.service_code,
                "service_name": quota.service_name,
                "quota_code": quota.quota_code,
                "quota_name": quota.quota_name,
                "arn": quota.quota_arn,
                "value": quota.value,
                "unit": quota.unit,
                "adjustable": quota.adjustable,
                "global_quota": quota.global_quota,
                "description": quota.description,
            });
            results.push(ExtractedResource {
                name: id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
