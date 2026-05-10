//! Terraform converters for API Gateway V2 resources.
//!
//! `ApiTfModel` is generated from `specs/apigatewayv2.toml`. The
//! protocol_type default ("HTTP"), the api_endpoint URL template, the
//! synthesised api_id (UUID), and the constant created_date /
//! cors_configuration / api_key_selection_expression / execution_arn
//! values are wired up here.

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
use crate::generated::apigatewayv2 as apigatewayv2_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: apigatewayv2_gen::ApiTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_apigatewayv2_api", e))?;

        let protocol_type = model.protocol_type.unwrap_or_else(|| "HTTP".to_string());
        let api_id = model
            .id
            .or(model.api_id)
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let api_endpoint = model
            .api_endpoint
            .unwrap_or_else(|| format!("https://{api_id}.execute-api.{region}.amazonaws.com"));

        let api_view = ApiView {
            api_id: api_id.clone(),
            name: model.name,
            protocol_type,
            route_selection_expression: model.route_selection_expression,
            description: model.description,
            api_endpoint,
            created_date: "2024-01-01T00:00:00Z".to_string(),
            tags: model.tags,
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
