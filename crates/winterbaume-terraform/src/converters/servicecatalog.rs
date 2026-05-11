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

// ---------------------------------------------------------------------------
// aws_servicecatalog_budget_resource_association
// ---------------------------------------------------------------------------
// Warning-only: `ServiceCatalogStateView` does not model budget-resource
// associations. The model is parsed for schema validation only.

pub struct AwsServicecatalogBudgetResourceAssociationConverter;

impl AwsServicecatalogBudgetResourceAssociationConverter {
    pub fn new(_service: Arc<ServiceCatalogService>) -> Self {
        Self
    }
}

impl TerraformResourceConverter for AwsServicecatalogBudgetResourceAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_servicecatalog_budget_resource_association"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: servicecatalog_gen::BudgetResourceAssociationTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_servicecatalog_budget_resource_association", e)
                })?;
            let msg = "aws_servicecatalog_budget_resource_association: budget-resource associations are not modelled in ServiceCatalogStateView; resource skipped";
            eprintln!("warning: {}", msg);
            Ok(ConversionResult {
                region,
                warnings: vec![msg.to_string()],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_servicecatalog_constraint
// ---------------------------------------------------------------------------
// Warning-only: `ServiceCatalogStateView` does not model constraints.

pub struct AwsServicecatalogConstraintConverter;

impl AwsServicecatalogConstraintConverter {
    pub fn new(_service: Arc<ServiceCatalogService>) -> Self {
        Self
    }
}

impl TerraformResourceConverter for AwsServicecatalogConstraintConverter {
    fn resource_type(&self) -> &str {
        "aws_servicecatalog_constraint"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: servicecatalog_gen::ConstraintTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_servicecatalog_constraint", e))?;
            let msg = "aws_servicecatalog_constraint: constraints are not modelled in ServiceCatalogStateView; resource skipped";
            eprintln!("warning: {}", msg);
            Ok(ConversionResult {
                region,
                warnings: vec![msg.to_string()],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_servicecatalog_organizations_access
// ---------------------------------------------------------------------------
// Warning-only: `ServiceCatalogStateView` does not model the AWS Organizations
// access toggle.

pub struct AwsServicecatalogOrganizationsAccessConverter;

impl AwsServicecatalogOrganizationsAccessConverter {
    pub fn new(_service: Arc<ServiceCatalogService>) -> Self {
        Self
    }
}

impl TerraformResourceConverter for AwsServicecatalogOrganizationsAccessConverter {
    fn resource_type(&self) -> &str {
        "aws_servicecatalog_organizations_access"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: servicecatalog_gen::OrganizationsAccessTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_servicecatalog_organizations_access", e)
                })?;
            let msg = "aws_servicecatalog_organizations_access: organizations access state is not modelled in ServiceCatalogStateView; resource skipped";
            eprintln!("warning: {}", msg);
            Ok(ConversionResult {
                region,
                warnings: vec![msg.to_string()],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_servicecatalog_portfolio_share
// ---------------------------------------------------------------------------
// Warning-only: `ServiceCatalogStateView` does not model portfolio shares.

pub struct AwsServicecatalogPortfolioShareConverter;

impl AwsServicecatalogPortfolioShareConverter {
    pub fn new(_service: Arc<ServiceCatalogService>) -> Self {
        Self
    }
}

impl TerraformResourceConverter for AwsServicecatalogPortfolioShareConverter {
    fn resource_type(&self) -> &str {
        "aws_servicecatalog_portfolio_share"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_servicecatalog_portfolio"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: servicecatalog_gen::PortfolioShareTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_servicecatalog_portfolio_share", e)
                })?;
            let msg = "aws_servicecatalog_portfolio_share: portfolio shares are not modelled in ServiceCatalogStateView; resource skipped";
            eprintln!("warning: {}", msg);
            Ok(ConversionResult {
                region,
                warnings: vec![msg.to_string()],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_servicecatalog_principal_portfolio_association
// ---------------------------------------------------------------------------
// Warning-only: `ServiceCatalogStateView` does not model principal-portfolio
// associations.

pub struct AwsServicecatalogPrincipalPortfolioAssociationConverter;

impl AwsServicecatalogPrincipalPortfolioAssociationConverter {
    pub fn new(_service: Arc<ServiceCatalogService>) -> Self {
        Self
    }
}

impl TerraformResourceConverter for AwsServicecatalogPrincipalPortfolioAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_servicecatalog_principal_portfolio_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_servicecatalog_portfolio"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: servicecatalog_gen::PrincipalPortfolioAssociationTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error(
                        "aws_servicecatalog_principal_portfolio_association",
                        e,
                    )
                })?;
            let msg = "aws_servicecatalog_principal_portfolio_association: principal-portfolio associations are not modelled in ServiceCatalogStateView; resource skipped";
            eprintln!("warning: {}", msg);
            Ok(ConversionResult {
                region,
                warnings: vec![msg.to_string()],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_servicecatalog_product_portfolio_association
// ---------------------------------------------------------------------------
// Warning-only: `ServiceCatalogStateView` does not model product-portfolio
// associations.

pub struct AwsServicecatalogProductPortfolioAssociationConverter;

impl AwsServicecatalogProductPortfolioAssociationConverter {
    pub fn new(_service: Arc<ServiceCatalogService>) -> Self {
        Self
    }
}

impl TerraformResourceConverter for AwsServicecatalogProductPortfolioAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_servicecatalog_product_portfolio_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_servicecatalog_portfolio", "aws_servicecatalog_product"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: servicecatalog_gen::ProductPortfolioAssociationTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error(
                        "aws_servicecatalog_product_portfolio_association",
                        e,
                    )
                })?;
            let msg = "aws_servicecatalog_product_portfolio_association: product-portfolio associations are not modelled in ServiceCatalogStateView; resource skipped";
            eprintln!("warning: {}", msg);
            Ok(ConversionResult {
                region,
                warnings: vec![msg.to_string()],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_servicecatalog_provisioned_product
// ---------------------------------------------------------------------------
// Warning-only: `ServiceCatalogStateView` does not model provisioned products.

pub struct AwsServicecatalogProvisionedProductConverter;

impl AwsServicecatalogProvisionedProductConverter {
    pub fn new(_service: Arc<ServiceCatalogService>) -> Self {
        Self
    }
}

impl TerraformResourceConverter for AwsServicecatalogProvisionedProductConverter {
    fn resource_type(&self) -> &str {
        "aws_servicecatalog_provisioned_product"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_servicecatalog_product"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: servicecatalog_gen::ProvisionedProductTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_servicecatalog_provisioned_product", e)
                })?;
            let msg = "aws_servicecatalog_provisioned_product: provisioned products are not modelled in ServiceCatalogStateView; resource skipped";
            eprintln!("warning: {}", msg);
            Ok(ConversionResult {
                region,
                warnings: vec![msg.to_string()],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_servicecatalog_provisioning_artifact
// ---------------------------------------------------------------------------
// Warning-only: `ServiceCatalogStateView` does not model provisioning
// artifacts (product versions).

pub struct AwsServicecatalogProvisioningArtifactConverter;

impl AwsServicecatalogProvisioningArtifactConverter {
    pub fn new(_service: Arc<ServiceCatalogService>) -> Self {
        Self
    }
}

impl TerraformResourceConverter for AwsServicecatalogProvisioningArtifactConverter {
    fn resource_type(&self) -> &str {
        "aws_servicecatalog_provisioning_artifact"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_servicecatalog_product"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: servicecatalog_gen::ProvisioningArtifactTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_servicecatalog_provisioning_artifact", e)
                })?;
            let msg = "aws_servicecatalog_provisioning_artifact: provisioning artifacts are not modelled in ServiceCatalogStateView; resource skipped";
            eprintln!("warning: {}", msg);
            Ok(ConversionResult {
                region,
                warnings: vec![msg.to_string()],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_servicecatalog_service_action
// ---------------------------------------------------------------------------
// Warning-only: `ServiceCatalogStateView` does not model self-service actions.

pub struct AwsServicecatalogServiceActionConverter;

impl AwsServicecatalogServiceActionConverter {
    pub fn new(_service: Arc<ServiceCatalogService>) -> Self {
        Self
    }
}

impl TerraformResourceConverter for AwsServicecatalogServiceActionConverter {
    fn resource_type(&self) -> &str {
        "aws_servicecatalog_service_action"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: servicecatalog_gen::ServiceActionTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_servicecatalog_service_action", e)
                })?;
            let msg = "aws_servicecatalog_service_action: service actions are not modelled in ServiceCatalogStateView; resource skipped";
            eprintln!("warning: {}", msg);
            Ok(ConversionResult {
                region,
                warnings: vec![msg.to_string()],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_servicecatalog_tag_option
// ---------------------------------------------------------------------------
// Warning-only: `ServiceCatalogStateView` does not model tag options.

pub struct AwsServicecatalogTagOptionConverter;

impl AwsServicecatalogTagOptionConverter {
    pub fn new(_service: Arc<ServiceCatalogService>) -> Self {
        Self
    }
}

impl TerraformResourceConverter for AwsServicecatalogTagOptionConverter {
    fn resource_type(&self) -> &str {
        "aws_servicecatalog_tag_option"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: servicecatalog_gen::TagOptionTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_servicecatalog_tag_option", e))?;
            let msg = "aws_servicecatalog_tag_option: tag options are not modelled in ServiceCatalogStateView; resource skipped";
            eprintln!("warning: {}", msg);
            Ok(ConversionResult {
                region,
                warnings: vec![msg.to_string()],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_servicecatalog_tag_option_resource_association
// ---------------------------------------------------------------------------
// Warning-only: `ServiceCatalogStateView` does not model tag-option resource
// associations.

pub struct AwsServicecatalogTagOptionResourceAssociationConverter;

impl AwsServicecatalogTagOptionResourceAssociationConverter {
    pub fn new(_service: Arc<ServiceCatalogService>) -> Self {
        Self
    }
}

impl TerraformResourceConverter for AwsServicecatalogTagOptionResourceAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_servicecatalog_tag_option_resource_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_servicecatalog_tag_option"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: servicecatalog_gen::TagOptionResourceAssociationTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error(
                        "aws_servicecatalog_tag_option_resource_association",
                        e,
                    )
                })?;
            let msg = "aws_servicecatalog_tag_option_resource_association: tag-option resource associations are not modelled in ServiceCatalogStateView; resource skipped";
            eprintln!("warning: {}", msg);
            Ok(ConversionResult {
                region,
                warnings: vec![msg.to_string()],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}
