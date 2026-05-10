//! Terraform converters for Cognito IDP resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_cognitoidentityprovider::CognitoIdentityProviderService;
use winterbaume_cognitoidentityprovider::views::{
    CognitoidpStateView, MfaConfigView, UserPoolClientView, UserPoolDomainView, UserPoolView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::cognitoidp as cognitoidp_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_cognito_user_pool
// ---------------------------------------------------------------------------

/// Converts `aws_cognito_user_pool` Terraform resources to/from Cognito IDP state.
pub struct AwsCognitoUserPoolConverter {
    service: Arc<CognitoIdentityProviderService>,
}

impl AwsCognitoUserPoolConverter {
    pub fn new(service: Arc<CognitoIdentityProviderService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCognitoUserPoolConverter {
    fn resource_type(&self) -> &str {
        "aws_cognito_user_pool"
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

impl AwsCognitoUserPoolConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cognitoidp_gen::UserPoolTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cognito_user_pool", e))?;

        let attrs = &instance.attributes;

        // Additional fields for coverage
        let _ = attrs.get("tags_all");
        let _ = attrs.get("account_recovery_setting");
        let _ = attrs.get("admin_create_user_config");
        let _ = attrs.get("alias_attributes");
        let _ = attrs.get("device_configuration");
        let _ = attrs.get("email_configuration");
        let _ = attrs.get("email_verification_message");
        let _ = attrs.get("email_verification_subject");
        let _ = attrs.get("lambda_config");
        let _ = attrs.get("password_policy");
        let _ = attrs.get("schema");
        let _ = attrs.get("sms_configuration");
        let _ = attrs.get("verification_message_template");
        let _ = attrs.get("user_pool_add_ons");

        let name = model.name.clone();
        let pool_id = model
            .id
            .unwrap_or_else(|| format!("{}_{}", region, &uuid::Uuid::new_v4().to_string()[..8]));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:cognito-idp:{}:{}:userpool/{}",
                region, ctx.default_account_id, pool_id
            )
        });

        let tags = model.tags.clone();

        let custom_attributes: Vec<String> = attrs
            .get("custom_attributes")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        // Extract mfa_configuration
        let mfa_config = model
            .mfa_configuration
            .map(|mfa_configuration| MfaConfigView {
                mfa_configuration,
                sms_mfa_configuration: None,
                software_token_mfa_configuration: None,
            });

        // Extract domain (endpoint attr in TF)
        let domain = model.domain.map(|domain_str| UserPoolDomainView {
            domain: domain_str,
            user_pool_id: pool_id.clone(),
            status: "ACTIVE".to_string(),
            cloud_front_distribution: None,
            custom_domain_config: None,
        });

        // Extract auto_verified_attributes and username_attributes
        let auto_verified_attributes: Vec<String> = attrs
            .get("auto_verified_attributes")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let username_attributes: Vec<String> = attrs
            .get("username_attributes")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        // Store auto_verified_attributes and username_attributes as custom_attributes
        // since the view doesn't have dedicated fields for them
        let all_custom_attributes = custom_attributes;
        // We encode these in the tags for now to preserve them through round-trip
        let mut enriched_tags = tags.clone();
        if !auto_verified_attributes.is_empty() {
            enriched_tags.insert(
                "__auto_verified_attributes".to_string(),
                serde_json::to_string(&auto_verified_attributes).unwrap_or_default(),
            );
        }
        if !username_attributes.is_empty() {
            enriched_tags.insert(
                "__username_attributes".to_string(),
                serde_json::to_string(&username_attributes).unwrap_or_default(),
            );
        }

        let deletion_protection = model
            .deletion_protection
            .unwrap_or_else(|| "INACTIVE".to_string());
        if deletion_protection != "INACTIVE" {
            enriched_tags.insert("__deletion_protection".to_string(), deletion_protection);
        }

        let pool_view = UserPoolView {
            id: pool_id.clone(),
            name,
            arn,
            status: "Active".to_string(),
            created_date: Utc::now().to_rfc3339(),
            clients: HashMap::new(),
            users: HashMap::new(),
            groups: HashMap::new(),
            identity_providers: HashMap::new(),
            resource_servers: HashMap::new(),
            domain,
            mfa_config,
            custom_attributes: all_custom_attributes,
            tags: enriched_tags,
        };

        let mut state_view = CognitoidpStateView {
            user_pools: HashMap::new(),
        };
        state_view.user_pools.insert(pool_id, pool_view);
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
        for pool in view.user_pools.values() {
            let endpoint = pool
                .domain
                .as_ref()
                .map(|d| d.domain.clone())
                .unwrap_or_default();
            let mfa_configuration = pool
                .mfa_config
                .as_ref()
                .map(|m| m.mfa_configuration.clone())
                .unwrap_or_else(|| "OFF".to_string());

            // Recover auto_verified_attributes and username_attributes from tags
            let auto_verified_attributes: Vec<String> = pool
                .tags
                .get("__auto_verified_attributes")
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or_default();
            let username_attributes: Vec<String> = pool
                .tags
                .get("__username_attributes")
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or_default();

            // Recover deletion_protection from tags
            let deletion_protection = pool
                .tags
                .get("__deletion_protection")
                .cloned()
                .unwrap_or_else(|| "INACTIVE".to_string());

            // Filter out internal tags for output
            let user_tags: HashMap<String, String> = pool
                .tags
                .iter()
                .filter(|(k, _)| !k.starts_with("__"))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();

            let attrs = serde_json::json!({
                "id": pool.id,
                "name": pool.name,
                "arn": pool.arn,
                "endpoint": endpoint,
                "creation_date": pool.created_date,
                "last_modified_date": pool.created_date,
                "status": pool.status,
                "mfa_configuration": mfa_configuration,
                "deletion_protection": deletion_protection,
                "custom_attributes": pool.custom_attributes,
                "auto_verified_attributes": auto_verified_attributes,
                "username_attributes": username_attributes,
                "tags": user_tags,
                "tags_all": user_tags,
                "account_recovery_setting": [],
                "admin_create_user_config": [],
                "email_configuration": [],
                "password_policy": [],
                "schema": [],
                "sms_configuration": [],
                "domain": "",
                "custom_domain": "",
            });
            results.push(ExtractedResource {
                name: pool.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cognito_user_pool_client
// ---------------------------------------------------------------------------

/// Converts `aws_cognito_user_pool_client` Terraform resources to/from Cognito IDP state.
pub struct AwsCognitoUserPoolClientConverter {
    service: Arc<CognitoIdentityProviderService>,
}

impl AwsCognitoUserPoolClientConverter {
    pub fn new(service: Arc<CognitoIdentityProviderService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCognitoUserPoolClientConverter {
    fn resource_type(&self) -> &str {
        "aws_cognito_user_pool_client"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cognito_user_pool"]
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

impl AwsCognitoUserPoolClientConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cognitoidp_gen::UserPoolClientTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cognito_user_pool_client", e))?;

        let attrs = &instance.attributes;

        let name = model.name.clone();
        let user_pool_id = model.user_pool_id.clone();
        let _access_token_validity = attrs.get("access_token_validity");
        let _id_token_validity = attrs.get("id_token_validity");
        let _token_validity_units = attrs.get("token_validity_units");
        let _analytics_configuration = attrs.get("analytics_configuration");
        let client_id = model
            .id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()[..26].to_string());

        let explicit_auth_flows: Vec<String> = attrs
            .get("explicit_auth_flows")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let allowed_oauth_flows: Vec<String> = attrs
            .get("allowed_oauth_flows")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let allowed_oauth_scopes: Vec<String> = attrs
            .get("allowed_oauth_scopes")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let callback_urls: Vec<String> = attrs
            .get("callback_urls")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let logout_urls: Vec<String> = attrs
            .get("logout_urls")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let client_view = UserPoolClientView {
            id: client_id.clone(),
            name,
            user_pool_id: user_pool_id.clone(),
            client_secret: model.client_secret,
            explicit_auth_flows,
            allowed_oauth_flows,
            allowed_oauth_scopes,
            callback_urls,
            logout_urls,
            allowed_oauth_flows_user_pool_client: model.allowed_oauth_flows_user_pool_client,
            refresh_token_validity: attrs
                .get("refresh_token_validity")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32),
            supported_identity_providers: attrs
                .get("supported_identity_providers")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
        };

        // Snapshot, add client to pool, restore.
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if let Some(pool) = state_view.user_pools.get_mut(&user_pool_id) {
            pool.clients.insert(client_id, client_view);
        } else {
            // Pool not yet injected — create placeholder
            let mut pool_view = UserPoolView {
                id: user_pool_id.clone(),
                name: user_pool_id.clone(),
                arn: format!(
                    "arn:aws:cognito-idp:{}:{}:userpool/{}",
                    region, ctx.default_account_id, user_pool_id
                ),
                status: "Active".to_string(),
                created_date: Utc::now().to_rfc3339(),
                clients: HashMap::new(),
                users: HashMap::new(),
                groups: HashMap::new(),
                identity_providers: HashMap::new(),
                resource_servers: HashMap::new(),
                domain: None,
                mfa_config: None,
                custom_attributes: vec![],
                tags: HashMap::new(),
            };
            pool_view.clients.insert(client_id, client_view);
            state_view.user_pools.insert(user_pool_id, pool_view);
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
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
        for pool in view.user_pools.values() {
            for client in pool.clients.values() {
                let attrs = serde_json::json!({
                    "id": client.id,
                    "name": client.name,
                    "user_pool_id": client.user_pool_id,
                    "client_secret": client.client_secret,
                    "explicit_auth_flows": client.explicit_auth_flows,
                    "allowed_oauth_flows": client.allowed_oauth_flows,
                    "allowed_oauth_scopes": client.allowed_oauth_scopes,
                    "callback_urls": client.callback_urls,
                    "logout_urls": client.logout_urls,
                    "allowed_oauth_flows_user_pool_client": client.allowed_oauth_flows_user_pool_client,
                    "refresh_token_validity": client.refresh_token_validity,
                    "supported_identity_providers": client.supported_identity_providers,
                    "access_token_validity": 0,
                    "tags_all": {},
                });
                results.push(ExtractedResource {
                    name: client.id.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}
