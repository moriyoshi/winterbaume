//! Serde-compatible view types for API Gateway state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ApiGatewayService;
use crate::state::ApiGatewayState;
use crate::types::{
    Account, ApiGatewayModel, ApiKey, Authorizer, BasePathMapping, ClientCertificate, Deployment,
    DocumentationPart, DocumentationVersion, DomainName, DomainNameAccessAssociation,
    EndpointConfiguration, GatewayResponse, Integration, IntegrationResponse, Method,
    MethodResponse, QuotaSettings, RequestValidator, Resource, RestApi, Stage, ThrottleSettings,
    UsagePlan, UsagePlanApiStage, UsagePlanKey, VpcLink,
};

/// Serializable view of the entire API Gateway state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiGatewayStateView {
    #[serde(default)]
    pub rest_apis: HashMap<String, RestApiView>,
    #[serde(default)]
    pub resources: HashMap<String, ResourceView>,
    #[serde(default)]
    pub stages: HashMap<String, StageView>,
    #[serde(default)]
    pub deployments: HashMap<String, DeploymentView>,
    #[serde(default)]
    pub authorizers: HashMap<String, AuthorizerView>,
    #[serde(default)]
    pub models: HashMap<String, ModelView>,
    #[serde(default)]
    pub api_keys: HashMap<String, ApiKeyView>,
    #[serde(default)]
    pub base_path_mappings: HashMap<String, BasePathMappingView>,
    #[serde(default)]
    pub domain_names: HashMap<String, DomainNameView>,
    #[serde(default)]
    pub gateway_responses: HashMap<String, GatewayResponseView>,
    #[serde(default)]
    pub request_validators: HashMap<String, RequestValidatorView>,
    #[serde(default)]
    pub usage_plans: HashMap<String, UsagePlanView>,
    #[serde(default)]
    pub usage_plan_keys: HashMap<String, UsagePlanKeyView>,
    #[serde(default)]
    pub vpc_links: HashMap<String, VpcLinkView>,
    #[serde(default)]
    pub account: AccountView,
    #[serde(default)]
    pub client_certificates: HashMap<String, ClientCertificateView>,
    #[serde(default)]
    pub documentation_parts: HashMap<String, DocumentationPartView>,
    #[serde(default)]
    pub documentation_versions: HashMap<String, DocumentationVersionView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub domain_name_access_associations: HashMap<String, DomainNameAccessAssociationView>,
}

// ── RestApi ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestApiView {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub created_date: f64,
    pub endpoint_types: Vec<String>,
    pub vpc_endpoint_ids: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub disable_execute_api_endpoint: Option<bool>,
    pub policy: Option<String>,
    pub api_key_source: Option<String>,
    #[serde(default)]
    pub binary_media_types: Vec<String>,
    pub minimum_compression_size: Option<i32>,
    pub root_resource_id: String,
}

impl From<&RestApi> for RestApiView {
    fn from(api: &RestApi) -> Self {
        let (types, vpc_ids) = api
            .endpoint_configuration
            .as_ref()
            .map(|ec| (ec.types.clone(), ec.vpc_endpoint_ids.clone()))
            .unwrap_or_default();
        Self {
            id: api.id.clone(),
            name: api.name.clone(),
            description: api.description.clone(),
            version: api.version.clone(),
            created_date: api.created_date,
            endpoint_types: types,
            vpc_endpoint_ids: vpc_ids,
            tags: api.tags.clone(),
            disable_execute_api_endpoint: api.disable_execute_api_endpoint,
            policy: api.policy.clone(),
            api_key_source: api.api_key_source.clone(),
            binary_media_types: api.binary_media_types.clone(),
            minimum_compression_size: api.minimum_compression_size,
            root_resource_id: api.root_resource_id.clone(),
        }
    }
}

impl From<RestApiView> for RestApi {
    fn from(v: RestApiView) -> Self {
        let endpoint_configuration = if v.endpoint_types.is_empty() && v.vpc_endpoint_ids.is_empty()
        {
            None
        } else {
            Some(EndpointConfiguration {
                types: v.endpoint_types,
                vpc_endpoint_ids: v.vpc_endpoint_ids,
            })
        };
        Self {
            id: v.id,
            name: v.name,
            description: v.description,
            version: v.version,
            created_date: v.created_date,
            endpoint_configuration,
            tags: v.tags,
            disable_execute_api_endpoint: v.disable_execute_api_endpoint,
            policy: v.policy,
            api_key_source: v.api_key_source,
            binary_media_types: v.binary_media_types,
            minimum_compression_size: v.minimum_compression_size,
            root_resource_id: v.root_resource_id,
        }
    }
}

