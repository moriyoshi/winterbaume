//! Terraform converters for CodeCommit resources.
//!
//! `RepositoryTfModel` is generated from `specs/codecommit.toml`. The
//! ARN template, the synthesised `repository_id` (UUID), the clone-URL
//! templates, and the `creation_date` / `last_modified_date`
//! constants are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_codecommit::CodeCommitService;
use winterbaume_codecommit::views::{CodeCommitStateView, RepositoryView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::codecommit as codecommit_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_codecommit_repository
// ---------------------------------------------------------------------------

pub struct AwsCodecommitRepositoryConverter {
    service: Arc<CodeCommitService>,
}

impl AwsCodecommitRepositoryConverter {
    pub fn new(service: Arc<CodeCommitService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCodecommitRepositoryConverter {
    fn resource_type(&self) -> &str {
        "aws_codecommit_repository"
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

impl AwsCodecommitRepositoryConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: codecommit_gen::RepositoryTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_codecommit_repository", e))?;

        let name = model.repository_name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:codecommit:{}:{}:{}",
                region, ctx.default_account_id, name
            )
        });
        let repository_id = model
            .repository_id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let description = model.description.unwrap_or_default();
        let clone_url_http = model.clone_url_http.unwrap_or_else(|| {
            format!(
                "https://git-codecommit.{}.amazonaws.com/v1/repos/{}",
                region, name
            )
        });
        let clone_url_ssh = model.clone_url_ssh.unwrap_or_else(|| {
            format!(
                "ssh://git-codecommit.{}.amazonaws.com/v1/repos/{}",
                region, name
            )
        });
        let now = Utc::now().to_rfc3339();

        let repo_view = RepositoryView {
            repository_id,
            repository_name: name.clone(),
            arn,
            description,
            clone_url_http,
            clone_url_ssh,
            creation_date: now.clone(),
            last_modified_date: now,
            account_id: ctx.default_account_id.clone(),
            default_branch: None,
            tags: HashMap::new(),
        };

        let mut state_view = CodeCommitStateView {
            ..Default::default()
        };
        state_view.repositories.insert(name, repo_view);
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
            let attrs = serde_json::json!({
                "id": repo.repository_name,
                "repository_name": repo.repository_name,
                "arn": repo.arn,
                "repository_id": repo.repository_id,
                "description": repo.description,
                "clone_url_http": repo.clone_url_http,
                "clone_url_ssh": repo.clone_url_ssh,
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
