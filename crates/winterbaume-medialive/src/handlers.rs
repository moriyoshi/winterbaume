use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::Value;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

fn opt_value<T: serde::Serialize>(v: &Option<T>) -> Option<Value> {
    v.as_ref().and_then(|x| serde_json::to_value(x).ok())
}

fn value_or_array<T: serde::Serialize>(v: &Option<T>) -> Value {
    opt_value(v).unwrap_or(Value::Array(vec![]))
}

fn value_or_object<T: serde::Serialize>(v: &Option<T>) -> Value {
    opt_value(v).unwrap_or(Value::Object(Default::default()))
}

use crate::state::{MediaLiveError, MediaLiveState};
use crate::types::*;
use crate::views::MediaLiveStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct MediaLiveService {
    pub(crate) state: Arc<BackendState<MediaLiveState>>,
    pub(crate) notifier: StateChangeNotifier<MediaLiveStateView>,
}

impl MediaLiveService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for MediaLiveService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for MediaLiveService {
    fn service_name(&self) -> &str {
        "medialive"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://medialive\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl MediaLiveService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let (path, query) = extract_path_and_query(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        // MediaLive API routes (REST JSON):
        // POST   /prod/channels                    - CreateChannel
        // GET    /prod/channels                    - ListChannels
        // GET    /prod/channels/{channelId}        - DescribeChannel
        // DELETE /prod/channels/{channelId}        - DeleteChannel
        // POST   /prod/channels/{channelId}/start  - StartChannel
        // POST   /prod/channels/{channelId}/stop   - StopChannel
        // PUT    /prod/channels/{channelId}        - UpdateChannel
        // POST   /prod/inputSecurityGroups         - CreateInputSecurityGroup
        // POST   /prod/inputs                      - CreateInput
        // GET    /prod/inputs                      - ListInputs
        // GET    /prod/inputs/{inputId}            - DescribeInput
        // DELETE /prod/inputs/{inputId}            - DeleteInput
        // PUT    /prod/inputs/{inputId}            - UpdateInput

        if segments.is_empty() || segments[0] != "prod" {
            return rest_json_error(404, "NotFoundException", "Not found");
        }

        if segments.len() >= 2 && segments[1] == "channels" {
            return self
                .dispatch_channels(&state, method, &segments[1..], &request, &query)
                .await;
        }

        if segments.len() >= 2 && segments[1] == "inputs" {
            return self
                .dispatch_inputs(&state, method, &segments[1..], &request, &query)
                .await;
        }

        if segments.len() >= 2 && segments[1] == "tags" {
            return self
                .dispatch_tags(&state, method, &segments[1..], &request, &query)
                .await;
        }

        if segments.len() >= 2 && segments[1] == "inputSecurityGroups" {
            // STUB[no-state]: InputSecurityGroups are not tracked in mock state; returns a
            //   minimal well-formed response with a placeholder ID.
            if method == "POST" {
                let resp = wire::CreateInputSecurityGroupResponse {
                    security_group: Some(wire::InputSecurityGroup {
                        id: Some("123456".to_string()),
                        arn: Some("arn:aws:medialive:inputSecurityGroup:123456".to_string()),
                        ..Default::default()
                    }),
                };
                return wire::serialize_create_input_security_group_response(&resp);
            }
        }

        rest_json_error(404, "NotFoundException", "Not found")
    }

    async fn dispatch_channels(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &str,
    ) -> MockResponse {
        let query_map: std::collections::HashMap<String, String> =
            winterbaume_core::parse_query_string(query);
        match (method, segments.len()) {
            // POST /prod/channels - CreateChannel
            ("POST", 1) => self.handle_create_channel(state, request, &query_map).await,
            // GET /prod/channels - ListChannels
            ("GET", 1) => {
                let max_results =
                    extract_query_param(query, "maxResults").and_then(|s| s.parse::<usize>().ok());
                let next_token = extract_query_param(query, "nextToken");
                self.handle_list_channels(state, max_results, next_token.as_deref())
                    .await
            }
            // GET /prod/channels/{id} - DescribeChannel
            ("GET", 2) => {
                let channel_id = segments[1];
                self.handle_describe_channel(state, channel_id).await
            }
            // DELETE /prod/channels/{id} - DeleteChannel
            ("DELETE", 2) => {
                let channel_id = segments[1];
                self.handle_delete_channel(state, channel_id).await
            }
            // PUT /prod/channels/{id} - UpdateChannel
            ("PUT", 2) => {
                let channel_id = segments[1];
                self.handle_update_channel(state, channel_id, request, &query_map)
                    .await
            }
            // POST /prod/channels/{id}/start - StartChannel
            ("POST", 3) if segments[2] == "start" => {
                let channel_id = segments[1];
                self.handle_start_channel(state, channel_id).await
            }
            // POST /prod/channels/{id}/stop - StopChannel
            ("POST", 3) if segments[2] == "stop" => {
                let channel_id = segments[1];
                self.handle_stop_channel(state, channel_id).await
            }
            _ => rest_json_error(404, "NotFoundException", "Not found"),
        }
    }

