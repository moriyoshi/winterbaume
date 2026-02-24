//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-apigatewayv2

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApiMappingRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "apiMappingKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_key: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(default)]
    pub stage: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApiMappingResponse {
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "apiMappingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_id: Option<String>,
    #[serde(rename = "apiMappingKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApiRequest {
    #[serde(rename = "apiKeySelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    #[serde(rename = "corsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    #[serde(rename = "credentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "disableExecuteApiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_execute_api_endpoint: Option<bool>,
    #[serde(rename = "disableSchemaValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "protocolType")]
    #[serde(default)]
    pub protocol_type: String,
    #[serde(rename = "routeKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_key: Option<String>,
    #[serde(rename = "routeSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cors {
    #[serde(rename = "allowCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
    #[serde(rename = "allowHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_headers: Option<Vec<String>>,
    #[serde(rename = "allowMethods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_methods: Option<Vec<String>>,
    #[serde(rename = "allowOrigins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_origins: Option<Vec<String>>,
    #[serde(rename = "exposeHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,
    #[serde(rename = "maxAge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApiResponse {
    #[serde(rename = "apiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "apiKeySelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    #[serde(rename = "corsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "disableExecuteApiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_execute_api_endpoint: Option<bool>,
    #[serde(rename = "disableSchemaValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    #[serde(rename = "importInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_info: Option<Vec<String>>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "protocolType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    #[serde(rename = "routeSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAuthorizerRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "authorizerCredentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    #[serde(rename = "authorizerPayloadFormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_payload_format_version: Option<String>,
    #[serde(rename = "authorizerResultTtlInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    #[serde(rename = "authorizerType")]
    #[serde(default)]
    pub authorizer_type: String,
    #[serde(rename = "authorizerUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    #[serde(rename = "enableSimpleResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_simple_responses: Option<bool>,
    #[serde(rename = "identitySource")]
    #[serde(default)]
    pub identity_source: Vec<String>,
    #[serde(rename = "identityValidationExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    #[serde(rename = "jwtConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<JWTConfiguration>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JWTConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAuthorizerResponse {
    #[serde(rename = "authorizerCredentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    #[serde(rename = "authorizerPayloadFormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_payload_format_version: Option<String>,
    #[serde(rename = "authorizerResultTtlInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    #[serde(rename = "authorizerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    #[serde(rename = "authorizerUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    #[serde(rename = "enableSimpleResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_simple_responses: Option<bool>,
    #[serde(rename = "identitySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    #[serde(rename = "identityValidationExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    #[serde(rename = "jwtConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<JWTConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeploymentRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "stageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeploymentResponse {
    #[serde(rename = "autoDeployed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deployed: Option<bool>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "deploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    #[serde(rename = "deploymentStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDomainNameRequest {
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "domainNameConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,
    #[serde(rename = "mutualTlsAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_tls_authentication: Option<MutualTlsAuthenticationInput>,
    #[serde(rename = "routingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainNameConfiguration {
    #[serde(rename = "apiGatewayDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_domain_name: Option<String>,
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    #[serde(rename = "certificateUploadDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_upload_date: Option<String>,
    #[serde(rename = "domainNameStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_status: Option<String>,
    #[serde(rename = "domainNameStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_status_message: Option<String>,
    #[serde(rename = "endpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "hostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "ownershipVerificationCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_verification_certificate_arn: Option<String>,
    #[serde(rename = "securityPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MutualTlsAuthenticationInput {
    #[serde(rename = "truststoreUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truststore_uri: Option<String>,
    #[serde(rename = "truststoreVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truststore_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDomainNameResponse {
    #[serde(rename = "apiMappingSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_selection_expression: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "domainNameArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_arn: Option<String>,
    #[serde(rename = "domainNameConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,
    #[serde(rename = "mutualTlsAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_tls_authentication: Option<MutualTlsAuthentication>,
    #[serde(rename = "routingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MutualTlsAuthentication {
    #[serde(rename = "truststoreUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truststore_uri: Option<String>,
    #[serde(rename = "truststoreVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truststore_version: Option<String>,
    #[serde(rename = "truststoreWarnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truststore_warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntegrationRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "connectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "contentHandlingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    #[serde(rename = "credentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "integrationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_method: Option<String>,
    #[serde(rename = "integrationSubtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_subtype: Option<String>,
    #[serde(rename = "integrationType")]
    #[serde(default)]
    pub integration_type: String,
    #[serde(rename = "integrationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    #[serde(rename = "passthroughBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    #[serde(rename = "payloadFormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_version: Option<String>,
    #[serde(rename = "requestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "requestTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    #[serde(rename = "templateSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    #[serde(rename = "timeoutInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i32>,
    #[serde(rename = "tlsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_config: Option<TlsConfigInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TlsConfigInput {
    #[serde(rename = "serverNameToVerify")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name_to_verify: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntegrationResponseRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "contentHandlingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    #[serde(rename = "IntegrationId")]
    #[serde(default)]
    pub integration_id: String,
    #[serde(rename = "integrationResponseKey")]
    #[serde(default)]
    pub integration_response_key: String,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "templateSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntegrationResponseResponse {
    #[serde(rename = "contentHandlingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    #[serde(rename = "integrationResponseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_id: Option<String>,
    #[serde(rename = "integrationResponseKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_key: Option<String>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "templateSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntegrationResult {
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "connectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "contentHandlingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    #[serde(rename = "credentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "integrationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
    #[serde(rename = "integrationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_method: Option<String>,
    #[serde(rename = "integrationResponseSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_selection_expression: Option<String>,
    #[serde(rename = "integrationSubtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_subtype: Option<String>,
    #[serde(rename = "integrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    #[serde(rename = "integrationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    #[serde(rename = "passthroughBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    #[serde(rename = "payloadFormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_version: Option<String>,
    #[serde(rename = "requestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "requestTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    #[serde(rename = "templateSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    #[serde(rename = "timeoutInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i32>,
    #[serde(rename = "tlsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_config: Option<TlsConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TlsConfig {
    #[serde(rename = "serverNameToVerify")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name_to_verify: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateModelRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "contentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub schema: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateModelResponse {
    #[serde(rename = "contentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "modelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePortalProductRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePortalProductResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<DisplayOrder>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "portalProductArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_product_arn: Option<String>,
    #[serde(rename = "portalProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_product_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisplayOrder {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<Section>>,
    #[serde(rename = "overviewPageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview_page_arn: Option<String>,
    #[serde(rename = "productPageArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_page_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Section {
    #[serde(rename = "productRestEndpointPageArns")]
    #[serde(default)]
    pub product_rest_endpoint_page_arns: Vec<String>,
    #[serde(rename = "sectionName")]
    #[serde(default)]
    pub section_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePortalRequest {
    #[serde(default)]
    pub authorization: Authorization,
    #[serde(rename = "endpointConfiguration")]
    #[serde(default)]
    pub endpoint_configuration: EndpointConfigurationRequest,
    #[serde(rename = "includedPortalProductArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_portal_product_arns: Option<Vec<String>>,
    #[serde(rename = "logoUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    #[serde(rename = "portalContent")]
    #[serde(default)]
    pub portal_content: PortalContent,
    #[serde(rename = "rumAppMonitorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rum_app_monitor_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Authorization {
    #[serde(rename = "cognitoConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_config: Option<CognitoConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub none: Option<None>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CognitoConfig {
    #[serde(rename = "appClientId")]
    #[serde(default)]
    pub app_client_id: String,
    #[serde(rename = "userPoolArn")]
    #[serde(default)]
    pub user_pool_arn: String,
    #[serde(rename = "userPoolDomain")]
    #[serde(default)]
    pub user_pool_domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct None {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointConfigurationRequest {
    #[serde(rename = "acmManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm_managed: Option<ACMManaged>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub none: Option<None>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ACMManaged {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortalContent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub theme: PortalTheme,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortalTheme {
    #[serde(rename = "customColors")]
    #[serde(default)]
    pub custom_colors: CustomColors,
    #[serde(rename = "logoLastUploaded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_last_uploaded: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomColors {
    #[serde(rename = "accentColor")]
    #[serde(default)]
    pub accent_color: String,
    #[serde(rename = "backgroundColor")]
    #[serde(default)]
    pub background_color: String,
    #[serde(rename = "errorValidationColor")]
    #[serde(default)]
    pub error_validation_color: String,
    #[serde(rename = "headerColor")]
    #[serde(default)]
    pub header_color: String,
    #[serde(rename = "navigationColor")]
    #[serde(default)]
    pub navigation_color: String,
    #[serde(rename = "textColor")]
    #[serde(default)]
    pub text_color: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePortalResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[serde(rename = "endpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfigurationResponse>,
    #[serde(rename = "includedPortalProductArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_portal_product_arns: Option<Vec<String>>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "lastPublished")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_published: Option<String>,
    #[serde(rename = "lastPublishedDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_published_description: Option<String>,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
    #[serde(rename = "portalContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_content: Option<PortalContent>,
    #[serde(rename = "portalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_id: Option<String>,
    #[serde(rename = "publishStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_status: Option<String>,
    #[serde(rename = "rumAppMonitorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rum_app_monitor_name: Option<String>,
    #[serde(rename = "statusException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_exception: Option<StatusException>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointConfigurationResponse {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "portalDefaultDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_default_domain_name: Option<String>,
    #[serde(rename = "portalDomainHostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_domain_hosted_zone_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatusException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProductPageRequest {
    #[serde(rename = "displayContent")]
    #[serde(default)]
    pub display_content: DisplayContent,
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisplayContent {
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub title: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProductPageResponse {
    #[serde(rename = "displayContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_content: Option<DisplayContent>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "productPageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_page_arn: Option<String>,
    #[serde(rename = "productPageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_page_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProductRestEndpointPageRequest {
    #[serde(rename = "displayContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_content: Option<EndpointDisplayContent>,
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
    #[serde(rename = "restEndpointIdentifier")]
    #[serde(default)]
    pub rest_endpoint_identifier: RestEndpointIdentifier,
    #[serde(rename = "tryItState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub try_it_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointDisplayContent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub none: Option<None>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<DisplayContentOverrides>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisplayContentOverrides {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "operationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestEndpointIdentifier {
    #[serde(rename = "identifierParts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_parts: Option<IdentifierParts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentifierParts {
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub path: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(default)]
    pub stage: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProductRestEndpointPageResponse {
    #[serde(rename = "displayContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_content: Option<EndpointDisplayContentResponse>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "productRestEndpointPageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_rest_endpoint_page_arn: Option<String>,
    #[serde(rename = "productRestEndpointPageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_rest_endpoint_page_id: Option<String>,
    #[serde(rename = "restEndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_endpoint_identifier: Option<RestEndpointIdentifier>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_exception: Option<StatusException>,
    #[serde(rename = "tryItState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub try_it_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointDisplayContentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "operationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRouteRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "apiKeyRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    #[serde(rename = "authorizationScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    #[serde(rename = "authorizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    #[serde(rename = "modelSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    #[serde(rename = "operationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(rename = "requestModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "requestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, ParameterConstraints>>,
    #[serde(rename = "routeKey")]
    #[serde(default)]
    pub route_key: String,
    #[serde(rename = "routeResponseSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterConstraints {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRouteResponseRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "modelSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    #[serde(rename = "responseModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, ParameterConstraints>>,
    #[serde(rename = "RouteId")]
    #[serde(default)]
    pub route_id: String,
    #[serde(rename = "routeResponseKey")]
    #[serde(default)]
    pub route_response_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRouteResponseResponse {
    #[serde(rename = "modelSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    #[serde(rename = "responseModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, ParameterConstraints>>,
    #[serde(rename = "routeResponseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_id: Option<String>,
    #[serde(rename = "routeResponseKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRouteResult {
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "apiKeyRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    #[serde(rename = "authorizationScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    #[serde(rename = "authorizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    #[serde(rename = "modelSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    #[serde(rename = "operationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(rename = "requestModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "requestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, ParameterConstraints>>,
    #[serde(rename = "routeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    #[serde(rename = "routeKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_key: Option<String>,
    #[serde(rename = "routeResponseSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRoutingRuleRequest {
    #[serde(default)]
    pub actions: Vec<RoutingRuleAction>,
    #[serde(default)]
    pub conditions: Vec<RoutingRuleCondition>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "DomainNameId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_id: Option<String>,
    #[serde(default)]
    pub priority: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingRuleAction {
    #[serde(rename = "invokeApi")]
    #[serde(default)]
    pub invoke_api: RoutingRuleActionInvokeApi,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingRuleActionInvokeApi {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    pub stage: String,
    #[serde(rename = "stripBasePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip_base_path: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingRuleCondition {
    #[serde(rename = "matchBasePaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_base_paths: Option<RoutingRuleMatchBasePaths>,
    #[serde(rename = "matchHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_headers: Option<RoutingRuleMatchHeaders>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingRuleMatchBasePaths {
    #[serde(rename = "anyOf")]
    #[serde(default)]
    pub any_of: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingRuleMatchHeaders {
    #[serde(rename = "anyOf")]
    #[serde(default)]
    pub any_of: Vec<RoutingRuleMatchHeaderValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingRuleMatchHeaderValue {
    #[serde(default)]
    pub header: String,
    #[serde(rename = "valueGlob")]
    #[serde(default)]
    pub value_glob: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRoutingRuleResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<RoutingRuleAction>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<RoutingRuleCondition>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "routingRuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_rule_arn: Option<String>,
    #[serde(rename = "routingRuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStageRequest {
    #[serde(rename = "accessLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "autoDeploy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    #[serde(rename = "clientCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    #[serde(rename = "defaultRouteSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<RouteSettings>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "routeSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<std::collections::HashMap<String, RouteSettings>>,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
    #[serde(rename = "stageVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessLogSettings {
    #[serde(rename = "destinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteSettings {
    #[serde(rename = "dataTraceEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_trace_enabled: Option<bool>,
    #[serde(rename = "detailedMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_metrics_enabled: Option<bool>,
    #[serde(rename = "loggingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    #[serde(rename = "throttlingBurstLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_burst_limit: Option<i32>,
    #[serde(rename = "throttlingRateLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_rate_limit: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStageResponse {
    #[serde(rename = "accessLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "autoDeploy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    #[serde(rename = "clientCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "defaultRouteSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<RouteSettings>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastDeploymentStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_status_message: Option<String>,
    #[serde(rename = "lastUpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<String>,
    #[serde(rename = "routeSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<std::collections::HashMap<String, RouteSettings>>,
    #[serde(rename = "stageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(rename = "stageVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcLinkRequest {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcLinkResponse {
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "vpcLinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_id: Option<String>,
    #[serde(rename = "vpcLinkStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_status: Option<String>,
    #[serde(rename = "vpcLinkStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_status_message: Option<String>,
    #[serde(rename = "vpcLinkVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccessLogSettingsRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "StageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApiMappingRequest {
    #[serde(rename = "ApiMappingId")]
    #[serde(default)]
    pub api_mapping_id: String,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApiRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAuthorizerRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "AuthorizerId")]
    #[serde(default)]
    pub authorizer_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCorsConfigurationRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeploymentRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "DeploymentId")]
    #[serde(default)]
    pub deployment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainNameRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIntegrationRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "IntegrationId")]
    #[serde(default)]
    pub integration_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIntegrationResponseRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "IntegrationId")]
    #[serde(default)]
    pub integration_id: String,
    #[serde(rename = "IntegrationResponseId")]
    #[serde(default)]
    pub integration_response_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteModelRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "ModelId")]
    #[serde(default)]
    pub model_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePortalProductRequest {
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePortalProductSharingPolicyRequest {
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePortalRequest {
    #[serde(rename = "PortalId")]
    #[serde(default)]
    pub portal_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProductPageRequest {
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
    #[serde(rename = "ProductPageId")]
    #[serde(default)]
    pub product_page_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProductRestEndpointPageRequest {
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
    #[serde(rename = "ProductRestEndpointPageId")]
    #[serde(default)]
    pub product_rest_endpoint_page_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRouteRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "RouteId")]
    #[serde(default)]
    pub route_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRouteRequestParameterRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "RequestParameterKey")]
    #[serde(default)]
    pub request_parameter_key: String,
    #[serde(rename = "RouteId")]
    #[serde(default)]
    pub route_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRouteResponseRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "RouteId")]
    #[serde(default)]
    pub route_id: String,
    #[serde(rename = "RouteResponseId")]
    #[serde(default)]
    pub route_response_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRouteSettingsRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "RouteKey")]
    #[serde(default)]
    pub route_key: String,
    #[serde(rename = "StageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRoutingRuleRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "DomainNameId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_id: Option<String>,
    #[serde(rename = "RoutingRuleId")]
    #[serde(default)]
    pub routing_rule_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStageRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "StageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcLinkRequest {
    #[serde(rename = "VpcLinkId")]
    #[serde(default)]
    pub vpc_link_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcLinkResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisablePortalRequest {
    #[serde(rename = "PortalId")]
    #[serde(default)]
    pub portal_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportApiRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "ExportVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_version: Option<String>,
    #[serde(rename = "IncludeExtensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_extensions: Option<bool>,
    #[serde(rename = "OutputType")]
    #[serde(default)]
    pub output_type: String,
    #[serde(rename = "Specification")]
    #[serde(default)]
    pub specification: String,
    #[serde(rename = "StageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportApiResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApiMappingRequest {
    #[serde(rename = "ApiMappingId")]
    #[serde(default)]
    pub api_mapping_id: String,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApiMappingResponse {
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "apiMappingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_id: Option<String>,
    #[serde(rename = "apiMappingKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApiMappingsRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApiMappingsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ApiMapping>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiMapping {
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "apiMappingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_id: Option<String>,
    #[serde(rename = "apiMappingKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApiRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApiResponse {
    #[serde(rename = "apiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "apiKeySelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    #[serde(rename = "corsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "disableExecuteApiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_execute_api_endpoint: Option<bool>,
    #[serde(rename = "disableSchemaValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    #[serde(rename = "importInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_info: Option<Vec<String>>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "protocolType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    #[serde(rename = "routeSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApisRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApisResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Api>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Api {
    #[serde(rename = "apiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "apiKeySelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    #[serde(rename = "corsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "disableExecuteApiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_execute_api_endpoint: Option<bool>,
    #[serde(rename = "disableSchemaValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    #[serde(rename = "importInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_info: Option<Vec<String>>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "protocolType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    #[serde(rename = "routeSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAuthorizerRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "AuthorizerId")]
    #[serde(default)]
    pub authorizer_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAuthorizerResponse {
    #[serde(rename = "authorizerCredentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    #[serde(rename = "authorizerPayloadFormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_payload_format_version: Option<String>,
    #[serde(rename = "authorizerResultTtlInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    #[serde(rename = "authorizerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    #[serde(rename = "authorizerUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    #[serde(rename = "enableSimpleResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_simple_responses: Option<bool>,
    #[serde(rename = "identitySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    #[serde(rename = "identityValidationExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    #[serde(rename = "jwtConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<JWTConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAuthorizersRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAuthorizersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Authorizer>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Authorizer {
    #[serde(rename = "authorizerCredentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    #[serde(rename = "authorizerPayloadFormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_payload_format_version: Option<String>,
    #[serde(rename = "authorizerResultTtlInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    #[serde(rename = "authorizerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    #[serde(rename = "authorizerUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    #[serde(rename = "enableSimpleResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_simple_responses: Option<bool>,
    #[serde(rename = "identitySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    #[serde(rename = "identityValidationExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    #[serde(rename = "jwtConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<JWTConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "DeploymentId")]
    #[serde(default)]
    pub deployment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentResponse {
    #[serde(rename = "autoDeployed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deployed: Option<bool>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "deploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    #[serde(rename = "deploymentStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentsRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Deployment>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Deployment {
    #[serde(rename = "autoDeployed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deployed: Option<bool>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "deploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    #[serde(rename = "deploymentStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainNameRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainNameResponse {
    #[serde(rename = "apiMappingSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_selection_expression: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "domainNameArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_arn: Option<String>,
    #[serde(rename = "domainNameConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,
    #[serde(rename = "mutualTlsAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_tls_authentication: Option<MutualTlsAuthentication>,
    #[serde(rename = "routingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainNamesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainNamesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DomainName>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainName {
    #[serde(rename = "apiMappingSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_selection_expression: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "domainNameArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_arn: Option<String>,
    #[serde(rename = "domainNameConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,
    #[serde(rename = "mutualTlsAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_tls_authentication: Option<MutualTlsAuthentication>,
    #[serde(rename = "routingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "IntegrationId")]
    #[serde(default)]
    pub integration_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationResponseRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "IntegrationId")]
    #[serde(default)]
    pub integration_id: String,
    #[serde(rename = "IntegrationResponseId")]
    #[serde(default)]
    pub integration_response_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationResponseResponse {
    #[serde(rename = "contentHandlingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    #[serde(rename = "integrationResponseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_id: Option<String>,
    #[serde(rename = "integrationResponseKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_key: Option<String>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "templateSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationResponsesRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "IntegrationId")]
    #[serde(default)]
    pub integration_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationResponsesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<IntegrationResponse>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationResponse {
    #[serde(rename = "contentHandlingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    #[serde(rename = "integrationResponseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_id: Option<String>,
    #[serde(rename = "integrationResponseKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_key: Option<String>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "templateSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationResult {
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "connectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "contentHandlingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    #[serde(rename = "credentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "integrationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
    #[serde(rename = "integrationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_method: Option<String>,
    #[serde(rename = "integrationResponseSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_selection_expression: Option<String>,
    #[serde(rename = "integrationSubtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_subtype: Option<String>,
    #[serde(rename = "integrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    #[serde(rename = "integrationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    #[serde(rename = "passthroughBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    #[serde(rename = "payloadFormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_version: Option<String>,
    #[serde(rename = "requestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "requestTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    #[serde(rename = "templateSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    #[serde(rename = "timeoutInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i32>,
    #[serde(rename = "tlsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_config: Option<TlsConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationsRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Integration>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Integration {
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "connectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "contentHandlingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    #[serde(rename = "credentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "integrationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
    #[serde(rename = "integrationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_method: Option<String>,
    #[serde(rename = "integrationResponseSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_selection_expression: Option<String>,
    #[serde(rename = "integrationSubtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_subtype: Option<String>,
    #[serde(rename = "integrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    #[serde(rename = "integrationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    #[serde(rename = "passthroughBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    #[serde(rename = "payloadFormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_version: Option<String>,
    #[serde(rename = "requestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "requestTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    #[serde(rename = "templateSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    #[serde(rename = "timeoutInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i32>,
    #[serde(rename = "tlsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_config: Option<TlsConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "ModelId")]
    #[serde(default)]
    pub model_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelResponse {
    #[serde(rename = "contentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "modelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelTemplateRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "ModelId")]
    #[serde(default)]
    pub model_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelTemplateResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelsRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Model>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Model {
    #[serde(rename = "contentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "modelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPortalProductRequest {
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
    #[serde(rename = "ResourceOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPortalProductResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<DisplayOrder>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "portalProductArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_product_arn: Option<String>,
    #[serde(rename = "portalProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_product_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPortalProductSharingPolicyRequest {
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPortalProductSharingPolicyResponse {
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "portalProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_product_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPortalRequest {
    #[serde(rename = "PortalId")]
    #[serde(default)]
    pub portal_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPortalResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[serde(rename = "endpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfigurationResponse>,
    #[serde(rename = "includedPortalProductArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_portal_product_arns: Option<Vec<String>>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "lastPublished")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_published: Option<String>,
    #[serde(rename = "lastPublishedDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_published_description: Option<String>,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
    #[serde(rename = "portalContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_content: Option<PortalContent>,
    #[serde(rename = "portalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<Preview>,
    #[serde(rename = "publishStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_status: Option<String>,
    #[serde(rename = "rumAppMonitorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rum_app_monitor_name: Option<String>,
    #[serde(rename = "statusException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_exception: Option<StatusException>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Preview {
    #[serde(rename = "previewStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_status: Option<String>,
    #[serde(rename = "previewUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
    #[serde(rename = "statusException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_exception: Option<StatusException>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetProductPageRequest {
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
    #[serde(rename = "ProductPageId")]
    #[serde(default)]
    pub product_page_id: String,
    #[serde(rename = "ResourceOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetProductPageResponse {
    #[serde(rename = "displayContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_content: Option<DisplayContent>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "productPageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_page_arn: Option<String>,
    #[serde(rename = "productPageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_page_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetProductRestEndpointPageRequest {
    #[serde(rename = "IncludeRawDisplayContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_raw_display_content: Option<String>,
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
    #[serde(rename = "ProductRestEndpointPageId")]
    #[serde(default)]
    pub product_rest_endpoint_page_id: String,
    #[serde(rename = "ResourceOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetProductRestEndpointPageResponse {
    #[serde(rename = "displayContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_content: Option<EndpointDisplayContentResponse>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "productRestEndpointPageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_rest_endpoint_page_arn: Option<String>,
    #[serde(rename = "productRestEndpointPageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_rest_endpoint_page_id: Option<String>,
    #[serde(rename = "rawDisplayContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_display_content: Option<String>,
    #[serde(rename = "restEndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_endpoint_identifier: Option<RestEndpointIdentifier>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_exception: Option<StatusException>,
    #[serde(rename = "tryItState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub try_it_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouteRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "RouteId")]
    #[serde(default)]
    pub route_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouteResponseRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "RouteId")]
    #[serde(default)]
    pub route_id: String,
    #[serde(rename = "RouteResponseId")]
    #[serde(default)]
    pub route_response_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouteResponseResponse {
    #[serde(rename = "modelSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    #[serde(rename = "responseModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, ParameterConstraints>>,
    #[serde(rename = "routeResponseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_id: Option<String>,
    #[serde(rename = "routeResponseKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouteResponsesRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RouteId")]
    #[serde(default)]
    pub route_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouteResponsesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RouteResponse>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteResponse {
    #[serde(rename = "modelSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    #[serde(rename = "responseModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, ParameterConstraints>>,
    #[serde(rename = "routeResponseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_id: Option<String>,
    #[serde(rename = "routeResponseKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRouteResult {
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "apiKeyRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    #[serde(rename = "authorizationScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    #[serde(rename = "authorizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    #[serde(rename = "modelSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    #[serde(rename = "operationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(rename = "requestModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "requestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, ParameterConstraints>>,
    #[serde(rename = "routeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    #[serde(rename = "routeKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_key: Option<String>,
    #[serde(rename = "routeResponseSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRoutesRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRoutesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Route>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Route {
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "apiKeyRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    #[serde(rename = "authorizationScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    #[serde(rename = "authorizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    #[serde(rename = "modelSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    #[serde(rename = "operationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(rename = "requestModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "requestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, ParameterConstraints>>,
    #[serde(rename = "routeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    #[serde(rename = "routeKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_key: Option<String>,
    #[serde(rename = "routeResponseSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRoutingRuleRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "DomainNameId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_id: Option<String>,
    #[serde(rename = "RoutingRuleId")]
    #[serde(default)]
    pub routing_rule_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRoutingRuleResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<RoutingRuleAction>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<RoutingRuleCondition>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "routingRuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_rule_arn: Option<String>,
    #[serde(rename = "routingRuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStageRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "StageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStageResponse {
    #[serde(rename = "accessLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "autoDeploy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    #[serde(rename = "clientCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "defaultRouteSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<RouteSettings>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastDeploymentStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_status_message: Option<String>,
    #[serde(rename = "lastUpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<String>,
    #[serde(rename = "routeSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<std::collections::HashMap<String, RouteSettings>>,
    #[serde(rename = "stageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(rename = "stageVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStagesRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStagesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Stage>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Stage {
    #[serde(rename = "accessLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "autoDeploy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    #[serde(rename = "clientCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "defaultRouteSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<RouteSettings>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastDeploymentStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_status_message: Option<String>,
    #[serde(rename = "lastUpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<String>,
    #[serde(rename = "routeSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<std::collections::HashMap<String, RouteSettings>>,
    #[serde(rename = "stageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(rename = "stageVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTagsRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTagsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVpcLinkRequest {
    #[serde(rename = "VpcLinkId")]
    #[serde(default)]
    pub vpc_link_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVpcLinkResponse {
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "vpcLinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_id: Option<String>,
    #[serde(rename = "vpcLinkStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_status: Option<String>,
    #[serde(rename = "vpcLinkStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_status_message: Option<String>,
    #[serde(rename = "vpcLinkVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVpcLinksRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVpcLinksResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<VpcLink>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcLink {
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "vpcLinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_id: Option<String>,
    #[serde(rename = "vpcLinkStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_status: Option<String>,
    #[serde(rename = "vpcLinkStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_status_message: Option<String>,
    #[serde(rename = "vpcLinkVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportApiRequest {
    #[serde(rename = "Basepath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basepath: Option<String>,
    #[serde(default)]
    pub body: String,
    #[serde(rename = "FailOnWarnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportApiResponse {
    #[serde(rename = "apiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "apiKeySelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    #[serde(rename = "corsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "disableExecuteApiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_execute_api_endpoint: Option<bool>,
    #[serde(rename = "disableSchemaValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    #[serde(rename = "importInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_info: Option<Vec<String>>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "protocolType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    #[serde(rename = "routeSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPortalProductsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPortalProductsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PortalProductSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortalProductSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "portalProductArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_product_arn: Option<String>,
    #[serde(rename = "portalProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_product_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPortalsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPortalsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PortalSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortalSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[serde(rename = "endpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfigurationResponse>,
    #[serde(rename = "includedPortalProductArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_portal_product_arns: Option<Vec<String>>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "lastPublished")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_published: Option<String>,
    #[serde(rename = "lastPublishedDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_published_description: Option<String>,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
    #[serde(rename = "portalContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_content: Option<PortalContent>,
    #[serde(rename = "portalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<Preview>,
    #[serde(rename = "publishStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_status: Option<String>,
    #[serde(rename = "rumAppMonitorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rum_app_monitor_name: Option<String>,
    #[serde(rename = "statusException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_exception: Option<StatusException>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProductPagesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
    #[serde(rename = "ResourceOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProductPagesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ProductPageSummaryNoBody>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProductPageSummaryNoBody {
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "pageTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_title: Option<String>,
    #[serde(rename = "productPageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_page_arn: Option<String>,
    #[serde(rename = "productPageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_page_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProductRestEndpointPagesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
    #[serde(rename = "ResourceOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProductRestEndpointPagesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ProductRestEndpointPageSummaryNoBody>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProductRestEndpointPageSummaryNoBody {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "operationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(rename = "productRestEndpointPageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_rest_endpoint_page_arn: Option<String>,
    #[serde(rename = "productRestEndpointPageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_rest_endpoint_page_id: Option<String>,
    #[serde(rename = "restEndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_endpoint_identifier: Option<RestEndpointIdentifier>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_exception: Option<StatusException>,
    #[serde(rename = "tryItState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub try_it_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoutingRulesRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "DomainNameId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoutingRulesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "routingRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_rules: Option<Vec<RoutingRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingRule {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<RoutingRuleAction>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<RoutingRuleCondition>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "routingRuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_rule_arn: Option<String>,
    #[serde(rename = "routingRuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PreviewPortalRequest {
    #[serde(rename = "PortalId")]
    #[serde(default)]
    pub portal_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PreviewPortalResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishPortalRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "PortalId")]
    #[serde(default)]
    pub portal_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishPortalResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPortalProductSharingPolicyRequest {
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPortalProductSharingPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRoutingRuleRequest {
    #[serde(default)]
    pub actions: Vec<RoutingRuleAction>,
    #[serde(default)]
    pub conditions: Vec<RoutingRuleCondition>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "DomainNameId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_id: Option<String>,
    #[serde(default)]
    pub priority: i32,
    #[serde(rename = "RoutingRuleId")]
    #[serde(default)]
    pub routing_rule_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRoutingRuleResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<RoutingRuleAction>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<RoutingRuleCondition>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "routingRuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_rule_arn: Option<String>,
    #[serde(rename = "routingRuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReimportApiRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "Basepath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basepath: Option<String>,
    #[serde(default)]
    pub body: String,
    #[serde(rename = "FailOnWarnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReimportApiResponse {
    #[serde(rename = "apiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "apiKeySelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    #[serde(rename = "corsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "disableExecuteApiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_execute_api_endpoint: Option<bool>,
    #[serde(rename = "disableSchemaValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    #[serde(rename = "importInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_info: Option<Vec<String>>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "protocolType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    #[serde(rename = "routeSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetAuthorizersCacheRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "StageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApiMappingRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "ApiMappingId")]
    #[serde(default)]
    pub api_mapping_id: String,
    #[serde(rename = "apiMappingKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_key: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApiMappingResponse {
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "apiMappingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_id: Option<String>,
    #[serde(rename = "apiMappingKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApiRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "apiKeySelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    #[serde(rename = "corsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    #[serde(rename = "credentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "disableExecuteApiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_execute_api_endpoint: Option<bool>,
    #[serde(rename = "disableSchemaValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "routeKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_key: Option<String>,
    #[serde(rename = "routeSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApiResponse {
    #[serde(rename = "apiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "apiKeySelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    #[serde(rename = "corsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "disableExecuteApiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_execute_api_endpoint: Option<bool>,
    #[serde(rename = "disableSchemaValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,
    #[serde(rename = "importInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_info: Option<Vec<String>>,
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "protocolType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    #[serde(rename = "routeSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAuthorizerRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "authorizerCredentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    #[serde(rename = "AuthorizerId")]
    #[serde(default)]
    pub authorizer_id: String,
    #[serde(rename = "authorizerPayloadFormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_payload_format_version: Option<String>,
    #[serde(rename = "authorizerResultTtlInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    #[serde(rename = "authorizerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    #[serde(rename = "authorizerUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    #[serde(rename = "enableSimpleResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_simple_responses: Option<bool>,
    #[serde(rename = "identitySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    #[serde(rename = "identityValidationExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    #[serde(rename = "jwtConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<JWTConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAuthorizerResponse {
    #[serde(rename = "authorizerCredentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials_arn: Option<String>,
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    #[serde(rename = "authorizerPayloadFormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_payload_format_version: Option<String>,
    #[serde(rename = "authorizerResultTtlInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    #[serde(rename = "authorizerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_type: Option<String>,
    #[serde(rename = "authorizerUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    #[serde(rename = "enableSimpleResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_simple_responses: Option<bool>,
    #[serde(rename = "identitySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<Vec<String>>,
    #[serde(rename = "identityValidationExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    #[serde(rename = "jwtConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_configuration: Option<JWTConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeploymentRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "DeploymentId")]
    #[serde(default)]
    pub deployment_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeploymentResponse {
    #[serde(rename = "autoDeployed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deployed: Option<bool>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "deploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    #[serde(rename = "deploymentStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainNameRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "domainNameConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,
    #[serde(rename = "mutualTlsAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_tls_authentication: Option<MutualTlsAuthenticationInput>,
    #[serde(rename = "routingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainNameResponse {
    #[serde(rename = "apiMappingSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mapping_selection_expression: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "domainNameArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_arn: Option<String>,
    #[serde(rename = "domainNameConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,
    #[serde(rename = "mutualTlsAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_tls_authentication: Option<MutualTlsAuthentication>,
    #[serde(rename = "routingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIntegrationRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "connectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "contentHandlingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    #[serde(rename = "credentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IntegrationId")]
    #[serde(default)]
    pub integration_id: String,
    #[serde(rename = "integrationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_method: Option<String>,
    #[serde(rename = "integrationSubtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_subtype: Option<String>,
    #[serde(rename = "integrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    #[serde(rename = "integrationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    #[serde(rename = "passthroughBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    #[serde(rename = "payloadFormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_version: Option<String>,
    #[serde(rename = "requestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "requestTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    #[serde(rename = "templateSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    #[serde(rename = "timeoutInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i32>,
    #[serde(rename = "tlsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_config: Option<TlsConfigInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIntegrationResponseRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "contentHandlingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    #[serde(rename = "IntegrationId")]
    #[serde(default)]
    pub integration_id: String,
    #[serde(rename = "IntegrationResponseId")]
    #[serde(default)]
    pub integration_response_id: String,
    #[serde(rename = "integrationResponseKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_key: Option<String>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "templateSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIntegrationResponseResponse {
    #[serde(rename = "contentHandlingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    #[serde(rename = "integrationResponseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_id: Option<String>,
    #[serde(rename = "integrationResponseKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_key: Option<String>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "templateSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIntegrationResult {
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "connectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "contentHandlingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,
    #[serde(rename = "credentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "integrationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<String>,
    #[serde(rename = "integrationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_method: Option<String>,
    #[serde(rename = "integrationResponseSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_response_selection_expression: Option<String>,
    #[serde(rename = "integrationSubtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_subtype: Option<String>,
    #[serde(rename = "integrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    #[serde(rename = "integrationUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_uri: Option<String>,
    #[serde(rename = "passthroughBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    #[serde(rename = "payloadFormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_version: Option<String>,
    #[serde(rename = "requestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "requestTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    #[serde(rename = "templateSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
    #[serde(rename = "timeoutInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i32>,
    #[serde(rename = "tlsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_config: Option<TlsConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateModelRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "contentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ModelId")]
    #[serde(default)]
    pub model_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateModelResponse {
    #[serde(rename = "contentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "modelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePortalProductRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<DisplayOrder>,
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePortalProductResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<DisplayOrder>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "portalProductArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_product_arn: Option<String>,
    #[serde(rename = "portalProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_product_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePortalRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[serde(rename = "endpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfigurationRequest>,
    #[serde(rename = "includedPortalProductArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_portal_product_arns: Option<Vec<String>>,
    #[serde(rename = "logoUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    #[serde(rename = "portalContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_content: Option<PortalContent>,
    #[serde(rename = "PortalId")]
    #[serde(default)]
    pub portal_id: String,
    #[serde(rename = "rumAppMonitorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rum_app_monitor_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePortalResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[serde(rename = "endpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfigurationResponse>,
    #[serde(rename = "includedPortalProductArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_portal_product_arns: Option<Vec<String>>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "lastPublished")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_published: Option<String>,
    #[serde(rename = "lastPublishedDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_published_description: Option<String>,
    #[serde(rename = "portalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_arn: Option<String>,
    #[serde(rename = "portalContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_content: Option<PortalContent>,
    #[serde(rename = "portalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<Preview>,
    #[serde(rename = "publishStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_status: Option<String>,
    #[serde(rename = "rumAppMonitorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rum_app_monitor_name: Option<String>,
    #[serde(rename = "statusException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_exception: Option<StatusException>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProductPageRequest {
    #[serde(rename = "displayContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_content: Option<DisplayContent>,
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
    #[serde(rename = "ProductPageId")]
    #[serde(default)]
    pub product_page_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProductPageResponse {
    #[serde(rename = "displayContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_content: Option<DisplayContent>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "productPageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_page_arn: Option<String>,
    #[serde(rename = "productPageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_page_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProductRestEndpointPageRequest {
    #[serde(rename = "displayContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_content: Option<EndpointDisplayContent>,
    #[serde(rename = "PortalProductId")]
    #[serde(default)]
    pub portal_product_id: String,
    #[serde(rename = "ProductRestEndpointPageId")]
    #[serde(default)]
    pub product_rest_endpoint_page_id: String,
    #[serde(rename = "tryItState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub try_it_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProductRestEndpointPageResponse {
    #[serde(rename = "displayContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_content: Option<EndpointDisplayContentResponse>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "productRestEndpointPageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_rest_endpoint_page_arn: Option<String>,
    #[serde(rename = "productRestEndpointPageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_rest_endpoint_page_id: Option<String>,
    #[serde(rename = "restEndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_endpoint_identifier: Option<RestEndpointIdentifier>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_exception: Option<StatusException>,
    #[serde(rename = "tryItState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub try_it_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRouteRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "apiKeyRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    #[serde(rename = "authorizationScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    #[serde(rename = "authorizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    #[serde(rename = "modelSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    #[serde(rename = "operationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(rename = "requestModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "requestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, ParameterConstraints>>,
    #[serde(rename = "RouteId")]
    #[serde(default)]
    pub route_id: String,
    #[serde(rename = "routeKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_key: Option<String>,
    #[serde(rename = "routeResponseSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRouteResponseRequest {
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "modelSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    #[serde(rename = "responseModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, ParameterConstraints>>,
    #[serde(rename = "RouteId")]
    #[serde(default)]
    pub route_id: String,
    #[serde(rename = "RouteResponseId")]
    #[serde(default)]
    pub route_response_id: String,
    #[serde(rename = "routeResponseKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRouteResponseResponse {
    #[serde(rename = "modelSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    #[serde(rename = "responseModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, ParameterConstraints>>,
    #[serde(rename = "routeResponseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_id: Option<String>,
    #[serde(rename = "routeResponseKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRouteResult {
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "apiKeyRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    #[serde(rename = "authorizationScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    #[serde(rename = "authorizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    #[serde(rename = "modelSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_selection_expression: Option<String>,
    #[serde(rename = "operationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(rename = "requestModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "requestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, ParameterConstraints>>,
    #[serde(rename = "routeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    #[serde(rename = "routeKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_key: Option<String>,
    #[serde(rename = "routeResponseSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_response_selection_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStageRequest {
    #[serde(rename = "accessLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    #[serde(rename = "ApiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "autoDeploy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    #[serde(rename = "clientCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    #[serde(rename = "defaultRouteSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<RouteSettings>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "routeSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<std::collections::HashMap<String, RouteSettings>>,
    #[serde(rename = "StageName")]
    #[serde(default)]
    pub stage_name: String,
    #[serde(rename = "stageVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStageResponse {
    #[serde(rename = "accessLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    #[serde(rename = "apiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "autoDeploy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    #[serde(rename = "clientCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "defaultRouteSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<RouteSettings>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastDeploymentStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_status_message: Option<String>,
    #[serde(rename = "lastUpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<String>,
    #[serde(rename = "routeSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<std::collections::HashMap<String, RouteSettings>>,
    #[serde(rename = "stageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(rename = "stageVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVpcLinkRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "VpcLinkId")]
    #[serde(default)]
    pub vpc_link_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVpcLinkResponse {
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "vpcLinkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_id: Option<String>,
    #[serde(rename = "vpcLinkStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_status: Option<String>,
    #[serde(rename = "vpcLinkStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_status_message: Option<String>,
    #[serde(rename = "vpcLinkVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_link_version: Option<String>,
}
