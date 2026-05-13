use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    json_error_response,
};

use crate::state::BcmRecommendedActionsState;
use crate::views::BcmRecommendedActionsStateView;
use crate::wire;

pub struct BcmRecommendedActionsService {
    pub(crate) state: Arc<BackendState<BcmRecommendedActionsState>>,
    pub(crate) notifier: StateChangeNotifier<BcmRecommendedActionsStateView>,
}

impl BcmRecommendedActionsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for BcmRecommendedActionsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for BcmRecommendedActionsService {
    fn service_name(&self) -> &str {
        "bcm-recommended-actions"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://bcm-recommended-actions\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl BcmRecommendedActionsService {
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
            None => return json_error_response(400, "MissingAction", "Missing X-Amz-Target"),
        };

        let _body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => {
                    return json_error_response(400, "SerializationException", "Invalid JSON body");
                }
            }
        };

        let state = self.state.get(account_id, &region);

        match action.as_str() {
            "ListRecommendedActions" => self.handle_list_recommended_actions(&state).await,
            other => json_error_response(
                400,
                "UnknownOperationException",
                &format!("Unknown action: {other}"),
            ),
        }
    }

    async fn handle_list_recommended_actions(
        &self,
        state: &Arc<tokio::sync::RwLock<BcmRecommendedActionsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let actions: Vec<wire::RecommendedAction> = state
            .recommended_actions
            .iter()
            .map(|a| wire::RecommendedAction {
                account_id: a.account_id.clone(),
                context: if a.context.is_empty() {
                    None
                } else {
                    Some(a.context.clone())
                },
                feature: a.feature.clone(),
                id: a.id.clone(),
                last_updated_time_stamp: a.last_updated_timestamp.clone(),
                next_steps: if a.next_steps.is_empty() {
                    None
                } else {
                    Some(a.next_steps.clone())
                },
                severity: a.severity.clone(),
                r#type: a.action_type.clone(),
            })
            .collect();
        wire::serialize_list_recommended_actions_response(&wire::ListRecommendedActionsResponse {
            next_token: None,
            recommended_actions: Some(actions),
        })
    }
}