    async fn dispatch_inputs(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &str,
    ) -> MockResponse {
        let query_map: std::collections::HashMap<String, String> =
            winterbaume_core::parse_query_string(query);
        match (method, segments.len()) {
            // POST /prod/inputs - CreateInput
            ("POST", 1) => self.handle_create_input(state, request, &query_map).await,
            // GET /prod/inputs - ListInputs
            ("GET", 1) => {
                let max_results =
                    extract_query_param(query, "maxResults").and_then(|s| s.parse::<usize>().ok());
                let next_token = extract_query_param(query, "nextToken");
                self.handle_list_inputs(state, max_results, next_token.as_deref())
                    .await
            }
            // GET /prod/inputs/{id} - DescribeInput
            ("GET", 2) => {
                let input_id = segments[1];
                self.handle_describe_input(state, input_id).await
            }
            // DELETE /prod/inputs/{id} - DeleteInput
            ("DELETE", 2) => {
                let input_id = segments[1];
                self.handle_delete_input(state, input_id).await
            }
            // PUT /prod/inputs/{id} - UpdateInput
            ("PUT", 2) => {
                let input_id = segments[1];
                self.handle_update_input(state, input_id, request, &query_map)
                    .await
            }
            _ => rest_json_error(404, "NotFoundException", "Not found"),
        }
    }

