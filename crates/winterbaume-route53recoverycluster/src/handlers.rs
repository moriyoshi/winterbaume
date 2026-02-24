use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{RecoveryClusterError, RecoveryClusterState};
use crate::views::RecoveryClusterStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct RecoveryClusterService {
    pub(crate) state: Arc<BackendState<RecoveryClusterState>>,
    pub(crate) notifier: StateChangeNotifier<RecoveryClusterStateView>,
}

impl RecoveryClusterService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for RecoveryClusterService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for RecoveryClusterService {
    fn service_name(&self) -> &str {
        "route53-recovery-cluster"
    }

    fn url_patterns(&self) -> Vec<&str> {
        // Real recovery-cluster endpoints are per-cluster (e.g.
        // `host-deadbeef.route53-recovery-cluster.us-east-1.amazonaws.com`).
        // The mock matches the canonical regional hostname plus cluster-specific
        // hosts via the wildcard prefix.
        vec![r"https?://([^/]+\.)?route53-recovery-cluster\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<RecoveryClusterState>>;

const MUTATING_ACTIONS: &[&str] = &["UpdateRoutingControlState", "UpdateRoutingControlStates"];

impl RecoveryClusterService {
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
            "GetRoutingControlState" => self.handle_get_state(&state, account_id, &body).await,
            "ListRoutingControls" => self.handle_list(&state, account_id, &body).await,
            "UpdateRoutingControlState" => {
                self.handle_update_state(&state, account_id, &body).await
            }
            "UpdateRoutingControlStates" => {
                self.handle_update_states(&state, account_id, &body).await
            }
            other => aws_json_error(
                400,
                "ValidationException",
                &format!("Unknown action: {other}"),
            ),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && MUTATING_ACTIONS.contains(&action.as_str()) {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_get_state(
        &self,
        state: &SharedState,
        account_id: &str,
        body: &Value,
    ) -> MockResponse {
        let arn = match require_str(body, "RoutingControlArn") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        state.ensure_seeded(account_id);
        match state.get(&arn) {
            Ok(r) => wire::serialize_get_routing_control_state_response(
                &wire::GetRoutingControlStateResponse {
                    routing_control_arn: Some(r.arn.clone()),
                    routing_control_name: Some(r.name.clone()),
                    routing_control_state: Some(r.state.clone()),
                },
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list(
        &self,
        state: &SharedState,
        account_id: &str,
        body: &Value,
    ) -> MockResponse {
        let control_panel_arn = body
            .get("ControlPanelArn")
            .and_then(|v| v.as_str())
            .map(String::from);
        let mut state = state.write().await;
        state.ensure_seeded(account_id);
        let controls: Vec<wire::RoutingControl> = state
            .list(control_panel_arn.as_deref())
            .into_iter()
            .map(rc_to_wire)
            .collect();
        wire::serialize_list_routing_controls_response(&wire::ListRoutingControlsResponse {
            next_token: None,
            routing_controls: Some(controls),
        })
    }

    async fn handle_update_state(
        &self,
        state: &SharedState,
        account_id: &str,
        body: &Value,
    ) -> MockResponse {
        let arn = match require_str(body, "RoutingControlArn") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let new_state = match require_str(body, "RoutingControlState") {
            Ok(v) => v,
            Err(r) => return r,
        };
        if new_state != "On" && new_state != "Off" {
            return aws_json_error(
                400,
                "ValidationException",
                "RoutingControlState must be On or Off",
            );
        }
        let mut state = state.write().await;
        state.ensure_seeded(account_id);
        match state.set_state(&arn, new_state) {
            Ok(_) => wire::serialize_update_routing_control_state_response(
                &wire::UpdateRoutingControlStateResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_update_states(
        &self,
        state: &SharedState,
        account_id: &str,
        body: &Value,
    ) -> MockResponse {
        let entries = body
            .get("UpdateRoutingControlStateEntries")
            .and_then(|v| v.as_array())
            .map(|a| a.to_vec())
            .unwrap_or_default();
        if entries.is_empty() {
            return aws_json_error(
                400,
                "ValidationException",
                "UpdateRoutingControlStateEntries is required",
            );
        }
        let mut state = state.write().await;
        state.ensure_seeded(account_id);
        for entry in entries {
            let arn = entry
                .get("RoutingControlArn")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let new_state = entry
                .get("RoutingControlState")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            if arn.is_empty() || (new_state != "On" && new_state != "Off") {
                return aws_json_error(
                    400,
                    "ValidationException",
                    "Each entry needs RoutingControlArn and a RoutingControlState of On|Off",
                );
            }
            if let Err(e) = state.set_state(&arn, new_state) {
                return err_response(&e);
            }
        }
        wire::serialize_update_routing_control_states_response(
            &wire::UpdateRoutingControlStatesResponse {},
        )
    }
}

fn rc_to_wire(r: &crate::state::RoutingControlRecord) -> wire::RoutingControl {
    wire::RoutingControl {
        control_panel_arn: Some(r.control_panel_arn.clone()),
        control_panel_name: Some(r.control_panel_name.clone()),
        owner: Some(r.owner.clone()),
        routing_control_arn: Some(r.arn.clone()),
        routing_control_name: Some(r.name.clone()),
        routing_control_state: Some(r.state.clone()),
    }
}

fn require_str(body: &Value, field: &str) -> Result<String, MockResponse> {
    body.get(field)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .ok_or_else(|| aws_json_error(400, "ValidationException", &format!("{field} is required")))
}

fn err_response(err: &RecoveryClusterError) -> MockResponse {
    let (status, error_type) = match err {
        RecoveryClusterError::NotFound { .. } => (404, "ResourceNotFoundException"),
        RecoveryClusterError::Validation { .. } => (400, "ValidationException"),
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
