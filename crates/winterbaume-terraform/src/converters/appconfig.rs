//! Terraform converters for AppConfig resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_appconfig::AppConfigService;
use winterbaume_appconfig::views::{
    AppconfigStateView, ApplicationView, ConfigurationProfileView, DeploymentStrategyView,
    DeploymentView, EnvironmentView, ExtensionAssociationView, ExtensionView,
    HostedConfigurationVersionView, MonitorView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::appconfig as appconfig_gen;
use crate::util::{classify_deserialize_error, extract_region};

fn minimal_state_view() -> AppconfigStateView {
    AppconfigStateView::default()
}

// ---------------------------------------------------------------------------
// aws_appconfig_application
// ---------------------------------------------------------------------------

pub struct AwsAppconfigApplicationConverter {
    service: Arc<AppConfigService>,
}

impl AwsAppconfigApplicationConverter {
    pub fn new(service: Arc<AppConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppconfigApplicationConverter {
    fn resource_type(&self) -> &str {
        "aws_appconfig_application"
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

impl AwsAppconfigApplicationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: appconfig_gen::ApplicationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_appconfig_application", e))?;

        let description = model.description.unwrap_or_default();

        let app_view = ApplicationView {
            id: model.id.clone(),
            name: model.name,
            description,
        };

        let mut state_view = minimal_state_view();
        state_view.applications.insert(model.id, app_view);
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
                "name": app.name,
                "description": app.description,
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: app.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appconfig_environment
// ---------------------------------------------------------------------------

pub struct AwsAppconfigEnvironmentConverter {
    service: Arc<AppConfigService>,
}

impl AwsAppconfigEnvironmentConverter {
    pub fn new(service: Arc<AppConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppconfigEnvironmentConverter {
    fn resource_type(&self) -> &str {
        "aws_appconfig_environment"
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

impl AwsAppconfigEnvironmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: appconfig_gen::EnvironmentTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_appconfig_environment", e))?;

        let description = model.description.unwrap_or_default();

        // monitor is a nested-block array; read raw from instance attributes.
        let monitors: Vec<MonitorView> = instance
            .attributes
            .get("monitor")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|m| {
                        let alarm_arn = m.get("alarm_arn").and_then(|v| v.as_str())?;
                        let alarm_role_arn = m
                            .get("alarm_role_arn")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        Some(MonitorView {
                            alarm_arn: alarm_arn.to_string(),
                            alarm_role_arn,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let env_view = EnvironmentView {
            id: model.environment_id.clone(),
            application_id: model.application_id.clone(),
            name: model.name,
            description,
            state: "READY_FOR_DEPLOYMENT".to_string(),
            monitors,
        };

        let key = format!("{}/{}", model.application_id, model.environment_id);
        let mut state_view = minimal_state_view();
        state_view.environments.insert(key.clone(), env_view);
        if !model.tags.is_empty() {
            state_view.tags.insert(key, model.tags);
        }
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
        for env in view.environments.values() {
            let monitors: Vec<serde_json::Value> = env
                .monitors
                .iter()
                .map(|m| {
                    serde_json::json!({
                        "alarm_arn": m.alarm_arn,
                        "alarm_role_arn": m.alarm_role_arn,
                    })
                })
                .collect();
            let attrs = serde_json::json!({
                "id": format!("{}:{}", env.application_id, env.id),
                "environment_id": env.id,
                "application_id": env.application_id,
                "name": env.name,
                "description": env.description,
                "state": env.state,
                "monitor": monitors,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", env.application_id, env.id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appconfig_configuration_profile
// ---------------------------------------------------------------------------

pub struct AwsAppconfigConfigurationProfileConverter {
    service: Arc<AppConfigService>,
}

impl AwsAppconfigConfigurationProfileConverter {
    pub fn new(service: Arc<AppConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppconfigConfigurationProfileConverter {
    fn resource_type(&self) -> &str {
        "aws_appconfig_configuration_profile"
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

impl AwsAppconfigConfigurationProfileConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: appconfig_gen::ConfigurationProfileTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_appconfig_configuration_profile", e)
            })?;

        let description = model.description.unwrap_or_default();
        let location_uri = model.location_uri.unwrap_or_else(|| "hosted".to_string());
        let r#type = model.r#type.unwrap_or_else(|| "AWS.Freeform".to_string());

        let profile_view = ConfigurationProfileView {
            id: model.configuration_profile_id.clone(),
            application_id: model.application_id.clone(),
            name: model.name,
            description,
            location_uri,
            r#type,
            retrieval_role_arn: model.retrieval_role_arn,
        };

        let mut state_view = minimal_state_view();
        state_view.configuration_profiles.push(profile_view);
        if !model.tags.is_empty() {
            let tag_key = format!(
                "{}/{}",
                model.application_id, model.configuration_profile_id
            );
            state_view.tags.insert(tag_key, model.tags);
        }
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
        for profile in &view.configuration_profiles {
            let attrs = serde_json::json!({
                "id": format!("{}:{}", profile.application_id, profile.id),
                "configuration_profile_id": profile.id,
                "application_id": profile.application_id,
                "name": profile.name,
                "description": profile.description,
                "location_uri": profile.location_uri,
                "type": profile.r#type,
                "retrieval_role_arn": profile.retrieval_role_arn,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", profile.application_id, profile.id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appconfig_deployment_strategy
// ---------------------------------------------------------------------------

pub struct AwsAppconfigDeploymentStrategyConverter {
    service: Arc<AppConfigService>,
}

impl AwsAppconfigDeploymentStrategyConverter {
    pub fn new(service: Arc<AppConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppconfigDeploymentStrategyConverter {
    fn resource_type(&self) -> &str {
        "aws_appconfig_deployment_strategy"
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

impl AwsAppconfigDeploymentStrategyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: appconfig_gen::DeploymentStrategyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_appconfig_deployment_strategy", e))?;

        let description = model.description.unwrap_or_default();
        let deployment_duration_in_minutes = model.deployment_duration_in_minutes as i32;
        let final_bake_time_in_minutes = model.final_bake_time_in_minutes as i32;
        // growth_factor is f32; not part of strongly-typed model — read raw.
        let growth_factor = instance
            .attributes
            .get("growth_factor")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as f32;
        let growth_type = model.growth_type.unwrap_or_else(|| "LINEAR".to_string());
        let replicate_to = model.replicate_to.unwrap_or_else(|| "NONE".to_string());

        let ds_view = DeploymentStrategyView {
            id: model.id.clone(),
            name: model.name,
            description,
            deployment_duration_in_minutes,
            final_bake_time_in_minutes,
            growth_factor,
            growth_type,
            replicate_to,
        };

        let mut state_view = minimal_state_view();
        state_view.deployment_strategies.insert(model.id, ds_view);
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
        for ds in view.deployment_strategies.values() {
            let attrs = serde_json::json!({
                "id": ds.id,
                "name": ds.name,
                "description": ds.description,
                "deployment_duration_in_minutes": ds.deployment_duration_in_minutes,
                "final_bake_time_in_minutes": ds.final_bake_time_in_minutes,
                "growth_factor": ds.growth_factor,
                "growth_type": ds.growth_type,
                "replicate_to": ds.replicate_to,
            });
            results.push(ExtractedResource {
                name: ds.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appconfig_deployment
// ---------------------------------------------------------------------------

pub struct AwsAppconfigDeploymentConverter {
    service: Arc<AppConfigService>,
}

impl AwsAppconfigDeploymentConverter {
    pub fn new(service: Arc<AppConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppconfigDeploymentConverter {
    fn resource_type(&self) -> &str {
        "aws_appconfig_deployment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_appconfig_application",
            "aws_appconfig_environment",
            "aws_appconfig_configuration_profile",
            "aws_appconfig_deployment_strategy",
        ]
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

impl AwsAppconfigDeploymentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: appconfig_gen::DeploymentTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_appconfig_deployment", e))?;

        let deployment_number = if model.deployment_number > 0 {
            model.deployment_number as i32
        } else {
            1
        };
        let description = model.description.unwrap_or_default();
        let state = model.state.unwrap_or_else(|| "COMPLETE".to_string());

        let key = format!(
            "{}/{}/{}",
            model.application_id, model.environment_id, deployment_number
        );

        let dep_view = DeploymentView {
            deployment_number,
            application_id: model.application_id.clone(),
            environment_id: model.environment_id.clone(),
            deployment_strategy_id: model.deployment_strategy_id,
            configuration_profile_id: model.configuration_profile_id,
            configuration_version: model.configuration_version,
            state,
            started_at: chrono::Utc::now().to_rfc3339(),
            completed_at: chrono::Utc::now().to_rfc3339(),
            description,
        };

        let mut state_view = minimal_state_view();
        state_view.deployments.insert(key.clone(), dep_view);
        if !model.tags.is_empty() {
            state_view.tags.insert(key, model.tags);
        }
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
        for dep in view.deployments.values() {
            let attrs = serde_json::json!({
                "id": format!("{}/{}/{}", dep.application_id, dep.environment_id, dep.deployment_number),
                "application_id": dep.application_id,
                "environment_id": dep.environment_id,
                "deployment_strategy_id": dep.deployment_strategy_id,
                "configuration_profile_id": dep.configuration_profile_id,
                "configuration_version": dep.configuration_version,
                "deployment_number": dep.deployment_number,
                "state": dep.state,
                "description": dep.description,
            });
            results.push(ExtractedResource {
                name: format!(
                    "{}_{}_{}",
                    dep.application_id, dep.environment_id, dep.deployment_number
                ),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appconfig_extension
// ---------------------------------------------------------------------------

pub struct AwsAppconfigExtensionConverter {
    service: Arc<AppConfigService>,
}

impl AwsAppconfigExtensionConverter {
    pub fn new(service: Arc<AppConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppconfigExtensionConverter {
    fn resource_type(&self) -> &str {
        "aws_appconfig_extension"
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

impl AwsAppconfigExtensionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: appconfig_gen::ExtensionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_appconfig_extension", e))?;

        let description = model.description.unwrap_or_default();
        let version_number = if model.version > 0 {
            model.version as i32
        } else {
            1
        };
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:appconfig:{}:{}:extension/{}/{}",
                region, ctx.default_account_id, model.id, version_number
            )
        });

        let ext_view = ExtensionView {
            id: model.id.clone(),
            name: model.name,
            description,
            version_number,
            arn,
            actions: HashMap::new(),
            parameters: HashMap::new(),
        };

        let mut state_view = minimal_state_view();
        state_view.extensions.insert(model.id.clone(), ext_view);
        if !model.tags.is_empty() {
            state_view.tags.insert(model.id, model.tags);
        }
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
        for ext in view.extensions.values() {
            let attrs = serde_json::json!({
                "id": ext.id,
                "name": ext.name,
                "description": ext.description,
                "arn": ext.arn,
                "version": ext.version_number,
            });
            results.push(ExtractedResource {
                name: ext.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appconfig_extension_association
// ---------------------------------------------------------------------------

pub struct AwsAppconfigExtensionAssociationConverter {
    service: Arc<AppConfigService>,
}

impl AwsAppconfigExtensionAssociationConverter {
    pub fn new(service: Arc<AppConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppconfigExtensionAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_appconfig_extension_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_appconfig_extension"]
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

impl AwsAppconfigExtensionAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: appconfig_gen::ExtensionAssociationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_appconfig_extension_association", e)
            })?;

        let extension_version_number = if model.extension_version > 0 {
            model.extension_version as i32
        } else {
            1
        };
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:appconfig:{}:{}:extensionassociation/{}",
                region, ctx.default_account_id, model.id
            )
        });

        // parameters HCL is HashMap<String, String> — read raw.
        let parameters: HashMap<String, String> = instance
            .attributes
            .get("parameters")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let assoc_view = ExtensionAssociationView {
            id: model.id.clone(),
            arn,
            extension_arn: model.extension_arn,
            resource_arn: model.resource_arn,
            extension_version_number,
            parameters,
        };

        let mut state_view = minimal_state_view();
        state_view
            .extension_associations
            .insert(model.id.clone(), assoc_view);
        if !model.tags.is_empty() {
            state_view.tags.insert(model.id, model.tags);
        }
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
        for assoc in view.extension_associations.values() {
            let attrs = serde_json::json!({
                "id": assoc.id,
                "arn": assoc.arn,
                "extension_arn": assoc.extension_arn,
                "resource_arn": assoc.resource_arn,
                "extension_version": assoc.extension_version_number,
                "parameters": assoc.parameters,
            });
            results.push(ExtractedResource {
                name: assoc.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appconfig_hosted_configuration_version
// ---------------------------------------------------------------------------

pub struct AwsAppconfigHostedConfigurationVersionConverter {
    service: Arc<AppConfigService>,
}

impl AwsAppconfigHostedConfigurationVersionConverter {
    pub fn new(service: Arc<AppConfigService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppconfigHostedConfigurationVersionConverter {
    fn resource_type(&self) -> &str {
        "aws_appconfig_hosted_configuration_version"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_appconfig_configuration_profile"]
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

impl AwsAppconfigHostedConfigurationVersionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: appconfig_gen::HostedConfigurationVersionTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_appconfig_hosted_configuration_version", e)
            })?;

        let version_number = if model.version_number > 0 {
            model.version_number as i32
        } else {
            1
        };
        let content_type = model
            .content_type
            .unwrap_or_else(|| "application/json".to_string());
        let description = model.description.unwrap_or_default();

        let hcv_view = HostedConfigurationVersionView {
            application_id: model.application_id.clone(),
            configuration_profile_id: model.configuration_profile_id.clone(),
            version_number,
            content_type,
            description,
        };

        let mut state_view = minimal_state_view();
        state_view.hosted_configuration_versions.push(hcv_view);
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
        for hcv in &view.hosted_configuration_versions {
            let attrs = serde_json::json!({
                "id": format!("{}/{}/{}", hcv.application_id, hcv.configuration_profile_id, hcv.version_number),
                "application_id": hcv.application_id,
                "configuration_profile_id": hcv.configuration_profile_id,
                "version_number": hcv.version_number,
                "content_type": hcv.content_type,
                "description": hcv.description,
            });
            results.push(ExtractedResource {
                name: format!(
                    "{}_{}_{}",
                    hcv.application_id, hcv.configuration_profile_id, hcv.version_number
                ),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
