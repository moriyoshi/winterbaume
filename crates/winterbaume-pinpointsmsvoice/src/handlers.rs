use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, percent_decode, rest_json_error,
};

use crate::state::{PinpointSmsVoiceError, PinpointSmsVoiceState};
use crate::views::PinpointSmsVoiceStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct PinpointSmsVoiceService {
    pub(crate) state: Arc<BackendState<PinpointSmsVoiceState>>,
    pub(crate) notifier: StateChangeNotifier<PinpointSmsVoiceStateView>,
}

impl PinpointSmsVoiceService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for PinpointSmsVoiceService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for PinpointSmsVoiceService {
    fn service_name(&self) -> &str {
        "sms-voice.pinpoint"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://sms-voice\.pinpoint\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<PinpointSmsVoiceState>>;

impl PinpointSmsVoiceService {
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
            ("POST", ["v1", "sms-voice", "configuration-sets"]) => {
                self.handle_create_configuration_set(&state, &body).await
            }
            ("GET", ["v1", "sms-voice", "configuration-sets"]) => {
                self.handle_list_configuration_sets(&state).await
            }
            ("DELETE", ["v1", "sms-voice", "configuration-sets", name]) => {
                self.handle_delete_configuration_set(&state, name).await
            }
            (
                "POST",
                [
                    "v1",
                    "sms-voice",
                    "configuration-sets",
                    name,
                    "event-destinations",
                ],
            ) => {
                self.handle_create_event_destination(&state, name, &body)
                    .await
            }
            (
                "GET",
                [
                    "v1",
                    "sms-voice",
                    "configuration-sets",
                    name,
                    "event-destinations",
                ],
            ) => self.handle_get_event_destinations(&state, name).await,
            (
                "PUT",
                [
                    "v1",
                    "sms-voice",
                    "configuration-sets",
                    name,
                    "event-destinations",
                    dest,
                ],
            ) => {
                self.handle_update_event_destination(&state, name, dest, &body)
                    .await
            }
            (
                "DELETE",
                [
                    "v1",
                    "sms-voice",
                    "configuration-sets",
                    name,
                    "event-destinations",
                    dest,
                ],
            ) => {
                self.handle_delete_event_destination(&state, name, dest)
                    .await
            }
            ("POST", ["v1", "sms-voice", "voice", "message"]) => {
                self.handle_send_voice_message(&state, &body).await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && method.as_str() != "GET" {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_configuration_set(
        &self,
        state: &SharedState,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ConfigurationSetName") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        match state.create_configuration_set(&name) {
            Ok(_) => wire::serialize_create_configuration_set_response(
                &wire::CreateConfigurationSetResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_delete_configuration_set(
        &self,
        state: &SharedState,
        name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_configuration_set(name) {
            Ok(()) => wire::serialize_delete_configuration_set_response(
                &wire::DeleteConfigurationSetResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_configuration_sets(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let names: Vec<String> = state
            .list_configuration_sets()
            .into_iter()
            .map(String::from)
            .collect();
        wire::serialize_list_configuration_sets_response(&wire::ListConfigurationSetsResponse {
            configuration_sets: if names.is_empty() { None } else { Some(names) },
            next_token: None,
        })
    }

    async fn handle_create_event_destination(
        &self,
        state: &SharedState,
        cs_name: &str,
        body: &Value,
    ) -> MockResponse {
        let dest_name = body
            .get("EventDestinationName")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(String::from)
            .unwrap_or_default();
        if dest_name.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "EventDestinationName is required",
            );
        }
        let destination = body.get("EventDestination").cloned().unwrap_or(json!({}));
        let mut state = state.write().await;
        match state.put_event_destination(cs_name, &dest_name, destination) {
            Ok(()) => wire::serialize_create_configuration_set_event_destination_response(
                &wire::CreateConfigurationSetEventDestinationResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_update_event_destination(
        &self,
        state: &SharedState,
        cs_name: &str,
        dest_name: &str,
        body: &Value,
    ) -> MockResponse {
        let destination = body.get("EventDestination").cloned().unwrap_or(json!({}));
        let mut state = state.write().await;
        // Update is idempotent insert; ensure the configuration set exists, but do not
        // require the destination to pre-exist (matches AWS PUT semantics).
        match state.put_event_destination(cs_name, dest_name, destination) {
            Ok(()) => wire::serialize_update_configuration_set_event_destination_response(
                &wire::UpdateConfigurationSetEventDestinationResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_delete_event_destination(
        &self,
        state: &SharedState,
        cs_name: &str,
        dest_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_event_destination(cs_name, dest_name) {
            Ok(()) => wire::serialize_delete_configuration_set_event_destination_response(
                &wire::DeleteConfigurationSetEventDestinationResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_get_event_destinations(
        &self,
        state: &SharedState,
        cs_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_event_destinations(cs_name) {
            Ok(map) => {
                let dests: Vec<wire::EventDestination> = map
                    .iter()
                    .filter_map(|(name, value)| {
                        let mut ed: wire::EventDestination =
                            serde_json::from_value(value.clone()).ok()?;
                        ed.name = Some(name.clone());
                        Some(ed)
                    })
                    .collect();
                wire::serialize_get_configuration_set_event_destinations_response(
                    &wire::GetConfigurationSetEventDestinationsResponse {
                        event_destinations: if dests.is_empty() { None } else { Some(dests) },
                    },
                )
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_send_voice_message(&self, state: &SharedState, body: &Value) -> MockResponse {
        let message_id = format!("msg-{}", uuid::Uuid::new_v4().simple());
        let mut state = state.write().await;
        state.record_message(message_id.clone(), body.clone());
        wire::serialize_send_voice_message_response(&wire::SendVoiceMessageResponse {
            message_id: Some(message_id),
        })
    }
}

fn require_str(body: &Value, field: &str) -> Result<String, MockResponse> {
    body.get(field)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .ok_or_else(|| rest_json_error(400, "ValidationException", &format!("{field} is required")))
}

fn err_response(err: &PinpointSmsVoiceError) -> MockResponse {
    let (status, error_type) = match err {
        PinpointSmsVoiceError::NotFound { .. }
        | PinpointSmsVoiceError::DestinationNotFound { .. } => (404, "NotFoundException"),
        PinpointSmsVoiceError::AlreadyExists { .. } => (409, "AlreadyExistsException"),
        PinpointSmsVoiceError::Validation { .. } => (400, "BadRequestException"),
    };
    let body = json!({"Message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
