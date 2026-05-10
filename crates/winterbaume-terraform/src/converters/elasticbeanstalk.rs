//! Terraform converters for Elastic Beanstalk resources.
//!
//! `ApplicationTfModel` and `EnvironmentTfModel` are generated from
//! `specs/elasticbeanstalk.toml`. The ARN templates, the
//! `appversion_lifecycle` raw JSON blob, the `setting` nested-block array,
//! the `tier_type` derivation from `tier_name`, and the `tags_all` merge
//! are wired up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_elasticbeanstalk::ElasticBeanstalkService;
use winterbaume_elasticbeanstalk::views::{
    ApplicationView, ElasticBeanstalkStateView, EnvironmentView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::elasticbeanstalk as elasticbeanstalk_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_elastic_beanstalk_application
// ---------------------------------------------------------------------------

pub struct AwsElasticBeanstalkApplicationConverter {
    service: Arc<ElasticBeanstalkService>,
}

impl AwsElasticBeanstalkApplicationConverter {
    pub fn new(service: Arc<ElasticBeanstalkService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsElasticBeanstalkApplicationConverter {
    fn resource_type(&self) -> &str {
        "aws_elastic_beanstalk_application"
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

impl AwsElasticBeanstalkApplicationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: elasticbeanstalk_gen::ApplicationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_elastic_beanstalk_application", e))?;

        let attrs = &instance.attributes;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:elasticbeanstalk:{}:{}:application/{}",
                region, ctx.default_account_id, name
            )
        });
        // appversion_lifecycle is a nested-block — read raw.
        let appversion_lifecycle = attrs
            .get("appversion_lifecycle")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });

        // Merge `tags_all` into `tags` (tags wins).
        let mut tags = model.tags;
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let app_view = ApplicationView {
            application_name: name,
            description: model.description,
            tags,
            date_created: model
                .date_created
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            date_updated: model
                .date_updated
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            arn,
            appversion_lifecycle,
        };

        let mut state_view = ElasticBeanstalkStateView::default();
        state_view
            .applications
            .insert(app_view.application_name.clone(), app_view);
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
                "id": app.application_name,
                "name": app.application_name,
                "description": app.description,
                "arn": app.arn,
                "date_created": app.date_created,
                "date_updated": app.date_updated,
                "tags": app.tags,
                "appversion_lifecycle": app.appversion_lifecycle,
            });
            results.push(ExtractedResource {
                name: app.application_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_elastic_beanstalk_environment
// ---------------------------------------------------------------------------

pub struct AwsElasticBeanstalkEnvironmentConverter {
    service: Arc<ElasticBeanstalkService>,
}

impl AwsElasticBeanstalkEnvironmentConverter {
    pub fn new(service: Arc<ElasticBeanstalkService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsElasticBeanstalkEnvironmentConverter {
    fn resource_type(&self) -> &str {
        "aws_elastic_beanstalk_environment"
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

impl AwsElasticBeanstalkEnvironmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: elasticbeanstalk_gen::EnvironmentTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_elastic_beanstalk_environment", e))?;

        let attrs = &instance.attributes;

        let name = model.name.clone();
        let application = model.application.clone();
        let environment_id = attrs
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("e-{}", &name[..name.len().min(10)]));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:elasticbeanstalk:{}:{}:environment/{}/{}",
                region, ctx.default_account_id, application, name
            )
        });
        let tier_name = model.tier.unwrap_or_else(|| "WebServer".to_string());
        let tier_type = if tier_name == "Worker" {
            "SQS/HTTP".to_string()
        } else {
            "Standard".to_string()
        };

        let mut tags = model.tags;
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        // setting is a nested-block array — read raw.
        let setting: Vec<serde_json::Value> = attrs
            .get("setting")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let env_view = EnvironmentView {
            environment_name: name,
            application_name: application,
            environment_id,
            description: model.description,
            status: "Ready".to_string(),
            tier_name,
            tier_type,
            health: "Green".to_string(),
            solution_stack_name: model.solution_stack_name,
            tags,
            date_created: model
                .date_created
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            date_updated: model
                .date_updated
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            arn,
            cname: model.cname,
            endpoint_url: model.endpoint_url,
            platform_arn: model.platform_arn,
            version_label: model.version_label,
            template_name: model.template_name,
            setting,
        };

        let mut state_view = ElasticBeanstalkStateView::default();
        state_view
            .environments
            .insert(env_view.environment_name.clone(), env_view);
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
            let attrs = serde_json::json!({
                "id": env.environment_id,
                "name": env.environment_name,
                "application": env.application_name,
                "environment_id": env.environment_id,
                "description": env.description,
                "status": env.status,
                "tier": env.tier_name,
                "tier_name": env.tier_name,
                "tier_type": env.tier_type,
                "health": env.health,
                "solution_stack_name": env.solution_stack_name,
                "arn": env.arn,
                "date_created": env.date_created,
                "date_updated": env.date_updated,
                "cname": env.cname,
                "endpoint_url": env.endpoint_url,
                "platform_arn": env.platform_arn,
                "version_label": env.version_label,
                "template_name": env.template_name,
                "tags": env.tags,
                "tags_all": env.tags,
                "wait_for_ready_timeout": "20m",
                "setting": env.setting,
            });
            results.push(ExtractedResource {
                name: env.environment_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
