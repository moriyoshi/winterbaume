use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{FreeTierError, FreeTierState};
use crate::views::FreeTierStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct FreeTierService {
    pub(crate) state: Arc<BackendState<FreeTierState>>,
    pub(crate) notifier: StateChangeNotifier<FreeTierStateView>,
}

impl FreeTierService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for FreeTierService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for FreeTierService {
    fn service_name(&self) -> &str {
        "freetier"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://freetier\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<FreeTierState>>;

const MUTATING_ACTIONS: &[&str] = &["UpgradeAccountPlan"];

impl FreeTierService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

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
            "GetAccountActivity" => self.handle_get_activity(&state, &body).await,
            "GetAccountPlanState" => self.handle_get_plan_state(&state, account_id).await,
            "GetFreeTierUsage" => self.handle_get_usage(&body).await,
            "ListAccountActivities" => self.handle_list_activities(&state, &body).await,
            "UpgradeAccountPlan" => self.handle_upgrade_plan(&state, account_id, &body).await,
            other => aws_json_error(
                400,
                "ResourceNotFoundException",
                &format!("Unknown action: {other}"),
            ),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && MUTATING_ACTIONS.contains(&action.as_str()) {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_get_activity(&self, state: &SharedState, body: &Value) -> MockResponse {
        let id = match require_str(body, "activityId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        match state.get_activity(&id) {
            Ok(a) => {
                wire::serialize_get_account_activity_response(&wire::GetAccountActivityResponse {
                    activity_id: Some(a.activity_id.clone()),
                    completed_at: a.completed_at.clone(),
                    description: Some(a.description.clone()),
                    estimated_time_to_complete_in_minutes: a.estimated_time_to_complete_in_minutes,
                    expires_at: a.expires_at.clone(),
                    instructions_url: a.instructions_url.clone(),
                    reward: Some(wire::ActivityReward {
                        credit: Some(wire::MonetaryAmount {
                            amount: Some(a.reward_amount),
                            unit: Some(a.reward_unit.clone()),
                        }),
                    }),
                    started_at: a.started_at.clone(),
                    status: Some(a.status.clone()),
                    title: Some(a.title.clone()),
                })
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_get_plan_state(&self, state: &SharedState, account_id: &str) -> MockResponse {
        let state = state.read().await;
        wire::serialize_get_account_plan_state_response(&wire::GetAccountPlanStateResponse {
            account_id: Some(account_id.to_string()),
            account_plan_expiration_date: state.plan_expiration_date.clone(),
            account_plan_remaining_credits: Some(wire::MonetaryAmount {
                amount: Some(state.remaining_credits_amount),
                unit: Some(state.remaining_credits_unit.clone()),
            }),
            account_plan_status: Some(state.plan_status.clone()),
            account_plan_type: Some(state.plan_type.clone()),
        })
    }

    // STUB[no-telemetry]: GetFreeTierUsage returns real infrastructure usage data that the mock
    //   has no access to; always returns an empty list.
    async fn handle_get_usage(&self, _body: &Value) -> MockResponse {
        wire::serialize_get_free_tier_usage_response(&wire::GetFreeTierUsageResponse {
            free_tier_usages: Some(vec![]),
            next_token: None,
        })
    }

    async fn handle_list_activities(&self, state: &SharedState, body: &Value) -> MockResponse {
        let statuses: Option<Vec<String>> = body
            .get("filterActivityStatuses")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            });
        let state = state.read().await;
        let summaries: Vec<wire::ActivitySummary> = state
            .list_activities(statuses.as_deref())
            .into_iter()
            .map(|a| wire::ActivitySummary {
                activity_id: Some(a.activity_id.clone()),
                reward: Some(wire::ActivityReward {
                    credit: Some(wire::MonetaryAmount {
                        amount: Some(a.reward_amount),
                        unit: Some(a.reward_unit.clone()),
                    }),
                }),
                status: Some(a.status.clone()),
                title: Some(a.title.clone()),
            })
            .collect();
        wire::serialize_list_account_activities_response(&wire::ListAccountActivitiesResponse {
            activities: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_upgrade_plan(
        &self,
        state: &SharedState,
        account_id: &str,
        body: &Value,
    ) -> MockResponse {
        let plan_type = match require_str(body, "accountPlanType") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        state.upgrade_plan(plan_type.clone());
        wire::serialize_upgrade_account_plan_response(&wire::UpgradeAccountPlanResponse {
            account_id: Some(account_id.to_string()),
            account_plan_status: Some(state.plan_status.clone()),
            account_plan_type: Some(plan_type),
        })
    }
}

fn require_str(body: &Value, field: &str) -> Result<String, MockResponse> {
    body.get(field)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .ok_or_else(|| aws_json_error(400, "ValidationException", &format!("{field} is required")))
}

fn err_response(err: &FreeTierError) -> MockResponse {
    let (status, error_type) = match err {
        FreeTierError::ActivityNotFound { .. } => (404, "ResourceNotFoundException"),
        FreeTierError::Validation { .. } => (400, "ValidationException"),
    };
    aws_json_error(status, error_type, &err.to_string())
}

fn aws_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "Message": message});
    let mut resp = MockResponse::json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