// ── Resource ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceView {
    pub api_id: String,
    pub id: String,
    pub parent_id: Option<String>,
    pub path_part: Option<String>,
    pub path: String,
    #[serde(default)]
    pub methods: HashMap<String, MethodView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MethodView {
    pub http_method: String,
    pub authorization_type: String,
    pub authorizer_id: Option<String>,
    pub api_key_required: Option<bool>,
    pub operation_name: Option<String>,
    #[serde(default)]
    pub request_models: HashMap<String, String>,
    #[serde(default)]
    pub request_parameters: HashMap<String, bool>,
    pub request_validator_id: Option<String>,
    #[serde(default)]
    pub method_responses: HashMap<String, MethodResponseView>,
    pub integration: Option<IntegrationView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MethodResponseView {
    pub status_code: String,
    #[serde(default)]
    pub response_models: HashMap<String, String>,
    #[serde(default)]
    pub response_parameters: HashMap<String, bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IntegrationView {
    pub integration_type: String,
    pub http_method: Option<String>,
    pub uri: Option<String>,
    pub credentials: Option<String>,
    #[serde(default)]
    pub request_parameters: HashMap<String, String>,
    #[serde(default)]
    pub request_templates: HashMap<String, String>,
    pub passthrough_behavior: Option<String>,
    pub content_handling: Option<String>,
    pub timeout_in_millis: Option<i32>,
    pub cache_namespace: Option<String>,
    #[serde(default)]
    pub cache_key_parameters: Vec<String>,
    pub connection_type: Option<String>,
    pub connection_id: Option<String>,
    #[serde(default)]
    pub integration_responses: HashMap<String, IntegrationResponseView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IntegrationResponseView {
    pub status_code: String,
    pub selection_pattern: Option<String>,
    #[serde(default)]
    pub response_templates: HashMap<String, String>,
    #[serde(default)]
    pub response_parameters: HashMap<String, String>,
    pub content_handling: Option<String>,
}

impl From<(&str, &Resource)> for ResourceView {
    fn from((api_id, r): (&str, &Resource)) -> Self {
        Self {
            api_id: api_id.to_string(),
            id: r.id.clone(),
            parent_id: r.parent_id.clone(),
            path_part: r.path_part.clone(),
            path: r.path.clone(),
            methods: r
                .methods
                .iter()
                .map(|(k, v)| (k.clone(), MethodView::from(v)))
                .collect(),
        }
    }
}

impl From<ResourceView> for Resource {
    fn from(v: ResourceView) -> Self {
        Self {
            id: v.id,
            parent_id: v.parent_id,
            path_part: v.path_part,
            path: v.path,
            methods: v
                .methods
                .into_iter()
                .map(|(k, mv)| (k, Method::from(mv)))
                .collect(),
        }
    }
}

impl From<&Method> for MethodView {
    fn from(m: &Method) -> Self {
        Self {
            http_method: m.http_method.clone(),
            authorization_type: m.authorization_type.clone(),
            authorizer_id: m.authorizer_id.clone(),
            api_key_required: m.api_key_required,
            operation_name: m.operation_name.clone(),
            request_models: m.request_models.clone(),
            request_parameters: m.request_parameters.clone(),
            request_validator_id: m.request_validator_id.clone(),
            method_responses: m
                .method_responses
                .iter()
                .map(|(k, v)| (k.clone(), MethodResponseView::from(v)))
                .collect(),
            integration: m.integration.as_ref().map(IntegrationView::from),
        }
    }
}

impl From<MethodView> for Method {
    fn from(v: MethodView) -> Self {
        Self {
            http_method: v.http_method,
            authorization_type: v.authorization_type,
            authorizer_id: v.authorizer_id,
            api_key_required: v.api_key_required,
            operation_name: v.operation_name,
            request_models: v.request_models,
            request_parameters: v.request_parameters,
            request_validator_id: v.request_validator_id,
            method_responses: v
                .method_responses
                .into_iter()
                .map(|(k, mv)| (k, MethodResponse::from(mv)))
                .collect(),
            integration: v.integration.map(Integration::from),
        }
    }
}

impl From<&MethodResponse> for MethodResponseView {
    fn from(r: &MethodResponse) -> Self {
        Self {
            status_code: r.status_code.clone(),
            response_models: r.response_models.clone(),
            response_parameters: r.response_parameters.clone(),
        }
    }
}

impl From<MethodResponseView> for MethodResponse {
    fn from(v: MethodResponseView) -> Self {
        Self {
            status_code: v.status_code,
            response_models: v.response_models,
            response_parameters: v.response_parameters,
        }
    }
}

impl From<&Integration> for IntegrationView {
    fn from(i: &Integration) -> Self {
        Self {
            integration_type: i.integration_type.clone(),
            http_method: i.http_method.clone(),
            uri: i.uri.clone(),
            credentials: i.credentials.clone(),
            request_parameters: i.request_parameters.clone(),
            request_templates: i.request_templates.clone(),
            passthrough_behavior: i.passthrough_behavior.clone(),
            content_handling: i.content_handling.clone(),
            timeout_in_millis: i.timeout_in_millis,
            cache_namespace: i.cache_namespace.clone(),
            cache_key_parameters: i.cache_key_parameters.clone(),
            connection_type: i.connection_type.clone(),
            connection_id: i.connection_id.clone(),
            integration_responses: i
                .integration_responses
                .iter()
                .map(|(k, v)| (k.clone(), IntegrationResponseView::from(v)))
                .collect(),
        }
    }
}

impl From<IntegrationView> for Integration {
    fn from(v: IntegrationView) -> Self {
        Self {
            integration_type: v.integration_type,
            http_method: v.http_method,
            uri: v.uri,
            credentials: v.credentials,
            request_parameters: v.request_parameters,
            request_templates: v.request_templates,
            passthrough_behavior: v.passthrough_behavior,
            content_handling: v.content_handling,
            timeout_in_millis: v.timeout_in_millis,
            cache_namespace: v.cache_namespace,
            cache_key_parameters: v.cache_key_parameters,
            connection_type: v.connection_type,
            connection_id: v.connection_id,
            integration_responses: v
                .integration_responses
                .into_iter()
                .map(|(k, irv)| (k, IntegrationResponse::from(irv)))
                .collect(),
        }
    }
}

impl From<&IntegrationResponse> for IntegrationResponseView {
    fn from(r: &IntegrationResponse) -> Self {
        Self {
            status_code: r.status_code.clone(),
            selection_pattern: r.selection_pattern.clone(),
            response_templates: r.response_templates.clone(),
            response_parameters: r.response_parameters.clone(),
            content_handling: r.content_handling.clone(),
        }
    }
}

impl From<IntegrationResponseView> for IntegrationResponse {
    fn from(v: IntegrationResponseView) -> Self {
        Self {
            status_code: v.status_code,
            selection_pattern: v.selection_pattern,
            response_templates: v.response_templates,
            response_parameters: v.response_parameters,
            content_handling: v.content_handling,
        }
    }
}

// ── Stage ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageView {
    pub rest_api_id: String,
    pub stage_name: String,
    pub deployment_id: String,
    pub description: Option<String>,
    pub created_date: f64,
    pub last_updated_date: f64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub tracing_enabled: Option<bool>,
    #[serde(default)]
    pub variables: HashMap<String, String>,
    pub cache_cluster_enabled: Option<bool>,
    pub cache_cluster_size: Option<String>,
    pub documentation_version: Option<String>,
    #[serde(default)]
    pub access_log_settings: Option<serde_json::Value>,
    #[serde(default)]
    pub canary_settings: Option<serde_json::Value>,
}

impl From<&Stage> for StageView {
    fn from(s: &Stage) -> Self {
        Self {
            rest_api_id: s.rest_api_id.clone(),
            stage_name: s.stage_name.clone(),
            deployment_id: s.deployment_id.clone(),
            description: s.description.clone(),
            created_date: s.created_date,
            last_updated_date: s.last_updated_date,
            tags: s.tags.clone(),
            tracing_enabled: s.tracing_enabled,
            variables: s.variables.clone(),
            cache_cluster_enabled: s.cache_cluster_enabled,
            cache_cluster_size: s.cache_cluster_size.clone(),
            documentation_version: s.documentation_version.clone(),
            access_log_settings: None,
            canary_settings: None,
        }
    }
}

impl From<StageView> for Stage {
    fn from(v: StageView) -> Self {
        Self {
            rest_api_id: v.rest_api_id,
            stage_name: v.stage_name,
            deployment_id: v.deployment_id,
            description: v.description,
            created_date: v.created_date,
            last_updated_date: v.last_updated_date,
            tags: v.tags,
            tracing_enabled: v.tracing_enabled,
            variables: v.variables,
            cache_cluster_enabled: v.cache_cluster_enabled,
            cache_cluster_size: v.cache_cluster_size,
            documentation_version: v.documentation_version,
        }
    }
}

// ── Deployment ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentView {
    pub rest_api_id: String,
    pub id: String,
    pub description: Option<String>,
    pub created_date: f64,
    #[serde(default)]
    pub canary_settings: Option<serde_json::Value>,
}

impl From<(&str, &Deployment)> for DeploymentView {
    fn from((api_id, d): (&str, &Deployment)) -> Self {
        Self {
            rest_api_id: api_id.to_string(),
            id: d.id.clone(),
            description: d.description.clone(),
            created_date: d.created_date,
            canary_settings: None,
        }
    }
}

impl From<DeploymentView> for Deployment {
    fn from(v: DeploymentView) -> Self {
        Self {
            id: v.id,
            description: v.description,
            created_date: v.created_date,
        }
    }
}

// ── Authorizer ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizerView {
    pub rest_api_id: String,
    pub id: String,
    pub name: String,
    pub authorizer_type: String,
    pub authorizer_uri: Option<String>,
    pub authorizer_credentials: Option<String>,
    pub identity_source: Option<String>,
    pub identity_validation_expression: Option<String>,
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    pub auth_type: Option<String>,
    #[serde(default)]
    pub provider_arns: Vec<String>,
}

