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
use crate::util::{extract_region, extract_tags, optional_i64, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let id = require_str(attrs, "id", "aws_appconfig_application")?;
        let name = require_str(attrs, "name", "aws_appconfig_application")?;
        let description = optional_str(attrs, "description").unwrap_or_default();
        let _tags_all = attrs.get("tags_all");

        let app_view = ApplicationView {
            id: id.to_string(),
            name: name.to_string(),
            description,
        };

        let mut state_view = minimal_state_view();
        state_view.applications.insert(id.to_string(), app_view);
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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let environment_id = require_str(attrs, "environment_id", "aws_appconfig_environment")?;
        let application_id = require_str(attrs, "application_id", "aws_appconfig_environment")?;
        let name = require_str(attrs, "name", "aws_appconfig_environment")?;
        let description = optional_str(attrs, "description").unwrap_or_default();

        let monitors: Vec<MonitorView> = attrs
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
            id: environment_id.to_string(),
            application_id: application_id.to_string(),
            name: name.to_string(),
            description,
            state: "READY_FOR_DEPLOYMENT".to_string(),
            monitors,
        };

        let tags = extract_tags(attrs);

        let key = format!("{}/{}", application_id, environment_id);
        let mut state_view = minimal_state_view();
        state_view.environments.insert(key.clone(), env_view);
        if !tags.is_empty() {
            state_view.tags.insert(key, tags);
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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let configuration_profile_id = require_str(
            attrs,
            "configuration_profile_id",
            "aws_appconfig_configuration_profile",
        )?;
        let application_id = require_str(
            attrs,
            "application_id",
            "aws_appconfig_configuration_profile",
        )?;
        let name = require_str(attrs, "name", "aws_appconfig_configuration_profile")?;
        let description = optional_str(attrs, "description").unwrap_or_default();
        let location_uri =
            optional_str(attrs, "location_uri").unwrap_or_else(|| "hosted".to_string());
        let r#type = optional_str(attrs, "type").unwrap_or_else(|| "AWS.Freeform".to_string());
        let retrieval_role_arn = optional_str(attrs, "retrieval_role_arn");

        let profile_view = ConfigurationProfileView {
            id: configuration_profile_id.to_string(),
            application_id: application_id.to_string(),
            name: name.to_string(),
            description,
            location_uri,
            r#type,
            retrieval_role_arn,
        };

        let tags = extract_tags(attrs);

        let mut state_view = minimal_state_view();
        state_view.configuration_profiles.push(profile_view);
        if !tags.is_empty() {
            let tag_key = format!("{}/{}", application_id, configuration_profile_id);
            state_view.tags.insert(tag_key, tags);
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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let id = require_str(attrs, "id", "aws_appconfig_deployment_strategy")?;
        let name = require_str(attrs, "name", "aws_appconfig_deployment_strategy")?;
        let description = optional_str(attrs, "description").unwrap_or_default();
        let deployment_duration_in_minutes =
            optional_i64(attrs, "deployment_duration_in_minutes").unwrap_or(0) as i32;
        let final_bake_time_in_minutes =
            optional_i64(attrs, "final_bake_time_in_minutes").unwrap_or(0) as i32;
        let growth_factor = attrs
            .get("growth_factor")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0) as f32;
        let growth_type =
            optional_str(attrs, "growth_type").unwrap_or_else(|| "LINEAR".to_string());
        let replicate_to =
            optional_str(attrs, "replicate_to").unwrap_or_else(|| "NONE".to_string());

        let ds_view = DeploymentStrategyView {
            id: id.to_string(),
            name: name.to_string(),
            description,
            deployment_duration_in_minutes,
            final_bake_time_in_minutes,
            growth_factor,
            growth_type,
            replicate_to,
        };

        let mut state_view = minimal_state_view();
        state_view
            .deployment_strategies
            .insert(id.to_string(), ds_view);
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
