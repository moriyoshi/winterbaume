//! Terraform converters for Amplify resources.
//!
//! `AmplifyAppTfModel` and `AmplifyBranchTfModel` are generated from
//! `specs/amplify.toml`. The ARN templates, the synthesised `app_id`
//! (lowercased name with spaces replaced), the `default_domain`
//! template, the nested-block reads (`auto_branch_creation_config`,
//! `cache_config`, `custom_rule`/`custom_rules`), and the various
//! constants are wired up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_amplify::AmplifyService;
use winterbaume_amplify::views::{
    AmplifyAppView, AmplifyBranchView, AmplifyDomainAssociationView, AmplifyStateView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::amplify as amplify_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: amplify_gen::AmplifyAppTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_amplify_app", e))?;

        let name = model.name;
        let app_id = model
            .app_id
            .unwrap_or_else(|| name.replace(' ', "-").to_lowercase());
        let app_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:amplify:{}:{}:apps/{}",
                region, ctx.default_account_id, app_id
            )
        });
        let default_domain = model
            .default_domain
            .unwrap_or_else(|| format!("{app_id}.amplifyapp.com"));

        let attrs = &instance.attributes;
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
            description: model.description,
            repository: model.repository,
            platform: model.platform,
            create_time: 0.0,
            update_time: 0.0,
            iam_service_role_arn: model.iam_service_role_arn,
            environment_variables: Default::default(),
            default_domain,
            enable_branch_auto_build: false,
            enable_branch_auto_deletion: false,
            enable_basic_auth: false,
            build_spec: model.build_spec,
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: amplify_gen::AmplifyBranchTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_amplify_branch", e))?;

        let app_id = model.app_id;
        let branch_name = model.branch_name;
        let branch_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:amplify:{}:{}:apps/{}/branches/{}",
                region, ctx.default_account_id, app_id, branch_name
            )
        });

        let branch_view = AmplifyBranchView {
            app_id: app_id.clone(),
            branch_arn,
            branch_name: branch_name.clone(),
            description: model.description,
            stage: model.stage,
            display_name: None,
            enable_auto_build: true,
            enable_basic_auth: false,
            enable_notification: false,
            enable_performance_mode: false,
            enable_pull_request_preview: false,
            environment_variables: Default::default(),
            framework: model.framework,
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

// ---------------------------------------------------------------------------
// aws_amplify_domain_association
// ---------------------------------------------------------------------------

pub struct AwsAmplifyDomainAssociationConverter {
    service: Arc<AmplifyService>,
}

impl AwsAmplifyDomainAssociationConverter {
    pub fn new(service: Arc<AmplifyService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAmplifyDomainAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_amplify_domain_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_amplify_app"]
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

impl AwsAmplifyDomainAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: amplify_gen::AmplifyDomainAssociationTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_amplify_domain_association", e))?;

        let app_id = model.app_id;
        let domain_name = model.domain_name;
        let key = format!("{}/{}", app_id, domain_name);

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:amplify:{}:{}:apps/{}/domains/{}",
                region, ctx.default_account_id, app_id, domain_name
            )
        });

        let view = AmplifyDomainAssociationView {
            app_id: app_id.clone(),
            domain_association_arn: arn,
            domain_name: domain_name.clone(),
            enable_auto_sub_domain: model.enable_auto_sub_domain,
            domain_status: "AVAILABLE".to_string(),
            status_reason: String::new(),
            sub_domains: vec![],
        };

        let mut state_view = AmplifyStateView::default();
        state_view.domain_associations.insert(key, view);
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
        for da in view.domain_associations.values() {
            let attrs = serde_json::json!({
                "id": format!("{}/{}", da.app_id, da.domain_name),
                "app_id": da.app_id,
                "domain_name": da.domain_name,
                "arn": da.domain_association_arn,
                "enable_auto_sub_domain": da.enable_auto_sub_domain,
                "domain_status": da.domain_status,
                "status_reason": da.status_reason,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", da.app_id, da.domain_name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Warning-only converters
//
// `aws_amplify_backend_environment` and `aws_amplify_webhook` do not have
// corresponding state slots in `winterbaume_amplify`. Inject validates
// the TF attributes against the generated model and emits a warning.
// ---------------------------------------------------------------------------

macro_rules! amplify_warning_only_converter {
    (
        struct_name = $struct_name:ident,
        resource_type = $resource_type:expr,
        model_type = $model_type:ident,
        warn_msg = $warn_msg:expr $(,)?
    ) => {
        pub struct $struct_name {
            #[allow(dead_code)]
            service: Arc<AmplifyService>,
        }

        impl $struct_name {
            pub fn new(service: Arc<AmplifyService>) -> Self {
                Self { service }
            }
        }

        impl TerraformResourceConverter for $struct_name {
            fn resource_type(&self) -> &str {
                $resource_type
            }

            fn inject<'a>(
                &'a self,
                instance: &'a ResourceInstance,
                ctx: &'a ConversionContext,
            ) -> Pin<
                Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>,
            > {
                Box::pin(async move { self.do_inject(instance, ctx).await })
            }

            fn extract<'a>(
                &'a self,
                _ctx: &'a ConversionContext,
            ) -> Pin<
                Box<
                    dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>>
                        + Send
                        + 'a,
                >,
            > {
                Box::pin(async move { Ok(vec![]) })
            }
        }

        impl $struct_name {
            async fn do_inject(
                &self,
                instance: &ResourceInstance,
                ctx: &ConversionContext,
            ) -> Result<ConversionResult, ConversionError> {
                let attrs = &instance.attributes;
                let region = extract_region(attrs, &ctx.default_region);
                let _model: amplify_gen::$model_type = serde_json::from_value(attrs.clone())
                    .map_err(|e| classify_deserialize_error($resource_type, e))?;
                eprintln!("warning: {}: {}", $resource_type, $warn_msg);
                Ok(ConversionResult {
                    region,
                    warnings: vec![format!("{}: {}", $resource_type, $warn_msg)],
                })
            }
        }
    };
}

amplify_warning_only_converter! {
    struct_name = AwsAmplifyBackendEnvironmentConverter,
    resource_type = "aws_amplify_backend_environment",
    model_type = AmplifyBackendEnvironmentTfModel,
    warn_msg = "no state slot in winterbaume_amplify; inject is a no-op",
}

amplify_warning_only_converter! {
    struct_name = AwsAmplifyWebhookConverter,
    resource_type = "aws_amplify_webhook",
    model_type = AmplifyWebhookTfModel,
    warn_msg = "no state slot in winterbaume_amplify; inject is a no-op",
}
