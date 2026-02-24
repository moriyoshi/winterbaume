//! Terraform converter for ECR resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_ecr::EcrService;
use winterbaume_ecr::views::{
    EcrStateView, EncryptionConfigView, ImageScanningConfigView, RegistryScanningConfigView,
    ReplicationConfigView, RepositoryView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

/// Converts `aws_ecr_repository` Terraform resources to/from ECR state.
pub struct AwsEcrRepositoryConverter {
    service: Arc<EcrService>,
}

impl AwsEcrRepositoryConverter {
    pub fn new(service: Arc<EcrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcrRepositoryConverter {
    fn resource_type(&self) -> &str {
        "aws_ecr_repository"
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

impl AwsEcrRepositoryConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_ecr_repository")?;
        let region = extract_region(attrs, &ctx.default_region);

        let arn = optional_str(attrs, "arn")
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:ecr:{}:{}:repository/{}",
                    region, ctx.default_account_id, name
                )
            });

        let repository_url = optional_str(attrs, "repository_url").unwrap_or_else(|| {
            format!(
                "{}.dkr.ecr.{}.amazonaws.com/{}",
                ctx.default_account_id, region, name
            )
        });

        let image_tag_mutability =
            optional_str(attrs, "image_tag_mutability").unwrap_or_else(|| "MUTABLE".to_string());

        let scan_on_push = attrs
            .get("image_scanning_configuration")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("scan_on_push"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let encryption_configuration = attrs
            .get("encryption_configuration")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|item| {
                let encryption_type = item
                    .get("encryption_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("AES256")
                    .to_string();
                let kms_key = item
                    .get("kms_key")
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string());
                EncryptionConfigView {
                    encryption_type,
                    kms_key,
                }
            })
            .unwrap_or_default();

        let repo_view = RepositoryView {
            repository_name: name.to_string(),
            repository_arn: arn.clone(),
            repository_uri: repository_url,
            registry_id: ctx.default_account_id.clone(),
            created_at: Some(Utc::now().to_rfc3339()),
            images: vec![],
            tags: extract_tags(attrs),
            lifecycle_policy: None,
            repository_policy: None,
            image_scanning_configuration: ImageScanningConfigView { scan_on_push },
            image_tag_mutability,
            encryption_configuration,
        };

        let state_view = EcrStateView {
            repositories: {
                let mut m = HashMap::new();
                m.insert(name.to_string(), repo_view);
                m
            },
            registry_policy: None,
            registry_scanning_configuration: RegistryScanningConfigView::default(),
            replication_configuration: ReplicationConfigView::default(),
            pull_through_cache_rules: vec![],
            ..Default::default()
        };
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
        for repo in view.repositories.values() {
            let mut encryption_obj = serde_json::json!({
                "encryption_type": repo.encryption_configuration.encryption_type,
            });
            if let Some(ref kms_key) = repo.encryption_configuration.kms_key {
                encryption_obj["kms_key"] = serde_json::json!(kms_key);
            }
            let attrs = serde_json::json!({
                "id": repo.repository_name,
                "name": repo.repository_name,
                "arn": repo.repository_arn,
                "repository_url": repo.repository_uri,
                "registry_id": repo.registry_id,
                "image_tag_mutability": repo.image_tag_mutability,
                "image_scanning_configuration": [{
                    "scan_on_push": repo.image_scanning_configuration.scan_on_push,
                }],
                "encryption_configuration": [encryption_obj],
                "tags": repo.tags,
            });
            results.push(ExtractedResource {
                name: repo.repository_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