    async fn dispatch_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &str,
    ) -> MockResponse {
        let query_map: std::collections::HashMap<String, String> =
            winterbaume_core::parse_query_string(query);
        // Routes:
        //   GET    /prod/tags/{resource-arn}                -> ListTagsForResource
        //   POST   /prod/tags/{resource-arn}                -> CreateTags
        //   DELETE /prod/tags/{resource-arn}?tagKeys=...    -> DeleteTags
        //
        // FIX(terraform-e2e): the terraform-aws provider URL-encodes the full
        // resource ARN ( e.g. `arn%3Aaws%3Amedialive%3Ainput%3Aabcd1234` ); we
        // re-join any segments that follow `tags/` ( a slash inside the ARN
        // would have been path-split ) and percent-decode before lookup.
        if segments.len() < 2 {
            return rest_json_error(404, "NotFoundException", "Not found");
        }
        let raw = segments[1..].join("/");
        let resource = urlencoding::decode(&raw)
            .map(|s| s.into_owned())
            .unwrap_or(raw);

        match method {
            "GET" => self.handle_list_tags_for_resource(state, &resource).await,
            "POST" => {
                self.handle_create_tags(state, &resource, request, &query_map)
                    .await
            }
            "DELETE" => {
                self.handle_delete_tags(state, &resource, request, &query_map)
                    .await
            }
            _ => rest_json_error(404, "NotFoundException", "Not found"),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        resource: &str,
    ) -> MockResponse {
        let st = state.read().await;
        let tags = st.list_tags_for_resource(resource).unwrap_or_default();
        let resp = wire::ListTagsForResourceResponse { tags: Some(tags) };
        wire::serialize_list_tags_for_resource_response(&resp)
    }

    async fn handle_create_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        resource: &str,
        request: &MockRequest,
        query: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("ResourceArn", resource)];
        let input = match wire::deserialize_create_tags_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON"),
        };
        let tags = input.tags.unwrap_or_default();
        let mut st = state.write().await;
        if !st.create_tags(resource, tags) {
            return rest_json_error(404, "NotFoundException", "Resource not found");
        }
        wire::serialize_create_tags_response()
    }

    async fn handle_delete_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        resource: &str,
        request: &MockRequest,
        query: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("ResourceArn", resource)];
        let input = match wire::deserialize_delete_tags_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON"),
        };
        let keys = input.tag_keys;
        let mut st = state.write().await;
        if !st.delete_tags(resource, &keys) {
            return rest_json_error(404, "NotFoundException", "Resource not found");
        }
        wire::serialize_delete_tags_response()
    }

    async fn handle_create_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        request: &MockRequest,
        query: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[];
        let input = match wire::deserialize_create_channel_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON"),
        };
        let name = input.name.as_deref().unwrap_or("");
        let channel_class = input.channel_class.as_deref();
        let role_arn = input.role_arn.as_deref().unwrap_or("");
        let input_attachments = value_or_array(&input.input_attachments);
        let destinations = value_or_array(&input.destinations);
        let encoder_settings = value_or_object(&input.encoder_settings);
        let input_specification = value_or_object(&input.input_specification);
        let log_level = input.log_level.as_deref().unwrap_or("DISABLED");
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        let channel = state.create_channel(
            name,
            channel_class,
            role_arn,
            input_attachments,
            destinations,
            encoder_settings,
            input_specification,
            log_level,
            tags,
        );

        let resp = wire::CreateChannelResponse {
            channel: Some(channel_to_wire(channel, None)),
        };
        wire::serialize_create_channel_response(&resp)
    }

    async fn handle_describe_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        channel_id: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.describe_channel(channel_id) {
            Ok(channel) => {
                // On describe, channel transitions from CREATING/UPDATING to IDLE
                let state_override = if channel.state == "CREATING" || channel.state == "UPDATING" {
                    Some("IDLE")
                } else {
                    None
                };
                let ch = channel_to_wire(channel, state_override);
                let resp = wire::DescribeChannelResponse {
                    arn: ch.arn,
                    channel_class: ch.channel_class,
                    destinations: ch.destinations,
                    encoder_settings: ch.encoder_settings,
                    id: ch.id,
                    input_attachments: ch.input_attachments,
                    input_specification: ch.input_specification,
                    log_level: ch.log_level,
                    name: ch.name,
                    pipelines_running_count: ch.pipelines_running_count,
                    role_arn: ch.role_arn,
                    state: ch.state,
                    tags: ch.tags,
                    ..Default::default()
                };
                wire::serialize_describe_channel_response(&resp)
            }
            Err(e) => medialive_error_response(&e),
        }
    }

    async fn handle_delete_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        channel_id: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.delete_channel(channel_id) {
            Ok(channel) => {
                let ch = channel_to_wire(channel, None);
                let resp = wire::DeleteChannelResponse {
                    arn: ch.arn,
                    channel_class: ch.channel_class,
                    destinations: ch.destinations,
                    encoder_settings: ch.encoder_settings,
                    id: ch.id,
                    input_attachments: ch.input_attachments,
                    input_specification: ch.input_specification,
                    log_level: ch.log_level,
                    name: ch.name,
                    pipelines_running_count: ch.pipelines_running_count,
                    role_arn: ch.role_arn,
                    state: ch.state,
                    tags: ch.tags,
                    ..Default::default()
                };
                wire::serialize_delete_channel_response(&resp)
            }
            Err(e) => medialive_error_response(&e),
        }
    }

    async fn handle_list_channels(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> MockResponse {
        let st = state.read().await;
        let (channels, next) = st.list_channels(max_results, next_token);
        let items: Vec<wire::ChannelSummary> =
            channels.iter().map(|c| channel_to_summary(c)).collect();
        let resp = wire::ListChannelsResponse {
            channels: Some(items),
            next_token: next,
        };
        wire::serialize_list_channels_response(&resp)
    }

    async fn handle_start_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        channel_id: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.start_channel(channel_id) {
            Ok(channel) => {
                // Start returns STARTING state but internally sets to RUNNING
                let ch = channel_to_wire(channel, Some("STARTING"));
                let resp = wire::StartChannelResponse {
                    arn: ch.arn,
                    channel_class: ch.channel_class,
                    destinations: ch.destinations,
                    encoder_settings: ch.encoder_settings,
                    id: ch.id,
                    input_attachments: ch.input_attachments,
                    input_specification: ch.input_specification,
                    log_level: ch.log_level,
                    name: ch.name,
                    pipelines_running_count: ch.pipelines_running_count,
                    role_arn: ch.role_arn,
                    state: ch.state,
                    tags: ch.tags,
                    ..Default::default()
                };
                wire::serialize_start_channel_response(&resp)
            }
            Err(e) => medialive_error_response(&e),
        }
    }

    async fn handle_stop_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        channel_id: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.stop_channel(channel_id) {
            Ok(channel) => {
                // Stop returns STOPPING state but internally sets to IDLE
                let ch = channel_to_wire(channel, Some("STOPPING"));
                let resp = wire::StopChannelResponse {
                    arn: ch.arn,
                    channel_class: ch.channel_class,
                    destinations: ch.destinations,
                    encoder_settings: ch.encoder_settings,
                    id: ch.id,
                    input_attachments: ch.input_attachments,
                    input_specification: ch.input_specification,
                    log_level: ch.log_level,
                    name: ch.name,
                    pipelines_running_count: ch.pipelines_running_count,
                    role_arn: ch.role_arn,
                    state: ch.state,
                    tags: ch.tags,
                    ..Default::default()
                };
                wire::serialize_stop_channel_response(&resp)
            }
            Err(e) => medialive_error_response(&e),
        }
    }

    async fn handle_update_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        channel_id: &str,
        request: &MockRequest,
        query: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("ChannelId", channel_id)];
        let input = match wire::deserialize_update_channel_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON"),
        };
        let name = input.name.as_deref();
        let destinations = opt_value(&input.destinations);
        let encoder_settings = opt_value(&input.encoder_settings);
        let input_attachments = opt_value(&input.input_attachments);
        let input_specification = opt_value(&input.input_specification);
        let log_level = input.log_level.as_deref();
        let role_arn = input.role_arn.as_deref();

        let mut st = state.write().await;
        match st.update_channel(
            channel_id,
            name,
            destinations,
            encoder_settings,
            input_attachments,
            input_specification,
            log_level,
            role_arn,
        ) {
            Ok(channel) => {
                let ch = channel_to_wire(channel, Some("UPDATING"));
                let resp = wire::UpdateChannelResponse { channel: Some(ch) };
                wire::serialize_update_channel_response(&resp)
            }
            Err(e) => medialive_error_response(&e),
        }
    }

    async fn handle_create_input(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        request: &MockRequest,
        query: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[];
        let input_req = match wire::deserialize_create_input_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON"),
        };
        let name = input_req.name.as_deref().unwrap_or("");
        let input_type = input_req.r#type.as_deref().unwrap_or("RTP_PUSH");
        let role_arn = input_req.role_arn.as_deref().unwrap_or("");
        let destinations = value_or_array(&input_req.destinations);
        let input_devices = value_or_array(&input_req.input_devices);
        let media_connect_flows = value_or_array(&input_req.media_connect_flows);
        let sources = value_or_array(&input_req.sources);
        let tags = input_req.tags.unwrap_or_default();

        let mut state = state.write().await;
        let input = state.create_input(
            name,
            input_type,
            role_arn,
            destinations,
            input_devices,
            media_connect_flows,
            sources,
            tags,
        );

        let resp = wire::CreateInputResponse {
            input: Some(input_to_wire(input, None)),
        };
        wire::serialize_create_input_response(&resp)
    }

    async fn handle_describe_input(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        input_id: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.describe_input(input_id) {
            Ok(input) => {
                // On describe, CREATING transitions to DETACHED
                let state_override = if input.state == "CREATING" {
                    Some("DETACHED")
                } else {
                    None
                };
                let inp = input_to_wire(input, state_override);
                let resp = wire::DescribeInputResponse {
                    arn: inp.arn,
                    attached_channels: inp.attached_channels,
                    destinations: inp.destinations,
                    id: inp.id,
                    input_class: inp.input_class,
                    input_devices: inp.input_devices,
                    input_source_type: inp.input_source_type,
                    media_connect_flows: inp.media_connect_flows,
                    name: inp.name,
                    role_arn: inp.role_arn,
                    security_groups: inp.security_groups,
                    sources: inp.sources,
                    state: inp.state,
                    tags: inp.tags,
                    r#type: inp.r#type,
                    ..Default::default()
                };
                wire::serialize_describe_input_response(&resp)
            }
            Err(e) => medialive_error_response(&e),
        }
    }

    async fn handle_delete_input(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        input_id: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.delete_input(input_id) {
            Ok(()) => {
                // wire::serialize_delete_input_response uses status 200
                wire::serialize_delete_input_response(&wire::DeleteInputResponse {})
            }
            Err(e) => medialive_error_response(&e),
        }
    }

    async fn handle_list_inputs(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> MockResponse {
        let st = state.read().await;
        let (inputs, next) = st.list_inputs(max_results, next_token);
        let items: Vec<wire::Input> = inputs.iter().map(|i| input_to_wire(i, None)).collect();
        let resp = wire::ListInputsResponse {
            inputs: Some(items),
            next_token: next,
        };
        wire::serialize_list_inputs_response(&resp)
    }

    async fn handle_update_input(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaLiveState>>,
        input_id: &str,
        request: &MockRequest,
        query: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("InputId", input_id)];
        let input_req = match wire::deserialize_update_input_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON"),
        };
        let name = input_req.name.as_deref();
        let destinations = opt_value(&input_req.destinations);
        let input_devices = opt_value(&input_req.input_devices);
        let media_connect_flows = opt_value(&input_req.media_connect_flows);
        let role_arn = input_req.role_arn.as_deref();
        let sources = opt_value(&input_req.sources);

        let mut st = state.write().await;
        match st.update_input(
            input_id,
            name,
            destinations,
            input_devices,
            media_connect_flows,
            role_arn,
            sources,
        ) {
            Ok(input) => {
                let resp = wire::UpdateInputResponse {
                    input: Some(input_to_wire(input, None)),
                };
                wire::serialize_update_input_response(&resp)
            }
            Err(e) => medialive_error_response(&e),
        }
    }
}

