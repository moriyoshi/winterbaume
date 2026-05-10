//! Terraform converter for ECR resources.
//!
//! `RepositoryTfModel` is generated from `specs/ecr.toml`. The ARN and
//! repository-URL templates, the `image_tag_mutability = "MUTABLE"`
//! default, the `created_at` timestamp, and the nested
//! `image_scanning_configuration` / `encryption_configuration` blocks are
//! wired up here.

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
use crate::generated::ecr as ecr_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ecr_gen::RepositoryTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_ecr_repository", e))?;

        let name = model.name.clone();
        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:ecr:{}:{}:repository/{}",
                region, ctx.default_account_id, name
            )
        });
        let repository_url = model.repository_url.unwrap_or_else(|| {
            format!(
                "{}.dkr.ecr.{}.amazonaws.com/{}",
                ctx.default_account_id, region, name
            )
        });
        let image_tag_mutability = model
            .image_tag_mutability
            .unwrap_or_else(|| "MUTABLE".to_string());

        // Nested blocks stay as raw reads — spec can't express list-of-object.
        let attrs = &instance.attributes;
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
            repository_name: name.clone(),
            repository_arn: arn.clone(),
            repository_uri: repository_url,
            registry_id: ctx.default_account_id.clone(),
            created_at: Some(Utc::now().to_rfc3339()),
            images: vec![],
            tags: model.tags,
            lifecycle_policy: None,
            repository_policy: None,
            image_scanning_configuration: ImageScanningConfigView { scan_on_push },
            image_tag_mutability,
            encryption_configuration,
        };

        let state_view = EcrStateView {
            repositories: {
                let mut m = HashMap::new();
                m.insert(name, repo_view);
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
