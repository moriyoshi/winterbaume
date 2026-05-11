//! Terraform converter for Cognito Identity resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_cognitoidentity::CognitoIdentityService;
use winterbaume_cognitoidentity::views::{
    CognitoIdentityProviderView, CognitoIdentityStateView, IdentityPoolRolesView, IdentityPoolView,
    PrincipalTagEntryView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::cognitoidentity as cognitoidentity_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_cognito_identity_pool
// ---------------------------------------------------------------------------

pub struct AwsCognitoIdentityPoolConverter {
    service: Arc<CognitoIdentityService>,
}

impl AwsCognitoIdentityPoolConverter {
    pub fn new(service: Arc<CognitoIdentityService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCognitoIdentityPoolConverter {
    fn resource_type(&self) -> &str {
        "aws_cognito_identity_pool"
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

impl AwsCognitoIdentityPoolConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cognitoidentity_gen::IdentityPoolTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cognito_identity_pool", e))?;

        // Vec/map nested fields stay raw.
        let attrs = &instance.attributes;
        let supported_login_providers: HashMap<String, String> = attrs
            .get("supported_login_providers")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let open_id_connect_provider_arns: Vec<String> = attrs
            .get("openid_connect_provider_arns")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let saml_provider_arns: Vec<String> = attrs
            .get("saml_provider_arns")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let cognito_identity_providers: Vec<CognitoIdentityProviderView> = attrs
            .get("cognito_identity_providers")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|p| {
                        let provider_name = p.get("provider_name").and_then(|v| v.as_str())?;
                        let client_id = p.get("client_id").and_then(|v| v.as_str())?;
                        let server_side_token_check = p
                            .get("server_side_token_check")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        Some(CognitoIdentityProviderView {
                            provider_name: provider_name.to_string(),
                            client_id: client_id.to_string(),
                            server_side_token_check,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let id = model.identity_pool_id.clone();

        let pool_view = IdentityPoolView {
            identity_pool_id: id.clone(),
            identity_pool_name: model.identity_pool_name,
            allow_unauthenticated_identities: model.allow_unauthenticated_identities,
            supported_login_providers,
            developer_provider_name: model.developer_provider_name,
            open_id_connect_provider_arns,
            cognito_identity_providers,
            saml_provider_arns,
            created_date: None,
        };

        let arn = format!(
            "arn:aws:cognito-identity:{}:{}:identitypool/{}",
            region, ctx.default_account_id, id
        );

        let mut state_view = CognitoIdentityStateView::default();
        state_view.identity_pools.insert(id, pool_view);
        if !model.tags.is_empty() {
            state_view.resource_tags.insert(arn, model.tags);
        }
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
        for pool in view.identity_pools.values() {
            let arn = format!(
                "arn:aws:cognito-identity:{}:{}:identitypool/{}",
                ctx.default_region, ctx.default_account_id, pool.identity_pool_id
            );
            let empty_tags: HashMap<String, String> = HashMap::new();
            let tags: &HashMap<String, String> =
                view.resource_tags.get(&arn).unwrap_or(&empty_tags);

            let providers: Vec<serde_json::Value> = pool
                .cognito_identity_providers
                .iter()
                .map(|p| {
                    serde_json::json!({
                        "provider_name": p.provider_name,
                        "client_id": p.client_id,
                        "server_side_token_check": p.server_side_token_check,
                    })
                })
                .collect();

            let attrs = serde_json::json!({
                "id": pool.identity_pool_id,
                "identity_pool_name": pool.identity_pool_name,
                "allow_unauthenticated_identities": pool.allow_unauthenticated_identities,
                "supported_login_providers": pool.supported_login_providers,
                "developer_provider_name": pool.developer_provider_name,
                "openid_connect_provider_arns": pool.open_id_connect_provider_arns,
                "cognito_identity_providers": providers,
                "saml_provider_arns": pool.saml_provider_arns,
                "arn": arn,
                "tags": tags,
            });
            results.push(ExtractedResource {
                name: pool.identity_pool_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cognito_identity_pool_roles_attachment
// ---------------------------------------------------------------------------

pub struct AwsCognitoIdentityPoolRolesAttachmentConverter {
    service: Arc<CognitoIdentityService>,
}

impl AwsCognitoIdentityPoolRolesAttachmentConverter {
    pub fn new(service: Arc<CognitoIdentityService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCognitoIdentityPoolRolesAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_cognito_identity_pool_roles_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cognito_identity_pool"]
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

impl AwsCognitoIdentityPoolRolesAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: cognitoidentity_gen::CognitoIdentityPoolRolesAttachmentTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_cognito_identity_pool_roles_attachment", e)
            })?;

        // The `roles` block in HCL is `roles = { authenticated = "...", unauthenticated = "..." }`.
        let roles: HashMap<String, String> = attrs
            .get("roles")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let role_mappings: HashMap<String, serde_json::Value> = attrs
            .get("role_mapping")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|e| {
                        e.get("identity_provider")
                            .and_then(|p| p.as_str())
                            .map(|p| (p.to_string(), e.clone()))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let view = IdentityPoolRolesView {
            roles,
            role_mappings,
        };

        let mut state_view = CognitoIdentityStateView::default();
        state_view.pool_roles.insert(model.identity_pool_id, view);
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
        for (pool_id, pr) in &view.pool_roles {
            let attrs = serde_json::json!({
                "id": pool_id,
                "identity_pool_id": pool_id,
                "roles": pr.roles,
                "role_mapping": pr.role_mappings.values().cloned().collect::<Vec<_>>(),
            });
            results.push(ExtractedResource {
                name: pool_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cognito_identity_pool_provider_principal_tag
// ---------------------------------------------------------------------------

pub struct AwsCognitoIdentityPoolProviderPrincipalTagConverter {
    service: Arc<CognitoIdentityService>,
}

impl AwsCognitoIdentityPoolProviderPrincipalTagConverter {
    pub fn new(service: Arc<CognitoIdentityService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCognitoIdentityPoolProviderPrincipalTagConverter {
    fn resource_type(&self) -> &str {
        "aws_cognito_identity_pool_provider_principal_tag"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cognito_identity_pool"]
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

impl AwsCognitoIdentityPoolProviderPrincipalTagConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: cognitoidentity_gen::CognitoIdentityPoolProviderPrincipalTagTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_cognito_identity_pool_provider_principal_tag", e)
            })?;

        let principal_tags: HashMap<String, String> = attrs
            .get("principal_tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let key = format!(
            "{}\u{0000}{}",
            model.identity_pool_id, model.identity_provider_name
        );
        let view = PrincipalTagEntryView {
            identity_pool_id: model.identity_pool_id,
            identity_provider_name: model.identity_provider_name,
            use_defaults: model.use_defaults,
            principal_tags,
        };

        let mut state_view = CognitoIdentityStateView::default();
        state_view.principal_tags.insert(key, view);
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
        for pt in view.principal_tags.values() {
            let id = format!("{}:{}", pt.identity_pool_id, pt.identity_provider_name);
            let attrs = serde_json::json!({
                "id": id,
                "identity_pool_id": pt.identity_pool_id,
                "identity_provider_name": pt.identity_provider_name,
                "use_defaults": pt.use_defaults,
                "principal_tags": pt.principal_tags,
            });
            results.push(ExtractedResource {
                name: id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