// ---- State-to-wire model conversion helpers ----

fn channel_to_wire(channel: &MediaLiveChannel, state_override: Option<&str>) -> wire::Channel {
    wire::Channel {
        arn: Some(channel.arn.clone()),
        channel_class: Some(channel.channel_class.clone()),
        destinations: serde_json::from_value(channel.destinations.clone()).ok(),
        encoder_settings: serde_json::from_value(channel.encoder_settings.clone()).ok(),
        id: Some(channel.id.clone()),
        input_attachments: serde_json::from_value(channel.input_attachments.clone()).ok(),
        input_specification: serde_json::from_value(channel.input_specification.clone()).ok(),
        log_level: Some(channel.log_level.clone()),
        name: Some(channel.name.clone()),
        pipelines_running_count: Some(channel.pipelines_running_count),
        role_arn: Some(channel.role_arn.clone()),
        state: Some(state_override.unwrap_or(&channel.state).to_string()),
        tags: Some(channel.tags.clone()),
        ..Default::default()
    }
}

fn channel_to_summary(channel: &MediaLiveChannel) -> wire::ChannelSummary {
    wire::ChannelSummary {
        arn: Some(channel.arn.clone()),
        channel_class: Some(channel.channel_class.clone()),
        destinations: serde_json::from_value(channel.destinations.clone()).ok(),
        id: Some(channel.id.clone()),
        input_attachments: serde_json::from_value(channel.input_attachments.clone()).ok(),
        input_specification: serde_json::from_value(channel.input_specification.clone()).ok(),
        log_level: Some(channel.log_level.clone()),
        name: Some(channel.name.clone()),
        pipelines_running_count: Some(channel.pipelines_running_count),
        role_arn: Some(channel.role_arn.clone()),
        state: Some(channel.state.clone()),
        tags: Some(channel.tags.clone()),
        ..Default::default()
    }
}

