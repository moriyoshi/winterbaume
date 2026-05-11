//! Terraform converters for API Gateway V2 resources.
//!
//! `ApiTfModel` is generated from `specs/apigatewayv2.toml`. The
//! protocol_type default ("HTTP"), the api_endpoint URL template, the
//! synthesised api_id (UUID), and the constant created_date /
//! cors_configuration / api_key_selection_expression / execution_arn
//! values are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_apigatewayv2::ApiGatewayV2Service;
use winterbaume_apigatewayv2::views::{
    ApiGatewayV2StateView, ApiMappingView, ApiView, AuthorizerView, DeploymentView, DomainNameView,
    IntegrationResponseView, IntegrationView, ModelView, RouteResponseView, RouteView, StageView,
    StoredDomainNameConfigurationView, VpcLinkView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::apigatewayv2 as apigatewayv2_gen;
use crate::util::{classify_deserialize_error, extract_region};

fn minimal_state_view() -> ApiGatewayV2StateView {
    ApiGatewayV2StateView::default()
}

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

// ---------------------------------------------------------------------------
// aws_apigatewayv2_api_mapping
// ---------------------------------------------------------------------------

/// Converts `aws_apigatewayv2_api_mapping` Terraform resources.
pub struct AwsApigatewayv2ApiMappingConverter {
    service: Arc<ApiGatewayV2Service>,
}

impl AwsApigatewayv2ApiMappingConverter {
    pub fn new(service: Arc<ApiGatewayV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApigatewayv2ApiMappingConverter {
    fn resource_type(&self) -> &str {
        "aws_apigatewayv2_api_mapping"
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

impl AwsApigatewayv2ApiMappingConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigatewayv2_gen::ApiMappingTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_apigatewayv2_api_mapping", e))?;

        let api_mapping_id = model
            .id
            .clone()
            .or_else(|| model.api_mapping_id.clone())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let mapping_view = ApiMappingView {
            api_mapping_id: api_mapping_id.clone(),
            domain_name: model.domain_name.clone(),
            api_id: model.api_id,
            stage: model.stage,
            api_mapping_key: model.api_mapping_key,
        };

        let key = format!("{}/{}", model.domain_name, api_mapping_id);
        let mut state_view = minimal_state_view();
        state_view.api_mappings.insert(key, mapping_view);
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
        for mapping in view.api_mappings.values() {
            let attrs = serde_json::json!({
                "id": mapping.api_mapping_id,
                "api_mapping_id": mapping.api_mapping_id,
                "domain_name": mapping.domain_name,
                "api_id": mapping.api_id,
                "stage": mapping.stage,
                "api_mapping_key": mapping.api_mapping_key,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", mapping.domain_name, mapping.api_mapping_id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_authorizer
// ---------------------------------------------------------------------------

/// Converts `aws_apigatewayv2_authorizer` Terraform resources.
///
/// Vec<String> `identity_sources` and Option<i32>
/// `authorizer_result_ttl_in_seconds` / Option<bool> `enable_simple_responses`
/// are read raw because the spec is scalar-only.
pub struct AwsApigatewayv2AuthorizerConverter {
    service: Arc<ApiGatewayV2Service>,
}

impl AwsApigatewayv2AuthorizerConverter {
    pub fn new(service: Arc<ApiGatewayV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApigatewayv2AuthorizerConverter {
    fn resource_type(&self) -> &str {
        "aws_apigatewayv2_authorizer"
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

impl AwsApigatewayv2AuthorizerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigatewayv2_gen::ApigatewayV2AuthorizerTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_apigatewayv2_authorizer", e))?;

        let identity_source: Option<Vec<String>> = attrs
            .get("identity_sources")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            });
        let authorizer_result_ttl_in_seconds = attrs
            .get("authorizer_result_ttl_in_seconds")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let enable_simple_responses = attrs
            .get("enable_simple_responses")
            .and_then(|v| v.as_bool());

        let authorizer_id = model
            .id
            .clone()
            .or_else(|| model.authorizer_id.clone())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let authorizer_view = AuthorizerView {
            authorizer_id: authorizer_id.clone(),
            api_id: model.api_id.clone(),
            authorizer_type: model.authorizer_type,
            authorizer_uri: model.authorizer_uri,
            authorizer_credentials_arn: model.authorizer_credentials_arn,
            authorizer_payload_format_version: model.authorizer_payload_format_version,
            authorizer_result_ttl_in_seconds,
            identity_source,
            identity_validation_expression: model.identity_validation_expression,
            name: model.name,
            enable_simple_responses,
        };

        let key = format!("{}/{}", model.api_id, authorizer_id);
        let mut state_view = minimal_state_view();
        state_view.authorizers.insert(key, authorizer_view);
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
        for authorizer in view.authorizers.values() {
            let attrs = serde_json::json!({
                "id": authorizer.authorizer_id,
                "authorizer_id": authorizer.authorizer_id,
                "api_id": authorizer.api_id,
                "name": authorizer.name,
                "authorizer_type": authorizer.authorizer_type,
                "authorizer_uri": authorizer.authorizer_uri,
                "authorizer_credentials_arn": authorizer.authorizer_credentials_arn,
                "authorizer_payload_format_version": authorizer.authorizer_payload_format_version,
                "authorizer_result_ttl_in_seconds": authorizer.authorizer_result_ttl_in_seconds,
                "identity_sources": authorizer.identity_source,
                "identity_validation_expression": authorizer.identity_validation_expression,
                "enable_simple_responses": authorizer.enable_simple_responses,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", authorizer.api_id, authorizer.authorizer_id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_deployment
// ---------------------------------------------------------------------------

/// Converts `aws_apigatewayv2_deployment` Terraform resources.
pub struct AwsApigatewayv2DeploymentConverter {
    service: Arc<ApiGatewayV2Service>,
}

impl AwsApigatewayv2DeploymentConverter {
    pub fn new(service: Arc<ApiGatewayV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApigatewayv2DeploymentConverter {
    fn resource_type(&self) -> &str {
        "aws_apigatewayv2_deployment"
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

impl AwsApigatewayv2DeploymentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigatewayv2_gen::ApigatewayV2DeploymentTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_apigatewayv2_deployment", e))?;

        let deployment_id = model
            .id
            .clone()
            .or_else(|| model.deployment_id.clone())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let deployment_view = DeploymentView {
            deployment_id: deployment_id.clone(),
            api_id: model.api_id.clone(),
            deployment_status: "DEPLOYED".to_string(),
            description: model.description,
            created_date: "2024-01-01T00:00:00Z".to_string(),
        };

        let key = format!("{}/{}", model.api_id, deployment_id);
        let mut state_view = minimal_state_view();
        state_view.deployments.insert(key, deployment_view);
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
        for deployment in view.deployments.values() {
            let attrs = serde_json::json!({
                "id": deployment.deployment_id,
                "deployment_id": deployment.deployment_id,
                "api_id": deployment.api_id,
                "description": deployment.description,
                "deployment_status": deployment.deployment_status,
                "auto_deployed": false,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", deployment.api_id, deployment.deployment_id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_domain_name
// ---------------------------------------------------------------------------

/// Converts `aws_apigatewayv2_domain_name` Terraform resources.
///
/// The nested `domain_name_configuration` HCL block is read raw and
/// projected into `StoredDomainNameConfigurationView`.
pub struct AwsApigatewayv2DomainNameConverter {
    service: Arc<ApiGatewayV2Service>,
}

impl AwsApigatewayv2DomainNameConverter {
    pub fn new(service: Arc<ApiGatewayV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApigatewayv2DomainNameConverter {
    fn resource_type(&self) -> &str {
        "aws_apigatewayv2_domain_name"
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

impl AwsApigatewayv2DomainNameConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigatewayv2_gen::ApigatewayV2DomainNameTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_apigatewayv2_domain_name", e))?;

        let domain_name_configurations: Vec<StoredDomainNameConfigurationView> = attrs
            .get("domain_name_configuration")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|c| StoredDomainNameConfigurationView {
                        certificate_arn: c
                            .get("certificate_arn")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        endpoint_type: c
                            .get("endpoint_type")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        security_policy: c
                            .get("security_policy")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let domain_view = DomainNameView {
            domain_name: model.domain_name.clone(),
            tags: model.tags,
            domain_name_configurations,
        };

        let mut state_view = minimal_state_view();
        state_view
            .domain_names
            .insert(model.domain_name, domain_view);
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
        for domain in view.domain_names.values() {
            let configs: Vec<serde_json::Value> = domain
                .domain_name_configurations
                .iter()
                .map(|c| {
                    serde_json::json!({
                        "certificate_arn": c.certificate_arn,
                        "endpoint_type": c.endpoint_type,
                        "security_policy": c.security_policy,
                    })
                })
                .collect();
            let attrs = serde_json::json!({
                "id": domain.domain_name,
                "domain_name": domain.domain_name,
                "tags": domain.tags,
                "tags_all": domain.tags,
                "domain_name_configuration": configs,
            });
            results.push(ExtractedResource {
                name: domain.domain_name.replace('.', "_"),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_integration
// ---------------------------------------------------------------------------

/// Converts `aws_apigatewayv2_integration` Terraform resources.
pub struct AwsApigatewayv2IntegrationConverter {
    service: Arc<ApiGatewayV2Service>,
}

impl AwsApigatewayv2IntegrationConverter {
    pub fn new(service: Arc<ApiGatewayV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApigatewayv2IntegrationConverter {
    fn resource_type(&self) -> &str {
        "aws_apigatewayv2_integration"
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

impl AwsApigatewayv2IntegrationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigatewayv2_gen::ApigatewayV2IntegrationTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_apigatewayv2_integration", e))?;

        let integration_id = model
            .id
            .clone()
            .or_else(|| model.integration_id.clone())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let integration_view = IntegrationView {
            integration_id: integration_id.clone(),
            api_id: model.api_id.clone(),
            integration_type: model.integration_type,
            integration_uri: model.integration_uri,
            integration_method: model.integration_method,
            description: model.description,
            payload_format_version: model.payload_format_version,
            connection_type: model.connection_type,
        };

        let key = format!("{}/{}", model.api_id, integration_id);
        let mut state_view = minimal_state_view();
        state_view.integrations.insert(key, integration_view);
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
        for integration in view.integrations.values() {
            let attrs = serde_json::json!({
                "id": integration.integration_id,
                "integration_id": integration.integration_id,
                "api_id": integration.api_id,
                "integration_type": integration.integration_type,
                "integration_uri": integration.integration_uri,
                "integration_method": integration.integration_method,
                "description": integration.description,
                "payload_format_version": integration.payload_format_version,
                "connection_type": integration.connection_type,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", integration.api_id, integration.integration_id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_integration_response
// ---------------------------------------------------------------------------

/// Converts `aws_apigatewayv2_integration_response` Terraform resources.
///
/// HashMap<String, String> fields `response_parameters` / `response_templates`
/// are read raw from the attributes.
pub struct AwsApigatewayv2IntegrationResponseConverter {
    service: Arc<ApiGatewayV2Service>,
}

impl AwsApigatewayv2IntegrationResponseConverter {
    pub fn new(service: Arc<ApiGatewayV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApigatewayv2IntegrationResponseConverter {
    fn resource_type(&self) -> &str {
        "aws_apigatewayv2_integration_response"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_apigatewayv2_integration"]
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

impl AwsApigatewayv2IntegrationResponseConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigatewayv2_gen::ApigatewayV2IntegrationResponseTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_apigatewayv2_integration_response", e)
            })?;

        let response_parameters: Option<HashMap<String, String>> = attrs
            .get("response_parameters")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            });
        let response_templates: Option<HashMap<String, String>> = attrs
            .get("response_templates")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            });

        let integration_response_id = model
            .id
            .clone()
            .or_else(|| model.integration_response_id.clone())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let response_view = IntegrationResponseView {
            integration_response_id: integration_response_id.clone(),
            api_id: model.api_id.clone(),
            integration_id: model.integration_id.clone(),
            integration_response_key: model.integration_response_key,
            content_handling_strategy: model.content_handling_strategy,
            response_parameters,
            response_templates,
            template_selection_expression: model.template_selection_expression,
        };

        let key = format!(
            "{}/{}/{}",
            model.api_id, model.integration_id, integration_response_id
        );
        let mut state_view = minimal_state_view();
        state_view.integration_responses.insert(key, response_view);
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
        for resp in view.integration_responses.values() {
            let attrs = serde_json::json!({
                "id": resp.integration_response_id,
                "integration_response_id": resp.integration_response_id,
                "api_id": resp.api_id,
                "integration_id": resp.integration_id,
                "integration_response_key": resp.integration_response_key,
                "content_handling_strategy": resp.content_handling_strategy,
                "template_selection_expression": resp.template_selection_expression,
                "response_parameters": resp.response_parameters,
                "response_templates": resp.response_templates,
            });
            results.push(ExtractedResource {
                name: format!(
                    "{}/{}/{}",
                    resp.api_id, resp.integration_id, resp.integration_response_id
                ),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_model
// ---------------------------------------------------------------------------

/// Converts `aws_apigatewayv2_model` Terraform resources.
pub struct AwsApigatewayv2ModelConverter {
    service: Arc<ApiGatewayV2Service>,
}

impl AwsApigatewayv2ModelConverter {
    pub fn new(service: Arc<ApiGatewayV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApigatewayv2ModelConverter {
    fn resource_type(&self) -> &str {
        "aws_apigatewayv2_model"
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

impl AwsApigatewayv2ModelConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigatewayv2_gen::ApigatewayV2ModelTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_apigatewayv2_model", e))?;

        let model_id = model
            .id
            .clone()
            .or_else(|| model.model_id.clone())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let model_view = ModelView {
            model_id: model_id.clone(),
            api_id: model.api_id.clone(),
            name: model.name,
            content_type: model.content_type,
            description: model.description,
            schema: model.schema,
        };

        let key = format!("{}/{}", model.api_id, model_id);
        let mut state_view = minimal_state_view();
        state_view.models.insert(key, model_view);
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
        for m in view.models.values() {
            let attrs = serde_json::json!({
                "id": m.model_id,
                "model_id": m.model_id,
                "api_id": m.api_id,
                "name": m.name,
                "content_type": m.content_type,
                "description": m.description,
                "schema": m.schema,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", m.api_id, m.model_id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_route
// ---------------------------------------------------------------------------

/// Converts `aws_apigatewayv2_route` Terraform resources.
pub struct AwsApigatewayv2RouteConverter {
    service: Arc<ApiGatewayV2Service>,
}

impl AwsApigatewayv2RouteConverter {
    pub fn new(service: Arc<ApiGatewayV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApigatewayv2RouteConverter {
    fn resource_type(&self) -> &str {
        "aws_apigatewayv2_route"
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

impl AwsApigatewayv2RouteConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigatewayv2_gen::ApigatewayV2RouteTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_apigatewayv2_route", e))?;

        let route_id = model
            .id
            .clone()
            .or_else(|| model.route_id.clone())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let route_view = RouteView {
            route_id: route_id.clone(),
            api_id: model.api_id.clone(),
            route_key: model.route_key,
            target: model.target,
            authorization_type: model.authorization_type,
            authorizer_id: model.authorizer_id,
        };

        let key = format!("{}/{}", model.api_id, route_id);
        let mut state_view = minimal_state_view();
        state_view.routes.insert(key, route_view);
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
        for route in view.routes.values() {
            let attrs = serde_json::json!({
                "id": route.route_id,
                "route_id": route.route_id,
                "api_id": route.api_id,
                "route_key": route.route_key,
                "target": route.target,
                "authorization_type": route.authorization_type,
                "authorizer_id": route.authorizer_id,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", route.api_id, route.route_id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_route_response
// ---------------------------------------------------------------------------

/// Converts `aws_apigatewayv2_route_response` Terraform resources.
pub struct AwsApigatewayv2RouteResponseConverter {
    service: Arc<ApiGatewayV2Service>,
}

impl AwsApigatewayv2RouteResponseConverter {
    pub fn new(service: Arc<ApiGatewayV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApigatewayv2RouteResponseConverter {
    fn resource_type(&self) -> &str {
        "aws_apigatewayv2_route_response"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_apigatewayv2_route"]
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

impl AwsApigatewayv2RouteResponseConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigatewayv2_gen::ApigatewayV2RouteResponseTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_apigatewayv2_route_response", e))?;

        let route_response_id = model
            .id
            .clone()
            .or_else(|| model.route_response_id.clone())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let response_view = RouteResponseView {
            route_response_id: route_response_id.clone(),
            api_id: model.api_id.clone(),
            route_id: model.route_id.clone(),
            route_response_key: model.route_response_key,
            model_selection_expression: model.model_selection_expression,
        };

        let key = format!("{}/{}/{}", model.api_id, model.route_id, route_response_id);
        let mut state_view = minimal_state_view();
        state_view.route_responses.insert(key, response_view);
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
        for resp in view.route_responses.values() {
            let attrs = serde_json::json!({
                "id": resp.route_response_id,
                "route_response_id": resp.route_response_id,
                "api_id": resp.api_id,
                "route_id": resp.route_id,
                "route_response_key": resp.route_response_key,
                "model_selection_expression": resp.model_selection_expression,
            });
            results.push(ExtractedResource {
                name: format!(
                    "{}/{}/{}",
                    resp.api_id, resp.route_id, resp.route_response_id
                ),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_stage
// ---------------------------------------------------------------------------

/// Converts `aws_apigatewayv2_stage` Terraform resources.
///
/// The `auto_deploy` bool is read raw because the spec is scalar-only.
pub struct AwsApigatewayv2StageConverter {
    service: Arc<ApiGatewayV2Service>,
}

impl AwsApigatewayv2StageConverter {
    pub fn new(service: Arc<ApiGatewayV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApigatewayv2StageConverter {
    fn resource_type(&self) -> &str {
        "aws_apigatewayv2_stage"
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

impl AwsApigatewayv2StageConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigatewayv2_gen::ApigatewayV2StageTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_apigatewayv2_stage", e))?;

        let auto_deploy = attrs
            .get("auto_deploy")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let stage_view = StageView {
            stage_name: model.stage_name.clone(),
            api_id: model.api_id.clone(),
            description: model.description,
            deployment_id: model.deployment_id,
            auto_deploy,
            created_date: "2024-01-01T00:00:00Z".to_string(),
            last_updated_date: "2024-01-01T00:00:00Z".to_string(),
            tags: model.tags,
        };

        let key = format!("{}/{}", model.api_id, model.stage_name);
        let mut state_view = minimal_state_view();
        state_view.stages.insert(key, stage_view);
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
        for stage in view.stages.values() {
            let invoke_url = format!(
                "wss://{}.execute-api.{}.amazonaws.com/{}",
                stage.api_id, ctx.default_region, stage.stage_name
            );
            let attrs = serde_json::json!({
                "id": format!("{}/{}", stage.api_id, stage.stage_name),
                "name": stage.stage_name,
                "api_id": stage.api_id,
                "deployment_id": stage.deployment_id,
                "description": stage.description,
                "auto_deploy": stage.auto_deploy,
                "tags": stage.tags,
                "tags_all": stage.tags,
                "invoke_url": invoke_url,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", stage.api_id, stage.stage_name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_apigatewayv2_vpc_link
// ---------------------------------------------------------------------------

/// Converts `aws_apigatewayv2_vpc_link` Terraform resources.
///
/// Vec<String> `security_group_ids` and `subnet_ids` are read raw because
/// the spec is scalar-only.
pub struct AwsApigatewayv2VpcLinkConverter {
    service: Arc<ApiGatewayV2Service>,
}

impl AwsApigatewayv2VpcLinkConverter {
    pub fn new(service: Arc<ApiGatewayV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApigatewayv2VpcLinkConverter {
    fn resource_type(&self) -> &str {
        "aws_apigatewayv2_vpc_link"
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

impl AwsApigatewayv2VpcLinkConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigatewayv2_gen::ApigatewayV2VpcLinkTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_apigatewayv2_vpc_link", e))?;

        let security_group_ids: Vec<String> = attrs
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let subnet_ids: Vec<String> = attrs
            .get("subnet_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let vpc_link_id = model
            .id
            .clone()
            .or_else(|| model.vpc_link_id.clone())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let vpc_link_view = VpcLinkView {
            vpc_link_id: vpc_link_id.clone(),
            name: model.name,
            security_group_ids,
            subnet_ids,
            tags: model.tags,
            created_date: "2024-01-01T00:00:00Z".to_string(),
        };

        let mut state_view = minimal_state_view();
        state_view.vpc_links.insert(vpc_link_id, vpc_link_view);
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
        for vpc_link in view.vpc_links.values() {
            let attrs = serde_json::json!({
                "id": vpc_link.vpc_link_id,
                "vpc_link_id": vpc_link.vpc_link_id,
                "name": vpc_link.name,
                "security_group_ids": vpc_link.security_group_ids,
                "subnet_ids": vpc_link.subnet_ids,
                "tags": vpc_link.tags,
                "tags_all": vpc_link.tags,
            });
            results.push(ExtractedResource {
                name: vpc_link.vpc_link_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
