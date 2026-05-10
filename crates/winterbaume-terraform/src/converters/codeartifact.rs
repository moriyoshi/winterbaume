//! Terraform converters for CodeArtifact resources.
//!
//! `DomainTfModel` and `RepositoryTfModel` are generated from
//! `specs/codeartifact.toml`. The ARN templates, the `tags_all` merge
//! (only fills keys absent from `tags`), the constant `status` /
//! `created_time` values, and the raw-Value `external_connections` /
//! `upstream` reads on the repository are wired up here.

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
use crate::generated::codeartifact as codeartifact_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: codeartifact_gen::DomainTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_codeartifact_domain", e))?;

        let domain = model.domain.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:codeartifact:{region}:{}:domain/{domain}",
                ctx.default_account_id
            )
        });

        let mut tags = model.tags;
        // Merge tags_all into tags
        if let Some(obj) = instance
            .attributes
            .get("tags_all")
            .and_then(|v| v.as_object())
        {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let domain_view = DomainView {
            name: domain.clone(),
            owner: ctx.default_account_id.clone(),
            arn: arn.clone(),
            encryption_key: model.encryption_key,
            status: "Active".to_string(),
            created_time: 0.0,
            tags,
        };

        let mut state_view = CodeArtifactStateView::default();
        state_view.domains.insert(domain, domain_view);
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: codeartifact_gen::RepositoryTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_codeartifact_repository", e))?;

        let domain = model.domain.clone();
        let repository = model.repository.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:codeartifact:{region}:{}:repository/{domain}/{repository}",
                ctx.default_account_id
            )
        });

        // Raw-Value blobs not part of the strongly-typed model.
        let attrs = &instance.attributes;
        let external_connections = attrs.get("external_connections").cloned();
        let upstream = attrs.get("upstream").cloned();

        let repo_view = RepositoryView {
            name: repository.clone(),
            domain_name: domain.clone(),
            domain_owner: ctx.default_account_id.clone(),
            arn: arn.clone(),
            description: model.description,
            created_time: 0.0,
            tags: model.tags,
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