fn input_to_wire(input: &MediaLiveInput, state_override: Option<&str>) -> wire::Input {
    wire::Input {
        arn: Some(input.arn.clone()),
        attached_channels: Some(input.attached_channels.clone()),
        destinations: serde_json::from_value(input.destinations.clone()).ok(),
        id: Some(input.id.clone()),
        input_class: Some(input.input_class.clone()),
        input_devices: serde_json::from_value(input.input_devices.clone()).ok(),
        input_source_type: Some(input.input_source_type.clone()),
        media_connect_flows: serde_json::from_value(input.media_connect_flows.clone()).ok(),
        name: Some(input.name.clone()),
        role_arn: Some(input.role_arn.clone()),
        security_groups: Some(input.security_groups.clone()),
        sources: serde_json::from_value(input.sources.clone()).ok(),
        state: Some(state_override.unwrap_or(&input.state).to_string()),
        tags: Some(input.tags.clone()),
        r#type: Some(input.input_type.clone()),
        ..Default::default()
    }
}

// ---- Utility functions ----

fn extract_path_and_query(uri: &str) -> (String, String) {
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
    for param in query.split('&') {
        let parts: Vec<&str> = param.splitn(2, '=').collect();
        if parts.len() == 2 && parts[0] == key {
            return Some(
                urlencoding::decode(parts[1])
                    .unwrap_or_else(|_| parts[1].into())
                    .to_string(),
            );
        }
    }
    None
}

fn medialive_error_response(err: &MediaLiveError) -> MockResponse {
    let (status, error_type) = match err {
        MediaLiveError::ChannelNotFound { .. } | MediaLiveError::InputNotFound { .. } => {
            (404u16, "NotFoundException")
        }
    };
    let body = format!("{{\"message\":{:?}}}", err.to_string());
    let mut resp = MockResponse::rest_json(status, body);
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = format!("{{\"message\":{:?}}}", message);
    let mut resp = MockResponse::rest_json(status, body);
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
