use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{CostOptimizationHubError, CostOptimizationHubState};
use crate::views::CostOptimizationHubStateView;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct CostOptimizationHubService {
    pub(crate) state: Arc<BackendState<CostOptimizationHubState>>,
    pub(crate) notifier: StateChangeNotifier<CostOptimizationHubStateView>,
}

impl CostOptimizationHubService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CostOptimizationHubService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CostOptimizationHubService {
    fn service_name(&self) -> &str {
        "cost-optimization-hub"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://cost-optimization-hub\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<CostOptimizationHubState>>;

const MUTATING_ACTIONS: &[&str] = &["UpdateEnrollmentStatus", "UpdatePreferences"];

impl CostOptimizationHubService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());
        let action = match action {
            Some(a) => a,
            None => return aws_json_error(400, "MissingAction", "Missing X-Amz-Target"),
        };

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => {
                    return aws_json_error(400, "SerializationException", "Invalid JSON body");
                }
            }
        };

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "GetPreferences" => self.handle_get_preferences(&state).await,
            "UpdatePreferences" => self.handle_update_preferences(&state, &body).await,
            "GetRecommendation" => self.handle_get_recommendation(&state, &body).await,
            "ListEnrollmentStatuses" => self.handle_list_enrollment(&state, &body).await,
            "UpdateEnrollmentStatus" => {
                self.handle_update_enrollment(&state, &body, account_id)
                    .await
            }
            "ListEfficiencyMetrics" => self.handle_list_efficiency().await,
            "ListRecommendations" => self.handle_list_recommendations(&state).await,
            "ListRecommendationSummaries" => self.handle_list_summaries().await,
            other => aws_json_error(
                400,
                "UnknownOperationException",
                &format!("Unknown action: {other}"),
            ),
        };

        if response.status / 100 == 2 && MUTATING_ACTIONS.contains(&action.as_str()) {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_get_preferences(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let prefs = state.preferences.clone();
        let savings_estimation = prefs
            .get("savingsEstimationMode")
            .cloned()
            .unwrap_or(json!("BeforeDiscounts"));
        let member_pref = prefs
            .get("memberAccountDiscountVisibility")
            .cloned()
            .unwrap_or(json!("All"));
        let preferred_commitment = prefs.get("preferredCommitment").cloned();
        let mut response = json!({
            "savingsEstimationMode": savings_estimation,
            "memberAccountDiscountVisibility": member_pref,
        });
        if let (Some(c), Value::Object(map)) = (preferred_commitment, &mut response) {
            map.insert("preferredCommitment".into(), c);
        }
        aws_json_response(response)
    }

    async fn handle_update_preferences(&self, state: &SharedState, body: &Value) -> MockResponse {
        let mut state = state.write().await;
        state.preferences = body.clone();
        let savings = body
            .get("savingsEstimationMode")
            .cloned()
            .unwrap_or(json!("BeforeDiscounts"));
        let member = body
            .get("memberAccountDiscountVisibility")
            .cloned()
            .unwrap_or(json!("All"));
        let preferred_commitment = body.get("preferredCommitment").cloned();
        let mut response = json!({
            "savingsEstimationMode": savings,
            "memberAccountDiscountVisibility": member,
        });
        if let (Some(c), Value::Object(map)) = (preferred_commitment, &mut response) {
            map.insert("preferredCommitment".into(), c);
        }
        aws_json_response(response)
    }

    async fn handle_get_recommendation(&self, state: &SharedState, body: &Value) -> MockResponse {
        let id = match body.get("recommendationId").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => {
                return aws_json_error(400, "ValidationException", "recommendationId is required");
            }
        };
        let state = state.read().await;
        match state.get_recommendation(&id) {
            Ok(r) => aws_json_response(r),
            Err(e @ CostOptimizationHubError::RecommendationNotFound(_)) => {
                aws_json_error(404, "ResourceNotFoundException", &e.to_string())
            }
        }
    }

    async fn handle_list_enrollment(&self, state: &SharedState, body: &Value) -> MockResponse {
        let include_org = body
            .get("includeOrganizationInfo")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let state = state.read().await;
        let items = state.list_enrollment_statuses(include_org);
        aws_json_response(json!({
            "items": items,
            "includeMemberAccounts": include_org,
            "nextToken": Value::Null,
        }))
    }

    async fn handle_update_enrollment(
        &self,
        state: &SharedState,
        body: &Value,
        account_id: &str,
    ) -> MockResponse {
        let status = body
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("Inactive")
            .to_string();
        let target_account = body
            .get("accountId")
            .and_then(|v| v.as_str())
            .unwrap_or(account_id)
            .to_string();
        let mut state = state.write().await;
        state.enrollment_statuses.insert(
            target_account.clone(),
            json!({
                "accountId": target_account,
                "status": status,
                "createdTimestamp": chrono_iso_now(),
                "lastUpdatedTimestamp": chrono_iso_now(),
            }),
        );
        aws_json_response(json!({"status": status}))
    }

    // STUB[no-telemetry]: Efficiency metrics are driven by real infrastructure telemetry; no mock data exists.
    async fn handle_list_efficiency(&self) -> MockResponse {
        aws_json_response(json!({
            "efficiencyMetricsByGroup": [],
            "nextToken": Value::Null,
        }))
    }

    async fn handle_list_recommendations(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let items = state.list_recommendations();
        aws_json_response(json!({
            "items": items,
            "nextToken": Value::Null,
        }))
    }

    // STUB[no-telemetry]: Recommendation summaries are aggregated from real infrastructure telemetry; no mock data exists.
    async fn handle_list_summaries(&self) -> MockResponse {
        aws_json_response(json!({
            "items": [],
            "groupBy": "ResourceType",
            "estimatedTotalDedupedSavings": 0,
            "currencyCode": "USD",
            "nextToken": Value::Null,
        }))
    }
}

fn chrono_iso_now() -> f64 {
    // awsJson1_0 timestamp default: epoch seconds as f64.
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);
    secs
}

fn aws_json_response(body: Value) -> MockResponse {
    MockResponse::json(200, body.to_string())
}

fn aws_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "Message": message});
    let mut resp = MockResponse::json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
