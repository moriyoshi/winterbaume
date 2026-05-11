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
    CollectionView, OpenSearchServerlessStateView, SecurityPolicyView, VpcEndpointView,
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

// ---------------------------------------------------------------------------
// aws_opensearchserverless_access_policy
// ---------------------------------------------------------------------------

pub struct AwsOpensearchserverlessAccessPolicyConverter {
    service: Arc<OpenSearchServerlessService>,
}

impl AwsOpensearchserverlessAccessPolicyConverter {
    pub fn new(service: Arc<OpenSearchServerlessService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOpensearchserverlessAccessPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_opensearchserverless_access_policy"
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

impl AwsOpensearchserverlessAccessPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: opensearchserverless_gen::OpensearchserverlessAccessPolicyTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_opensearchserverless_access_policy", e)
            })?;

        let policy: serde_json::Value =
            serde_json::from_str(&model.policy).unwrap_or(serde_json::json!([]));
        let policy_type = model.policy_type.unwrap_or_else(|| "data".to_string());
        let key = format!("access:{}:{}", policy_type, model.name);

        let view = SecurityPolicyView {
            name: model.name,
            type_: policy_type,
            policy,
            policy_version: model
                .policy_version
                .unwrap_or_else(|| "MTY3MjUzNTY0OS4xNjY=".to_string()),
            description: model.description,
            created_date: 0,
            last_modified_date: 0,
        };

        let mut state_view = OpenSearchServerlessStateView::default();
        state_view.security_policies.insert(key, view);
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
        for (key, sp) in &view.security_policies {
            if !key.starts_with("access:") {
                continue;
            }
            let attrs = serde_json::json!({
                "id": sp.name,
                "name": sp.name,
                "type": sp.type_,
                "policy": sp.policy,
                "policy_version": sp.policy_version,
                "description": sp.description,
            });
            results.push(ExtractedResource {
                name: sp.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_opensearchserverless_vpc_endpoint
// ---------------------------------------------------------------------------

pub struct AwsOpensearchserverlessVpcEndpointConverter {
    service: Arc<OpenSearchServerlessService>,
}

impl AwsOpensearchserverlessVpcEndpointConverter {
    pub fn new(service: Arc<OpenSearchServerlessService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOpensearchserverlessVpcEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_opensearchserverless_vpc_endpoint"
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

impl AwsOpensearchserverlessVpcEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: opensearchserverless_gen::OpensearchserverlessVpcEndpointTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_opensearchserverless_vpc_endpoint", e)
            })?;

        let id = model
            .id
            .unwrap_or_else(|| format!("vpce-{}", uuid::Uuid::new_v4().simple()));

        let subnet_ids: Vec<String> = attrs
            .get("subnet_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let security_group_ids: Vec<String> = attrs
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let view = VpcEndpointView {
            id: id.clone(),
            name: model.name,
            vpc_id: model.vpc_id,
            subnet_ids,
            security_group_ids,
            status: "ACTIVE".to_string(),
        };

        let mut state_view = OpenSearchServerlessStateView::default();
        state_view.vpc_endpoints.insert(id, view);
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
        for vpce in view.vpc_endpoints.values() {
            let attrs = serde_json::json!({
                "id": vpce.id,
                "name": vpce.name,
                "vpc_id": vpce.vpc_id,
                "subnet_ids": vpce.subnet_ids,
                "security_group_ids": vpce.security_group_ids,
            });
            results.push(ExtractedResource {
                name: vpce.name.clone(),
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
// ---------------------------------------------------------------------------

macro_rules! oss_warning_only_converter {
    (
        struct_name = $struct_name:ident,
        resource_type = $resource_type:expr,
        model_type = $model_type:ident,
        warn_msg = $warn_msg:expr $(,)?
    ) => {
        pub struct $struct_name {
            #[allow(dead_code)]
            service: Arc<OpenSearchServerlessService>,
        }

        impl $struct_name {
            pub fn new(service: Arc<OpenSearchServerlessService>) -> Self {
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
                let _model: opensearchserverless_gen::$model_type =
                    serde_json::from_value(attrs.clone())
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

oss_warning_only_converter! {
    struct_name = AwsOpensearchserverlessLifecyclePolicyConverter,
    resource_type = "aws_opensearchserverless_lifecycle_policy",
    model_type = OpensearchserverlessLifecyclePolicyTfModel,
    warn_msg = "no state slot in winterbaume_opensearchserverless; inject is a no-op",
}

oss_warning_only_converter! {
    struct_name = AwsOpensearchserverlessSecurityConfigConverter,
    resource_type = "aws_opensearchserverless_security_config",
    model_type = OpensearchserverlessSecurityConfigTfModel,
    warn_msg = "no state slot in winterbaume_opensearchserverless; inject is a no-op",
}
