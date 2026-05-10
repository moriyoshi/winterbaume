//! Terraform converters for AppConfig resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_appconfig::AppConfigService;
use winterbaume_appconfig::views::{
    AppconfigStateView, ApplicationView, ConfigurationProfileView, DeploymentStrategyView,
    EnvironmentView, MonitorView,
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
