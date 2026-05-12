use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, extract_query_string, parse_query_string, rest_json_error,
};

use crate::state::{CloudTrailDataState, EventRecord};
use crate::views::CloudTrailDataStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct CloudTrailDataService {
    pub(crate) state: Arc<BackendState<CloudTrailDataState>>,
    pub(crate) notifier: StateChangeNotifier<CloudTrailDataStateView>,
}

impl CloudTrailDataService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CloudTrailDataService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CloudTrailDataService {
    fn service_name(&self) -> &str {
        "cloudtrail-data"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://cloudtrail-data\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<CloudTrailDataState>>;

impl CloudTrailDataService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let path = path.trim_start_matches('/');
        let method = request.method.as_str().to_uppercase();

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
            }
        };

        let response = match (method.as_str(), path) {
            ("POST", "PutAuditEvents") => {
                self.handle_put_audit_events(&state, &body, &request.uri)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && method != "GET" {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_put_audit_events(
        &self,
        state: &SharedState,
        body: &Value,
        uri: &str,
    ) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let channel_arn = qs.get("channelArn").cloned().unwrap_or_default();
        let external_id = qs.get("externalId").cloned().unwrap_or_default();
        if channel_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "channelArn query parameter is required",
            );
        }
        let entries: Vec<Value> = body
            .get("auditEvents")
            .and_then(|v| v.as_array())
            .map(|a| a.to_vec())
            .unwrap_or_default();
        if entries.is_empty() {
            return rest_json_error(400, "ValidationException", "auditEvents is required");
        }
        let mut successful: Vec<wire::AuditEventResultEntry> = vec![];
        let mut failed: Vec<wire::ResultErrorEntry> = vec![];
        let mut state = state.write().await;
        for entry in entries {
            let id = entry
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let event_data = entry.get("eventData").cloned().unwrap_or(Value::Null);
            let checksum = entry
                .get("eventDataChecksum")
                .and_then(|v| v.as_str())
                .map(String::from);
            if id.is_empty() {
                failed.push(wire::ResultErrorEntry {
                    error_code: Some("InvalidEventCategory".to_string()),
                    error_message: Some("id is required".to_string()),
                    id: None,
                });
                continue;
            }
            let event_id = format!("evt-{}", uuid::Uuid::new_v4().simple());
            let event_data_value = match event_data {
                Value::String(s) => serde_json::from_str(&s).unwrap_or(Value::String(s)),
                other => other,
            };
            state.record(EventRecord {
                channel_arn: channel_arn.clone(),
                event_id: event_id.clone(),
                external_id: external_id.clone(),
                event_data: event_data_value,
                checksum,
            });
            successful.push(wire::AuditEventResultEntry {
                event_i_d: Some(event_id),
                id: Some(id),
            });
        }
        wire::serialize_put_audit_events_response(&wire::PutAuditEventsResponse {
            failed: if failed.is_empty() {
                None
            } else {
                Some(failed)
            },
            successful: if successful.is_empty() {
                None
            } else {
                Some(successful)
            },
        })
    }
}

#[allow(dead_code)]
fn err_response(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"Message": message});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