impl From<(&str, &Authorizer)> for AuthorizerView {
    fn from((api_id, a): (&str, &Authorizer)) -> Self {
        Self {
            rest_api_id: api_id.to_string(),
            id: a.id.clone(),
            name: a.name.clone(),
            authorizer_type: a.authorizer_type.clone(),
            authorizer_uri: a.authorizer_uri.clone(),
            authorizer_credentials: a.authorizer_credentials.clone(),
            identity_source: a.identity_source.clone(),
            identity_validation_expression: a.identity_validation_expression.clone(),
            authorizer_result_ttl_in_seconds: a.authorizer_result_ttl_in_seconds,
            auth_type: a.auth_type.clone(),
            provider_arns: a.provider_arns.clone(),
        }
    }
}

impl From<AuthorizerView> for Authorizer {
    fn from(v: AuthorizerView) -> Self {
        Self {
            id: v.id,
            name: v.name,
            authorizer_type: v.authorizer_type,
            authorizer_uri: v.authorizer_uri,
            authorizer_credentials: v.authorizer_credentials,
            identity_source: v.identity_source,
            identity_validation_expression: v.identity_validation_expression,
            authorizer_result_ttl_in_seconds: v.authorizer_result_ttl_in_seconds,
            auth_type: v.auth_type,
            provider_arns: v.provider_arns,
        }
    }
}

// ── Model ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelView {
    pub rest_api_id: String,
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub schema: Option<String>,
    pub content_type: Option<String>,
}

impl From<(&str, &ApiGatewayModel)> for ModelView {
    fn from((api_id, m): (&str, &ApiGatewayModel)) -> Self {
        Self {
            rest_api_id: api_id.to_string(),
            id: m.id.clone(),
            name: m.name.clone(),
            description: m.description.clone(),
            schema: m.schema.clone(),
            content_type: m.content_type.clone(),
        }
    }
}

