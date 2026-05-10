//! Terraform converters for Service Catalog resources.
//!
//! `PortfolioTfModel` and `ProductTfModel` are generated from
//! `specs/servicecatalog.toml`. The synthesised `port-`/`prod-`
//! identifiers, the ARN templates, the `created_time` fallback to
//! `Utc::now()`, and the conversion from a `tags` HashMap into the
//! view's `Vec<PortfolioTagView>` are wired up here.

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
use crate::generated::servicecatalog as servicecatalog_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: servicecatalog_gen::PortfolioTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_servicecatalog_portfolio", e))?;

        let id = model.id.unwrap_or_else(|| {
            format!(
                "port-{}",
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..12]
            )
        });
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:catalog:{}:{}:portfolio/{}",
                region, ctx.default_account_id, id
            )
        });
        let description = model.description.unwrap_or_default();
        let provider_name = model.provider_name.unwrap_or_default();
        let created_time = model
            .created_time
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(chrono::Utc::now);

        let tags: Vec<PortfolioTagView> = model
            .tags
            .into_iter()
            .map(|(k, v)| PortfolioTagView { key: k, value: v })
            .collect();

        let portfolio_view = PortfolioDetailView {
            id: id.clone(),
            arn,
            display_name: model.name,
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: servicecatalog_gen::ProductTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_servicecatalog_product", e))?;

        let product_id = model.id.unwrap_or_else(|| {
            format!(
                "prod-{}",
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..12]
            )
        });
        let product_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:catalog:{}:{}:product/{}",
                region, ctx.default_account_id, product_id
            )
        });
        let owner = model.owner.unwrap_or_default();
        let product_type = model
            .product_type
            .unwrap_or_else(|| "CLOUD_FORMATION_TEMPLATE".to_string());
        let created_time = model
            .created_time
            .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

        let tags: Vec<PortfolioTagView> = model
            .tags
            .into_iter()
            .map(|(k, v)| PortfolioTagView { key: k, value: v })
            .collect();

        let product_view = ProductView {
            product_id: product_id.clone(),
            product_arn,
            name: model.name,
            owner,
            description: model.description,
            product_type,
            distributor: model.distributor,
            support_description: model.support_description,
            support_email: model.support_email,
            support_url: model.support_url,
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
