//! Terraform converter for SimpleDB resources.

use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_simpledbv2::SimpleDbV2Service;
use winterbaume_simpledbv2::views::SdbStateView;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, require_str};

// ---------------------------------------------------------------------------
// aws_simpledb_domain
// ---------------------------------------------------------------------------

/// Converts `aws_simpledb_domain` Terraform resources to/from SimpleDB state.
pub struct AwsSimpleDbDomainConverter {
    service: Arc<SimpleDbV2Service>,
}

impl AwsSimpleDbDomainConverter {
    pub fn new(service: Arc<SimpleDbV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSimpleDbDomainConverter {
    fn resource_type(&self) -> &str {
        "aws_simpledb_domain"
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

impl AwsSimpleDbDomainConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_simpledb_domain")?;
        let region = extract_region(attrs, &ctx.default_region);

        let mut domains = HashSet::new();
        domains.insert(name.to_string());

        let state_view = SdbStateView {
            exports: std::collections::HashMap::new(),
            domains,
            next_export_id: 0,
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
        for domain_name in &view.domains {
            let attrs = serde_json::json!({
                "id": domain_name,
                "name": domain_name,
            });
            results.push(ExtractedResource {
                name: domain_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
