//! Terraform converter for Service Catalog App Registry resources.
//!
//! `ApplicationTfModel` is generated from `specs/servicecatalogappregistry.toml`.
//! The id synthesis (UUID when missing), the ARN template, and the
//! `creation_time`/`last_update_time` "now" timestamps are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_servicecatalogappregistry::ServiceCatalogAppRegistryService;
use winterbaume_servicecatalogappregistry::views::{
    AppRegistryConfigView, ApplicationView, ServiceCatalogAppRegistryStateView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::servicecatalogappregistry as servicecatalogappregistry_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_servicecatalogappregistry_application
// ---------------------------------------------------------------------------

/// Converts `aws_servicecatalogappregistry_application` Terraform resources to/from state.
pub struct AwsServicecatalogappregistryApplicationConverter {
    service: Arc<ServiceCatalogAppRegistryService>,
}

impl AwsServicecatalogappregistryApplicationConverter {
    pub fn new(service: Arc<ServiceCatalogAppRegistryService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsServicecatalogappregistryApplicationConverter {
    fn resource_type(&self) -> &str {
        "aws_servicecatalogappregistry_application"
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

impl AwsServicecatalogappregistryApplicationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: servicecatalogappregistry_gen::ApplicationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_servicecatalogappregistry_application", e)
            })?;

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:servicecatalog:{}:{}:/applications/{}",
                region, ctx.default_account_id, id
            )
        });
        let now = chrono::Utc::now();

        let app_view = ApplicationView {
            id: id.clone(),
            arn,
            name: model.name.clone(),
            description: model.description,
            creation_time: now,
            last_update_time: now,
            tags: model.tags,
        };

        let mut state_view = ServiceCatalogAppRegistryStateView {
            applications: HashMap::new(),
            application_names: HashMap::new(),
            attribute_groups: HashMap::new(),
            attribute_group_names: HashMap::new(),
            app_to_attr_groups: HashMap::new(),
            app_to_resources: HashMap::new(),
            configuration: AppRegistryConfigView { tag_key: None },
        };
        state_view.application_names.insert(model.name, id.clone());
        state_view.applications.insert(id, app_view);
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
        for app in view.applications.values() {
            let attrs = serde_json::json!({
                "id": app.id,
                "arn": app.arn,
                "name": app.name,
                "description": app.description,
                "creation_time": app.creation_time.to_rfc3339(),
                "last_update_time": app.last_update_time.to_rfc3339(),
                "tags": app.tags,
            });
            results.push(ExtractedResource {
                name: app.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
