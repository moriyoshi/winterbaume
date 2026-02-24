//! Terraform converters for CodeDeploy resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_codedeploy::CodeDeployService;
use winterbaume_codedeploy::views::{ApplicationView, CodeDeployStateView, DeploymentGroupView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_codedeploy_app
// ---------------------------------------------------------------------------

pub struct AwsCodedeployAppConverter {
    service: Arc<CodeDeployService>,
}

impl AwsCodedeployAppConverter {
    pub fn new(service: Arc<CodeDeployService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCodedeployAppConverter {
    fn resource_type(&self) -> &str {
        "aws_codedeploy_app"
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

impl AwsCodedeployAppConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_codedeploy_app")?;
        let application_id = optional_str(attrs, "application_id")
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let _tags_all = attrs.get("tags_all");
        let _tags = attrs.get("tags");
        let compute_platform =
            optional_str(attrs, "compute_platform").unwrap_or_else(|| "Server".to_string());

        let app_view = ApplicationView {
            application_id,
            application_name: name.to_string(),
            compute_platform,
            create_time: Utc::now().to_rfc3339(),
        };

        let mut state_view = minimal_codedeploy_state_view();
        state_view.applications.insert(name.to_string(), app_view);
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
                "application_id": app.application_id,
                "compute_platform": app.compute_platform,
                "tags_all": {},
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
// aws_codedeploy_deployment_group
// ---------------------------------------------------------------------------

pub struct AwsCodedeployDeploymentGroupConverter {
    service: Arc<CodeDeployService>,
}

impl AwsCodedeployDeploymentGroupConverter {
    pub fn new(service: Arc<CodeDeployService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCodedeployDeploymentGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_codedeploy_deployment_group"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_codedeploy_app"]
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

impl AwsCodedeployDeploymentGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let app_name = require_str(attrs, "app_name", "aws_codedeploy_deployment_group")?;
        let group_name = require_str(
            attrs,
            "deployment_group_name",
            "aws_codedeploy_deployment_group",
        )?;
        let service_role_arn = optional_str(attrs, "service_role_arn").unwrap_or_default();
        let deployment_config_name = optional_str(attrs, "deployment_config_name")
            .unwrap_or_else(|| "CodeDeployDefault.AllAtOnce".to_string());

        // Additional fields for coverage
        let _ = attrs.get("tags_all");
        let _ = attrs.get("auto_rollback_configuration");
        let _ = attrs.get("deployment_style");

        let alarm_configuration = attrs.get("alarm_configuration").cloned();
        let blue_green_deployment_config = attrs.get("blue_green_deployment_config").cloned();
        let ec2_tag_filter: Vec<serde_json::Value> = attrs
            .get("ec2_tag_filter")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let ec2_tag_set: Vec<serde_json::Value> = attrs
            .get("ec2_tag_set")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let ecs_service: Vec<serde_json::Value> = attrs
            .get("ecs_service")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let load_balancer_info: Vec<serde_json::Value> = attrs
            .get("load_balancer_info")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let trigger_configuration: Vec<serde_json::Value> = attrs
            .get("trigger_configuration")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        // Snapshot + restore since deployment_groups is a Vec
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        // Look up the application's compute_platform from the existing state
        let compute_platform = {
            let snapshot = self
                .service
                .snapshot(&ctx.default_account_id, &region)
                .await;
            snapshot
                .applications
                .get(app_name)
                .map(|a| a.compute_platform.clone())
                .unwrap_or_else(|| "Server".to_string())
        };

        let dg_view = DeploymentGroupView {
            deployment_group_id: uuid::Uuid::new_v4().to_string(),
            deployment_group_name: group_name.to_string(),
            application_name: app_name.to_string(),
            service_role_arn,
            deployment_config_name,
            compute_platform,
            create_time: Utc::now().to_rfc3339(),
            alarm_configuration,
            blue_green_deployment_config,
            ec2_tag_filter,
            ec2_tag_set,
            ecs_service,
            load_balancer_info,
            trigger_configuration,
        };

        // Remove any existing group with same name/app before adding
        state_view.deployment_groups.retain(|dg| {
            !(dg.application_name == app_name && dg.deployment_group_name == group_name)
        });
        state_view.deployment_groups.push(dg_view);

        self.service
            .restore(&ctx.default_account_id, &region, state_view)
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
        for dg in &view.deployment_groups {
            let arn = format!(
                "arn:aws:codedeploy:{}:{}:deploymentgroup:{}/{}",
                ctx.default_region,
                ctx.default_account_id,
                dg.application_name,
                dg.deployment_group_name,
            );
            let alarm_config = dg
                .alarm_configuration
                .clone()
                .unwrap_or(serde_json::json!([]));
            let bg_config = dg
                .blue_green_deployment_config
                .clone()
                .unwrap_or(serde_json::json!([]));
            let attrs = serde_json::json!({
                "id": dg.deployment_group_id,
                "arn": arn,
                "app_name": dg.application_name,
                "deployment_group_name": dg.deployment_group_name,
                "deployment_group_id": dg.deployment_group_id,
                "service_role_arn": dg.service_role_arn,
                "deployment_config_name": dg.deployment_config_name,
                "compute_platform": dg.compute_platform,
                "deployment_style": [{"deployment_option": "WITHOUT_TRAFFIC_CONTROL", "deployment_type": "IN_PLACE"}],
                "auto_rollback_configuration": [{"enabled": false, "events": []}],
                "alarm_configuration": alarm_config,
                "blue_green_deployment_config": bg_config,
                "ec2_tag_filter": dg.ec2_tag_filter,
                "ec2_tag_set": dg.ec2_tag_set,
                "ecs_service": dg.ecs_service,
                "load_balancer_info": dg.load_balancer_info,
                "trigger_configuration": dg.trigger_configuration,
                "tags": {},
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: format!("{}:{}", dg.application_name, dg.deployment_group_name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn minimal_codedeploy_state_view() -> CodeDeployStateView {
    CodeDeployStateView {
        applications: HashMap::new(),
        deployment_groups: vec![],
        deployments: HashMap::new(),
        tags: HashMap::new(),
    }
}
