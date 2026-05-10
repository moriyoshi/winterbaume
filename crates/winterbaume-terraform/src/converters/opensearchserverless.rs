//! Terraform converters for OpenSearch Serverless resources.
//!
//! `CollectionTfModel` and `SecurityPolicyTfModel` are generated from
//! `specs/opensearchserverless.toml`. The synthesised collection ID,
//! ARN template, the constants (`status = "ACTIVE"`, `policy_version
//! = "v1"`, `type = "SEARCH"`), and the JSON-blob `policy` parsing
//! are wired up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_opensearchserverless::OpenSearchServerlessService;
use winterbaume_opensearchserverless::views::{
    CollectionView, OpenSearchServerlessStateView, SecurityPolicyView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::opensearchserverless as opensearchserverless_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_opensearchserverless_collection
// ---------------------------------------------------------------------------

pub struct AwsOpensearchserverlessCollectionConverter {
    service: Arc<OpenSearchServerlessService>,
}

impl AwsOpensearchserverlessCollectionConverter {
    pub fn new(service: Arc<OpenSearchServerlessService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOpensearchserverlessCollectionConverter {
    fn resource_type(&self) -> &str {
        "aws_opensearchserverless_collection"
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

impl AwsOpensearchserverlessCollectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: opensearchserverless_gen::CollectionTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_opensearchserverless_collection", e)
            })?;

        let id = model.id.clone().unwrap_or_else(|| {
            // Generate a short ID similar to AOSS format
            uuid::Uuid::new_v4().to_string().replace('-', "")[..12].to_string()
        });
        let arn = model.arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:aoss:{}:{}:collection/{}",
                region, ctx.default_account_id, id
            )
        });
        let type_ = model.r#type.clone().unwrap_or_else(|| "SEARCH".to_string());

        let view = OpenSearchServerlessStateView {
            collections: std::collections::HashMap::from([(
                id.clone(),
                CollectionView {
                    id,
                    name: model.name,
                    arn,
                    type_,
                    status: "ACTIVE".to_string(),
                    description: model.description,
                    kms_key_arn: model.kms_key_arn,
                    created_date: 0,
                    last_modified_date: 0,
                },
            )]),
            ..Default::default()
        };

        self.service
            .merge(&ctx.default_account_id, &region, view)
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
        let region = ctx.default_region.clone();
        let snap = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        let resources = snap
            .collections
            .into_values()
            .map(|c| {
                let attrs = serde_json::json!({
                    "id": c.id,
                    "name": c.name,
                    "arn": c.arn,
                    "type": c.type_,
                    "description": c.description,
                    "tags_all": {},
                    "standby_replicas": "ENABLED",
                });
                ExtractedResource {
                    name: c.id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                }
            })
            .collect();

        Ok(resources)
    }
}

// ---------------------------------------------------------------------------
// aws_opensearchserverless_security_policy
// ---------------------------------------------------------------------------

pub struct AwsOpensearchserverlessSecurityPolicyConverter {
    service: Arc<OpenSearchServerlessService>,
}

impl AwsOpensearchserverlessSecurityPolicyConverter {
    pub fn new(service: Arc<OpenSearchServerlessService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOpensearchserverlessSecurityPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_opensearchserverless_security_policy"
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

impl AwsOpensearchserverlessSecurityPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: opensearchserverless_gen::SecurityPolicyTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_opensearchserverless_security_policy", e)
            })?;

        let policy_str = model.policy.clone();
        let policy: serde_json::Value =
            serde_json::from_str(&policy_str).unwrap_or(serde_json::Value::String(policy_str));
        let policy_version = model
            .policy_version
            .clone()
            .unwrap_or_else(|| "v1".to_string());
        let name = model.name.clone();
        let type_ = model.r#type.clone();

        let key = format!("{type_}/{name}");
        let view = OpenSearchServerlessStateView {
            security_policies: std::collections::HashMap::from([(
                key,
                SecurityPolicyView {
                    name,
                    type_,
                    policy,
                    policy_version,
                    description: model.description,
                    created_date: 0,
                    last_modified_date: 0,
                },
            )]),
            ..Default::default()
        };

        self.service
            .merge(&ctx.default_account_id, &region, view)
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
        let region = ctx.default_region.clone();
        let snap = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        let resources = snap
            .security_policies
            .into_values()
            .map(|sp| {
                let id = format!("{}/{}", sp.type_, sp.name);
                let attrs = serde_json::json!({
                    "name": sp.name,
                    "type": sp.type_,
                    "policy": sp.policy.to_string(),
                    "policy_version": sp.policy_version,
                    "description": sp.description,
                });
                ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                }
            })
            .collect();

        Ok(resources)
    }
}
