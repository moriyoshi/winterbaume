//! Terraform converter for SWF resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_swf::SwfService;
use winterbaume_swf::views::{DomainView, SwfStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_swf_domain
// ---------------------------------------------------------------------------

/// Converts `aws_swf_domain` Terraform resources to/from SWF state.
pub struct AwsSwfDomainConverter {
    service: Arc<SwfService>,
}

impl AwsSwfDomainConverter {
    pub fn new(service: Arc<SwfService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSwfDomainConverter {
    fn resource_type(&self) -> &str {
        "aws_swf_domain"
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

impl AwsSwfDomainConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_swf_domain")?;
        let region = extract_region(attrs, &ctx.default_region);
        let description = optional_str(attrs, "description");
        let retention = optional_str(attrs, "workflow_execution_retention_period_in_days")
            .unwrap_or_else(|| "0".to_string());

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:swf:{}:{}:/domain/{}",
                region, ctx.default_account_id, name
            )
        });

        let _tags = extract_tags(attrs);

        let domain_view = DomainView {
            name: name.to_string(),
            description,
            status: "REGISTERED".to_string(),
            workflow_execution_retention_period_in_days: retention,
            arn,
        };

        let mut state_view = SwfStateView {
            domains: HashMap::new(),
            activity_types: HashMap::new(),
            workflow_types: HashMap::new(),
            executions: HashMap::new(),
            activity_tasks: HashMap::new(),
            signals: HashMap::new(),
        };
        state_view.domains.insert(name.to_string(), domain_view);
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
        for domain in view.domains.values() {
            let attrs = serde_json::json!({
                "id": domain.name,
                "name": domain.name,
                "description": domain.description,
                "arn": domain.arn,
                "workflow_execution_retention_period_in_days": domain.workflow_execution_retention_period_in_days,
            });
            results.push(ExtractedResource {
                name: domain.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
