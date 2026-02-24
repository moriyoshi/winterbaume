use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RestApi {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub created_date: f64,
    pub endpoint_configuration: Option<EndpointConfiguration>,
    pub tags: HashMap<String, String>,
    pub disable_execute_api_endpoint: Option<bool>,
    pub policy: Option<String>,
    pub api_key_source: Option<String>,
    pub binary_media_types: Vec<String>,
    pub minimum_compression_size: Option<i32>,
    /// The root resource id (the "/" resource)
    pub root_resource_id: String,
}

#[derive(Debug, Clone)]
pub struct EndpointConfiguration {
    pub types: Vec<String>,
    pub vpc_endpoint_ids: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub id: String,
    pub parent_id: Option<String>,
    pub path_part: Option<String>,
    pub path: String,
    /// Methods keyed by HTTP method name (GET, POST, etc.)
    pub methods: HashMap<String, Method>,
}

#[derive(Debug, Clone, Default)]
pub struct Method {
    pub http_method: String,
    pub authorization_type: String,
    pub authorizer_id: Option<String>,
    pub api_key_required: Option<bool>,
    pub operation_name: Option<String>,
    pub request_models: HashMap<String, String>,
    pub request_parameters: HashMap<String, bool>,
    pub request_validator_id: Option<String>,
    /// Method responses keyed by statusCode
    pub method_responses: HashMap<String, MethodResponse>,
    /// Integration (if any)
    pub integration: Option<Integration>,
}

#[derive(Debug, Clone, Default)]
pub struct MethodResponse {
    pub status_code: String,
    pub response_models: HashMap<String, String>,
    pub response_parameters: HashMap<String, bool>,
}

#[derive(Debug, Clone, Default)]
pub struct Integration {
    pub integration_type: String,
    pub http_method: Option<String>,
    pub uri: Option<String>,
    pub credentials: Option<String>,
    pub request_parameters: HashMap<String, String>,
    pub request_templates: HashMap<String, String>,
    pub passthrough_behavior: Option<String>,
    pub content_handling: Option<String>,
    pub timeout_in_millis: Option<i32>,
    pub cache_namespace: Option<String>,
    pub cache_key_parameters: Vec<String>,
    pub connection_type: Option<String>,
    pub connection_id: Option<String>,
    /// Integration responses keyed by statusCode
    pub integration_responses: HashMap<String, IntegrationResponse>,
}

#[derive(Debug, Clone, Default)]
pub struct IntegrationResponse {
    pub status_code: String,
    pub selection_pattern: Option<String>,
    pub response_templates: HashMap<String, String>,
    pub response_parameters: HashMap<String, String>,
    pub content_handling: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Stage {
    pub rest_api_id: String,
    pub stage_name: String,
    pub deployment_id: String,
    pub description: Option<String>,
    pub created_date: f64,
    pub last_updated_date: f64,
    pub tags: HashMap<String, String>,
    pub tracing_enabled: Option<bool>,
    pub variables: HashMap<String, String>,
    pub cache_cluster_enabled: Option<bool>,
    pub cache_cluster_size: Option<String>,
    pub documentation_version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Deployment {
    pub id: String,
    pub description: Option<String>,
    pub created_date: f64,
}

#[derive(Debug, Clone)]
pub struct Authorizer {
    pub id: String,
    pub name: String,
    pub authorizer_type: String,
    pub authorizer_uri: Option<String>,
    pub authorizer_credentials: Option<String>,
    pub identity_source: Option<String>,
    pub identity_validation_expression: Option<String>,
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    pub auth_type: Option<String>,
    pub provider_arns: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ApiGatewayModel {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub schema: Option<String>,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub id: String,
    pub name: String,
    pub value: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub stage_keys: Vec<String>,
    pub tags: HashMap<String, String>,
    pub created_date: f64,
}

#[derive(Debug, Clone)]
pub struct BasePathMapping {
    pub base_path: String,
    pub rest_api_id: String,
    pub stage: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DomainName {
    pub domain_name: String,
    pub certificate_name: Option<String>,
    pub distribution_domain_name: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct GatewayResponse {
    pub response_type: String,
    pub status_code: Option<String>,
    pub response_parameters: HashMap<String, String>,
    pub response_templates: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct RequestValidator {
    pub id: String,
    pub name: String,
    pub validate_request_body: bool,
    pub validate_request_parameters: bool,
}

#[derive(Debug, Clone)]
pub struct UsagePlan {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub api_stages: Vec<UsagePlanApiStage>,
    pub throttle: Option<ThrottleSettings>,
    pub quota: Option<QuotaSettings>,
    pub product_code: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct UsagePlanApiStage {
    pub api_id: String,
    pub stage: String,
}

#[derive(Debug, Clone)]
pub struct ThrottleSettings {
    pub burst_limit: Option<i32>,
    pub rate_limit: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct QuotaSettings {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub period: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UsagePlanKey {
    pub id: String,
    pub key_type: String,
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VpcLink {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub target_arns: Vec<String>,
    pub status: String,
    pub status_message: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct Account {
    pub cloudwatch_role_arn: Option<String>,
    pub throttle_settings: Option<ThrottleSettings>,
    pub features: Vec<String>,
    pub api_key_version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ClientCertificate {
    pub client_certificate_id: String,
    pub description: Option<String>,
    pub pem_encoded_certificate: String,
    pub created_date: f64,
    pub expiration_date: f64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct DocumentationPart {
    pub id: String,
    pub location_type: String,
    pub location_path: Option<String>,
    pub location_method: Option<String>,
    pub location_status_code: Option<String>,
    pub location_name: Option<String>,
    pub properties: String,
}

#[derive(Debug, Clone)]
pub struct DocumentationVersion {
    pub version: String,
    pub created_date: f64,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DomainNameAccessAssociation {
    pub arn: String,
    pub domain_name_arn: String,
    pub access_association_source: String,
    pub access_association_source_type: String,
    pub tags: HashMap<String, String>,
}
