//! Terraform converters for AppSync resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_appsync::AppSyncService;
use winterbaume_appsync::views::{AppsyncStateView, GraphqlApiView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_appsync_graphql_api")?;
        let api_id = optional_str(attrs, "id")
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string().replace('-', "")[..16].to_string());
        let authentication_type =
            optional_str(attrs, "authentication_type").unwrap_or_else(|| "API_KEY".to_string());
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
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

        let _tags_all = attrs.get("tags_all");
        let _log_config = attrs.get("log_config");
        let _openid_connect_config = attrs.get("openid_connect_config");
        let user_pool_config = attrs.get("user_pool_config").cloned();
        let lambda_authorizer_config = attrs.get("lambda_authorizer_config").cloned();
        let additional_authentication_provider =
            attrs.get("additional_authentication_provider").cloned();
        let enhanced_metrics_config = attrs.get("enhanced_metrics_config").cloned();
        let _schema = optional_str(attrs, "schema");

        let mut tags: HashMap<String, String> = HashMap::new();
        if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.insert(k.clone(), s.to_string());
                }
            }
        }

        let xray_enabled = attrs
            .get("xray_enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let api_view = GraphqlApiView {
            api_id: api_id.clone(),
            name: name.to_string(),
            authentication_type,
            arn,
            uris,
            tags,
            xray_enabled,
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
