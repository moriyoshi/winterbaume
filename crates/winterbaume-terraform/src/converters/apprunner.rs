//! Terraform converters for App Runner resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_apprunner::AppRunnerService;
use winterbaume_apprunner::views::{AppRunnerServiceView, AppRunnerStateView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::apprunner as apprunner_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_apprunner_service
// ---------------------------------------------------------------------------

pub struct AwsAppRunnerServiceConverter {
    service: Arc<AppRunnerService>,
}

impl AwsAppRunnerServiceConverter {
    pub fn new(service: Arc<AppRunnerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppRunnerServiceConverter {
    fn resource_type(&self) -> &str {
        "aws_apprunner_service"
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

impl AwsAppRunnerServiceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: apprunner_gen::AppRunnerServiceTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_apprunner_service", e))?;

        let service_name = model.service_name.clone();
        let service_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:apprunner:{}:{}:service/{}/placeholder",
                region, ctx.default_account_id, service_name
            )
        });
        let service_id = model
            .service_id
            .unwrap_or_else(|| "placeholder".to_string());
        let service_url = model
            .service_url
            .unwrap_or_else(|| format!("{service_name}.{region}.awsapprunner.com"));
        let status = model.status.unwrap_or_else(|| "RUNNING".to_string());

        // JSON-blob fields stay as raw attributes (not part of the strongly-typed model).
        let attrs = &instance.attributes;
        let encryption_configuration = attrs.get("encryption_configuration").cloned();
        let health_check_configuration = attrs.get("health_check_configuration").cloned();
        let instance_configuration = attrs.get("instance_configuration").cloned();
        let network_configuration = attrs.get("network_configuration").cloned();
        let observability_configuration = attrs.get("observability_configuration").cloned();
        let source_configuration = attrs.get("source_configuration").cloned();

        let svc_view = AppRunnerServiceView {
            service_id,
            service_name: service_name.clone(),
            service_arn,
            service_url,
            status,
            created_at: 0.0,
            updated_at: 0.0,
            tags: model.tags.into_iter().collect(),
            encryption_configuration,
            health_check_configuration,
            instance_configuration,
            network_configuration,
            observability_configuration,
            source_configuration,
        };

        let mut state_view = AppRunnerStateView::default();
        state_view.services.insert(service_name, svc_view);
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
        for svc in view.services.values() {
            let tags: HashMap<String, String> = svc.tags.iter().cloned().collect();
            let attrs = serde_json::json!({
                "id": svc.service_arn,
                "service_name": svc.service_name,
                "arn": svc.service_arn,
                "service_id": svc.service_id,
                "service_url": svc.service_url,
                "status": svc.status,
                "tags_all": tags,
                "encryption_configuration": svc.encryption_configuration,
                "health_check_configuration": svc.health_check_configuration,
                "instance_configuration": svc.instance_configuration,
                "network_configuration": svc.network_configuration,
                "observability_configuration": svc.observability_configuration,
                "source_configuration": svc.source_configuration,
            });
            results.push(ExtractedResource {
                name: svc.service_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
