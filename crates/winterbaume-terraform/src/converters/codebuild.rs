//! Terraform converters for CodeBuild resources.
//!
//! `ProjectTfModel` is generated from `specs/codebuild.toml`. The ARN
//! template, the `source` / `artifacts` / `environment` nested-block
//! parsing, the `tags` projection into `Vec<TagView>`, the
//! `created` / `last_modified` constants (set to `Utc::now()`), and
//! the raw-Value `build_batch_config` / `cache` /
//! `file_system_locations` / `logs_config` / `secondary_artifacts` /
//! `secondary_sources` / `vpc_config` reads are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_codebuild::CodeBuildService;
use winterbaume_codebuild::views::{
    CodeBuildStateView, ProjectView, ReportGroupView, SourceCredentialView, TagView, WebhookView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::codebuild as codebuild_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_codebuild_project
// ---------------------------------------------------------------------------

pub struct AwsCodebuildProjectConverter {
    service: Arc<CodeBuildService>,
}

impl AwsCodebuildProjectConverter {
    pub fn new(service: Arc<CodeBuildService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCodebuildProjectConverter {
    fn resource_type(&self) -> &str {
        "aws_codebuild_project"
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

impl AwsCodebuildProjectConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: codebuild_gen::ProjectTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_codebuild_project", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:codebuild:{}:{}:project/{}",
                region, ctx.default_account_id, name
            )
        });
        let description = model.description.unwrap_or_default();
        let service_role = model.service_role.unwrap_or_default();

        let source_type = attrs
            .get("source")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|s| s.get("type"))
            .and_then(|v| v.as_str())
            .unwrap_or("NO_SOURCE")
            .to_string();
        let source_location = attrs
            .get("source")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|s| s.get("location"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let artifact_type = attrs
            .get("artifacts")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|a| a.get("type"))
            .and_then(|v| v.as_str())
            .unwrap_or("NO_ARTIFACTS")
            .to_string();
        let artifact_location = attrs
            .get("artifacts")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|a| a.get("location"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let environment_type = attrs
            .get("environment")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|e| e.get("type"))
            .and_then(|v| v.as_str())
            .unwrap_or("LINUX_CONTAINER")
            .to_string();
        let environment_image = attrs
            .get("environment")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|e| e.get("image"))
            .and_then(|v| v.as_str())
            .unwrap_or("aws/codebuild/standard:7.0")
            .to_string();
        let environment_compute_type = attrs
            .get("environment")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|e| e.get("compute_type"))
            .and_then(|v| v.as_str())
            .unwrap_or("BUILD_GENERAL1_SMALL")
            .to_string();

        let build_batch_config = attrs
            .get("build_batch_config")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let cache = attrs
            .get("cache")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let file_system_locations: Vec<serde_json::Value> = attrs
            .get("file_system_locations")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let logs_config = attrs
            .get("logs_config")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let secondary_artifacts: Vec<serde_json::Value> = attrs
            .get("secondary_artifacts")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let secondary_sources: Vec<serde_json::Value> = attrs
            .get("secondary_sources")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let vpc_config = attrs
            .get("vpc_config")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });

        let tags: Vec<TagView> = attrs
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| {
                        v.as_str().map(|s| TagView {
                            key: k.clone(),
                            value: s.to_string(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let now = Utc::now().to_rfc3339();
        let project_view = ProjectView {
            name: name.clone(),
            arn,
            description,
            source_type,
            source_location,
            artifact_type,
            artifact_location,
            environment_type,
            environment_image,
            environment_compute_type,
            service_role,
            tags,
            created: now.clone(),
            last_modified: now,
            build_batch_config,
            cache,
            file_system_locations,
            logs_config,
            secondary_artifacts,
            secondary_sources,
            vpc_config,
        };

        let mut state_view = CodeBuildStateView::default();
        state_view.projects.insert(name, project_view);
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
        for project in view.projects.values() {
            let tags: HashMap<String, String> = project
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": project.arn,
                "name": project.name,
                "arn": project.arn,
                "description": project.description,
                "service_role": project.service_role,
                "source": [{
                    "type": project.source_type,
                    "location": project.source_location,
                }],
                "artifacts": [{
                    "type": project.artifact_type,
                    "location": project.artifact_location,
                }],
                "environment": [{
                    "type": project.environment_type,
                    "image": project.environment_image,
                    "compute_type": project.environment_compute_type,
                }],
                "created": project.created,
                "last_modified": project.last_modified,
                "badge_enabled": false,
                "badge_url": "",
                "build_timeout": 60,
                "queued_timeout": 480,
                "encryption_key": "",
                "concurrent_build_limit": 0,
                "tags": tags,
                "tags_all": tags,
                "build_batch_config": project.build_batch_config,
                "cache": project.cache,
                "file_system_locations": project.file_system_locations,
                "logs_config": project.logs_config,
                "secondary_artifacts": project.secondary_artifacts,
                "secondary_sources": project.secondary_sources,
                "vpc_config": project.vpc_config,
            });
            results.push(ExtractedResource {
                name: project.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_codebuild_fleet — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_codebuild_fleet` resources (validation-only; no backing
/// state slot in `winterbaume_codebuild`).
pub struct AwsCodebuildFleetConverter {
    #[allow(dead_code)]
    service: Arc<CodeBuildService>,
}

impl AwsCodebuildFleetConverter {
    pub fn new(service: Arc<CodeBuildService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCodebuildFleetConverter {
    fn resource_type(&self) -> &str {
        "aws_codebuild_fleet"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: codebuild_gen::FleetTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_codebuild_fleet", e))?;
            let warn_msg =
                "no state slot in winterbaume_codebuild for fleets; inject is a no-op".to_string();
            eprintln!("warning: aws_codebuild_fleet: {warn_msg}");
            Ok(ConversionResult {
                region,
                warnings: vec![format!("aws_codebuild_fleet: {warn_msg}")],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_codebuild_report_group
// ---------------------------------------------------------------------------

pub struct AwsCodebuildReportGroupConverter {
    service: Arc<CodeBuildService>,
}

impl AwsCodebuildReportGroupConverter {
    pub fn new(service: Arc<CodeBuildService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCodebuildReportGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_codebuild_report_group"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: codebuild_gen::ReportGroupTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_codebuild_report_group", e))?;

            let arn = model.arn.unwrap_or_else(|| {
                format!(
                    "arn:aws:codebuild:{}:{}:report-group/{}",
                    region, ctx.default_account_id, model.name
                )
            });

            let attrs = &instance.attributes;
            let tags: Vec<TagView> = attrs
                .get("tags")
                .and_then(|v| v.as_object())
                .map(|obj| {
                    obj.iter()
                        .filter_map(|(k, v)| {
                            v.as_str().map(|s| TagView {
                                key: k.clone(),
                                value: s.to_string(),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            let export_config_type = attrs
                .get("export_config")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .and_then(|e| e.get("type"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let now = Utc::now().to_rfc3339();
            let rg = ReportGroupView {
                arn: arn.clone(),
                name: model.name.clone(),
                r#type: model.report_type,
                export_config_type,
                tags,
                created: now.clone(),
                last_modified: now,
                status: "ACTIVE".to_string(),
            };

            let mut state_view = CodeBuildStateView::default();
            state_view.report_groups.insert(arn, rg);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for rg in view.report_groups.values() {
                let tags: HashMap<String, String> = rg
                    .tags
                    .iter()
                    .map(|t| (t.key.clone(), t.value.clone()))
                    .collect();
                let attrs = serde_json::json!({
                    "id": rg.arn,
                    "name": rg.name,
                    "arn": rg.arn,
                    "type": rg.r#type,
                    "created": rg.created,
                    "last_modified": rg.last_modified,
                    "status": rg.status,
                    "tags": tags,
                    "tags_all": tags,
                });
                results.push(ExtractedResource {
                    name: rg.name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_codebuild_resource_policy
// ---------------------------------------------------------------------------

pub struct AwsCodebuildResourcePolicyConverter {
    service: Arc<CodeBuildService>,
}

impl AwsCodebuildResourcePolicyConverter {
    pub fn new(service: Arc<CodeBuildService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCodebuildResourcePolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_codebuild_resource_policy"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: codebuild_gen::ResourcePolicyTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_codebuild_resource_policy", e))?;

            let mut state_view = CodeBuildStateView::default();
            state_view
                .resource_policies
                .insert(model.resource_arn, model.policy);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for (arn, policy) in view.resource_policies.iter() {
                let attrs = serde_json::json!({
                    "id": arn,
                    "resource_arn": arn,
                    "policy": policy,
                });
                results.push(ExtractedResource {
                    name: arn.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_codebuild_source_credential
// ---------------------------------------------------------------------------

pub struct AwsCodebuildSourceCredentialConverter {
    service: Arc<CodeBuildService>,
}

impl AwsCodebuildSourceCredentialConverter {
    pub fn new(service: Arc<CodeBuildService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCodebuildSourceCredentialConverter {
    fn resource_type(&self) -> &str {
        "aws_codebuild_source_credential"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: codebuild_gen::SourceCredentialTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_codebuild_source_credential", e)
                })?;

            let arn = model.arn.unwrap_or_else(|| {
                format!(
                    "arn:aws:codebuild:{}:{}:token/{}",
                    region,
                    ctx.default_account_id,
                    model.server_type.to_lowercase()
                )
            });

            let sc = SourceCredentialView {
                arn: arn.clone(),
                server_type: model.server_type,
                auth_type: model.auth_type,
                resource: model.user_name,
            };

            let mut state_view = CodeBuildStateView::default();
            state_view.source_credentials.insert(arn, sc);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for sc in view.source_credentials.values() {
                let attrs = serde_json::json!({
                    "id": sc.arn,
                    "arn": sc.arn,
                    "auth_type": sc.auth_type,
                    "server_type": sc.server_type,
                    "user_name": sc.resource,
                });
                results.push(ExtractedResource {
                    name: sc.arn.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_codebuild_webhook
// ---------------------------------------------------------------------------

pub struct AwsCodebuildWebhookConverter {
    service: Arc<CodeBuildService>,
}

impl AwsCodebuildWebhookConverter {
    pub fn new(service: Arc<CodeBuildService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCodebuildWebhookConverter {
    fn resource_type(&self) -> &str {
        "aws_codebuild_webhook"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: codebuild_gen::WebhookTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_codebuild_webhook", e))?;

            let wh = WebhookView {
                project_name: model.project_name.clone(),
                url: model.url.unwrap_or_default(),
                branch_filter: model.branch_filter,
                build_type: model.build_type,
                secret: model.secret,
            };

            let mut state_view = CodeBuildStateView::default();
            state_view.webhooks.insert(model.project_name, wh);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for wh in view.webhooks.values() {
                let attrs = serde_json::json!({
                    "id": wh.project_name,
                    "project_name": wh.project_name,
                    "url": wh.url,
                    "branch_filter": wh.branch_filter,
                    "build_type": wh.build_type,
                    "secret": wh.secret,
                });
                results.push(ExtractedResource {
                    name: wh.project_name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}
