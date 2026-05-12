use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::Value;
use tokio::sync::RwLock;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::model;
use crate::state::{ApiGatewayError, ApiGatewayState};
use crate::types::{
    EndpointConfiguration, GatewayResponse, Integration, IntegrationResponse, QuotaSettings,
    ThrottleSettings, UsagePlanApiStage,
};
use crate::views::ApiGatewayStateView;
use crate::wire;

#[allow(clippy::result_large_err)]
fn parse_body_value(body: &[u8]) -> Result<Value, MockResponse> {
    if body.is_empty() {
        return Ok(serde_json::json!({}));
    }
    serde_json::from_slice(body)
        .map_err(|_| rest_json_error(400, "BadRequestException", "Invalid JSON body"))
}

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ApiGatewayService {
    pub(crate) state: Arc<BackendState<ApiGatewayState>>,
    pub(crate) notifier: StateChangeNotifier<ApiGatewayStateView>,
}

impl ApiGatewayService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ApiGatewayService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ApiGatewayService {
    fn service_name(&self) -> &str {
        "apigateway"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://apigateway\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ApiGatewayService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();
        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        let query_string = if let Some(pos) = request.uri.find('?') {
            &request.uri[pos + 1..]
        } else {
            ""
        };
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(query_string);

        let response = match (method, segments.as_slice()) {
            // POST /restapis?mode=import — ImportRestApi
            ("POST", ["restapis"]) if query_string.contains("mode=import") => {
                self.handle_import_rest_api(&state, &request, &[], &query_map)
                    .await
            }
            // POST /restapis — CreateRestApi
            ("POST", ["restapis"]) => {
                self.handle_create_rest_api(&state, &request, &[], &query_map)
                    .await
            }
            // GET /restapis — GetRestApis
            ("GET", ["restapis"]) => self.handle_get_rest_apis(&state).await,
            // GET /restapis/{restApiId} — GetRestApi
            ("GET", ["restapis", api_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_get_rest_api(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /restapis/{restApiId} — DeleteRestApi
            ("DELETE", ["restapis", api_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_delete_rest_api(&state, &request, labels, &query_map)
                    .await
            }
            // POST /restapis/{restApiId}/resources/{parentId} — CreateResource
            ("POST", ["restapis", api_id, "resources", parent_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let parent_id_decoded = percent_decode(parent_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("parentId", parent_id_decoded.as_str()),
                ];
                self.handle_create_resource(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/resources — GetResources
            ("GET", ["restapis", api_id, "resources"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_get_resources(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/resources/{resourceId} — GetResource
            ("GET", ["restapis", api_id, "resources", resource_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                ];
                self.handle_get_resource(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /restapis/{restApiId}/resources/{resourceId} — DeleteResource
            ("DELETE", ["restapis", api_id, "resources", resource_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                ];
                self.handle_delete_resource(&state, &request, labels, &query_map)
                    .await
            }
            // POST /restapis/{restApiId}/stages — CreateStage
            ("POST", ["restapis", api_id, "stages"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_create_stage(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/stages — GetStages
            ("GET", ["restapis", api_id, "stages"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_get_stages(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/stages/{stageName} — GetStage
            ("GET", ["restapis", api_id, "stages", stage_name]) => {
                let api_id_decoded = percent_decode(api_id);
                let stage_name_decoded = percent_decode(stage_name);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("stageName", stage_name_decoded.as_str()),
                ];
                self.handle_get_stage(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /restapis/{restApiId}/stages/{stageName} — DeleteStage
            ("DELETE", ["restapis", api_id, "stages", stage_name]) => {
                let api_id_decoded = percent_decode(api_id);
                let stage_name_decoded = percent_decode(stage_name);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("stageName", stage_name_decoded.as_str()),
                ];
                self.handle_delete_stage(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /restapis/{restApiId}/stages/{stageName} — UpdateStage
            ("PATCH", ["restapis", api_id, "stages", stage_name]) => {
                let api_id_decoded = percent_decode(api_id);
                let stage_name_decoded = percent_decode(stage_name);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("stageName", stage_name_decoded.as_str()),
                ];
                self.handle_update_stage(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /restapis/{restApiId}/stages/{stageName}/cache/data — FlushStageCache
            ("DELETE", ["restapis", api_id, "stages", stage_name, "cache", "data"]) => {
                let api_id_decoded = percent_decode(api_id);
                let stage_name_decoded = percent_decode(stage_name);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("stageName", stage_name_decoded.as_str()),
                ];
                self.handle_flush_stage_cache(&state, &request, labels, &query_map)
                    .await
            }
            // POST /restapis/{restApiId}/deployments — CreateDeployment
            ("POST", ["restapis", api_id, "deployments"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_create_deployment(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/deployments — GetDeployments
            ("GET", ["restapis", api_id, "deployments"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_get_deployments(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/deployments/{deploymentId} — GetDeployment
            ("GET", ["restapis", api_id, "deployments", deployment_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let deployment_id_decoded = percent_decode(deployment_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("deploymentId", deployment_id_decoded.as_str()),
                ];
                self.handle_get_deployment(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /restapis/{restApiId}/deployments/{deploymentId} — DeleteDeployment
            ("DELETE", ["restapis", api_id, "deployments", deployment_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let deployment_id_decoded = percent_decode(deployment_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("deploymentId", deployment_id_decoded.as_str()),
                ];
                self.handle_delete_deployment(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod} — PutMethod
            (
                "PUT",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                ];
                self.handle_put_method(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod} — GetMethod
            (
                "GET",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                ];
                self.handle_get_method(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod} — DeleteMethod
            (
                "DELETE",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                ];
                self.handle_delete_method(&state, &request, labels, &query_map)
                    .await
            }
            // PUT .../methods/{httpMethod}/responses/{statusCode} — PutMethodResponse
            (
                "PUT",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                    "responses",
                    status_code,
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let status_code_decoded = percent_decode(status_code);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                    ("statusCode", status_code_decoded.as_str()),
                ];
                self.handle_put_method_response(&state, &request, labels, &query_map)
                    .await
            }
            // GET .../methods/{httpMethod}/responses/{statusCode} — GetMethodResponse
            (
                "GET",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                    "responses",
                    status_code,
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let status_code_decoded = percent_decode(status_code);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                    ("statusCode", status_code_decoded.as_str()),
                ];
                self.handle_get_method_response(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE .../methods/{httpMethod}/responses/{statusCode} — DeleteMethodResponse
            (
                "DELETE",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                    "responses",
                    status_code,
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let status_code_decoded = percent_decode(status_code);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                    ("statusCode", status_code_decoded.as_str()),
                ];
                self.handle_delete_method_response(&state, &request, labels, &query_map)
                    .await
            }
            // PUT .../methods/{httpMethod}/integration — PutIntegration
            (
                "PUT",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                    "integration",
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                ];
                self.handle_put_integration(&state, &request, labels, &query_map)
                    .await
            }
            // GET .../methods/{httpMethod}/integration — GetIntegration
            (
                "GET",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                    "integration",
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                ];
                self.handle_get_integration(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE .../methods/{httpMethod}/integration — DeleteIntegration
            (
                "DELETE",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                    "integration",
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                ];
                self.handle_delete_integration(&state, &request, labels, &query_map)
                    .await
            }
            // PUT .../integration/responses/{statusCode} — PutIntegrationResponse
            (
                "PUT",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                    "integration",
                    "responses",
                    status_code,
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let status_code_decoded = percent_decode(status_code);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                    ("statusCode", status_code_decoded.as_str()),
                ];
                self.handle_put_integration_response(&state, &request, labels, &query_map)
                    .await
            }
            // GET .../integration/responses/{statusCode} — GetIntegrationResponse
            (
                "GET",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                    "integration",
                    "responses",
                    status_code,
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let status_code_decoded = percent_decode(status_code);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                    ("statusCode", status_code_decoded.as_str()),
                ];
                self.handle_get_integration_response(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE .../integration/responses/{statusCode} — DeleteIntegrationResponse
            (
                "DELETE",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                    "integration",
                    "responses",
                    status_code,
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let status_code_decoded = percent_decode(status_code);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                    ("statusCode", status_code_decoded.as_str()),
                ];
                self.handle_delete_integration_response(&state, &request, labels, &query_map)
                    .await
            }
            // POST /restapis/{restApiId}/models — CreateModel
            ("POST", ["restapis", api_id, "models"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_create_model(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/models — GetModels
            ("GET", ["restapis", api_id, "models"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_get_models(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/models/{modelName} — GetModel
            ("GET", ["restapis", api_id, "models", model_name]) => {
                let api_id_decoded = percent_decode(api_id);
                let model_name_decoded = percent_decode(model_name);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("modelName", model_name_decoded.as_str()),
                ];
                self.handle_get_model(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /restapis/{restApiId}/models/{modelName} — DeleteModel
            ("DELETE", ["restapis", api_id, "models", model_name]) => {
                let api_id_decoded = percent_decode(api_id);
                let model_name_decoded = percent_decode(model_name);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("modelName", model_name_decoded.as_str()),
                ];
                self.handle_delete_model(&state, &request, labels, &query_map)
                    .await
            }
            // POST /restapis/{restApiId}/authorizers — CreateAuthorizer
            ("POST", ["restapis", api_id, "authorizers"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_create_authorizer(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/authorizers — GetAuthorizers
            ("GET", ["restapis", api_id, "authorizers"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_get_authorizers(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/authorizers/{authorizerId} — GetAuthorizer
            ("GET", ["restapis", api_id, "authorizers", authorizer_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let authorizer_id_decoded = percent_decode(authorizer_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("authorizerId", authorizer_id_decoded.as_str()),
                ];
                self.handle_get_authorizer(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /restapis/{restApiId}/authorizers/{authorizerId} — DeleteAuthorizer
            ("DELETE", ["restapis", api_id, "authorizers", authorizer_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let authorizer_id_decoded = percent_decode(authorizer_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("authorizerId", authorizer_id_decoded.as_str()),
                ];
                self.handle_delete_authorizer(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /restapis/{restApiId}/authorizers/{authorizerId} — UpdateAuthorizer
            ("PATCH", ["restapis", api_id, "authorizers", authorizer_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let authorizer_id_decoded = percent_decode(authorizer_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("authorizerId", authorizer_id_decoded.as_str()),
                ];
                self.handle_update_authorizer(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /restapis/{restApiId} — UpdateRestApi
            ("PATCH", ["restapis", api_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_update_rest_api(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /restapis/{restApiId} — PutRestApi
            ("PUT", ["restapis", api_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_put_rest_api(&state, &request, labels, &query_map)
                    .await
            }
            // ---- API Key routes ----
            // POST /apikeys?mode=import — ImportApiKeys
            ("POST", ["apikeys"]) if query_string.contains("mode=import") => {
                self.handle_import_api_keys(&state, &request, &[], &query_map)
                    .await
            }
            // POST /apikeys — CreateApiKey
            ("POST", ["apikeys"]) => {
                self.handle_create_api_key(&state, &request, &[], &query_map)
                    .await
            }
            // GET /apikeys — GetApiKeys
            ("GET", ["apikeys"]) => {
                self.handle_get_api_keys(&state, &request, &[], &query_map)
                    .await
            }
            // GET /apikeys/{apiKey} — GetApiKey
            ("GET", ["apikeys", key_id]) => {
                let key_id_decoded = percent_decode(key_id);
                let labels: &[(&str, &str)] = &[("apiKey", key_id_decoded.as_str())];
                self.handle_get_api_key(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /apikeys/{apiKey} — DeleteApiKey
            ("DELETE", ["apikeys", key_id]) => {
                let key_id_decoded = percent_decode(key_id);
                let labels: &[(&str, &str)] = &[("apiKey", key_id_decoded.as_str())];
                self.handle_delete_api_key(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /apikeys/{apiKey} — UpdateApiKey
            ("PATCH", ["apikeys", key_id]) => {
                let key_id_decoded = percent_decode(key_id);
                let labels: &[(&str, &str)] = &[("apiKey", key_id_decoded.as_str())];
                self.handle_update_api_key(&state, &request, labels, &query_map)
                    .await
            }
            // ---- Domain Name routes ----
            // POST /domainnames — CreateDomainName
            ("POST", ["domainnames"]) => {
                self.handle_create_domain_name(&state, &request, &[], &query_map)
                    .await
            }
            // GET /domainnames — GetDomainNames
            ("GET", ["domainnames"]) => {
                self.handle_get_domain_names(&state, &request, &[], &query_map)
                    .await
            }
            // GET /domainnames/{domainName} — GetDomainName
            ("GET", ["domainnames", domain_name]) => {
                let domain_name_decoded = percent_decode(domain_name);
                let labels: &[(&str, &str)] = &[("domainName", domain_name_decoded.as_str())];
                self.handle_get_domain_name(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /domainnames/{domainName} — DeleteDomainName
            ("DELETE", ["domainnames", domain_name]) => {
                let domain_name_decoded = percent_decode(domain_name);
                let labels: &[(&str, &str)] = &[("domainName", domain_name_decoded.as_str())];
                self.handle_delete_domain_name(&state, &request, labels, &query_map)
                    .await
            }
            // ---- Base Path Mapping routes ----
            // POST /domainnames/{domainName}/basepathmappings — CreateBasePathMapping
            ("POST", ["domainnames", domain_name, "basepathmappings"]) => {
                let domain_name_decoded = percent_decode(domain_name);
                let labels: &[(&str, &str)] = &[("domainName", domain_name_decoded.as_str())];
                self.handle_create_base_path_mapping(&state, &request, labels, &query_map)
                    .await
            }
            // GET /domainnames/{domainName}/basepathmappings — GetBasePathMappings
            ("GET", ["domainnames", domain_name, "basepathmappings"]) => {
                let domain_name_decoded = percent_decode(domain_name);
                let labels: &[(&str, &str)] = &[("domainName", domain_name_decoded.as_str())];
                self.handle_get_base_path_mappings(&state, &request, labels, &query_map)
                    .await
            }
            // GET /domainnames/{domainName}/basepathmappings/{basePath} — GetBasePathMapping
            ("GET", ["domainnames", domain_name, "basepathmappings", base_path]) => {
                let domain_name_decoded = percent_decode(domain_name);
                let base_path_decoded = percent_decode(base_path);
                let labels: &[(&str, &str)] = &[
                    ("domainName", domain_name_decoded.as_str()),
                    ("basePath", base_path_decoded.as_str()),
                ];
                self.handle_get_base_path_mapping(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /domainnames/{domainName}/basepathmappings/{basePath} — DeleteBasePathMapping
            ("DELETE", ["domainnames", domain_name, "basepathmappings", base_path]) => {
                let domain_name_decoded = percent_decode(domain_name);
                let base_path_decoded = percent_decode(base_path);
                let labels: &[(&str, &str)] = &[
                    ("domainName", domain_name_decoded.as_str()),
                    ("basePath", base_path_decoded.as_str()),
                ];
                self.handle_delete_base_path_mapping(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /domainnames/{domainName}/basepathmappings/{basePath} — UpdateBasePathMapping
            ("PATCH", ["domainnames", domain_name, "basepathmappings", base_path]) => {
                let domain_name_decoded = percent_decode(domain_name);
                let base_path_decoded = percent_decode(base_path);
                let labels: &[(&str, &str)] = &[
                    ("domainName", domain_name_decoded.as_str()),
                    ("basePath", base_path_decoded.as_str()),
                ];
                self.handle_update_base_path_mapping(&state, &request, labels, &query_map)
                    .await
            }
            // ---- Gateway Response routes ----
            // GET /restapis/{restApiId}/gatewayresponses — GetGatewayResponses
            ("GET", ["restapis", api_id, "gatewayresponses"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_get_gateway_responses(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/gatewayresponses/{responseType} — GetGatewayResponse
            ("GET", ["restapis", api_id, "gatewayresponses", response_type]) => {
                let api_id_decoded = percent_decode(api_id);
                let response_type_decoded = percent_decode(response_type);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("responseType", response_type_decoded.as_str()),
                ];
                self.handle_get_gateway_response(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /restapis/{restApiId}/gatewayresponses/{responseType} — PutGatewayResponse
            ("PUT", ["restapis", api_id, "gatewayresponses", response_type]) => {
                let api_id_decoded = percent_decode(api_id);
                let response_type_decoded = percent_decode(response_type);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("responseType", response_type_decoded.as_str()),
                ];
                self.handle_put_gateway_response(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /restapis/{restApiId}/gatewayresponses/{responseType} — DeleteGatewayResponse
            ("DELETE", ["restapis", api_id, "gatewayresponses", response_type]) => {
                let api_id_decoded = percent_decode(api_id);
                let response_type_decoded = percent_decode(response_type);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("responseType", response_type_decoded.as_str()),
                ];
                self.handle_delete_gateway_response(&state, &request, labels, &query_map)
                    .await
            }
            // ---- Request Validator routes ----
            // POST /restapis/{restApiId}/requestvalidators — CreateRequestValidator
            ("POST", ["restapis", api_id, "requestvalidators"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_create_request_validator(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/requestvalidators — GetRequestValidators
            ("GET", ["restapis", api_id, "requestvalidators"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_get_request_validators(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/requestvalidators/{requestValidatorId} — GetRequestValidator
            ("GET", ["restapis", api_id, "requestvalidators", validator_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let validator_id_decoded = percent_decode(validator_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("requestValidatorId", validator_id_decoded.as_str()),
                ];
                self.handle_get_request_validator(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /restapis/{restApiId}/requestvalidators/{requestValidatorId} — DeleteRequestValidator
            ("DELETE", ["restapis", api_id, "requestvalidators", validator_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let validator_id_decoded = percent_decode(validator_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("requestValidatorId", validator_id_decoded.as_str()),
                ];
                self.handle_delete_request_validator(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /restapis/{restApiId}/requestvalidators/{requestValidatorId} — UpdateRequestValidator
            ("PATCH", ["restapis", api_id, "requestvalidators", validator_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let validator_id_decoded = percent_decode(validator_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("requestValidatorId", validator_id_decoded.as_str()),
                ];
                self.handle_update_request_validator(&state, &request, labels, &query_map)
                    .await
            }
            // ---- Usage Plan routes ----
            // POST /usageplans — CreateUsagePlan
            ("POST", ["usageplans"]) => {
                self.handle_create_usage_plan(&state, &request, &[], &query_map)
                    .await
            }
            // GET /usageplans — GetUsagePlans
            ("GET", ["usageplans"]) => {
                self.handle_get_usage_plans(&state, &request, &[], &query_map)
                    .await
            }
            // GET /usageplans/{usagePlanId} — GetUsagePlan
            ("GET", ["usageplans", plan_id]) => {
                let plan_id_decoded = percent_decode(plan_id);
                let labels: &[(&str, &str)] = &[("usagePlanId", plan_id_decoded.as_str())];
                self.handle_get_usage_plan(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /usageplans/{usagePlanId} — DeleteUsagePlan
            ("DELETE", ["usageplans", plan_id]) => {
                let plan_id_decoded = percent_decode(plan_id);
                let labels: &[(&str, &str)] = &[("usagePlanId", plan_id_decoded.as_str())];
                self.handle_delete_usage_plan(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /usageplans/{usagePlanId} — UpdateUsagePlan
            ("PATCH", ["usageplans", plan_id]) => {
                let plan_id_decoded = percent_decode(plan_id);
                let labels: &[(&str, &str)] = &[("usagePlanId", plan_id_decoded.as_str())];
                self.handle_update_usage_plan(&state, &request, labels, &query_map)
                    .await
            }
            // POST /usageplans/{usagePlanId}/keys — CreateUsagePlanKey
            ("POST", ["usageplans", plan_id, "keys"]) => {
                let plan_id_decoded = percent_decode(plan_id);
                let labels: &[(&str, &str)] = &[("usagePlanId", plan_id_decoded.as_str())];
                self.handle_create_usage_plan_key(&state, &request, labels, &query_map)
                    .await
            }
            // GET /usageplans/{usagePlanId}/keys — GetUsagePlanKeys
            ("GET", ["usageplans", plan_id, "keys"]) => {
                let plan_id_decoded = percent_decode(plan_id);
                let labels: &[(&str, &str)] = &[("usagePlanId", plan_id_decoded.as_str())];
                self.handle_get_usage_plan_keys(&state, &request, labels, &query_map)
                    .await
            }
            // GET /usageplans/{usagePlanId}/keys/{keyId} — GetUsagePlanKey
            ("GET", ["usageplans", plan_id, "keys", key_id]) => {
                let plan_id_decoded = percent_decode(plan_id);
                let key_id_decoded = percent_decode(key_id);
                let labels: &[(&str, &str)] = &[
                    ("usagePlanId", plan_id_decoded.as_str()),
                    ("keyId", key_id_decoded.as_str()),
                ];
                self.handle_get_usage_plan_key(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /usageplans/{usagePlanId}/keys/{keyId} — DeleteUsagePlanKey
            ("DELETE", ["usageplans", plan_id, "keys", key_id]) => {
                let plan_id_decoded = percent_decode(plan_id);
                let key_id_decoded = percent_decode(key_id);
                let labels: &[(&str, &str)] = &[
                    ("usagePlanId", plan_id_decoded.as_str()),
                    ("keyId", key_id_decoded.as_str()),
                ];
                self.handle_delete_usage_plan_key(&state, &request, labels, &query_map)
                    .await
            }
            // ---- VPC Link routes ----
            // POST /vpclinks — CreateVpcLink
            ("POST", ["vpclinks"]) => {
                self.handle_create_vpc_link(&state, &request, &[], &query_map)
                    .await
            }
            // GET /vpclinks — GetVpcLinks
            ("GET", ["vpclinks"]) => {
                self.handle_get_vpc_links(&state, &request, &[], &query_map)
                    .await
            }
            // GET /vpclinks/{vpcLinkId} — GetVpcLink
            ("GET", ["vpclinks", link_id]) => {
                let link_id_decoded = percent_decode(link_id);
                let labels: &[(&str, &str)] = &[("vpcLinkId", link_id_decoded.as_str())];
                self.handle_get_vpc_link(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /vpclinks/{vpcLinkId} — DeleteVpcLink
            ("DELETE", ["vpclinks", link_id]) => {
                let link_id_decoded = percent_decode(link_id);
                let labels: &[(&str, &str)] = &[("vpcLinkId", link_id_decoded.as_str())];
                self.handle_delete_vpc_link(&state, &request, labels, &query_map)
                    .await
            }
            // ---- Client Certificate routes ----
            // POST /clientcertificates — GenerateClientCertificate
            ("POST", ["clientcertificates"]) => {
                self.handle_generate_client_certificate(&state, &request, &[], &query_map)
                    .await
            }
            // GET /clientcertificates — GetClientCertificates
            ("GET", ["clientcertificates"]) => {
                self.handle_get_client_certificates(&state, &request, &[], &query_map)
                    .await
            }
            // GET /clientcertificates/{clientCertificateId} — GetClientCertificate
            ("GET", ["clientcertificates", cert_id]) => {
                let cert_id_decoded = percent_decode(cert_id);
                let labels: &[(&str, &str)] = &[("clientCertificateId", cert_id_decoded.as_str())];
                self.handle_get_client_certificate(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /clientcertificates/{clientCertificateId} — DeleteClientCertificate
            ("DELETE", ["clientcertificates", cert_id]) => {
                let cert_id_decoded = percent_decode(cert_id);
                let labels: &[(&str, &str)] = &[("clientCertificateId", cert_id_decoded.as_str())];
                self.handle_delete_client_certificate(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /clientcertificates/{clientCertificateId} — UpdateClientCertificate
            ("PATCH", ["clientcertificates", cert_id]) => {
                let cert_id_decoded = percent_decode(cert_id);
                let labels: &[(&str, &str)] = &[("clientCertificateId", cert_id_decoded.as_str())];
                self.handle_update_client_certificate(&state, &request, labels, &query_map)
                    .await
            }
            // ---- Documentation Part routes ----
            // POST /restapis/{restApiId}/documentation/parts — CreateDocumentationPart
            ("POST", ["restapis", api_id, "documentation", "parts"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_create_documentation_part(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/documentation/parts — GetDocumentationParts
            ("GET", ["restapis", api_id, "documentation", "parts"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_get_documentation_parts(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/documentation/parts/{documentationPartId} — GetDocumentationPart
            ("GET", ["restapis", api_id, "documentation", "parts", part_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let part_id_decoded = percent_decode(part_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("documentationPartId", part_id_decoded.as_str()),
                ];
                self.handle_get_documentation_part(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /restapis/{restApiId}/documentation/parts/{documentationPartId} — DeleteDocumentationPart
            ("DELETE", ["restapis", api_id, "documentation", "parts", part_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let part_id_decoded = percent_decode(part_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("documentationPartId", part_id_decoded.as_str()),
                ];
                self.handle_delete_documentation_part(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /restapis/{restApiId}/documentation/parts/{documentationPartId} — UpdateDocumentationPart
            ("PATCH", ["restapis", api_id, "documentation", "parts", part_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let part_id_decoded = percent_decode(part_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("documentationPartId", part_id_decoded.as_str()),
                ];
                self.handle_update_documentation_part(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /restapis/{restApiId}/documentation/parts — ImportDocumentationParts
            ("PUT", ["restapis", api_id, "documentation", "parts"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_import_documentation_parts(&state, &request, labels, &query_map)
                    .await
            }
            // ---- Documentation Version routes ----
            // POST /restapis/{restApiId}/documentation/versions — CreateDocumentationVersion
            ("POST", ["restapis", api_id, "documentation", "versions"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_create_documentation_version(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/documentation/versions — GetDocumentationVersions
            ("GET", ["restapis", api_id, "documentation", "versions"]) => {
                let api_id_decoded = percent_decode(api_id);
                let labels: &[(&str, &str)] = &[("restApiId", api_id_decoded.as_str())];
                self.handle_get_documentation_versions(&state, &request, labels, &query_map)
                    .await
            }
            // GET /restapis/{restApiId}/documentation/versions/{documentationVersion} — GetDocumentationVersion
            ("GET", ["restapis", api_id, "documentation", "versions", doc_version]) => {
                let api_id_decoded = percent_decode(api_id);
                let doc_version_decoded = percent_decode(doc_version);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("documentationVersion", doc_version_decoded.as_str()),
                ];
                self.handle_get_documentation_version(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /restapis/{restApiId}/documentation/versions/{documentationVersion} — DeleteDocumentationVersion
            ("DELETE", ["restapis", api_id, "documentation", "versions", doc_version]) => {
                let api_id_decoded = percent_decode(api_id);
                let doc_version_decoded = percent_decode(doc_version);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("documentationVersion", doc_version_decoded.as_str()),
                ];
                self.handle_delete_documentation_version(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /restapis/{restApiId}/documentation/versions/{documentationVersion} — UpdateDocumentationVersion
            ("PATCH", ["restapis", api_id, "documentation", "versions", doc_version]) => {
                let api_id_decoded = percent_decode(api_id);
                let doc_version_decoded = percent_decode(doc_version);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("documentationVersion", doc_version_decoded.as_str()),
                ];
                self.handle_update_documentation_version(&state, &request, labels, &query_map)
                    .await
            }
            // ---- Stage cache routes ----
            // DELETE /restapis/{restApiId}/stages/{stageName}/cache/authorizers — FlushStageAuthorizersCache
            (
                "DELETE",
                [
                    "restapis",
                    api_id,
                    "stages",
                    stage_name,
                    "cache",
                    "authorizers",
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let stage_name_decoded = percent_decode(stage_name);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("stageName", stage_name_decoded.as_str()),
                ];
                self.handle_flush_stage_authorizers_cache(&state, &request, labels, &query_map)
                    .await
            }
            // ---- SDK routes ----
            // GET /sdktypes — GetSdkTypes
            ("GET", ["sdktypes"]) => self.handle_get_sdk_types().await,
            // GET /sdktypes/{id} — GetSdkType
            ("GET", ["sdktypes", sdk_type_id]) => {
                let sdk_type_id_decoded = percent_decode(sdk_type_id);
                let labels: &[(&str, &str)] = &[("id", sdk_type_id_decoded.as_str())];
                self.handle_get_sdk_type(&request, labels, &query_map).await
            }
            // GET /restapis/{restApiId}/stages/{stageName}/sdks/{sdkType} — GetSdk
            ("GET", ["restapis", api_id, "stages", stage_name, "sdks", sdk_type]) => {
                let api_id_decoded = percent_decode(api_id);
                let stage_name_decoded = percent_decode(stage_name);
                let sdk_type_decoded = percent_decode(sdk_type);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("stageName", stage_name_decoded.as_str()),
                    ("sdkType", sdk_type_decoded.as_str()),
                ];
                self.handle_get_sdk(&state, &request, labels, &query_map)
                    .await
            }
            // ---- Export routes ----
            // GET /restapis/{restApiId}/stages/{stageName}/exports/{exportType} — GetExport
            (
                "GET",
                [
                    "restapis",
                    api_id,
                    "stages",
                    stage_name,
                    "exports",
                    export_type,
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let stage_name_decoded = percent_decode(stage_name);
                let export_type_decoded = percent_decode(export_type);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("stageName", stage_name_decoded.as_str()),
                    ("exportType", export_type_decoded.as_str()),
                ];
                self.handle_get_export(&state, &request, labels, &query_map)
                    .await
            }
            // ---- Model template route ----
            // GET /restapis/{restApiId}/models/{modelName}/default_template — GetModelTemplate
            ("GET", ["restapis", api_id, "models", model_name, "default_template"]) => {
                let api_id_decoded = percent_decode(api_id);
                let model_name_decoded = percent_decode(model_name);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("modelName", model_name_decoded.as_str()),
                ];
                self.handle_get_model_template(&state, &request, labels, &query_map)
                    .await
            }
            // ---- Tag routes ----
            // GET /tags/{resourceArn} — GetTags
            ("GET", ["tags", resource_arn]) => {
                let resource_arn_decoded = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn_decoded.as_str())];
                self.handle_get_tags(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /tags/{resourceArn} — TagResource
            ("PUT", ["tags", resource_arn]) => {
                let resource_arn_decoded = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn_decoded.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /tags/{resourceArn} — UntagResource
            ("DELETE", ["tags", resource_arn]) => {
                let resource_arn_decoded = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn_decoded.as_str())];
                self.handle_untag_resource(&state, &request, labels, &query_map, query_string)
                    .await
            }
            // ---- Usage route ----
            // GET /usageplans/{usagePlanId}/usage — GetUsage
            ("GET", ["usageplans", plan_id, "usage"]) => {
                let plan_id_decoded = percent_decode(plan_id);
                let labels: &[(&str, &str)] = &[("usagePlanId", plan_id_decoded.as_str())];
                self.handle_get_usage(&state, &request, labels, &query_map)
                    .await
            }
            // ---- Domain Name Access Association routes ----
            // POST /domainnameaccessassociations — CreateDomainNameAccessAssociation
            ("POST", ["domainnameaccessassociations"]) => {
                self.handle_create_domain_name_access_association(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    &region,
                )
                .await
            }
            // GET /domainnameaccessassociations — GetDomainNameAccessAssociations
            ("GET", ["domainnameaccessassociations"]) => {
                self.handle_get_domain_name_access_associations(&state, &request, &[], &query_map)
                    .await
            }
            // DELETE /domainnameaccessassociations/{domainNameAccessAssociationArn} — DeleteDomainNameAccessAssociation
            ("DELETE", ["domainnameaccessassociations", assoc_arn]) => {
                let assoc_arn_decoded = percent_decode(assoc_arn);
                let labels: &[(&str, &str)] =
                    &[("domainNameAccessAssociationArn", assoc_arn_decoded.as_str())];
                self.handle_delete_domain_name_access_association(
                    &state, &request, labels, &query_map,
                )
                .await
            }
            // POST /rejectdomainnameaccessassociations — RejectDomainNameAccessAssociation
            ("POST", ["rejectdomainnameaccessassociations"]) => {
                self.handle_reject_domain_name_access_association(&request, &[], &query_map)
                    .await
            }
            // ---- Update routes ----
            // PATCH /restapis/{restApiId}/deployments/{deploymentId} — UpdateDeployment
            ("PATCH", ["restapis", api_id, "deployments", deployment_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let deployment_id_decoded = percent_decode(deployment_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("deploymentId", deployment_id_decoded.as_str()),
                ];
                self.handle_update_deployment(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /domainnames/{domainName} — UpdateDomainName
            ("PATCH", ["domainnames", domain_name]) => {
                let domain_name_decoded = percent_decode(domain_name);
                let labels: &[(&str, &str)] = &[("domainName", domain_name_decoded.as_str())];
                self.handle_update_domain_name(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /restapis/{restApiId}/gatewayresponses/{responseType} — UpdateGatewayResponse
            ("PATCH", ["restapis", api_id, "gatewayresponses", response_type]) => {
                let api_id_decoded = percent_decode(api_id);
                let response_type_decoded = percent_decode(response_type);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("responseType", response_type_decoded.as_str()),
                ];
                self.handle_update_gateway_response(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /restapis/{restApiId}/models/{modelName} — UpdateModel
            ("PATCH", ["restapis", api_id, "models", model_name]) => {
                let api_id_decoded = percent_decode(api_id);
                let model_name_decoded = percent_decode(model_name);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("modelName", model_name_decoded.as_str()),
                ];
                self.handle_update_model(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /vpclinks/{vpcLinkId} — UpdateVpcLink
            ("PATCH", ["vpclinks", link_id]) => {
                let link_id_decoded = percent_decode(link_id);
                let labels: &[(&str, &str)] = &[("vpcLinkId", link_id_decoded.as_str())];
                self.handle_update_vpc_link(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /usageplans/{usagePlanId}/keys/{keyId}/usage — UpdateUsage
            ("PATCH", ["usageplans", plan_id, "keys", key_id, "usage"]) => {
                let plan_id_decoded = percent_decode(plan_id);
                let key_id_decoded = percent_decode(key_id);
                let labels: &[(&str, &str)] = &[
                    ("usagePlanId", plan_id_decoded.as_str()),
                    ("keyId", key_id_decoded.as_str()),
                ];
                self.handle_update_usage(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /restapis/{restApiId}/resources/{resourceId} — UpdateResource
            ("PATCH", ["restapis", api_id, "resources", resource_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                ];
                self.handle_update_resource(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod} — UpdateMethod
            (
                "PATCH",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                ];
                self.handle_update_method(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH .../methods/{httpMethod}/responses/{statusCode} — UpdateMethodResponse
            (
                "PATCH",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                    "responses",
                    status_code,
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let status_code_decoded = percent_decode(status_code);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                    ("statusCode", status_code_decoded.as_str()),
                ];
                self.handle_update_method_response(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH .../methods/{httpMethod}/integration — UpdateIntegration
            (
                "PATCH",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                    "integration",
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                ];
                self.handle_update_integration(&state, &request, labels, &query_map)
                    .await
            }
            // PATCH .../methods/{httpMethod}/integration/responses/{statusCode} — UpdateIntegrationResponse
            (
                "PATCH",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                    "integration",
                    "responses",
                    status_code,
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let status_code_decoded = percent_decode(status_code);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                    ("statusCode", status_code_decoded.as_str()),
                ];
                self.handle_update_integration_response(&state, &request, labels, &query_map)
                    .await
            }
            // ---- TestInvoke routes ----
            // POST /restapis/{restApiId}/authorizers/{authorizerId} — TestInvokeAuthorizer
            ("POST", ["restapis", api_id, "authorizers", authorizer_id]) => {
                let api_id_decoded = percent_decode(api_id);
                let authorizer_id_decoded = percent_decode(authorizer_id);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("authorizerId", authorizer_id_decoded.as_str()),
                ];
                self.handle_test_invoke_authorizer(&state, &request, labels, &query_map)
                    .await
            }
            // POST /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod} — TestInvokeMethod
            (
                "POST",
                [
                    "restapis",
                    api_id,
                    "resources",
                    resource_id,
                    "methods",
                    http_method,
                ],
            ) => {
                let api_id_decoded = percent_decode(api_id);
                let resource_id_decoded = percent_decode(resource_id);
                let http_method_decoded = percent_decode(http_method);
                let labels: &[(&str, &str)] = &[
                    ("restApiId", api_id_decoded.as_str()),
                    ("resourceId", resource_id_decoded.as_str()),
                    ("httpMethod", http_method_decoded.as_str()),
                ];
                self.handle_test_invoke_method(&state, &request, labels, &query_map)
                    .await
            }
            // ---- Account routes ----
            // GET /account — GetAccount
            ("GET", ["account"]) => {
                self.handle_get_account(&state, &request, &[], &query_map)
                    .await
            }
            // PATCH /account — UpdateAccount
            ("PATCH", ["account"]) => {
                self.handle_update_account(&state, &request, &[], &query_map)
                    .await
            }
            // --- Unimplemented operations (auto-generated stubs) ---
            // POST /apikeys => CreateApiKey (not implemented)
            // POST /restapis/{restApiId}/authorizers => CreateAuthorizer (not implemented)
            // POST /domainnames/{domainName}/basepathmappings => CreateBasePathMapping (not implemented)
            // POST /restapis/{restApiId}/deployments => CreateDeployment (not implemented)
            // POST /restapis/{restApiId}/documentation/parts => CreateDocumentationPart (not implemented)
            // POST /restapis/{restApiId}/documentation/versions => CreateDocumentationVersion (not implemented)
            // POST /domainnames => CreateDomainName (not implemented)
            // POST /domainnameaccessassociations => CreateDomainNameAccessAssociation (not implemented)
            // POST /restapis/{restApiId}/models => CreateModel (not implemented)
            // POST /restapis/{restApiId}/requestvalidators => CreateRequestValidator (not implemented)
            // POST /restapis/{restApiId}/stages => CreateStage (not implemented)
            // POST /usageplans => CreateUsagePlan (not implemented)
            // POST /usageplans/{usagePlanId}/keys => CreateUsagePlanKey (not implemented)
            // POST /vpclinks => CreateVpcLink (not implemented)
            // DELETE /apikeys/{apiKey} => DeleteApiKey (not implemented)
            // DELETE /restapis/{restApiId}/authorizers/{authorizerId} => DeleteAuthorizer (not implemented)
            // DELETE /domainnames/{domainName}/basepathmappings/{basePath} => DeleteBasePathMapping (not implemented)
            // DELETE /clientcertificates/{clientCertificateId} => DeleteClientCertificate (not implemented)
            // DELETE /restapis/{restApiId}/deployments/{deploymentId} => DeleteDeployment (not implemented)
            // DELETE /restapis/{restApiId}/documentation/parts/{documentationPartId} => DeleteDocumentationPart (not implemented)
            // DELETE /restapis/{restApiId}/documentation/versions/{documentationVersion} => DeleteDocumentationVersion (not implemented)
            // DELETE /domainnames/{domainName} => DeleteDomainName (not implemented)
            // DELETE /domainnameaccessassociations/{domainNameAccessAssociationArn} => DeleteDomainNameAccessAssociation (not implemented)
            // DELETE /restapis/{restApiId}/gatewayresponses/{responseType} => DeleteGatewayResponse (not implemented)
            // DELETE /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration => DeleteIntegration (not implemented)
            // DELETE /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration/responses/{statusCode} => DeleteIntegrationResponse (not implemented)
            // DELETE /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod} => DeleteMethod (not implemented)
            // DELETE /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/responses/{statusCode} => DeleteMethodResponse (not implemented)
            // DELETE /restapis/{restApiId}/models/{modelName} => DeleteModel (not implemented)
            // DELETE /restapis/{restApiId}/requestvalidators/{requestValidatorId} => DeleteRequestValidator (not implemented)
            // DELETE /restapis/{restApiId}/stages/{stageName} => DeleteStage (not implemented)
            // DELETE /usageplans/{usagePlanId} => DeleteUsagePlan (not implemented)
            // DELETE /usageplans/{usagePlanId}/keys/{keyId} => DeleteUsagePlanKey (not implemented)
            // DELETE /vpclinks/{vpcLinkId} => DeleteVpcLink (not implemented)
            // DELETE /restapis/{restApiId}/stages/{stageName}/cache/authorizers => FlushStageAuthorizersCache (not implemented)
            // DELETE /restapis/{restApiId}/stages/{stageName}/cache/data => FlushStageCache (not implemented)
            // POST /clientcertificates => GenerateClientCertificate (not implemented)
            // GET /account => GetAccount (not implemented)
            // GET /apikeys/{apiKey} => GetApiKey (not implemented)
            // GET /apikeys => GetApiKeys (not implemented)
            // GET /restapis/{restApiId}/authorizers/{authorizerId} => GetAuthorizer (not implemented)
            // GET /restapis/{restApiId}/authorizers => GetAuthorizers (not implemented)
            // GET /domainnames/{domainName}/basepathmappings/{basePath} => GetBasePathMapping (not implemented)
            // GET /domainnames/{domainName}/basepathmappings => GetBasePathMappings (not implemented)
            // GET /clientcertificates/{clientCertificateId} => GetClientCertificate (not implemented)
            // GET /clientcertificates => GetClientCertificates (not implemented)
            // GET /restapis/{restApiId}/deployments/{deploymentId} => GetDeployment (not implemented)
            // GET /restapis/{restApiId}/deployments => GetDeployments (not implemented)
            // GET /restapis/{restApiId}/documentation/parts/{documentationPartId} => GetDocumentationPart (not implemented)
            // GET /restapis/{restApiId}/documentation/parts => GetDocumentationParts (not implemented)
            // GET /restapis/{restApiId}/documentation/versions/{documentationVersion} => GetDocumentationVersion (not implemented)
            // GET /restapis/{restApiId}/documentation/versions => GetDocumentationVersions (not implemented)
            // GET /domainnames/{domainName} => GetDomainName (not implemented)
            // GET /domainnameaccessassociations => GetDomainNameAccessAssociations (not implemented)
            // GET /domainnames => GetDomainNames (not implemented)
            // GET /restapis/{restApiId}/stages/{stageName}/exports/{exportType} => GetExport (not implemented)
            // GET /restapis/{restApiId}/gatewayresponses/{responseType} => GetGatewayResponse (not implemented)
            // GET /restapis/{restApiId}/gatewayresponses => GetGatewayResponses (not implemented)
            // GET /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration => GetIntegration (not implemented)
            // GET /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration/responses/{statusCode} => GetIntegrationResponse (not implemented)
            // GET /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod} => GetMethod (not implemented)
            // GET /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/responses/{statusCode} => GetMethodResponse (not implemented)
            // GET /restapis/{restApiId}/models/{modelName} => GetModel (not implemented)
            // GET /restapis/{restApiId}/models/{modelName}/default_template => GetModelTemplate (not implemented)
            // GET /restapis/{restApiId}/models => GetModels (not implemented)
            // GET /restapis/{restApiId}/requestvalidators/{requestValidatorId} => GetRequestValidator (not implemented)
            // GET /restapis/{restApiId}/requestvalidators => GetRequestValidators (not implemented)
            // GET /restapis/{restApiId}/stages/{stageName}/sdks/{sdkType} => GetSdk (not implemented)
            // GET /sdktypes/{id} => GetSdkType (not implemented)
            // GET /sdktypes => GetSdkTypes (not implemented)
            // GET /restapis/{restApiId}/stages/{stageName} => GetStage (not implemented)
            // GET /restapis/{restApiId}/stages => GetStages (not implemented)
            // GET /tags/{resourceArn} => GetTags (not implemented)
            // GET /usageplans/{usagePlanId}/usage => GetUsage (not implemented)
            // GET /usageplans/{usagePlanId} => GetUsagePlan (not implemented)
            // GET /usageplans/{usagePlanId}/keys/{keyId} => GetUsagePlanKey (not implemented)
            // GET /usageplans/{usagePlanId}/keys => GetUsagePlanKeys (not implemented)
            // GET /usageplans => GetUsagePlans (not implemented)
            // GET /vpclinks/{vpcLinkId} => GetVpcLink (not implemented)
            // GET /vpclinks => GetVpcLinks (not implemented)
            // POST /apikeys?mode=import => ImportApiKeys (not implemented)
            // PUT /restapis/{restApiId}/documentation/parts => ImportDocumentationParts (not implemented)
            // POST /restapis?mode=import => ImportRestApi (not implemented)
            // PUT /restapis/{restApiId}/gatewayresponses/{responseType} => PutGatewayResponse (not implemented)
            // PUT /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration => PutIntegration (not implemented)
            // PUT /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration/responses/{statusCode} => PutIntegrationResponse (not implemented)
            // PUT /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod} => PutMethod (not implemented)
            // PUT /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/responses/{statusCode} => PutMethodResponse (not implemented)
            // PUT /restapis/{restApiId} => PutRestApi (not implemented)
            // POST /rejectdomainnameaccessassociations => RejectDomainNameAccessAssociation (not implemented)
            // PUT /tags/{resourceArn} => TagResource (not implemented)
            // POST /restapis/{restApiId}/authorizers/{authorizerId} => TestInvokeAuthorizer (not implemented)
            // POST /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod} => TestInvokeMethod (not implemented)
            // DELETE /tags/{resourceArn} => UntagResource (not implemented)
            // PATCH /account => UpdateAccount (not implemented)
            // PATCH /apikeys/{apiKey} => UpdateApiKey (not implemented)
            // PATCH /restapis/{restApiId}/authorizers/{authorizerId} => UpdateAuthorizer (not implemented)
            // PATCH /domainnames/{domainName}/basepathmappings/{basePath} => UpdateBasePathMapping (not implemented)
            // PATCH /clientcertificates/{clientCertificateId} => UpdateClientCertificate (not implemented)
            // PATCH /restapis/{restApiId}/deployments/{deploymentId} => UpdateDeployment (not implemented)
            // PATCH /restapis/{restApiId}/documentation/parts/{documentationPartId} => UpdateDocumentationPart (not implemented)
            // PATCH /restapis/{restApiId}/documentation/versions/{documentationVersion} => UpdateDocumentationVersion (not implemented)
            // PATCH /domainnames/{domainName} => UpdateDomainName (not implemented)
            // PATCH /restapis/{restApiId}/gatewayresponses/{responseType} => UpdateGatewayResponse (not implemented)
            // PATCH /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration => UpdateIntegration (not implemented)
            // PATCH /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration/responses/{statusCode} => UpdateIntegrationResponse (not implemented)
            // PATCH /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod} => UpdateMethod (not implemented)
            // PATCH /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/responses/{statusCode} => UpdateMethodResponse (not implemented)
            // PATCH /restapis/{restApiId}/models/{modelName} => UpdateModel (not implemented)
            // PATCH /restapis/{restApiId}/requestvalidators/{requestValidatorId} => UpdateRequestValidator (not implemented)
            // PATCH /restapis/{restApiId}/resources/{resourceId} => UpdateResource (not implemented)
            // PATCH /restapis/{restApiId} => UpdateRestApi (not implemented)
            // PATCH /restapis/{restApiId}/stages/{stageName} => UpdateStage (not implemented)
            // PATCH /usageplans/{usagePlanId}/keys/{keyId}/usage => UpdateUsage (not implemented)
            // PATCH /usageplans/{usagePlanId} => UpdateUsagePlan (not implemented)
            // PATCH /vpclinks/{vpcLinkId} => UpdateVpcLink (not implemented)

            // 116 unimplemented operations above
            _ => rest_json_error(404, "NotFoundException", "Not found"),
        };

        if matches!(method, "POST" | "PUT" | "PATCH" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // POST /restapis - CreateRestApi
    async fn handle_create_rest_api(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_rest_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing required field 'name'");
        }
        let endpoint_configuration =
            input
                .endpoint_configuration
                .as_ref()
                .map(|ec| EndpointConfiguration {
                    types: ec.types.clone().unwrap_or_default(),
                    vpc_endpoint_ids: ec.vpc_endpoint_ids.clone().unwrap_or_default(),
                });

        let mut s = state.write().await;
        let api = s.create_rest_api(
            &input.name,
            input.description.as_deref(),
            input.version.as_deref(),
            endpoint_configuration,
            input.tags.unwrap_or_default(),
            input.disable_execute_api_endpoint,
            input.policy.as_deref(),
            input.api_key_source.as_deref(),
            input.binary_media_types.unwrap_or_default(),
            input.minimum_compression_size,
        );
        wire::serialize_create_rest_api_response(&rest_api_to_model(api))
    }

    // GET /restapis/{restapi_id} - GetRestApi
    async fn handle_get_rest_api(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_rest_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_rest_api(&input.rest_api_id) {
            Ok(api) => wire::serialize_get_rest_api_response(&rest_api_to_model(api)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_rest_apis(&self, state: &Arc<RwLock<ApiGatewayState>>) -> MockResponse {
        let s = state.read().await;
        let apis: Vec<model::RestApi> = s
            .list_rest_apis()
            .into_iter()
            .map(rest_api_to_model)
            .collect();
        wire::serialize_get_rest_apis_response(&model::RestApis {
            items: if apis.is_empty() { None } else { Some(apis) },
        })
    }

    // DELETE /restapis/{restapi_id} - DeleteRestApi
    async fn handle_delete_rest_api(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_rest_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_rest_api(&input.rest_api_id) {
            Ok(()) => MockResponse::rest_json(202, String::new()),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_create_resource(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.path_part.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing required field 'pathPart'",
            );
        }
        let mut s = state.write().await;
        match s.create_resource(&input.rest_api_id, &input.parent_id, &input.path_part) {
            Ok(resource) => wire::serialize_create_resource_response(&resource_to_model(&resource)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_resource(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_resource(&input.rest_api_id, &input.resource_id) {
            Ok(resource) => wire::serialize_get_resource_response(&resource_to_model(resource)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_resources(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_resources_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_resources(&input.rest_api_id) {
            Ok(resources) => {
                let items: Vec<model::Resource> =
                    resources.into_iter().map(resource_to_model).collect();
                wire::serialize_get_resources_response(&model::Resources {
                    items: if items.is_empty() { None } else { Some(items) },
                })
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_delete_resource(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_resource(&input.rest_api_id, &input.resource_id) {
            Ok(()) => MockResponse::rest_json(204, String::new()),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Stage handlers ----

    async fn handle_create_stage(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_stage_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.stage_name.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing required field 'stageName'",
            );
        }
        if input.deployment_id.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing required field 'deploymentId'",
            );
        }
        let mut s = state.write().await;
        match s.create_stage(
            &input.rest_api_id,
            &input.stage_name,
            &input.deployment_id,
            input.description.as_deref(),
            input.tracing_enabled,
            input.tags.unwrap_or_default(),
            input.variables.unwrap_or_default(),
            input.cache_cluster_enabled,
            input.cache_cluster_size,
            input.documentation_version,
        ) {
            Ok(stage) => wire::serialize_create_stage_response(&stage_to_model(stage)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_stage(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_stage_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_stage(&input.rest_api_id, &input.stage_name) {
            Ok(stage) => wire::serialize_get_stage_response(&stage_to_model(stage)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_stages(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_stages_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_stages(&input.rest_api_id) {
            Ok(stages) => {
                let items: Vec<model::Stage> = stages.into_iter().map(stage_to_model).collect();
                wire::serialize_get_stages_response(&model::Stages {
                    item: if items.is_empty() { None } else { Some(items) },
                })
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_delete_stage(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_stage_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_stage(&input.rest_api_id, &input.stage_name) {
            Ok(()) => wire::serialize_delete_stage_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_stage(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_stage_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let patch_ops = patch_operations_to_value(&input.patch_operations);
        let mut s = state.write().await;
        match s.update_stage(&input.rest_api_id, &input.stage_name, &patch_ops) {
            Ok(stage) => wire::serialize_get_stage_response(&stage_to_model(stage)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_flush_stage_cache(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_flush_stage_cache_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_stage(&input.rest_api_id, &input.stage_name) {
            Ok(_) => wire::serialize_flush_stage_cache_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Deployment handlers ----

    async fn handle_create_deployment(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_deployment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.create_deployment(&input.rest_api_id, input.description.as_deref()) {
            Ok(d) => wire::serialize_create_deployment_response(&deployment_to_model(d)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_deployment(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_deployment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_deployment(&input.rest_api_id, &input.deployment_id) {
            Ok(d) => wire::serialize_get_deployment_response(&deployment_to_model(d)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_deployments(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_deployments_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_deployments(&input.rest_api_id) {
            Ok(deployments) => {
                let items: Vec<model::Deployment> =
                    deployments.into_iter().map(deployment_to_model).collect();
                wire::serialize_get_deployments_response(&model::Deployments {
                    items: if items.is_empty() { None } else { Some(items) },
                })
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_delete_deployment(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_deployment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_deployment(&input.rest_api_id, &input.deployment_id) {
            Ok(()) => wire::serialize_delete_deployment_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Method handlers ----

    async fn handle_put_method(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_method_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.authorization_type.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing required field 'authorizationType'",
            );
        }
        let mut s = state.write().await;
        match s.put_method(
            &input.rest_api_id,
            &input.resource_id,
            &input.http_method,
            &input.authorization_type,
            input.authorizer_id.as_deref(),
            input.api_key_required,
            input.operation_name.as_deref(),
            input.request_models.unwrap_or_default(),
            input.request_parameters.unwrap_or_default(),
            input.request_validator_id.as_deref(),
        ) {
            Ok(m) => wire::serialize_put_method_response(&method_to_model(m)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_method(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_method_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_method(&input.rest_api_id, &input.resource_id, &input.http_method) {
            Ok(m) => wire::serialize_get_method_response(&method_to_model(m)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_delete_method(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_method_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_method(&input.rest_api_id, &input.resource_id, &input.http_method) {
            Ok(()) => wire::serialize_delete_method_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- MethodResponse handlers ----

    async fn handle_put_method_response(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_method_response_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.put_method_response(
            &input.rest_api_id,
            &input.resource_id,
            &input.http_method,
            &input.status_code,
            input.response_models.unwrap_or_default(),
            input.response_parameters.unwrap_or_default(),
        ) {
            Ok(mr) => wire::serialize_put_method_response_response(&method_response_to_model(mr)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_method_response(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_method_response_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_method_response(
            &input.rest_api_id,
            &input.resource_id,
            &input.http_method,
            &input.status_code,
        ) {
            Ok(mr) => wire::serialize_get_method_response_response(&method_response_to_model(mr)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_delete_method_response(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_method_response_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_method_response(
            &input.rest_api_id,
            &input.resource_id,
            &input.http_method,
            &input.status_code,
        ) {
            Ok(()) => wire::serialize_delete_method_response_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Integration handlers ----

    async fn handle_put_integration(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_integration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.r#type.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing required field 'type'");
        }
        let integration = Integration {
            integration_type: input.r#type,
            http_method: input.integration_http_method,
            uri: input.uri,
            credentials: input.credentials,
            request_parameters: input.request_parameters.unwrap_or_default(),
            request_templates: input.request_templates.unwrap_or_default(),
            passthrough_behavior: input.passthrough_behavior,
            content_handling: input.content_handling,
            timeout_in_millis: input.timeout_in_millis,
            cache_namespace: input.cache_namespace,
            cache_key_parameters: input.cache_key_parameters.unwrap_or_default(),
            connection_type: input.connection_type,
            connection_id: input.connection_id,
            integration_responses: HashMap::new(),
        };
        let mut s = state.write().await;
        match s.put_integration(
            &input.rest_api_id,
            &input.resource_id,
            &input.http_method,
            integration,
        ) {
            Ok(integ) => wire::serialize_put_integration_response(&integration_to_model(integ)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_integration(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_integration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_integration(&input.rest_api_id, &input.resource_id, &input.http_method) {
            Ok(integ) => wire::serialize_get_integration_response(&integration_to_model(integ)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_delete_integration(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_integration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_integration(&input.rest_api_id, &input.resource_id, &input.http_method) {
            Ok(()) => wire::serialize_delete_integration_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- IntegrationResponse handlers ----

    async fn handle_put_integration_response(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_integration_response_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let ir = IntegrationResponse {
            status_code: input.status_code.clone(),
            selection_pattern: input.selection_pattern,
            response_templates: input.response_templates.unwrap_or_default(),
            response_parameters: input.response_parameters.unwrap_or_default(),
            content_handling: input.content_handling,
        };
        let mut s = state.write().await;
        match s.put_integration_response(
            &input.rest_api_id,
            &input.resource_id,
            &input.http_method,
            &input.status_code,
            ir,
        ) {
            Ok(ir) => wire::serialize_put_integration_response_response(
                &integration_response_to_model(ir),
            ),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_integration_response(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
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
            &input.rest_api_id,
            &input.resource_id,
            &input.http_method,
            &input.status_code,
        ) {
            Ok(ir) => wire::serialize_get_integration_response_response(
                &integration_response_to_model(ir),
            ),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_delete_integration_response(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
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
            &input.rest_api_id,
            &input.resource_id,
            &input.http_method,
            &input.status_code,
        ) {
            Ok(()) => wire::serialize_delete_integration_response_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Model handlers ----

    async fn handle_create_model(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_model_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing required field 'name'");
        }
        let content_type_opt = if input.content_type.is_empty() {
            None
        } else {
            Some(input.content_type.as_str())
        };
        let mut s = state.write().await;
        match s.create_model(
            &input.rest_api_id,
            &input.name,
            input.description.as_deref(),
            input.schema.as_deref(),
            content_type_opt,
        ) {
            Ok(m) => wire::serialize_create_model_response(&model_to_model(m)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_model(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_model_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_model(&input.rest_api_id, &input.model_name) {
            Ok(m) => wire::serialize_get_model_response(&model_to_model(m)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_models(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_models_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_models(&input.rest_api_id) {
            Ok(models) => {
                let items: Vec<model::Model> = models.into_iter().map(model_to_model).collect();
                wire::serialize_get_models_response(&model::Models {
                    items: if items.is_empty() { None } else { Some(items) },
                })
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_delete_model(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_model_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_model(&input.rest_api_id, &input.model_name) {
            Ok(()) => wire::serialize_delete_model_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Authorizer handlers ----

    async fn handle_create_authorizer(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_authorizer_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing required field 'name'");
        }
        if input.r#type.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing required field 'type'");
        }
        let mut s = state.write().await;
        match s.create_authorizer(
            &input.rest_api_id,
            &input.name,
            &input.r#type,
            input.authorizer_uri.as_deref(),
            input.authorizer_credentials.as_deref(),
            input.identity_source.as_deref(),
            input.identity_validation_expression.as_deref(),
            input.authorizer_result_ttl_in_seconds,
            input.auth_type.as_deref(),
            input.provider_a_r_ns.unwrap_or_default(),
        ) {
            Ok(a) => wire::serialize_create_authorizer_response(&authorizer_to_model(a)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_authorizer(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_authorizer_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_authorizer(&input.rest_api_id, &input.authorizer_id) {
            Ok(a) => wire::serialize_get_authorizer_response(&authorizer_to_model(a)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_authorizers(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_authorizers_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_authorizers(&input.rest_api_id) {
            Ok(authorizers) => {
                let items: Vec<model::Authorizer> =
                    authorizers.into_iter().map(authorizer_to_model).collect();
                wire::serialize_get_authorizers_response(&model::Authorizers {
                    items: if items.is_empty() { None } else { Some(items) },
                })
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_delete_authorizer(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_authorizer_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_authorizer(&input.rest_api_id, &input.authorizer_id) {
            Ok(()) => wire::serialize_delete_authorizer_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- UpdateAuthorizer ----

    async fn handle_update_authorizer(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // The Smithy model for UpdateAuthorizer carries only `patchOperations`,
        // but this handler historically accepts a flat body; preserve that
        // behaviour by parsing the raw body alongside the wire deserializer.
        let _input = match wire::deserialize_update_authorizer_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let api_id = label_value(labels, "restApiId");
        let authorizer_id = label_value(labels, "authorizerId");
        let name = body.get("name").and_then(|v| v.as_str());
        let authorizer_type = body.get("type").and_then(|v| v.as_str());
        let authorizer_uri = body.get("authorizerUri").and_then(|v| v.as_str());
        let authorizer_credentials = body.get("authorizerCredentials").and_then(|v| v.as_str());
        let identity_source = body.get("identitySource").and_then(|v| v.as_str());
        let identity_validation_expression = body
            .get("identityValidationExpression")
            .and_then(|v| v.as_str());
        let authorizer_result_ttl_in_seconds = body
            .get("authorizerResultTtlInSeconds")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let auth_type = body.get("authType").and_then(|v| v.as_str());
        let provider_arns: Option<Vec<String>> = body
            .get("providerARNs")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            });
        let mut s = state.write().await;
        match s.update_authorizer(
            api_id,
            authorizer_id,
            name,
            authorizer_type,
            authorizer_uri,
            authorizer_credentials,
            identity_source,
            identity_validation_expression,
            authorizer_result_ttl_in_seconds,
            auth_type,
            provider_arns,
        ) {
            Ok(a) => wire::serialize_update_authorizer_response(&authorizer_to_model(a)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // PATCH /restapis/{restApiId} - UpdateRestApi

    async fn handle_update_rest_api(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // The Smithy model for UpdateRestApi carries only `patchOperations`, but
        // this handler historically accepts a flat body with `name`/`description`.
        let _input = match wire::deserialize_update_rest_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let api_id = label_value(labels, "restApiId");
        let name = body.get("name").and_then(|v| v.as_str());
        let description = body.get("description").and_then(|v| v.as_str());
        let mut s = state.write().await;
        match s.update_rest_api(api_id, name, description) {
            Ok(api) => wire::serialize_update_rest_api_response(&rest_api_to_model(api)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // POST /restapis?mode=import - ImportRestApi

    async fn handle_import_rest_api(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_import_rest_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // Extract the API name from the OpenAPI spec body (`info.title`), falling back
        // to "imported-api" if the body is absent or unparseable.
        let name = serde_json::from_slice::<serde_json::Value>(input.body.as_bytes())
            .ok()
            .and_then(|v| v.get("info")?.get("title")?.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "imported-api".to_string());
        let mut s = state.write().await;
        let api = s.create_rest_api(
            &name,
            None,
            None,
            None,
            HashMap::new(),
            None,
            None,
            None,
            vec![],
            None,
        );
        wire::serialize_import_rest_api_response(&rest_api_to_model(api))
    }

    // PUT /restapis/{restApiId} - PutRestApi

    // STUB[no-engine]: Merging/replacing an API definition from an imported OpenAPI spec
    //   requires a full spec parser; returns the existing API unchanged.
    async fn handle_put_rest_api(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_rest_api_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_rest_api(&input.rest_api_id) {
            Ok(api) => wire::serialize_put_rest_api_response(&rest_api_to_model(api)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- API Key handlers ----

    // POST /apikeys - CreateApiKey

    async fn handle_create_api_key(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_api_key_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.name.unwrap_or_else(|| "unnamed".to_string());
        let enabled = input.enabled.unwrap_or(true);
        // The historical handler accepts `stageKeys` as a list of strings, while the
        // Smithy model types it as a list of StageKey records; preserve flat-string
        // behaviour by re-parsing the body.
        let stage_keys: Vec<String> = if request.body.is_empty() {
            vec![]
        } else {
            serde_json::from_slice::<Value>(&request.body)
                .ok()
                .and_then(|v| v.get("stageKeys").cloned())
                .and_then(|v| v.as_array().cloned())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default()
        };
        let mut s = state.write().await;
        let key = s.create_api_key(
            &name,
            input.description.as_deref(),
            enabled,
            stage_keys,
            input.tags.unwrap_or_default(),
        );
        wire::serialize_create_api_key_response(&api_key_to_model(key))
    }

    // GET /apikeys/{apiKey} - GetApiKey

    async fn handle_get_api_key(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_api_key_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_api_key(&input.api_key) {
            Ok(k) => wire::serialize_get_api_key_response(&api_key_to_model(k)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // GET /apikeys - GetApiKeys

    async fn handle_get_api_keys(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_get_api_keys_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let items: Vec<model::ApiKey> = s
            .list_api_keys()
            .into_iter()
            .map(api_key_to_model)
            .collect();
        wire::serialize_get_api_keys_response(&model::ApiKeys {
            items: if items.is_empty() { None } else { Some(items) },
            ..Default::default()
        })
    }

    // DELETE /apikeys/{apiKey} - DeleteApiKey

    async fn handle_delete_api_key(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_api_key_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_api_key(&input.api_key) {
            Ok(()) => wire::serialize_delete_api_key_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // PATCH /apikeys/{apiKey} - UpdateApiKey

    async fn handle_update_api_key(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // UpdateApiKey carries `patchOperations` in Smithy but the historical
        // handler reads flat `name`/`description`/`enabled` body fields.
        let _input = match wire::deserialize_update_api_key_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let key_id = label_value(labels, "apiKey");
        let name = body.get("name").and_then(|v| v.as_str());
        let description = body.get("description").and_then(|v| v.as_str());
        let enabled = body.get("enabled").and_then(|v| v.as_bool());
        let mut s = state.write().await;
        match s.update_api_key(key_id, name, description, enabled) {
            Ok(k) => wire::serialize_update_api_key_response(&api_key_to_model(k)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Domain Name handlers ----

    async fn handle_create_domain_name(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_domain_name_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.domain_name.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing required field 'domainName'",
            );
        }
        // `distributionDomainName` is a response-only field in Smithy but the
        // historical handler accepts it from the request body.
        let distribution_domain_name = serde_json::from_slice::<Value>(&request.body)
            .ok()
            .and_then(|v| {
                v.get("distributionDomainName")
                    .and_then(|x| x.as_str())
                    .map(str::to_string)
            });
        let mut s = state.write().await;
        let dn = s.create_domain_name(
            &input.domain_name,
            input.certificate_name.as_deref(),
            distribution_domain_name.as_deref(),
            input.tags.unwrap_or_default(),
        );
        wire::serialize_create_domain_name_response(&domain_name_to_model(dn))
    }

    async fn handle_get_domain_name(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
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
            Ok(dn) => wire::serialize_get_domain_name_response(&domain_name_to_model(dn)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_domain_names(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_get_domain_names_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let items: Vec<model::DomainName> = s
            .list_domain_names()
            .into_iter()
            .map(domain_name_to_model)
            .collect();
        wire::serialize_get_domain_names_response(&model::DomainNames {
            items: if items.is_empty() { None } else { Some(items) },
        })
    }

    async fn handle_delete_domain_name(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
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
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Base Path Mapping handlers ----

    async fn handle_create_base_path_mapping(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_base_path_mapping_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.rest_api_id.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing required field 'restApiId'",
            );
        }
        let base_path = input.base_path.as_deref().unwrap_or("(none)");
        let mut s = state.write().await;
        match s.create_base_path_mapping(
            &input.domain_name,
            base_path,
            &input.rest_api_id,
            input.stage.as_deref(),
        ) {
            Ok(m) => {
                wire::serialize_create_base_path_mapping_response(&base_path_mapping_to_model(m))
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_base_path_mapping(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_base_path_mapping_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_base_path_mapping(&input.domain_name, &input.base_path) {
            Ok(m) => wire::serialize_get_base_path_mapping_response(&base_path_mapping_to_model(m)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_base_path_mappings(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_base_path_mappings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let items: Vec<model::BasePathMapping> = s
            .list_base_path_mappings(&input.domain_name)
            .into_iter()
            .map(base_path_mapping_to_model)
            .collect();
        wire::serialize_get_base_path_mappings_response(&model::BasePathMappings {
            items: if items.is_empty() { None } else { Some(items) },
        })
    }

    async fn handle_delete_base_path_mapping(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_base_path_mapping_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_base_path_mapping(&input.domain_name, &input.base_path) {
            Ok(()) => wire::serialize_delete_base_path_mapping_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_base_path_mapping(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // UpdateBasePathMapping carries only `patchOperations` in Smithy but the
        // historical handler reads flat `restApiId`/`stage`/`basePath` fields.
        let _input =
            match wire::deserialize_update_base_path_mapping_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let domain_name = label_value(labels, "domainName");
        let base_path = label_value(labels, "basePath");
        let rest_api_id = body.get("restApiId").and_then(|v| v.as_str());
        let stage = body.get("stage").and_then(|v| v.as_str());
        let new_base_path = body.get("basePath").and_then(|v| v.as_str());
        let mut s = state.write().await;
        match s.update_base_path_mapping(domain_name, base_path, rest_api_id, stage, new_base_path)
        {
            Ok(m) => {
                wire::serialize_update_base_path_mapping_response(&base_path_mapping_to_model(&m))
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Gateway Response handlers ----

    async fn handle_get_gateway_response(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_gateway_response_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_gateway_response(&input.rest_api_id, &input.response_type) {
            Ok(gr) => wire::serialize_get_gateway_response_response(&gateway_response_to_model(gr)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_gateway_responses(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_gateway_responses_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_gateway_responses(&input.rest_api_id) {
            Ok(responses) => {
                let items: Vec<model::GatewayResponse> = responses
                    .into_iter()
                    .map(gateway_response_to_model)
                    .collect();
                wire::serialize_get_gateway_responses_response(&model::GatewayResponses {
                    items: if items.is_empty() { None } else { Some(items) },
                })
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_put_gateway_response(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_gateway_response_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.put_gateway_response(
            &input.rest_api_id,
            &input.response_type,
            input.status_code.as_deref(),
            input.response_parameters.unwrap_or_default(),
            input.response_templates.unwrap_or_default(),
        ) {
            Ok(gr) => wire::serialize_put_gateway_response_response(&gateway_response_to_model(gr)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_delete_gateway_response(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_gateway_response_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_gateway_response(&input.rest_api_id, &input.response_type) {
            Ok(()) => wire::serialize_delete_gateway_response_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Request Validator handlers ----

    async fn handle_create_request_validator(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_request_validator_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = match input.name.as_deref() {
            Some(n) if !n.is_empty() => n,
            _ => {
                return rest_json_error(
                    400,
                    "BadRequestException",
                    "Missing required field 'name'",
                );
            }
        };
        let mut s = state.write().await;
        match s.create_request_validator(
            &input.rest_api_id,
            name,
            input.validate_request_body.unwrap_or(false),
            input.validate_request_parameters.unwrap_or(false),
        ) {
            Ok(v) => {
                wire::serialize_create_request_validator_response(&request_validator_to_model(v))
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_request_validator(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_request_validator_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_request_validator(&input.rest_api_id, &input.request_validator_id) {
            Ok(v) => wire::serialize_get_request_validator_response(&request_validator_to_model(v)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_request_validators(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_request_validators_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_request_validators(&input.rest_api_id) {
            Ok(validators) => {
                let items: Vec<model::RequestValidator> = validators
                    .into_iter()
                    .map(request_validator_to_model)
                    .collect();
                wire::serialize_get_request_validators_response(&model::RequestValidators {
                    items: if items.is_empty() { None } else { Some(items) },
                })
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_delete_request_validator(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_request_validator_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_request_validator(&input.rest_api_id, &input.request_validator_id) {
            Ok(()) => wire::serialize_delete_request_validator_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_request_validator(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // UpdateRequestValidator carries only `patchOperations` in Smithy but the
        // historical handler reads flat body fields.
        let _input =
            match wire::deserialize_update_request_validator_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let api_id = label_value(labels, "restApiId");
        let validator_id = label_value(labels, "requestValidatorId");
        let name = body.get("name").and_then(|v| v.as_str());
        let validate_request_body = body.get("validateRequestBody").and_then(|v| v.as_bool());
        let validate_request_parameters = body
            .get("validateRequestParameters")
            .and_then(|v| v.as_bool());
        let mut s = state.write().await;
        match s.update_request_validator(
            api_id,
            validator_id,
            name,
            validate_request_body,
            validate_request_parameters,
        ) {
            Ok(v) => {
                wire::serialize_update_request_validator_response(&request_validator_to_model(v))
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Usage Plan handlers ----

    async fn handle_create_usage_plan(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_usage_plan_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing required field 'name'");
        }
        let api_stages: Vec<UsagePlanApiStage> = input
            .api_stages
            .unwrap_or_default()
            .into_iter()
            .filter_map(|s| {
                let api_id = s.api_id?;
                let stage = s.stage?;
                Some(UsagePlanApiStage { api_id, stage })
            })
            .collect();
        let throttle = input.throttle.map(|t| ThrottleSettings {
            burst_limit: t.burst_limit,
            rate_limit: t.rate_limit,
        });
        let quota = input.quota.map(|q| QuotaSettings {
            limit: q.limit,
            offset: q.offset,
            period: q.period,
        });
        // `productCode` is not present on CreateUsagePlanRequest in Smithy but the
        // historical handler accepts it from the body.
        let product_code = serde_json::from_slice::<Value>(&request.body)
            .ok()
            .and_then(|v| {
                v.get("productCode")
                    .and_then(|x| x.as_str())
                    .map(str::to_string)
            });
        let mut s = state.write().await;
        let plan = s.create_usage_plan(
            &input.name,
            input.description.as_deref(),
            api_stages,
            throttle,
            quota,
            product_code.as_deref(),
            input.tags.unwrap_or_default(),
        );
        wire::serialize_create_usage_plan_response(&usage_plan_to_model(plan))
    }

    async fn handle_get_usage_plan(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_usage_plan_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_usage_plan(&input.usage_plan_id) {
            Ok(p) => wire::serialize_get_usage_plan_response(&usage_plan_to_model(p)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_usage_plans(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_get_usage_plans_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let items: Vec<model::UsagePlan> = s
            .list_usage_plans()
            .into_iter()
            .map(usage_plan_to_model)
            .collect();
        wire::serialize_get_usage_plans_response(&model::UsagePlans {
            items: if items.is_empty() { None } else { Some(items) },
        })
    }

    async fn handle_delete_usage_plan(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_usage_plan_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_usage_plan(&input.usage_plan_id) {
            Ok(()) => wire::serialize_delete_usage_plan_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_usage_plan(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // UpdateUsagePlan carries only `patchOperations` in Smithy but the
        // historical handler reads flat body fields.
        let _input = match wire::deserialize_update_usage_plan_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let plan_id = label_value(labels, "usagePlanId");
        let name = body.get("name").and_then(|v| v.as_str());
        let description = body.get("description").and_then(|v| v.as_str());
        let product_code = body.get("productCode").and_then(|v| v.as_str());
        let mut s = state.write().await;
        match s.update_usage_plan(plan_id, name, description, product_code) {
            Ok(p) => wire::serialize_update_usage_plan_response(&usage_plan_to_model(p)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_create_usage_plan_key(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_usage_plan_key_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing required field 'keyId'");
        }
        let key_type = if input.key_type.is_empty() {
            "API_KEY"
        } else {
            input.key_type.as_str()
        };
        let mut s = state.write().await;
        match s.create_usage_plan_key(&input.usage_plan_id, &input.key_id, key_type) {
            Ok(k) => wire::serialize_create_usage_plan_key_response(&usage_plan_key_to_model(k)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_usage_plan_key(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_usage_plan_key_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_usage_plan_key(&input.usage_plan_id, &input.key_id) {
            Ok(k) => wire::serialize_get_usage_plan_key_response(&usage_plan_key_to_model(k)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_usage_plan_keys(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_usage_plan_keys_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_usage_plan_keys(&input.usage_plan_id) {
            Ok(keys) => {
                let items: Vec<model::UsagePlanKey> =
                    keys.into_iter().map(usage_plan_key_to_model).collect();
                wire::serialize_get_usage_plan_keys_response(&model::UsagePlanKeys {
                    items: if items.is_empty() { None } else { Some(items) },
                })
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_delete_usage_plan_key(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_usage_plan_key_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_usage_plan_key(&input.usage_plan_id, &input.key_id) {
            Ok(()) => wire::serialize_delete_usage_plan_key_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- VPC Link handlers ----

    // POST /vpclinks - CreateVpcLink

    async fn handle_create_vpc_link(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_vpc_link_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing required field 'name'");
        }
        let mut s = state.write().await;
        let link = s.create_vpc_link(
            &input.name,
            input.description.as_deref(),
            input.target_arns,
            input.tags.unwrap_or_default(),
        );
        wire::serialize_create_vpc_link_response(&vpc_link_to_model(link))
    }

    // GET /vpclinks/{vpcLinkId} - GetVpcLink

    async fn handle_get_vpc_link(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
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
            Ok(l) => wire::serialize_get_vpc_link_response(&vpc_link_to_model(l)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // GET /vpclinks - GetVpcLinks

    async fn handle_get_vpc_links(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_get_vpc_links_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let items: Vec<model::VpcLink> = s
            .list_vpc_links()
            .into_iter()
            .map(vpc_link_to_model)
            .collect();
        wire::serialize_get_vpc_links_response(&model::VpcLinks {
            items: if items.is_empty() { None } else { Some(items) },
        })
    }

    // DELETE /vpclinks/{vpcLinkId} - DeleteVpcLink

    async fn handle_delete_vpc_link(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
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
            Ok(()) => wire::serialize_delete_vpc_link_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Account handlers ----

    async fn handle_get_account(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_get_account_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let account = s.get_account();
        wire::serialize_get_account_response(&account_to_model(account))
    }

    async fn handle_update_account(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // UpdateAccount carries only `patchOperations` in Smithy but the
        // historical handler reads flat body fields.
        let _input = match wire::deserialize_update_account_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let cloudwatch_role_arn = body.get("cloudwatchRoleArn").and_then(|v| v.as_str());
        let mut s = state.write().await;
        let account = s.update_account(cloudwatch_role_arn);
        wire::serialize_update_account_response(&account_to_model(account))
    }

    // ---- Client Certificate handlers ----

    async fn handle_generate_client_certificate(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_generate_client_certificate_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut s = state.write().await;
        let cert = s.generate_client_certificate(
            input.description.as_deref(),
            input.tags.unwrap_or_default(),
        );
        wire::serialize_generate_client_certificate_response(&client_certificate_to_model(cert))
    }

    async fn handle_get_client_certificate(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_client_certificate_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_client_certificate(&input.client_certificate_id) {
            Ok(cert) => {
                wire::serialize_get_client_certificate_response(&client_certificate_to_model(cert))
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_client_certificates(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_get_client_certificates_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let items: Vec<model::ClientCertificate> = s
            .list_client_certificates()
            .into_iter()
            .map(client_certificate_to_model)
            .collect();
        wire::serialize_get_client_certificates_response(&model::ClientCertificates {
            items: if items.is_empty() { None } else { Some(items) },
        })
    }

    async fn handle_delete_client_certificate(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_client_certificate_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut s = state.write().await;
        match s.delete_client_certificate(&input.client_certificate_id) {
            Ok(()) => wire::serialize_delete_client_certificate_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_client_certificate(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // UpdateClientCertificate carries only `patchOperations` in Smithy but the
        // historical handler reads flat body fields.
        let _input =
            match wire::deserialize_update_client_certificate_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let cert_id = label_value(labels, "clientCertificateId");
        let description = body.get("description").and_then(|v| v.as_str());
        let mut s = state.write().await;
        match s.update_client_certificate(cert_id, description) {
            Ok(cert) => wire::serialize_update_client_certificate_response(
                &client_certificate_to_model(cert),
            ),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Documentation Part handlers ----

    async fn handle_create_documentation_part(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // CreateDocumentationPart requires `location` to be present in the body
        // before deserialization can populate `location.type`. Verify presence
        // first to preserve the original BadRequestException messages.
        let raw_body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        if raw_body.get("location").is_none() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing required field 'location'",
            );
        }
        let input =
            match wire::deserialize_create_documentation_part_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.location.r#type.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing required field 'location.type'",
            );
        }
        if input.properties.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing required field 'properties'",
            );
        }
        let mut s = state.write().await;
        match s.create_documentation_part(
            &input.rest_api_id,
            &input.location.r#type,
            input.location.path.as_deref(),
            input.location.method.as_deref(),
            input.location.status_code.as_deref(),
            input.location.name.as_deref(),
            &input.properties,
        ) {
            Ok(part) => wire::serialize_create_documentation_part_response(
                &documentation_part_to_model(part),
            ),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_documentation_part(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_documentation_part_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_documentation_part(&input.rest_api_id, &input.documentation_part_id) {
            Ok(part) => {
                wire::serialize_get_documentation_part_response(&documentation_part_to_model(part))
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_documentation_parts(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_documentation_parts_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.list_documentation_parts(&input.rest_api_id) {
            Ok(parts) => {
                let items: Vec<model::DocumentationPart> =
                    parts.into_iter().map(documentation_part_to_model).collect();
                wire::serialize_get_documentation_parts_response(&model::DocumentationParts {
                    items: if items.is_empty() { None } else { Some(items) },
                })
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_delete_documentation_part(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_documentation_part_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut s = state.write().await;
        match s.delete_documentation_part(&input.rest_api_id, &input.documentation_part_id) {
            Ok(()) => wire::serialize_delete_documentation_part_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_documentation_part(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // UpdateDocumentationPart carries only `patchOperations` in Smithy but
        // the historical handler reads a flat `properties` field.
        let _input =
            match wire::deserialize_update_documentation_part_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let api_id = label_value(labels, "restApiId");
        let part_id = label_value(labels, "documentationPartId");
        let properties = body.get("properties").and_then(|v| v.as_str());
        let mut s = state.write().await;
        match s.update_documentation_part(api_id, part_id, properties) {
            Ok(part) => wire::serialize_update_documentation_part_response(
                &documentation_part_to_model(part),
            ),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_import_documentation_parts(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_import_documentation_parts_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let s = state.read().await;
        if !s.rest_apis.contains_key(&input.rest_api_id) {
            return rest_json_error(
                404,
                "NotFoundException",
                "Invalid REST API identifier specified",
            );
        }
        wire::serialize_import_documentation_parts_response(&model::DocumentationPartIds {
            ids: Some(vec![]),
            warnings: None,
        })
    }

    // ---- Documentation Version handlers ----

    async fn handle_create_documentation_version(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_documentation_version_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.documentation_version.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing required field 'documentationVersion'",
            );
        }
        let mut s = state.write().await;
        match s.create_documentation_version(
            &input.rest_api_id,
            &input.documentation_version,
            input.description.as_deref(),
        ) {
            Ok(dv) => wire::serialize_create_documentation_version_response(
                &documentation_version_to_model(dv),
            ),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_documentation_version(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_documentation_version_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let s = state.read().await;
        match s.get_documentation_version(&input.rest_api_id, &input.documentation_version) {
            Ok(dv) => wire::serialize_get_documentation_version_response(
                &documentation_version_to_model(dv),
            ),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_get_documentation_versions(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_documentation_versions_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let s = state.read().await;
        match s.list_documentation_versions(&input.rest_api_id) {
            Ok(versions) => {
                let items: Vec<model::DocumentationVersion> = versions
                    .into_iter()
                    .map(documentation_version_to_model)
                    .collect();
                wire::serialize_get_documentation_versions_response(&model::DocumentationVersions {
                    items: if items.is_empty() { None } else { Some(items) },
                })
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_delete_documentation_version(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_documentation_version_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut s = state.write().await;
        match s.delete_documentation_version(&input.rest_api_id, &input.documentation_version) {
            Ok(()) => wire::serialize_delete_documentation_version_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_documentation_version(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // UpdateDocumentationVersion carries only `patchOperations` in Smithy
        // but the historical handler reads a flat `description` field.
        let _input =
            match wire::deserialize_update_documentation_version_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let api_id = label_value(labels, "restApiId");
        let version = label_value(labels, "documentationVersion");
        let description = body.get("description").and_then(|v| v.as_str());
        let mut s = state.write().await;
        match s.update_documentation_version(api_id, version, description) {
            Ok(dv) => wire::serialize_update_documentation_version_response(
                &documentation_version_to_model(dv),
            ),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Stage cache handlers ----

    async fn handle_flush_stage_authorizers_cache(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_flush_stage_authorizers_cache_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let s = state.read().await;
        match s.get_stage(&input.rest_api_id, &input.stage_name) {
            Ok(_) => wire::serialize_flush_stage_authorizers_cache_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- SDK handlers ----

    async fn handle_get_sdk_types(&self) -> MockResponse {
        let items = vec![
            model::SdkType {
                id: Some("android".to_string()),
                friendly_name: Some("Android SDK".to_string()),
                description: Some("Generate an Android SDK for your API".to_string()),
                configuration_properties: None,
            },
            model::SdkType {
                id: Some("javascript".to_string()),
                friendly_name: Some("JavaScript SDK".to_string()),
                description: Some("Generate a JavaScript SDK for your API".to_string()),
                configuration_properties: None,
            },
            model::SdkType {
                id: Some("objectivec".to_string()),
                friendly_name: Some("Objective-C SDK".to_string()),
                description: Some("Generate an Objective-C SDK for your API".to_string()),
                configuration_properties: None,
            },
            model::SdkType {
                id: Some("ruby".to_string()),
                friendly_name: Some("Ruby SDK".to_string()),
                description: Some("Generate a Ruby gem for your API".to_string()),
                configuration_properties: None,
            },
            model::SdkType {
                id: Some("swift".to_string()),
                friendly_name: Some("Swift SDK".to_string()),
                description: Some("Generate a Swift SDK for your API".to_string()),
                configuration_properties: None,
            },
        ];
        wire::serialize_get_sdk_types_response(&model::SdkTypes { items: Some(items) })
    }

    async fn handle_get_sdk_type(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_sdk_type_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let known = ["android", "javascript", "objectivec", "ruby", "swift"];
        if !known.contains(&input.id.as_str()) {
            return rest_json_error(
                404,
                "NotFoundException",
                "Invalid SDK type identifier specified",
            );
        }
        wire::serialize_get_sdk_type_response(&model::SdkType {
            id: Some(input.id.clone()),
            friendly_name: Some(format!("{} SDK", input.id)),
            description: Some(format!("SDK for {}", input.id)),
            configuration_properties: None,
        })
    }

    // STUB[no-engine]: Generating a real SDK bundle requires compiling client code for the
    //   given SDK type; always returns a placeholder archive stub.
    async fn handle_get_sdk(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_sdk_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_stage(&input.rest_api_id, &input.stage_name) {
            Ok(_) => wire::serialize_get_sdk_response(&model::SdkResponse {
                body: Some(format!(
                    "// SDK stub for api={} stage={} type={}",
                    input.rest_api_id, input.stage_name, input.sdk_type
                )),
                content_type: Some("application/octet-stream".to_string()),
                content_disposition: Some(format!(
                    "attachment; filename=\"{}-{}-{}.zip\"",
                    input.rest_api_id, input.stage_name, input.sdk_type
                )),
            }),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Export handler ----

    // STUB[no-engine]: Generating a real OpenAPI/Swagger export requires traversing all
    //   resources, methods, integrations and producing a spec document; always returns a
    //   minimal placeholder JSON body.
    async fn handle_get_export(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_export_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_stage(&input.rest_api_id, &input.stage_name) {
            Ok(_) => wire::serialize_get_export_response(&model::ExportResponse {
                body: Some(format!(
                    "{{\"openapi\":\"3.0.0\",\"info\":{{\"title\":\"{}\",\"version\":\"{}\"}}}}",
                    input.rest_api_id, input.stage_name
                )),
                content_type: Some("application/json".to_string()),
                content_disposition: Some(format!(
                    "attachment; filename=\"{}-{}.{}.json\"",
                    input.rest_api_id, input.stage_name, input.export_type
                )),
            }),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Model template handler ----

    // STUB[no-engine]: Expanding a model schema into a template requires a full JSON Schema
    //   resolver; always returns an empty object template.
    async fn handle_get_model_template(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_model_template_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_model(&input.rest_api_id, &input.model_name) {
            Ok(_) => wire::serialize_get_model_template_response(&model::Template {
                value: Some("{}".to_string()),
            }),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Tag handlers ----

    async fn handle_get_tags(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
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
        wire::serialize_get_tags_response(&model::Tags {
            tags: if tags.is_empty() { None } else { Some(tags) },
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        s.tag_resource(&input.resource_arn, input.tags);
        wire::serialize_tag_resource_response()
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        query_string: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // AWS SDKs serialise the `tagKeys` httpQuery as repeated params
        // (`?tagKeys=a&tagKeys=b`), but the wire deserializer expects a
        // comma-separated single value. Re-parse from the raw query string so
        // multi-key untag requests are not silently truncated.
        let mut tag_keys: Vec<String> = query_string
            .split('&')
            .filter_map(|part| part.strip_prefix("tagKeys="))
            .map(percent_decode)
            .collect();
        if tag_keys.is_empty() {
            tag_keys = input.tag_keys;
        }
        let tag_key_refs: Vec<&str> = tag_keys.iter().map(String::as_str).collect();
        let mut s = state.write().await;
        s.untag_resource(&input.resource_arn, &tag_key_refs);
        wire::serialize_untag_resource_response()
    }

    // ---- Usage handler ----

    async fn handle_get_usage(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_usage_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_usage_plan(&input.usage_plan_id) {
            Ok(_) => {
                let start_date = if input.start_date.is_empty() {
                    "2024-01-01".to_string()
                } else {
                    input.start_date.clone()
                };
                let end_date = if input.end_date.is_empty() {
                    "2024-01-31".to_string()
                } else {
                    input.end_date.clone()
                };
                wire::serialize_get_usage_response(&model::Usage {
                    usage_plan_id: Some(input.usage_plan_id.clone()),
                    start_date: Some(start_date),
                    end_date: Some(end_date),
                    items: None,
                })
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- Import API Keys handler ----

    async fn handle_import_api_keys(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_import_api_keys_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let ids: Vec<String> = s
            .list_api_keys()
            .into_iter()
            .map(|k| k.id.clone())
            .collect();
        wire::serialize_import_api_keys_response(&model::ApiKeyIds {
            ids: if ids.is_empty() { None } else { Some(ids) },
            warnings: None,
        })
    }

    // ---- Domain Name Access Association handlers ----

    async fn handle_create_domain_name_access_association(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_domain_name_access_association_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.domain_name_arn.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing required field 'domainNameArn'",
            );
        }
        if input.access_association_source.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing required field 'accessAssociationSource'",
            );
        }
        if input.access_association_source_type.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing required field 'accessAssociationSourceType'",
            );
        }
        let mut s = state.write().await;
        let assoc = s.create_domain_name_access_association(
            &input.domain_name_arn,
            &input.access_association_source,
            &input.access_association_source_type,
            input.tags.unwrap_or_default(),
            default_account_id(),
            region,
        );
        wire::serialize_create_domain_name_access_association_response(
            &domain_name_access_association_to_model(assoc),
        )
    }

    async fn handle_get_domain_name_access_associations(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_get_domain_name_access_associations_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let items: Vec<model::DomainNameAccessAssociation> = s
            .list_domain_name_access_associations()
            .into_iter()
            .map(domain_name_access_association_to_model)
            .collect();
        wire::serialize_get_domain_name_access_associations_response(
            &model::DomainNameAccessAssociations {
                items: if items.is_empty() { None } else { Some(items) },
            },
        )
    }

    async fn handle_delete_domain_name_access_association(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_domain_name_access_association_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_domain_name_access_association(&input.domain_name_access_association_arn) {
            Ok(()) => wire::serialize_delete_domain_name_access_association_response(),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_reject_domain_name_access_association(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_reject_domain_name_access_association_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        wire::serialize_reject_domain_name_access_association_response()
    }

    // ---- Update operation handlers ----

    async fn handle_update_deployment(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_update_deployment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let api_id = label_value(labels, "restApiId");
        let deployment_id = label_value(labels, "deploymentId");
        let description = body.get("description").and_then(|v| v.as_str());
        let mut s = state.write().await;
        match s.update_deployment(api_id, deployment_id, description) {
            Ok(d) => wire::serialize_update_deployment_response(&deployment_to_model(d)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_domain_name(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_update_domain_name_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let domain_name = label_value(labels, "domainName");
        let certificate_name = body.get("certificateName").and_then(|v| v.as_str());
        let mut s = state.write().await;
        match s.update_domain_name(domain_name, certificate_name) {
            Ok(dn) => wire::serialize_update_domain_name_response(&domain_name_to_model(dn)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_gateway_response(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_update_gateway_response_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let api_id = label_value(labels, "restApiId");
        let response_type = label_value(labels, "responseType");
        let status_code = body.get("statusCode").and_then(|v| v.as_str());
        let response_parameters = parse_string_map(body.get("responseParameters"));
        let response_templates = parse_string_map(body.get("responseTemplates"));
        let mut s = state.write().await;
        match s.update_gateway_response(
            api_id,
            response_type,
            status_code,
            response_parameters,
            response_templates,
        ) {
            Ok(gr) => {
                wire::serialize_update_gateway_response_response(&gateway_response_to_model(gr))
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_model(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_update_model_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let api_id = label_value(labels, "restApiId");
        let model_name = label_value(labels, "modelName");
        let description = body.get("description").and_then(|v| v.as_str());
        let schema = body.get("schema").and_then(|v| v.as_str());
        let mut s = state.write().await;
        match s.update_model(api_id, model_name, description, schema) {
            Ok(m) => wire::serialize_update_model_response(&model_to_model(m)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_vpc_link(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_update_vpc_link_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let link_id = label_value(labels, "vpcLinkId");
        let name = body.get("name").and_then(|v| v.as_str());
        let description = body.get("description").and_then(|v| v.as_str());
        let mut s = state.write().await;
        match s.update_vpc_link(link_id, name, description) {
            Ok(l) => wire::serialize_update_vpc_link_response(&vpc_link_to_model(l)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_usage(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_usage_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_usage_plan(&input.usage_plan_id) {
            Ok(_) => wire::serialize_update_usage_response(&model::Usage {
                usage_plan_id: Some(input.usage_plan_id.clone()),
                start_date: Some("2024-01-01".to_string()),
                end_date: Some("2024-01-31".to_string()),
                items: Some(std::collections::HashMap::from([(
                    input.key_id.clone(),
                    vec![vec![0i64, 0i64]],
                )])),
            }),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_resource(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_update_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let api_id = label_value(labels, "restApiId");
        let resource_id = label_value(labels, "resourceId");
        let path_part = body.get("pathPart").and_then(|v| v.as_str());
        let mut s = state.write().await;
        match s.update_resource(api_id, resource_id, path_part) {
            Ok(r) => wire::serialize_update_resource_response(&resource_to_model(r)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_method(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_update_method_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let api_id = label_value(labels, "restApiId");
        let resource_id = label_value(labels, "resourceId");
        let http_method = label_value(labels, "httpMethod");
        let authorization_type = body.get("authorizationType").and_then(|v| v.as_str());
        let authorizer_id = body.get("authorizerId").and_then(|v| v.as_str());
        let api_key_required = body.get("apiKeyRequired").and_then(|v| v.as_bool());
        let operation_name = body.get("operationName").and_then(|v| v.as_str());
        let mut s = state.write().await;
        match s.update_method(
            api_id,
            resource_id,
            http_method,
            authorization_type,
            authorizer_id,
            api_key_required,
            operation_name,
        ) {
            Ok(m) => wire::serialize_update_method_response(&method_to_model(m)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_method_response(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_method_response_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.update_method_response(
            &input.rest_api_id,
            &input.resource_id,
            &input.http_method,
            &input.status_code,
        ) {
            Ok(mr) => {
                wire::serialize_update_method_response_response(&method_response_to_model(mr))
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_integration(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_update_integration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let api_id = label_value(labels, "restApiId");
        let resource_id = label_value(labels, "resourceId");
        let http_method = label_value(labels, "httpMethod");
        let uri = body.get("uri").and_then(|v| v.as_str());
        let passthrough_behavior = body.get("passthroughBehavior").and_then(|v| v.as_str());
        let timeout_in_millis = body
            .get("timeoutInMillis")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let mut s = state.write().await;
        match s.update_integration(
            api_id,
            resource_id,
            http_method,
            uri,
            passthrough_behavior,
            timeout_in_millis,
        ) {
            Ok(i) => wire::serialize_update_integration_response(&integration_to_model(i)),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_update_integration_response(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input =
            match wire::deserialize_update_integration_response_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let body = match parse_body_value(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let api_id = label_value(labels, "restApiId");
        let resource_id = label_value(labels, "resourceId");
        let http_method = label_value(labels, "httpMethod");
        let status_code = label_value(labels, "statusCode");
        let selection_pattern = body.get("selectionPattern").and_then(|v| v.as_str());
        let content_handling = body.get("contentHandling").and_then(|v| v.as_str());
        let mut s = state.write().await;
        match s.update_integration_response(
            api_id,
            resource_id,
            http_method,
            status_code,
            selection_pattern,
            content_handling,
        ) {
            Ok(ir) => wire::serialize_update_integration_response_response(
                &integration_response_to_model(ir),
            ),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    // ---- TestInvoke handlers ----

    async fn handle_test_invoke_authorizer(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_test_invoke_authorizer_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_authorizer(&input.rest_api_id, &input.authorizer_id) {
            Ok(_) => wire::serialize_test_invoke_authorizer_response(
                &model::TestInvokeAuthorizerResponse {
                    client_status: Some(200),
                    latency: Some(0),
                    log: Some(String::new()),
                    principal_id: Some("user".to_string()),
                    policy: Some("{\"Version\":\"2012-10-17\",\"Statement\":[]}".to_string()),
                    claims: None,
                    authorization: None,
                },
            ),
            Err(e) => api_gateway_error_response(&e),
        }
    }

    async fn handle_test_invoke_method(
        &self,
        state: &Arc<RwLock<ApiGatewayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_test_invoke_method_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_method(&input.rest_api_id, &input.resource_id, &input.http_method) {
            Ok(_) => {
                wire::serialize_test_invoke_method_response(&model::TestInvokeMethodResponse {
                    status: Some(200),
                    body: Some(String::new()),
                    headers: None,
                    multi_value_headers: None,
                    latency: Some(0),
                    log: Some(String::new()),
                })
            }
            Err(e) => api_gateway_error_response(&e),
        }
    }
}

fn rest_api_to_model(api: &crate::types::RestApi) -> model::RestApi {
    let endpoint_configuration =
        api.endpoint_configuration
            .as_ref()
            .map(|ec| model::EndpointConfiguration {
                types: if ec.types.is_empty() {
                    None
                } else {
                    Some(ec.types.clone())
                },
                vpc_endpoint_ids: if ec.vpc_endpoint_ids.is_empty() {
                    None
                } else {
                    Some(ec.vpc_endpoint_ids.clone())
                },
                ..Default::default()
            });
    model::RestApi {
        id: Some(api.id.clone()),
        name: Some(api.name.clone()),
        description: api.description.clone(),
        version: api.version.clone(),
        created_date: Some(api.created_date),
        endpoint_configuration,
        tags: if api.tags.is_empty() {
            None
        } else {
            Some(api.tags.clone())
        },
        disable_execute_api_endpoint: api.disable_execute_api_endpoint,
        policy: api.policy.clone(),
        api_key_source: api.api_key_source.clone(),
        binary_media_types: if api.binary_media_types.is_empty() {
            None
        } else {
            Some(api.binary_media_types.clone())
        },
        minimum_compression_size: api.minimum_compression_size,
        root_resource_id: Some(api.root_resource_id.clone()),
        ..Default::default()
    }
}

fn resource_to_model(r: &crate::types::Resource) -> model::Resource {
    model::Resource {
        id: Some(r.id.clone()),
        parent_id: r.parent_id.clone(),
        path_part: r.path_part.clone(),
        path: Some(r.path.clone()),
        ..Default::default()
    }
}

fn stage_to_model(s: &crate::types::Stage) -> model::Stage {
    model::Stage {
        stage_name: Some(s.stage_name.clone()),
        deployment_id: Some(s.deployment_id.clone()),
        description: s.description.clone(),
        created_date: Some(s.created_date),
        last_updated_date: Some(s.last_updated_date),
        tracing_enabled: s.tracing_enabled,
        tags: if s.tags.is_empty() {
            None
        } else {
            Some(s.tags.clone())
        },
        variables: if s.variables.is_empty() {
            None
        } else {
            Some(s.variables.clone())
        },
        cache_cluster_enabled: s.cache_cluster_enabled,
        cache_cluster_size: s.cache_cluster_size.clone(),
        documentation_version: s.documentation_version.clone(),
        ..Default::default()
    }
}

fn deployment_to_model(d: &crate::types::Deployment) -> model::Deployment {
    model::Deployment {
        id: Some(d.id.clone()),
        description: d.description.clone(),
        created_date: Some(d.created_date),
        ..Default::default()
    }
}

fn method_to_model(m: &crate::types::Method) -> model::Method {
    let method_responses: HashMap<String, model::MethodResponse> = m
        .method_responses
        .iter()
        .map(|(k, v)| (k.clone(), method_response_to_model(v)))
        .collect();
    model::Method {
        http_method: Some(m.http_method.clone()),
        authorization_type: Some(m.authorization_type.clone()),
        authorizer_id: m.authorizer_id.clone(),
        api_key_required: m.api_key_required,
        operation_name: m.operation_name.clone(),
        request_models: if m.request_models.is_empty() {
            None
        } else {
            Some(m.request_models.clone())
        },
        request_parameters: if m.request_parameters.is_empty() {
            None
        } else {
            Some(m.request_parameters.clone())
        },
        request_validator_id: m.request_validator_id.clone(),
        method_responses: if method_responses.is_empty() {
            None
        } else {
            Some(method_responses)
        },
        method_integration: m.integration.as_ref().map(integration_to_model),
        ..Default::default()
    }
}

fn method_response_to_model(mr: &crate::types::MethodResponse) -> model::MethodResponse {
    model::MethodResponse {
        status_code: Some(mr.status_code.clone()),
        response_models: if mr.response_models.is_empty() {
            None
        } else {
            Some(mr.response_models.clone())
        },
        response_parameters: if mr.response_parameters.is_empty() {
            None
        } else {
            Some(mr.response_parameters.clone())
        },
    }
}

fn integration_to_model(i: &crate::types::Integration) -> model::Integration {
    let integration_responses: HashMap<String, model::IntegrationResponse> = i
        .integration_responses
        .iter()
        .map(|(k, v)| (k.clone(), integration_response_to_model(v)))
        .collect();
    model::Integration {
        r#type: Some(i.integration_type.clone()),
        http_method: i.http_method.clone(),
        uri: i.uri.clone(),
        credentials: i.credentials.clone(),
        request_parameters: if i.request_parameters.is_empty() {
            None
        } else {
            Some(i.request_parameters.clone())
        },
        request_templates: if i.request_templates.is_empty() {
            None
        } else {
            Some(i.request_templates.clone())
        },
        passthrough_behavior: i.passthrough_behavior.clone(),
        content_handling: i.content_handling.clone(),
        timeout_in_millis: i.timeout_in_millis,
        cache_namespace: i.cache_namespace.clone(),
        cache_key_parameters: if i.cache_key_parameters.is_empty() {
            None
        } else {
            Some(i.cache_key_parameters.clone())
        },
        connection_type: i.connection_type.clone(),
        connection_id: i.connection_id.clone(),
        integration_responses: if integration_responses.is_empty() {
            None
        } else {
            Some(integration_responses)
        },
        ..Default::default()
    }
}

fn integration_response_to_model(
    ir: &crate::types::IntegrationResponse,
) -> model::IntegrationResponse {
    model::IntegrationResponse {
        status_code: Some(ir.status_code.clone()),
        selection_pattern: ir.selection_pattern.clone(),
        response_templates: if ir.response_templates.is_empty() {
            None
        } else {
            Some(ir.response_templates.clone())
        },
        response_parameters: if ir.response_parameters.is_empty() {
            None
        } else {
            Some(ir.response_parameters.clone())
        },
        content_handling: ir.content_handling.clone(),
    }
}

fn model_to_model(m: &crate::types::ApiGatewayModel) -> model::Model {
    model::Model {
        id: Some(m.id.clone()),
        name: Some(m.name.clone()),
        description: m.description.clone(),
        schema: m.schema.clone(),
        content_type: m.content_type.clone(),
    }
}

fn authorizer_to_model(a: &crate::types::Authorizer) -> model::Authorizer {
    model::Authorizer {
        id: Some(a.id.clone()),
        name: Some(a.name.clone()),
        r#type: Some(a.authorizer_type.clone()),
        authorizer_uri: a.authorizer_uri.clone(),
        authorizer_credentials: a.authorizer_credentials.clone(),
        identity_source: a.identity_source.clone(),
        identity_validation_expression: a.identity_validation_expression.clone(),
        authorizer_result_ttl_in_seconds: a.authorizer_result_ttl_in_seconds,
        auth_type: a.auth_type.clone(),
        provider_a_r_ns: if a.provider_arns.is_empty() {
            None
        } else {
            Some(a.provider_arns.clone())
        },
    }
}

fn api_key_to_model(k: &crate::types::ApiKey) -> model::ApiKey {
    model::ApiKey {
        id: Some(k.id.clone()),
        name: Some(k.name.clone()),
        value: Some(k.value.clone()),
        description: k.description.clone(),
        enabled: Some(k.enabled),
        created_date: Some(k.created_date),
        stage_keys: if k.stage_keys.is_empty() {
            None
        } else {
            Some(k.stage_keys.clone())
        },
        tags: if k.tags.is_empty() {
            None
        } else {
            Some(k.tags.clone())
        },
        ..Default::default()
    }
}

fn domain_name_to_model(dn: &crate::types::DomainName) -> model::DomainName {
    model::DomainName {
        domain_name: Some(dn.domain_name.clone()),
        certificate_name: dn.certificate_name.clone(),
        distribution_domain_name: dn.distribution_domain_name.clone(),
        tags: if dn.tags.is_empty() {
            None
        } else {
            Some(dn.tags.clone())
        },
        ..Default::default()
    }
}

fn base_path_mapping_to_model(m: &crate::types::BasePathMapping) -> model::BasePathMapping {
    model::BasePathMapping {
        base_path: Some(m.base_path.clone()),
        rest_api_id: Some(m.rest_api_id.clone()),
        stage: m.stage.clone(),
    }
}

fn gateway_response_to_model(gr: &GatewayResponse) -> model::GatewayResponse {
    model::GatewayResponse {
        response_type: Some(gr.response_type.clone()),
        status_code: gr.status_code.clone(),
        response_parameters: if gr.response_parameters.is_empty() {
            None
        } else {
            Some(gr.response_parameters.clone())
        },
        response_templates: if gr.response_templates.is_empty() {
            None
        } else {
            Some(gr.response_templates.clone())
        },
        ..Default::default()
    }
}

fn request_validator_to_model(v: &crate::types::RequestValidator) -> model::RequestValidator {
    model::RequestValidator {
        id: Some(v.id.clone()),
        name: Some(v.name.clone()),
        validate_request_body: Some(v.validate_request_body),
        validate_request_parameters: Some(v.validate_request_parameters),
    }
}

fn usage_plan_to_model(p: &crate::types::UsagePlan) -> model::UsagePlan {
    let api_stages: Vec<model::ApiStage> = p
        .api_stages
        .iter()
        .map(|s| model::ApiStage {
            api_id: Some(s.api_id.clone()),
            stage: Some(s.stage.clone()),
            ..Default::default()
        })
        .collect();
    model::UsagePlan {
        id: Some(p.id.clone()),
        name: Some(p.name.clone()),
        description: p.description.clone(),
        api_stages: if api_stages.is_empty() {
            None
        } else {
            Some(api_stages)
        },
        throttle: p.throttle.as_ref().map(|t| model::ThrottleSettings {
            burst_limit: t.burst_limit,
            rate_limit: t.rate_limit,
        }),
        quota: p.quota.as_ref().map(|q| model::QuotaSettings {
            limit: q.limit,
            offset: q.offset,
            period: q.period.clone(),
        }),
        product_code: p.product_code.clone(),
        tags: if p.tags.is_empty() {
            None
        } else {
            Some(p.tags.clone())
        },
    }
}

fn usage_plan_key_to_model(k: &crate::types::UsagePlanKey) -> model::UsagePlanKey {
    model::UsagePlanKey {
        id: Some(k.id.clone()),
        r#type: Some(k.key_type.clone()),
        name: k.name.clone(),
        value: k.value.clone(),
    }
}

fn vpc_link_to_model(l: &crate::types::VpcLink) -> model::VpcLink {
    model::VpcLink {
        id: Some(l.id.clone()),
        name: Some(l.name.clone()),
        description: l.description.clone(),
        target_arns: if l.target_arns.is_empty() {
            None
        } else {
            Some(l.target_arns.clone())
        },
        status: Some(l.status.clone()),
        status_message: l.status_message.clone(),
        tags: if l.tags.is_empty() {
            None
        } else {
            Some(l.tags.clone())
        },
    }
}

fn client_certificate_to_model(cert: &crate::types::ClientCertificate) -> model::ClientCertificate {
    model::ClientCertificate {
        client_certificate_id: Some(cert.client_certificate_id.clone()),
        description: cert.description.clone(),
        pem_encoded_certificate: Some(cert.pem_encoded_certificate.clone()),
        created_date: Some(cert.created_date),
        expiration_date: Some(cert.expiration_date),
        tags: if cert.tags.is_empty() {
            None
        } else {
            Some(cert.tags.clone())
        },
    }
}

fn documentation_part_to_model(part: &crate::types::DocumentationPart) -> model::DocumentationPart {
    model::DocumentationPart {
        id: Some(part.id.clone()),
        location: Some(model::DocumentationPartLocation {
            r#type: part.location_type.clone(),
            path: part.location_path.clone(),
            method: part.location_method.clone(),
            status_code: part.location_status_code.clone(),
            name: part.location_name.clone(),
        }),
        properties: Some(part.properties.clone()),
    }
}

fn documentation_version_to_model(
    dv: &crate::types::DocumentationVersion,
) -> model::DocumentationVersion {
    model::DocumentationVersion {
        version: Some(dv.version.clone()),
        created_date: Some(dv.created_date),
        description: dv.description.clone(),
    }
}

fn domain_name_access_association_to_model(
    assoc: &crate::types::DomainNameAccessAssociation,
) -> model::DomainNameAccessAssociation {
    model::DomainNameAccessAssociation {
        domain_name_access_association_arn: Some(assoc.arn.clone()),
        domain_name_arn: Some(assoc.domain_name_arn.clone()),
        access_association_source: Some(assoc.access_association_source.clone()),
        access_association_source_type: Some(assoc.access_association_source_type.clone()),
        tags: if assoc.tags.is_empty() {
            None
        } else {
            Some(assoc.tags.clone())
        },
    }
}

fn account_to_model(a: &crate::types::Account) -> model::Account {
    model::Account {
        cloudwatch_role_arn: a.cloudwatch_role_arn.clone(),
        throttle_settings: a
            .throttle_settings
            .as_ref()
            .map(|t| model::ThrottleSettings {
                burst_limit: t.burst_limit,
                rate_limit: t.rate_limit,
            }),
        features: if a.features.is_empty() {
            None
        } else {
            Some(a.features.clone())
        },
        api_key_version: a.api_key_version.clone(),
    }
}

fn label_value<'a>(labels: &'a [(&'a str, &'a str)], key: &str) -> &'a str {
    labels
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
        .unwrap_or("")
}

fn patch_operations_to_value(ops: &Option<Vec<model::PatchOperation>>) -> Value {
    match ops {
        Some(list) => Value::Array(
            list.iter()
                .map(|op| {
                    serde_json::to_value(op).unwrap_or_else(|_| Value::Object(Default::default()))
                })
                .collect(),
        ),
        None => Value::Array(vec![]),
    }
}

fn parse_string_map(v: Option<&Value>) -> HashMap<String, String> {
    v.and_then(|v| v.as_object())
        .map(|m| {
            m.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect()
        })
        .unwrap_or_default()
}

fn api_gateway_error_response(err: &ApiGatewayError) -> MockResponse {
    let (status, error_type) = match err {
        ApiGatewayError::RestApiNotFound => (404, "NotFoundException"),
        ApiGatewayError::ResourceNotFound => (404, "NotFoundException"),
        ApiGatewayError::StageNotFound => (404, "NotFoundException"),
        ApiGatewayError::DeploymentNotFound => (404, "NotFoundException"),
        ApiGatewayError::DeploymentNotFoundLower => (404, "NotFoundException"),
        ApiGatewayError::MethodNotFound => (404, "NotFoundException"),
        ApiGatewayError::MethodResponseNotFound => (404, "NotFoundException"),
        ApiGatewayError::IntegrationNotFound => (404, "NotFoundException"),
        ApiGatewayError::IntegrationResponseNotFound => (404, "NotFoundException"),
        ApiGatewayError::ModelNotFound => (404, "NotFoundException"),
        ApiGatewayError::AuthorizerNotFound => (404, "NotFoundException"),
        ApiGatewayError::ApiKeyNotFound => (404, "NotFoundException"),
        ApiGatewayError::DomainNameNotFound => (404, "NotFoundException"),
        ApiGatewayError::BasePathMappingNotFound => (404, "NotFoundException"),
        ApiGatewayError::GatewayResponseNotFound => (404, "NotFoundException"),
        ApiGatewayError::RequestValidatorNotFound => (404, "NotFoundException"),
        ApiGatewayError::UsagePlanNotFound => (404, "NotFoundException"),
        ApiGatewayError::UsagePlanKeyNotFound => (404, "NotFoundException"),
        ApiGatewayError::VpcLinkNotFound => (404, "NotFoundException"),
        ApiGatewayError::ClientCertificateNotFound => (404, "NotFoundException"),
        ApiGatewayError::DocumentationPartNotFound => (404, "NotFoundException"),
        ApiGatewayError::DocumentationVersionNotFound => (404, "NotFoundException"),
        ApiGatewayError::DomainNameAccessAssociationNotFound => (404, "NotFoundException"),
    };
    let body = serde_json::json!({
        "message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = serde_json::json!({
        "message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}

fn extract_path(uri: &str) -> &str {
    let without_scheme = if let Some(pos) = uri.find("://") {
        &uri[pos + 3..]
    } else {
        uri
    };
    let without_host = if let Some(pos) = without_scheme.find('/') {
        &without_scheme[pos..]
    } else {
        "/"
    };
    // strip query string
    if let Some(pos) = without_host.find('?') {
        &without_host[..pos]
    } else {
        without_host
    }
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut result = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let (Some(h), Some(l)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2]))
        {
            result.push((h << 4) | l);
            i += 3;
            continue;
        }
        result.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&result).into_owned()
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
