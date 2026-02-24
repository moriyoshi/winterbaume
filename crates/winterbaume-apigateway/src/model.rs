//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-apigateway

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Account {
    #[serde(rename = "apiKeyVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_version: Option<String>,
    #[serde(rename = "cloudwatchRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(rename = "throttleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_settings: Option<ThrottleSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThrottleSettings {
    #[serde(rename = "burstLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burst_limit: Option<i32>,
    #[serde(rename = "rateLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiKey {
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "customerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "stageKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_keys: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiKeyIds {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiKeys {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ApiKey>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Authorizer {
    #[serde(rename = "authType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "authorizerCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials: Option<String>,
    #[serde(rename = "authorizerResultTtlInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    #[serde(rename = "authorizerUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "identitySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<String>,
    #[serde(rename = "identityValidationExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "providerARNs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_a_r_ns: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Authorizers {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Authorizer>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BasePathMapping {
    #[serde(rename = "basePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_api_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BasePathMappings {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BasePathMapping>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientCertificate {
    #[serde(rename = "clientCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    #[serde(rename = "pemEncodedCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem_encoded_certificate: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientCertificates {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ClientCertificate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApiKeyRequest {
    #[serde(rename = "customerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "generateDistinctId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_distinct_id: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "stageKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_keys: Option<Vec<StageKey>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StageKey {
    #[serde(rename = "restApiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_api_id: Option<String>,
    #[serde(rename = "stageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAuthorizerRequest {
    #[serde(rename = "authType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "authorizerCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials: Option<String>,
    #[serde(rename = "authorizerResultTtlInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    #[serde(rename = "authorizerUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    #[serde(rename = "identitySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<String>,
    #[serde(rename = "identityValidationExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "providerARNs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_a_r_ns: Option<Vec<String>>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBasePathMappingRequest {
    #[serde(rename = "basePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "domainNameId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_id: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeploymentRequest {
    #[serde(rename = "cacheClusterEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_enabled: Option<bool>,
    #[serde(rename = "cacheClusterSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_size: Option<String>,
    #[serde(rename = "canarySettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_settings: Option<DeploymentCanarySettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "stageDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_description: Option<String>,
    #[serde(rename = "stageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(rename = "tracingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentCanarySettings {
    #[serde(rename = "percentTraffic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_traffic: Option<f64>,
    #[serde(rename = "stageVariableOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variable_overrides: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "useStageCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stage_cache: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDocumentationPartRequest {
    #[serde(default)]
    pub location: DocumentationPartLocation,
    #[serde(default)]
    pub properties: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentationPartLocation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDocumentationVersionRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "documentationVersion")]
    #[serde(default)]
    pub documentation_version: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDomainNameAccessAssociationRequest {
    #[serde(rename = "accessAssociationSource")]
    #[serde(default)]
    pub access_association_source: String,
    #[serde(rename = "accessAssociationSourceType")]
    #[serde(default)]
    pub access_association_source_type: String,
    #[serde(rename = "domainNameArn")]
    #[serde(default)]
    pub domain_name_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDomainNameRequest {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_body: Option<String>,
    #[serde(rename = "certificateChain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    #[serde(rename = "certificateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    #[serde(rename = "certificatePrivateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_private_key: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "endpointAccessMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_access_mode: Option<String>,
    #[serde(rename = "endpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfiguration>,
    #[serde(rename = "mutualTlsAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_tls_authentication: Option<MutualTlsAuthenticationInput>,
    #[serde(rename = "ownershipVerificationCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_verification_certificate_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "regionalCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_certificate_arn: Option<String>,
    #[serde(rename = "regionalCertificateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_certificate_name: Option<String>,
    #[serde(rename = "routingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mode: Option<String>,
    #[serde(rename = "securityPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointConfiguration {
    #[serde(rename = "ipAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    #[serde(rename = "vpcEndpointIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_ids: Option<Vec<String>>,
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
pub struct CreateModelRequest {
    #[serde(rename = "contentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRequestValidatorRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "validateRequestBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_request_body: Option<bool>,
    #[serde(rename = "validateRequestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_request_parameters: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceRequest {
    #[serde(rename = "parentId")]
    #[serde(default)]
    pub parent_id: String,
    #[serde(rename = "pathPart")]
    #[serde(default)]
    pub path_part: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRestApiRequest {
    #[serde(rename = "apiKeySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_source: Option<String>,
    #[serde(rename = "binaryMediaTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_media_types: Option<Vec<String>>,
    #[serde(rename = "cloneFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_from: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "disableExecuteApiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_execute_api_endpoint: Option<bool>,
    #[serde(rename = "endpointAccessMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_access_mode: Option<String>,
    #[serde(rename = "endpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfiguration>,
    #[serde(rename = "minimumCompressionSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_compression_size: Option<i32>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "securityPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStageRequest {
    #[serde(rename = "cacheClusterEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_enabled: Option<bool>,
    #[serde(rename = "cacheClusterSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_size: Option<String>,
    #[serde(rename = "canarySettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_settings: Option<CanarySettings>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    pub deployment_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "documentationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_version: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "tracingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanarySettings {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "percentTraffic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_traffic: Option<f64>,
    #[serde(rename = "stageVariableOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variable_overrides: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "useStageCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stage_cache: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUsagePlanKeyRequest {
    #[serde(rename = "keyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "keyType")]
    #[serde(default)]
    pub key_type: String,
    #[serde(rename = "usagePlanId")]
    #[serde(default)]
    pub usage_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUsagePlanRequest {
    #[serde(rename = "apiStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_stages: Option<Vec<ApiStage>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<QuotaSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle: Option<ThrottleSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiStage {
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle: Option<std::collections::HashMap<String, ThrottleSettings>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuotaSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcLinkRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetArns")]
    #[serde(default)]
    pub target_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApiKeyRequest {
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAuthorizerRequest {
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    pub authorizer_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBasePathMappingRequest {
    #[serde(rename = "basePath")]
    #[serde(default)]
    pub base_path: String,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "domainNameId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClientCertificateRequest {
    #[serde(rename = "clientCertificateId")]
    #[serde(default)]
    pub client_certificate_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeploymentRequest {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    pub deployment_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDocumentationPartRequest {
    #[serde(rename = "documentationPartId")]
    #[serde(default)]
    pub documentation_part_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDocumentationVersionRequest {
    #[serde(rename = "documentationVersion")]
    #[serde(default)]
    pub documentation_version: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainNameAccessAssociationRequest {
    #[serde(rename = "domainNameAccessAssociationArn")]
    #[serde(default)]
    pub domain_name_access_association_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainNameRequest {
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "domainNameId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGatewayResponseRequest {
    #[serde(rename = "responseType")]
    #[serde(default)]
    pub response_type: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIntegrationRequest {
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIntegrationResponseRequest {
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    pub status_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMethodRequest {
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMethodResponseRequest {
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    pub status_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteModelRequest {
    #[serde(rename = "modelName")]
    #[serde(default)]
    pub model_name: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRequestValidatorRequest {
    #[serde(rename = "requestValidatorId")]
    #[serde(default)]
    pub request_validator_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceRequest {
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRestApiRequest {
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStageRequest {
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUsagePlanKeyRequest {
    #[serde(rename = "keyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "usagePlanId")]
    #[serde(default)]
    pub usage_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUsagePlanRequest {
    #[serde(rename = "usagePlanId")]
    #[serde(default)]
    pub usage_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcLinkRequest {
    #[serde(rename = "vpcLinkId")]
    #[serde(default)]
    pub vpc_link_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Deployment {
    #[serde(rename = "apiSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_summary: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, MethodSnapshot>>,
    >,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MethodSnapshot {
    #[serde(rename = "apiKeyRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    #[serde(rename = "authorizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Deployments {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Deployment>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentationPart {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<DocumentationPartLocation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentationPartIds {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentationParts {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DocumentationPart>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentationVersion {
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentationVersions {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DocumentationVersion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainName {
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
    pub certificate_upload_date: Option<f64>,
    #[serde(rename = "distributionDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_domain_name: Option<String>,
    #[serde(rename = "distributionHostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_hosted_zone_id: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "domainNameArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_arn: Option<String>,
    #[serde(rename = "domainNameId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_id: Option<String>,
    #[serde(rename = "domainNameStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_status: Option<String>,
    #[serde(rename = "domainNameStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_status_message: Option<String>,
    #[serde(rename = "endpointAccessMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_access_mode: Option<String>,
    #[serde(rename = "endpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfiguration>,
    #[serde(rename = "managementPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_policy: Option<String>,
    #[serde(rename = "mutualTlsAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_tls_authentication: Option<MutualTlsAuthentication>,
    #[serde(rename = "ownershipVerificationCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_verification_certificate_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "regionalCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_certificate_arn: Option<String>,
    #[serde(rename = "regionalCertificateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_certificate_name: Option<String>,
    #[serde(rename = "regionalDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_domain_name: Option<String>,
    #[serde(rename = "regionalHostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_hosted_zone_id: Option<String>,
    #[serde(rename = "routingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mode: Option<String>,
    #[serde(rename = "securityPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy: Option<String>,
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
pub struct DomainNameAccessAssociation {
    #[serde(rename = "accessAssociationSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_association_source: Option<String>,
    #[serde(rename = "accessAssociationSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_association_source_type: Option<String>,
    #[serde(rename = "domainNameAccessAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_access_association_arn: Option<String>,
    #[serde(rename = "domainNameArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainNameAccessAssociations {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DomainNameAccessAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainNames {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DomainName>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "contentDisposition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_disposition: Option<String>,
    #[serde(rename = "contentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlushStageAuthorizersCacheRequest {
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlushStageCacheRequest {
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GatewayResponse {
    #[serde(rename = "defaultResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_response: Option<bool>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GatewayResponses {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<GatewayResponse>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateClientCertificateRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApiKeyRequest {
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    #[serde(rename = "includeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApiKeysRequest {
    #[serde(rename = "customerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(rename = "includeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_values: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nameQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_query: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAuthorizerRequest {
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    pub authorizer_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAuthorizersRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBasePathMappingRequest {
    #[serde(rename = "basePath")]
    #[serde(default)]
    pub base_path: String,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "domainNameId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBasePathMappingsRequest {
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "domainNameId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClientCertificateRequest {
    #[serde(rename = "clientCertificateId")]
    #[serde(default)]
    pub client_certificate_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClientCertificatesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentRequest {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    pub deployment_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Vec<String>>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDocumentationPartRequest {
    #[serde(rename = "documentationPartId")]
    #[serde(default)]
    pub documentation_part_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDocumentationPartsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "locationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_status: Option<String>,
    #[serde(rename = "nameQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_query: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDocumentationVersionRequest {
    #[serde(rename = "documentationVersion")]
    #[serde(default)]
    pub documentation_version: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDocumentationVersionsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainNameAccessAssociationsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "resourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainNameRequest {
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "domainNameId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainNamesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "resourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExportRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts: Option<String>,
    #[serde(rename = "exportType")]
    #[serde(default)]
    pub export_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGatewayResponseRequest {
    #[serde(rename = "responseType")]
    #[serde(default)]
    pub response_type: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGatewayResponsesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationRequest {
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntegrationResponseRequest {
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    pub status_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMethodRequest {
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMethodResponseRequest {
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    pub status_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten: Option<bool>,
    #[serde(rename = "modelName")]
    #[serde(default)]
    pub model_name: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelTemplateRequest {
    #[serde(rename = "modelName")]
    #[serde(default)]
    pub model_name: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRequestValidatorRequest {
    #[serde(rename = "requestValidatorId")]
    #[serde(default)]
    pub request_validator_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRequestValidatorsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Vec<String>>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRestApiRequest {
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRestApisRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSdkRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "sdkType")]
    #[serde(default)]
    pub sdk_type: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSdkTypeRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSdkTypesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStageRequest {
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStagesRequest {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTagsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsagePlanKeyRequest {
    #[serde(rename = "keyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "usagePlanId")]
    #[serde(default)]
    pub usage_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsagePlanKeysRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nameQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_query: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "usagePlanId")]
    #[serde(default)]
    pub usage_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsagePlanRequest {
    #[serde(rename = "usagePlanId")]
    #[serde(default)]
    pub usage_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsagePlansRequest {
    #[serde(rename = "keyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsageRequest {
    #[serde(rename = "endDate")]
    #[serde(default)]
    pub end_date: String,
    #[serde(rename = "keyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: String,
    #[serde(rename = "usagePlanId")]
    #[serde(default)]
    pub usage_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVpcLinkRequest {
    #[serde(rename = "vpcLinkId")]
    #[serde(default)]
    pub vpc_link_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVpcLinksRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportApiKeysRequest {
    #[serde(default)]
    pub body: String,
    #[serde(rename = "failOnWarnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
    #[serde(default)]
    pub format: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportDocumentationPartsRequest {
    #[serde(default)]
    pub body: String,
    #[serde(rename = "failOnWarnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportRestApiRequest {
    #[serde(default)]
    pub body: String,
    #[serde(rename = "failOnWarnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Integration {
    #[serde(rename = "cacheKeyParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_key_parameters: Option<Vec<String>>,
    #[serde(rename = "cacheNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_namespace: Option<String>,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "connectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "contentHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[serde(rename = "integrationResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_responses: Option<std::collections::HashMap<String, IntegrationResponse>>,
    #[serde(rename = "integrationTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_target: Option<String>,
    #[serde(rename = "passthroughBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    #[serde(rename = "requestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "requestTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseTransferMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_transfer_mode: Option<String>,
    #[serde(rename = "timeoutInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i32>,
    #[serde(rename = "tlsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_config: Option<TlsConfig>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationResponse {
    #[serde(rename = "contentHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<String>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "selectionPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_pattern: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TlsConfig {
    #[serde(rename = "insecureSkipVerification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_skip_verification: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Method {
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
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[serde(rename = "methodIntegration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_integration: Option<Integration>,
    #[serde(rename = "methodResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_responses: Option<std::collections::HashMap<String, MethodResponse>>,
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
    pub request_parameters: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "requestValidatorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_validator_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MethodResponse {
    #[serde(rename = "responseModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Models {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Model>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutGatewayResponseRequest {
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseType")]
    #[serde(default)]
    pub response_type: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutIntegrationRequest {
    #[serde(rename = "cacheKeyParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_key_parameters: Option<Vec<String>>,
    #[serde(rename = "cacheNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_namespace: Option<String>,
    #[serde(rename = "connectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "connectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "contentHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    #[serde(rename = "requestHttpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_http_method: Option<String>,
    #[serde(rename = "integrationTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_target: Option<String>,
    #[serde(rename = "passthroughBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    #[serde(rename = "requestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "requestTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "responseTransferMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_transfer_mode: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "timeoutInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i32>,
    #[serde(rename = "tlsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_config: Option<TlsConfig>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutIntegrationResponseRequest {
    #[serde(rename = "contentHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<String>,
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "selectionPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_pattern: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    pub status_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutMethodRequest {
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
    pub authorization_type: String,
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
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
    pub request_parameters: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "requestValidatorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_validator_id: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutMethodResponseRequest {
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "responseModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "responseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    pub status_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRestApiRequest {
    #[serde(default)]
    pub body: String,
    #[serde(rename = "failOnWarnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectDomainNameAccessAssociationRequest {
    #[serde(rename = "domainNameAccessAssociationArn")]
    #[serde(default)]
    pub domain_name_access_association_arn: String,
    #[serde(rename = "domainNameArn")]
    #[serde(default)]
    pub domain_name_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestValidator {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "validateRequestBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_request_body: Option<bool>,
    #[serde(rename = "validateRequestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_request_parameters: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestValidators {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RequestValidator>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Resource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "parentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "pathPart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_part: Option<String>,
    #[serde(rename = "resourceMethods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_methods: Option<std::collections::HashMap<String, Method>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Resources {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Resource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestApi {
    #[serde(rename = "apiKeySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_source: Option<String>,
    #[serde(rename = "apiStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_status: Option<String>,
    #[serde(rename = "apiStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_status_message: Option<String>,
    #[serde(rename = "binaryMediaTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_media_types: Option<Vec<String>>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "disableExecuteApiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_execute_api_endpoint: Option<bool>,
    #[serde(rename = "endpointAccessMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_access_mode: Option<String>,
    #[serde(rename = "endpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "minimumCompressionSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_compression_size: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "rootResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_resource_id: Option<String>,
    #[serde(rename = "securityPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy: Option<String>,
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
pub struct RestApis {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RestApi>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SdkResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "contentDisposition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_disposition: Option<String>,
    #[serde(rename = "contentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SdkType {
    #[serde(rename = "configurationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_properties: Option<Vec<SdkConfigurationProperty>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SdkConfigurationProperty {
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "friendlyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SdkTypes {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<SdkType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Stage {
    #[serde(rename = "accessLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    #[serde(rename = "cacheClusterEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_enabled: Option<bool>,
    #[serde(rename = "cacheClusterSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_size: Option<String>,
    #[serde(rename = "cacheClusterStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_status: Option<String>,
    #[serde(rename = "canarySettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_settings: Option<CanarySettings>,
    #[serde(rename = "clientCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "documentationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_version: Option<String>,
    #[serde(rename = "lastUpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    #[serde(rename = "methodSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_settings: Option<std::collections::HashMap<String, MethodSetting>>,
    #[serde(rename = "stageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "tracingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "webAclArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_arn: Option<String>,
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
pub struct MethodSetting {
    #[serde(rename = "cacheDataEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_data_encrypted: Option<bool>,
    #[serde(rename = "cacheTtlInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_ttl_in_seconds: Option<i32>,
    #[serde(rename = "cachingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching_enabled: Option<bool>,
    #[serde(rename = "dataTraceEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_trace_enabled: Option<bool>,
    #[serde(rename = "loggingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    #[serde(rename = "metricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_enabled: Option<bool>,
    #[serde(rename = "requireAuthorizationForCacheControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_authorization_for_cache_control: Option<bool>,
    #[serde(rename = "throttlingBurstLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_burst_limit: Option<i32>,
    #[serde(rename = "throttlingRateLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_rate_limit: Option<f64>,
    #[serde(rename = "unauthorizedCacheControlHeaderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unauthorized_cache_control_header_strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Stages {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<Stage>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tags {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Template {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestInvokeAuthorizerRequest {
    #[serde(rename = "additionalContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    pub authorizer_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "multiValueHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_headers: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "pathWithQueryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_with_query_string: Option<String>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "stageVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestInvokeAuthorizerResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "clientStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_status: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "principalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestInvokeMethodRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "clientCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "multiValueHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_headers: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "pathWithQueryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_with_query_string: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "stageVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestInvokeMethodResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
    #[serde(rename = "multiValueHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_headers: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountRequest {
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PatchOperation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApiKeyRequest {
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAuthorizerRequest {
    #[serde(rename = "authorizerId")]
    #[serde(default)]
    pub authorizer_id: String,
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBasePathMappingRequest {
    #[serde(rename = "basePath")]
    #[serde(default)]
    pub base_path: String,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "domainNameId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_id: Option<String>,
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClientCertificateRequest {
    #[serde(rename = "clientCertificateId")]
    #[serde(default)]
    pub client_certificate_id: String,
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeploymentRequest {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    pub deployment_id: String,
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDocumentationPartRequest {
    #[serde(rename = "documentationPartId")]
    #[serde(default)]
    pub documentation_part_id: String,
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDocumentationVersionRequest {
    #[serde(rename = "documentationVersion")]
    #[serde(default)]
    pub documentation_version: String,
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainNameRequest {
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "domainNameId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_id: Option<String>,
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGatewayResponseRequest {
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "responseType")]
    #[serde(default)]
    pub response_type: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIntegrationRequest {
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIntegrationResponseRequest {
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    pub status_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMethodRequest {
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMethodResponseRequest {
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    pub status_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateModelRequest {
    #[serde(rename = "modelName")]
    #[serde(default)]
    pub model_name: String,
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRequestValidatorRequest {
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "requestValidatorId")]
    #[serde(default)]
    pub request_validator_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceRequest {
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRestApiRequest {
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStageRequest {
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "restApiId")]
    #[serde(default)]
    pub rest_api_id: String,
    #[serde(rename = "stageName")]
    #[serde(default)]
    pub stage_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUsagePlanRequest {
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "usagePlanId")]
    #[serde(default)]
    pub usage_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUsageRequest {
    #[serde(rename = "keyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "usagePlanId")]
    #[serde(default)]
    pub usage_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVpcLinkRequest {
    #[serde(rename = "patchOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    #[serde(rename = "vpcLinkId")]
    #[serde(default)]
    pub vpc_link_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Usage {
    #[serde(rename = "endDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<std::collections::HashMap<String, Vec<Vec<i64>>>>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "usagePlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_plan_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsagePlan {
    #[serde(rename = "apiStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_stages: Option<Vec<ApiStage>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "productCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<QuotaSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle: Option<ThrottleSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsagePlanKey {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsagePlanKeys {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UsagePlanKey>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsagePlans {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UsagePlan>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcLink {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcLinks {
    #[serde(rename = "item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<VpcLink>>,
}
