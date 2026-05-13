use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{KinesisVideoError, KinesisVideoState};
use crate::types::{
    DeletionConfig, EdgeConfigState, ImageGenerationConfig, NotificationConfig, RecorderConfig,
    StreamStorageConfig, UploaderConfig,
};
use crate::views::KinesisVideoStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct KinesisVideoService {
    pub(crate) state: Arc<BackendState<KinesisVideoState>>,
    pub(crate) notifier: StateChangeNotifier<KinesisVideoStateView>,
}

impl KinesisVideoService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for KinesisVideoService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for KinesisVideoService {
    fn service_name(&self) -> &str {
        "kinesisvideo"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://kinesisvideo\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl KinesisVideoService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let query = extract_query(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&query);

        let is_mutating = matches!(
            path.as_str(),
            "/createStream"
                | "/deleteStream"
                | "/updateStream"
                | "/updateDataRetention"
                | "/updateImageGenerationConfiguration"
                | "/updateNotificationConfiguration"
                | "/updateStreamStorageConfiguration"
                | "/tagStream"
                | "/untagStream"
                | "/startEdgeConfigurationUpdate"
                | "/deleteEdgeConfiguration"
                | "/createSignalingChannel"
                | "/deleteSignalingChannel"
                | "/updateSignalingChannel"
                | "/TagResource"
                | "/UntagResource"
                | "/updateMediaStorageConfiguration"
        );

        let response = match path.as_str() {
            // Stream CRUD
            "/createStream" => {
                self.handle_create_stream(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            "/describeStream" => {
                self.handle_describe_stream(&state, &request, &[], &query_map)
                    .await
            }
            "/deleteStream" => {
                self.handle_delete_stream(&state, &request, &[], &query_map)
                    .await
            }
            "/listStreams" => {
                self.handle_list_streams(&state, &request, &[], &query_map)
                    .await
            }
            "/updateStream" => {
                self.handle_update_stream(&state, &request, &[], &query_map)
                    .await
            }
            "/updateDataRetention" => {
                self.handle_update_data_retention(&state, &request, &[], &query_map)
                    .await
            }
            "/getDataEndpoint" => {
                self.handle_get_data_endpoint(&state, &request, &[], &query_map)
                    .await
            }

            // Stream tags
            "/tagStream" => {
                self.handle_tag_stream(&state, &request, &[], &query_map)
                    .await
            }
            "/untagStream" => {
                self.handle_untag_stream(&state, &request, &[], &query_map)
                    .await
            }
            "/listTagsForStream" => {
                self.handle_list_tags_for_stream(&state, &request, &[], &query_map)
                    .await
            }

            // Resource (channel) tags
            "/TagResource" => {
                self.handle_tag_resource(&state, &request, &[], &query_map)
                    .await
            }
            "/UntagResource" => {
                self.handle_untag_resource(&state, &request, &[], &query_map)
                    .await
            }
            "/ListTagsForResource" => {
                self.handle_list_tags_for_resource(&state, &request, &[], &query_map)
                    .await
            }

            // Stream configurations
            "/describeImageGenerationConfiguration" => {
                self.handle_describe_image_generation_configuration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                )
                .await
            }
            "/updateImageGenerationConfiguration" => {
                self.handle_update_image_generation_configuration(&state, &request, &[], &query_map)
                    .await
            }
            "/describeNotificationConfiguration" => {
                self.handle_describe_notification_configuration(&state, &request, &[], &query_map)
                    .await
            }
            "/updateNotificationConfiguration" => {
                self.handle_update_notification_configuration(&state, &request, &[], &query_map)
                    .await
            }
            "/describeStreamStorageConfiguration" => {
                self.handle_describe_stream_storage_configuration(&state, &request, &[], &query_map)
                    .await
            }
            "/updateStreamStorageConfiguration" => {
                self.handle_update_stream_storage_configuration(&state, &request, &[], &query_map)
                    .await
            }
            "/describeMappedResourceConfiguration" => {
                self.handle_describe_mapped_resource_configuration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                )
                .await
            }

            // Edge configurations
            "/startEdgeConfigurationUpdate" => {
                self.handle_start_edge_configuration_update(&state, &request, &[], &query_map)
                    .await
            }
            "/describeEdgeConfiguration" => {
                self.handle_describe_edge_configuration(&state, &request, &[], &query_map)
                    .await
            }
            "/deleteEdgeConfiguration" => {
                self.handle_delete_edge_configuration(&state, &request, &[], &query_map)
                    .await
            }
            "/listEdgeAgentConfigurations" => {
                self.handle_list_edge_agent_configurations(&state, &request, &[], &query_map)
                    .await
            }

            // Signaling channels
            "/createSignalingChannel" => {
                self.handle_create_signaling_channel(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            "/describeSignalingChannel" => {
                self.handle_describe_signaling_channel(&state, &request, &[], &query_map)
                    .await
            }
            "/deleteSignalingChannel" => {
                self.handle_delete_signaling_channel(&state, &request, &[], &query_map)
                    .await
            }
            "/listSignalingChannels" => {
                self.handle_list_signaling_channels(&state, &request, &[], &query_map)
                    .await
            }
            "/updateSignalingChannel" => {
                self.handle_update_signaling_channel(&state, &request, &[], &query_map)
                    .await
            }
            "/getSignalingChannelEndpoint" => {
                self.handle_get_signaling_channel_endpoint(&state, &request, &[], &query_map)
                    .await
            }

            // Media storage
            "/describeMediaStorageConfiguration" => {
                self.handle_describe_media_storage_configuration(&state, &request, &[], &query_map)
                    .await
            }
            "/updateMediaStorageConfiguration" => {
                self.handle_update_media_storage_configuration(&state, &request, &[], &query_map)
                    .await
            }

            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        if is_mutating && response.status / 100 == 2 {
            use winterbaume_core::StatefulService;
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    // =========================================================================
    // Stream CRUD
    // =========================================================================

    async fn handle_create_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_stream_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        if input.stream_name.is_empty() {
            return rest_json_error(400, "InvalidArgumentException", "StreamName is required");
        }

        let device_name = input.device_name.as_deref();
        let media_type = input.media_type.as_deref();
        let kms_key_id = input.kms_key_id.as_deref();
        let data_retention_in_hours = input.data_retention_in_hours;

        let mut state = state.write().await;
        match state.create_stream(
            &input.stream_name,
            device_name,
            media_type,
            kms_key_id,
            data_retention_in_hours,
            account_id,
            region,
        ) {
            Ok(stream) => wire::serialize_create_stream_response(&wire::CreateStreamOutput {
                stream_a_r_n: Some(stream.stream_arn.clone()),
            }),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_describe_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_stream_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let state = state.read().await;
        match state.describe_stream(stream_name, stream_arn) {
            Ok(stream) => wire::serialize_describe_stream_response(&wire::DescribeStreamOutput {
                stream_info: Some(stream_to_wire(stream)),
            }),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_delete_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_stream_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        if input.stream_a_r_n.is_empty() {
            return rest_json_error(400, "InvalidArgumentException", "StreamARN is required");
        }
        let current_version = input.current_version.as_deref();

        let mut state = state.write().await;
        match state.delete_stream(&input.stream_a_r_n, current_version) {
            Ok(()) => wire::serialize_delete_stream_response(&wire::DeleteStreamOutput {}),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_list_streams(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_streams_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let state = state.read().await;
        let streams = state.list_streams();
        let stream_info_list: Vec<wire::StreamInfo> =
            streams.iter().map(|s| stream_to_wire(s)).collect();
        wire::serialize_list_streams_response(&wire::ListStreamsOutput {
            stream_info_list: Some(stream_info_list),
            next_token: None,
        })
    }

    async fn handle_update_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_stream_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        if input.current_version.is_empty() {
            return rest_json_error(
                400,
                "InvalidArgumentException",
                "CurrentVersion is required",
            );
        }

        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        let device_name = input.device_name.as_deref();
        let media_type = input.media_type.as_deref();

        let mut state = state.write().await;
        match state.update_stream(
            stream_name,
            stream_arn,
            &input.current_version,
            device_name,
            media_type,
        ) {
            Ok(()) => wire::serialize_update_stream_response(&wire::UpdateStreamOutput {}),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_update_data_retention(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_data_retention_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        if input.current_version.is_empty() {
            return rest_json_error(
                400,
                "InvalidArgumentException",
                "CurrentVersion is required",
            );
        }
        if input.operation.is_empty() {
            return rest_json_error(400, "InvalidArgumentException", "Operation is required");
        }
        // The wire model lacks an Option wrapper for `data_retention_change_in_hours`; it
        // defaults to 0 when absent. Mirror legacy behaviour by treating 0 as "missing".
        if input.data_retention_change_in_hours == 0 {
            return rest_json_error(
                400,
                "InvalidArgumentException",
                "DataRetentionChangeInHours is required",
            );
        }

        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let mut state = state.write().await;
        match state.update_data_retention(
            stream_name,
            stream_arn,
            &input.current_version,
            &input.operation,
            input.data_retention_change_in_hours,
        ) {
            Ok(()) => {
                wire::serialize_update_data_retention_response(&wire::UpdateDataRetentionOutput {})
            }
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_get_data_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_data_endpoint_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        if input.a_p_i_name.is_empty() {
            return rest_json_error(400, "InvalidArgumentException", "APIName is required");
        }
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let state = state.read().await;
        match state.get_data_endpoint(stream_name, stream_arn, &input.a_p_i_name) {
            Ok(endpoint) => {
                wire::serialize_get_data_endpoint_response(&wire::GetDataEndpointOutput {
                    data_endpoint: Some(endpoint),
                })
            }
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    // =========================================================================
    // Stream tags
    // =========================================================================

    async fn handle_tag_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_stream_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let tags = input.tags.clone();
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let mut state = state.write().await;
        match state.tag_stream(stream_name, stream_arn, tags) {
            Ok(()) => wire::serialize_tag_stream_response(&wire::TagStreamOutput {}),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_untag_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_stream_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let tag_keys = input.tag_key_list.clone();
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let mut state = state.write().await;
        match state.untag_stream(stream_name, stream_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_stream_response(&wire::UntagStreamOutput {}),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_list_tags_for_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_stream_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let state = state.read().await;
        match state.list_tags_for_stream(stream_name, stream_arn) {
            Ok(tags) => {
                wire::serialize_list_tags_for_stream_response(&wire::ListTagsForStreamOutput {
                    tags: Some(tags),
                    next_token: None,
                })
            }
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    // =========================================================================
    // Resource (channel) tags
    // =========================================================================

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return rest_json_error(400, "InvalidArgumentException", "ResourceARN is required");
        }
        let tags: Vec<(String, String)> = input
            .tags
            .iter()
            .map(|t| (t.key.clone(), t.value.clone()))
            .collect();

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_a_r_n, &tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceOutput {}),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return rest_json_error(400, "InvalidArgumentException", "ResourceARN is required");
        }
        let tag_keys = input.tag_key_list.clone();

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_a_r_n, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceOutput {}),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return rest_json_error(400, "InvalidArgumentException", "ResourceARN is required");
        }

        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_a_r_n) {
            Ok(tags) => {
                wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceOutput {
                    tags: Some(tags),
                    next_token: None,
                })
            }
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    // =========================================================================
    // Stream configuration operations
    // =========================================================================

    async fn handle_describe_image_generation_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_image_generation_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let state = state.read().await;
        match state.describe_image_generation_configuration(stream_name, stream_arn) {
            Ok(config) => wire::serialize_describe_image_generation_configuration_response(
                &wire::DescribeImageGenerationConfigurationOutput {
                    image_generation_configuration: config.map(|c| {
                        wire::ImageGenerationConfiguration {
                            status: c.status.clone(),
                            image_selector_type: c.image_selector_type.clone(),
                            destination_config: wire::ImageGenerationDestinationConfig {
                                uri: c.destination_uri.clone(),
                                destination_region: c.destination_region.clone(),
                            },
                            sampling_interval: c.sampling_interval,
                            format: c.format.clone(),
                            format_config: if c.format_config.is_empty() {
                                None
                            } else {
                                Some(c.format_config.clone())
                            },
                            height_pixels: c.height_pixels,
                            width_pixels: c.width_pixels,
                        }
                    }),
                },
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_update_image_generation_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_image_generation_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let config = input.image_generation_configuration.map(|c| {
            // Provide the same defaults the legacy hand-written parser used.
            let image_selector_type = if c.image_selector_type.is_empty() {
                "SERVER_TIMESTAMP".to_string()
            } else {
                c.image_selector_type.clone()
            };
            let destination_region = if c.destination_config.destination_region.is_empty() {
                "us-east-1".to_string()
            } else {
                c.destination_config.destination_region.clone()
            };
            let format = if c.format.is_empty() {
                "JPEG".to_string()
            } else {
                c.format.clone()
            };
            let sampling_interval = if c.sampling_interval == 0 {
                3000
            } else {
                c.sampling_interval
            };
            let format_config = c.format_config.clone().unwrap_or_default();
            ImageGenerationConfig {
                status: c.status,
                image_selector_type,
                destination_uri: c.destination_config.uri,
                destination_region,
                sampling_interval,
                format,
                format_config,
                height_pixels: c.height_pixels,
                width_pixels: c.width_pixels,
            }
        });

        let mut state = state.write().await;
        match state.update_image_generation_configuration(stream_name, stream_arn, config) {
            Ok(()) => wire::serialize_update_image_generation_configuration_response(
                &wire::UpdateImageGenerationConfigurationOutput {},
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_describe_notification_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_notification_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let state = state.read().await;
        match state.describe_notification_configuration(stream_name, stream_arn) {
            Ok(config) => wire::serialize_describe_notification_configuration_response(
                &wire::DescribeNotificationConfigurationOutput {
                    notification_configuration: config.map(|c| wire::NotificationConfiguration {
                        status: c.status.clone(),
                        destination_config: wire::NotificationDestinationConfig {
                            uri: c.destination_uri.clone(),
                        },
                    }),
                },
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_update_notification_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_notification_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let config = input.notification_configuration.map(|nc| {
            let status = if nc.status.is_empty() {
                "DISABLED".to_string()
            } else {
                nc.status
            };
            NotificationConfig {
                status,
                destination_uri: nc.destination_config.uri,
            }
        });

        let mut state = state.write().await;
        match state.update_notification_configuration(stream_name, stream_arn, config) {
            Ok(()) => wire::serialize_update_notification_configuration_response(
                &wire::UpdateNotificationConfigurationOutput {},
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_describe_stream_storage_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_stream_storage_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let state = state.read().await;
        match state.describe_stream_storage_configuration(stream_name, stream_arn) {
            Ok((stream, config)) => wire::serialize_describe_stream_storage_configuration_response(
                &wire::DescribeStreamStorageConfigurationOutput {
                    stream_a_r_n: Some(stream.stream_arn.clone()),
                    stream_name: Some(stream.stream_name.clone()),
                    stream_storage_configuration: config.map(|c| {
                        wire::StreamStorageConfiguration {
                            default_storage_tier: c.default_storage_tier.clone(),
                        }
                    }),
                },
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_update_stream_storage_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_stream_storage_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        if input.current_version.is_empty() {
            return rest_json_error(
                400,
                "InvalidArgumentException",
                "CurrentVersion is required",
            );
        }

        let default_tier = if input
            .stream_storage_configuration
            .default_storage_tier
            .is_empty()
        {
            "HOT".to_string()
        } else {
            input.stream_storage_configuration.default_storage_tier
        };

        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let mut state = state.write().await;
        match state.update_stream_storage_configuration(
            stream_name,
            stream_arn,
            &input.current_version,
            StreamStorageConfig {
                default_storage_tier: default_tier,
            },
        ) {
            Ok(()) => wire::serialize_update_stream_storage_configuration_response(
                &wire::UpdateStreamStorageConfigurationOutput {},
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_describe_mapped_resource_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_mapped_resource_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let state = state.read().await;
        match state.describe_mapped_resource_configuration(stream_name, stream_arn) {
            Ok(items) => {
                let list = items
                    .iter()
                    .map(|(type_, arn)| wire::MappedResourceConfigurationListItem {
                        r#type: Some(type_.clone()),
                        a_r_n: Some(arn.clone()),
                    })
                    .collect::<Vec<_>>();
                wire::serialize_describe_mapped_resource_configuration_response(
                    &wire::DescribeMappedResourceConfigurationOutput {
                        mapped_resource_configuration_list: if list.is_empty() {
                            None
                        } else {
                            Some(list)
                        },
                        next_token: None,
                    },
                )
            }
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    // =========================================================================
    // Edge configuration operations
    // =========================================================================

    async fn handle_start_edge_configuration_update(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_start_edge_configuration_update_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
            };
        let ec = input.edge_config;
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let recorder_config = {
            let rc = &ec.recorder_config;
            // Treat "no media source provided" as null — the legacy code created a recorder
            // config only if RecorderConfig was present in the JSON.
            if rc.media_source_config.media_uri_secret_arn.is_empty()
                && rc.media_source_config.media_uri_type.is_empty()
                && rc.schedule_config.is_none()
            {
                None
            } else {
                Some(RecorderConfig {
                    media_uri_secret_arn: rc.media_source_config.media_uri_secret_arn.clone(),
                    media_uri_type: if rc.media_source_config.media_uri_type.is_empty() {
                        "RTSP_URI".to_string()
                    } else {
                        rc.media_source_config.media_uri_type.clone()
                    },
                    schedule_expression: rc
                        .schedule_config
                        .as_ref()
                        .map(|sc| sc.schedule_expression.clone()),
                    duration_in_seconds: rc
                        .schedule_config
                        .as_ref()
                        .map(|sc| sc.duration_in_seconds),
                })
            }
        };

        let uploader_config = ec.uploader_config.as_ref().map(|uc| UploaderConfig {
            schedule_expression: uc.schedule_config.schedule_expression.clone(),
            duration_in_seconds: uc.schedule_config.duration_in_seconds,
        });

        let deletion_config = ec.deletion_config.as_ref().map(|dc| DeletionConfig {
            delete_after_upload: dc.delete_after_upload,
            edge_retention_in_hours: dc.edge_retention_in_hours,
            max_local_media_size_in_mb: dc
                .local_size_config
                .as_ref()
                .and_then(|lsc| lsc.max_local_media_size_in_m_b),
            strategy_on_full_size: dc
                .local_size_config
                .as_ref()
                .and_then(|lsc| lsc.strategy_on_full_size.clone()),
        });

        let now = chrono::Utc::now();
        let edge_state = EdgeConfigState {
            hub_device_arn: ec.hub_device_arn,
            sync_status: "SYNCING".to_string(),
            creation_time: now,
            last_updated_time: now,
            failed_status_details: None,
            recorder_config: recorder_config.unwrap_or_else(|| RecorderConfig {
                media_uri_secret_arn: String::new(),
                media_uri_type: "RTSP_URI".to_string(),
                schedule_expression: None,
                duration_in_seconds: None,
            }),
            uploader_config,
            deletion_config,
        };

        let mut state = state.write().await;
        match state.start_edge_configuration_update(stream_name, stream_arn, edge_state) {
            Ok(ec) => wire::serialize_start_edge_configuration_update_response(
                &wire::StartEdgeConfigurationUpdateOutput {
                    stream_a_r_n: stream_arn.map(String::from),
                    stream_name: stream_name.map(String::from),
                    sync_status: Some(ec.sync_status.clone()),
                    creation_time: Some(ec.creation_time.timestamp() as f64),
                    last_updated_time: Some(ec.last_updated_time.timestamp() as f64),
                    failed_status_details: ec.failed_status_details.clone(),
                    edge_config: Some(edge_config_state_to_wire(ec)),
                },
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_describe_edge_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_edge_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
            };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let state = state.read().await;
        match state.describe_edge_configuration(stream_name, stream_arn) {
            Ok((stream, ec)) => wire::serialize_describe_edge_configuration_response(
                &wire::DescribeEdgeConfigurationOutput {
                    stream_a_r_n: Some(stream.stream_arn.clone()),
                    stream_name: Some(stream.stream_name.clone()),
                    sync_status: Some(ec.sync_status.clone()),
                    creation_time: Some(ec.creation_time.timestamp() as f64),
                    last_updated_time: Some(ec.last_updated_time.timestamp() as f64),
                    failed_status_details: ec.failed_status_details.clone(),
                    edge_config: Some(edge_config_state_to_wire(ec)),
                    edge_agent_status: None,
                },
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_delete_edge_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_edge_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
            };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let mut state = state.write().await;
        match state.delete_edge_configuration(stream_name, stream_arn) {
            Ok(()) => wire::serialize_delete_edge_configuration_response(
                &wire::DeleteEdgeConfigurationOutput {},
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_list_edge_agent_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_edge_agent_configurations_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };

        let state = state.read().await;
        let configs = state.list_edge_agent_configurations(&input.hub_device_arn);

        let edge_configs: Vec<wire::ListEdgeAgentConfigurationsEdgeConfig> = configs
            .iter()
            .map(|(stream, ec)| wire::ListEdgeAgentConfigurationsEdgeConfig {
                stream_a_r_n: Some(stream.stream_arn.clone()),
                stream_name: Some(stream.stream_name.clone()),
                sync_status: Some(ec.sync_status.clone()),
                creation_time: Some(ec.creation_time.timestamp() as f64),
                last_updated_time: Some(ec.last_updated_time.timestamp() as f64),
                failed_status_details: ec.failed_status_details.clone(),
                edge_config: Some(edge_config_state_to_wire(ec)),
            })
            .collect();

        wire::serialize_list_edge_agent_configurations_response(
            &wire::ListEdgeAgentConfigurationsOutput {
                edge_configs: if edge_configs.is_empty() {
                    None
                } else {
                    Some(edge_configs)
                },
                next_token: None,
            },
        )
    }

    // =========================================================================
    // Signaling channel operations
    // =========================================================================

    async fn handle_create_signaling_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_signaling_channel_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        if input.channel_name.is_empty() {
            return rest_json_error(400, "InvalidArgumentException", "ChannelName is required");
        }

        let channel_type = input.channel_type.as_deref();
        let message_ttl_seconds = input
            .single_master_configuration
            .as_ref()
            .and_then(|smc| smc.message_ttl_seconds);

        let tags: Vec<(String, String)> = input
            .tags
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|t| (t.key.clone(), t.value.clone()))
            .collect();

        let mut state = state.write().await;
        match state.create_signaling_channel(
            &input.channel_name,
            channel_type,
            message_ttl_seconds,
            tags,
            account_id,
            region,
        ) {
            Ok(channel) => wire::serialize_create_signaling_channel_response(
                &wire::CreateSignalingChannelOutput {
                    channel_a_r_n: Some(channel.channel_arn.clone()),
                },
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_describe_signaling_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_signaling_channel_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
            };
        let channel_name = input.channel_name.as_deref();
        let channel_arn = input.channel_a_r_n.as_deref();

        let state = state.read().await;
        match state.describe_signaling_channel(channel_name, channel_arn) {
            Ok(channel) => wire::serialize_describe_signaling_channel_response(
                &wire::DescribeSignalingChannelOutput {
                    channel_info: Some(channel_to_wire(channel)),
                },
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_delete_signaling_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_signaling_channel_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        if input.channel_a_r_n.is_empty() {
            return rest_json_error(400, "InvalidArgumentException", "ChannelARN is required");
        }
        let current_version = input.current_version.as_deref();

        let mut state = state.write().await;
        match state.delete_signaling_channel(&input.channel_a_r_n, current_version) {
            Ok(()) => wire::serialize_delete_signaling_channel_response(
                &wire::DeleteSignalingChannelOutput {},
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_list_signaling_channels(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_signaling_channels_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let state = state.read().await;
        let channels = state.list_signaling_channels();
        let channel_info_list: Vec<wire::ChannelInfo> =
            channels.iter().map(|c| channel_to_wire(c)).collect();
        wire::serialize_list_signaling_channels_response(&wire::ListSignalingChannelsOutput {
            channel_info_list: Some(channel_info_list),
            next_token: None,
        })
    }

    async fn handle_update_signaling_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_signaling_channel_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        if input.channel_a_r_n.is_empty() {
            return rest_json_error(400, "InvalidArgumentException", "ChannelARN is required");
        }
        if input.current_version.is_empty() {
            return rest_json_error(
                400,
                "InvalidArgumentException",
                "CurrentVersion is required",
            );
        }
        let message_ttl_seconds = input
            .single_master_configuration
            .as_ref()
            .and_then(|smc| smc.message_ttl_seconds);

        let mut state = state.write().await;
        match state.update_signaling_channel(
            &input.channel_a_r_n,
            &input.current_version,
            message_ttl_seconds,
        ) {
            Ok(()) => wire::serialize_update_signaling_channel_response(
                &wire::UpdateSignalingChannelOutput {},
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_get_signaling_channel_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_signaling_channel_endpoint_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        if input.channel_a_r_n.is_empty() {
            return rest_json_error(400, "InvalidArgumentException", "ChannelARN is required");
        }
        let protocols = input
            .single_master_channel_endpoint_configuration
            .as_ref()
            .and_then(|smc| smc.protocols.clone());
        let role = input
            .single_master_channel_endpoint_configuration
            .as_ref()
            .and_then(|smc| smc.role.as_deref());

        let state = state.read().await;
        match state.get_signaling_channel_endpoint(&input.channel_a_r_n, protocols.as_deref(), role)
        {
            Ok(endpoints) => {
                let endpoint_list: Vec<wire::ResourceEndpointListItem> = endpoints
                    .iter()
                    .map(|(protocol, endpoint)| wire::ResourceEndpointListItem {
                        protocol: Some(protocol.clone()),
                        resource_endpoint: Some(endpoint.clone()),
                    })
                    .collect();
                wire::serialize_get_signaling_channel_endpoint_response(
                    &wire::GetSignalingChannelEndpointOutput {
                        resource_endpoint_list: Some(endpoint_list),
                    },
                )
            }
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    // =========================================================================
    // Media storage configuration
    // =========================================================================

    async fn handle_describe_media_storage_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_media_storage_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let channel_name = input.channel_name.as_deref();
        let channel_arn = input.channel_a_r_n.as_deref();

        let state = state.read().await;
        match state.describe_media_storage_configuration(channel_name, channel_arn) {
            Ok(config) => wire::serialize_describe_media_storage_configuration_response(
                &wire::DescribeMediaStorageConfigurationOutput {
                    media_storage_configuration: config.map(|c| wire::MediaStorageConfiguration {
                        status: c.status.clone(),
                        stream_a_r_n: c.stream_arn.clone(),
                    }),
                },
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }

    async fn handle_update_media_storage_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_media_storage_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        if input.channel_a_r_n.is_empty() {
            return rest_json_error(400, "InvalidArgumentException", "ChannelARN is required");
        }

        let status = if input.media_storage_configuration.status.is_empty() {
            "DISABLED".to_string()
        } else {
            input.media_storage_configuration.status
        };
        let stream_arn = input.media_storage_configuration.stream_a_r_n;

        let mut state = state.write().await;
        match state.update_media_storage_configuration(
            &input.channel_a_r_n,
            &status,
            stream_arn.as_deref(),
        ) {
            Ok(()) => wire::serialize_update_media_storage_configuration_response(
                &wire::UpdateMediaStorageConfigurationOutput {},
            ),
            Err(e) => kinesis_video_error_response(&e),
        }
    }
}

// =========================================================================
// Wire conversion helpers
// =========================================================================

fn stream_to_wire(stream: &crate::types::Stream) -> wire::StreamInfo {
    wire::StreamInfo {
        stream_name: Some(stream.stream_name.clone()),
        stream_a_r_n: Some(stream.stream_arn.clone()),
        kms_key_id: Some(stream.kms_key_id.clone()),
        version: Some(stream.version.clone()),
        status: Some(stream.status.clone()),
        creation_time: Some(stream.creation_time.timestamp() as f64),
        data_retention_in_hours: Some(stream.data_retention_in_hours),
        device_name: stream.device_name.clone(),
        media_type: stream.media_type.clone(),
    }
}

fn channel_to_wire(channel: &crate::types::SignalingChannel) -> wire::ChannelInfo {
    wire::ChannelInfo {
        channel_name: Some(channel.channel_name.clone()),
        channel_a_r_n: Some(channel.channel_arn.clone()),
        channel_type: Some(channel.channel_type.clone()),
        channel_status: Some(channel.channel_status.clone()),
        creation_time: Some(channel.creation_time.timestamp() as f64),
        version: Some(channel.version.clone()),
        single_master_configuration: channel.message_ttl_seconds.map(|ttl| {
            wire::SingleMasterConfiguration {
                message_ttl_seconds: Some(ttl),
            }
        }),
    }
}

fn edge_config_state_to_wire(ec: &EdgeConfigState) -> wire::EdgeConfig {
    wire::EdgeConfig {
        hub_device_arn: ec.hub_device_arn.clone(),
        recorder_config: wire::RecorderConfig {
            media_source_config: wire::MediaSourceConfig {
                media_uri_secret_arn: ec.recorder_config.media_uri_secret_arn.clone(),
                media_uri_type: ec.recorder_config.media_uri_type.clone(),
            },
            schedule_config: ec.recorder_config.schedule_expression.as_ref().map(|se| {
                wire::ScheduleConfig {
                    schedule_expression: se.clone(),
                    duration_in_seconds: ec.recorder_config.duration_in_seconds.unwrap_or(0),
                }
            }),
        },
        uploader_config: ec.uploader_config.as_ref().map(|uc| wire::UploaderConfig {
            schedule_config: wire::ScheduleConfig {
                schedule_expression: uc.schedule_expression.clone(),
                duration_in_seconds: uc.duration_in_seconds,
            },
        }),
        deletion_config: ec.deletion_config.as_ref().map(|dc| wire::DeletionConfig {
            delete_after_upload: dc.delete_after_upload,
            edge_retention_in_hours: dc.edge_retention_in_hours,
            local_size_config: if dc.max_local_media_size_in_mb.is_some()
                || dc.strategy_on_full_size.is_some()
            {
                Some(wire::LocalSizeConfig {
                    max_local_media_size_in_m_b: dc.max_local_media_size_in_mb,
                    strategy_on_full_size: dc.strategy_on_full_size.clone(),
                })
            } else {
                None
            },
        }),
    }
}

// =========================================================================
// Error response helpers
// =========================================================================

fn extract_path(uri: &str) -> String {
    if let Some(idx) = uri.find("amazonaws.com") {
        let after_host = &uri[idx + "amazonaws.com".len()..];
        if let Some(q) = after_host.find('?') {
            after_host[..q].to_string()
        } else {
            after_host.to_string()
        }
    } else {
        uri.split('?').next().unwrap_or(uri).to_string()
    }
}

fn extract_query(uri: &str) -> String {
    if let Some(idx) = uri.find('?') {
        uri[idx + 1..].to_string()
    } else {
        String::new()
    }
}

fn kinesis_video_error_response(err: &KinesisVideoError) -> MockResponse {
    let (status, error_type) = match err {
        KinesisVideoError::StreamAlreadyExists(_) => (400, "ResourceInUseException"),
        KinesisVideoError::ChannelAlreadyExists(_) => (400, "ResourceInUseException"),
        KinesisVideoError::StreamNotFoundByName(_) => (404, "ResourceNotFoundException"),
        KinesisVideoError::StreamNotFoundByArn(_) => (404, "ResourceNotFoundException"),
        KinesisVideoError::ChannelNotFoundByName(_) => (404, "ResourceNotFoundException"),
        KinesisVideoError::ChannelNotFoundByArn(_) => (404, "ResourceNotFoundException"),
        KinesisVideoError::ResourceNotFoundByArn(_) => (404, "ResourceNotFoundException"),
        KinesisVideoError::EdgeConfigNotFound => (404, "ResourceNotFoundException"),
        KinesisVideoError::StreamIdentifierRequired => (400, "InvalidArgumentException"),
        KinesisVideoError::ChannelIdentifierRequired => (400, "InvalidArgumentException"),
        KinesisVideoError::UnknownOperation(_) => (400, "InvalidArgumentException"),
        KinesisVideoError::StreamVersionMismatch { .. } => (400, "VersionMismatchException"),
        KinesisVideoError::ChannelVersionMismatch { .. } => (400, "VersionMismatchException"),
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

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Type": "User",
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