impl From<ModelView> for ApiGatewayModel {
    fn from(v: ModelView) -> Self {
        Self {
            id: v.id,
            name: v.name,
            description: v.description,
            schema: v.schema,
            content_type: v.content_type,
        }
    }
}

// ── ApiKey ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyView {
    pub id: String,
    pub name: String,
    pub value: String,
    pub description: Option<String>,
    pub enabled: bool,
    #[serde(default)]
    pub stage_keys: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub created_date: f64,
}

impl From<&ApiKey> for ApiKeyView {
    fn from(k: &ApiKey) -> Self {
        Self {
            id: k.id.clone(),
            name: k.name.clone(),
            value: k.value.clone(),
            description: k.description.clone(),
            enabled: k.enabled,
            stage_keys: k.stage_keys.clone(),
            tags: k.tags.clone(),
            created_date: k.created_date,
        }
    }
}

impl From<ApiKeyView> for ApiKey {
    fn from(v: ApiKeyView) -> Self {
        Self {
            id: v.id,
            name: v.name,
            value: v.value,
            description: v.description,
            enabled: v.enabled,
            stage_keys: v.stage_keys,
            tags: v.tags,
            created_date: v.created_date,
        }
    }
}

// ── BasePathMapping ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasePathMappingView {
    pub domain_name: String,
    pub base_path: String,
    pub rest_api_id: String,
    pub stage: Option<String>,
}

impl From<(&str, &BasePathMapping)> for BasePathMappingView {
    fn from((domain_name, b): (&str, &BasePathMapping)) -> Self {
        Self {
            domain_name: domain_name.to_string(),
            base_path: b.base_path.clone(),
            rest_api_id: b.rest_api_id.clone(),
            stage: b.stage.clone(),
        }
    }
}

impl From<BasePathMappingView> for BasePathMapping {
    fn from(v: BasePathMappingView) -> Self {
        Self {
            base_path: v.base_path,
            rest_api_id: v.rest_api_id,
            stage: v.stage,
        }
    }
}

// ── DomainName ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainNameView {
    pub domain_name: String,
    pub certificate_name: Option<String>,
    pub distribution_domain_name: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&DomainName> for DomainNameView {
    fn from(d: &DomainName) -> Self {
        Self {
            domain_name: d.domain_name.clone(),
            certificate_name: d.certificate_name.clone(),
            distribution_domain_name: d.distribution_domain_name.clone(),
            tags: d.tags.clone(),
        }
    }
}

impl From<DomainNameView> for DomainName {
    fn from(v: DomainNameView) -> Self {
        Self {
            domain_name: v.domain_name,
            certificate_name: v.certificate_name,
            distribution_domain_name: v.distribution_domain_name,
            tags: v.tags,
        }
    }
}

// ── GatewayResponse ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayResponseView {
    pub rest_api_id: String,
    pub response_type: String,
    pub status_code: Option<String>,
    #[serde(default)]
    pub response_parameters: HashMap<String, String>,
    #[serde(default)]
    pub response_templates: HashMap<String, String>,
}

impl From<(&str, &GatewayResponse)> for GatewayResponseView {
    fn from((api_id, g): (&str, &GatewayResponse)) -> Self {
        Self {
            rest_api_id: api_id.to_string(),
            response_type: g.response_type.clone(),
            status_code: g.status_code.clone(),
            response_parameters: g.response_parameters.clone(),
            response_templates: g.response_templates.clone(),
        }
    }
}

impl From<GatewayResponseView> for GatewayResponse {
    fn from(v: GatewayResponseView) -> Self {
        Self {
            response_type: v.response_type,
            status_code: v.status_code,
            response_parameters: v.response_parameters,
            response_templates: v.response_templates,
        }
    }
}

// ── RequestValidator ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestValidatorView {
    pub rest_api_id: String,
    pub id: String,
    pub name: String,
    pub validate_request_body: bool,
    pub validate_request_parameters: bool,
}

impl From<(&str, &RequestValidator)> for RequestValidatorView {
    fn from((api_id, r): (&str, &RequestValidator)) -> Self {
        Self {
            rest_api_id: api_id.to_string(),
            id: r.id.clone(),
            name: r.name.clone(),
            validate_request_body: r.validate_request_body,
            validate_request_parameters: r.validate_request_parameters,
        }
    }
}

impl From<RequestValidatorView> for RequestValidator {
    fn from(v: RequestValidatorView) -> Self {
        Self {
            id: v.id,
            name: v.name,
            validate_request_body: v.validate_request_body,
            validate_request_parameters: v.validate_request_parameters,
        }
    }
}

