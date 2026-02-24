//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-apigatewayv2

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
pub fn serialize_create_api_response(result: &CreateApiResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_api_mapping_response(result: &CreateApiMappingResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_authorizer_response(result: &CreateAuthorizerResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_deployment_response(result: &CreateDeploymentResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_domain_name_response(result: &CreateDomainNameResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_integration_response(result: &CreateIntegrationResult) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_integration_response_response(
    result: &CreateIntegrationResponseResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_model_response(result: &CreateModelResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_portal_response(result: &CreatePortalResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_portal_product_response(
    result: &CreatePortalProductResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_product_page_response(result: &CreateProductPageResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_product_rest_endpoint_page_response(
    result: &CreateProductRestEndpointPageResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_route_response(result: &CreateRouteResult) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_route_response_response(
    result: &CreateRouteResponseResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_routing_rule_response(result: &CreateRoutingRuleResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_stage_response(result: &CreateStageResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_vpc_link_response(result: &CreateVpcLinkResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_access_log_settings_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_api_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_api_mapping_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_authorizer_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_cors_configuration_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_deployment_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_domain_name_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
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
pub fn serialize_delete_model_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_portal_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_portal_product_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_portal_product_sharing_policy_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_product_page_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_product_rest_endpoint_page_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_route_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_route_request_parameter_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_route_response_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_route_settings_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_routing_rule_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_stage_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_vpc_link_response(result: &DeleteVpcLinkResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_disable_portal_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_export_api_response(result: &ExportApiResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_api_response(result: &GetApiResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_api_mapping_response(result: &GetApiMappingResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_api_mappings_response(result: &GetApiMappingsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_apis_response(result: &GetApisResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_authorizer_response(result: &GetAuthorizerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_authorizers_response(result: &GetAuthorizersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_deployment_response(result: &GetDeploymentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_deployments_response(result: &GetDeploymentsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_domain_name_response(result: &GetDomainNameResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_domain_names_response(result: &GetDomainNamesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_integration_response(result: &GetIntegrationResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_integration_response_response(
    result: &GetIntegrationResponseResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_integration_responses_response(
    result: &GetIntegrationResponsesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_integrations_response(result: &GetIntegrationsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_model_response(result: &GetModelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_model_template_response(result: &GetModelTemplateResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_models_response(result: &GetModelsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_portal_response(result: &GetPortalResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_portal_product_response(result: &GetPortalProductResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_portal_product_sharing_policy_response(
    result: &GetPortalProductSharingPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_product_page_response(result: &GetProductPageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_product_rest_endpoint_page_response(
    result: &GetProductRestEndpointPageResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_route_response(result: &GetRouteResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_route_response_response(result: &GetRouteResponseResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_route_responses_response(result: &GetRouteResponsesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_routes_response(result: &GetRoutesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_routing_rule_response(result: &GetRoutingRuleResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_stage_response(result: &GetStageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_stages_response(result: &GetStagesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_tags_response(result: &GetTagsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_vpc_link_response(result: &GetVpcLinkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_vpc_links_response(result: &GetVpcLinksResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_import_api_response(result: &ImportApiResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_portal_products_response(
    result: &ListPortalProductsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_portals_response(result: &ListPortalsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_product_pages_response(result: &ListProductPagesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_product_rest_endpoint_pages_response(
    result: &ListProductRestEndpointPagesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_routing_rules_response(result: &ListRoutingRulesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_preview_portal_response(result: &PreviewPortalResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_publish_portal_response(result: &PublishPortalResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_portal_product_sharing_policy_response(
    result: &PutPortalProductSharingPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_routing_rule_response(result: &PutRoutingRuleResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_reimport_api_response(result: &ReimportApiResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_reset_authorizers_cache_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_api_response(result: &UpdateApiResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_api_mapping_response(result: &UpdateApiMappingResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_authorizer_response(result: &UpdateAuthorizerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_deployment_response(result: &UpdateDeploymentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_domain_name_response(result: &UpdateDomainNameResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_integration_response(result: &UpdateIntegrationResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_integration_response_response(
    result: &UpdateIntegrationResponseResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_model_response(result: &UpdateModelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_portal_response(result: &UpdatePortalResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_portal_product_response(
    result: &UpdatePortalProductResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_product_page_response(result: &UpdateProductPageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_product_rest_endpoint_page_response(
    result: &UpdateProductRestEndpointPageResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_route_response(result: &UpdateRouteResult) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_route_response_response(
    result: &UpdateRouteResponseResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_stage_response(result: &UpdateStageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_vpc_link_response(result: &UpdateVpcLinkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateApiRequest, String> {
    let mut input = CreateApiRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateApiRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateApi request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_api_mapping_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateApiMappingRequest, String> {
    let mut input = CreateApiMappingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateApiMappingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateApiMapping request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
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
            "ApiId" => {
                input.api_id = value.to_string();
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
pub fn deserialize_create_integration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateIntegrationRequest, String> {
    let mut input = CreateIntegrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateIntegrationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateIntegration request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_integration_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateIntegrationResponseRequest, String> {
    let mut input = CreateIntegrationResponseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateIntegrationResponseRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateIntegrationResponse request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "IntegrationId" => {
                input.integration_id = value.to_string();
            }
            _ => {}
        }
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_portal_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePortalRequest, String> {
    let mut input = CreatePortalRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePortalRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreatePortal request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_portal_product_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePortalProductRequest, String> {
    let mut input = CreatePortalProductRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePortalProductRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreatePortalProduct request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_product_page_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateProductPageRequest, String> {
    let mut input = CreateProductPageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateProductPageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateProductPage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_product_rest_endpoint_page_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateProductRestEndpointPageRequest, String> {
    let mut input = CreateProductRestEndpointPageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateProductRestEndpointPageRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateProductRestEndpointPage request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_route_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRouteRequest, String> {
    let mut input = CreateRouteRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRouteRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateRoute request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_route_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRouteResponseRequest, String> {
    let mut input = CreateRouteResponseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRouteResponseRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateRouteResponse request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "RouteId" => {
                input.route_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_routing_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRoutingRuleRequest, String> {
    let mut input = CreateRoutingRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRoutingRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateRoutingRule request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DomainName" => {
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
            "ApiId" => {
                input.api_id = value.to_string();
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
pub fn deserialize_delete_access_log_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessLogSettingsRequest, String> {
    let mut input = DeleteAccessLogSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "StageName" => {
                input.stage_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteApiRequest, String> {
    let mut input = DeleteApiRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_api_mapping_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteApiMappingRequest, String> {
    let mut input = DeleteApiMappingRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiMappingId" => {
                input.api_mapping_id = value.to_string();
            }
            "DomainName" => {
                input.domain_name = value.to_string();
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "AuthorizerId" => {
                input.authorizer_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_cors_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCorsConfigurationRequest, String> {
    let mut input = DeleteCorsConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "DeploymentId" => {
                input.deployment_id = value.to_string();
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
            "DomainName" => {
                input.domain_name = value.to_string();
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "IntegrationId" => {
                input.integration_id = value.to_string();
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "IntegrationId" => {
                input.integration_id = value.to_string();
            }
            "IntegrationResponseId" => {
                input.integration_response_id = value.to_string();
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "ModelId" => {
                input.model_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_portal_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePortalRequest, String> {
    let mut input = DeletePortalRequest::default();
    for (name, value) in labels {
        match *name {
            "PortalId" => {
                input.portal_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_portal_product_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePortalProductRequest, String> {
    let mut input = DeletePortalProductRequest::default();
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_portal_product_sharing_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePortalProductSharingPolicyRequest, String> {
    let mut input = DeletePortalProductSharingPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_product_page_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteProductPageRequest, String> {
    let mut input = DeleteProductPageRequest::default();
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            "ProductPageId" => {
                input.product_page_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_product_rest_endpoint_page_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteProductRestEndpointPageRequest, String> {
    let mut input = DeleteProductRestEndpointPageRequest::default();
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            "ProductRestEndpointPageId" => {
                input.product_rest_endpoint_page_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_route_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRouteRequest, String> {
    let mut input = DeleteRouteRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "RouteId" => {
                input.route_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_route_request_parameter_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRouteRequestParameterRequest, String> {
    let mut input = DeleteRouteRequestParameterRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "RequestParameterKey" => {
                input.request_parameter_key = value.to_string();
            }
            "RouteId" => {
                input.route_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_route_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRouteResponseRequest, String> {
    let mut input = DeleteRouteResponseRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "RouteId" => {
                input.route_id = value.to_string();
            }
            "RouteResponseId" => {
                input.route_response_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_route_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRouteSettingsRequest, String> {
    let mut input = DeleteRouteSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "RouteKey" => {
                input.route_key = value.to_string();
            }
            "StageName" => {
                input.stage_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_routing_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRoutingRuleRequest, String> {
    let mut input = DeleteRoutingRuleRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            "RoutingRuleId" => {
                input.routing_rule_id = value.to_string();
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
pub fn deserialize_delete_stage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteStageRequest, String> {
    let mut input = DeleteStageRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "StageName" => {
                input.stage_name = value.to_string();
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
            "VpcLinkId" => {
                input.vpc_link_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disable_portal_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisablePortalRequest, String> {
    let mut input = DisablePortalRequest::default();
    for (name, value) in labels {
        match *name {
            "PortalId" => {
                input.portal_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_export_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ExportApiRequest, String> {
    let mut input = ExportApiRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "Specification" => {
                input.specification = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("exportVersion") {
        input.export_version = Some(value.to_string());
    }
    if let Some(value) = query.get("includeExtensions") {
        input.include_extensions = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("outputType") {
        input.output_type = value.to_string();
    }
    if let Some(value) = query.get("stageName") {
        input.stage_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetApiRequest, String> {
    let mut input = GetApiRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_api_mapping_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetApiMappingRequest, String> {
    let mut input = GetApiMappingRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiMappingId" => {
                input.api_mapping_id = value.to_string();
            }
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_api_mappings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetApiMappingsRequest, String> {
    let mut input = GetApiMappingsRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_apis_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetApisRequest, String> {
    let mut input = GetApisRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "AuthorizerId" => {
                input.authorizer_id = value.to_string();
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "DeploymentId" => {
                input.deployment_id = value.to_string();
            }
            _ => {}
        }
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
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
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
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
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "IntegrationId" => {
                input.integration_id = value.to_string();
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "IntegrationId" => {
                input.integration_id = value.to_string();
            }
            "IntegrationResponseId" => {
                input.integration_response_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_integration_responses_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetIntegrationResponsesRequest, String> {
    let mut input = GetIntegrationResponsesRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "IntegrationId" => {
                input.integration_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_integrations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetIntegrationsRequest, String> {
    let mut input = GetIntegrationsRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "ModelId" => {
                input.model_id = value.to_string();
            }
            _ => {}
        }
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "ModelId" => {
                input.model_id = value.to_string();
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_portal_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPortalRequest, String> {
    let mut input = GetPortalRequest::default();
    for (name, value) in labels {
        match *name {
            "PortalId" => {
                input.portal_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_portal_product_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPortalProductRequest, String> {
    let mut input = GetPortalProductRequest::default();
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("resourceOwnerAccountId") {
        input.resource_owner_account_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_portal_product_sharing_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPortalProductSharingPolicyRequest, String> {
    let mut input = GetPortalProductSharingPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_product_page_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetProductPageRequest, String> {
    let mut input = GetProductPageRequest::default();
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            "ProductPageId" => {
                input.product_page_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("resourceOwnerAccountId") {
        input.resource_owner_account_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_product_rest_endpoint_page_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetProductRestEndpointPageRequest, String> {
    let mut input = GetProductRestEndpointPageRequest::default();
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            "ProductRestEndpointPageId" => {
                input.product_rest_endpoint_page_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("includeRawDisplayContent") {
        input.include_raw_display_content = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceOwnerAccountId") {
        input.resource_owner_account_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_route_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRouteRequest, String> {
    let mut input = GetRouteRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "RouteId" => {
                input.route_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_route_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRouteResponseRequest, String> {
    let mut input = GetRouteResponseRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "RouteId" => {
                input.route_id = value.to_string();
            }
            "RouteResponseId" => {
                input.route_response_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_route_responses_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRouteResponsesRequest, String> {
    let mut input = GetRouteResponsesRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "RouteId" => {
                input.route_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_routes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRoutesRequest, String> {
    let mut input = GetRoutesRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_routing_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRoutingRuleRequest, String> {
    let mut input = GetRoutingRuleRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            "RoutingRuleId" => {
                input.routing_rule_id = value.to_string();
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
pub fn deserialize_get_stage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetStageRequest, String> {
    let mut input = GetStageRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "StageName" => {
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
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
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
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
            "VpcLinkId" => {
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
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_import_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ImportApiRequest, String> {
    let mut input = ImportApiRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ImportApiRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ImportApi request: {err}"))?;
    }
    if let Some(value) = query.get("basepath") {
        input.basepath = Some(value.to_string());
    }
    if let Some(value) = query.get("failOnWarnings") {
        input.fail_on_warnings = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_portal_products_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPortalProductsRequest, String> {
    let mut input = ListPortalProductsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceOwner") {
        input.resource_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_portals_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPortalsRequest, String> {
    let mut input = ListPortalsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_product_pages_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListProductPagesRequest, String> {
    let mut input = ListProductPagesRequest::default();
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceOwnerAccountId") {
        input.resource_owner_account_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_product_rest_endpoint_pages_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListProductRestEndpointPagesRequest, String> {
    let mut input = ListProductRestEndpointPagesRequest::default();
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceOwnerAccountId") {
        input.resource_owner_account_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_routing_rules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRoutingRulesRequest, String> {
    let mut input = ListRoutingRulesRequest::default();
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("domainNameId") {
        input.domain_name_id = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_preview_portal_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PreviewPortalRequest, String> {
    let mut input = PreviewPortalRequest::default();
    for (name, value) in labels {
        match *name {
            "PortalId" => {
                input.portal_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_publish_portal_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PublishPortalRequest, String> {
    let mut input = PublishPortalRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PublishPortalRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PublishPortal request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "PortalId" => {
                input.portal_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_portal_product_sharing_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutPortalProductSharingPolicyRequest, String> {
    let mut input = PutPortalProductSharingPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutPortalProductSharingPolicyRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutPortalProductSharingPolicy request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_routing_rule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutRoutingRuleRequest, String> {
    let mut input = PutRoutingRuleRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutRoutingRuleRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutRoutingRule request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "DomainName" => {
                input.domain_name = value.to_string();
            }
            "RoutingRuleId" => {
                input.routing_rule_id = value.to_string();
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
pub fn deserialize_reimport_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ReimportApiRequest, String> {
    let mut input = ReimportApiRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ReimportApiRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ReimportApi request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("basepath") {
        input.basepath = Some(value.to_string());
    }
    if let Some(value) = query.get("failOnWarnings") {
        input.fail_on_warnings = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reset_authorizers_cache_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ResetAuthorizersCacheRequest, String> {
    let mut input = ResetAuthorizersCacheRequest::default();
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "StageName" => {
                input.stage_name = value.to_string();
            }
            _ => {}
        }
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
            "ResourceArn" => {
                input.resource_arn = value.to_string();
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
            "ResourceArn" => {
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
pub fn deserialize_update_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateApiRequest, String> {
    let mut input = UpdateApiRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateApiRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateApi request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_api_mapping_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateApiMappingRequest, String> {
    let mut input = UpdateApiMappingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateApiMappingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateApiMapping request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ApiMappingId" => {
                input.api_mapping_id = value.to_string();
            }
            "DomainName" => {
                input.domain_name = value.to_string();
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "AuthorizerId" => {
                input.authorizer_id = value.to_string();
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "DeploymentId" => {
                input.deployment_id = value.to_string();
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
            "DomainName" => {
                input.domain_name = value.to_string();
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "IntegrationId" => {
                input.integration_id = value.to_string();
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "IntegrationId" => {
                input.integration_id = value.to_string();
            }
            "IntegrationResponseId" => {
                input.integration_response_id = value.to_string();
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "ModelId" => {
                input.model_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_portal_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePortalRequest, String> {
    let mut input = UpdatePortalRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePortalRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdatePortal request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "PortalId" => {
                input.portal_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_portal_product_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePortalProductRequest, String> {
    let mut input = UpdatePortalProductRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePortalProductRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdatePortalProduct request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_product_page_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateProductPageRequest, String> {
    let mut input = UpdateProductPageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateProductPageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateProductPage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            "ProductPageId" => {
                input.product_page_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_product_rest_endpoint_page_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateProductRestEndpointPageRequest, String> {
    let mut input = UpdateProductRestEndpointPageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateProductRestEndpointPageRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateProductRestEndpointPage request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "PortalProductId" => {
                input.portal_product_id = value.to_string();
            }
            "ProductRestEndpointPageId" => {
                input.product_rest_endpoint_page_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_route_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRouteRequest, String> {
    let mut input = UpdateRouteRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRouteRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateRoute request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "RouteId" => {
                input.route_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_route_response_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateRouteResponseRequest, String> {
    let mut input = UpdateRouteResponseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateRouteResponseRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateRouteResponse request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "RouteId" => {
                input.route_id = value.to_string();
            }
            "RouteResponseId" => {
                input.route_response_id = value.to_string();
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
            "ApiId" => {
                input.api_id = value.to_string();
            }
            "StageName" => {
                input.stage_name = value.to_string();
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
            "VpcLinkId" => {
                input.vpc_link_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
