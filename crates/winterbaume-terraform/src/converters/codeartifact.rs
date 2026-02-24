//! Terraform converters for CodeArtifact resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_codeartifact::CodeArtifactService;
use winterbaume_codeartifact::views::{CodeArtifactStateView, DomainView, RepositoryView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_codeartifact_domain
// ---------------------------------------------------------------------------

pub struct AwsCodeArtifactDomainConverter {
    service: Arc<CodeArtifactService>,
}

impl AwsCodeArtifactDomainConverter {
    pub fn new(service: Arc<CodeArtifactService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCodeArtifactDomainConverter {
    fn resource_type(&self) -> &str {
        "aws_codeartifact_domain"
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

impl AwsCodeArtifactDomainConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let domain = require_str(attrs, "domain", "aws_codeartifact_domain")?;
        let _s3_bucket_arn = optional_str(attrs, "s3_bucket_arn");
        let encryption_key = optional_str(attrs, "encryption_key")
            .or_else(|| optional_str(attrs, "encryption_key"))
            .map(|s| s.to_string());

        let arn = optional_str(attrs, "arn")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:codeartifact:{region}:{}:domain/{domain}",
                    ctx.default_account_id
                )
            });

        let mut tags = extract_tags(attrs);
        // Merge tags_all into tags
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let domain_view = DomainView {
            name: domain.to_string(),
            owner: ctx.default_account_id.clone(),
            arn: arn.clone(),
            encryption_key,
            status: "Active".to_string(),
            created_time: 0.0,
            tags,
        };

        let mut state_view = CodeArtifactStateView::default();
        state_view.domains.insert(domain.to_string(), domain_view);
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
        for domain in view.domains.values() {
            let attrs = serde_json::json!({
                "id": domain.arn,
                "domain": domain.name,
                "arn": domain.arn,
                "owner": domain.owner,
                "encryption_key": domain.encryption_key,
                "tags": domain.tags,
                "tags_all": domain.tags,
            });
            results.push(ExtractedResource {
                name: domain.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_codeartifact_repository
// ---------------------------------------------------------------------------

pub struct AwsCodeArtifactRepositoryConverter {
    service: Arc<CodeArtifactService>,
}

impl AwsCodeArtifactRepositoryConverter {
    pub fn new(service: Arc<CodeArtifactService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCodeArtifactRepositoryConverter {
    fn resource_type(&self) -> &str {
        "aws_codeartifact_repository"
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

impl AwsCodeArtifactRepositoryConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let domain = require_str(attrs, "domain", "aws_codeartifact_repository")?;
        let repository = require_str(attrs, "repository", "aws_codeartifact_repository")?;
        let description = optional_str(attrs, "description").map(|s| s.to_string());

        let arn = optional_str(attrs, "arn")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:codeartifact:{region}:{}:repository/{domain}/{repository}",
                    ctx.default_account_id
                )
            });

        let tags = extract_tags(attrs);
        let external_connections = attrs.get("external_connections").cloned();
        let upstream = attrs.get("upstream").cloned();

        let repo_view = RepositoryView {
            name: repository.to_string(),
            domain_name: domain.to_string(),
            domain_owner: ctx.default_account_id.clone(),
            arn: arn.clone(),
            description,
            created_time: 0.0,
            tags,
            external_connections,
            upstream,
        };

        let key = format!("{domain}/{repository}");
        let mut state_view = CodeArtifactStateView::default();
        state_view.repositories.insert(key, repo_view);
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
                "id": repo.arn,
                "domain": repo.domain_name,
                "repository": repo.name,
                "arn": repo.arn,
                "domain_owner": repo.domain_owner,
                "description": repo.description,
                "tags": repo.tags,
                "external_connections": repo.external_connections,
                "upstream": repo.upstream,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", repo.domain_name, repo.name),
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

fn extract_tags(attrs: &serde_json::Value) -> HashMap<String, String> {
    let mut tags = HashMap::new();
    if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
        for (k, v) in obj {
            if let Some(s) = v.as_str() {
                tags.insert(k.clone(), s.to_string());
            }
        }
    }
    tags
}
