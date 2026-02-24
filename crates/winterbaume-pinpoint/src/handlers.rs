use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{PinpointError, PinpointState};
use crate::types::*;
use crate::views::PinpointStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct PinpointService {
    pub(crate) state: Arc<BackendState<PinpointState>>,
    pub(crate) notifier: StateChangeNotifier<PinpointStateView>,
}

impl PinpointService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for PinpointService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for PinpointService {
    fn service_name(&self) -> &str {
        "mobiletargeting"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://pinpoint\..*\.amazonaws\.com",
            r"https?://mobiletargeting\..*\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl PinpointService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        // Pinpoint API routes:
        // POST   /v1/apps                              - CreateApp
        // GET    /v1/apps                              - GetApps
        // GET    /v1/apps/{application-id}              - GetApp
        // DELETE /v1/apps/{application-id}              - DeleteApp
        // PUT    /v1/apps/{application-id}/settings     - UpdateApplicationSettings
        // GET    /v1/apps/{application-id}/settings     - GetApplicationSettings
        // POST   /v1/apps/{application-id}/eventstream  - PutEventStream
        // GET    /v1/apps/{application-id}/eventstream  - GetEventStream
        // DELETE /v1/apps/{application-id}/eventstream  - DeleteEventStream
        // GET    /v1/tags/{resource-arn}                - ListTagsForResource
        // POST   /v1/tags/{resource-arn}                - TagResource
        // DELETE /v1/tags/{resource-arn}                - UntagResource

        if segments.is_empty() || segments[0] != "v1" {
            return rest_json_error(404, "NotFoundException", "Not found");
        }

        if segments.len() >= 2 && segments[1] == "tags" {
            // Tag operations - the resource ARN is everything after /v1/tags/
            let resource_arn = decode_tag_resource_arn(&path);
            return match method {
                "GET" => {
                    self.handle_list_tags_for_resource(&state, &resource_arn)
                        .await
                }
                "POST" => {
                    let body: Value = match serde_json::from_slice(&request.body) {
                        Ok(v) => v,
                        Err(_) => {
                            return rest_json_error(400, "BadRequestException", "Invalid JSON");
                        }
                    };
                    self.handle_tag_resource(&state, &resource_arn, &body).await
                }
                "DELETE" => {
                    // Tag keys come as query parameters
                    let tag_keys = extract_tag_keys_from_query(&request.uri);
                    self.handle_untag_resource(&state, &resource_arn, &tag_keys)
                        .await
                }
                _ => rest_json_error(404, "NotFoundException", "Not found"),
            };
        }

        if segments.len() < 2 || segments[1] != "apps" {
            return rest_json_error(404, "NotFoundException", "Not found");
        }

        match (method, segments.len()) {
            // POST /v1/apps - CreateApp
            ("POST", 2) => {
                let body: Value = match serde_json::from_slice(&request.body) {
                    Ok(v) => v,
                    Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON"),
                };
                self.handle_create_app(&state, &body, account_id, &region)
                    .await
            }
            // GET /v1/apps - GetApps
            ("GET", 2) => self.handle_get_apps(&state).await,
            // GET /v1/apps/{id} - GetApp
            ("GET", 3) => {
                let app_id = segments[2];
                self.handle_get_app(&state, app_id).await
            }
            // DELETE /v1/apps/{id} - DeleteApp
            ("DELETE", 3) => {
                let app_id = segments[2];
                self.handle_delete_app(&state, app_id).await
            }
            // PUT /v1/apps/{id}/settings - UpdateApplicationSettings
            ("PUT", 4) if segments[3] == "settings" => {
                let app_id = segments[2];
                let body: Value = match serde_json::from_slice(&request.body) {
                    Ok(v) => v,
                    Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON"),
                };
                self.handle_update_application_settings(&state, app_id, &body)
                    .await
            }
            // GET /v1/apps/{id}/settings - GetApplicationSettings
            ("GET", 4) if segments[3] == "settings" => {
                let app_id = segments[2];
                self.handle_get_application_settings(&state, app_id).await
            }
            // POST /v1/apps/{id}/eventstream - PutEventStream
            ("POST", 4) if segments[3] == "eventstream" => {
                let app_id = segments[2];
                let body: Value = match serde_json::from_slice(&request.body) {
                    Ok(v) => v,
                    Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON"),
                };
                self.handle_put_event_stream(&state, app_id, &body).await
            }
            // GET /v1/apps/{id}/eventstream - GetEventStream
            ("GET", 4) if segments[3] == "eventstream" => {
                let app_id = segments[2];
                self.handle_get_event_stream(&state, app_id).await
            }
            // DELETE /v1/apps/{id}/eventstream - DeleteEventStream
            ("DELETE", 4) if segments[3] == "eventstream" => {
                let app_id = segments[2];
                self.handle_delete_event_stream(&state, app_id).await
            }
            // PUT /v1/apps/{id}/channels/email - UpdateEmailChannel
            ("PUT", 5) if segments[3] == "channels" && segments[4] == "email" => {
                let app_id = segments[2];
                let body: Value = match serde_json::from_slice(&request.body) {
                    Ok(v) => v,
                    Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON"),
                };
                self.handle_update_email_channel(&state, app_id, &body)
                    .await
            }
            // GET /v1/apps/{id}/channels/email - GetEmailChannel
            ("GET", 5) if segments[3] == "channels" && segments[4] == "email" => {
                let app_id = segments[2];
                self.handle_get_email_channel(&state, app_id).await
            }
            // DELETE /v1/apps/{id}/channels/email - DeleteEmailChannel
            ("DELETE", 5) if segments[3] == "channels" && segments[4] == "email" => {
                let app_id = segments[2];
                self.handle_delete_email_channel(&state, app_id).await
            }
            _ => rest_json_error(404, "NotFoundException", "Not found"),
        }
    }

