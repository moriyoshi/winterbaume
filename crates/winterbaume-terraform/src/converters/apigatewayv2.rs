//! Terraform converters for API Gateway V2 resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_apigatewayv2::ApiGatewayV2Service;
use winterbaume_apigatewayv2::views::{ApiGatewayV2StateView, ApiView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_apigatewayv2_api
// ---------------------------------------------------------------------------

/// Converts `aws_apigatewayv2_api` Terraform resources to/from API Gateway V2 API state.
pub struct AwsApigatewayv2ApiConverter {
    service: Arc<ApiGatewayV2Service>,
}

impl AwsApigatewayv2ApiConverter {
    pub fn new(service: Arc<ApiGatewayV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApigatewayv2ApiConverter {
    fn resource_type(&self) -> &str {
        "aws_apigatewayv2_api"
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

impl AwsApigatewayv2ApiConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_apigatewayv2_api")?;
        let protocol_type =
            optional_str(attrs, "protocol_type").unwrap_or_else(|| "HTTP".to_string());
        let region = extract_region(attrs, &ctx.default_region);

        let api_id = optional_str(attrs, "id")
            .or_else(|| optional_str(attrs, "api_id"))
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let description = optional_str(attrs, "description");
        let route_selection_expression = optional_str(attrs, "route_selection_expression");
        let tags = extract_tags(attrs);

        let _tags_all = attrs.get("tags_all");
        let _cors_configuration = attrs.get("cors_configuration");
        let _disable_execute_api_endpoint = attrs.get("disable_execute_api_endpoint");
        let _version = optional_str(attrs, "version");
        let _body = optional_str(attrs, "body");
        let _fail_on_warnings = attrs.get("fail_on_warnings");

        let api_endpoint = optional_str(attrs, "api_endpoint")
            .unwrap_or_else(|| format!("https://{api_id}.execute-api.{region}.amazonaws.com"));

        let api_view = ApiView {
            api_id: api_id.clone(),
            name: name.to_string(),
            protocol_type,
            route_selection_expression,
            description,
            api_endpoint,
            created_date: "2024-01-01T00:00:00Z".to_string(),
            tags,
        };

        let view = ApiGatewayV2StateView {
            apis: std::iter::once((api_id, api_view)).collect(),
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
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        let mut resources = Vec::new();
        for (api_id, api) in &snapshot.apis {
            let execution_arn = format!(
                "arn:aws:execute-api:{}:{}:{}",
                region, ctx.default_account_id, api_id
            );
            let attrs = serde_json::json!({
                "id": api_id,
                "api_id": api_id,
                "name": api.name,
                "protocol_type": api.protocol_type,
                "api_endpoint": api.api_endpoint,
                "description": api.description,
                "route_selection_expression": api.route_selection_expression,
                "created_date": api.created_date,
                "tags": api.tags,
                "tags_all": api.tags,
                "execution_arn": execution_arn,
                "cors_configuration": null,
                "api_key_selection_expression": "$request.header.x-api-key",
            });

            resources.push(ExtractedResource {
                name: api.name.replace(' ', "_").to_lowercase(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }

        Ok(resources)
    }
}
