use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, rest_json_error,
};

use crate::model;
use crate::state::{MediaPackageV2Error, MediaPackageV2State};
use crate::views::MediaPackageV2StateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

/// Convert an RFC 3339 timestamp string into the wire's `f64` epoch seconds.
fn rfc3339_to_epoch(s: &str) -> Option<f64> {
    chrono::DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.timestamp() as f64)
}

pub struct MediaPackageV2Service {
    pub(crate) state: Arc<BackendState<MediaPackageV2State>>,
    pub(crate) notifier: StateChangeNotifier<MediaPackageV2StateView>,
}

impl MediaPackageV2Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for MediaPackageV2Service {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for MediaPackageV2Service {
    fn service_name(&self) -> &str {
        "mediapackagev2"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://mediapackagev2\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl MediaPackageV2Service {
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

        match (method, segments.as_slice()) {
            // POST /channelGroup - CreateChannelGroup
            ("POST", ["channelGroup"]) => {
                let body: Value = match serde_json::from_slice(&request.body) {
                    Ok(v) => v,
                    Err(_) => {
                        return rest_json_error(400, "ValidationException", "Invalid JSON body");
                    }
                };
                self.handle_create_channel_group(&state, &body, account_id, &region)
                    .await
            }
            // GET /channelGroup - ListChannelGroups
            ("GET", ["channelGroup"]) => self.handle_list_channel_groups(&state).await,
            // GET /channelGroup/{ChannelGroupName} - GetChannelGroup
            ("GET", ["channelGroup", channel_group_name]) => {
                self.handle_get_channel_group(&state, channel_group_name)
                    .await
            }
            // DELETE /channelGroup/{ChannelGroupName} - DeleteChannelGroup
            ("DELETE", ["channelGroup", channel_group_name]) => {
                self.handle_delete_channel_group(&state, channel_group_name)
                    .await
            }
            // POST /channelGroup/{ChannelGroupName}/channel - CreateChannel
            ("POST", ["channelGroup", cg_name, "channel"]) => {
                let body: Value = match serde_json::from_slice(&request.body) {
                    Ok(v) => v,
                    Err(_) => {
                        return rest_json_error(400, "ValidationException", "Invalid JSON body");
                    }
                };
                self.handle_create_channel(&state, cg_name, &body, account_id, &region)
                    .await
            }
            // GET /channelGroup/{ChannelGroupName}/channel/{ChannelName} - GetChannel
            ("GET", ["channelGroup", cg_name, "channel", ch_name]) => {
                self.handle_get_channel(&state, cg_name, ch_name).await
            }
            // DELETE /channelGroup/{ChannelGroupName}/channel/{ChannelName} - DeleteChannel
            ("DELETE", ["channelGroup", cg_name, "channel", ch_name]) => {
                self.handle_delete_channel(&state, cg_name, ch_name).await
            }
            // --- Unimplemented operations ---
            // POST /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint => CreateOriginEndpoint (not implemented)
            // GET /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint => ListOriginEndpoints (not implemented)
            // GET /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName} => GetOriginEndpoint (not implemented)
            // PUT /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName} => UpdateOriginEndpoint (not implemented)
            // DELETE /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName} => DeleteOriginEndpoint (not implemented)
            // PUT /channelGroup/{ChannelGroupName}/channel/{ChannelName}/policy => PutChannelPolicy (not implemented)
            // GET /channelGroup/{ChannelGroupName}/channel/{ChannelName}/policy => GetChannelPolicy (not implemented)
            // DELETE /channelGroup/{ChannelGroupName}/channel/{ChannelName}/policy => DeleteChannelPolicy (not implemented)
            // PUT /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}/policy => PutOriginEndpointPolicy (not implemented)
            // GET /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}/policy => GetOriginEndpointPolicy (not implemented)
            // DELETE /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}/policy => DeleteOriginEndpointPolicy (not implemented)
            // POST /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}/harvestJob => CreateHarvestJob (not implemented)
            // GET /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}/harvestJob => ListHarvestJobs (not implemented)
            // GET /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}/harvestJob/{HarvestJobName} => GetHarvestJob (not implemented)
            // PUT /channelGroup/{ChannelGroupName}/channel/{ChannelName}/originEndpoint/{OriginEndpointName}/harvestJob/{HarvestJobName} => CancelHarvestJob (not implemented)
            // PUT /channelGroup/{ChannelGroupName}/channel/{ChannelName} => UpdateChannel (not implemented)
            // PUT /channelGroup/{ChannelGroupName}/channel/{ChannelName}/reset => ResetChannelState (not implemented)
            // POST /tags/{ResourceArn+} => TagResource (not implemented)
            // DELETE /tags/{ResourceArn+} => UntagResource (not implemented)
            // GET /tags/{ResourceArn+} => ListTagsForResource (not implemented)
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_create_channel_group(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageV2State>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let channel_group_name = match body.get("ChannelGroupName").and_then(|v| v.as_str()) {
            Some(name) => name,
            None => {
                return rest_json_error(400, "ValidationException", "ChannelGroupName is required");
            }
        };

        let description = body
            .get("Description")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let tags: HashMap<String, String> = body
            .get("tags")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_channel_group(channel_group_name, description, tags, account_id, region)
        {
            Ok(cg) => {
                let resp = model::CreateChannelGroupResponse {
                    arn: Some(cg.arn.clone()),
                    channel_group_name: Some(cg.channel_group_name.clone()),
                    created_at: rfc3339_to_epoch(&cg.created_at),
                    modified_at: rfc3339_to_epoch(&cg.modified_at),
                    e_tag: Some(cg.e_tag.clone()),
                    egress_domain: Some(cg.egress_domain.clone()),
                    description: Some(cg.description.clone()),
                    tags: Some(cg.tags.clone()),
                };
                wire::serialize_create_channel_group_response(&resp)
            }
            Err(e) => mediapackagev2_error_response(&e),
        }
    }

    async fn handle_get_channel_group(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageV2State>>,
        channel_group_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_channel_group(channel_group_name) {
            Ok(cg) => {
                let resp = model::GetChannelGroupResponse {
                    arn: Some(cg.arn.clone()),
                    channel_group_name: Some(cg.channel_group_name.clone()),
                    created_at: rfc3339_to_epoch(&cg.created_at),
                    modified_at: rfc3339_to_epoch(&cg.modified_at),
                    e_tag: Some(cg.e_tag.clone()),
                    egress_domain: Some(cg.egress_domain.clone()),
                    description: Some(cg.description.clone()),
                    tags: Some(cg.tags.clone()),
                };
                wire::serialize_get_channel_group_response(&resp)
            }
            Err(e) => mediapackagev2_error_response(&e),
        }
    }

    async fn handle_delete_channel_group(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageV2State>>,
        channel_group_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_channel_group(channel_group_name) {
            Ok(()) => {
                wire::serialize_delete_channel_group_response(&model::DeleteChannelGroupResponse {})
            }
            Err(e) => mediapackagev2_error_response(&e),
        }
    }

    async fn handle_create_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageV2State>>,
        channel_group_name: &str,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let channel_name = match body.get("ChannelName").and_then(|v| v.as_str()) {
            Some(name) => name,
            None => {
                return rest_json_error(400, "ValidationException", "ChannelName is required");
            }
        };

        let description = body
            .get("Description")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let tags: HashMap<String, String> = body
            .get("tags")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_channel(
            channel_group_name,
            channel_name,
            description,
            tags,
            account_id,
            region,
        ) {
            Ok(ch) => {
                let resp = model::CreateChannelResponse {
                    arn: Some(ch.arn.clone()),
                    channel_group_name: Some(ch.channel_group_name.clone()),
                    channel_name: Some(ch.channel_name.clone()),
                    created_at: rfc3339_to_epoch(&ch.created_at),
                    modified_at: rfc3339_to_epoch(&ch.modified_at),
                    e_tag: Some(ch.e_tag.clone()),
                    description: Some(ch.description.clone()),
                    tags: Some(ch.tags.clone()),
                    ..Default::default()
                };
                wire::serialize_create_channel_response(&resp)
            }
            Err(e) => mediapackagev2_error_response(&e),
        }
    }

    async fn handle_get_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageV2State>>,
        channel_group_name: &str,
        channel_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_channel(channel_group_name, channel_name) {
            Ok(ch) => {
                let resp = model::GetChannelResponse {
                    arn: Some(ch.arn.clone()),
                    channel_group_name: Some(ch.channel_group_name.clone()),
                    channel_name: Some(ch.channel_name.clone()),
                    created_at: rfc3339_to_epoch(&ch.created_at),
                    modified_at: rfc3339_to_epoch(&ch.modified_at),
                    e_tag: Some(ch.e_tag.clone()),
                    description: Some(ch.description.clone()),
                    tags: Some(ch.tags.clone()),
                    ..Default::default()
                };
                wire::serialize_get_channel_response(&resp)
            }
            Err(e) => mediapackagev2_error_response(&e),
        }
    }

