use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, rest_json_error,
};

use crate::state::PersonalizeEventsState;
use crate::views::PersonalizeEventsStateView;
use crate::wire;

pub struct PersonalizeEventsService {
    pub(crate) state: Arc<BackendState<PersonalizeEventsState>>,
    pub(crate) notifier: StateChangeNotifier<PersonalizeEventsStateView>,
}

impl PersonalizeEventsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for PersonalizeEventsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for PersonalizeEventsService {
    fn service_name(&self) -> &str {
        "personalize-events"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://personalize-events\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<PersonalizeEventsState>>;

impl PersonalizeEventsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let path = path.trim_start_matches('/').trim_end_matches('/');

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
            }
        };

        let response = match path {
            "events" => self.handle_put_events(&state, &body).await,
            "action-interactions" => self.handle_put_action_interactions(&state, &body).await,
            "actions" => self.handle_put_actions(&state, &body).await,
            "items" => self.handle_put_items(&state, &body).await,
            "users" => self.handle_put_users(&state, &body).await,
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_put_events(&self, state: &SharedState, body: &Value) -> MockResponse {
        let tracking_id = match require_str(body, "trackingId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let events = body
            .get("eventList")
            .and_then(|v| v.as_array())
            .map(|a| a.to_vec())
            .unwrap_or_default();
        let mut state = state.write().await;
        state.put_events(&tracking_id, events);
        wire::serialize_put_events_response()
    }

    async fn handle_put_action_interactions(
        &self,
        state: &SharedState,
        body: &Value,
    ) -> MockResponse {
        let tracking_id = match require_str(body, "trackingId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let interactions = body
            .get("actionInteractions")
            .and_then(|v| v.as_array())
            .map(|a| a.to_vec())
            .unwrap_or_default();
        let mut state = state.write().await;
        state.put_action_interactions(&tracking_id, interactions);
        wire::serialize_put_action_interactions_response()
    }

    async fn handle_put_actions(&self, state: &SharedState, body: &Value) -> MockResponse {
        let dataset_arn = match require_str(body, "datasetArn") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let actions = body
            .get("actions")
            .and_then(|v| v.as_array())
            .map(|a| a.to_vec())
            .unwrap_or_default();
        let mut state = state.write().await;
        state.put_actions(&dataset_arn, actions);
        wire::serialize_put_actions_response()
    }

    async fn handle_put_items(&self, state: &SharedState, body: &Value) -> MockResponse {
        let dataset_arn = match require_str(body, "datasetArn") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let items = body
            .get("items")
            .and_then(|v| v.as_array())
            .map(|a| a.to_vec())
            .unwrap_or_default();
        let mut state = state.write().await;
        state.put_items(&dataset_arn, items);
        wire::serialize_put_items_response()
    }

    async fn handle_put_users(&self, state: &SharedState, body: &Value) -> MockResponse {
        let dataset_arn = match require_str(body, "datasetArn") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let users = body
            .get("users")
            .and_then(|v| v.as_array())
            .map(|a| a.to_vec())
            .unwrap_or_default();
        let mut state = state.write().await;
        state.put_users(&dataset_arn, users);
        wire::serialize_put_users_response()
    }
}

fn require_str(body: &Value, field: &str) -> Result<String, MockResponse> {
    body.get(field)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .ok_or_else(|| rest_json_error(400, "ValidationException", &format!("{field} is required")))
}
