//! Terraform converters for Elastic Beanstalk resources.

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
use crate::util::{extract_region, extract_tags, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_elastic_beanstalk_application")?;
        let description = optional_str(attrs, "description");
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:elasticbeanstalk:{}:{}:application/{}",
                region, ctx.default_account_id, name
            )
        });
        let tags = extract_tags(attrs);
        let appversion_lifecycle = attrs
            .get("appversion_lifecycle")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });

        let app_view = ApplicationView {
            application_name: name.to_string(),
            description,
            tags,
            date_created: optional_str(attrs, "date_created")
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            date_updated: optional_str(attrs, "date_updated")
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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_elastic_beanstalk_environment")?;
        let application = require_str(attrs, "application", "aws_elastic_beanstalk_environment")?;
        let environment_id = optional_str(attrs, "id")
            .unwrap_or_else(|| format!("e-{}", &name[..name.len().min(10)]));
        let description = optional_str(attrs, "description");
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:elasticbeanstalk:{}:{}:environment/{}/{}",
                region, ctx.default_account_id, application, name
            )
        });
        let tier_name = optional_str(attrs, "tier").unwrap_or_else(|| "WebServer".to_string());
        let tier_type = if tier_name == "Worker" {
            "SQS/HTTP".to_string()
        } else {
            "Standard".to_string()
        };
        let solution_stack_name = optional_str(attrs, "solution_stack_name");
        let mut tags = extract_tags(attrs);
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let cname = optional_str(attrs, "cname");
        let endpoint_url = optional_str(attrs, "endpoint_url");
        let platform_arn = optional_str(attrs, "platform_arn");
        let version_label = optional_str(attrs, "version_label");
        let template_name = optional_str(attrs, "template_name");
        let _wait_for_ready_timeout = optional_str(attrs, "wait_for_ready_timeout");
        let _poll_interval = optional_str(attrs, "poll_interval");
        let setting: Vec<serde_json::Value> = attrs
            .get("setting")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let env_view = EnvironmentView {
            environment_name: name.to_string(),
            application_name: application.to_string(),
            environment_id,
            description,
            status: "Ready".to_string(),
            tier_name,
            tier_type,
            health: "Green".to_string(),
            solution_stack_name,
            tags,
            date_created: optional_str(attrs, "date_created")
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            date_updated: optional_str(attrs, "date_updated")
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            arn,
            cname,
            endpoint_url,
            platform_arn,
            version_label,
            template_name,
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
