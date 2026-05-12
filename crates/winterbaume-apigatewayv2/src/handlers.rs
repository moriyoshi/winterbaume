use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use tokio::sync;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::model::{
    Api as ModelApi, ApiMapping as ModelApiMapping, Authorizer as ModelAuthorizer,
    CreateApiMappingResponse, CreateApiResponse, CreateAuthorizerResponse,
    CreateDeploymentResponse, CreateDomainNameResponse, CreateIntegrationResponseResponse,
    CreateIntegrationResult, CreateModelResponse, CreateRouteResponseResponse, CreateRouteResult,
    CreateStageResponse, CreateVpcLinkResponse, Deployment as ModelDeployment,
    DomainName as ModelDomainName, GetApiMappingResponse, GetApiMappingsResponse, GetApiResponse,
    GetApisResponse, GetAuthorizerResponse, GetAuthorizersResponse, GetDeploymentResponse,
    GetDeploymentsResponse, GetDomainNameResponse, GetDomainNamesResponse,
    GetIntegrationResponseResponse, GetIntegrationResponsesResponse, GetIntegrationResult,
    GetIntegrationsResponse, GetModelResponse, GetModelsResponse, GetRouteResponseResponse,
    GetRouteResponsesResponse, GetRouteResult, GetRoutesResponse, GetStageResponse,
    GetStagesResponse, GetTagsResponse, GetVpcLinkResponse, GetVpcLinksResponse,
    Integration as ModelIntegration, IntegrationResponse as ModelIntegrationResponse,
    Model as ModelModel, ReimportApiResponse, Route as ModelRoute,
    RouteResponse as ModelRouteResponse, Stage as ModelStage, TagResourceResponse,
    UpdateApiResponse, UpdateAuthorizerResponse, UpdateIntegrationResponseResponse,
    UpdateIntegrationResult, UpdateModelResponse, UpdateRouteResult, UpdateStageResponse,
    UpdateVpcLinkResponse, VpcLink as ModelVpcLink,
};
use crate::state::{ApiGatewayV2Error, ApiGatewayV2State};
use crate::views::ApiGatewayV2StateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ApiGatewayV2Service {
    pub(crate) state: Arc<BackendState<ApiGatewayV2State>>,
    pub(crate) notifier: StateChangeNotifier<ApiGatewayV2StateView>,
}

impl ApiGatewayV2Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ApiGatewayV2Service {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ApiGatewayV2Service {
    fn service_name(&self) -> &str {
        // FIX(e2e): The Go SDK signs API Gateway V2 requests with "apigateway" (not
        // "apigatewayv2") in the SigV4 credential scope. When the provider uses a custom
        // endpoint (e.g., http://127.0.0.1:{port}), the router falls back to matching the
        // service name extracted from the Authorization header, so we must match that name.
        "apigateway"
    }

