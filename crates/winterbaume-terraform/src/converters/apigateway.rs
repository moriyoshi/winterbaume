//! Terraform converters for API Gateway resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_apigateway::ApiGatewayService;
use winterbaume_apigateway::views::{
    AccountView, ApiGatewayStateView, ApiKeyView, AuthorizerView, BasePathMappingView,
    ClientCertificateView, DeploymentView, DocumentationPartView, DocumentationVersionView,
    DomainNameAccessAssociationView, DomainNameView, GatewayResponseView, IntegrationResponseView,
    IntegrationView, MethodResponseView, MethodView, ModelView, QuotaSettingsView,
    RequestValidatorView, ResourceView, RestApiView, StageView, ThrottleSettingsView,
    UsagePlanApiStageView, UsagePlanKeyView, UsagePlanView, VpcLinkView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::apigateway as apigateway_gen;
use crate::util::{
    classify_deserialize_error, extract_region, extract_tags, optional_bool, optional_str,
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
        let model: apigateway_gen::RestApiTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_api_gateway_rest_api", e))?;

        // disable_execute_api_endpoint must remain Option<bool> (None when absent),
        // but the generated model deserialises it as a plain bool. Read raw to
        // preserve None-vs-false distinction.
        let disable_execute_api_endpoint = optional_bool(attrs, "disable_execute_api_endpoint");

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

        let id = model.id.clone();
        let minimum_compression_size = attrs
            .get("minimum_compression_size")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let root_resource_id = model.root_resource_id.unwrap_or_default();

        let rest_api_view = RestApiView {
            id: id.clone(),
            name: model.name,
            description: model.description,
            version: model.version,
            created_date: 0.0,
            endpoint_types,
            vpc_endpoint_ids,
            tags: extract_tags(attrs),
            disable_execute_api_endpoint,
            policy: model.policy,
            api_key_source: model.api_key_source,
            binary_media_types,
            minimum_compression_size,
            root_resource_id,
        };

        let mut state_view = minimal_state_view();
        state_view.rest_apis.insert(id, rest_api_view);
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
        let model: apigateway_gen::ApiGatewayResourceTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_resource", e))?;

        let id = model.id.clone();
        let rest_api_id = model.rest_api_id.clone();
        let path_part = model.path_part.clone();
        let path = model
            .path
            .unwrap_or_else(|| path_part.clone().unwrap_or_else(|| "/".to_string()));

        let resource_view = ResourceView {
            api_id: rest_api_id.clone(),
            id: id.clone(),
            parent_id: model.parent_id,
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
        let model: apigateway_gen::ApiGatewayMethodTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_api_gateway_method", e))?;

        let rest_api_id = model.rest_api_id.clone();
        let resource_id = model.resource_id.clone();
        let http_method = model.http_method.clone();
        let authorization = model.authorization.unwrap_or_else(|| "NONE".to_string());
        // api_key_required must remain Option<bool> (None when absent).
        let api_key_required = optional_bool(attrs, "api_key_required");

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
            http_method: http_method.clone(),
            authorization_type: authorization,
            authorizer_id: model.authorizer_id,
            api_key_required,
            operation_name: model.operation_name,
            request_models,
            request_parameters,
            request_validator_id: model.request_validator_id,
            method_responses: HashMap::new(),
            integration: None,
        };

        // Build a ResourceView with just this method so the merge adds it
        let resource_view = ResourceView {
            api_id: rest_api_id.clone(),
            id: resource_id.clone(),
            parent_id: None,
            path_part: None,
            path: String::new(),
            methods: HashMap::from([(http_method, method_view)]),
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
        let model: apigateway_gen::ApiGatewayStageTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_stage", e))?;

        // Bool fields stay Option<bool> in the view.
        let tracing_enabled = optional_bool(attrs, "xray_tracing_enabled");
        let cache_cluster_enabled = optional_bool(attrs, "cache_cluster_enabled");

        let access_log_settings = attrs.get("access_log_settings").cloned();
        let canary_settings_stage = attrs.get("canary_settings").cloned();

        let variables: HashMap<String, String> = attrs
            .get("variables")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let rest_api_id = model.rest_api_id.clone();
        let stage_name = model.stage_name.clone();

        let stage_view = StageView {
            rest_api_id: rest_api_id.clone(),
            stage_name: stage_name.clone(),
            deployment_id: model.deployment_id,
            description: model.description,
            created_date: 0.0,
            last_updated_date: 0.0,
            tags: extract_tags(attrs),
            tracing_enabled,
            variables,
            cache_cluster_enabled,
            cache_cluster_size: model.cache_cluster_size,
            documentation_version: model.documentation_version,
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
        let model: apigateway_gen::ApiGatewayDeploymentTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_deployment", e))?;

        // Triggers/variables are TF-only lifecycle fields; preserve the original
        // discard pattern via _ bindings.
        let _triggers = attrs.get("triggers");
        let _stage_name = model.stage_name;
        let _stage_description = model.stage_description;
        let _variables = attrs.get("variables");
        let canary_settings = attrs.get("canary_settings").cloned();

        let id = model.id.clone();
        let rest_api_id = model.rest_api_id.clone();

        let deployment_view = DeploymentView {
            rest_api_id: rest_api_id.clone(),
            id: id.clone(),
            description: model.description,
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
        let model: apigateway_gen::ApiKeyTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_api_gateway_api_key", e))?;

        // Original semantics: enabled defaults to true when absent (the
        // generated model defaults to false, so override here from raw attrs).
        let enabled = optional_bool(attrs, "enabled").unwrap_or(true);

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

        let id = model.id.clone();
        let api_key_view = ApiKeyView {
            id: id.clone(),
            name: model.name,
            value: model.value.unwrap_or_default(),
            description: model.description,
            enabled,
            stage_keys,
            tags: extract_tags(attrs),
            created_date: 0.0,
        };

        let mut state_view = minimal_state_view();
        state_view.api_keys.insert(id, api_key_view);
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

// ---------------------------------------------------------------------------
// aws_api_gateway_account
// ---------------------------------------------------------------------------

/// Converts the singleton `aws_api_gateway_account` Terraform resource.
///
/// The view writes to `ApiGatewayStateView::account` (a single field, not a
/// HashMap), so the converter uses snapshot+mutate+restore.
pub struct AwsApiGatewayAccountConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayAccountConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayAccountConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_account"
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

impl AwsApiGatewayAccountConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayAccountTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_api_gateway_account", e))?;

        let features: Vec<String> = attrs
            .get("features")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let throttle = attrs
            .get("throttle_settings")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|t| ThrottleSettingsView {
                burst_limit: t
                    .get("burst_limit")
                    .and_then(|v| v.as_i64())
                    .map(|v| v as i32),
                rate_limit: t.get("rate_limit").and_then(|v| v.as_f64()),
            });

        let account_view = AccountView {
            cloudwatch_role_arn: model.cloudwatch_role_arn,
            throttle,
            features,
            api_key_version: model.api_key_version,
        };

        let mut state_view = minimal_state_view();
        state_view.account = account_view;
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
        let a = &view.account;
        // Only emit an instance if the account has been configured (i.e.
        // a cloudwatch_role_arn is set). Otherwise treat as absent.
        if a.cloudwatch_role_arn.is_none()
            && a.throttle.is_none()
            && a.features.is_empty()
            && a.api_key_version.is_none()
        {
            return Ok(vec![]);
        }
        let throttle_json = a.throttle.as_ref().map(|t| {
            serde_json::json!([{
                "burst_limit": t.burst_limit,
                "rate_limit": t.rate_limit,
            }])
        });
        let attrs = serde_json::json!({
            "id": "api-gateway-account",
            "cloudwatch_role_arn": a.cloudwatch_role_arn,
            "api_key_version": a.api_key_version,
            "features": a.features,
            "throttle_settings": throttle_json,
        });
        Ok(vec![ExtractedResource {
            name: "api-gateway-account".to_string(),
            account_id: ctx.default_account_id.clone(),
            region: ctx.default_region.clone(),
            attributes: attrs,
        }])
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_authorizer
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayAuthorizerConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayAuthorizerConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayAuthorizerConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_authorizer"
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

impl AwsApiGatewayAuthorizerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayAuthorizerTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_authorizer", e))?;

        let provider_arns: Vec<String> = attrs
            .get("provider_arns")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let authorizer_result_ttl_in_seconds = attrs
            .get("authorizer_result_ttl_in_seconds")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let rest_api_id = model.rest_api_id.clone();
        let id = model.id.clone();
        let authorizer_view = AuthorizerView {
            rest_api_id: rest_api_id.clone(),
            id: id.clone(),
            name: model.name,
            authorizer_type: model.authorizer_type.unwrap_or_else(|| "TOKEN".to_string()),
            authorizer_uri: model.authorizer_uri,
            authorizer_credentials: model.authorizer_credentials,
            identity_source: model.identity_source,
            identity_validation_expression: model.identity_validation_expression,
            authorizer_result_ttl_in_seconds,
            auth_type: None,
            provider_arns,
        };

        let key = format!("{}/{}", rest_api_id, id);
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
        for a in view.authorizers.values() {
            let attrs = serde_json::json!({
                "id": a.id,
                "rest_api_id": a.rest_api_id,
                "name": a.name,
                "type": a.authorizer_type,
                "authorizer_uri": a.authorizer_uri,
                "authorizer_credentials": a.authorizer_credentials,
                "identity_source": a.identity_source,
                "identity_validation_expression": a.identity_validation_expression,
                "authorizer_result_ttl_in_seconds": a.authorizer_result_ttl_in_seconds,
                "provider_arns": a.provider_arns,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", a.rest_api_id, a.id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_base_path_mapping
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayBasePathMappingConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayBasePathMappingConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayBasePathMappingConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_base_path_mapping"
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

impl AwsApiGatewayBasePathMappingConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayBasePathMappingTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_base_path_mapping", e))?;

        let domain_name = model.domain_name.clone();
        let base_path = model.base_path.unwrap_or_default();
        let mapping_view = BasePathMappingView {
            domain_name: domain_name.clone(),
            base_path: base_path.clone(),
            rest_api_id: model.rest_api_id,
            stage: model.stage,
        };

        let key = format!("{}/{}", domain_name, base_path);
        let mut state_view = minimal_state_view();
        state_view.base_path_mappings.insert(key, mapping_view);
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
        for m in view.base_path_mappings.values() {
            let attrs = serde_json::json!({
                "id": format!("{}/{}", m.domain_name, m.base_path),
                "domain_name": m.domain_name,
                "api_id": m.rest_api_id,
                "base_path": m.base_path,
                "stage_name": m.stage,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", m.domain_name, m.base_path),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_client_certificate
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayClientCertificateConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayClientCertificateConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayClientCertificateConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_client_certificate"
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

impl AwsApiGatewayClientCertificateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayClientCertificateTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_client_certificate", e))?;

        let id = model.id.clone();
        let cert_view = ClientCertificateView {
            client_certificate_id: id.clone(),
            description: model.description,
            pem_encoded_certificate: model.pem_encoded_certificate.unwrap_or_default(),
            created_date: 0.0,
            expiration_date: 0.0,
            tags: extract_tags(attrs),
        };

        let mut state_view = minimal_state_view();
        state_view.client_certificates.insert(id, cert_view);
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
        for c in view.client_certificates.values() {
            let attrs = serde_json::json!({
                "id": c.client_certificate_id,
                "description": c.description,
                "pem_encoded_certificate": c.pem_encoded_certificate,
                "created_date": c.created_date,
                "expiration_date": c.expiration_date,
                "tags": c.tags,
            });
            results.push(ExtractedResource {
                name: c.client_certificate_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_documentation_part
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayDocumentationPartConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayDocumentationPartConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayDocumentationPartConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_documentation_part"
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

impl AwsApiGatewayDocumentationPartConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayDocumentationPartTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_documentation_part", e))?;

        // The `location` field is a nested block in HCL; split its scalar
        // sub-fields out into the flat view.
        let location_obj = attrs
            .get("location")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));
        let location_type = location_obj
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("API")
            .to_string();
        let location_path = location_obj
            .get("path")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let location_method = location_obj
            .get("method")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let location_status_code = location_obj
            .get("status_code")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let location_name = location_obj
            .get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let rest_api_id = model.rest_api_id.clone();
        let id = model.id.clone();
        let part_view = DocumentationPartView {
            rest_api_id: rest_api_id.clone(),
            id: id.clone(),
            location_type,
            location_path,
            location_method,
            location_status_code,
            location_name,
            properties: model.properties.unwrap_or_default(),
        };

        let key = format!("{}/{}", rest_api_id, id);
        let mut state_view = minimal_state_view();
        state_view.documentation_parts.insert(key, part_view);
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
        for d in view.documentation_parts.values() {
            let location = serde_json::json!([{
                "type": d.location_type,
                "path": d.location_path,
                "method": d.location_method,
                "status_code": d.location_status_code,
                "name": d.location_name,
            }]);
            let attrs = serde_json::json!({
                "id": d.id,
                "rest_api_id": d.rest_api_id,
                "properties": d.properties,
                "location": location,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", d.rest_api_id, d.id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_documentation_version
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayDocumentationVersionConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayDocumentationVersionConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayDocumentationVersionConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_documentation_version"
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

impl AwsApiGatewayDocumentationVersionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayDocumentationVersionTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_api_gateway_documentation_version", e)
            })?;

        let rest_api_id = model.rest_api_id.clone();
        let version = model.version.clone();
        let version_view = DocumentationVersionView {
            rest_api_id: rest_api_id.clone(),
            version: version.clone(),
            created_date: 0.0,
            description: model.description,
        };

        let key = format!("{}/{}", rest_api_id, version);
        let mut state_view = minimal_state_view();
        state_view.documentation_versions.insert(key, version_view);
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
        for d in view.documentation_versions.values() {
            let attrs = serde_json::json!({
                "id": format!("{}/{}", d.rest_api_id, d.version),
                "rest_api_id": d.rest_api_id,
                "version": d.version,
                "description": d.description,
                "created_date": d.created_date,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", d.rest_api_id, d.version),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_domain_name
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayDomainNameConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayDomainNameConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayDomainNameConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_domain_name"
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

impl AwsApiGatewayDomainNameConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayDomainNameTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_domain_name", e))?;

        let domain_name = model.domain_name.clone();
        let domain_view = DomainNameView {
            domain_name: domain_name.clone(),
            certificate_name: model.certificate_name,
            distribution_domain_name: model.distribution_domain_name,
            tags: extract_tags(attrs),
        };

        let mut state_view = minimal_state_view();
        state_view.domain_names.insert(domain_name, domain_view);
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
        for d in view.domain_names.values() {
            let attrs = serde_json::json!({
                "id": d.domain_name,
                "domain_name": d.domain_name,
                "certificate_name": d.certificate_name,
                "cloudfront_domain_name": d.distribution_domain_name,
                "tags": d.tags,
            });
            results.push(ExtractedResource {
                name: d.domain_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_domain_name_access_association
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayDomainNameAccessAssociationConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayDomainNameAccessAssociationConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayDomainNameAccessAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_domain_name_access_association"
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

impl AwsApiGatewayDomainNameAccessAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayDomainNameAccessAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_api_gateway_domain_name_access_association", e)
            })?;

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:apigateway:{}:{}:/domainnameaccessassociations/{}",
                region, ctx.default_account_id, model.access_association_source
            )
        });
        let assoc_view = DomainNameAccessAssociationView {
            arn: arn.clone(),
            domain_name_arn: model.domain_name_arn,
            access_association_source: model.access_association_source,
            access_association_source_type: model.access_association_source_type,
            tags: extract_tags(attrs),
        };

        let mut state_view = minimal_state_view();
        state_view
            .domain_name_access_associations
            .insert(arn, assoc_view);
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
        for d in view.domain_name_access_associations.values() {
            let attrs = serde_json::json!({
                "id": d.arn,
                "arn": d.arn,
                "domain_name_arn": d.domain_name_arn,
                "access_association_source": d.access_association_source,
                "access_association_source_type": d.access_association_source_type,
                "tags": d.tags,
            });
            results.push(ExtractedResource {
                name: d.arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_gateway_response
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayGatewayResponseConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayGatewayResponseConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayGatewayResponseConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_gateway_response"
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

impl AwsApiGatewayGatewayResponseConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayGatewayResponseTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_gateway_response", e))?;

        let response_parameters: HashMap<String, String> = attrs
            .get("response_parameters")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();
        let response_templates: HashMap<String, String> = attrs
            .get("response_templates")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let rest_api_id = model.rest_api_id.clone();
        let response_type = model.response_type.clone();
        let response_view = GatewayResponseView {
            rest_api_id: rest_api_id.clone(),
            response_type: response_type.clone(),
            status_code: model.status_code,
            response_parameters,
            response_templates,
        };

        let key = format!("{}/{}", rest_api_id, response_type);
        let mut state_view = minimal_state_view();
        state_view.gateway_responses.insert(key, response_view);
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
        for g in view.gateway_responses.values() {
            let attrs = serde_json::json!({
                "id": format!("aggr-{}-{}", g.rest_api_id, g.response_type),
                "rest_api_id": g.rest_api_id,
                "response_type": g.response_type,
                "status_code": g.status_code,
                "response_parameters": g.response_parameters,
                "response_templates": g.response_templates,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", g.rest_api_id, g.response_type),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_integration
// ---------------------------------------------------------------------------

/// Converts `aws_api_gateway_integration` Terraform resources.
///
/// Sub-resource of a method: the converter snapshots, attaches/replaces
/// the `integration` field on the target Method, then restores.
pub struct AwsApiGatewayIntegrationConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayIntegrationConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayIntegrationConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_integration"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_api_gateway_method"]
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

impl AwsApiGatewayIntegrationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayIntegrationTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_integration", e))?;

        let request_parameters: HashMap<String, String> = attrs
            .get("request_parameters")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();
        let request_templates: HashMap<String, String> = attrs
            .get("request_templates")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();
        let cache_key_parameters: Vec<String> = attrs
            .get("cache_key_parameters")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let timeout_in_millis = attrs
            .get("timeout_milliseconds")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let integration_view = IntegrationView {
            integration_type: model.integration_type,
            http_method: model.integration_http_method,
            uri: model.uri,
            credentials: model.credentials,
            request_parameters,
            request_templates,
            passthrough_behavior: model.passthrough_behavior,
            content_handling: model.content_handling,
            timeout_in_millis,
            cache_namespace: model.cache_namespace,
            cache_key_parameters,
            connection_type: model.connection_type,
            connection_id: model.connection_id,
            integration_responses: HashMap::new(),
        };

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let key = format!("{}/{}", model.rest_api_id, model.resource_id);
        let mut warnings = vec![];
        if let Some(resource) = view.resources.get_mut(&key) {
            if let Some(method) = resource.methods.get_mut(&model.http_method) {
                // Preserve existing integration_responses if we're replacing.
                let preserved = method
                    .integration
                    .take()
                    .map(|i| i.integration_responses)
                    .unwrap_or_default();
                let mut new_integration = integration_view;
                new_integration.integration_responses = preserved;
                method.integration = Some(new_integration);
            } else {
                warnings.push(format!(
                    "method '{}' not found on resource '{}'; integration skipped",
                    model.http_method, key
                ));
            }
        } else {
            warnings.push(format!(
                "resource '{}' not found in state; integration skipped",
                key
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

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
        for resource in view.resources.values() {
            for method in resource.methods.values() {
                if let Some(integration) = &method.integration {
                    let attrs = serde_json::json!({
                        "id": format!("agi-{}-{}-{}", resource.api_id, resource.id, method.http_method),
                        "rest_api_id": resource.api_id,
                        "resource_id": resource.id,
                        "http_method": method.http_method,
                        "type": integration.integration_type,
                        "integration_http_method": integration.http_method,
                        "uri": integration.uri,
                        "credentials": integration.credentials,
                        "passthrough_behavior": integration.passthrough_behavior,
                        "content_handling": integration.content_handling,
                        "cache_namespace": integration.cache_namespace,
                        "connection_type": integration.connection_type,
                        "connection_id": integration.connection_id,
                        "request_parameters": integration.request_parameters,
                        "request_templates": integration.request_templates,
                        "cache_key_parameters": integration.cache_key_parameters,
                        "timeout_milliseconds": integration.timeout_in_millis,
                    });
                    results.push(ExtractedResource {
                        name: format!("{}/{}/{}", resource.api_id, resource.id, method.http_method),
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
// aws_api_gateway_integration_response
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayIntegrationResponseConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayIntegrationResponseConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayIntegrationResponseConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_integration_response"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_api_gateway_integration"]
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

impl AwsApiGatewayIntegrationResponseConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayIntegrationResponseTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_api_gateway_integration_response", e)
            })?;

        let response_parameters: HashMap<String, String> = attrs
            .get("response_parameters")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();
        let response_templates: HashMap<String, String> = attrs
            .get("response_templates")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let resp_view = IntegrationResponseView {
            status_code: model.status_code.clone(),
            selection_pattern: model.selection_pattern,
            response_templates,
            response_parameters,
            content_handling: model.content_handling,
        };

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let key = format!("{}/{}", model.rest_api_id, model.resource_id);
        let mut warnings = vec![];
        if let Some(resource) = view.resources.get_mut(&key) {
            if let Some(method) = resource.methods.get_mut(&model.http_method) {
                if let Some(integration) = method.integration.as_mut() {
                    integration
                        .integration_responses
                        .insert(model.status_code.clone(), resp_view);
                } else {
                    warnings.push(format!(
                        "method '{}' on '{}' has no integration; response skipped",
                        model.http_method, key
                    ));
                }
            } else {
                warnings.push(format!(
                    "method '{}' not found on resource '{}'; integration response skipped",
                    model.http_method, key
                ));
            }
        } else {
            warnings.push(format!(
                "resource '{}' not found in state; integration response skipped",
                key
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

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
        for resource in view.resources.values() {
            for method in resource.methods.values() {
                if let Some(integration) = &method.integration {
                    for resp in integration.integration_responses.values() {
                        let attrs = serde_json::json!({
                            "id": format!(
                                "agir-{}-{}-{}-{}",
                                resource.api_id, resource.id, method.http_method, resp.status_code
                            ),
                            "rest_api_id": resource.api_id,
                            "resource_id": resource.id,
                            "http_method": method.http_method,
                            "status_code": resp.status_code,
                            "selection_pattern": resp.selection_pattern,
                            "content_handling": resp.content_handling,
                            "response_parameters": resp.response_parameters,
                            "response_templates": resp.response_templates,
                        });
                        results.push(ExtractedResource {
                            name: format!(
                                "{}/{}/{}/{}",
                                resource.api_id, resource.id, method.http_method, resp.status_code
                            ),
                            account_id: ctx.default_account_id.clone(),
                            region: ctx.default_region.clone(),
                            attributes: attrs,
                        });
                    }
                }
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_method_response
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayMethodResponseConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayMethodResponseConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayMethodResponseConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_method_response"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_api_gateway_method"]
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

impl AwsApiGatewayMethodResponseConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayMethodResponseTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_method_response", e))?;

        let response_models: HashMap<String, String> = attrs
            .get("response_models")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();
        let response_parameters: HashMap<String, bool> = attrs
            .get("response_parameters")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_bool().map(|b| (k.clone(), b)))
                    .collect()
            })
            .unwrap_or_default();

        let resp_view = MethodResponseView {
            status_code: model.status_code.clone(),
            response_models,
            response_parameters,
        };

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let key = format!("{}/{}", model.rest_api_id, model.resource_id);
        let mut warnings = vec![];
        if let Some(resource) = view.resources.get_mut(&key) {
            if let Some(method) = resource.methods.get_mut(&model.http_method) {
                method
                    .method_responses
                    .insert(model.status_code.clone(), resp_view);
            } else {
                warnings.push(format!(
                    "method '{}' not found on resource '{}'; method response skipped",
                    model.http_method, key
                ));
            }
        } else {
            warnings.push(format!(
                "resource '{}' not found in state; method response skipped",
                key
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

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
        for resource in view.resources.values() {
            for method in resource.methods.values() {
                for resp in method.method_responses.values() {
                    let attrs = serde_json::json!({
                        "id": format!(
                            "agmr-{}-{}-{}-{}",
                            resource.api_id, resource.id, method.http_method, resp.status_code
                        ),
                        "rest_api_id": resource.api_id,
                        "resource_id": resource.id,
                        "http_method": method.http_method,
                        "status_code": resp.status_code,
                        "response_models": resp.response_models,
                        "response_parameters": resp.response_parameters,
                    });
                    results.push(ExtractedResource {
                        name: format!(
                            "{}/{}/{}/{}",
                            resource.api_id, resource.id, method.http_method, resp.status_code
                        ),
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
// aws_api_gateway_model
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayModelConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayModelConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayModelConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_model"
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

impl AwsApiGatewayModelConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayModelTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_model", e))?;

        let rest_api_id = model.rest_api_id.clone();
        let name = model.name.clone();
        let id = model.id.clone();
        let model_view = ModelView {
            rest_api_id: rest_api_id.clone(),
            id,
            name: name.clone(),
            description: model.description,
            schema: model.schema,
            content_type: model.content_type,
        };

        // Stored keyed by (rest_api_id, model_name) per state map.
        let key = format!("{}/{}", rest_api_id, name);
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
                "id": m.id,
                "rest_api_id": m.rest_api_id,
                "name": m.name,
                "description": m.description,
                "schema": m.schema,
                "content_type": m.content_type,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", m.rest_api_id, m.name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_request_validator
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayRequestValidatorConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayRequestValidatorConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayRequestValidatorConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_request_validator"
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

impl AwsApiGatewayRequestValidatorConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayRequestValidatorTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_request_validator", e))?;

        // bool fields default to false when absent (TF schema default).
        let validate_request_body = optional_bool(attrs, "validate_request_body").unwrap_or(false);
        let validate_request_parameters =
            optional_bool(attrs, "validate_request_parameters").unwrap_or(false);

        let rest_api_id = model.rest_api_id.clone();
        let id = model.id.clone();
        let validator_view = RequestValidatorView {
            rest_api_id: rest_api_id.clone(),
            id: id.clone(),
            name: model.name,
            validate_request_body,
            validate_request_parameters,
        };

        let key = format!("{}/{}", rest_api_id, id);
        let mut state_view = minimal_state_view();
        state_view.request_validators.insert(key, validator_view);
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
        for v in view.request_validators.values() {
            let attrs = serde_json::json!({
                "id": v.id,
                "rest_api_id": v.rest_api_id,
                "name": v.name,
                "validate_request_body": v.validate_request_body,
                "validate_request_parameters": v.validate_request_parameters,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", v.rest_api_id, v.id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_rest_api_policy
// ---------------------------------------------------------------------------

/// Sub-resource modifier: sets the `policy` field on an existing
/// RestApiView via snapshot+mutate+restore.
pub struct AwsApiGatewayRestApiPolicyConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayRestApiPolicyConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayRestApiPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_rest_api_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_api_gateway_rest_api"]
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

impl AwsApiGatewayRestApiPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayRestApiPolicyTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_rest_api_policy", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(api) = view.rest_apis.get_mut(&model.rest_api_id) {
            api.policy = Some(model.policy);
        } else {
            warnings.push(format!(
                "rest_api '{}' not found in state; policy skipped",
                model.rest_api_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

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
        for api in view.rest_apis.values() {
            if let Some(policy) = &api.policy {
                let attrs = serde_json::json!({
                    "id": api.id,
                    "rest_api_id": api.id,
                    "policy": policy,
                });
                results.push(ExtractedResource {
                    name: api.id.clone(),
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
// aws_api_gateway_usage_plan
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayUsagePlanConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayUsagePlanConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayUsagePlanConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_usage_plan"
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

impl AwsApiGatewayUsagePlanConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayUsagePlanTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_usage_plan", e))?;

        let api_stages: Vec<UsagePlanApiStageView> = attrs
            .get("api_stages")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        let api_id = v.get("api_id").and_then(|v| v.as_str())?.to_string();
                        let stage = v.get("stage").and_then(|v| v.as_str())?.to_string();
                        Some(UsagePlanApiStageView { api_id, stage })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let throttle = attrs
            .get("throttle_settings")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|t| ThrottleSettingsView {
                burst_limit: t
                    .get("burst_limit")
                    .and_then(|v| v.as_i64())
                    .map(|v| v as i32),
                rate_limit: t.get("rate_limit").and_then(|v| v.as_f64()),
            });
        let quota = attrs
            .get("quota_settings")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|q| QuotaSettingsView {
                limit: q.get("limit").and_then(|v| v.as_i64()).map(|v| v as i32),
                offset: q.get("offset").and_then(|v| v.as_i64()).map(|v| v as i32),
                period: q
                    .get("period")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
            });

        let id = model.id.clone();
        let plan_view = UsagePlanView {
            id: id.clone(),
            name: model.name,
            description: model.description,
            api_stages,
            throttle,
            quota,
            product_code: model.product_code,
            tags: extract_tags(attrs),
        };

        let mut state_view = minimal_state_view();
        state_view.usage_plans.insert(id, plan_view);
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
        for p in view.usage_plans.values() {
            let api_stages_json: Vec<serde_json::Value> = p
                .api_stages
                .iter()
                .map(|s| serde_json::json!({"api_id": s.api_id, "stage": s.stage}))
                .collect();
            let throttle_json = p.throttle.as_ref().map(|t| {
                serde_json::json!([{
                    "burst_limit": t.burst_limit,
                    "rate_limit": t.rate_limit,
                }])
            });
            let quota_json = p.quota.as_ref().map(|q| {
                serde_json::json!([{
                    "limit": q.limit,
                    "offset": q.offset,
                    "period": q.period,
                }])
            });
            let attrs = serde_json::json!({
                "id": p.id,
                "name": p.name,
                "description": p.description,
                "product_code": p.product_code,
                "api_stages": api_stages_json,
                "throttle_settings": throttle_json,
                "quota_settings": quota_json,
                "tags": p.tags,
            });
            results.push(ExtractedResource {
                name: p.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_usage_plan_key
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayUsagePlanKeyConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayUsagePlanKeyConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayUsagePlanKeyConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_usage_plan_key"
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

impl AwsApiGatewayUsagePlanKeyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayUsagePlanKeyTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_api_gateway_usage_plan_key", e))?;

        let usage_plan_id = model.usage_plan_id.clone();
        let key_id = model.key_id.clone();
        let key_view = UsagePlanKeyView {
            usage_plan_id: usage_plan_id.clone(),
            id: key_id.clone(),
            key_type: model.key_type,
            name: model.name,
            value: model.value,
        };

        let composite = format!("{}/{}", usage_plan_id, key_id);
        let mut state_view = minimal_state_view();
        state_view.usage_plan_keys.insert(composite, key_view);
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
        for k in view.usage_plan_keys.values() {
            let attrs = serde_json::json!({
                "id": k.id,
                "usage_plan_id": k.usage_plan_id,
                "key_id": k.id,
                "key_type": k.key_type,
                "name": k.name,
                "value": k.value,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", k.usage_plan_id, k.id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_api_gateway_vpc_link
// ---------------------------------------------------------------------------

pub struct AwsApiGatewayVpcLinkConverter {
    service: Arc<ApiGatewayService>,
}

impl AwsApiGatewayVpcLinkConverter {
    pub fn new(service: Arc<ApiGatewayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApiGatewayVpcLinkConverter {
    fn resource_type(&self) -> &str {
        "aws_api_gateway_vpc_link"
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

impl AwsApiGatewayVpcLinkConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: apigateway_gen::ApiGatewayVpcLinkTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_api_gateway_vpc_link", e))?;

        let target_arns: Vec<String> = attrs
            .get("target_arns")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let id = model.id.clone();
        let link_view = VpcLinkView {
            id: id.clone(),
            name: model.name,
            description: model.description,
            target_arns,
            status: optional_str(attrs, "status").unwrap_or_else(|| "AVAILABLE".to_string()),
            status_message: optional_str(attrs, "status_message"),
            tags: extract_tags(attrs),
        };

        let mut state_view = minimal_state_view();
        state_view.vpc_links.insert(id, link_view);
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
        for v in view.vpc_links.values() {
            let attrs = serde_json::json!({
                "id": v.id,
                "name": v.name,
                "description": v.description,
                "target_arns": v.target_arns,
                "status": v.status,
                "status_message": v.status_message,
                "tags": v.tags,
            });
            results.push(ExtractedResource {
                name: v.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
