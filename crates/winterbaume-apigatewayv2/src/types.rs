use std::collections::HashMap;

/// A single API Gateway V2 API.
#[derive(Debug, Clone)]
pub struct Api {
    pub api_id: String,
    pub name: String,
    pub protocol_type: String,
    pub route_selection_expression: Option<String>,
    pub description: Option<String>,
    pub api_endpoint: String,
    pub created_date: String,
    pub tags: HashMap<String, String>,
}

/// A stage for an API.
#[derive(Debug, Clone)]
pub struct Stage {
    pub stage_name: String,
    pub api_id: String,
    pub description: Option<String>,
    pub deployment_id: Option<String>,
    pub auto_deploy: bool,
    pub created_date: String,
    pub last_updated_date: String,
    pub tags: HashMap<String, String>,
}

/// An integration for an API.
#[derive(Debug, Clone)]
pub struct Integration {
    pub integration_id: String,
    pub api_id: String,
    pub integration_type: String,
    pub integration_uri: Option<String>,
    pub integration_method: Option<String>,
    pub description: Option<String>,
    pub payload_format_version: Option<String>,
    pub connection_type: Option<String>,
}

/// A route for an API.
#[derive(Debug, Clone)]
pub struct Route {
    pub route_id: String,
    pub api_id: String,
    pub route_key: String,
    pub target: Option<String>,
    pub authorization_type: Option<String>,
    pub authorizer_id: Option<String>,
}

/// A deployment for an API.
#[derive(Debug, Clone)]
pub struct Deployment {
    pub deployment_id: String,
    pub api_id: String,
    pub deployment_status: String,
    pub description: Option<String>,
    pub created_date: String,
}

/// An authorizer for an API.
#[derive(Debug, Clone)]
pub struct Authorizer {
    pub authorizer_id: String,
    pub api_id: String,
    pub authorizer_type: String,
    pub authorizer_uri: Option<String>,
    pub authorizer_credentials_arn: Option<String>,
    pub authorizer_payload_format_version: Option<String>,
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    pub identity_source: Option<Vec<String>>,
    pub identity_validation_expression: Option<String>,
    pub name: String,
    pub enable_simple_responses: Option<bool>,
}

/// A model for an API.
#[derive(Debug, Clone)]
pub struct Model {
    pub model_id: String,
    pub api_id: String,
    pub name: String,
    pub content_type: Option<String>,
    pub description: Option<String>,
    pub schema: Option<String>,
}

/// A VPC link.
#[derive(Debug, Clone)]
pub struct VpcLink {
    pub vpc_link_id: String,
    pub name: String,
    pub security_group_ids: Vec<String>,
    pub subnet_ids: Vec<String>,
    pub tags: HashMap<String, String>,
    pub created_date: String,
}

/// A custom domain name.
#[derive(Debug, Clone)]
pub struct DomainName {
    pub domain_name: String,
    pub tags: HashMap<String, String>,
    pub domain_name_configurations: Vec<StoredDomainNameConfiguration>,
}

/// Stored domain name configuration (the fields we persist from the create request).
#[derive(Debug, Clone)]
pub struct StoredDomainNameConfiguration {
    pub certificate_arn: Option<String>,
    pub endpoint_type: Option<String>,
    pub security_policy: Option<String>,
}

/// An API mapping for a domain name.
#[derive(Debug, Clone)]
pub struct ApiMapping {
    pub api_mapping_id: String,
    pub domain_name: String,
    pub api_id: String,
    pub stage: String,
    pub api_mapping_key: Option<String>,
}

/// An integration response.
#[derive(Debug, Clone)]
pub struct IntegrationResponse {
    pub integration_response_id: String,
    pub api_id: String,
    pub integration_id: String,
    pub integration_response_key: String,
    pub content_handling_strategy: Option<String>,
    pub response_parameters: Option<HashMap<String, String>>,
    pub response_templates: Option<HashMap<String, String>>,
    pub template_selection_expression: Option<String>,
}

/// A route response.
#[derive(Debug, Clone)]
pub struct RouteResponse {
    pub route_response_id: String,
    pub api_id: String,
    pub route_id: String,
    pub route_response_key: String,
    pub model_selection_expression: Option<String>,
}