    fn url_patterns(&self) -> Vec<&str> {
        // Match apigateway endpoint URLs that contain a /v2/ path segment.
        // This ensures v1 requests (which don't start with /v2/) are not intercepted here.
        vec![
            r"https?://apigateway\.[^/]+\.amazonaws\.com/v2/",
            r"https?://[^/]+/v2/",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ApiGatewayV2Service {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        let query_string = if let Some(pos) = request.uri.find('?') {
            &request.uri[pos + 1..]
        } else {
            ""
        };
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(query_string);

        // All v2 routes start with /v2/
        // URL-decode path segments so that %24default → $default, etc.
        let raw_segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        if raw_segments.is_empty() || raw_segments[0] != "v2" {
            return rest_json_error(404, "NotFoundException", "Not found");
        }
        let decoded_segments: Vec<String> = raw_segments[1..]
            .iter()
            .map(|s| percent_decode(s))
            .collect();
        let segments: Vec<&str> = decoded_segments.iter().map(|s| s.as_str()).collect();

        let response = match (method, segments.as_slice()) {
            // POST /v2/apis - CreateApi
            ("POST", ["apis"]) => {
                self.handle_create_api(&state, &request, &[], &query_map)
                    .await
            }
            // GET /v2/apis - GetApis
            ("GET", ["apis"]) => {
                self.handle_get_apis(&state, &request, &[], &query_map)
                    .await
            }
            // GET /v2/apis/{apiId} - GetApi
            ("GET", ["apis", api_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_get_api(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/apis/{apiId} - DeleteApi
            ("DELETE", ["apis", api_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_delete_api(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /v2/apis/{apiId} - UpdateApi
            ("PATCH", ["apis", api_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_update_api(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v2/apis/{apiId}/stages - CreateStage
            ("POST", ["apis", api_id, "stages"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_create_stage(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{apiId}/stages - GetStages
            ("GET", ["apis", api_id, "stages"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_get_stages(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{apiId}/stages/{stageName} - GetStage
            ("GET", ["apis", api_id, "stages", stage_name]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("StageName", stage_name)];
                self.handle_get_stage(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/apis/{apiId}/stages/{stageName} - DeleteStage
            ("DELETE", ["apis", api_id, "stages", stage_name]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("StageName", stage_name)];
                self.handle_delete_stage(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /v2/apis/{apiId}/stages/{stageName} - UpdateStage
            ("PATCH", ["apis", api_id, "stages", stage_name]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("StageName", stage_name)];
                self.handle_update_stage(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v2/apis/{apiId}/integrations - CreateIntegration
            ("POST", ["apis", api_id, "integrations"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_create_integration(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{apiId}/integrations - GetIntegrations
            ("GET", ["apis", api_id, "integrations"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_get_integrations(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{apiId}/integrations/{integrationId} - GetIntegration
            ("GET", ["apis", api_id, "integrations", integration_id]) => {
                let labels: &[(&str, &str)] =
                    &[("ApiId", api_id), ("IntegrationId", integration_id)];
                self.handle_get_integration(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/apis/{apiId}/integrations/{integrationId} - DeleteIntegration
            ("DELETE", ["apis", api_id, "integrations", integration_id]) => {
                let labels: &[(&str, &str)] =
                    &[("ApiId", api_id), ("IntegrationId", integration_id)];
                self.handle_delete_integration(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /v2/apis/{apiId}/integrations/{integrationId} - UpdateIntegration
            ("PATCH", ["apis", api_id, "integrations", integration_id]) => {
                let labels: &[(&str, &str)] =
                    &[("ApiId", api_id), ("IntegrationId", integration_id)];
                self.handle_update_integration(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v2/apis/{apiId}/routes - CreateRoute
            ("POST", ["apis", api_id, "routes"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_create_route(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{apiId}/routes - GetRoutes
            ("GET", ["apis", api_id, "routes"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_get_routes(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{apiId}/routes/{routeId} - GetRoute
            ("GET", ["apis", api_id, "routes", route_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("RouteId", route_id)];
                self.handle_get_route(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/apis/{apiId}/routes/{routeId} - DeleteRoute
            ("DELETE", ["apis", api_id, "routes", route_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("RouteId", route_id)];
                self.handle_delete_route(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /v2/apis/{apiId}/routes/{routeId} - UpdateRoute
            ("PATCH", ["apis", api_id, "routes", route_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("RouteId", route_id)];
                self.handle_update_route(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v2/apis/{apiId}/deployments - CreateDeployment
            ("POST", ["apis", api_id, "deployments"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_create_deployment(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{apiId}/deployments - GetDeployments
            ("GET", ["apis", api_id, "deployments"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_get_deployments(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{apiId}/deployments/{deploymentId} - GetDeployment
            ("GET", ["apis", api_id, "deployments", deployment_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("DeploymentId", deployment_id)];
                self.handle_get_deployment(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/apis/{apiId}/deployments/{deploymentId} - DeleteDeployment
            ("DELETE", ["apis", api_id, "deployments", deployment_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("DeploymentId", deployment_id)];
                self.handle_delete_deployment(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v2/apis/{ApiId}/authorizers - CreateAuthorizer
            ("POST", ["apis", api_id, "authorizers"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_create_authorizer(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{ApiId}/authorizers - GetAuthorizers
            ("GET", ["apis", api_id, "authorizers"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_get_authorizers(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{ApiId}/authorizers/{AuthorizerId} - GetAuthorizer
            ("GET", ["apis", api_id, "authorizers", authorizer_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("AuthorizerId", authorizer_id)];
                self.handle_get_authorizer(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/apis/{ApiId}/authorizers/{AuthorizerId} - DeleteAuthorizer
            ("DELETE", ["apis", api_id, "authorizers", authorizer_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("AuthorizerId", authorizer_id)];
                self.handle_delete_authorizer(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /v2/apis/{ApiId}/authorizers/{AuthorizerId} - UpdateAuthorizer
            ("PATCH", ["apis", api_id, "authorizers", authorizer_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("AuthorizerId", authorizer_id)];
                self.handle_update_authorizer(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v2/apis/{ApiId}/models - CreateModel
            ("POST", ["apis", api_id, "models"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_create_model(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{ApiId}/models - GetModels
            ("GET", ["apis", api_id, "models"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_get_models(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{ApiId}/models/{ModelId} - GetModel
            ("GET", ["apis", api_id, "models", model_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("ModelId", model_id)];
                self.handle_get_model(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/apis/{ApiId}/models/{ModelId} - DeleteModel
            ("DELETE", ["apis", api_id, "models", model_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("ModelId", model_id)];
                self.handle_delete_model(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /v2/apis/{ApiId}/models/{ModelId} - UpdateModel
            ("PATCH", ["apis", api_id, "models", model_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("ModelId", model_id)];
                self.handle_update_model(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses - CreateIntegrationResponse
            (
                "POST",
                [
                    "apis",
                    api_id,
                    "integrations",
                    integration_id,
                    "integrationresponses",
                ],
            ) => {
                let labels: &[(&str, &str)] =
                    &[("ApiId", api_id), ("IntegrationId", integration_id)];
                self.handle_create_integration_response(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses - GetIntegrationResponses
            (
                "GET",
                [
                    "apis",
                    api_id,
                    "integrations",
                    integration_id,
                    "integrationresponses",
                ],
            ) => {
                let labels: &[(&str, &str)] =
                    &[("ApiId", api_id), ("IntegrationId", integration_id)];
                self.handle_get_integration_responses(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses/{IntegrationResponseId} - GetIntegrationResponse
            (
                "GET",
                [
                    "apis",
                    api_id,
                    "integrations",
                    integration_id,
                    "integrationresponses",
                    response_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("ApiId", api_id),
                    ("IntegrationId", integration_id),
                    ("IntegrationResponseId", response_id),
                ];
                self.handle_get_integration_response(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses/{IntegrationResponseId} - DeleteIntegrationResponse
            (
                "DELETE",
                [
                    "apis",
                    api_id,
                    "integrations",
                    integration_id,
                    "integrationresponses",
                    response_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("ApiId", api_id),
                    ("IntegrationId", integration_id),
                    ("IntegrationResponseId", response_id),
                ];
                self.handle_delete_integration_response(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses/{IntegrationResponseId} - UpdateIntegrationResponse
            (
                "PATCH",
                [
                    "apis",
                    api_id,
                    "integrations",
                    integration_id,
                    "integrationresponses",
                    response_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("ApiId", api_id),
                    ("IntegrationId", integration_id),
                    ("IntegrationResponseId", response_id),
                ];
                self.handle_update_integration_response(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v2/apis/{ApiId}/routes/{RouteId}/routeresponses - CreateRouteResponse
            ("POST", ["apis", api_id, "routes", route_id, "routeresponses"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("RouteId", route_id)];
                self.handle_create_route_response(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{ApiId}/routes/{RouteId}/routeresponses - GetRouteResponses
            ("GET", ["apis", api_id, "routes", route_id, "routeresponses"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id), ("RouteId", route_id)];
                self.handle_get_route_responses(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/apis/{ApiId}/routes/{RouteId}/routeresponses/{RouteResponseId} - GetRouteResponse
            (
                "GET",
                [
                    "apis",
                    api_id,
                    "routes",
                    route_id,
                    "routeresponses",
                    response_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("ApiId", api_id),
                    ("RouteId", route_id),
                    ("RouteResponseId", response_id),
                ];
                self.handle_get_route_response(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/apis/{ApiId}/routes/{RouteId}/routeresponses/{RouteResponseId} - DeleteRouteResponse
            (
                "DELETE",
                [
                    "apis",
                    api_id,
                    "routes",
                    route_id,
                    "routeresponses",
                    response_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("ApiId", api_id),
                    ("RouteId", route_id),
                    ("RouteResponseId", response_id),
                ];
                self.handle_delete_route_response(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/apis/{ApiId}/routes/{RouteId}/requestparameters/{RequestParameterKey} - DeleteRouteRequestParameter
            ("DELETE", ["apis", api_id, "routes", route_id, "requestparameters", key]) => {
                let labels: &[(&str, &str)] = &[
                    ("ApiId", api_id),
                    ("RouteId", route_id),
                    ("RequestParameterKey", key),
                ];
                self.handle_delete_route_request_parameter(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/apis/{ApiId}/cors - DeleteCorsConfiguration
            ("DELETE", ["apis", api_id, "cors"]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_delete_cors_configuration(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /v2/apis/{ApiId} - ReimportApi
            ("PUT", ["apis", api_id]) => {
                let labels: &[(&str, &str)] = &[("ApiId", api_id)];
                self.handle_reimport_api(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v2/vpclinks - CreateVpcLink
            ("POST", ["vpclinks"]) => {
                self.handle_create_vpc_link(&state, &request, &[], &query_map)
                    .await
            }
            // GET /v2/vpclinks - GetVpcLinks
            ("GET", ["vpclinks"]) => {
                self.handle_get_vpc_links(&state, &request, &[], &query_map)
                    .await
            }
            // GET /v2/vpclinks/{VpcLinkId} - GetVpcLink
            ("GET", ["vpclinks", vpc_link_id]) => {
                let labels: &[(&str, &str)] = &[("VpcLinkId", vpc_link_id)];
                self.handle_get_vpc_link(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/vpclinks/{VpcLinkId} - DeleteVpcLink
            ("DELETE", ["vpclinks", vpc_link_id]) => {
                let labels: &[(&str, &str)] = &[("VpcLinkId", vpc_link_id)];
                self.handle_delete_vpc_link(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /v2/vpclinks/{VpcLinkId} - UpdateVpcLink
            ("PATCH", ["vpclinks", vpc_link_id]) => {
                let labels: &[(&str, &str)] = &[("VpcLinkId", vpc_link_id)];
                self.handle_update_vpc_link(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/tags/{ResourceArn} - GetTags (ResourceArn may contain slashes, use wildcard)
            ("GET", ["tags", resource_arn @ ..]) => {
                let arn = resource_arn.join("/");
                let labels: &[(&str, &str)] = &[("ResourceArn", arn.as_str())];
                self.handle_get_tags(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v2/tags/{ResourceArn} - TagResource
            ("POST", ["tags", resource_arn @ ..]) => {
                let arn = resource_arn.join("/");
                let labels: &[(&str, &str)] = &[("ResourceArn", arn.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/tags/{ResourceArn} - UntagResource
            ("DELETE", ["tags", resource_arn @ ..]) => {
                let arn = resource_arn.join("/");
                let labels: &[(&str, &str)] = &[("ResourceArn", arn.as_str())];
                self.handle_untag_resource(&state, &request, labels, &query_map, query_string)
                    .await
            }
            // POST /v2/domainnames - CreateDomainName
            ("POST", ["domainnames"]) => {
                self.handle_create_domain_name(&state, &request, &[], &query_map)
                    .await
            }
            // GET /v2/domainnames - GetDomainNames
            ("GET", ["domainnames"]) => {
                self.handle_get_domain_names(&state, &request, &[], &query_map)
                    .await
            }
            // GET /v2/domainnames/{DomainName} - GetDomainName
            ("GET", ["domainnames", domain_name]) => {
                let labels: &[(&str, &str)] = &[("DomainName", domain_name)];
                self.handle_get_domain_name(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/domainnames/{DomainName} - DeleteDomainName
            ("DELETE", ["domainnames", domain_name]) => {
                let labels: &[(&str, &str)] = &[("DomainName", domain_name)];
                self.handle_delete_domain_name(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v2/domainnames/{DomainName}/apimappings - CreateApiMapping
            ("POST", ["domainnames", domain_name, "apimappings"]) => {
                let labels: &[(&str, &str)] = &[("DomainName", domain_name)];
                self.handle_create_api_mapping(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/domainnames/{DomainName}/apimappings - GetApiMappings
            ("GET", ["domainnames", domain_name, "apimappings"]) => {
                let labels: &[(&str, &str)] = &[("DomainName", domain_name)];
                self.handle_get_api_mappings(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v2/domainnames/{DomainName}/apimappings/{ApiMappingId} - GetApiMapping
            ("GET", ["domainnames", domain_name, "apimappings", mapping_id]) => {
                let labels: &[(&str, &str)] =
                    &[("DomainName", domain_name), ("ApiMappingId", mapping_id)];
                self.handle_get_api_mapping(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v2/domainnames/{DomainName}/apimappings/{ApiMappingId} - DeleteApiMapping
            ("DELETE", ["domainnames", domain_name, "apimappings", mapping_id]) => {
                let labels: &[(&str, &str)] =
                    &[("DomainName", domain_name), ("ApiMappingId", mapping_id)];
                self.handle_delete_api_mapping(&state, &request, labels, &query_map)
                    .await
            }

            // --- Unimplemented operations (stubs for non-moto operations) ---
            _ => rest_json_error(404, "NotFoundException", "Not found"),
        };

        if matches!(method, "POST" | "PUT" | "PATCH" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // -----------------------------------------------------------------------
    // API handlers
    // -----------------------------------------------------------------------

    // POST /v2/apis - CreateApi
    async fn handle_create_api(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'name'");
        }
        if input.protocol_type.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'protocolType'");
        }

        let mut s = state.write().await;
        match s.create_api(
            &input.name,
            &input.protocol_type,
            input.route_selection_expression.as_deref(),
            input.description.as_deref(),
            input.tags.unwrap_or_default(),
        ) {
            Ok(api) => wire::serialize_create_api_response(&api_to_create_response(api)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis - GetApis
    async fn handle_get_apis(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_get_apis_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let apis: Vec<ModelApi> = s.list_apis().into_iter().map(api_to_model).collect();
        wire::serialize_get_apis_response(&GetApisResponse {
            items: Some(apis),
            ..Default::default()
        })
    }

    // GET /v2/apis/{apiId} - GetApi
    async fn handle_get_api(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_api(&input.api_id) {
            Ok(api) => wire::serialize_get_api_response(&api_to_get_response(api)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // DELETE /v2/apis/{apiId} - DeleteApi
    async fn handle_delete_api(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_api(&input.api_id) {
            Ok(()) => wire::serialize_delete_api_response(),
            Err(e) => apigw_error_response(&e),
        }
    }

    // PATCH /v2/apis/{apiId} - UpdateApi
    async fn handle_update_api(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.update_api(
            &input.api_id,
            input.name.as_deref(),
            input.description.as_deref(),
            input.route_selection_expression.as_deref(),
        ) {
            Ok(api) => wire::serialize_update_api_response(&api_to_update_response(api)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // -----------------------------------------------------------------------
    // Stage handlers
    // -----------------------------------------------------------------------

    // POST /v2/apis/{apiId}/stages - CreateStage
    async fn handle_create_stage(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_stage_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.stage_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'stageName'");
        }
        let mut s = state.write().await;
        match s.create_stage(
            &input.api_id,
            &input.stage_name,
            input.description.as_deref(),
            input.deployment_id.as_deref(),
            input.auto_deploy.unwrap_or(false),
            input.tags.unwrap_or_default(),
        ) {
            Ok(stage) => wire::serialize_create_stage_response(&stage_to_create_response(stage)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{apiId}/stages - GetStages
    async fn handle_get_stages(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_stages_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_stages(&input.api_id) {
            Ok(stages) => {
                let items: Vec<ModelStage> = stages.into_iter().map(stage_to_model).collect();
                wire::serialize_get_stages_response(&GetStagesResponse {
                    items: Some(items),
                    ..Default::default()
                })
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{apiId}/stages/{stageName} - GetStage
    async fn handle_get_stage(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_stage_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_stage(&input.api_id, &input.stage_name) {
            Ok(stage) => wire::serialize_get_stage_response(&stage_to_get_response(stage)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // DELETE /v2/apis/{apiId}/stages/{stageName} - DeleteStage
    async fn handle_delete_stage(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_stage_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_stage(&input.api_id, &input.stage_name) {
            Ok(()) => wire::serialize_delete_stage_response(),
            Err(e) => apigw_error_response(&e),
        }
    }

    // PATCH /v2/apis/{apiId}/stages/{stageName} - UpdateStage
    async fn handle_update_stage(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_stage_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.update_stage(
            &input.api_id,
            &input.stage_name,
            input.description.as_deref(),
            input.deployment_id.as_deref(),
            input.auto_deploy,
        ) {
            Ok(stage) => wire::serialize_update_stage_response(&stage_to_update_response(stage)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // -----------------------------------------------------------------------
    // Integration handlers
    // -----------------------------------------------------------------------

    // POST /v2/apis/{apiId}/integrations - CreateIntegration
    async fn handle_create_integration(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_integration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.integration_type.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'integrationType'");
        }
        let mut s = state.write().await;
        match s.create_integration(
            &input.api_id,
            &input.integration_type,
            input.integration_uri.as_deref(),
            input.integration_method.as_deref(),
            input.description.as_deref(),
            input.payload_format_version.as_deref(),
            input.connection_type.as_deref(),
        ) {
            Ok(integration) => wire::serialize_create_integration_response(
                &integration_to_create_result(integration),
            ),
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{apiId}/integrations - GetIntegrations
    async fn handle_get_integrations(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_integrations_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_integrations(&input.api_id) {
            Ok(integrations) => {
                let items: Vec<ModelIntegration> =
                    integrations.into_iter().map(integration_to_model).collect();
                wire::serialize_get_integrations_response(&GetIntegrationsResponse {
                    items: Some(items),
                    ..Default::default()
                })
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{apiId}/integrations/{integrationId} - GetIntegration
    async fn handle_get_integration(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_integration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_integration(&input.api_id, &input.integration_id) {
            Ok(integration) => {
                wire::serialize_get_integration_response(&integration_to_get_result(integration))
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // DELETE /v2/apis/{apiId}/integrations/{integrationId} - DeleteIntegration
    async fn handle_delete_integration(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_integration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_integration(&input.api_id, &input.integration_id) {
            Ok(()) => wire::serialize_delete_integration_response(),
            Err(e) => apigw_error_response(&e),
        }
    }

    // PATCH /v2/apis/{apiId}/integrations/{integrationId} - UpdateIntegration
    async fn handle_update_integration(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_integration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.update_integration(
            &input.api_id,
            &input.integration_id,
            input.integration_type.as_deref(),
            input.integration_uri.as_deref(),
            input.integration_method.as_deref(),
            input.description.as_deref(),
            input.payload_format_version.as_deref(),
            input.connection_type.as_deref(),
        ) {
            Ok(integration) => wire::serialize_update_integration_response(
                &integration_to_update_result(integration),
            ),
            Err(e) => apigw_error_response(&e),
        }
    }

    // -----------------------------------------------------------------------
    // Route handlers
    // -----------------------------------------------------------------------

    // POST /v2/apis/{apiId}/routes - CreateRoute
    async fn handle_create_route(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_route_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.route_key.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'routeKey'");
        }
        let mut s = state.write().await;
        match s.create_route(
            &input.api_id,
            &input.route_key,
            input.target.as_deref(),
            input.authorization_type.as_deref(),
            input.authorizer_id.as_deref(),
        ) {
            Ok(route) => wire::serialize_create_route_response(&route_to_create_result(route)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{apiId}/routes - GetRoutes
    async fn handle_get_routes(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_routes_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_routes(&input.api_id) {
            Ok(routes) => {
                let items: Vec<ModelRoute> = routes.into_iter().map(route_to_model).collect();
                wire::serialize_get_routes_response(&GetRoutesResponse {
                    items: Some(items),
                    ..Default::default()
                })
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{apiId}/routes/{routeId} - GetRoute
    async fn handle_get_route(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_route_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_route(&input.api_id, &input.route_id) {
            Ok(route) => wire::serialize_get_route_response(&route_to_get_result(route)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // DELETE /v2/apis/{apiId}/routes/{routeId} - DeleteRoute
    async fn handle_delete_route(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_route_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_route(&input.api_id, &input.route_id) {
            Ok(()) => wire::serialize_delete_route_response(),
            Err(e) => apigw_error_response(&e),
        }
    }

    // PATCH /v2/apis/{apiId}/routes/{routeId} - UpdateRoute
    async fn handle_update_route(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_route_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.update_route(
            &input.api_id,
            &input.route_id,
            input.route_key.as_deref(),
            input.target.as_deref(),
            input.authorization_type.as_deref(),
            input.authorizer_id.as_deref(),
        ) {
            Ok(route) => wire::serialize_update_route_response(&route_to_update_result(route)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // -----------------------------------------------------------------------
    // Deployment handlers
    // -----------------------------------------------------------------------

    // POST /v2/apis/{apiId}/deployments - CreateDeployment
    async fn handle_create_deployment(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_deployment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.create_deployment(&input.api_id, input.description.as_deref()) {
            Ok(deployment) => wire::serialize_create_deployment_response(
                &deployment_to_create_response(deployment),
            ),
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{apiId}/deployments - GetDeployments
    async fn handle_get_deployments(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_deployments_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_deployments(&input.api_id) {
            Ok(deployments) => {
                let items: Vec<ModelDeployment> =
                    deployments.into_iter().map(deployment_to_model).collect();
                wire::serialize_get_deployments_response(&GetDeploymentsResponse {
                    items: Some(items),
                    ..Default::default()
                })
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{apiId}/deployments/{deploymentId} - GetDeployment
    async fn handle_get_deployment(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_deployment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_deployment(&input.api_id, &input.deployment_id) {
            Ok(deployment) => {
                wire::serialize_get_deployment_response(&deployment_to_get_response(deployment))
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // DELETE /v2/apis/{apiId}/deployments/{deploymentId} - DeleteDeployment
    async fn handle_delete_deployment(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_deployment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_deployment(&input.api_id, &input.deployment_id) {
            Ok(()) => wire::serialize_delete_deployment_response(),
            Err(e) => apigw_error_response(&e),
        }
    }

    // -----------------------------------------------------------------------
    // Authorizer handlers
    // -----------------------------------------------------------------------

    // POST /v2/apis/{ApiId}/authorizers - CreateAuthorizer
    async fn handle_create_authorizer(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_authorizer_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'name'");
        }
        if input.authorizer_type.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'authorizerType'");
        }
        let identity_source = if input.identity_source.is_empty() {
            None
        } else {
            Some(input.identity_source)
        };
        let mut s = state.write().await;
        match s.create_authorizer(
            &input.api_id,
            &input.name,
            &input.authorizer_type,
            input.authorizer_uri.as_deref(),
            input.authorizer_credentials_arn.as_deref(),
            input.authorizer_payload_format_version.as_deref(),
            input.authorizer_result_ttl_in_seconds,
            identity_source,
            input.identity_validation_expression.as_deref(),
            input.enable_simple_responses,
        ) {
            Ok(a) => wire::serialize_create_authorizer_response(&authorizer_to_create_response(a)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{ApiId}/authorizers - GetAuthorizers
    async fn handle_get_authorizers(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_authorizers_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_authorizers(&input.api_id) {
            Ok(authorizers) => {
                let items: Vec<ModelAuthorizer> =
                    authorizers.into_iter().map(authorizer_to_model).collect();
                wire::serialize_get_authorizers_response(&GetAuthorizersResponse {
                    items: Some(items),
                    ..Default::default()
                })
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{ApiId}/authorizers/{AuthorizerId} - GetAuthorizer
    async fn handle_get_authorizer(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_authorizer_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_authorizer(&input.api_id, &input.authorizer_id) {
            Ok(a) => wire::serialize_get_authorizer_response(&authorizer_to_get_response(a)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // DELETE /v2/apis/{ApiId}/authorizers/{AuthorizerId} - DeleteAuthorizer
    async fn handle_delete_authorizer(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_authorizer_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_authorizer(&input.api_id, &input.authorizer_id) {
            Ok(()) => wire::serialize_delete_authorizer_response(),
            Err(e) => apigw_error_response(&e),
        }
    }

    // PATCH /v2/apis/{ApiId}/authorizers/{AuthorizerId} - UpdateAuthorizer
    async fn handle_update_authorizer(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_authorizer_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.update_authorizer(
            &input.api_id,
            &input.authorizer_id,
            input.name.as_deref(),
            input.authorizer_type.as_deref(),
            input.authorizer_uri.as_deref(),
            input.authorizer_credentials_arn.as_deref(),
            input.authorizer_payload_format_version.as_deref(),
            input.authorizer_result_ttl_in_seconds,
            input.identity_source,
            input.identity_validation_expression.as_deref(),
            input.enable_simple_responses,
        ) {
            Ok(a) => wire::serialize_update_authorizer_response(&authorizer_to_update_response(a)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // -----------------------------------------------------------------------
    // Model handlers
    // -----------------------------------------------------------------------

    // POST /v2/apis/{ApiId}/models - CreateModel
    async fn handle_create_model(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_model_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'name'");
        }
        let schema = if input.schema.is_empty() {
            None
        } else {
            Some(input.schema)
        };
        let mut s = state.write().await;
        match s.create_model(
            &input.api_id,
            &input.name,
            input.content_type.as_deref(),
            input.description.as_deref(),
            schema.as_deref(),
        ) {
            Ok(m) => wire::serialize_create_model_response(&model_to_create_response(m)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{ApiId}/models - GetModels
    async fn handle_get_models(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_models_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_models(&input.api_id) {
            Ok(models) => {
                let items: Vec<ModelModel> = models.into_iter().map(model_to_model).collect();
                wire::serialize_get_models_response(&GetModelsResponse {
                    items: Some(items),
                    ..Default::default()
                })
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{ApiId}/models/{ModelId} - GetModel
    async fn handle_get_model(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_model_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_model(&input.api_id, &input.model_id) {
            Ok(m) => wire::serialize_get_model_response(&model_to_get_response(m)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // DELETE /v2/apis/{ApiId}/models/{ModelId} - DeleteModel
    async fn handle_delete_model(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_model_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_model(&input.api_id, &input.model_id) {
            Ok(()) => wire::serialize_delete_model_response(),
            Err(e) => apigw_error_response(&e),
        }
    }

    // PATCH /v2/apis/{ApiId}/models/{ModelId} - UpdateModel
    async fn handle_update_model(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_model_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.update_model(
            &input.api_id,
            &input.model_id,
            input.name.as_deref(),
            input.content_type.as_deref(),
            input.description.as_deref(),
            input.schema.as_deref(),
        ) {
            Ok(m) => wire::serialize_update_model_response(&model_to_update_response(m)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // -----------------------------------------------------------------------
    // Integration Response handlers
    // -----------------------------------------------------------------------

    // POST /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses - CreateIntegrationResponse
    async fn handle_create_integration_response(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_integration_response_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.integration_response_key.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing 'integrationResponseKey'",
            );
        }
        let mut s = state.write().await;
        match s.create_integration_response(
            &input.api_id,
            &input.integration_id,
            &input.integration_response_key,
            input.content_handling_strategy.as_deref(),
            input.response_parameters,
            input.response_templates,
            input.template_selection_expression.as_deref(),
        ) {
            Ok(r) => wire::serialize_create_integration_response_response(
                &int_response_to_create_response(r),
            ),
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses - GetIntegrationResponses
    async fn handle_get_integration_responses(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_integration_responses_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let s = state.read().await;
        match s.list_integration_responses(&input.api_id, &input.integration_id) {
            Ok(responses) => {
                let items: Vec<ModelIntegrationResponse> =
                    responses.into_iter().map(int_response_to_model).collect();
                wire::serialize_get_integration_responses_response(
                    &GetIntegrationResponsesResponse {
                        items: Some(items),
                        ..Default::default()
                    },
                )
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses/{IntegrationResponseId} - GetIntegrationResponse
    async fn handle_get_integration_response(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_integration_response_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_integration_response(
            &input.api_id,
            &input.integration_id,
            &input.integration_response_id,
        ) {
            Ok(r) => {
                wire::serialize_get_integration_response_response(&int_response_to_get_response(r))
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // DELETE /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses/{IntegrationResponseId} - DeleteIntegrationResponse
    async fn handle_delete_integration_response(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_integration_response_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut s = state.write().await;
        match s.delete_integration_response(
            &input.api_id,
            &input.integration_id,
            &input.integration_response_id,
        ) {
            Ok(()) => wire::serialize_delete_integration_response_response(),
            Err(e) => apigw_error_response(&e),
        }
    }

    // PATCH /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses/{IntegrationResponseId} - UpdateIntegrationResponse
    async fn handle_update_integration_response(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_integration_response_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut s = state.write().await;
        match s.update_integration_response(
            &input.api_id,
            &input.integration_id,
            &input.integration_response_id,
            input.integration_response_key.as_deref(),
            input.content_handling_strategy.as_deref(),
            input.response_parameters,
            input.response_templates,
            input.template_selection_expression.as_deref(),
        ) {
            Ok(r) => wire::serialize_update_integration_response_response(
                &int_response_to_update_response(r),
            ),
            Err(e) => apigw_error_response(&e),
        }
    }

    // -----------------------------------------------------------------------
    // Route Response handlers
    // -----------------------------------------------------------------------

    // POST /v2/apis/{ApiId}/routes/{RouteId}/routeresponses - CreateRouteResponse
    async fn handle_create_route_response(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_route_response_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.route_response_key.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'routeResponseKey'");
        }
        let mut s = state.write().await;
        match s.create_route_response(
            &input.api_id,
            &input.route_id,
            &input.route_response_key,
            input.model_selection_expression.as_deref(),
        ) {
            Ok(r) => wire::serialize_create_route_response_response(
                &route_response_to_create_response(r),
            ),
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{ApiId}/routes/{RouteId}/routeresponses - GetRouteResponses
    async fn handle_get_route_responses(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_route_responses_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_route_responses(&input.api_id, &input.route_id) {
            Ok(responses) => {
                let items: Vec<ModelRouteResponse> =
                    responses.into_iter().map(route_response_to_model).collect();
                wire::serialize_get_route_responses_response(&GetRouteResponsesResponse {
                    items: Some(items),
                    ..Default::default()
                })
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/apis/{ApiId}/routes/{RouteId}/routeresponses/{RouteResponseId} - GetRouteResponse
    async fn handle_get_route_response(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_route_response_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_route_response(&input.api_id, &input.route_id, &input.route_response_id) {
            Ok(r) => {
                wire::serialize_get_route_response_response(&route_response_to_get_response(r))
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // DELETE /v2/apis/{ApiId}/routes/{RouteId}/routeresponses/{RouteResponseId} - DeleteRouteResponse
    async fn handle_delete_route_response(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_route_response_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_route_response(&input.api_id, &input.route_id, &input.route_response_id) {
            Ok(()) => wire::serialize_delete_route_response_response(),
            Err(e) => apigw_error_response(&e),
        }
    }

    // DELETE /v2/apis/{ApiId}/routes/{RouteId}/requestparameters/{RequestParameterKey} - DeleteRouteRequestParameter
    async fn handle_delete_route_request_parameter(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_route_request_parameter_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_route(&input.api_id, &input.route_id) {
            Ok(_) => wire::serialize_delete_route_request_parameter_response(),
            Err(e) => apigw_error_response(&e),
        }
    }

    // DELETE /v2/apis/{ApiId}/cors - DeleteCorsConfiguration
    async fn handle_delete_cors_configuration(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_cors_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut s = state.write().await;
        match s.delete_cors_configuration(&input.api_id) {
            Ok(()) => wire::serialize_delete_cors_configuration_response(),
            Err(e) => apigw_error_response(&e),
        }
    }

    // PUT /v2/apis/{ApiId} - ReimportApi (stub: delegates to update)
    async fn handle_reimport_api(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reimport_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // The Smithy model for ReimportApi only carries `apiId`, `body`, `Basepath`,
        // and `failOnWarnings`; the historical handler accepts a flat body with
        // `name`/`description`/`routeSelectionExpression`. Re-parse the body for
        // those fields, mirroring the apigateway pattern.
        let (name, description, rse) = serde_json::from_slice::<serde_json::Value>(&request.body)
            .ok()
            .map(|v| {
                (
                    v.get("name").and_then(|x| x.as_str()).map(str::to_string),
                    v.get("description")
                        .and_then(|x| x.as_str())
                        .map(str::to_string),
                    v.get("routeSelectionExpression")
                        .and_then(|x| x.as_str())
                        .map(str::to_string),
                )
            })
            .unwrap_or((None, None, None));
        let mut s = state.write().await;
        match s.update_api(
            &input.api_id,
            name.as_deref(),
            description.as_deref(),
            rse.as_deref(),
        ) {
            Ok(api) => {
                let resp = ReimportApiResponse {
                    api_id: Some(api.api_id.clone()),
                    name: Some(api.name.clone()),
                    protocol_type: Some(api.protocol_type.clone()),
                    description: api.description.clone(),
                    api_endpoint: Some(api.api_endpoint.clone()),
                    created_date: Some(api.created_date.clone()),
                    ..Default::default()
                };
                wire::serialize_reimport_api_response(&resp)
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // -----------------------------------------------------------------------
    // VPC Link handlers
    // -----------------------------------------------------------------------

    // POST /v2/vpclinks - CreateVpcLink
    async fn handle_create_vpc_link(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_vpc_link_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'name'");
        }
        let security_group_ids = input.security_group_ids.unwrap_or_default();
        let mut s = state.write().await;
        let vpc_link = s.create_vpc_link(
            &input.name,
            security_group_ids,
            input.subnet_ids,
            input.tags.unwrap_or_default(),
        );
        wire::serialize_create_vpc_link_response(&vpc_link_to_create_response(vpc_link))
    }

    // GET /v2/vpclinks - GetVpcLinks
    async fn handle_get_vpc_links(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_get_vpc_links_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let items: Vec<ModelVpcLink> = s
            .list_vpc_links()
            .into_iter()
            .map(vpc_link_to_model)
            .collect();
        wire::serialize_get_vpc_links_response(&GetVpcLinksResponse {
            items: Some(items),
            ..Default::default()
        })
    }

    // GET /v2/vpclinks/{VpcLinkId} - GetVpcLink
    async fn handle_get_vpc_link(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_vpc_link_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_vpc_link(&input.vpc_link_id) {
            Ok(v) => wire::serialize_get_vpc_link_response(&vpc_link_to_get_response(v)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // DELETE /v2/vpclinks/{VpcLinkId} - DeleteVpcLink
    async fn handle_delete_vpc_link(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_vpc_link_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_vpc_link(&input.vpc_link_id) {
            Ok(()) => wire::serialize_delete_vpc_link_response(&Default::default()),
            Err(e) => apigw_error_response(&e),
        }
    }

    // PATCH /v2/vpclinks/{VpcLinkId} - UpdateVpcLink
    async fn handle_update_vpc_link(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_vpc_link_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.update_vpc_link(&input.vpc_link_id, input.name.as_deref()) {
            Ok(v) => wire::serialize_update_vpc_link_response(&vpc_link_to_update_response(v)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // -----------------------------------------------------------------------
    // Tags handlers
    // -----------------------------------------------------------------------

    // GET /v2/tags/{ResourceArn} - GetTags
    async fn handle_get_tags(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_tags_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let tags = s.get_tags(&input.resource_arn);
        wire::serialize_get_tags_response(&GetTagsResponse {
            tags: if tags.is_empty() { None } else { Some(tags) },
        })
    }

    // POST /v2/tags/{ResourceArn} - TagResource
    async fn handle_tag_resource(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let tags = input.tags.unwrap_or_default();
        let mut s = state.write().await;
        s.tag_resource(&input.resource_arn, tags);
        wire::serialize_tag_resource_response(&TagResourceResponse {})
    }

    // DELETE /v2/tags/{ResourceArn} - UntagResource
    async fn handle_untag_resource(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        raw_query_string: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // The AWS SDK encodes `tagKeys` as repeated query params; preserve the
        // historical behaviour by re-extracting from the raw query string.
        let tag_keys: Vec<String> = raw_query_string
            .split('&')
            .filter_map(|p| {
                let (k, v) = p.split_once('=')?;
                if k == "tagKeys" {
                    Some(v.to_string())
                } else {
                    None
                }
            })
            .collect();
        let tag_keys = if tag_keys.is_empty() {
            input.tag_keys
        } else {
            tag_keys
        };
        let keys: Vec<&str> = tag_keys.iter().map(|s| s.as_str()).collect();
        let mut s = state.write().await;
        s.untag_resource(&input.resource_arn, &keys);
        wire::serialize_untag_resource_response()
    }

    // -----------------------------------------------------------------------
    // Domain Name handlers
    // -----------------------------------------------------------------------

    // POST /v2/domainnames - CreateDomainName
    async fn handle_create_domain_name(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_domain_name_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.domain_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'domainName'");
        }
        let configs: Vec<crate::types::StoredDomainNameConfiguration> = input
            .domain_name_configurations
            .unwrap_or_default()
            .into_iter()
            .map(|c| crate::types::StoredDomainNameConfiguration {
                certificate_arn: c.certificate_arn,
                endpoint_type: c.endpoint_type,
                security_policy: c.security_policy,
            })
            .collect();
        let mut s = state.write().await;
        let dn = s.create_domain_name(&input.domain_name, input.tags.unwrap_or_default(), configs);
        wire::serialize_create_domain_name_response(&domain_name_to_create_response(dn))
    }

    // GET /v2/domainnames - GetDomainNames
    async fn handle_get_domain_names(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_get_domain_names_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let items: Vec<ModelDomainName> = s
            .list_domain_names()
            .into_iter()
            .map(domain_name_to_model)
            .collect();
        wire::serialize_get_domain_names_response(&GetDomainNamesResponse {
            items: Some(items),
            ..Default::default()
        })
    }

    // GET /v2/domainnames/{DomainName} - GetDomainName
    async fn handle_get_domain_name(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_domain_name_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_domain_name(&input.domain_name) {
            Ok(dn) => wire::serialize_get_domain_name_response(&domain_name_to_get_response(dn)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // DELETE /v2/domainnames/{DomainName} - DeleteDomainName
    async fn handle_delete_domain_name(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_domain_name_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_domain_name(&input.domain_name) {
            Ok(()) => wire::serialize_delete_domain_name_response(),
            Err(e) => apigw_error_response(&e),
        }
    }

    // -----------------------------------------------------------------------
    // API Mapping handlers
    // -----------------------------------------------------------------------

    // POST /v2/domainnames/{DomainName}/apimappings - CreateApiMapping
    async fn handle_create_api_mapping(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_api_mapping_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.api_id.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'apiId'");
        }
        if input.stage.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'stage'");
        }
        let mut s = state.write().await;
        match s.create_api_mapping(
            &input.domain_name,
            &input.api_id,
            &input.stage,
            input.api_mapping_key.as_deref(),
        ) {
            Ok(m) => {
                wire::serialize_create_api_mapping_response(&api_mapping_to_create_response(m))
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/domainnames/{DomainName}/apimappings - GetApiMappings
    async fn handle_get_api_mappings(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_api_mappings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_api_mappings(&input.domain_name) {
            Ok(mappings) => {
                let items: Vec<ModelApiMapping> =
                    mappings.into_iter().map(api_mapping_to_model).collect();
                wire::serialize_get_api_mappings_response(&GetApiMappingsResponse {
                    items: Some(items),
                    ..Default::default()
                })
            }
            Err(e) => apigw_error_response(&e),
        }
    }

    // GET /v2/domainnames/{DomainName}/apimappings/{ApiMappingId} - GetApiMapping
    async fn handle_get_api_mapping(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_api_mapping_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_api_mapping(&input.domain_name, &input.api_mapping_id) {
            Ok(m) => wire::serialize_get_api_mapping_response(&api_mapping_to_get_response(m)),
            Err(e) => apigw_error_response(&e),
        }
    }

    // DELETE /v2/domainnames/{DomainName}/apimappings/{ApiMappingId} - DeleteApiMapping
    async fn handle_delete_api_mapping(
        &self,
        state: &Arc<sync::RwLock<ApiGatewayV2State>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_api_mapping_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_api_mapping(&input.domain_name, &input.api_mapping_id) {
            Ok(()) => wire::serialize_delete_api_mapping_response(),
            Err(e) => apigw_error_response(&e),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// URL-percent-decode a single path segment (e.g. `%24default` → `$default`).
fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let h1 = chars.next();
            let h2 = chars.next();
            if let (Some(h1), Some(h2)) = (h1, h2) {
                if let Ok(byte) = u8::from_str_radix(&format!("{h1}{h2}"), 16) {
                    result.push(byte as char);
                    continue;
                }
                // Not a valid hex sequence — push the raw characters back
                result.push('%');
                result.push(h1);
                result.push(h2);
            } else {
                result.push('%');
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn extract_path(uri: &str) -> &str {
    let without_scheme = if let Some(pos) = uri.find("://") {
        &uri[pos + 3..]
    } else {
        uri
    };
    if let Some(pos) = without_scheme.find('/') {
        let path = &without_scheme[pos..];
        // Strip query string
        if let Some(q) = path.find('?') {
            &path[..q]
        } else {
            path
        }
    } else {
        "/"
    }
}

fn rest_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = serde_json::json!({
        "message": message,
        "__type": error_type
    })
    .to_string();
    let mut resp = MockResponse::rest_json(status, body);
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn apigw_error_response(e: &ApiGatewayV2Error) -> MockResponse {
    let (status, error_type) = match e {
        ApiGatewayV2Error::NameRequired => (400, "BadRequestException"),
        ApiGatewayV2Error::ProtocolTypeRequired => (400, "BadRequestException"),
        ApiGatewayV2Error::ApiNotFound(_) => (404, "NotFoundException"),
        ApiGatewayV2Error::StageNotFound(_) => (404, "NotFoundException"),
        ApiGatewayV2Error::IntegrationNotFound(_) => (404, "NotFoundException"),
        ApiGatewayV2Error::RouteNotFound(_) => (404, "NotFoundException"),
        ApiGatewayV2Error::DeploymentNotFound(_) => (404, "NotFoundException"),
        ApiGatewayV2Error::AuthorizerNotFound(_) => (404, "NotFoundException"),
        ApiGatewayV2Error::ModelNotFound(_) => (404, "NotFoundException"),
        ApiGatewayV2Error::VpcLinkNotFound(_) => (404, "NotFoundException"),
        ApiGatewayV2Error::DomainNameNotFound(_) => (404, "NotFoundException"),
        ApiGatewayV2Error::ApiMappingNotFound(_) => (404, "NotFoundException"),
        ApiGatewayV2Error::IntegrationResponseNotFound(_) => (404, "NotFoundException"),
        ApiGatewayV2Error::RouteResponseNotFound(_) => (404, "NotFoundException"),
    };
    rest_json_error(status, error_type, &e.to_string())
}

// ---------------------------------------------------------------------------
// Type conversion helpers
// ---------------------------------------------------------------------------

fn api_to_create_response(api: &crate::types::Api) -> CreateApiResponse {
    CreateApiResponse {
        api_id: Some(api.api_id.clone()),
        name: Some(api.name.clone()),
        protocol_type: Some(api.protocol_type.clone()),
        route_selection_expression: api.route_selection_expression.clone(),
        description: api.description.clone(),
        api_endpoint: Some(api.api_endpoint.clone()),
        created_date: Some(api.created_date.clone()),
        tags: if api.tags.is_empty() {
            None
        } else {
            Some(api.tags.clone())
        },
        ..Default::default()
    }
}

fn api_to_get_response(api: &crate::types::Api) -> GetApiResponse {
    GetApiResponse {
        api_id: Some(api.api_id.clone()),
        name: Some(api.name.clone()),
        protocol_type: Some(api.protocol_type.clone()),
        route_selection_expression: api.route_selection_expression.clone(),
        description: api.description.clone(),
        api_endpoint: Some(api.api_endpoint.clone()),
        created_date: Some(api.created_date.clone()),
        tags: if api.tags.is_empty() {
            None
        } else {
            Some(api.tags.clone())
        },
        ..Default::default()
    }
}

fn api_to_update_response(api: &crate::types::Api) -> UpdateApiResponse {
    UpdateApiResponse {
        api_id: Some(api.api_id.clone()),
        name: Some(api.name.clone()),
        protocol_type: Some(api.protocol_type.clone()),
        route_selection_expression: api.route_selection_expression.clone(),
        description: api.description.clone(),
        api_endpoint: Some(api.api_endpoint.clone()),
        created_date: Some(api.created_date.clone()),
        tags: if api.tags.is_empty() {
            None
        } else {
            Some(api.tags.clone())
        },
        ..Default::default()
    }
}

fn api_to_model(api: &crate::types::Api) -> ModelApi {
    ModelApi {
        api_id: Some(api.api_id.clone()),
        name: Some(api.name.clone()),
        protocol_type: Some(api.protocol_type.clone()),
        route_selection_expression: api.route_selection_expression.clone(),
        description: api.description.clone(),
        api_endpoint: Some(api.api_endpoint.clone()),
        created_date: Some(api.created_date.clone()),
        tags: if api.tags.is_empty() {
            None
        } else {
            Some(api.tags.clone())
        },
        ..Default::default()
    }
}

fn stage_to_create_response(stage: &crate::types::Stage) -> CreateStageResponse {
    CreateStageResponse {
        stage_name: Some(stage.stage_name.clone()),
        description: stage.description.clone(),
        deployment_id: stage.deployment_id.clone(),
        auto_deploy: Some(stage.auto_deploy),
        created_date: Some(stage.created_date.clone()),
        last_updated_date: Some(stage.last_updated_date.clone()),
        tags: if stage.tags.is_empty() {
            None
        } else {
            Some(stage.tags.clone())
        },
        ..Default::default()
    }
}

fn stage_to_get_response(stage: &crate::types::Stage) -> GetStageResponse {
    GetStageResponse {
        stage_name: Some(stage.stage_name.clone()),
        description: stage.description.clone(),
        deployment_id: stage.deployment_id.clone(),
        auto_deploy: Some(stage.auto_deploy),
        created_date: Some(stage.created_date.clone()),
        last_updated_date: Some(stage.last_updated_date.clone()),
        tags: if stage.tags.is_empty() {
            None
        } else {
            Some(stage.tags.clone())
        },
        ..Default::default()
    }
}

fn stage_to_update_response(stage: &crate::types::Stage) -> UpdateStageResponse {
    UpdateStageResponse {
        stage_name: Some(stage.stage_name.clone()),
        description: stage.description.clone(),
        deployment_id: stage.deployment_id.clone(),
        auto_deploy: Some(stage.auto_deploy),
        created_date: Some(stage.created_date.clone()),
        last_updated_date: Some(stage.last_updated_date.clone()),
        tags: if stage.tags.is_empty() {
            None
        } else {
            Some(stage.tags.clone())
        },
        ..Default::default()
    }
}

fn stage_to_model(stage: &crate::types::Stage) -> ModelStage {
    ModelStage {
        stage_name: Some(stage.stage_name.clone()),
        description: stage.description.clone(),
        deployment_id: stage.deployment_id.clone(),
        auto_deploy: Some(stage.auto_deploy),
        created_date: Some(stage.created_date.clone()),
        last_updated_date: Some(stage.last_updated_date.clone()),
        tags: if stage.tags.is_empty() {
            None
        } else {
            Some(stage.tags.clone())
        },
        ..Default::default()
    }
}

fn integration_to_create_result(i: &crate::types::Integration) -> CreateIntegrationResult {
    CreateIntegrationResult {
        integration_id: Some(i.integration_id.clone()),
        integration_type: Some(i.integration_type.clone()),
        integration_uri: i.integration_uri.clone(),
        integration_method: i.integration_method.clone(),
        description: i.description.clone(),
        payload_format_version: i.payload_format_version.clone(),
        connection_type: i.connection_type.clone(),
        ..Default::default()
    }
}

fn integration_to_get_result(i: &crate::types::Integration) -> GetIntegrationResult {
    GetIntegrationResult {
        integration_id: Some(i.integration_id.clone()),
        integration_type: Some(i.integration_type.clone()),
        integration_uri: i.integration_uri.clone(),
        integration_method: i.integration_method.clone(),
        description: i.description.clone(),
        payload_format_version: i.payload_format_version.clone(),
        connection_type: i.connection_type.clone(),
        ..Default::default()
    }
}

fn integration_to_update_result(i: &crate::types::Integration) -> UpdateIntegrationResult {
    UpdateIntegrationResult {
        integration_id: Some(i.integration_id.clone()),
        integration_type: Some(i.integration_type.clone()),
        integration_uri: i.integration_uri.clone(),
        integration_method: i.integration_method.clone(),
        description: i.description.clone(),
        payload_format_version: i.payload_format_version.clone(),
        connection_type: i.connection_type.clone(),
        ..Default::default()
    }
}

fn integration_to_model(i: &crate::types::Integration) -> ModelIntegration {
    ModelIntegration {
        integration_id: Some(i.integration_id.clone()),
        integration_type: Some(i.integration_type.clone()),
        integration_uri: i.integration_uri.clone(),
        integration_method: i.integration_method.clone(),
        description: i.description.clone(),
        payload_format_version: i.payload_format_version.clone(),
        connection_type: i.connection_type.clone(),
        ..Default::default()
    }
}

fn route_to_create_result(r: &crate::types::Route) -> CreateRouteResult {
    CreateRouteResult {
        route_id: Some(r.route_id.clone()),
        route_key: Some(r.route_key.clone()),
        target: r.target.clone(),
        authorization_type: r.authorization_type.clone(),
        authorizer_id: r.authorizer_id.clone(),
        ..Default::default()
    }
}

fn route_to_get_result(r: &crate::types::Route) -> GetRouteResult {
    GetRouteResult {
        route_id: Some(r.route_id.clone()),
        route_key: Some(r.route_key.clone()),
        target: r.target.clone(),
        authorization_type: r.authorization_type.clone(),
        authorizer_id: r.authorizer_id.clone(),
        ..Default::default()
    }
}

fn route_to_update_result(r: &crate::types::Route) -> UpdateRouteResult {
    UpdateRouteResult {
        route_id: Some(r.route_id.clone()),
        route_key: Some(r.route_key.clone()),
        target: r.target.clone(),
        authorization_type: r.authorization_type.clone(),
        authorizer_id: r.authorizer_id.clone(),
        ..Default::default()
    }
}

fn route_to_model(r: &crate::types::Route) -> ModelRoute {
    ModelRoute {
        route_id: Some(r.route_id.clone()),
        route_key: Some(r.route_key.clone()),
        target: r.target.clone(),
        authorization_type: r.authorization_type.clone(),
        authorizer_id: r.authorizer_id.clone(),
        ..Default::default()
    }
}

fn deployment_to_create_response(d: &crate::types::Deployment) -> CreateDeploymentResponse {
    CreateDeploymentResponse {
        deployment_id: Some(d.deployment_id.clone()),
        deployment_status: Some(d.deployment_status.clone()),
        description: d.description.clone(),
        created_date: Some(d.created_date.clone()),
        ..Default::default()
    }
}

fn deployment_to_get_response(d: &crate::types::Deployment) -> GetDeploymentResponse {
    GetDeploymentResponse {
        deployment_id: Some(d.deployment_id.clone()),
        deployment_status: Some(d.deployment_status.clone()),
        description: d.description.clone(),
        created_date: Some(d.created_date.clone()),
        ..Default::default()
    }
}

fn deployment_to_model(d: &crate::types::Deployment) -> ModelDeployment {
    ModelDeployment {
        deployment_id: Some(d.deployment_id.clone()),
        deployment_status: Some(d.deployment_status.clone()),
        description: d.description.clone(),
        created_date: Some(d.created_date.clone()),
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// Authorizer conversion helpers
// ---------------------------------------------------------------------------

fn authorizer_to_create_response(a: &crate::types::Authorizer) -> CreateAuthorizerResponse {
    CreateAuthorizerResponse {
        authorizer_id: Some(a.authorizer_id.clone()),
        name: Some(a.name.clone()),
        authorizer_type: Some(a.authorizer_type.clone()),
        authorizer_uri: a.authorizer_uri.clone(),
        authorizer_credentials_arn: a.authorizer_credentials_arn.clone(),
        authorizer_payload_format_version: a.authorizer_payload_format_version.clone(),
        authorizer_result_ttl_in_seconds: a.authorizer_result_ttl_in_seconds,
        identity_source: a.identity_source.clone(),
        identity_validation_expression: a.identity_validation_expression.clone(),
        enable_simple_responses: a.enable_simple_responses,
        ..Default::default()
    }
}

fn authorizer_to_get_response(a: &crate::types::Authorizer) -> GetAuthorizerResponse {
    GetAuthorizerResponse {
        authorizer_id: Some(a.authorizer_id.clone()),
        name: Some(a.name.clone()),
        authorizer_type: Some(a.authorizer_type.clone()),
        authorizer_uri: a.authorizer_uri.clone(),
        authorizer_credentials_arn: a.authorizer_credentials_arn.clone(),
        authorizer_payload_format_version: a.authorizer_payload_format_version.clone(),
        authorizer_result_ttl_in_seconds: a.authorizer_result_ttl_in_seconds,
        identity_source: a.identity_source.clone(),
        identity_validation_expression: a.identity_validation_expression.clone(),
        enable_simple_responses: a.enable_simple_responses,
        ..Default::default()
    }
}

fn authorizer_to_model(a: &crate::types::Authorizer) -> ModelAuthorizer {
    ModelAuthorizer {
        authorizer_id: Some(a.authorizer_id.clone()),
        name: Some(a.name.clone()),
        authorizer_type: Some(a.authorizer_type.clone()),
        authorizer_uri: a.authorizer_uri.clone(),
        authorizer_credentials_arn: a.authorizer_credentials_arn.clone(),
        authorizer_payload_format_version: a.authorizer_payload_format_version.clone(),
        authorizer_result_ttl_in_seconds: a.authorizer_result_ttl_in_seconds,
        identity_source: a.identity_source.clone(),
        identity_validation_expression: a.identity_validation_expression.clone(),
        enable_simple_responses: a.enable_simple_responses,
        ..Default::default()
    }
}

fn authorizer_to_update_response(a: &crate::types::Authorizer) -> UpdateAuthorizerResponse {
    UpdateAuthorizerResponse {
        authorizer_id: Some(a.authorizer_id.clone()),
        name: Some(a.name.clone()),
        authorizer_type: Some(a.authorizer_type.clone()),
        authorizer_uri: a.authorizer_uri.clone(),
        authorizer_credentials_arn: a.authorizer_credentials_arn.clone(),
        authorizer_payload_format_version: a.authorizer_payload_format_version.clone(),
        authorizer_result_ttl_in_seconds: a.authorizer_result_ttl_in_seconds,
        identity_source: a.identity_source.clone(),
        identity_validation_expression: a.identity_validation_expression.clone(),
        enable_simple_responses: a.enable_simple_responses,
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// Model conversion helpers
// ---------------------------------------------------------------------------

fn model_to_create_response(m: &crate::types::Model) -> CreateModelResponse {
    CreateModelResponse {
        model_id: Some(m.model_id.clone()),
        name: Some(m.name.clone()),
        content_type: m.content_type.clone(),
        description: m.description.clone(),
        schema: m.schema.clone(),
    }
}

fn model_to_get_response(m: &crate::types::Model) -> GetModelResponse {
    GetModelResponse {
        model_id: Some(m.model_id.clone()),
        name: Some(m.name.clone()),
        content_type: m.content_type.clone(),
        description: m.description.clone(),
        schema: m.schema.clone(),
    }
}

fn model_to_model(m: &crate::types::Model) -> ModelModel {
    ModelModel {
        model_id: Some(m.model_id.clone()),
        name: Some(m.name.clone()),
        content_type: m.content_type.clone(),
        description: m.description.clone(),
        ..Default::default()
    }
}

fn model_to_update_response(m: &crate::types::Model) -> UpdateModelResponse {
    UpdateModelResponse {
        model_id: Some(m.model_id.clone()),
        name: Some(m.name.clone()),
        content_type: m.content_type.clone(),
        description: m.description.clone(),
        schema: m.schema.clone(),
    }
}

// ---------------------------------------------------------------------------
// VPC Link conversion helpers
// ---------------------------------------------------------------------------

fn vpc_link_to_create_response(v: &crate::types::VpcLink) -> CreateVpcLinkResponse {
    CreateVpcLinkResponse {
        vpc_link_id: Some(v.vpc_link_id.clone()),
        name: Some(v.name.clone()),
        security_group_ids: Some(v.security_group_ids.clone()),
        subnet_ids: Some(v.subnet_ids.clone()),
        tags: if v.tags.is_empty() {
            None
        } else {
            Some(v.tags.clone())
        },
        created_date: Some(v.created_date.clone()),
        vpc_link_status: Some("AVAILABLE".to_string()),
        ..Default::default()
    }
}

fn vpc_link_to_get_response(v: &crate::types::VpcLink) -> GetVpcLinkResponse {
    GetVpcLinkResponse {
        vpc_link_id: Some(v.vpc_link_id.clone()),
        name: Some(v.name.clone()),
        security_group_ids: Some(v.security_group_ids.clone()),
        subnet_ids: Some(v.subnet_ids.clone()),
        tags: if v.tags.is_empty() {
            None
        } else {
            Some(v.tags.clone())
        },
        created_date: Some(v.created_date.clone()),
        vpc_link_status: Some("AVAILABLE".to_string()),
        ..Default::default()
    }
}

fn vpc_link_to_model(v: &crate::types::VpcLink) -> ModelVpcLink {
    ModelVpcLink {
        vpc_link_id: Some(v.vpc_link_id.clone()),
        name: Some(v.name.clone()),
        security_group_ids: Some(v.security_group_ids.clone()),
        subnet_ids: Some(v.subnet_ids.clone()),
        tags: if v.tags.is_empty() {
            None
        } else {
            Some(v.tags.clone())
        },
        created_date: Some(v.created_date.clone()),
        vpc_link_status: Some("AVAILABLE".to_string()),
        ..Default::default()
    }
}

fn vpc_link_to_update_response(v: &crate::types::VpcLink) -> UpdateVpcLinkResponse {
    UpdateVpcLinkResponse {
        vpc_link_id: Some(v.vpc_link_id.clone()),
        name: Some(v.name.clone()),
        security_group_ids: Some(v.security_group_ids.clone()),
        subnet_ids: Some(v.subnet_ids.clone()),
        tags: if v.tags.is_empty() {
            None
        } else {
            Some(v.tags.clone())
        },
        created_date: Some(v.created_date.clone()),
        vpc_link_status: Some("AVAILABLE".to_string()),
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// Domain Name conversion helpers
// ---------------------------------------------------------------------------

fn stored_configs_to_wire(
    configs: &[crate::types::StoredDomainNameConfiguration],
) -> Option<Vec<crate::model::DomainNameConfiguration>> {
    if configs.is_empty() {
        // Always return at least one config with AVAILABLE status so the provider
        // waiter does not poll indefinitely.
        Some(vec![crate::model::DomainNameConfiguration {
            domain_name_status: Some("AVAILABLE".to_string()),
            ..Default::default()
        }])
    } else {
        Some(
            configs
                .iter()
                .map(|c| crate::model::DomainNameConfiguration {
                    certificate_arn: c.certificate_arn.clone(),
                    endpoint_type: c.endpoint_type.clone(),
                    domain_name_status: Some("AVAILABLE".to_string()),
                    ..Default::default()
                })
                .collect(),
        )
    }
}

fn domain_name_to_create_response(d: &crate::types::DomainName) -> CreateDomainNameResponse {
    CreateDomainNameResponse {
        domain_name: Some(d.domain_name.clone()),
        tags: if d.tags.is_empty() {
            None
        } else {
            Some(d.tags.clone())
        },
        domain_name_configurations: stored_configs_to_wire(&d.domain_name_configurations),
        ..Default::default()
    }
}

fn domain_name_to_get_response(d: &crate::types::DomainName) -> GetDomainNameResponse {
    GetDomainNameResponse {
        domain_name: Some(d.domain_name.clone()),
        tags: if d.tags.is_empty() {
            None
        } else {
            Some(d.tags.clone())
        },
        domain_name_configurations: stored_configs_to_wire(&d.domain_name_configurations),
        ..Default::default()
    }
}

fn domain_name_to_model(d: &crate::types::DomainName) -> ModelDomainName {
    ModelDomainName {
        domain_name: Some(d.domain_name.clone()),
        tags: if d.tags.is_empty() {
            None
        } else {
            Some(d.tags.clone())
        },
        domain_name_configurations: stored_configs_to_wire(&d.domain_name_configurations),
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// API Mapping conversion helpers
// ---------------------------------------------------------------------------

fn api_mapping_to_create_response(m: &crate::types::ApiMapping) -> CreateApiMappingResponse {
    CreateApiMappingResponse {
        api_id: Some(m.api_id.clone()),
        api_mapping_id: Some(m.api_mapping_id.clone()),
        api_mapping_key: m.api_mapping_key.clone(),
        stage: Some(m.stage.clone()),
    }
}

fn api_mapping_to_get_response(m: &crate::types::ApiMapping) -> GetApiMappingResponse {
    GetApiMappingResponse {
        api_id: Some(m.api_id.clone()),
        api_mapping_id: Some(m.api_mapping_id.clone()),
        api_mapping_key: m.api_mapping_key.clone(),
        stage: Some(m.stage.clone()),
    }
}

fn api_mapping_to_model(m: &crate::types::ApiMapping) -> ModelApiMapping {
    ModelApiMapping {
        api_id: Some(m.api_id.clone()),
        api_mapping_id: Some(m.api_mapping_id.clone()),
        api_mapping_key: m.api_mapping_key.clone(),
        stage: Some(m.stage.clone()),
    }
}

// ---------------------------------------------------------------------------
// Integration Response conversion helpers
// ---------------------------------------------------------------------------

fn int_response_to_create_response(
    r: &crate::types::IntegrationResponse,
) -> CreateIntegrationResponseResponse {
    CreateIntegrationResponseResponse {
        integration_response_id: Some(r.integration_response_id.clone()),
        integration_response_key: Some(r.integration_response_key.clone()),
        content_handling_strategy: r.content_handling_strategy.clone(),
        response_parameters: r.response_parameters.clone(),
        response_templates: r.response_templates.clone(),
        template_selection_expression: r.template_selection_expression.clone(),
    }
}

fn int_response_to_get_response(
    r: &crate::types::IntegrationResponse,
) -> GetIntegrationResponseResponse {
    GetIntegrationResponseResponse {
        integration_response_id: Some(r.integration_response_id.clone()),
        integration_response_key: Some(r.integration_response_key.clone()),
        content_handling_strategy: r.content_handling_strategy.clone(),
        response_parameters: r.response_parameters.clone(),
        response_templates: r.response_templates.clone(),
        template_selection_expression: r.template_selection_expression.clone(),
    }
}

fn int_response_to_model(r: &crate::types::IntegrationResponse) -> ModelIntegrationResponse {
    ModelIntegrationResponse {
        integration_response_id: Some(r.integration_response_id.clone()),
        integration_response_key: Some(r.integration_response_key.clone()),
        content_handling_strategy: r.content_handling_strategy.clone(),
        response_parameters: r.response_parameters.clone(),
        response_templates: r.response_templates.clone(),
        template_selection_expression: r.template_selection_expression.clone(),
    }
}

fn int_response_to_update_response(
    r: &crate::types::IntegrationResponse,
) -> UpdateIntegrationResponseResponse {
    UpdateIntegrationResponseResponse {
        integration_response_id: Some(r.integration_response_id.clone()),
        integration_response_key: Some(r.integration_response_key.clone()),
        content_handling_strategy: r.content_handling_strategy.clone(),
        response_parameters: r.response_parameters.clone(),
        response_templates: r.response_templates.clone(),
        template_selection_expression: r.template_selection_expression.clone(),
    }
}

// ---------------------------------------------------------------------------
// Route Response conversion helpers
// ---------------------------------------------------------------------------

fn route_response_to_create_response(
    r: &crate::types::RouteResponse,
) -> CreateRouteResponseResponse {
    CreateRouteResponseResponse {
        route_response_id: Some(r.route_response_id.clone()),
        route_response_key: Some(r.route_response_key.clone()),
        model_selection_expression: r.model_selection_expression.clone(),
        ..Default::default()
    }
}

fn route_response_to_get_response(r: &crate::types::RouteResponse) -> GetRouteResponseResponse {
    GetRouteResponseResponse {
        route_response_id: Some(r.route_response_id.clone()),
        route_response_key: Some(r.route_response_key.clone()),
        model_selection_expression: r.model_selection_expression.clone(),
        ..Default::default()
    }
}

fn route_response_to_model(r: &crate::types::RouteResponse) -> ModelRouteResponse {
    ModelRouteResponse {
        route_response_id: Some(r.route_response_id.clone()),
        route_response_key: Some(r.route_response_key.clone()),
        model_selection_expression: r.model_selection_expression.clone(),
        ..Default::default()
    }
}
