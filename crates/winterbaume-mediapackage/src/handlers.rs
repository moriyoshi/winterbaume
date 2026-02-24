use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path,
};

use crate::model;
use crate::state::{MediaPackageError, MediaPackageState};
use crate::types;
use crate::views::MediaPackageStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct MediaPackageService {
    pub(crate) state: Arc<BackendState<MediaPackageState>>,
    pub(crate) notifier: StateChangeNotifier<MediaPackageStateView>,
}

impl MediaPackageService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for MediaPackageService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for MediaPackageService {
    fn service_name(&self) -> &str {
        "mediapackage"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://mediapackage\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl MediaPackageService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_query = extract_query(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);
        let method = request.method.as_str();

        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

        // Routes:
        // POST   /channels          - CreateChannel
        // GET    /channels/{id}     - DescribeChannel
        // DELETE /channels/{id}     - DeleteChannel
        // GET    /channels          - ListChannels

        if segs.first() == Some(&"origin_endpoints") {
            return match (method, segs.len()) {
                ("POST", 1) => {
                    self.handle_create_origin_endpoint(
                        &state,
                        &request,
                        &[],
                        &query_map,
                        account_id,
                        &region,
                    )
                    .await
                }
                ("GET", 1) => {
                    self.handle_list_origin_endpoints(&state, &request, &[], &query_map)
                        .await
                }
                ("GET", 2) => {
                    let labels: &[(&str, &str)] = &[("Id", segs[1])];
                    self.handle_describe_origin_endpoint(&state, &request, labels, &query_map)
                        .await
                }
                ("DELETE", 2) => {
                    let labels: &[(&str, &str)] = &[("Id", segs[1])];
                    self.handle_delete_origin_endpoint(&state, &request, labels, &query_map)
                        .await
                }
                ("PUT", 2) => {
                    let labels: &[(&str, &str)] = &[("Id", segs[1])];
                    self.handle_update_origin_endpoint(&state, &request, labels, &query_map)
                        .await
                }
                _ => rest_json_error(404, "NotFoundException", "Not found"),
            };
        }

        if segs.is_empty() || segs[0] != "channels" {
            return rest_json_error(404, "NotFoundException", "Not found");
        }

        match (method, segs.len()) {
            // POST /channels - CreateChannel
            ("POST", 1) => {
                self.handle_create_channel(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            // GET /channels - ListChannels
            ("GET", 1) => {
                self.handle_list_channels(&state, &request, &[], &query_map)
                    .await
            }
            // GET /channels/{id} - DescribeChannel
            ("GET", 2) => {
                let labels: &[(&str, &str)] = &[("Id", segs[1])];
                self.handle_describe_channel(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /channels/{id} - DeleteChannel
            ("DELETE", 2) => {
                let labels: &[(&str, &str)] = &[("Id", segs[1])];
                self.handle_delete_channel(&state, &request, labels, &query_map)
                    .await
            }
            _ => rest_json_error(404, "NotFoundException", "Not found"),
        }
    }

    async fn handle_create_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_channel_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(422, "UnprocessableEntityException", &e),
        };
        if input.id.is_empty() {
            return rest_json_error(
                422,
                "UnprocessableEntityException",
                "Missing 'id' in request body",
            );
        }
        let description = input.description.as_deref().unwrap_or("");
        let tags: HashMap<String, String> = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_channel(&input.id, description, tags, account_id, region) {
            Ok(channel) => {
                let resp = model::CreateChannelResponse {
                    arn: Some(channel.arn.clone()),
                    description: Some(channel.description.clone()),
                    hls_ingest: Some(model::HlsIngest {
                        ingest_endpoints: Some(vec![]),
                    }),
                    id: Some(channel.id.clone()),
                    tags: Some(channel.tags.clone()),
                    ..Default::default()
                };
                wire::serialize_create_channel_response(&resp)
            }
            Err(e) => mediapackage_error_response(&e),
        }
    }

    async fn handle_describe_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_channel_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_channel(&input.id) {
            Ok(channel) => {
                let resp = model::DescribeChannelResponse {
                    arn: Some(channel.arn.clone()),
                    description: Some(channel.description.clone()),
                    hls_ingest: Some(model::HlsIngest {
                        ingest_endpoints: Some(vec![]),
                    }),
                    id: Some(channel.id.clone()),
                    tags: Some(channel.tags.clone()),
                    ..Default::default()
                };
                wire::serialize_describe_channel_response(&resp)
            }
            Err(e) => mediapackage_error_response(&e),
        }
    }

    async fn handle_delete_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_channel_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_channel(&input.id) {
            Ok(()) => wire::serialize_delete_channel_response(&model::DeleteChannelResponse {}),
            Err(e) => mediapackage_error_response(&e),
        }
    }

