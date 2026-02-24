//! Terraform converter for Cognito Identity resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_cognitoidentity::CognitoIdentityService;
use winterbaume_cognitoidentity::views::{
    CognitoIdentityProviderView, CognitoIdentityStateView, IdentityPoolView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_bool, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let id = require_str(attrs, "id", "aws_cognito_identity_pool")?;
        let identity_pool_name =
            require_str(attrs, "identity_pool_name", "aws_cognito_identity_pool")?;
        let allow_unauthenticated_identities =
            optional_bool(attrs, "allow_unauthenticated_identities").unwrap_or(false);
        let developer_provider_name = optional_str(attrs, "developer_provider_name");

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

        let tags = extract_tags(attrs);

        let pool_view = IdentityPoolView {
            identity_pool_id: id.to_string(),
            identity_pool_name: identity_pool_name.to_string(),
            allow_unauthenticated_identities,
            supported_login_providers,
            developer_provider_name,
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
        state_view.identity_pools.insert(id.to_string(), pool_view);
        if !tags.is_empty() {
            state_view.resource_tags.insert(arn, tags);
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