    async fn handle_delete_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageV2State>>,
        channel_group_name: &str,
        channel_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_channel(channel_group_name, channel_name) {
            Ok(()) => wire::serialize_delete_channel_response(&model::DeleteChannelResponse {}),
            Err(e) => mediapackagev2_error_response(&e),
        }
    }

    async fn handle_list_channel_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaPackageV2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let items: Vec<model::ChannelGroupListConfiguration> = state
            .list_channel_groups()
            .iter()
            .map(|cg| model::ChannelGroupListConfiguration {
                arn: Some(cg.arn.clone()),
                channel_group_name: Some(cg.channel_group_name.clone()),
                created_at: rfc3339_to_epoch(&cg.created_at),
                modified_at: rfc3339_to_epoch(&cg.modified_at),
                description: Some(cg.description.clone()),
            })
            .collect();
        let resp = model::ListChannelGroupsResponse {
            items: Some(items),
            ..Default::default()
        };
        wire::serialize_list_channel_groups_response(&resp)
    }
}

fn mediapackagev2_error_response(err: &MediaPackageV2Error) -> MockResponse {
    let (status, error_type) = match err {
        MediaPackageV2Error::ChannelGroupAlreadyExists(_) => (409, "ConflictException"),
        MediaPackageV2Error::ChannelAlreadyExists(_) => (409, "ConflictException"),
        MediaPackageV2Error::ChannelGroupNotFound(_) => (404, "ResourceNotFoundException"),
        MediaPackageV2Error::ChannelNotFound(_) => (404, "ResourceNotFoundException"),
    };
    let body = json!({
        "Type": "User",
        "Message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
