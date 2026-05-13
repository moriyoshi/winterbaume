use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::{HeaderName, HeaderValue};
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, percent_decode, rest_json_error,
};

use crate::state::{AttachmentRecord, ConnectParticipantError, ConnectParticipantState};
use crate::views::ConnectParticipantStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ConnectParticipantService {
    pub(crate) state: Arc<BackendState<ConnectParticipantState>>,
    pub(crate) notifier: StateChangeNotifier<ConnectParticipantStateView>,
}

impl ConnectParticipantService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ConnectParticipantService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ConnectParticipantService {
    fn service_name(&self) -> &str {
        "participant.connect"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://participant\.connect\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<ConnectParticipantState>>;

const X_AMZ_BEARER: &str = "x-amz-bearer";

fn header_value(req: &MockRequest, name: &str) -> Option<String> {
    req.headers
        .get(name)
        .or_else(|| req.headers.get(name.to_lowercase()))
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

impl ConnectParticipantService {
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

        let connection_token = header_value(&request, X_AMZ_BEARER).unwrap_or_default();

        let response = match (method.as_str(), segs.as_slice()) {
            ("POST", ["participant", "cancel-authentication"]) => {
                self.handle_cancel_authentication(&state, &connection_token)
                    .await
            }
            ("POST", ["participant", "complete-attachment-upload"]) => {
                self.handle_complete_attachment_upload(&state, &connection_token, &body)
                    .await
            }
            ("POST", ["participant", "connection"]) => {
                self.handle_create_connection(&state, &connection_token, &body)
                    .await
            }
            ("GET", ["participant", "views", token]) => self.handle_describe_view(token).await,
            ("POST", ["participant", "disconnect"]) => {
                self.handle_disconnect(&state, &connection_token).await
            }
            ("POST", ["participant", "start-attachment-upload"]) => {
                self.handle_start_attachment_upload(&state, &connection_token, &body)
                    .await
            }
            ("POST", ["participant", "authentication-url"]) => {
                self.handle_get_authentication_url(&body).await
            }
            ("POST", ["participant", "transcript"]) => {
                self.handle_get_transcript(&state, &connection_token).await
            }
            ("POST", ["participant", "event"]) => {
                self.handle_send_event(&state, &connection_token, &body)
                    .await
            }
            ("POST", ["participant", "message"]) => {
                self.handle_send_message(&state, &connection_token, &body)
                    .await
            }
            ("POST", ["participant", "attachment"]) => {
                self.handle_get_attachment(&state, &connection_token, &body)
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

    async fn handle_create_connection(
        &self,
        state: &SharedState,
        participant_token_header: &str,
        body: &Value,
    ) -> MockResponse {
        // ParticipantToken is sent in the X-Amz-Bearer header during connection creation,
        // not the JSON body. Fall back to body for compatibility with hand-rolled callers.
        let participant_token = if !participant_token_header.is_empty() {
            participant_token_header.to_string()
        } else if let Some(t) = body.get("ParticipantToken").and_then(|v| v.as_str()) {
            t.to_string()
        } else {
            String::new()
        };
        if participant_token.is_empty() {
            return rest_json_error(400, "ValidationException", "ParticipantToken is required");
        }
        let mut state = state.write().await;
        let connection = state.create_connection(participant_token);
        let connection_token = connection.connection_token.clone();
        wire::serialize_create_participant_connection_response(
            &wire::CreateParticipantConnectionResponse {
                connection_credentials: Some(wire::ConnectionCredentials {
                    connection_token: Some(connection_token.clone()),
                    expiry: Some((chrono::Utc::now() + chrono::Duration::hours(24)).to_rfc3339()),
                }),
                web_r_t_c_connection: None,
                websocket: Some(wire::Websocket {
                    connection_expiry: Some(
                        (chrono::Utc::now() + chrono::Duration::hours(24)).to_rfc3339(),
                    ),
                    url: Some(format!("wss://participant.connect/ws/{connection_token}")),
                }),
            },
        )
    }

    async fn handle_disconnect(&self, state: &SharedState, connection_token: &str) -> MockResponse {
        let mut state = state.write().await;
        match state.disconnect(connection_token) {
            Ok(()) => wire::serialize_disconnect_participant_response(
                &wire::DisconnectParticipantResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_send_message(
        &self,
        state: &SharedState,
        connection_token: &str,
        body: &Value,
    ) -> MockResponse {
        let content = body.get("Content").and_then(|v| v.as_str()).unwrap_or("");
        let content_type = body
            .get("ContentType")
            .and_then(|v| v.as_str())
            .unwrap_or("text/plain");
        let mut state = state.write().await;
        if state.require_active(connection_token).is_err() {
            return err_response(&ConnectParticipantError::ConnectionNotFound {
                token: connection_token.to_string(),
            });
        }
        let id = format!("msg-{}", uuid::Uuid::new_v4().simple());
        let absolute_time = chrono::Utc::now().to_rfc3339();
        state.append_transcript(
            connection_token,
            json!({
                "Id": id,
                "Type": "MESSAGE",
                "ContentType": content_type,
                "Content": content,
                "AbsoluteTime": absolute_time,
            }),
        );
        wire::serialize_send_message_response(&wire::SendMessageResponse {
            absolute_time: Some(absolute_time),
            id: Some(id),
            message_metadata: None,
        })
    }

    async fn handle_send_event(
        &self,
        state: &SharedState,
        connection_token: &str,
        body: &Value,
    ) -> MockResponse {
        let content_type = body
            .get("ContentType")
            .and_then(|v| v.as_str())
            .unwrap_or("application/vnd.amazonaws.connect.event.typing");
        let content = body.get("Content").and_then(|v| v.as_str());
        let mut state = state.write().await;
        if state.require_active(connection_token).is_err() {
            return err_response(&ConnectParticipantError::ConnectionNotFound {
                token: connection_token.to_string(),
            });
        }
        let id = format!("evt-{}", uuid::Uuid::new_v4().simple());
        let absolute_time = chrono::Utc::now().to_rfc3339();
        let mut item = json!({
            "Id": id,
            "Type": "EVENT",
            "ContentType": content_type,
            "AbsoluteTime": absolute_time,
        });
        if let Some(c) = content {
            item.as_object_mut()
                .unwrap()
                .insert("Content".to_string(), Value::String(c.to_string()));
        }
        state.append_transcript(connection_token, item);
        wire::serialize_send_event_response(&wire::SendEventResponse {
            absolute_time: Some(absolute_time),
            id: Some(id),
        })
    }

    async fn handle_get_transcript(
        &self,
        state: &SharedState,
        connection_token: &str,
    ) -> MockResponse {
        let state = state.read().await;
        if state.require_active(connection_token).is_err() {
            return err_response(&ConnectParticipantError::ConnectionNotFound {
                token: connection_token.to_string(),
            });
        }
        let items: Vec<wire::Item> = state
            .transcript(connection_token)
            .iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();
        wire::serialize_get_transcript_response(&wire::GetTranscriptResponse {
            initial_contact_id: None,
            next_token: None,
            transcript: Some(items),
        })
    }

    async fn handle_start_attachment_upload(
        &self,
        state: &SharedState,
        connection_token: &str,
        body: &Value,
    ) -> MockResponse {
        let attachment_name = body
            .get("AttachmentName")
            .and_then(|v| v.as_str())
            .unwrap_or("attachment")
            .to_string();
        let content_type = body
            .get("ContentType")
            .and_then(|v| v.as_str())
            .unwrap_or("application/octet-stream")
            .to_string();
        let size = body
            .get("AttachmentSizeInBytes")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let mut state = state.write().await;
        if state.require_active(connection_token).is_err() {
            return err_response(&ConnectParticipantError::ConnectionNotFound {
                token: connection_token.to_string(),
            });
        }
        let attachment_id = format!("att-{}", uuid::Uuid::new_v4().simple());
        state.upsert_attachment(AttachmentRecord {
            attachment_id: attachment_id.clone(),
            connection_token: connection_token.to_string(),
            name: attachment_name,
            size_in_bytes: size,
            content_type,
            status: "IN_PROGRESS".to_string(),
        });
        let mut headers = HashMap::new();
        headers.insert(
            "Content-Type".to_string(),
            "application/octet-stream".to_string(),
        );
        wire::serialize_start_attachment_upload_response(&wire::StartAttachmentUploadResponse {
            attachment_id: Some(attachment_id.clone()),
            upload_metadata: Some(wire::UploadMetadata {
                headers_to_include: Some(headers),
                url: Some(format!(
                    "https://participant.connect/upload/{attachment_id}"
                )),
                url_expiry: Some((chrono::Utc::now() + chrono::Duration::minutes(10)).to_rfc3339()),
            }),
        })
    }

    async fn handle_complete_attachment_upload(
        &self,
        state: &SharedState,
        connection_token: &str,
        body: &Value,
    ) -> MockResponse {
        let ids: Vec<String> = body
            .get("AttachmentIds")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        if ids.is_empty() {
            return rest_json_error(400, "ValidationException", "AttachmentIds is required");
        }
        let mut state = state.write().await;
        if state.require_active(connection_token).is_err() {
            return err_response(&ConnectParticipantError::ConnectionNotFound {
                token: connection_token.to_string(),
            });
        }
        match state.complete_attachments(&ids) {
            Ok(()) => wire::serialize_complete_attachment_upload_response(
                &wire::CompleteAttachmentUploadResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_get_attachment(
        &self,
        state: &SharedState,
        connection_token: &str,
        body: &Value,
    ) -> MockResponse {
        let attachment_id = body
            .get("AttachmentId")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if attachment_id.is_empty() {
            return rest_json_error(400, "ValidationException", "AttachmentId is required");
        }
        let state = state.read().await;
        if state.require_active(connection_token).is_err() {
            return err_response(&ConnectParticipantError::ConnectionNotFound {
                token: connection_token.to_string(),
            });
        }
        match state.attachments.get(&attachment_id) {
            Some(a) => wire::serialize_get_attachment_response(&wire::GetAttachmentResponse {
                attachment_size_in_bytes: Some(a.size_in_bytes),
                url: Some(format!(
                    "https://participant.connect/download/{}",
                    a.attachment_id
                )),
                url_expiry: Some((chrono::Utc::now() + chrono::Duration::minutes(10)).to_rfc3339()),
            }),
            None => {
                err_response(&ConnectParticipantError::AttachmentNotFound { id: attachment_id })
            }
        }
    }

    async fn handle_get_authentication_url(&self, _body: &Value) -> MockResponse {
        wire::serialize_get_authentication_url_response(&wire::GetAuthenticationUrlResponse {
            authentication_url: Some("https://participant.connect/auth?state=mock".to_string()),
        })
    }

    async fn handle_cancel_authentication(
        &self,
        state: &SharedState,
        connection_token: &str,
    ) -> MockResponse {
        let state = state.read().await;
        if state.require_active(connection_token).is_err() {
            return err_response(&ConnectParticipantError::ConnectionNotFound {
                token: connection_token.to_string(),
            });
        }
        wire::serialize_cancel_participant_authentication_response(
            &wire::CancelParticipantAuthenticationResponse {},
        )
    }

    async fn handle_describe_view(&self, view_token: &str) -> MockResponse {
        wire::serialize_describe_view_response(&wire::DescribeViewResponse {
            view: Some(wire::View {
                arn: Some(format!("arn:aws:connect::view/{view_token}")),
                content: None,
                id: Some(view_token.to_string()),
                name: Some(format!("view-{view_token}")),
                version: Some(1),
            }),
        })
    }
}

#[allow(dead_code)]
fn header_to_value(s: &str) -> Option<HeaderValue> {
    HeaderValue::from_str(s).ok()
}

fn err_response(err: &ConnectParticipantError) -> MockResponse {
    let (status, error_type) = match err {
        ConnectParticipantError::ConnectionNotFound { .. }
        | ConnectParticipantError::AttachmentNotFound { .. } => (404, "ResourceNotFoundException"),
        ConnectParticipantError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"Message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
