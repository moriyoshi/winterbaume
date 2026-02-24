//! Terraform converters for API Gateway resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_apigateway::ApiGatewayService;
use winterbaume_apigateway::views::{
    ApiGatewayStateView, ApiKeyView, DeploymentView, MethodView, ResourceView, RestApiView,
    StageView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{
    extract_region, extract_tags, optional_bool, optional_i64, optional_str, require_str,
};

fn minimal_state_view() -> ApiGatewayStateView {
    ApiGatewayStateView::default()
}

// ---------------------------------------------------------------------------
// aws_api_gateway_rest_api
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayRestApiConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayRestApiConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayRestApiConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_rest_api"
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

impl AwsApiGatewayRestApiConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let id = require_str(attrs, "id", "aws_api_gateway_rest_api")?;
        let name = require_str(attrs, "name", "aws_api_gateway_rest_api")?;
        let description = optional_str(attrs, "description");
        let api_key_source = optional_str(attrs, "api_key_source");
        let disable_execute_api_endpoint = optional_bool(attrs, "disable_execute_api_endpoint");
        let policy = optional_str(attrs, "policy");
        let minimum_compression_size =
            optional_i64(attrs, "minimum_compression_size").map(|v| v as i32);
        let binary_media_types: Vec<String> = attrs
            .get("binary_media_types")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let endpoint_types: Vec<String> = attrs
            .get("endpoint_configuration")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|v| v.get("types"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let vpc_endpoint_ids: Vec<String> = attrs
            .get("endpoint_configuration")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|v| v.get("vpc_endpoint_ids"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let root_resource_id = optional_str(attrs, "root_resource_id").unwrap_or_default();
        let tags = extract_tags(attrs);

        let rest_api_view = RestApiView {
            id: id.to_string(),
            name: name.to_string(),
            description,
            version: optional_str(attrs, "version"),
            created_date: 0.0,
            endpoint_types,
            vpc_endpoint_ids,
            tags,
            disable_execute_api_endpoint,
            policy,
            api_key_source,
            binary_media_types,
            minimum_compression_size,
            root_resource_id,
        };

        let mut state_view = minimal_state_view();
        state_view.rest_apis.insert(id.to_string(), rest_api_view);
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
        for api in view.rest_apis.values() {
            let endpoint_config = if !api.endpoint_types.is_empty() {
                serde_json::json!([{
                    "types": api.endpoint_types,
                    "vpc_endpoint_ids": api.vpc_endpoint_ids,
                }])
            } else {
                serde_json::json!([])
            };
            let attrs = serde_json::json!({
                "id": api.id,
                "name": api.name,
                "description": api.description,
                "version": api.version,
                "created_date": api.created_date,
                "endpoint_configuration": endpoint_config,
                "tags": api.tags,
                "disable_execute_api_endpoint": api.disable_execute_api_endpoint,
                "policy": api.policy,
                "api_key_source": api.api_key_source,
                "binary_media_types": api.binary_media_types,
                "minimum_compression_size": api.minimum_compression_size,
                "root_resource_id": api.root_resource_id,
            });
            results.push(ExtractedResource {
                name: api.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_resource
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayResourceConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayResourceConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayResourceConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_resource"
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

impl AwsApiGatewayResourceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let id = require_str(attrs, "id", "aws_api_gateway_resource")?;
        let rest_api_id = require_str(attrs, "rest_api_id", "aws_api_gateway_resource")?;
        let parent_id = optional_str(attrs, "parent_id");
        let path_part = optional_str(attrs, "path_part");
        let path = optional_str(attrs, "path")
            .unwrap_or_else(|| path_part.clone().unwrap_or_else(|| "/".to_string()));

        let resource_view = ResourceView {
            api_id: rest_api_id.to_string(),
            id: id.to_string(),
            parent_id,
            path_part,
            path,
            methods: HashMap::new(),
        };

        let key = format!("{}/{}", rest_api_id, id);
        let mut state_view = minimal_state_view();
        state_view.resources.insert(key, resource_view);
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
        for res in view.resources.values() {
            let attrs = serde_json::json!({
                "id": res.id,
                "rest_api_id": res.api_id,
                "parent_id": res.parent_id,
                "path_part": res.path_part,
                "path": res.path,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", res.api_id, res.id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_method
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayMethodConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayMethodConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayMethodConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_method"
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

impl AwsApiGatewayMethodConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let rest_api_id = require_str(attrs, "rest_api_id", "aws_api_gateway_method")?;
        let resource_id = require_str(attrs, "resource_id", "aws_api_gateway_method")?;
        let http_method = require_str(attrs, "http_method", "aws_api_gateway_method")?;
        let authorization =
            optional_str(attrs, "authorization").unwrap_or_else(|| "NONE".to_string());
        let authorizer_id = optional_str(attrs, "authorizer_id");
        let api_key_required = optional_bool(attrs, "api_key_required");
        let operation_name = optional_str(attrs, "operation_name");
        let request_validator_id = optional_str(attrs, "request_validator_id");

        let request_models: HashMap<String, String> = attrs
            .get("request_models")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let request_parameters: HashMap<String, bool> = attrs
            .get("request_parameters")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_bool().map(|b| (k.clone(), b)))
                    .collect()
            })
            .unwrap_or_default();

        let method_view = MethodView {
            http_method: http_method.to_string(),
            authorization_type: authorization,
            authorizer_id,
            api_key_required,
            operation_name,
            request_models,
            request_parameters,
            request_validator_id,
            method_responses: HashMap::new(),
            integration: None,
        };

        // Build a ResourceView with just this method so the merge adds it
        let resource_view = ResourceView {
            api_id: rest_api_id.to_string(),
            id: resource_id.to_string(),
            parent_id: None,
            path_part: None,
            path: String::new(),
            methods: HashMap::from([(http_method.to_string(), method_view)]),
        };

        let key = format!("{}/{}", rest_api_id, resource_id);
        let mut state_view = minimal_state_view();
        state_view.resources.insert(key, resource_view);
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
        for res in view.resources.values() {
            for method in res.methods.values() {
                let attrs = serde_json::json!({
                    "id": format!("agm-{}-{}-{}", res.api_id, res.id, method.http_method),
                    "rest_api_id": res.api_id,
                    "resource_id": res.id,
                    "http_method": method.http_method,
                    "authorization": method.authorization_type,
                    "authorizer_id": method.authorizer_id,
                    "api_key_required": method.api_key_required,
                    "operation_name": method.operation_name,
                    "request_models": method.request_models,
                    "request_parameters": method.request_parameters,
                    "request_validator_id": method.request_validator_id,
                });
                results.push(ExtractedResource {
                    name: format!("{}/{}/{}", res.api_id, res.id, method.http_method),
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
// aws_api_gateway_stage
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayStageConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayStageConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayStageConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_stage"
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

impl AwsApiGatewayStageConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let rest_api_id = require_str(attrs, "rest_api_id", "aws_api_gateway_stage")?;
        let stage_name = require_str(attrs, "stage_name", "aws_api_gateway_stage")?;
        let deployment_id = require_str(attrs, "deployment_id", "aws_api_gateway_stage")?;
        let description = optional_str(attrs, "description");
        let tracing_enabled = optional_bool(attrs, "xray_tracing_enabled");
        let cache_cluster_enabled = optional_bool(attrs, "cache_cluster_enabled");
        let cache_cluster_size = optional_str(attrs, "cache_cluster_size");
        let documentation_version = optional_str(attrs, "documentation_version");
        let access_log_settings = attrs.get("access_log_settings").cloned();
        let canary_settings_stage = attrs.get("canary_settings").cloned();
        let tags = extract_tags(attrs);

        let variables: HashMap<String, String> = attrs
            .get("variables")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let stage_view = StageView {
            rest_api_id: rest_api_id.to_string(),
            stage_name: stage_name.to_string(),
            deployment_id: deployment_id.to_string(),
            description,
            created_date: 0.0,
            last_updated_date: 0.0,
            tags,
            tracing_enabled,
            variables,
            cache_cluster_enabled,
            cache_cluster_size,
            documentation_version,
            access_log_settings,
            canary_settings: canary_settings_stage,
        };

        let key = format!("{}/{}", rest_api_id, stage_name);
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
            let attrs = serde_json::json!({
                "id": format!("ags-{}-{}", stage.rest_api_id, stage.stage_name),
                "rest_api_id": stage.rest_api_id,
                "stage_name": stage.stage_name,
                "deployment_id": stage.deployment_id,
                "description": stage.description,
                "created_date": stage.created_date,
                "last_updated_date": stage.last_updated_date,
                "tags": stage.tags,
                "xray_tracing_enabled": stage.tracing_enabled,
                "variables": stage.variables,
                "cache_cluster_enabled": stage.cache_cluster_enabled,
                "cache_cluster_size": stage.cache_cluster_size,
                "documentation_version": stage.documentation_version,
                "access_log_settings": stage.access_log_settings,
                "canary_settings": stage.canary_settings,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", stage.rest_api_id, stage.stage_name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_deployment
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayDeploymentConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayDeploymentConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayDeploymentConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_deployment"
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

impl AwsApiGatewayDeploymentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let id = require_str(attrs, "id", "aws_api_gateway_deployment")?;
        let rest_api_id = require_str(attrs, "rest_api_id", "aws_api_gateway_deployment")?;
        let description = optional_str(attrs, "description");
        let _triggers = attrs.get("triggers");
        let _stage_name = optional_str(attrs, "stage_name");
        let _stage_description = optional_str(attrs, "stage_description");
        let _variables = attrs.get("variables");
        let canary_settings = attrs.get("canary_settings").cloned();

        let deployment_view = DeploymentView {
            rest_api_id: rest_api_id.to_string(),
            id: id.to_string(),
            description,
            created_date: 0.0,
            canary_settings,
        };

        let key = format!("{}/{}", rest_api_id, id);
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
        for dep in view.deployments.values() {
            let attrs = serde_json::json!({
                "id": dep.id,
                "rest_api_id": dep.rest_api_id,
                "description": dep.description,
                "created_date": dep.created_date,
                "execution_arn": format!("arn:aws:execute-api:{}:000000000000:{}", ctx.default_region, dep.rest_api_id),
                "invoke_url": format!("https://{}.execute-api.{}.amazonaws.com/", dep.rest_api_id, ctx.default_region),
                "canary_settings": dep.canary_settings,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", dep.rest_api_id, dep.id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_api_key
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayApiKeyConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayApiKeyConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayApiKeyConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_api_key"
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

impl AwsApiGatewayApiKeyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let id = require_str(attrs, "id", "aws_api_gateway_api_key")?;
        let name = require_str(attrs, "name", "aws_api_gateway_api_key")?;
        let value = optional_str(attrs, "value").unwrap_or_default();
        let description = optional_str(attrs, "description");
        let enabled = optional_bool(attrs, "enabled").unwrap_or(true);
        let tags = extract_tags(attrs);

        let stage_keys: Vec<String> = attrs
            .get("stage_key")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        let rest_api_id = v.get("rest_api_id").and_then(|v| v.as_str())?;
                        let stage_name = v.get("stage_name").and_then(|v| v.as_str())?;
                        Some(format!("{}/{}", rest_api_id, stage_name))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let api_key_view = ApiKeyView {
            id: id.to_string(),
            name: name.to_string(),
            value,
            description,
            enabled,
            stage_keys,
            tags,
            created_date: 0.0,
        };

        let mut state_view = minimal_state_view();
        state_view.api_keys.insert(id.to_string(), api_key_view);
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
        for key in view.api_keys.values() {
            let attrs = serde_json::json!({
                "id": key.id,
                "name": key.name,
                "value": key.value,
                "description": key.description,
                "enabled": key.enabled,
                "stage_key": key.stage_keys,
                "tags": key.tags,
                "created_date": key.created_date,
            });
            results.push(ExtractedResource {
                name: key.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