// ── UsagePlan ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsagePlanView {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub api_stages: Vec<UsagePlanApiStageView>,
    pub throttle: Option<ThrottleSettingsView>,
    pub quota: Option<QuotaSettingsView>,
    pub product_code: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsagePlanApiStageView {
    pub api_id: String,
    pub stage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrottleSettingsView {
    pub burst_limit: Option<i32>,
    pub rate_limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaSettingsView {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub period: Option<String>,
}

impl From<&UsagePlan> for UsagePlanView {
    fn from(u: &UsagePlan) -> Self {
        Self {
            id: u.id.clone(),
            name: u.name.clone(),
            description: u.description.clone(),
            api_stages: u
                .api_stages
                .iter()
                .map(UsagePlanApiStageView::from)
                .collect(),
            throttle: u.throttle.as_ref().map(ThrottleSettingsView::from),
            quota: u.quota.as_ref().map(QuotaSettingsView::from),
            product_code: u.product_code.clone(),
            tags: u.tags.clone(),
        }
    }
}

impl From<UsagePlanView> for UsagePlan {
    fn from(v: UsagePlanView) -> Self {
        Self {
            id: v.id,
            name: v.name,
            description: v.description,
            api_stages: v
                .api_stages
                .into_iter()
                .map(UsagePlanApiStage::from)
                .collect(),
            throttle: v.throttle.map(ThrottleSettings::from),
            quota: v.quota.map(QuotaSettings::from),
            product_code: v.product_code,
            tags: v.tags,
        }
    }
}

impl From<&UsagePlanApiStage> for UsagePlanApiStageView {
    fn from(s: &UsagePlanApiStage) -> Self {
        Self {
            api_id: s.api_id.clone(),
            stage: s.stage.clone(),
        }
    }
}

impl From<UsagePlanApiStageView> for UsagePlanApiStage {
    fn from(v: UsagePlanApiStageView) -> Self {
        Self {
            api_id: v.api_id,
            stage: v.stage,
        }
    }
}

impl From<&ThrottleSettings> for ThrottleSettingsView {
    fn from(t: &ThrottleSettings) -> Self {
        Self {
            burst_limit: t.burst_limit,
            rate_limit: t.rate_limit,
        }
    }
}

impl From<ThrottleSettingsView> for ThrottleSettings {
    fn from(v: ThrottleSettingsView) -> Self {
        Self {
            burst_limit: v.burst_limit,
            rate_limit: v.rate_limit,
        }
    }
}

impl From<&QuotaSettings> for QuotaSettingsView {
    fn from(q: &QuotaSettings) -> Self {
        Self {
            limit: q.limit,
            offset: q.offset,
            period: q.period.clone(),
        }
    }
}

impl From<QuotaSettingsView> for QuotaSettings {
    fn from(v: QuotaSettingsView) -> Self {
        Self {
            limit: v.limit,
            offset: v.offset,
            period: v.period,
        }
    }
}

// ── UsagePlanKey ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsagePlanKeyView {
    pub usage_plan_id: String,
    pub id: String,
    pub key_type: String,
    pub name: Option<String>,
    pub value: Option<String>,
}

impl From<(&str, &UsagePlanKey)> for UsagePlanKeyView {
    fn from((plan_id, k): (&str, &UsagePlanKey)) -> Self {
        Self {
            usage_plan_id: plan_id.to_string(),
            id: k.id.clone(),
            key_type: k.key_type.clone(),
            name: k.name.clone(),
            value: k.value.clone(),
        }
    }
}

impl From<UsagePlanKeyView> for UsagePlanKey {
    fn from(v: UsagePlanKeyView) -> Self {
        Self {
            id: v.id,
            key_type: v.key_type,
            name: v.name,
            value: v.value,
        }
    }
}

// ── VpcLink ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcLinkView {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub target_arns: Vec<String>,
    pub status: String,
    pub status_message: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&VpcLink> for VpcLinkView {
    fn from(v: &VpcLink) -> Self {
        Self {
            id: v.id.clone(),
            name: v.name.clone(),
            description: v.description.clone(),
            target_arns: v.target_arns.clone(),
            status: v.status.clone(),
            status_message: v.status_message.clone(),
            tags: v.tags.clone(),
        }
    }
}

impl From<VpcLinkView> for VpcLink {
    fn from(v: VpcLinkView) -> Self {
        Self {
            id: v.id,
            name: v.name,
            description: v.description,
            target_arns: v.target_arns,
            status: v.status,
            status_message: v.status_message,
            tags: v.tags,
        }
    }
}

// ── Account ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccountView {
    pub cloudwatch_role_arn: Option<String>,
    pub throttle: Option<ThrottleSettingsView>,
    #[serde(default)]
    pub features: Vec<String>,
    pub api_key_version: Option<String>,
}

impl From<&Account> for AccountView {
    fn from(a: &Account) -> Self {
        Self {
            cloudwatch_role_arn: a.cloudwatch_role_arn.clone(),
            throttle: a.throttle_settings.as_ref().map(ThrottleSettingsView::from),
            features: a.features.clone(),
            api_key_version: a.api_key_version.clone(),
        }
    }
}

impl From<AccountView> for Account {
    fn from(v: AccountView) -> Self {
        Self {
            cloudwatch_role_arn: v.cloudwatch_role_arn,
            throttle_settings: v.throttle.map(ThrottleSettings::from),
            features: v.features,
            api_key_version: v.api_key_version,
        }
    }
}

// ── ClientCertificate ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCertificateView {
    pub client_certificate_id: String,
    pub description: Option<String>,
    pub pem_encoded_certificate: String,
    pub created_date: f64,
    pub expiration_date: f64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&ClientCertificate> for ClientCertificateView {
    fn from(c: &ClientCertificate) -> Self {
        Self {
            client_certificate_id: c.client_certificate_id.clone(),
            description: c.description.clone(),
            pem_encoded_certificate: c.pem_encoded_certificate.clone(),
            created_date: c.created_date,
            expiration_date: c.expiration_date,
            tags: c.tags.clone(),
        }
    }
}

impl From<ClientCertificateView> for ClientCertificate {
    fn from(v: ClientCertificateView) -> Self {
        Self {
            client_certificate_id: v.client_certificate_id,
            description: v.description,
            pem_encoded_certificate: v.pem_encoded_certificate,
            created_date: v.created_date,
            expiration_date: v.expiration_date,
            tags: v.tags,
        }
    }
}