    async fn handle_create_app(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        // httpPayload: CreateApplicationRequest is the body directly
        let req = body;

        let name = match req.get("Name").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => return rest_json_error(400, "BadRequestException", "Missing 'Name'"),
        };

        let tags: HashMap<String, String> = req
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_app(name, tags, account_id, region) {
            Ok(app) => serialize_payload(201, &app_to_wire(app)),
            Err(e) => pinpoint_error_response(&e),
        }
    }

    async fn handle_get_app(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
        app_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_app(app_id) {
            Ok(app) => serialize_payload(200, &app_to_wire(app)),
            Err(e) => pinpoint_error_response(&e),
        }
    }

    async fn handle_delete_app(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
        app_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_app(app_id) {
            Ok(app) => serialize_payload(200, &app_to_wire(&app)),
            Err(e) => pinpoint_error_response(&e),
        }
    }

    async fn handle_get_apps(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let apps = state.list_apps();
        let items: Vec<wire::ApplicationResponse> = apps.iter().map(|a| app_to_wire(a)).collect();
        let payload = wire::ApplicationsResponse {
            item: Some(items),
            ..Default::default()
        };
        serialize_payload(200, &payload)
    }

    async fn handle_update_application_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
        app_id: &str,
        body: &Value,
    ) -> MockResponse {
        // httpPayload: WriteApplicationSettingsRequest is the body directly
        let req = body;

        let campaign_hook = req.get("CampaignHook").map(|ch| CampaignHook {
            lambda_function_name: ch
                .get("LambdaFunctionName")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            mode: ch
                .get("Mode")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            web_url: ch
                .get("WebUrl")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        });

        let limits = req.get("Limits").map(|l| Limits {
            daily: l.get("Daily").and_then(|v| v.as_i64()),
            maximum_duration: l.get("MaximumDuration").and_then(|v| v.as_i64()),
            messages_per_second: l.get("MessagesPerSecond").and_then(|v| v.as_i64()),
            total: l.get("Total").and_then(|v| v.as_i64()),
            session: l.get("Session").and_then(|v| v.as_i64()),
        });

        let mut state = state.write().await;
        match state.update_application_settings(app_id, campaign_hook, limits) {
            Ok(app) => serialize_payload(200, &app_settings_to_wire(app)),
            Err(e) => pinpoint_error_response(&e),
        }
    }

    async fn handle_get_application_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
        app_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_application_settings(app_id) {
            Ok(app) => serialize_payload(200, &app_settings_to_wire(app)),
            Err(e) => pinpoint_error_response(&e),
        }
    }

    async fn handle_put_event_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
        app_id: &str,
        body: &Value,
    ) -> MockResponse {
        // httpPayload: WriteEventStream is the body directly
        let req = body;

        let destination_stream_arn = match req.get("DestinationStreamArn").and_then(|v| v.as_str())
        {
            Some(s) => s,
            None => {
                return rest_json_error(
                    400,
                    "BadRequestException",
                    "Missing 'DestinationStreamArn'",
                );
            }
        };
        let role_arn = match req.get("RoleArn").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => return rest_json_error(400, "BadRequestException", "Missing 'RoleArn'"),
        };

        let mut state = state.write().await;
        match state.put_event_stream(app_id, destination_stream_arn, role_arn) {
            Ok(es) => serialize_payload(200, &event_stream_to_wire(es)),
            Err(e) => pinpoint_error_response(&e),
        }
    }

    async fn handle_get_event_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
        app_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_event_stream(app_id) {
            Ok(es) => serialize_payload(200, &event_stream_to_wire(es)),
            Err(e) => pinpoint_error_response(&e),
        }
    }

    async fn handle_delete_event_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
        app_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_event_stream(app_id) {
            Ok(es) => serialize_payload(200, &event_stream_to_wire(&es)),
            Err(e) => pinpoint_error_response(&e),
        }
    }

    async fn handle_update_email_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
        app_id: &str,
        body: &Value,
    ) -> MockResponse {
        // httpPayload: EmailChannelRequest is the body directly
        let req = body;

        let enabled = req.get("Enabled").and_then(|v| v.as_bool()).unwrap_or(true);
        let from_address = match req.get("FromAddress").and_then(|v| v.as_str()) {
            Some(s) => s.to_string(),
            None => {
                return rest_json_error(400, "BadRequestException", "Missing 'FromAddress'");
            }
        };
        let identity = match req.get("Identity").and_then(|v| v.as_str()) {
            Some(s) => s.to_string(),
            None => return rest_json_error(400, "BadRequestException", "Missing 'Identity'"),
        };
        let role_arn = req
            .get("RoleArn")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let configuration_set = req
            .get("ConfigurationSet")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mut state = state.write().await;
        match state.update_email_channel(
            app_id,
            enabled,
            from_address,
            identity,
            role_arn,
            configuration_set,
        ) {
            Ok(channel) => serialize_payload(200, &email_channel_to_wire(channel)),
            Err(e) => pinpoint_error_response(&e),
        }
    }

    async fn handle_get_email_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
        app_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_email_channel(app_id) {
            Ok(channel) => serialize_payload(200, &email_channel_to_wire(channel)),
            Err(e) => pinpoint_error_response(&e),
        }
    }

    async fn handle_delete_email_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
        app_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_email_channel(app_id) {
            Ok(channel) => serialize_payload(200, &email_channel_to_wire(&channel)),
            Err(e) => pinpoint_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
        resource_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_tags_for_resource(resource_arn) {
            Ok(tags) => {
                let payload = wire::TagsModel { tags: tags.clone() };
                serialize_payload(200, &payload)
            }
            Err(e) => pinpoint_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
        resource_arn: &str,
        body: &Value,
    ) -> MockResponse {
        // httpPayload: TagsModel is the body directly
        let tags_model = body;

        let tags: HashMap<String, String> = tags_model
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.tag_resource(resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(),
            Err(e) => pinpoint_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<PinpointState>>,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.untag_resource(resource_arn, tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(),
            Err(e) => pinpoint_error_response(&e),
        }
    }
}

fn app_to_wire(app: &PinpointApp) -> wire::ApplicationResponse {
    wire::ApplicationResponse {
        arn: Some(app.arn.clone()),
        id: Some(app.id.clone()),
        name: Some(app.name.clone()),
        creation_date: Some(
            app.creation_date
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        ),
        tags: if app.tags.is_empty() {
            None
        } else {
            Some(app.tags.clone())
        },
    }
}

fn app_settings_to_wire(app: &PinpointApp) -> wire::ApplicationSettingsResource {
    // The terraform-provider-aws v5.x flattenCampaignHook/flattenCampaignLimits/
    // flattenQuietTime helpers panic with a nil-pointer dereference if these
    // fields are absent from the response. Always emit a non-nil object —
    // empty when nothing has been configured — so the provider sees a valid
    // pointer and can safely flatten the (possibly empty) attributes.
    let campaign_hook = app
        .settings
        .as_ref()
        .and_then(|s| s.campaign_hook.as_ref())
        .map(|ch| wire::CampaignHook {
            lambda_function_name: ch.lambda_function_name.clone(),
            mode: ch.mode.clone(),
            web_url: ch.web_url.clone(),
        })
        .unwrap_or_default();

    let limits = app
        .settings
        .as_ref()
        .and_then(|s| s.limits.as_ref())
        .map(|l| wire::CampaignLimits {
            daily: l.daily.map(|v| v as i32),
            maximum_duration: l.maximum_duration.map(|v| v as i32),
            messages_per_second: l.messages_per_second.map(|v| v as i32),
            total: l.total.map(|v| v as i32),
            session: l.session.map(|v| v as i32),
        })
        .unwrap_or_default();

    let quiet_time = app
        .quiet_time
        .as_ref()
        .map(|qt| wire::QuietTime {
            start: qt
                .get("start")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            end: qt
                .get("end")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        })
        .unwrap_or_default();

    wire::ApplicationSettingsResource {
        application_id: Some(app.id.clone()),
        last_modified_date: Some(
            app.settings
                .as_ref()
                .map(|s| {
                    s.last_modified_date
                        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                        .to_string()
                })
                .unwrap_or_else(|| {
                    app.creation_date
                        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                        .to_string()
                }),
        ),
        campaign_hook: Some(campaign_hook),
        limits: Some(limits),
        quiet_time: Some(quiet_time),
        ..Default::default()
    }
}

fn email_channel_to_wire(ec: &EmailChannel) -> wire::EmailChannelResponse {
    wire::EmailChannelResponse {
        application_id: Some(ec.application_id.clone()),
        configuration_set: ec.configuration_set.clone(),
        creation_date: Some(
            chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        ),
        enabled: Some(ec.enabled),
        from_address: Some(ec.from_address.clone()),
        has_credential: Some(false),
        id: Some(ec.application_id.clone()),
        identity: Some(ec.identity.clone()),
        is_archived: Some(false),
        last_modified_by: None,
        last_modified_date: Some(
            chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        ),
        messages_per_second: ec.messages_per_second,
        orchestration_sending_role_arn: None,
        platform: Some("EMAIL".to_string()),
        role_arn: ec.role_arn.clone(),
        version: Some(1),
    }
}

fn event_stream_to_wire(es: &EventStream) -> wire::EventStream {
    wire::EventStream {
        application_id: Some(es.application_id.clone()),
        destination_stream_arn: Some(es.destination_stream_arn.clone()),
        role_arn: Some(es.role_arn.clone()),
        last_modified_date: Some(
            es.last_modified_date
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        ),
        ..Default::default()
    }
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

fn decode_tag_resource_arn(path: &str) -> String {
    // Path format: /v1/tags/{url-encoded-arn}
    // The ARN is everything after /v1/tags/
    let prefix = "/v1/tags/";
    if let Some(arn_encoded) = path.strip_prefix(prefix) {
        urlencoding::decode(arn_encoded)
            .unwrap_or_else(|_| arn_encoded.into())
            .to_string()
    } else {
        String::new()
    }
}

fn extract_tag_keys_from_query(uri: &str) -> Vec<String> {
    if let Some(query_start) = uri.find('?') {
        let query = &uri[query_start + 1..];
        query
            .split('&')
            .filter_map(|param| {
                let parts: Vec<&str> = param.splitn(2, '=').collect();
                if parts.len() == 2 && parts[0] == "tagKeys" {
                    Some(
                        urlencoding::decode(parts[1])
                            .unwrap_or_else(|_| parts[1].into())
                            .to_string(),
                    )
                } else {
                    None
                }
            })
            .collect()
    } else {
        Vec::new()
    }
}

/// Serialize a payload-bound response. Pinpoint uses `@httpPayload` binding
/// where the inner payload member IS the HTTP body directly.
fn serialize_payload<T: serde::Serialize>(status: u16, payload: &T) -> MockResponse {
    let body = serde_json::to_string(payload).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

fn pinpoint_error_response(err: &PinpointError) -> MockResponse {
    let (status, error_type) = match err {
        PinpointError::ApplicationNotFound => (404, "NotFoundException"),
        PinpointError::ResourceNotFound => (404, "NotFoundException"),
    };
    let body = json!({
        "Message": err.to_string(),
        "RequestID": uuid::Uuid::new_v4().to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
