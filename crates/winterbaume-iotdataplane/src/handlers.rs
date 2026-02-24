use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use base64::Engine as _;
use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{IotDataPlaneError, IotDataPlaneState};
use crate::types::RetainedMessage;
use crate::views::IotDataPlaneStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct IotDataPlaneService {
    pub(crate) state: Arc<BackendState<IotDataPlaneState>>,
    pub(crate) notifier: StateChangeNotifier<IotDataPlaneStateView>,
}

impl IotDataPlaneService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for IotDataPlaneService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for IotDataPlaneService {
    fn service_name(&self) -> &str {
        "iotdata"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://data\.iot\.(.+)\.amazonaws\.com",
            r"https?://data-ats\.iot\.(.+)\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl IotDataPlaneService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let (path_part, query_string) = split_path_query(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        match method {
            // GET /things/{thingName}/shadow?name={shadowName} - GetThingShadow
            "GET" if matches_shadow_path(&segments) => {
                let thing_name = segments[1];
                let shadow_name = extract_query_param(&query_string, "name");
                self.handle_get_thing_shadow(&state, thing_name, shadow_name.as_deref())
                    .await
            }
            // POST /things/{thingName}/shadow?name={shadowName} - UpdateThingShadow
            "POST" if matches_shadow_path(&segments) => {
                let thing_name = segments[1];
                let shadow_name = extract_query_param(&query_string, "name");
                self.handle_update_thing_shadow(
                    &state,
                    thing_name,
                    shadow_name.as_deref(),
                    request.body.to_vec(),
                )
                .await
            }
            // DELETE /things/{thingName}/shadow?name={shadowName} - DeleteThingShadow
            "DELETE" if matches_shadow_path(&segments) => {
                let thing_name = segments[1];
                let shadow_name = extract_query_param(&query_string, "name");
                self.handle_delete_thing_shadow(&state, thing_name, shadow_name.as_deref())
                    .await
            }
            // GET /api/things/shadow/ListNamedShadowsForThing/{thingName} - ListNamedShadowsForThing
            "GET" if matches_list_named_shadows_path(&segments) => {
                let thing_name = segments[4];
                self.handle_list_named_shadows_for_thing(&state, thing_name)
                    .await
            }
            // POST /topics/{topic} - Publish
            "POST" if segments.len() >= 2 && segments[0] == "topics" => {
                let topic = urlencoding_decode(&segments[1..].join("/"));
                let qos = extract_query_param(&query_string, "qos")
                    .and_then(|v| v.parse::<i32>().ok())
                    .unwrap_or(0);
                let retain = extract_query_param(&query_string, "retain")
                    .map(|v| v == "true")
                    .unwrap_or(false);
                self.handle_publish(&state, &topic, request.body.to_vec(), qos, retain)
                    .await
            }
            // DELETE /connections/{clientId} - DeleteConnection
            "DELETE" if segments.len() == 2 && segments[0] == "connections" => {
                self.handle_delete_connection().await
            }
            // GET /retainedMessage - ListRetainedMessages
            "GET" if segments.len() == 1 && segments[0] == "retainedMessage" => {
                let max_results = extract_query_param(&query_string, "maxResults")
                    .and_then(|v| v.parse::<i32>().ok());
                let next_token = extract_query_param(&query_string, "nextToken");
                self.handle_list_retained_messages(&state, max_results, next_token.as_deref())
                    .await
            }
            // GET /retainedMessage/{topic+} - GetRetainedMessage
            "GET" if segments.len() >= 2 && segments[0] == "retainedMessage" => {
                let topic = urlencoding_decode(&segments[1..].join("/"));
                self.handle_get_retained_message(&state, &topic).await
            }
            _ => {
                let _ = (path_part, path);
                rest_json_error(404, "UnknownOperationException", "Not found")
            }
        }
    }

    async fn handle_get_thing_shadow(
        &self,
        state: &Arc<tokio::sync::RwLock<IotDataPlaneState>>,
        thing_name: &str,
        shadow_name: Option<&str>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_thing_shadow(thing_name, shadow_name) {
            Ok(shadow) => {
                let body = String::from_utf8_lossy(&shadow.payload).into_owned();
                MockResponse::rest_json(200, body)
            }
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_update_thing_shadow(
        &self,
        state: &Arc<tokio::sync::RwLock<IotDataPlaneState>>,
        thing_name: &str,
        shadow_name: Option<&str>,
        payload: Vec<u8>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.update_thing_shadow(thing_name, shadow_name, payload) {
            Ok(shadow) => {
                let body = String::from_utf8_lossy(&shadow.payload).into_owned();
                MockResponse::rest_json(200, body)
            }
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_delete_thing_shadow(
        &self,
        state: &Arc<tokio::sync::RwLock<IotDataPlaneState>>,
        thing_name: &str,
        shadow_name: Option<&str>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_thing_shadow(thing_name, shadow_name) {
            Ok(shadow) => {
                let body = serde_json::json!({
                    "version": shadow.version,
                    "timestamp": shadow.last_modified.timestamp(),
                })
                .to_string();
                MockResponse::rest_json(200, body)
            }
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_list_named_shadows_for_thing(
        &self,
        state: &Arc<tokio::sync::RwLock<IotDataPlaneState>>,
        thing_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let shadow_names = state.list_named_shadows_for_thing(thing_name);
        wire::serialize_list_named_shadows_for_thing_response(&list_named_shadows_wire(
            shadow_names,
        ))
    }

    async fn handle_publish(
        &self,
        state: &Arc<tokio::sync::RwLock<IotDataPlaneState>>,
        topic: &str,
        payload: Vec<u8>,
        qos: i32,
        retain: bool,
    ) -> MockResponse {
        if topic.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "Topic must not be empty");
        }
        let mut state = state.write().await;
        state.publish(topic, payload, qos, retain);
        wire::serialize_publish_response()
    }

    async fn handle_delete_connection(&self) -> MockResponse {
        wire::serialize_delete_connection_response()
    }

    async fn handle_get_retained_message(
        &self,
        state: &Arc<tokio::sync::RwLock<IotDataPlaneState>>,
        topic: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_retained_message(topic) {
            Ok(msg) => {
                wire::serialize_get_retained_message_response(&get_retained_message_wire(msg))
            }
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_list_retained_messages(
        &self,
        state: &Arc<tokio::sync::RwLock<IotDataPlaneState>>,
        max_results: Option<i32>,
        _next_token: Option<&str>,
    ) -> MockResponse {
        let state = state.read().await;
        let all = state.list_retained_messages();
        let limit = max_results.unwrap_or(i32::MAX) as usize;
        let topics: Vec<wire::RetainedMessageSummary> = all
            .into_iter()
            .take(limit)
            .map(retained_message_summary_wire)
            .collect();
        wire::serialize_list_retained_messages_response(&wire::ListRetainedMessagesResponse {
            next_token: None,
            retained_topics: Some(topics),
        })
    }
}

/// Check if segments match /things/{thingName}/shadow
fn matches_shadow_path(segments: &[&str]) -> bool {
    segments.len() == 3 && segments[0] == "things" && segments[2] == "shadow"
}

/// Check if segments match /api/things/shadow/ListNamedShadowsForThing/{thingName}
fn matches_list_named_shadows_path(segments: &[&str]) -> bool {
    segments.len() == 5
        && segments[0] == "api"
        && segments[1] == "things"
        && segments[2] == "shadow"
        && segments[3] == "ListNamedShadowsForThing"
}

fn extract_path(uri: &str) -> String {
    let after_scheme = if let Some(idx) = uri.find("://") {
        &uri[idx + 3..]
    } else {
        uri
    };
    let path_start = after_scheme.find('/').unwrap_or(after_scheme.len());
    let path_and_query = &after_scheme[path_start..];
    match path_and_query.find('?') {
        Some(q) => path_and_query[..q].to_string(),
        None => path_and_query.to_string(),
    }
}

fn split_path_query(uri: &str) -> (String, String) {
    let after_scheme = if let Some(idx) = uri.find("://") {
        &uri[idx + 3..]
    } else {
        uri
    };
    let path_start = after_scheme.find('/').unwrap_or(after_scheme.len());
    let path_and_query = &after_scheme[path_start..];
    match path_and_query.find('?') {
        Some(q) => (
            path_and_query[..q].to_string(),
            path_and_query[q + 1..].to_string(),
        ),
        None => (path_and_query.to_string(), String::new()),
    }
}

fn extract_query_param(query: &str, key: &str) -> Option<String> {
    if query.is_empty() {
        return None;
    }
    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let (Some(k), Some(v)) = (parts.next(), parts.next())
            && k == key
        {
            return Some(urlencoding_decode(v));
        }
    }
    None
}

fn urlencoding_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
            } else {
                result.push('%');
                result.push_str(&hex);
            }
        } else if c == '+' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }
    result
}