// ── DocumentationPart ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationPartView {
    pub rest_api_id: String,
    pub id: String,
    pub location_type: String,
    pub location_path: Option<String>,
    pub location_method: Option<String>,
    pub location_status_code: Option<String>,
    pub location_name: Option<String>,
    pub properties: String,
}

impl From<(&str, &DocumentationPart)> for DocumentationPartView {
    fn from((api_id, d): (&str, &DocumentationPart)) -> Self {
        Self {
            rest_api_id: api_id.to_string(),
            id: d.id.clone(),
            location_type: d.location_type.clone(),
            location_path: d.location_path.clone(),
            location_method: d.location_method.clone(),
            location_status_code: d.location_status_code.clone(),
            location_name: d.location_name.clone(),
            properties: d.properties.clone(),
        }
    }
}

impl From<DocumentationPartView> for DocumentationPart {
    fn from(v: DocumentationPartView) -> Self {
        Self {
            id: v.id,
            location_type: v.location_type,
            location_path: v.location_path,
            location_method: v.location_method,
            location_status_code: v.location_status_code,
            location_name: v.location_name,
            properties: v.properties,
        }
    }
}

// ── DocumentationVersion ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationVersionView {
    pub rest_api_id: String,
    pub version: String,
    pub created_date: f64,
    pub description: Option<String>,
}

impl From<(&str, &DocumentationVersion)> for DocumentationVersionView {
    fn from((api_id, d): (&str, &DocumentationVersion)) -> Self {
        Self {
            rest_api_id: api_id.to_string(),
            version: d.version.clone(),
            created_date: d.created_date,
            description: d.description.clone(),
        }
    }
}

impl From<DocumentationVersionView> for DocumentationVersion {
    fn from(v: DocumentationVersionView) -> Self {
        Self {
            version: v.version,
            created_date: v.created_date,
            description: v.description,
        }
    }
}

// ── DomainNameAccessAssociation ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainNameAccessAssociationView {
    pub arn: String,
    pub domain_name_arn: String,
    pub access_association_source: String,
    pub access_association_source_type: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&DomainNameAccessAssociation> for DomainNameAccessAssociationView {
    fn from(d: &DomainNameAccessAssociation) -> Self {
        Self {
            arn: d.arn.clone(),
            domain_name_arn: d.domain_name_arn.clone(),
            access_association_source: d.access_association_source.clone(),
            access_association_source_type: d.access_association_source_type.clone(),
            tags: d.tags.clone(),
        }
    }
}

impl From<DomainNameAccessAssociationView> for DomainNameAccessAssociation {
    fn from(v: DomainNameAccessAssociationView) -> Self {
        Self {
            arn: v.arn,
            domain_name_arn: v.domain_name_arn,
            access_association_source: v.access_association_source,
            access_association_source_type: v.access_association_source_type,
            tags: v.tags,
        }
    }
}

// ── StatefulService implementation ────────────────────────────────────────────

