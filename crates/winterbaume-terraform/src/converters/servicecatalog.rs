//! Terraform converter for Service Catalog resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_servicecatalog::ServiceCatalogService;
use winterbaume_servicecatalog::views::{
    PortfolioDetailView, PortfolioTagView, ProductView, ServiceCatalogStateView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_servicecatalog_portfolio
// ---------------------------------------------------------------------------

/// Converts `aws_servicecatalog_portfolio` Terraform resources to/from Service Catalog state.
pub struct AwsServicecatalogPortfolioConverter {
    service: Arc<ServiceCatalogService>,
}

impl AwsServicecatalogPortfolioConverter {
    pub fn new(service: Arc<ServiceCatalogService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsServicecatalogPortfolioConverter {
    fn resource_type(&self) -> &str {
        "aws_servicecatalog_portfolio"
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

impl AwsServicecatalogPortfolioConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_servicecatalog_portfolio")?;
        let region = extract_region(attrs, &ctx.default_region);
        let id = optional_str(attrs, "id").unwrap_or_else(|| {
            format!(
                "port-{}",
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..12]
            )
        });
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:catalog:{}:{}:portfolio/{}",
                region, ctx.default_account_id, id
            )
        });
        let description = optional_str(attrs, "description").unwrap_or_default();
        let provider_name = optional_str(attrs, "provider_name").unwrap_or_default();
        let created_time = optional_str(attrs, "created_time")
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(chrono::Utc::now);

        let tags_map = extract_tags(attrs);
        let tags: Vec<PortfolioTagView> = tags_map
            .into_iter()
            .map(|(k, v)| PortfolioTagView { key: k, value: v })
            .collect();

        let portfolio_view = PortfolioDetailView {
            id: id.clone(),
            arn,
            display_name: name.to_string(),
            description,
            created_time,
            provider_name,
            tags,
        };

        let mut state_view = ServiceCatalogStateView::default();
        state_view.portfolios.insert(id, portfolio_view);
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
        for portfolio in view.portfolios.values() {
            let tags_map: serde_json::Map<String, serde_json::Value> = portfolio
                .tags
                .iter()
                .map(|t| (t.key.clone(), serde_json::Value::String(t.value.clone())))
                .collect();
            let attrs = serde_json::json!({
                "id": portfolio.id,
                "arn": portfolio.arn,
                "name": portfolio.display_name,
                "description": portfolio.description,
                "provider_name": portfolio.provider_name,
                "created_time": portfolio.created_time.to_rfc3339(),
                "tags": tags_map,
            });
            results.push(ExtractedResource {
                name: portfolio.display_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_servicecatalog_product
// ---------------------------------------------------------------------------

/// Converts `aws_servicecatalog_product` Terraform resources to/from Service Catalog state.
pub struct AwsServicecatalogProductConverter {
    service: Arc<ServiceCatalogService>,
}

impl AwsServicecatalogProductConverter {
    pub fn new(service: Arc<ServiceCatalogService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsServicecatalogProductConverter {
    fn resource_type(&self) -> &str {
        "aws_servicecatalog_product"
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

impl AwsServicecatalogProductConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_servicecatalog_product")?;
        let region = extract_region(attrs, &ctx.default_region);

        let product_id = optional_str(attrs, "id").unwrap_or_else(|| {
            format!(
                "prod-{}",
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..12]
            )
        });
        let product_arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:catalog:{}:{}:product/{}",
                region, ctx.default_account_id, product_id
            )
        });
        let owner = optional_str(attrs, "owner").unwrap_or_default();
        let description = optional_str(attrs, "description");
        let product_type =
            optional_str(attrs, "type").unwrap_or_else(|| "CLOUD_FORMATION_TEMPLATE".to_string());
        let distributor = optional_str(attrs, "distributor");
        let support_description = optional_str(attrs, "support_description");
        let support_email = optional_str(attrs, "support_email");
        let support_url = optional_str(attrs, "support_url");
        let created_time =
            optional_str(attrs, "created_time").unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

        let tags_map = extract_tags(attrs);
        let tags: Vec<PortfolioTagView> = tags_map
            .into_iter()
            .map(|(k, v)| PortfolioTagView { key: k, value: v })
            .collect();

        let product_view = ProductView {
            product_id: product_id.clone(),
            product_arn,
            name: name.to_string(),
            owner,
            description,
            product_type,
            distributor,
            support_description,
            support_email,
            support_url,
            created_time,
            tags,
        };

        let mut state_view = ServiceCatalogStateView::default();
        state_view.products.insert(product_id, product_view);
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
        for product in view.products.values() {
            let tags_map: serde_json::Map<String, serde_json::Value> = product
                .tags
                .iter()
                .map(|t| (t.key.clone(), serde_json::Value::String(t.value.clone())))
                .collect();
            let attrs = serde_json::json!({
                "id": product.product_id,
                "arn": product.product_arn,
                "name": product.name,
                "owner": product.owner,
                "description": product.description,
                "type": product.product_type,
                "distributor": product.distributor,
                "support_description": product.support_description,
                "support_email": product.support_email,
                "support_url": product.support_url,
                "created_time": product.created_time,
                "tags": tags_map,
            });
            results.push(ExtractedResource {
                name: product.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
