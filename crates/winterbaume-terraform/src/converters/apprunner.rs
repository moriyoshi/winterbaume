//! Terraform converters for App Runner resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_apprunner::AppRunnerService;
use winterbaume_apprunner::views::{
    AppRunnerServiceView, AppRunnerStateView, AutoScalingConfigView, ConnectionView,
};
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

// ---------------------------------------------------------------------------
// aws_apprunner_connection
// ---------------------------------------------------------------------------

/// Converts `aws_apprunner_connection` Terraform resources to/from App Runner state.
pub struct AwsAppRunnerConnectionConverter {
    service: Arc<AppRunnerService>,
}

impl AwsAppRunnerConnectionConverter {
    pub fn new(service: Arc<AppRunnerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppRunnerConnectionConverter {
    fn resource_type(&self) -> &str {
        "aws_apprunner_connection"
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

impl AwsAppRunnerConnectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: apprunner_gen::AppRunnerConnectionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_apprunner_connection", e))?;

        let connection_name = model.connection_name.clone();
        let connection_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:apprunner:{}:{}:connection/{}",
                region, ctx.default_account_id, connection_name
            )
        });

        let conn_view = ConnectionView {
            connection_name: connection_name.clone(),
            connection_arn,
            provider_type: model.provider_type,
            status: model
                .status
                .unwrap_or_else(|| "PENDING_HANDSHAKE".to_string()),
            created_at: 0.0,
            tags: model.tags.into_iter().collect(),
        };