    async fn handle_list_channels(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_channels_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let channels = state.list_channels();
        let entries: Vec<model::Channel> = channels.iter().map(|ch| channel_to_wire(ch)).collect();
        let resp = model::ListChannelsResponse {
            channels: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_channels_response(&resp)
    }

    async fn handle_create_origin_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_origin_endpoint_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(422, "UnprocessableEntityException", &e),
        };
        if input.id.is_empty() {
            return rest_json_error(422, "UnprocessableEntityException", "Missing 'id'");
        }
        if input.channel_id.is_empty() {
            return rest_json_error(422, "UnprocessableEntityException", "Missing 'channelId'");
        }
        let description = input.description.as_deref().unwrap_or("");
        let manifest_name = input.manifest_name.as_deref().unwrap_or("default");
        let startover = input.startover_window_seconds.unwrap_or(0);
        let time_delay = input.time_delay_seconds.unwrap_or(0);
        let tags: HashMap<String, String> = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_origin_endpoint(
            &input.id,
            &input.channel_id,
            description,
            manifest_name,
            startover,
            time_delay,
            tags,
            account_id,
            region,
        ) {
            Ok(ep) => {
                let resp = model::CreateOriginEndpointResponse {
                    arn: Some(ep.arn.clone()),
                    channel_id: Some(ep.channel_id.clone()),
                    description: Some(ep.description.clone()),
                    id: Some(ep.id.clone()),
                    manifest_name: Some(ep.manifest_name.clone()),
                    startover_window_seconds: Some(ep.startover_window_seconds),
                    tags: Some(ep.tags.clone()),
                    time_delay_seconds: Some(ep.time_delay_seconds),
                    url: Some(ep.url.clone()),
                    ..Default::default()
                };
                wire::serialize_create_origin_endpoint_response(&resp)
            }
            Err(e) => mediapackage_error_response(&e),
        }
    }

    async fn handle_describe_origin_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_origin_endpoint_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_origin_endpoint(&input.id) {
            Ok(ep) => {
                let resp = model::DescribeOriginEndpointResponse {
                    arn: Some(ep.arn.clone()),
                    channel_id: Some(ep.channel_id.clone()),
                    description: Some(ep.description.clone()),
                    id: Some(ep.id.clone()),
                    manifest_name: Some(ep.manifest_name.clone()),
                    startover_window_seconds: Some(ep.startover_window_seconds),
                    tags: Some(ep.tags.clone()),
                    time_delay_seconds: Some(ep.time_delay_seconds),
                    url: Some(ep.url.clone()),
                    ..Default::default()
                };
                wire::serialize_describe_origin_endpoint_response(&resp)
            }
            Err(e) => mediapackage_error_response(&e),
        }
    }

    async fn handle_delete_origin_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_origin_endpoint_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_origin_endpoint(&input.id) {
            Ok(()) => wire::serialize_delete_origin_endpoint_response(
                &model::DeleteOriginEndpointResponse {},
            ),
            Err(e) => mediapackage_error_response(&e),
        }
    }

    async fn handle_list_origin_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_origin_endpoints_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let endpoints = state.list_origin_endpoints(input.channel_id.as_deref());
        let entries: Vec<model::OriginEndpoint> =
            endpoints.iter().map(|ep| ep_to_wire_model(ep)).collect();
        let resp = model::ListOriginEndpointsResponse {
            origin_endpoints: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_origin_endpoints_response(&resp)
    }

    async fn handle_update_origin_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_origin_endpoint_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(422, "UnprocessableEntityException", &e),
        };
        let description = input.description.as_deref();
        let manifest_name = input.manifest_name.as_deref();
        let startover = input.startover_window_seconds;
        let time_delay = input.time_delay_seconds;

        let mut state = state.write().await;
        match state.update_origin_endpoint(
            &input.id,
            description,
            manifest_name,
            startover,
            time_delay,
        ) {
            Ok(ep) => {
                let resp = model::UpdateOriginEndpointResponse {
                    arn: Some(ep.arn.clone()),
                    channel_id: Some(ep.channel_id.clone()),
                    description: Some(ep.description.clone()),
                    id: Some(ep.id.clone()),
                    manifest_name: Some(ep.manifest_name.clone()),
                    startover_window_seconds: Some(ep.startover_window_seconds),
                    tags: Some(ep.tags.clone()),
                    time_delay_seconds: Some(ep.time_delay_seconds),
                    url: Some(ep.url.clone()),
                    ..Default::default()
                };
                wire::serialize_update_origin_endpoint_response(&resp)
            }
            Err(e) => mediapackage_error_response(&e),
        }
    }
}

fn ep_to_wire_model(ep: &types::OriginEndpoint) -> model::OriginEndpoint {
    model::OriginEndpoint {
        arn: Some(ep.arn.clone()),
        channel_id: Some(ep.channel_id.clone()),
        description: Some(ep.description.clone()),
        id: Some(ep.id.clone()),
        manifest_name: Some(ep.manifest_name.clone()),
        startover_window_seconds: Some(ep.startover_window_seconds),
        tags: Some(ep.tags.clone()),
        time_delay_seconds: Some(ep.time_delay_seconds),
        url: Some(ep.url.clone()),
        ..Default::default()
    }
}

fn extract_query(uri: &str) -> String {
    match uri.find('?') {
        Some(idx) => uri[idx + 1..].to_string(),
        None => String::new(),
    }
}

/// Convert a state `Channel` to the wire `model::Channel` type.
fn channel_to_wire(ch: &types::Channel) -> model::Channel {
    model::Channel {
        arn: Some(ch.arn.clone()),
        description: Some(ch.description.clone()),
        hls_ingest: Some(model::HlsIngest {
            ingest_endpoints: Some(vec![]),
        }),
        id: Some(ch.id.clone()),
        tags: Some(ch.tags.clone()),
        ..Default::default()
    }
}

fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            b'+' => result.push(' '),
            _ => result.push(b as char),
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn mediapackage_error_response(err: &MediaPackageError) -> MockResponse {
    let (status, error_type) = match err {
        MediaPackageError::ChannelAlreadyExists { .. } => (422, "UnprocessableEntityException"),
        MediaPackageError::ChannelNotFound { .. } => (404, "NotFoundException"),
        MediaPackageError::OriginEndpointAlreadyExists { .. } => {
            (422, "UnprocessableEntityException")
        }
        MediaPackageError::OriginEndpointNotFound { .. } => (404, "NotFoundException"),
    };
    let body = json!({
        "message": err.to_string(),
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
