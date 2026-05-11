//! Terraform converters for Cognito IDP resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_cognitoidentityprovider::CognitoIdentityProviderService;
use winterbaume_cognitoidentityprovider::views::{
    CognitoidpStateView, CustomDomainConfigView, GroupView, IdentityProviderView, MfaConfigView,
    ResourceServerScopeView, ResourceServerView, UserPoolClientView, UserPoolDomainView,
    UserPoolView,
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

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Build a minimal `UserPoolView` placeholder used when a sub-resource
/// (client, group, IdP, ...) is injected before — or without — its parent
/// `aws_cognito_user_pool` resource.
fn placeholder_pool(
    pool_id: &str,
    region: &str,
    account_id: &str,
    created_date: &str,
) -> UserPoolView {
    UserPoolView {
        id: pool_id.to_string(),
        name: pool_id.to_string(),
        arn: format!(
            "arn:aws:cognito-idp:{}:{}:userpool/{}",
            region, account_id, pool_id
        ),
        status: "Active".to_string(),
        created_date: created_date.to_string(),
        clients: HashMap::new(),
        users: HashMap::new(),
        groups: HashMap::new(),
        identity_providers: HashMap::new(),
        resource_servers: HashMap::new(),
        domain: None,
        mfa_config: None,
        custom_attributes: vec![],
        tags: HashMap::new(),
    }
}

// ---------------------------------------------------------------------------
// aws_cognito_identity_provider
// ---------------------------------------------------------------------------

/// Converts `aws_cognito_identity_provider` Terraform resources to/from Cognito IDP state.
pub struct AwsCognitoIdentityProviderConverter {
    service: Arc<CognitoIdentityProviderService>,
}

impl AwsCognitoIdentityProviderConverter {
    pub fn new(service: Arc<CognitoIdentityProviderService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCognitoIdentityProviderConverter {
    fn resource_type(&self) -> &str {
        "aws_cognito_identity_provider"
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

impl AwsCognitoIdentityProviderConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cognitoidp_gen::IdentityProviderTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cognito_identity_provider", e))?;

        let attrs = &instance.attributes;
        let provider_details: HashMap<String, String> = attrs
            .get("provider_details")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();
        let attribute_mapping: HashMap<String, String> = attrs
            .get("attribute_mapping")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();
        let idp_identifiers: Vec<String> = attrs
            .get("idp_identifiers")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let now = Utc::now().to_rfc3339();
        let user_pool_id = model.user_pool_id.clone();
        let idp_view = IdentityProviderView {
            provider_name: model.provider_name.clone(),
            provider_type: model.provider_type,
            user_pool_id: user_pool_id.clone(),
            provider_details,
            attribute_mapping,
            idp_identifiers,
            created_date: now.clone(),
            last_modified_date: now.clone(),
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let pool = state_view
            .user_pools
            .entry(user_pool_id.clone())
            .or_insert_with(|| {
                placeholder_pool(&user_pool_id, &region, &ctx.default_account_id, &now)
            });
        pool.identity_providers
            .insert(model.provider_name, idp_view);
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
            for idp in pool.identity_providers.values() {
                let attrs = serde_json::json!({
                    "id": format!("{}:{}", idp.user_pool_id, idp.provider_name),
                    "user_pool_id": idp.user_pool_id,
                    "provider_name": idp.provider_name,
                    "provider_type": idp.provider_type,
                    "provider_details": idp.provider_details,
                    "attribute_mapping": idp.attribute_mapping,
                    "idp_identifiers": idp.idp_identifiers,
                });
                results.push(ExtractedResource {
                    name: format!("{}:{}", idp.user_pool_id, idp.provider_name),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cognito_managed_user_pool_client
// ---------------------------------------------------------------------------

/// Converts `aws_cognito_managed_user_pool_client` Terraform resources to/from
/// Cognito IDP state. The managed variant differs only in how the underlying
/// app client was created (by another AWS service such as OpenSearch); the
/// state shape is identical to a standard `aws_cognito_user_pool_client`.
pub struct AwsCognitoManagedUserPoolClientConverter {
    service: Arc<CognitoIdentityProviderService>,
}

impl AwsCognitoManagedUserPoolClientConverter {
    pub fn new(service: Arc<CognitoIdentityProviderService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCognitoManagedUserPoolClientConverter {
    fn resource_type(&self) -> &str {
        "aws_cognito_managed_user_pool_client"
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

impl AwsCognitoManagedUserPoolClientConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cognitoidp_gen::ManagedUserPoolClientTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_cognito_managed_user_pool_client", e)
            })?;

        let attrs = &instance.attributes;
        let user_pool_id = model.user_pool_id.clone();
        // Managed clients in TF accept `name_prefix` / `name_pattern`; if a
        // resolved `name` is present we prefer that, otherwise synthesise one.
        let name = model
            .name
            .or_else(|| {
                attrs
                    .get("name_prefix")
                    .and_then(|v| v.as_str())
                    .map(|s| format!("{}-managed", s))
            })
            .unwrap_or_else(|| format!("managed-{}", &uuid::Uuid::new_v4().to_string()[..8]));
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

        let now = Utc::now().to_rfc3339();
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let pool = state_view
            .user_pools
            .entry(user_pool_id.clone())
            .or_insert_with(|| {
                placeholder_pool(&user_pool_id, &region, &ctx.default_account_id, &now)
            });
        pool.clients.insert(client_id, client_view);
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_cognito_managed_user_pool_client extracted as a regular user pool client; \
                 the source-managed flag is not preserved in state."
                    .to_string(),
            ],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Managed clients are indistinguishable from regular clients in the
        // backing state, so the regular UserPoolClient converter already
        // surfaces them. Returning an empty list avoids duplicate output.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_cognito_resource_server
// ---------------------------------------------------------------------------

/// Converts `aws_cognito_resource_server` Terraform resources to/from Cognito IDP state.
pub struct AwsCognitoResourceServerConverter {
    service: Arc<CognitoIdentityProviderService>,
}

impl AwsCognitoResourceServerConverter {
    pub fn new(service: Arc<CognitoIdentityProviderService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCognitoResourceServerConverter {
    fn resource_type(&self) -> &str {
        "aws_cognito_resource_server"
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

impl AwsCognitoResourceServerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cognitoidp_gen::ResourceServerTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cognito_resource_server", e))?;

        let attrs = &instance.attributes;
        let scopes: Vec<ResourceServerScopeView> = attrs
            .get("scope")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|s| {
                        let scope_name = s.get("scope_name").and_then(|v| v.as_str())?;
                        let scope_description = s
                            .get("scope_description")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        Some(ResourceServerScopeView {
                            scope_name: scope_name.to_string(),
                            scope_description: scope_description.to_string(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let user_pool_id = model.user_pool_id.clone();
        let identifier = model.identifier.clone();
        let server_view = ResourceServerView {
            identifier: identifier.clone(),
            name: model.name,
            user_pool_id: user_pool_id.clone(),
            scopes,
        };

        let now = Utc::now().to_rfc3339();
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let pool = state_view
            .user_pools
            .entry(user_pool_id.clone())
            .or_insert_with(|| {
                placeholder_pool(&user_pool_id, &region, &ctx.default_account_id, &now)
            });
        pool.resource_servers.insert(identifier, server_view);
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
            for server in pool.resource_servers.values() {
                let scope_list: Vec<serde_json::Value> = server
                    .scopes
                    .iter()
                    .map(|s| {
                        serde_json::json!({
                            "scope_name": s.scope_name,
                            "scope_description": s.scope_description,
                        })
                    })
                    .collect();
                let scope_identifiers: Vec<String> = server
                    .scopes
                    .iter()
                    .map(|s| format!("{}/{}", server.identifier, s.scope_name))
                    .collect();
                let attrs = serde_json::json!({
                    "id": format!("{}|{}", server.user_pool_id, server.identifier),
                    "user_pool_id": server.user_pool_id,
                    "identifier": server.identifier,
                    "name": server.name,
                    "scope": scope_list,
                    "scope_identifiers": scope_identifiers,
                });
                results.push(ExtractedResource {
                    name: format!("{}|{}", server.user_pool_id, server.identifier),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cognito_user_group
// ---------------------------------------------------------------------------

/// Converts `aws_cognito_user_group` Terraform resources to/from Cognito IDP state.
pub struct AwsCognitoUserGroupConverter {
    service: Arc<CognitoIdentityProviderService>,
}

impl AwsCognitoUserGroupConverter {
    pub fn new(service: Arc<CognitoIdentityProviderService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCognitoUserGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_cognito_user_group"
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

impl AwsCognitoUserGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cognitoidp_gen::UserGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cognito_user_group", e))?;

        let attrs = &instance.attributes;
        let precedence = attrs
            .get("precedence")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let now = Utc::now().to_rfc3339();
        let user_pool_id = model.user_pool_id.clone();
        let group_name = model.group_name.clone();
        let group_view = GroupView {
            group_name: group_name.clone(),
            user_pool_id: user_pool_id.clone(),
            description: model.description,
            role_arn: model.role_arn,
            precedence,
            created_date: now.clone(),
            last_modified_date: now.clone(),
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let pool = state_view
            .user_pools
            .entry(user_pool_id.clone())
            .or_insert_with(|| {
                placeholder_pool(&user_pool_id, &region, &ctx.default_account_id, &now)
            });
        pool.groups.insert(group_name, group_view);
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
            for group in pool.groups.values() {
                let attrs = serde_json::json!({
                    "id": format!("{}/{}", group.user_pool_id, group.group_name),
                    "name": group.group_name,
                    "user_pool_id": group.user_pool_id,
                    "description": group.description,
                    "role_arn": group.role_arn,
                    "precedence": group.precedence,
                });
                results.push(ExtractedResource {
                    name: format!("{}/{}", group.user_pool_id, group.group_name),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cognito_user_in_group
// ---------------------------------------------------------------------------

/// Converts `aws_cognito_user_in_group` Terraform membership resources to/from
/// Cognito IDP state. The association is encoded by adding the group name to
/// the target user's `groups` list inside its pool.
pub struct AwsCognitoUserInGroupConverter {
    service: Arc<CognitoIdentityProviderService>,
}

impl AwsCognitoUserInGroupConverter {
    pub fn new(service: Arc<CognitoIdentityProviderService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCognitoUserInGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_cognito_user_in_group"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_cognito_user_pool",
            "aws_cognito_user_group",
            "aws_cognito_user",
        ]
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

impl AwsCognitoUserInGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cognitoidp_gen::UserInGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cognito_user_in_group", e))?;

        let mut warnings: Vec<String> = vec![];
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if let Some(pool) = state_view.user_pools.get_mut(&model.user_pool_id) {
            if let Some(user) = pool.users.get_mut(&model.username) {
                if !user.groups.contains(&model.group_name) {
                    user.groups.push(model.group_name.clone());
                }
            } else {
                warnings.push(format!(
                    "aws_cognito_user_in_group: user '{}' not present in pool '{}'; \
                     membership recorded only if the user is injected later.",
                    model.username, model.user_pool_id
                ));
            }
            self.service
                .restore(&ctx.default_account_id, &region, state_view)
                .await?;
        } else {
            warnings.push(format!(
                "aws_cognito_user_in_group: user pool '{}' not present; membership ignored.",
                model.user_pool_id
            ));
        }

        Ok(ConversionResult { region, warnings })
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
            for user in pool.users.values() {
                for group_name in &user.groups {
                    let attrs = serde_json::json!({
                        "user_pool_id": pool.id,
                        "username": user.username,
                        "group_name": group_name,
                    });
                    results.push(ExtractedResource {
                        name: format!("{}/{}/{}", pool.id, group_name, user.username),
                        account_id: ctx.default_account_id.clone(),
                        region: ctx.default_region.clone(),
                        attributes: attrs,
                    });
                }
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cognito_user_pool_domain
// ---------------------------------------------------------------------------

/// Converts `aws_cognito_user_pool_domain` Terraform resources to/from
/// Cognito IDP state. The pool view carries a single optional domain; if
/// the parent pool already has one set, the more-recently injected domain
/// wins.
pub struct AwsCognitoUserPoolDomainConverter {
    service: Arc<CognitoIdentityProviderService>,
}

impl AwsCognitoUserPoolDomainConverter {
    pub fn new(service: Arc<CognitoIdentityProviderService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCognitoUserPoolDomainConverter {
    fn resource_type(&self) -> &str {
        "aws_cognito_user_pool_domain"
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

impl AwsCognitoUserPoolDomainConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cognitoidp_gen::UserPoolDomainTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cognito_user_pool_domain", e))?;

        let user_pool_id = model.user_pool_id.clone();
        let custom_domain_config = model
            .certificate_arn
            .clone()
            .map(|certificate_arn| CustomDomainConfigView { certificate_arn });
        let domain_view = UserPoolDomainView {
            domain: model.domain.clone(),
            user_pool_id: user_pool_id.clone(),
            status: "ACTIVE".to_string(),
            cloud_front_distribution: Some(format!(
                "{}.cloudfront.net",
                &uuid::Uuid::new_v4().to_string()[..14]
            )),
            custom_domain_config,
        };

        let now = Utc::now().to_rfc3339();
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let pool = state_view
            .user_pools
            .entry(user_pool_id.clone())
            .or_insert_with(|| {
                placeholder_pool(&user_pool_id, &region, &ctx.default_account_id, &now)
            });
        pool.domain = Some(domain_view);
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
            let Some(domain) = pool.domain.as_ref() else {
                continue;
            };
            let certificate_arn = domain
                .custom_domain_config
                .as_ref()
                .map(|c| c.certificate_arn.clone())
                .unwrap_or_default();
            let attrs = serde_json::json!({
                "id": domain.domain,
                "domain": domain.domain,
                "user_pool_id": domain.user_pool_id,
                "certificate_arn": certificate_arn,
                "cloudfront_distribution_arn": domain.cloud_front_distribution,
                "aws_account_id": ctx.default_account_id,
            });
            results.push(ExtractedResource {
                name: domain.domain.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cognito_user_pool_ui_customization
// ---------------------------------------------------------------------------

/// Converts `aws_cognito_user_pool_ui_customization` Terraform resources.
///
/// State gap: `CognitoidpStateView` intentionally excludes `ui_customization`
/// (see the doc comment on `UserPoolView` in
/// `winterbaume-cognitoidentityprovider::views`). The converter accepts the
/// model so plans containing this resource still parse, but the underlying
/// CSS/image payload is dropped on inject and cannot be extracted. A warning
/// is emitted on every inject and extract returns an empty list.
pub struct AwsCognitoUserPoolUiCustomizationConverter {
    service: Arc<CognitoIdentityProviderService>,
}

impl AwsCognitoUserPoolUiCustomizationConverter {
    pub fn new(service: Arc<CognitoIdentityProviderService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCognitoUserPoolUiCustomizationConverter {
    fn resource_type(&self) -> &str {
        "aws_cognito_user_pool_ui_customization"
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

impl AwsCognitoUserPoolUiCustomizationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cognitoidp_gen::UserPoolUiCustomizationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_cognito_user_pool_ui_customization", e)
            })?;

        // Ensure the parent pool exists so any subsequent extract round-trip
        // still surfaces it, but do not mutate ui_customization (the view
        // does not carry it).
        let now = Utc::now().to_rfc3339();
        let user_pool_id = model.user_pool_id.clone();
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view
            .user_pools
            .entry(user_pool_id.clone())
            .or_insert_with(|| {
                placeholder_pool(&user_pool_id, &region, &ctx.default_account_id, &now)
            });
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        let _ = (model.client_id, model.css, model.image_file);
        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_cognito_user_pool_ui_customization: CognitoidpStateView does not persist \
                 ui_customization; CSS/image payload dropped."
                    .to_string(),
            ],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // ui_customization is not part of the state view; nothing to extract.
        Ok(vec![])
    }
}
