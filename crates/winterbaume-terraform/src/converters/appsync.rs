//! Terraform converters for AppSync resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_appsync::AppSyncService;
use winterbaume_appsync::views::{
    ApiCacheView, ApiKeyView, AppsyncStateView, GraphqlApiView, TypeView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::appsync as appsync_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_appsync_graphql_api
// ---------------------------------------------------------------------------

pub struct AwsAppsyncGraphqlApiConverter {
    service: Arc<AppSyncService>,
}

impl AwsAppsyncGraphqlApiConverter {
    pub fn new(service: Arc<AppSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppsyncGraphqlApiConverter {
    fn resource_type(&self) -> &str {
        "aws_appsync_graphql_api"
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

impl AwsAppsyncGraphqlApiConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: appsync_gen::GraphqlApiTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_appsync_graphql_api", e))?;

        let name = model.name.clone();
        let api_id = model
            .id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string().replace('-', "")[..16].to_string());
        let authentication_type = model
            .authentication_type
            .unwrap_or_else(|| "API_KEY".to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:appsync:{}:{}:apis/{}",
                region, ctx.default_account_id, api_id
            )
        });

        let mut uris: HashMap<String, String> = HashMap::new();
        uris.insert(
            "GRAPHQL".to_string(),
            format!(
                "https://{}.appsync-api.{}.amazonaws.com/graphql",
                api_id, region
            ),
        );

        // Raw-attribute passthrough blobs (not part of the strongly-typed model).
        let attrs = &instance.attributes;
        let user_pool_config = attrs.get("user_pool_config").cloned();
        let lambda_authorizer_config = attrs.get("lambda_authorizer_config").cloned();
        let additional_authentication_provider =
            attrs.get("additional_authentication_provider").cloned();
        let enhanced_metrics_config = attrs.get("enhanced_metrics_config").cloned();

        let api_view = GraphqlApiView {
            api_id: api_id.clone(),
            name,
            authentication_type,
            arn,
            uris,
            tags: model.tags,
            xray_enabled: model.xray_enabled,
            additional_authentication_provider,
            lambda_authorizer_config,
            user_pool_config,
            enhanced_metrics_config,
        };

        let mut state_view = minimal_appsync_state_view();
        state_view.apis.insert(api_id, api_view);
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
        for api in view.apis.values() {
            let attrs = serde_json::json!({
                "id": api.api_id,
                "name": api.name,
                "arn": api.arn,
                "authentication_type": api.authentication_type,
                "uris": api.uris,
                "tags": api.tags,
                "tags_all": api.tags,
                "xray_enabled": api.xray_enabled,
                "schema": null,
                "log_config": [],
                "openid_connect_config": [],
                "additional_authentication_provider": api.additional_authentication_provider,
                "lambda_authorizer_config": api.lambda_authorizer_config,
                "user_pool_config": api.user_pool_config,
                "enhanced_metrics_config": api.enhanced_metrics_config,
            });
            results.push(ExtractedResource {
                name: api.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appsync_api_cache
// ---------------------------------------------------------------------------

pub struct AwsAppsyncApiCacheConverter {
    service: Arc<AppSyncService>,
}

impl AwsAppsyncApiCacheConverter {
    pub fn new(service: Arc<AppSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppsyncApiCacheConverter {
    fn resource_type(&self) -> &str {
        "aws_appsync_api_cache"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_appsync_graphql_api"]
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

impl AwsAppsyncApiCacheConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: appsync_gen::ApiCacheTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_appsync_api_cache", e))?;

        let cache_view = ApiCacheView {
            api_id: model.api_id.clone(),
            api_caching_behavior: model
                .api_caching_behavior
                .unwrap_or_else(|| "FULL_REQUEST_CACHING".to_string()),
            r#type: model.cache_type.unwrap_or_else(|| "SMALL".to_string()),
            ttl: model.ttl,
            at_rest_encryption_enabled: model.at_rest_encryption_enabled,
            transit_encryption_enabled: model.transit_encryption_enabled,
            status: "AVAILABLE".to_string(),
            health_metrics_config: model.health_metrics_config,
        };

        let mut state_view = minimal_appsync_state_view();
        state_view.api_caches.insert(model.api_id, cache_view);
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
        for cache in view.api_caches.values() {
            let attrs = serde_json::json!({
                "api_id": cache.api_id,
                "type": cache.r#type,
                "api_caching_behavior": cache.api_caching_behavior,
                "ttl": cache.ttl,
                "at_rest_encryption_enabled": cache.at_rest_encryption_enabled,
                "transit_encryption_enabled": cache.transit_encryption_enabled,
                "health_metrics_config": cache.health_metrics_config,
            });
            results.push(ExtractedResource {
                name: cache.api_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appsync_api_key
// ---------------------------------------------------------------------------

pub struct AwsAppsyncApiKeyConverter {
    service: Arc<AppSyncService>,
}

impl AwsAppsyncApiKeyConverter {
    pub fn new(service: Arc<AppSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppsyncApiKeyConverter {
    fn resource_type(&self) -> &str {
        "aws_appsync_api_key"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_appsync_graphql_api"]
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

impl AwsAppsyncApiKeyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: appsync_gen::ApiKeyTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_appsync_api_key", e))?;

        // `expires` on the TF side is an RFC3339 timestamp string; the
        // service crate stores it as an i64 epoch. Read raw to recover an
        // i64 fallback (0 if absent/unparsable).
        let expires = attrs
            .get("expires")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();

        let id = model
            .id
            .clone()
            .or_else(|| model.key_id.clone())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()[..8].to_string());

        let key_view = ApiKeyView {
            id: id.clone(),
            api_id: model.api_id.clone(),
            description: model.description,
            expires,
            deletes: expires.saturating_add(60 * 60 * 24 * 2),
        };

        let mut state_view = minimal_appsync_state_view();
        state_view.api_keys.insert(model.api_id, vec![key_view]);
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
        for keys in view.api_keys.values() {
            for key in keys {
                let attrs = serde_json::json!({
                    "id": key.id,
                    "api_id": key.api_id,
                    "key_id": key.id,
                    "description": key.description,
                    "expires": key.expires,
                });
                results.push(ExtractedResource {
                    name: format!("{}/{}", key.api_id, key.id),
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
// aws_appsync_type
// ---------------------------------------------------------------------------

pub struct AwsAppsyncTypeConverter {
    service: Arc<AppSyncService>,
}

impl AwsAppsyncTypeConverter {
    pub fn new(service: Arc<AppSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppsyncTypeConverter {
    fn resource_type(&self) -> &str {
        "aws_appsync_type"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_appsync_graphql_api"]
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

impl AwsAppsyncTypeConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: appsync_gen::AppsyncTypeTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_appsync_type", e))?;

        let name = model.name.clone().unwrap_or_default();
        let format = model.format.clone().unwrap_or_else(|| "SDL".to_string());
        let arn = model.arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:appsync:{}:{}:apis/{}/types/{}",
                region, ctx.default_account_id, model.api_id, name
            )
        });

        let type_view = TypeView {
            api_id: model.api_id.clone(),
            name,
            definition: model.definition,
            format,
            arn,
        };

        let mut state_view = minimal_appsync_state_view();
        state_view.types.insert(model.api_id, vec![type_view]);
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
        for types in view.types.values() {
            for t in types {
                let attrs = serde_json::json!({
                    "id": format!("{}/{}", t.api_id, t.name),
                    "api_id": t.api_id,
                    "name": t.name,
                    "definition": t.definition,
                    "format": t.format,
                    "arn": t.arn,
                });
                results.push(ExtractedResource {
                    name: format!("{}/{}", t.api_id, t.name),
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
// Warning-only converters
// ---------------------------------------------------------------------------
//
// The following AppSync resources have no addressable slot on
// `AppsyncStateView`: data sources, custom domain names and their API
// associations, functions, resolvers, and source-API associations are
// not modelled in the `winterbaume_appsync` state crate. The converters
// validate the HCL input (so typos still fail) and emit a single
// warning describing the state-layer gap. Extract is a no-op to avoid
// pseudo-resources next to every parent record.

macro_rules! appsync_warning_only_converter {
    (
        struct_name = $struct_name:ident,
        resource_type = $resource_type:expr,
        model_type = $model_type:ident,
        warn_msg = $warn_msg:expr $(,)?
    ) => {
        pub struct $struct_name {
            #[allow(dead_code)]
            service: Arc<AppSyncService>,
        }

        impl $struct_name {
            pub fn new(service: Arc<AppSyncService>) -> Self {
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
                let _model: appsync_gen::$model_type = serde_json::from_value(attrs.clone())
                    .map_err(|e| classify_deserialize_error($resource_type, e))?;
                Ok(ConversionResult {
                    region,
                    warnings: vec![format!("{}: {}", $resource_type, $warn_msg)],
                })
            }
        }
    };
}

appsync_warning_only_converter! {
    struct_name = AwsAppsyncDatasourceConverter,
    resource_type = "aws_appsync_datasource",
    model_type = AppsyncDatasourceTfModel,
    warn_msg = "AppsyncStateView does not expose datasource state; inject is a no-op",
}

appsync_warning_only_converter! {
    struct_name = AwsAppsyncDomainNameConverter,
    resource_type = "aws_appsync_domain_name",
    model_type = AppsyncDomainNameTfModel,
    warn_msg = "custom domain names are not modelled in winterbaume_appsync; inject is a no-op",
}

appsync_warning_only_converter! {
    struct_name = AwsAppsyncDomainNameApiAssociationConverter,
    resource_type = "aws_appsync_domain_name_api_association",
    model_type = AppsyncDomainNameApiAssociationTfModel,
    warn_msg = "domain-name/API associations are not modelled in winterbaume_appsync; inject is a no-op",
}

appsync_warning_only_converter! {
    struct_name = AwsAppsyncFunctionConverter,
    resource_type = "aws_appsync_function",
    model_type = AppsyncFunctionTfModel,
    warn_msg = "AppsyncStateView does not expose function state; inject is a no-op",
}

appsync_warning_only_converter! {
    struct_name = AwsAppsyncResolverConverter,
    resource_type = "aws_appsync_resolver",
    model_type = AppsyncResolverTfModel,
    warn_msg = "AppsyncStateView does not expose resolver state; inject is a no-op",
}

appsync_warning_only_converter! {
    struct_name = AwsAppsyncSourceApiAssociationConverter,
    resource_type = "aws_appsync_source_api_association",
    model_type = AppsyncSourceApiAssociationTfModel,
    warn_msg = "merged-API source associations are not modelled in winterbaume_appsync; inject is a no-op",
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn minimal_appsync_state_view() -> AppsyncStateView {
    AppsyncStateView {
        apis: HashMap::new(),
        event_apis: HashMap::new(),
        api_caches: HashMap::new(),
        api_keys: HashMap::new(),
        channel_namespaces: HashMap::new(),
        types: HashMap::new(),
        schema_statuses: HashMap::new(),
        resource_tags: HashMap::new(),
    }
}
