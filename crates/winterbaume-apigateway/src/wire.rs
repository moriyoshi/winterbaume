//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-apigateway

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restJson protocol.
pub fn serialize_create_api_key_response(result: &ApiKey) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_authorizer_response(result: &Authorizer) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_base_path_mapping_response(result: &BasePathMapping) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_deployment_response(result: &Deployment) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_documentation_part_response(result: &DocumentationPart) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_documentation_version_response(
    result: &DocumentationVersion,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_domain_name_response(result: &DomainName) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_domain_name_access_association_response(
    result: &DomainNameAccessAssociation,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_model_response(result: &Model) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_request_validator_response(result: &RequestValidator) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_resource_response(result: &Resource) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_rest_api_response(result: &RestApi) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_stage_response(result: &Stage) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_usage_plan_response(result: &UsagePlan) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_usage_plan_key_response(result: &UsagePlanKey) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_vpc_link_response(result: &VpcLink) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_api_key_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_authorizer_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_base_path_mapping_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_client_certificate_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_deployment_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_documentation_part_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_documentation_version_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_domain_name_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_domain_name_access_association_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_gateway_response_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_integration_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_integration_response_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_method_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_method_response_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_model_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_request_validator_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_resource_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_rest_api_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_stage_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_usage_plan_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_usage_plan_key_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_vpc_link_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_flush_stage_authorizers_cache_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_flush_stage_cache_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_generate_client_certificate_response(result: &ClientCertificate) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_account_response(result: &Account) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_api_key_response(result: &ApiKey) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_api_keys_response(result: &ApiKeys) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_authorizer_response(result: &Authorizer) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_authorizers_response(result: &Authorizers) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_base_path_mapping_response(result: &BasePathMapping) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_base_path_mappings_response(result: &BasePathMappings) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_client_certificate_response(result: &ClientCertificate) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_client_certificates_response(result: &ClientCertificates) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_deployment_response(result: &Deployment) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_deployments_response(result: &Deployments) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_documentation_part_response(result: &DocumentationPart) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_documentation_parts_response(result: &DocumentationParts) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_documentation_version_response(result: &DocumentationVersion) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_documentation_versions_response(
    result: &DocumentationVersions,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_domain_name_response(result: &DomainName) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_domain_name_access_associations_response(
    result: &DomainNameAccessAssociations,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_domain_names_response(result: &DomainNames) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_export_response(result: &ExportResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "content-disposition": set by caller from content_disposition field
    // Header "content-type": set by caller from content_type field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_get_gateway_response_response(result: &GatewayResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_gateway_responses_response(result: &GatewayResponses) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_integration_response(result: &Integration) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_integration_response_response(result: &IntegrationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_method_response(result: &Method) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_method_response_response(result: &MethodResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_model_response(result: &Model) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_model_template_response(result: &Template) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_models_response(result: &Models) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_request_validator_response(result: &RequestValidator) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_request_validators_response(result: &RequestValidators) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_response(result: &Resource) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resources_response(result: &Resources) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_rest_api_response(result: &RestApi) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_rest_apis_response(result: &RestApis) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_sdk_response(result: &SdkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "content-disposition": set by caller from content_disposition field
    // Header "content-type": set by caller from content_type field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_get_sdk_type_response(result: &SdkType) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_sdk_types_response(result: &SdkTypes) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_stage_response(result: &Stage) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_stages_response(result: &Stages) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_tags_response(result: &Tags) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_usage_response(result: &Usage) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_usage_plan_response(result: &UsagePlan) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_usage_plan_key_response(result: &UsagePlanKey) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_usage_plan_keys_response(result: &UsagePlanKeys) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_usage_plans_response(result: &UsagePlans) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_vpc_link_response(result: &VpcLink) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_vpc_links_response(result: &VpcLinks) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_import_api_keys_response(result: &ApiKeyIds) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_import_documentation_parts_response(
    result: &DocumentationPartIds,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_import_rest_api_response(result: &RestApi) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_gateway_response_response(result: &GatewayResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_integration_response(result: &Integration) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_integration_response_response(result: &IntegrationResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_method_response(result: &Method) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_method_response_response(result: &MethodResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_rest_api_response(result: &RestApi) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_reject_domain_name_access_association_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_test_invoke_authorizer_response(
    result: &TestInvokeAuthorizerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_test_invoke_method_response(result: &TestInvokeMethodResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_account_response(result: &Account) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_api_key_response(result: &ApiKey) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_authorizer_response(result: &Authorizer) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_base_path_mapping_response(result: &BasePathMapping) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_client_certificate_response(result: &ClientCertificate) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_deployment_response(result: &Deployment) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_documentation_part_response(result: &DocumentationPart) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_documentation_version_response(
    result: &DocumentationVersion,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_domain_name_response(result: &DomainName) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_gateway_response_response(result: &GatewayResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_integration_response(result: &Integration) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_integration_response_response(
    result: &IntegrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_method_response(result: &Method) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_method_response_response(result: &MethodResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_model_response(result: &Model) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_request_validator_response(result: &RequestValidator) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_resource_response(result: &Resource) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_rest_api_response(result: &RestApi) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_stage_response(result: &Stage) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_usage_response(result: &Usage) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_usage_plan_response(result: &UsagePlan) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_vpc_link_response(result: &VpcLink) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_api_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateApiKeyRequest, String> {
    let mut input = CreateApiKeyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateApiKeyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateApiKey request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_authorizer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAuthorizerRequest, String> {
    let mut input = CreateAuthorizerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAuthorizerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAuthorizer request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_base_path_mapping_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBasePathMappingRequest, String> {
    let mut input = CreateBasePathMappingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBasePathMappingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBasePathMapping request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("domainNameId") {
        input.domain_name_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDeploymentRequest, String> {
    let mut input = CreateDeploymentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDeploymentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDeployment request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_documentation_part_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDocumentationPartRequest, String> {
    let mut input = CreateDocumentationPartRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDocumentationPartRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateDocumentationPart request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_documentation_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDocumentationVersionRequest, String> {
    let mut input = CreateDocumentationVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDocumentationVersionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateDocumentationVersion request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_domain_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDomainNameRequest, String> {
    let mut input = CreateDomainNameRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDomainNameRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDomainName request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_domain_name_access_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDomainNameAccessAssociationRequest, String> {
    let mut input = CreateDomainNameAccessAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDomainNameAccessAssociationRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize CreateDomainNameAccessAssociation request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_model_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateModelRequest, String> {
    let mut input = CreateModelRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateModelRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateModel request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_request_validator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRequestValidatorRequest, String> {
    let mut input = CreateRequestValidatorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRequestValidatorRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateRequestValidator request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateResourceRequest, String> {
    let mut input = CreateResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateResource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "parentId" => {
                input.parent_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_rest_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRestApiRequest, String> {
    let mut input = CreateRestApiRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRestApiRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateRestApi request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_stage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateStageRequest, String> {
    let mut input = CreateStageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateStageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateStage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_usage_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateUsagePlanRequest, String> {
    let mut input = CreateUsagePlanRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateUsagePlanRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateUsagePlan request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_usage_plan_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateUsagePlanKeyRequest, String> {
    let mut input = CreateUsagePlanKeyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateUsagePlanKeyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateUsagePlanKey request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "usagePlanId" => {
                input.usage_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_vpc_link_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateVpcLinkRequest, String> {
    let mut input = CreateVpcLinkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateVpcLinkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateVpcLink request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_api_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteApiKeyRequest, String> {
    let mut input = DeleteApiKeyRequest::default();
    for (name, value) in labels {
        match *name {
            "apiKey" => {
                input.api_key = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_authorizer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAuthorizerRequest, String> {
    let mut input = DeleteAuthorizerRequest::default();
    for (name, value) in labels {
        match *name {
            "authorizerId" => {
                input.authorizer_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_base_path_mapping_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBasePathMappingRequest, String> {
    let mut input = DeleteBasePathMappingRequest::default();
    for (name, value) in labels {
        match *name {
            "basePath" => {
                input.base_path = value.to_string();
            }
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("domainNameId") {
        input.domain_name_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_client_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteClientCertificateRequest, String> {
    let mut input = DeleteClientCertificateRequest::default();
    for (name, value) in labels {
        match *name {
            "clientCertificateId" => {
                input.client_certificate_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDeploymentRequest, String> {
    let mut input = DeleteDeploymentRequest::default();
    for (name, value) in labels {
        match *name {
            "deploymentId" => {
                input.deployment_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_documentation_part_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDocumentationPartRequest, String> {
    let mut input = DeleteDocumentationPartRequest::default();
    for (name, value) in labels {
        match *name {
            "documentationPartId" => {
                input.documentation_part_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_documentation_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDocumentationVersionRequest, String> {
    let mut input = DeleteDocumentationVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "documentationVersion" => {
                input.documentation_version = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_domain_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDomainNameRequest, String> {
    let mut input = DeleteDomainNameRequest::default();
    for (name, value) in labels {
        match *name {
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("domainNameId") {
        input.domain_name_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_domain_name_access_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDomainNameAccessAssociationRequest, String> {
    let mut input = DeleteDomainNameAccessAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "domainNameAccessAssociationArn" => {
                input.domain_name_access_association_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_gateway_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteGatewayResponseRequest, String> {
    let mut input = DeleteGatewayResponseRequest::default();
    for (name, value) in labels {
        match *name {
            "responseType" => {
                input.response_type = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_integration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteIntegrationRequest, String> {
    let mut input = DeleteIntegrationRequest::default();
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_integration_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteIntegrationResponseRequest, String> {
    let mut input = DeleteIntegrationResponseRequest::default();
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "statusCode" => {
                input.status_code = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_method_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMethodRequest, String> {
    let mut input = DeleteMethodRequest::default();
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_method_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMethodResponseRequest, String> {
    let mut input = DeleteMethodResponseRequest::default();
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "statusCode" => {
                input.status_code = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_model_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteModelRequest, String> {
    let mut input = DeleteModelRequest::default();
    for (name, value) in labels {
        match *name {
            "modelName" => {
                input.model_name = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_request_validator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRequestValidatorRequest, String> {
    let mut input = DeleteRequestValidatorRequest::default();
    for (name, value) in labels {
        match *name {
            "requestValidatorId" => {
                input.request_validator_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourceRequest, String> {
    let mut input = DeleteResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_rest_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRestApiRequest, String> {
    let mut input = DeleteRestApiRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_stage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteStageRequest, String> {
    let mut input = DeleteStageRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "stageName" => {
                input.stage_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_usage_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteUsagePlanRequest, String> {
    let mut input = DeleteUsagePlanRequest::default();
    for (name, value) in labels {
        match *name {
            "usagePlanId" => {
                input.usage_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_usage_plan_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteUsagePlanKeyRequest, String> {
    let mut input = DeleteUsagePlanKeyRequest::default();
    for (name, value) in labels {
        match *name {
            "keyId" => {
                input.key_id = value.to_string();
            }
            "usagePlanId" => {
                input.usage_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_vpc_link_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteVpcLinkRequest, String> {
    let mut input = DeleteVpcLinkRequest::default();
    for (name, value) in labels {
        match *name {
            "vpcLinkId" => {
                input.vpc_link_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_flush_stage_authorizers_cache_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<FlushStageAuthorizersCacheRequest, String> {
    let mut input = FlushStageAuthorizersCacheRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "stageName" => {
                input.stage_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_flush_stage_cache_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<FlushStageCacheRequest, String> {
    let mut input = FlushStageCacheRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "stageName" => {
                input.stage_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_generate_client_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GenerateClientCertificateRequest, String> {
    let mut input = GenerateClientCertificateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GenerateClientCertificateRequest>(&request.body).map_err(
            |err| format!("failed to deserialize GenerateClientCertificate request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccountRequest, String> {
    let input = GetAccountRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_api_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetApiKeyRequest, String> {
    let mut input = GetApiKeyRequest::default();
    for (name, value) in labels {
        match *name {
            "apiKey" => {
                input.api_key = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("includeValue") {
        input.include_value = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_api_keys_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetApiKeysRequest, String> {
    let mut input = GetApiKeysRequest::default();
    if let Some(value) = query.get("customerId") {
        input.customer_id = Some(value.to_string());
    }
    if let Some(value) = query.get("includeValues") {
        input.include_values = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("name") {
        input.name_query = Some(value.to_string());
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_authorizer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAuthorizerRequest, String> {
    let mut input = GetAuthorizerRequest::default();
    for (name, value) in labels {
        match *name {
            "authorizerId" => {
                input.authorizer_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_authorizers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAuthorizersRequest, String> {
    let mut input = GetAuthorizersRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_base_path_mapping_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBasePathMappingRequest, String> {
    let mut input = GetBasePathMappingRequest::default();
    for (name, value) in labels {
        match *name {
            "basePath" => {
                input.base_path = value.to_string();
            }
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("domainNameId") {
        input.domain_name_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_base_path_mappings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBasePathMappingsRequest, String> {
    let mut input = GetBasePathMappingsRequest::default();
    for (name, value) in labels {
        match *name {
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("domainNameId") {
        input.domain_name_id = Some(value.to_string());
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_client_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetClientCertificateRequest, String> {
    let mut input = GetClientCertificateRequest::default();
    for (name, value) in labels {
        match *name {
            "clientCertificateId" => {
                input.client_certificate_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_client_certificates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetClientCertificatesRequest, String> {
    let mut input = GetClientCertificatesRequest::default();
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDeploymentRequest, String> {
    let mut input = GetDeploymentRequest::default();
    for (name, value) in labels {
        match *name {
            "deploymentId" => {
                input.deployment_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("embed") {
        input.embed = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_deployments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDeploymentsRequest, String> {
    let mut input = GetDeploymentsRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_documentation_part_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDocumentationPartRequest, String> {
    let mut input = GetDocumentationPartRequest::default();
    for (name, value) in labels {
        match *name {
            "documentationPartId" => {
                input.documentation_part_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_documentation_parts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDocumentationPartsRequest, String> {
    let mut input = GetDocumentationPartsRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("locationStatus") {
        input.location_status = Some(value.to_string());
    }
    if let Some(value) = query.get("name") {
        input.name_query = Some(value.to_string());
    }
    if let Some(value) = query.get("path") {
        input.path = Some(value.to_string());
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    if let Some(value) = query.get("type") {
        input.r#type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_documentation_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDocumentationVersionRequest, String> {
    let mut input = GetDocumentationVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "documentationVersion" => {
                input.documentation_version = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_documentation_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDocumentationVersionsRequest, String> {
    let mut input = GetDocumentationVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_domain_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDomainNameRequest, String> {
    let mut input = GetDomainNameRequest::default();
    for (name, value) in labels {
        match *name {
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("domainNameId") {
        input.domain_name_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_domain_name_access_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDomainNameAccessAssociationsRequest, String> {
    let mut input = GetDomainNameAccessAssociationsRequest::default();
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceOwner") {
        input.resource_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_domain_names_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDomainNamesRequest, String> {
    let mut input = GetDomainNamesRequest::default();
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceOwner") {
        input.resource_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_export_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetExportRequest, String> {
    let mut input = GetExportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetExportRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetExport request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "exportType" => {
                input.export_type = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "stageName" => {
                input.stage_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("Accept")
        .and_then(|value| value.to_str().ok())
    {
        input.accepts = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_gateway_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGatewayResponseRequest, String> {
    let mut input = GetGatewayResponseRequest::default();
    for (name, value) in labels {
        match *name {
            "responseType" => {
                input.response_type = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_gateway_responses_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGatewayResponsesRequest, String> {
    let mut input = GetGatewayResponsesRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_integration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetIntegrationRequest, String> {
    let mut input = GetIntegrationRequest::default();
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_integration_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetIntegrationResponseRequest, String> {
    let mut input = GetIntegrationResponseRequest::default();
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "statusCode" => {
                input.status_code = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_method_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMethodRequest, String> {
    let mut input = GetMethodRequest::default();
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_method_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMethodResponseRequest, String> {
    let mut input = GetMethodResponseRequest::default();
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "statusCode" => {
                input.status_code = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_model_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetModelRequest, String> {
    let mut input = GetModelRequest::default();
    for (name, value) in labels {
        match *name {
            "modelName" => {
                input.model_name = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("flatten") {
        input.flatten = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_model_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetModelTemplateRequest, String> {
    let mut input = GetModelTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "modelName" => {
                input.model_name = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_models_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetModelsRequest, String> {
    let mut input = GetModelsRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_request_validator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRequestValidatorRequest, String> {
    let mut input = GetRequestValidatorRequest::default();
    for (name, value) in labels {
        match *name {
            "requestValidatorId" => {
                input.request_validator_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_request_validators_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRequestValidatorsRequest, String> {
    let mut input = GetRequestValidatorsRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourceRequest, String> {
    let mut input = GetResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("embed") {
        input.embed = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourcesRequest, String> {
    let mut input = GetResourcesRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("embed") {
        input.embed = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_rest_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRestApiRequest, String> {
    let mut input = GetRestApiRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_rest_apis_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRestApisRequest, String> {
    let mut input = GetRestApisRequest::default();
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_sdk_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSdkRequest, String> {
    let mut input = GetSdkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetSdkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetSdk request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "sdkType" => {
                input.sdk_type = value.to_string();
            }
            "stageName" => {
                input.stage_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_sdk_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSdkTypeRequest, String> {
    let mut input = GetSdkTypeRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_sdk_types_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSdkTypesRequest, String> {
    let mut input = GetSdkTypesRequest::default();
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_stage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetStageRequest, String> {
    let mut input = GetStageRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "stageName" => {
                input.stage_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_stages_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetStagesRequest, String> {
    let mut input = GetStagesRequest::default();
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("deploymentId") {
        input.deployment_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTagsRequest, String> {
    let mut input = GetTagsRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_usage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetUsageRequest, String> {
    let mut input = GetUsageRequest::default();
    for (name, value) in labels {
        match *name {
            "usagePlanId" => {
                input.usage_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("endDate") {
        input.end_date = value.to_string();
    }
    if let Some(value) = query.get("keyId") {
        input.key_id = Some(value.to_string());
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    if let Some(value) = query.get("startDate") {
        input.start_date = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_usage_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetUsagePlanRequest, String> {
    let mut input = GetUsagePlanRequest::default();
    for (name, value) in labels {
        match *name {
            "usagePlanId" => {
                input.usage_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_usage_plan_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetUsagePlanKeyRequest, String> {
    let mut input = GetUsagePlanKeyRequest::default();
    for (name, value) in labels {
        match *name {
            "keyId" => {
                input.key_id = value.to_string();
            }
            "usagePlanId" => {
                input.usage_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_usage_plan_keys_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetUsagePlanKeysRequest, String> {
    let mut input = GetUsagePlanKeysRequest::default();
    for (name, value) in labels {
        match *name {
            "usagePlanId" => {
                input.usage_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("name") {
        input.name_query = Some(value.to_string());
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_usage_plans_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetUsagePlansRequest, String> {
    let mut input = GetUsagePlansRequest::default();
    if let Some(value) = query.get("keyId") {
        input.key_id = Some(value.to_string());
    }
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_vpc_link_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetVpcLinkRequest, String> {
    let mut input = GetVpcLinkRequest::default();
    for (name, value) in labels {
        match *name {
            "vpcLinkId" => {
                input.vpc_link_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_vpc_links_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetVpcLinksRequest, String> {
    let mut input = GetVpcLinksRequest::default();
    if let Some(value) = query.get("limit") {
        input.limit = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("position") {
        input.position = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_import_api_keys_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ImportApiKeysRequest, String> {
    let mut input = ImportApiKeysRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input.body = body.to_string();
    }
    if let Some(value) = query.get("failonwarnings") {
        input.fail_on_warnings = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("format") {
        input.format = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_import_documentation_parts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ImportDocumentationPartsRequest, String> {
    let mut input = ImportDocumentationPartsRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input.body = body.to_string();
    }
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("failonwarnings") {
        input.fail_on_warnings = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("mode") {
        input.mode = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_import_rest_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ImportRestApiRequest, String> {
    let mut input = ImportRestApiRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input.body = body.to_string();
    }
    if let Some(value) = query.get("failonwarnings") {
        input.fail_on_warnings = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_gateway_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutGatewayResponseRequest, String> {
    let mut input = PutGatewayResponseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutGatewayResponseRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutGatewayResponse request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "responseType" => {
                input.response_type = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_integration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutIntegrationRequest, String> {
    let mut input = PutIntegrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutIntegrationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutIntegration request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_integration_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutIntegrationResponseRequest, String> {
    let mut input = PutIntegrationResponseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutIntegrationResponseRequest>(&request.body).map_err(
            |err| format!("failed to deserialize PutIntegrationResponse request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "statusCode" => {
                input.status_code = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_method_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutMethodRequest, String> {
    let mut input = PutMethodRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutMethodRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutMethod request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_method_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutMethodResponseRequest, String> {
    let mut input = PutMethodResponseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutMethodResponseRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutMethodResponse request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "statusCode" => {
                input.status_code = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_rest_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutRestApiRequest, String> {
    let mut input = PutRestApiRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input.body = body.to_string();
    }
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("failonwarnings") {
        input.fail_on_warnings = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("mode") {
        input.mode = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reject_domain_name_access_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RejectDomainNameAccessAssociationRequest, String> {
    let mut input = RejectDomainNameAccessAssociationRequest::default();
    if let Some(value) = query.get("domainNameAccessAssociationArn") {
        input.domain_name_access_association_arn = value.to_string();
    }
    if let Some(value) = query.get("domainNameArn") {
        input.domain_name_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceRequest, String> {
    let mut input = TagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_test_invoke_authorizer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TestInvokeAuthorizerRequest, String> {
    let mut input = TestInvokeAuthorizerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TestInvokeAuthorizerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TestInvokeAuthorizer request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "authorizerId" => {
                input.authorizer_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_test_invoke_method_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TestInvokeMethodRequest, String> {
    let mut input = TestInvokeMethodRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TestInvokeMethodRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TestInvokeMethod request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("tagKeys") {
        input.tag_keys = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAccountRequest, String> {
    let mut input = UpdateAccountRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAccountRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAccount request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_api_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateApiKeyRequest, String> {
    let mut input = UpdateApiKeyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateApiKeyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateApiKey request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiKey" => {
                input.api_key = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_authorizer_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAuthorizerRequest, String> {
    let mut input = UpdateAuthorizerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAuthorizerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAuthorizer request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "authorizerId" => {
                input.authorizer_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_base_path_mapping_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBasePathMappingRequest, String> {
    let mut input = UpdateBasePathMappingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBasePathMappingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBasePathMapping request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "basePath" => {
                input.base_path = value.to_string();
            }
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("domainNameId") {
        input.domain_name_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_client_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateClientCertificateRequest, String> {
    let mut input = UpdateClientCertificateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateClientCertificateRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateClientCertificate request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "clientCertificateId" => {
                input.client_certificate_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDeploymentRequest, String> {
    let mut input = UpdateDeploymentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDeploymentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDeployment request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "deploymentId" => {
                input.deployment_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_documentation_part_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDocumentationPartRequest, String> {
    let mut input = UpdateDocumentationPartRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDocumentationPartRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateDocumentationPart request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "documentationPartId" => {
                input.documentation_part_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_documentation_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDocumentationVersionRequest, String> {
    let mut input = UpdateDocumentationVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDocumentationVersionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateDocumentationVersion request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "documentationVersion" => {
                input.documentation_version = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_domain_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDomainNameRequest, String> {
    let mut input = UpdateDomainNameRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDomainNameRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDomainName request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("domainNameId") {
        input.domain_name_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_gateway_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateGatewayResponseRequest, String> {
    let mut input = UpdateGatewayResponseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateGatewayResponseRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateGatewayResponse request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "responseType" => {
                input.response_type = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_integration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateIntegrationRequest, String> {
    let mut input = UpdateIntegrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateIntegrationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateIntegration request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_integration_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateIntegrationResponseRequest, String> {
    let mut input = UpdateIntegrationResponseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateIntegrationResponseRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateIntegrationResponse request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "statusCode" => {
                input.status_code = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_method_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateMethodRequest, String> {
    let mut input = UpdateMethodRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateMethodRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateMethod request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_method_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateMethodResponseRequest, String> {
    let mut input = UpdateMethodResponseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateMethodResponseRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateMethodResponse request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "httpMethod" => {
                input.http_method = value.to_string();
            }
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "statusCode" => {
                input.status_code = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_model_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateModelRequest, String> {
    let mut input = UpdateModelRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateModelRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateModel request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "modelName" => {
                input.model_name = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_request_validator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRequestValidatorRequest, String> {
    let mut input = UpdateRequestValidatorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRequestValidatorRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateRequestValidator request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "requestValidatorId" => {
                input.request_validator_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateResourceRequest, String> {
    let mut input = UpdateResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateResource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "resourceId" => {
                input.resource_id = value.to_string();
            }
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_rest_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRestApiRequest, String> {
    let mut input = UpdateRestApiRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRestApiRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateRestApi request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_stage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateStageRequest, String> {
    let mut input = UpdateStageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateStageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateStage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "restApiId" => {
                input.rest_api_id = value.to_string();
            }
            "stageName" => {
                input.stage_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_usage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUsageRequest, String> {
    let mut input = UpdateUsageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUsageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateUsage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "keyId" => {
                input.key_id = value.to_string();
            }
            "usagePlanId" => {
                input.usage_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_usage_plan_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUsagePlanRequest, String> {
    let mut input = UpdateUsagePlanRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUsagePlanRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateUsagePlan request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "usagePlanId" => {
                input.usage_plan_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_vpc_link_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateVpcLinkRequest, String> {
    let mut input = UpdateVpcLinkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateVpcLinkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateVpcLink request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "vpcLinkId" => {
                input.vpc_link_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
