use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, percent_decode, rest_json_error,
};

use crate::state::{TrustedAdvisorError, TrustedAdvisorState};
use crate::views::TrustedAdvisorStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct TrustedAdvisorService {
    pub(crate) state: Arc<BackendState<TrustedAdvisorState>>,
    pub(crate) notifier: StateChangeNotifier<TrustedAdvisorStateView>,
}

impl TrustedAdvisorService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for TrustedAdvisorService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for TrustedAdvisorService {
    fn service_name(&self) -> &str {
        "trustedadvisor"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://trustedadvisor\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<TrustedAdvisorState>>;

impl TrustedAdvisorService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str().to_uppercase();

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
            }
        };

        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

        let response = match (method.as_str(), segs.as_slice()) {
            ("PUT", ["v1", "batch-update-recommendation-resource-exclusion"]) => {
                self.handle_batch_update_exclusion(&state, &body).await
            }
            ("GET", ["v1", "checks"]) => self.handle_list_checks().await,
            ("GET", ["v1", "recommendations"]) => self.handle_list_recommendations(&state).await,
            ("GET", ["v1", "recommendations", id]) => {
                self.handle_get_recommendation(&state, id).await
            }
            ("GET", ["v1", "recommendations", _id, "resources"]) => {
                self.handle_list_recommendation_resources().await
            }
            ("PUT", ["v1", "recommendations", id, "lifecycle"]) => {
                self.handle_update_recommendation_lifecycle(&state, id, &body)
                    .await
            }
            ("GET", ["v1", "organization-recommendations"]) => {
                self.handle_list_organization_recommendations(&state).await
            }
            ("GET", ["v1", "organization-recommendations", id]) => {
                self.handle_get_organization_recommendation(&state, id)
                    .await
            }
            ("GET", ["v1", "organization-recommendations", _id, "accounts"]) => {
                self.handle_list_organization_recommendation_accounts()
                    .await
            }
            ("GET", ["v1", "organization-recommendations", _id, "resources"]) => {
                self.handle_list_organization_recommendation_resources()
                    .await
            }
            ("PUT", ["v1", "organization-recommendations", id, "lifecycle"]) => {
                self.handle_update_organization_recommendation_lifecycle(&state, id, &body)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && method.as_str() != "GET" {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // STUB[no-telemetry]: Trusted Advisor checks are an AWS-managed catalogue; the mock
    //   has no access to the real check corpus, so always returns an empty list.
    async fn handle_list_checks(&self) -> MockResponse {
        wire::serialize_list_checks_response(&wire::ListChecksResponse {
            check_summaries: Some(vec![]),
            next_token: None,
        })
    }

    async fn handle_list_recommendations(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let summaries: Vec<wire::RecommendationSummary> = state
            .list_recommendations()
            .into_iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();
        wire::serialize_list_recommendations_response(&wire::ListRecommendationsResponse {
            recommendation_summaries: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_get_recommendation(&self, state: &SharedState, id: &str) -> MockResponse {
        let state = state.read().await;
        match state.get_recommendation(id) {
            Some(v) => match serde_json::from_value::<wire::Recommendation>(v.clone()) {
                Ok(rec) => {
                    wire::serialize_get_recommendation_response(&wire::GetRecommendationResponse {
                        recommendation: Some(rec),
                    })
                }
                Err(_) => rest_json_error(500, "InternalServerException", "decode failure"),
            },
            None => err_response(&TrustedAdvisorError::NotFound { id: id.to_string() }),
        }
    }

    // STUB[no-state]: Per-recommendation resource details are not tracked in mock state;
    //   always returns an empty list.
    async fn handle_list_recommendation_resources(&self) -> MockResponse {
        wire::serialize_list_recommendation_resources_response(
            &wire::ListRecommendationResourcesResponse {
                next_token: None,
                recommendation_resource_summaries: Some(vec![]),
            },
        )
    }

    async fn handle_update_recommendation_lifecycle(
        &self,
        state: &SharedState,
        id: &str,
        body: &Value,
    ) -> MockResponse {
        let stage = body
            .get("lifecycleStage")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if stage.is_empty() {
            return rest_json_error(400, "ValidationException", "lifecycleStage is required");
        }
        let mut state = state.write().await;
        match state.update_recommendation_lifecycle(id, stage) {
            Ok(()) => wire::serialize_update_recommendation_lifecycle_response(),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_organization_recommendations(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let summaries: Vec<wire::OrganizationRecommendationSummary> = state
            .list_organization_recommendations()
            .into_iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();
        wire::serialize_list_organization_recommendations_response(
            &wire::ListOrganizationRecommendationsResponse {
                organization_recommendation_summaries: Some(summaries),
                next_token: None,
            },
        )
    }

    async fn handle_get_organization_recommendation(
        &self,
        state: &SharedState,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_organization_recommendation(id) {
            Some(v) => {
                match serde_json::from_value::<wire::OrganizationRecommendation>(v.clone()) {
                    Ok(rec) => wire::serialize_get_organization_recommendation_response(
                        &wire::GetOrganizationRecommendationResponse {
                            organization_recommendation: Some(rec),
                        },
                    ),
                    Err(_) => rest_json_error(500, "InternalServerException", "decode failure"),
                }
            }
            None => err_response(&TrustedAdvisorError::NotFound { id: id.to_string() }),
        }
    }

    // STUB[org-integration]: Per-org account lifecycle summaries require real AWS Organizations
    //   cross-account state; always returns an empty list.
    async fn handle_list_organization_recommendation_accounts(&self) -> MockResponse {
        wire::serialize_list_organization_recommendation_accounts_response(
            &wire::ListOrganizationRecommendationAccountsResponse {
                account_recommendation_lifecycle_summaries: Some(vec![]),
                next_token: None,
            },
        )
    }

    // STUB[no-state]: Per-recommendation resource details for org recommendations are not
    //   tracked in mock state; always returns an empty list.
    async fn handle_list_organization_recommendation_resources(&self) -> MockResponse {
        wire::serialize_list_organization_recommendation_resources_response(
            &wire::ListOrganizationRecommendationResourcesResponse {
                next_token: None,
                organization_recommendation_resource_summaries: Some(vec![]),
            },
        )
    }

    async fn handle_update_organization_recommendation_lifecycle(
        &self,
        state: &SharedState,
        id: &str,
        body: &Value,
    ) -> MockResponse {
        let stage = body
            .get("lifecycleStage")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if stage.is_empty() {
            return rest_json_error(400, "ValidationException", "lifecycleStage is required");
        }
        let mut state = state.write().await;
        match state.update_organization_recommendation_lifecycle(id, stage) {
            Ok(()) => wire::serialize_update_organization_recommendation_lifecycle_response(),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_batch_update_exclusion(
        &self,
        state: &SharedState,
        body: &Value,
    ) -> MockResponse {
        let exclusions = body
            .get("recommendationResourceExclusions")
            .and_then(|v| v.as_array())
            .map(|a| a.to_vec())
            .unwrap_or_default();
        let mut state = state.write().await;
        for ex in exclusions {
            let arn = ex.get("arn").and_then(|v| v.as_str()).unwrap_or("");
            let excluded = ex
                .get("isExcluded")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if !arn.is_empty() {
                state.set_resource_exclusion(arn, excluded);
            }
        }
        wire::serialize_batch_update_recommendation_resource_exclusion_response(
            &wire::BatchUpdateRecommendationResourceExclusionResponse {
                batch_update_recommendation_resource_exclusion_errors: Some(vec![]),
            },
        )
    }
}

fn err_response(err: &TrustedAdvisorError) -> MockResponse {
    let (status, error_type) = match err {
        TrustedAdvisorError::NotFound { .. } => (404, "ResourceNotFoundException"),
        TrustedAdvisorError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