        let mut state_view = AppRunnerStateView::default();
        state_view.connections.insert(connection_name, conn_view);
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
        for conn in view.connections.values() {
            let tags: HashMap<String, String> = conn.tags.iter().cloned().collect();
            let attrs = serde_json::json!({
                "id": conn.connection_arn,
                "arn": conn.connection_arn,
                "connection_name": conn.connection_name,
                "provider_type": conn.provider_type,
                "status": conn.status,
                "tags": tags,
                "tags_all": tags,
            });
            results.push(ExtractedResource {
                name: conn.connection_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_apprunner_auto_scaling_configuration_version
// ---------------------------------------------------------------------------

/// Converts `aws_apprunner_auto_scaling_configuration_version` Terraform resources
/// to/from App Runner state.
pub struct AwsAppRunnerAutoScalingConfigurationVersionConverter {
    service: Arc<AppRunnerService>,
}

impl AwsAppRunnerAutoScalingConfigurationVersionConverter {
    pub fn new(service: Arc<AppRunnerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppRunnerAutoScalingConfigurationVersionConverter {
    fn resource_type(&self) -> &str {
        "aws_apprunner_auto_scaling_configuration_version"
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

impl AwsAppRunnerAutoScalingConfigurationVersionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: apprunner_gen::AppRunnerAutoScalingConfigurationVersionTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_apprunner_auto_scaling_configuration_version", e)
            })?;

        let name = model.auto_scaling_configuration_name.clone();
        let revision = if model.auto_scaling_configuration_revision > 0 {
            model.auto_scaling_configuration_revision as i32
        } else {
            1
        };
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:apprunner:{}:{}:autoscalingconfiguration/{}/{}",
                region, ctx.default_account_id, name, revision
            )
        });

        let min_size = if model.min_size > 0 {
            model.min_size as i32
        } else {
            1
        };
        let max_size = if model.max_size > 0 {
            model.max_size as i32
        } else {
            25
        };
        let max_concurrency = if model.max_concurrency > 0 {
            model.max_concurrency as i32
        } else {
            100
        };

        let cfg_view = AutoScalingConfigView {
            name: name.clone(),
            arn: arn.clone(),
            revision,
            status: model.status.unwrap_or_else(|| "ACTIVE".to_string()),
            is_default: false,
            latest: model.latest,
            min_size,
            max_size,
            max_concurrency,
            created_at: 0.0,
            tags: model.tags.into_iter().collect(),
        };

        let mut state_view = AppRunnerStateView::default();
        state_view.auto_scaling_configs.insert(arn, cfg_view);
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
        for cfg in view.auto_scaling_configs.values() {
            let tags: HashMap<String, String> = cfg.tags.iter().cloned().collect();
            let attrs = serde_json::json!({
                "id": cfg.arn,
                "arn": cfg.arn,
                "auto_scaling_configuration_name": cfg.name,
                "auto_scaling_configuration_revision": cfg.revision,
                "status": cfg.status,
                "latest": cfg.latest,
                "min_size": cfg.min_size,
                "max_size": cfg.max_size,
                "max_concurrency": cfg.max_concurrency,
                "tags": tags,
                "tags_all": tags,
            });
            results.push(ExtractedResource {
                name: cfg.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_apprunner_default_auto_scaling_configuration_version
// ---------------------------------------------------------------------------

/// Converts `aws_apprunner_default_auto_scaling_configuration_version` Terraform
/// resources to/from App Runner state.
///
/// On inject, marks the referenced auto-scaling configuration as the account
/// default by setting `is_default = true` and clearing the flag on all others.
pub struct AwsAppRunnerDefaultAutoScalingConfigurationVersionConverter {
    service: Arc<AppRunnerService>,
}

impl AwsAppRunnerDefaultAutoScalingConfigurationVersionConverter {
    pub fn new(service: Arc<AppRunnerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppRunnerDefaultAutoScalingConfigurationVersionConverter {
    fn resource_type(&self) -> &str {
        "aws_apprunner_default_auto_scaling_configuration_version"
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

impl AwsAppRunnerDefaultAutoScalingConfigurationVersionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: apprunner_gen::AppRunnerDefaultAutoScalingConfigurationVersionTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error(
                    "aws_apprunner_default_auto_scaling_configuration_version",
                    e,
                )
            })?;

        let target_arn = model.auto_scaling_configuration_arn.clone();

        // Snapshot the current state, flip is_default on every config (matching by arn or name),
        // and merge the rewritten map back in.
        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let configs = std::mem::take(&mut snapshot.auto_scaling_configs);

        let mut warnings = vec![];
        let mut matched = false;
        let updated: HashMap<String, AutoScalingConfigView> = configs
            .into_iter()
            .map(|(k, cfg)| {
                let is_target = cfg.arn == target_arn || cfg.name == target_arn;
                if is_target {
                    matched = true;
                }
                (
                    k,
                    AutoScalingConfigView {
                        is_default: is_target,
                        ..cfg
                    },
                )
            })
            .collect();
        if !matched {
            warnings.push(format!(
                "aws_apprunner_default_auto_scaling_configuration_version: \
                 auto_scaling_configuration_arn {target_arn} not found in state; \
                 default flag not applied"
            ));
        }

        // restore (not merge) so cleared is_default flags propagate.
        let state_view = AppRunnerStateView {
            auto_scaling_configs: updated,
            ..snapshot
        };
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
        for cfg in view.auto_scaling_configs.values().filter(|c| c.is_default) {
            let attrs = serde_json::json!({
                "id": cfg.arn,
                "auto_scaling_configuration_arn": cfg.arn,
            });
            results.push(ExtractedResource {
                name: cfg.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_apprunner_custom_domain_association — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_apprunner_custom_domain_association` Terraform resources
/// (validation-only; no backing state slot in `winterbaume_apprunner`).
pub struct AwsAppRunnerCustomDomainAssociationConverter {
    #[allow(dead_code)]
    service: Arc<AppRunnerService>,
}

impl AwsAppRunnerCustomDomainAssociationConverter {
    pub fn new(service: Arc<AppRunnerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppRunnerCustomDomainAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_apprunner_custom_domain_association"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsAppRunnerCustomDomainAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: apprunner_gen::AppRunnerCustomDomainAssociationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_apprunner_custom_domain_association", e)
            })?;
        let warn_msg = "no state slot in winterbaume_apprunner for custom domain associations; \
             inject is a no-op"
            .to_string();
        eprintln!("warning: aws_apprunner_custom_domain_association: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!(
                "aws_apprunner_custom_domain_association: {warn_msg}"
            )],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_apprunner_deployment — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_apprunner_deployment` Terraform resources (validation-only;
/// deployments are one-shot operations with no persistent backing state).
pub struct AwsAppRunnerDeploymentConverter {
    #[allow(dead_code)]
    service: Arc<AppRunnerService>,
}

impl AwsAppRunnerDeploymentConverter {
    pub fn new(service: Arc<AppRunnerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppRunnerDeploymentConverter {
    fn resource_type(&self) -> &str {
        "aws_apprunner_deployment"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsAppRunnerDeploymentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: apprunner_gen::AppRunnerDeploymentTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_apprunner_deployment", e))?;
        let warn_msg =
            "no state slot in winterbaume_apprunner for deployments; inject is a no-op".to_string();
        eprintln!("warning: aws_apprunner_deployment: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!("aws_apprunner_deployment: {warn_msg}")],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_apprunner_observability_configuration — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_apprunner_observability_configuration` Terraform resources
/// (validation-only; no backing state slot in `winterbaume_apprunner`).
pub struct AwsAppRunnerObservabilityConfigurationConverter {
    #[allow(dead_code)]
    service: Arc<AppRunnerService>,
}

impl AwsAppRunnerObservabilityConfigurationConverter {
    pub fn new(service: Arc<AppRunnerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppRunnerObservabilityConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_apprunner_observability_configuration"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsAppRunnerObservabilityConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: apprunner_gen::AppRunnerObservabilityConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_apprunner_observability_configuration", e)
            })?;
        let warn_msg = "no state slot in winterbaume_apprunner for observability \
             configurations; inject is a no-op"
            .to_string();
        eprintln!("warning: aws_apprunner_observability_configuration: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!(
                "aws_apprunner_observability_configuration: {warn_msg}"
            )],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_apprunner_vpc_connector — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_apprunner_vpc_connector` Terraform resources (validation-only;
/// no backing state slot in `winterbaume_apprunner`).
pub struct AwsAppRunnerVpcConnectorConverter {
    #[allow(dead_code)]
    service: Arc<AppRunnerService>,
}

impl AwsAppRunnerVpcConnectorConverter {
    pub fn new(service: Arc<AppRunnerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppRunnerVpcConnectorConverter {
    fn resource_type(&self) -> &str {
        "aws_apprunner_vpc_connector"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsAppRunnerVpcConnectorConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: apprunner_gen::AppRunnerVpcConnectorTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_apprunner_vpc_connector", e))?;
        let warn_msg =
            "no state slot in winterbaume_apprunner for VPC connectors; inject is a no-op"
                .to_string();
        eprintln!("warning: aws_apprunner_vpc_connector: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!("aws_apprunner_vpc_connector: {warn_msg}")],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_apprunner_vpc_ingress_connection — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_apprunner_vpc_ingress_connection` Terraform resources
/// (validation-only; no backing state slot in `winterbaume_apprunner`).
pub struct AwsAppRunnerVpcIngressConnectionConverter {
    #[allow(dead_code)]
    service: Arc<AppRunnerService>,
}

impl AwsAppRunnerVpcIngressConnectionConverter {
    pub fn new(service: Arc<AppRunnerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppRunnerVpcIngressConnectionConverter {
    fn resource_type(&self) -> &str {
        "aws_apprunner_vpc_ingress_connection"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsAppRunnerVpcIngressConnectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: apprunner_gen::AppRunnerVpcIngressConnectionTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_apprunner_vpc_ingress_connection", e)
            })?;
        let warn_msg = "no state slot in winterbaume_apprunner for VPC ingress connections; \
             inject is a no-op"
            .to_string();
        eprintln!("warning: aws_apprunner_vpc_ingress_connection: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!("aws_apprunner_vpc_ingress_connection: {warn_msg}")],
        })
    }
}