impl StatefulService for ApiGatewayService {
    type StateView = ApiGatewayStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        let rest_apis = guard
            .rest_apis
            .iter()
            .map(|(k, v)| (k.clone(), RestApiView::from(v)))
            .collect();
        let resources = guard
            .resources
            .iter()
            .map(|((api_id, res_id), r)| {
                (
                    format!("{api_id}/{res_id}"),
                    ResourceView::from((api_id.as_str(), r)),
                )
            })
            .collect();
        let stages = guard
            .stages
            .iter()
            .map(|((api_id, stage_name), s)| (format!("{api_id}/{stage_name}"), StageView::from(s)))
            .collect();
        let deployments = guard
            .deployments
            .iter()
            .map(|((api_id, dep_id), d)| {
                (
                    format!("{api_id}/{dep_id}"),
                    DeploymentView::from((api_id.as_str(), d)),
                )
            })
            .collect();
        let authorizers = guard
            .authorizers
            .iter()
            .map(|((api_id, auth_id), a)| {
                (
                    format!("{api_id}/{auth_id}"),
                    AuthorizerView::from((api_id.as_str(), a)),
                )
            })
            .collect();
        let models = guard
            .models
            .iter()
            .map(|((api_id, model_name), m)| {
                (
                    format!("{api_id}/{model_name}"),
                    ModelView::from((api_id.as_str(), m)),
                )
            })
            .collect();
        let api_keys = guard
            .api_keys
            .iter()
            .map(|(k, v)| (k.clone(), ApiKeyView::from(v)))
            .collect();
        let base_path_mappings = guard
            .base_path_mappings
            .iter()
            .map(|((dn, bp), b)| {
                (
                    format!("{dn}/{bp}"),
                    BasePathMappingView::from((dn.as_str(), b)),
                )
            })
            .collect();
        let domain_names = guard
            .domain_names
            .iter()
            .map(|(k, v)| (k.clone(), DomainNameView::from(v)))
            .collect();
        let gateway_responses = guard
            .gateway_responses
            .iter()
            .map(|((api_id, rt), g)| {
                (
                    format!("{api_id}/{rt}"),
                    GatewayResponseView::from((api_id.as_str(), g)),
                )
            })
            .collect();
        let request_validators = guard
            .request_validators
            .iter()
            .map(|((api_id, val_id), r)| {
                (
                    format!("{api_id}/{val_id}"),
                    RequestValidatorView::from((api_id.as_str(), r)),
                )
            })
            .collect();
        let usage_plans = guard
            .usage_plans
            .iter()
            .map(|(k, v)| (k.clone(), UsagePlanView::from(v)))
            .collect();
        let usage_plan_keys = guard
            .usage_plan_keys
            .iter()
            .map(|((plan_id, key_id), k)| {
                (
                    format!("{plan_id}/{key_id}"),
                    UsagePlanKeyView::from((plan_id.as_str(), k)),
                )
            })
            .collect();
        let vpc_links = guard
            .vpc_links
            .iter()
            .map(|(k, v)| (k.clone(), VpcLinkView::from(v)))
            .collect();
        let account = AccountView::from(&guard.account);
        let client_certificates = guard
            .client_certificates
            .iter()
            .map(|(k, v)| (k.clone(), ClientCertificateView::from(v)))
            .collect();
        let documentation_parts = guard
            .documentation_parts
            .iter()
            .map(|((api_id, part_id), d)| {
                (
                    format!("{api_id}/{part_id}"),
                    DocumentationPartView::from((api_id.as_str(), d)),
                )
            })
            .collect();
        let documentation_versions = guard
            .documentation_versions
            .iter()
            .map(|((api_id, ver), d)| {
                (
                    format!("{api_id}/{ver}"),
                    DocumentationVersionView::from((api_id.as_str(), d)),
                )
            })
            .collect();
        let tags = guard.tags.clone();
        let domain_name_access_associations = guard
            .domain_name_access_associations
            .iter()
            .map(|(k, v)| (k.clone(), DomainNameAccessAssociationView::from(v)))
            .collect();
        ApiGatewayStateView {
            rest_apis,
            resources,
            stages,
            deployments,
            authorizers,
            models,
            api_keys,
            base_path_mappings,
            domain_names,
            gateway_responses,
            request_validators,
            usage_plans,
            usage_plan_keys,
            vpc_links,
            account,
            client_certificates,
            documentation_parts,
            documentation_versions,
            tags,
            domain_name_access_associations,
        }
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = state_from_view(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in view.rest_apis {
                guard.rest_apis.insert(k, RestApi::from(v));
            }
            for (composite_key, rv) in view.resources {
                if let Some((api_id, res_id)) = composite_key.split_once('/') {
                    guard
                        .resources
                        .insert((api_id.to_string(), res_id.to_string()), Resource::from(rv));
                }
            }
            for (composite_key, sv) in view.stages {
                if let Some((api_id, stage_name)) = composite_key.split_once('/') {
                    guard.stages.insert(
                        (api_id.to_string(), stage_name.to_string()),
                        Stage::from(sv),
                    );
                }
            }
            for (composite_key, dv) in view.deployments {
                if let Some((api_id, dep_id)) = composite_key.split_once('/') {
                    guard.deployments.insert(
                        (api_id.to_string(), dep_id.to_string()),
                        Deployment::from(dv),
                    );
                }
            }
            for (composite_key, av) in view.authorizers {
                if let Some((api_id, auth_id)) = composite_key.split_once('/') {
                    guard.authorizers.insert(
                        (api_id.to_string(), auth_id.to_string()),
                        Authorizer::from(av),
                    );
                }
            }
            for (composite_key, mv) in view.models {
                if let Some((api_id, model_name)) = composite_key.split_once('/') {
                    guard.models.insert(
                        (api_id.to_string(), model_name.to_string()),
                        ApiGatewayModel::from(mv),
                    );
                }
            }
            for (k, v) in view.api_keys {
                guard.api_keys.insert(k, ApiKey::from(v));
            }
            for (composite_key, bv) in view.base_path_mappings {
                if let Some((dn, bp)) = composite_key.split_once('/') {
                    guard
                        .base_path_mappings
                        .insert((dn.to_string(), bp.to_string()), BasePathMapping::from(bv));
                }
            }
            for (k, v) in view.domain_names {
                guard.domain_names.insert(k, DomainName::from(v));
            }
            for (composite_key, gv) in view.gateway_responses {
                if let Some((api_id, rt)) = composite_key.split_once('/') {
                    guard.gateway_responses.insert(
                        (api_id.to_string(), rt.to_string()),
                        GatewayResponse::from(gv),
                    );
                }
            }
            for (composite_key, rv) in view.request_validators {
                if let Some((api_id, val_id)) = composite_key.split_once('/') {
                    guard.request_validators.insert(
                        (api_id.to_string(), val_id.to_string()),
                        RequestValidator::from(rv),
                    );
                }
            }
            for (k, v) in view.usage_plans {
                guard.usage_plans.insert(k, UsagePlan::from(v));
            }
            for (composite_key, kv) in view.usage_plan_keys {
                if let Some((plan_id, key_id)) = composite_key.split_once('/') {
                    guard.usage_plan_keys.insert(
                        (plan_id.to_string(), key_id.to_string()),
                        UsagePlanKey::from(kv),
                    );
                }
            }
            for (k, v) in view.vpc_links {
                guard.vpc_links.insert(k, VpcLink::from(v));
            }
            guard.account = Account::from(view.account);
            for (k, v) in view.client_certificates {
                guard
                    .client_certificates
                    .insert(k, ClientCertificate::from(v));
            }
            for (composite_key, dv) in view.documentation_parts {
                if let Some((api_id, part_id)) = composite_key.split_once('/') {
                    guard.documentation_parts.insert(
                        (api_id.to_string(), part_id.to_string()),
                        DocumentationPart::from(dv),
                    );
                }
            }
            for (composite_key, dv) in view.documentation_versions {
                if let Some((api_id, ver)) = composite_key.split_once('/') {
                    guard.documentation_versions.insert(
                        (api_id.to_string(), ver.to_string()),
                        DocumentationVersion::from(dv),
                    );
                }
            }
            for (k, v) in view.tags {
                guard.tags.insert(k, v);
            }
            for (k, v) in view.domain_name_access_associations {
                guard
                    .domain_name_access_associations
                    .insert(k, DomainNameAccessAssociation::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

fn state_from_view(view: ApiGatewayStateView) -> ApiGatewayState {
    let rest_apis = view
        .rest_apis
        .into_iter()
        .map(|(k, v)| (k, RestApi::from(v)))
        .collect();
    let resources = view
        .resources
        .into_iter()
        .filter_map(|(composite_key, rv)| {
            composite_key.split_once('/').map(|(api_id, res_id)| {
                ((api_id.to_string(), res_id.to_string()), Resource::from(rv))
            })
        })
        .collect();
    let stages = view
        .stages
        .into_iter()
        .filter_map(|(composite_key, sv)| {
            composite_key.split_once('/').map(|(api_id, stage_name)| {
                (
                    (api_id.to_string(), stage_name.to_string()),
                    Stage::from(sv),
                )
            })
        })
        .collect();
    let deployments = view
        .deployments
        .into_iter()
        .filter_map(|(composite_key, dv)| {
            composite_key.split_once('/').map(|(api_id, dep_id)| {
                (
                    (api_id.to_string(), dep_id.to_string()),
                    Deployment::from(dv),
                )
            })
        })
        .collect();
    let authorizers = view
        .authorizers
        .into_iter()
        .filter_map(|(composite_key, av)| {
            composite_key.split_once('/').map(|(api_id, auth_id)| {
                (
                    (api_id.to_string(), auth_id.to_string()),
                    Authorizer::from(av),
                )
            })
        })
        .collect();
    let models = view
        .models
        .into_iter()
        .filter_map(|(composite_key, mv)| {
            composite_key.split_once('/').map(|(api_id, model_name)| {
                (
                    (api_id.to_string(), model_name.to_string()),
                    ApiGatewayModel::from(mv),
                )
            })
        })
        .collect();
    let api_keys = view
        .api_keys
        .into_iter()
        .map(|(k, v)| (k, ApiKey::from(v)))
        .collect();
    let base_path_mappings = view
        .base_path_mappings
        .into_iter()
        .filter_map(|(composite_key, bv)| {
            composite_key
                .split_once('/')
                .map(|(dn, bp)| ((dn.to_string(), bp.to_string()), BasePathMapping::from(bv)))
        })
        .collect();
    let domain_names = view
        .domain_names
        .into_iter()
        .map(|(k, v)| (k, DomainName::from(v)))
        .collect();
    let gateway_responses = view
        .gateway_responses
        .into_iter()
        .filter_map(|(composite_key, gv)| {
            composite_key.split_once('/').map(|(api_id, rt)| {
                (
                    (api_id.to_string(), rt.to_string()),
                    GatewayResponse::from(gv),
                )
            })
        })
        .collect();
    let request_validators = view
        .request_validators
        .into_iter()
        .filter_map(|(composite_key, rv)| {
            composite_key.split_once('/').map(|(api_id, val_id)| {
                (
                    (api_id.to_string(), val_id.to_string()),
                    RequestValidator::from(rv),
                )
            })
        })
        .collect();
    let usage_plans = view
        .usage_plans
        .into_iter()
        .map(|(k, v)| (k, UsagePlan::from(v)))
        .collect();
    let usage_plan_keys = view
        .usage_plan_keys
        .into_iter()
        .filter_map(|(composite_key, kv)| {
            composite_key.split_once('/').map(|(plan_id, key_id)| {
                (
                    (plan_id.to_string(), key_id.to_string()),
                    UsagePlanKey::from(kv),
                )
            })
        })
        .collect();
    let vpc_links = view
        .vpc_links
        .into_iter()
        .map(|(k, v)| (k, VpcLink::from(v)))
        .collect();
    let account = Account::from(view.account);
    let client_certificates = view
        .client_certificates
        .into_iter()
        .map(|(k, v)| (k, ClientCertificate::from(v)))
        .collect();
    let documentation_parts = view
        .documentation_parts
        .into_iter()
        .filter_map(|(composite_key, dv)| {
            composite_key.split_once('/').map(|(api_id, part_id)| {
                (
                    (api_id.to_string(), part_id.to_string()),
                    DocumentationPart::from(dv),
                )
            })
        })
        .collect();
    let documentation_versions = view
        .documentation_versions
        .into_iter()
        .filter_map(|(composite_key, dv)| {
            composite_key.split_once('/').map(|(api_id, ver)| {
                (
                    (api_id.to_string(), ver.to_string()),
                    DocumentationVersion::from(dv),
                )
            })
        })
        .collect();
    let tags = view.tags;
    let domain_name_access_associations = view
        .domain_name_access_associations
        .into_iter()
        .map(|(k, v)| (k, DomainNameAccessAssociation::from(v)))
        .collect();
    ApiGatewayState {
        rest_apis,
        resources,
        stages,
        deployments,
        authorizers,
        models,
        api_keys,
        base_path_mappings,
        domain_names,
        gateway_responses,
        request_validators,
        usage_plans,
        usage_plan_keys,
        vpc_links,
        account,
        client_certificates,
        documentation_parts,
        documentation_versions,
        tags,
        domain_name_access_associations,
    }
}
