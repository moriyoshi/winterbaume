//! Terraform converters for Amplify resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_amplify::AmplifyService;
use winterbaume_amplify::views::{AmplifyAppView, AmplifyBranchView, AmplifyStateView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_amplify_app
// ---------------------------------------------------------------------------

pub struct AwsAmplifyAppConverter {
    service: Arc<AmplifyService>,
}

impl AwsAmplifyAppConverter {
    pub fn new(service: Arc<AmplifyService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAmplifyAppConverter {
    fn resource_type(&self) -> &str {
        "aws_amplify_app"
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

impl AwsAmplifyAppConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_amplify_app")?.to_string();
        let app_id = optional_str(attrs, "app_id")
            .map(|s| s.to_string())
            .unwrap_or_else(|| name.replace(' ', "-").to_lowercase());
        let app_arn = optional_str(attrs, "arn")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:amplify:{}:{}:apps/{}",
                    region, ctx.default_account_id, app_id
                )
            });
        let default_domain = optional_str(attrs, "default_domain")
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("{app_id}.amplifyapp.com"));
        let description = optional_str(attrs, "description").map(|s| s.to_string());
        let repository = optional_str(attrs, "repository").map(|s| s.to_string());
        let platform = optional_str(attrs, "platform").map(|s| s.to_string());

        let _tags_all = attrs.get("tags_all");
        let _access_token = optional_str(attrs, "access_token");
        let _auto_branch_creation_patterns = attrs.get("auto_branch_creation_patterns");
        let _custom_headers = optional_str(attrs, "custom_headers");
        let _oauth_token = optional_str(attrs, "oauth_token");
        let _enable_auto_sub_domain = attrs.get("enable_auto_sub_domain");

        // Parse `auto_branch_creation_config` nested block: [{...}]
        let auto_branch_creation_config: Option<serde_json::Value> = attrs
            .get("auto_branch_creation_config")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned();

        // Parse `cache_config` nested block: [{type: ...}]
        let cache_config: Option<serde_json::Value> = attrs
            .get("cache_config")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned();

        // Parse `custom_rule` blocks: [{source, target, status, condition}]
        let custom_rules: Vec<serde_json::Value> = attrs
            .get("custom_rule")
            .or_else(|| attrs.get("custom_rules"))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let app_view = AmplifyAppView {
            app_id: app_id.clone(),
            app_arn,
            name,
            description,
            repository,
            platform,
            create_time: 0.0,
            update_time: 0.0,
            iam_service_role_arn: optional_str(attrs, "iam_service_role_arn")
                .map(|s| s.to_string()),
            environment_variables: Default::default(),
            default_domain,
            enable_branch_auto_build: false,
            enable_branch_auto_deletion: false,
            enable_basic_auth: false,
            build_spec: optional_str(attrs, "build_spec").map(|s| s.to_string()),
            custom_headers: None,
            tags: Default::default(),
            auto_branch_creation_config,
            cache_config,
            custom_rules,
        };

        let mut state_view = AmplifyStateView::default();
        state_view.apps.insert(app_id, app_view);
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
        for app in view.apps.values() {
            let auto_branch_config_block: Vec<serde_json::Value> = app
                .auto_branch_creation_config
                .as_ref()
                .map(|v| vec![v.clone()])
                .unwrap_or_default();

            let cache_config_block: Vec<serde_json::Value> = app
                .cache_config
                .as_ref()
                .map(|v| vec![v.clone()])
                .unwrap_or_default();

            let attrs = serde_json::json!({
                "id": app.app_id,
                "app_id": app.app_id,
                "arn": app.app_arn,
                "name": app.name,
                "default_domain": app.default_domain,
                "description": app.description,
                "repository": app.repository,
                "platform": app.platform,
                "create_time": app.create_time,
                "update_time": app.update_time,
                "environment_variables": app.environment_variables,
                "enable_branch_auto_build": app.enable_branch_auto_build,
                "enable_basic_auth": app.enable_basic_auth,
                "build_spec": app.build_spec,
                "tags_all": app.tags,
                "production_branch": null,
                "auto_branch_creation_config": auto_branch_config_block,
                "cache_config": cache_config_block,
                "custom_rule": app.custom_rules,
                "enable_auto_branch_creation": false,
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

// ---------------------------------------------------------------------------
// aws_amplify_branch
// ---------------------------------------------------------------------------

pub struct AwsAmplifyBranchConverter {
    service: Arc<AmplifyService>,
}

impl AwsAmplifyBranchConverter {
    pub fn new(service: Arc<AmplifyService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAmplifyBranchConverter {
    fn resource_type(&self) -> &str {
        "aws_amplify_branch"
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

impl AwsAmplifyBranchConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let app_id = require_str(attrs, "app_id", "aws_amplify_branch")?.to_string();
        let branch_name = require_str(attrs, "branch_name", "aws_amplify_branch")?.to_string();
        let branch_arn = optional_str(attrs, "arn")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:amplify:{}:{}:apps/{}/branches/{}",
                    region, ctx.default_account_id, app_id, branch_name
                )
            });
        let description = optional_str(attrs, "description").map(|s| s.to_string());
        let stage = optional_str(attrs, "stage").map(|s| s.to_string());
        let framework = optional_str(attrs, "framework").map(|s| s.to_string());

        let _tags_all = attrs.get("tags_all");
        let _backend_environment_arn = optional_str(attrs, "backend_environment_arn");
        let _basic_auth_credentials = optional_str(attrs, "basic_auth_credentials");
        let _enable_performance_mode = attrs.get("enable_performance_mode");
        let _enable_pull_request_preview = attrs.get("enable_pull_request_preview");
        let _pull_request_environment_name = optional_str(attrs, "pull_request_environment_name");
        let _enable_notification = attrs.get("enable_notification");
        let _ = attrs.get("source_branch");

        let branch_view = AmplifyBranchView {
            app_id: app_id.clone(),
            branch_arn,
            branch_name: branch_name.clone(),
            description,
            stage,
            display_name: None,
            enable_auto_build: true,
            enable_basic_auth: false,
            enable_notification: false,
            enable_performance_mode: false,
            enable_pull_request_preview: false,
            environment_variables: Default::default(),
            framework,
            ttl: None,
            create_time: 0.0,
            update_time: 0.0,
            total_number_of_jobs: "0".to_string(),
            active_job_id: None,
            tags: Default::default(),
        };

        let key = format!("{}\x00{}", app_id, branch_name);
        let mut state_view = AmplifyStateView::default();
        state_view.branches.insert(key, branch_view);
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
        for branch in view.branches.values() {
            let attrs = serde_json::json!({
                "id": branch.branch_arn,
                "arn": branch.branch_arn,
                "app_id": branch.app_id,
                "branch_name": branch.branch_name,
                "description": branch.description,
                "stage": branch.stage,
                "framework": branch.framework,
                "create_time": branch.create_time,
                "update_time": branch.update_time,
                "enable_auto_build": branch.enable_auto_build,
                "enable_basic_auth": branch.enable_basic_auth,
                "display_name": branch.display_name,
                "environment_variables": branch.environment_variables,
                "ttl": branch.ttl,
                "tags_all": branch.tags,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", branch.app_id, branch.branch_name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