/// Convert a list of shadow names to a ListNamedShadowsForThingResponse wire type.
fn list_named_shadows_wire(shadow_names: Vec<String>) -> wire::ListNamedShadowsForThingResponse {
    wire::ListNamedShadowsForThingResponse {
        results: Some(shadow_names),
        timestamp: Some(chrono::Utc::now().timestamp()),
        next_token: None,
    }
}

/// Convert a RetainedMessage to a GetRetainedMessageResponse wire type.
fn get_retained_message_wire(msg: &RetainedMessage) -> wire::GetRetainedMessageResponse {
    let payload_b64 = base64::engine::general_purpose::STANDARD.encode(&msg.payload);
    wire::GetRetainedMessageResponse {
        last_modified_time: Some(msg.last_modified.timestamp()),
        payload: Some(payload_b64),
        qos: Some(msg.qos),
        topic: Some(msg.topic.clone()),
        user_properties: None,
    }
}

/// Convert a RetainedMessage to a RetainedMessageSummary wire type.
fn retained_message_summary_wire(msg: &RetainedMessage) -> wire::RetainedMessageSummary {
    wire::RetainedMessageSummary {
        last_modified_time: Some(msg.last_modified.timestamp()),
        payload_size: Some(msg.payload.len() as i64),
        qos: Some(msg.qos),
        topic: Some(msg.topic.clone()),
    }
}

fn iot_error_response(err: &IotDataPlaneError) -> MockResponse {
    let (status, error_type, message) = match err {
        IotDataPlaneError::ShadowNotFound { .. } => {
            (404, "ResourceNotFoundException", err.to_string())
        }
        IotDataPlaneError::RetainedMessageNotFound { .. } => {
            (404, "ResourceNotFoundException", err.to_string())
        }
        IotDataPlaneError::InvalidShadowDocument => {
            (400, "InvalidRequestException", err.to_string())
        }
    };
    let body = json!({
        "message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
