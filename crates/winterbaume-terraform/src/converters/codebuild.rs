//! Terraform converters for CodeBuild resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_codebuild::CodeBuildService;
use winterbaume_codebuild::views::{CodeBuildStateView, ProjectView, TagView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_codebuild_project")?;
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:codebuild:{}:{}:project/{}",
                region, ctx.default_account_id, name
            )
        });
        let description = optional_str(attrs, "description").unwrap_or_default();
        let service_role = optional_str(attrs, "service_role").unwrap_or_default();

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

        let _tags_all = attrs.get("tags_all");
        let _build_timeout = attrs.get("build_timeout");
        let _queued_timeout = attrs.get("queued_timeout");
        let _concurrent_build_limit = attrs.get("concurrent_build_limit");
        let _badge_enabled = attrs.get("badge_enabled");
        let _source_version = optional_str(attrs, "source_version");

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
            name: name.to_string(),
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
        state_view.projects.insert(name.to_string(), project_view);
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
