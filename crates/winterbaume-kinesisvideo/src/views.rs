//! Serde-compatible view types for KinesisVideo state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::KinesisVideoService;
use crate::state::KinesisVideoState;
use crate::types::{
    DeletionConfig, EdgeConfigState, ImageGenerationConfig, MediaStorageConfig, NotificationConfig,
    RecorderConfig, SignalingChannel, Stream, StreamStorageConfig, UploaderConfig,
};

// ---------------------------------------------------------------------------
// View types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KinesisVideoStateView {
    #[serde(default)]
    pub streams: HashMap<String, StreamView>,
    #[serde(default)]
    pub channels: HashMap<String, SignalingChannelView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamView {
    pub stream_name: String,
    pub stream_arn: String,
    pub device_name: Option<String>,
    pub media_type: Option<String>,
    pub kms_key_id: String,
    pub version: String,
    pub status: String,
    pub creation_time: String,
    pub data_retention_in_hours: i32,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub image_generation_config: Option<ImageGenerationConfigView>,
    pub notification_config: Option<NotificationConfigView>,
    pub storage_config: Option<StreamStorageConfigView>,
    pub edge_config: Option<EdgeConfigStateView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalingChannelView {
    pub channel_name: String,
    pub channel_arn: String,
    pub channel_type: String,
    pub channel_status: String,
    pub creation_time: String,
    pub version: String,
    pub message_ttl_seconds: Option<i32>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub media_storage_config: Option<MediaStorageConfigView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGenerationConfigView {
    pub status: String,
    pub image_selector_type: String,
    pub destination_uri: String,
    pub destination_region: String,
    pub sampling_interval: i32,
    pub format: String,
    pub format_config: HashMap<String, String>,
    pub height_pixels: Option<i32>,
    pub width_pixels: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfigView {
    pub status: String,
    pub destination_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamStorageConfigView {
    pub default_storage_tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeConfigStateView {
    pub hub_device_arn: String,
    pub sync_status: String,
    pub creation_time: String,
    pub last_updated_time: String,
    pub failed_status_details: Option<String>,
    pub recorder_media_uri_secret_arn: String,
    pub recorder_media_uri_type: String,
    pub recorder_schedule_expression: Option<String>,
    pub recorder_duration_in_seconds: Option<i32>,
    pub uploader_schedule_expression: Option<String>,
    pub uploader_duration_in_seconds: Option<i32>,
    pub delete_after_upload: Option<bool>,
    pub edge_retention_in_hours: Option<i32>,
    pub max_local_media_size_in_mb: Option<i32>,
    pub strategy_on_full_size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaStorageConfigView {
    pub status: String,
    pub stream_arn: Option<String>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

// ---------------------------------------------------------------------------
// From internal -> view
// ---------------------------------------------------------------------------

impl From<&KinesisVideoState> for KinesisVideoStateView {
    fn from(state: &KinesisVideoState) -> Self {
        KinesisVideoStateView {
            streams: state
                .streams
                .iter()
                .map(|(k, v)| (k.clone(), StreamView::from(v)))
                .collect(),
            channels: state
                .channels
                .iter()
                .map(|(k, v)| (k.clone(), SignalingChannelView::from(v)))
                .collect(),
        }
    }
}

impl From<&Stream> for StreamView {
    fn from(s: &Stream) -> Self {
        StreamView {
            stream_name: s.stream_name.clone(),
            stream_arn: s.stream_arn.clone(),
            device_name: s.device_name.clone(),
            media_type: s.media_type.clone(),
            kms_key_id: s.kms_key_id.clone(),
            version: s.version.clone(),
            status: s.status.clone(),
            creation_time: s.creation_time.to_rfc3339(),
            data_retention_in_hours: s.data_retention_in_hours,
            tags: s.tags.clone(),
            image_generation_config: s.image_generation_config.as_ref().map(|c| {
                ImageGenerationConfigView {
                    status: c.status.clone(),
                    image_selector_type: c.image_selector_type.clone(),
                    destination_uri: c.destination_uri.clone(),
                    destination_region: c.destination_region.clone(),
                    sampling_interval: c.sampling_interval,
                    format: c.format.clone(),
                    format_config: c.format_config.clone(),
                    height_pixels: c.height_pixels,
                    width_pixels: c.width_pixels,
                }
            }),
            notification_config: s
                .notification_config
                .as_ref()
                .map(|c| NotificationConfigView {
                    status: c.status.clone(),
                    destination_uri: c.destination_uri.clone(),
                }),
            storage_config: s.storage_config.as_ref().map(|c| StreamStorageConfigView {
                default_storage_tier: c.default_storage_tier.clone(),
            }),
            edge_config: s.edge_config.as_ref().map(|e| EdgeConfigStateView {
                hub_device_arn: e.hub_device_arn.clone(),
                sync_status: e.sync_status.clone(),
                creation_time: e.creation_time.to_rfc3339(),
                last_updated_time: e.last_updated_time.to_rfc3339(),
                failed_status_details: e.failed_status_details.clone(),
                recorder_media_uri_secret_arn: e.recorder_config.media_uri_secret_arn.clone(),
                recorder_media_uri_type: e.recorder_config.media_uri_type.clone(),
                recorder_schedule_expression: e.recorder_config.schedule_expression.clone(),
                recorder_duration_in_seconds: e.recorder_config.duration_in_seconds,
                uploader_schedule_expression: e
                    .uploader_config
                    .as_ref()
                    .map(|u| u.schedule_expression.clone()),
                uploader_duration_in_seconds: e
                    .uploader_config
                    .as_ref()
                    .map(|u| u.duration_in_seconds),
                delete_after_upload: e
                    .deletion_config
                    .as_ref()
                    .and_then(|d| d.delete_after_upload),
                edge_retention_in_hours: e
                    .deletion_config
                    .as_ref()
                    .and_then(|d| d.edge_retention_in_hours),
                max_local_media_size_in_mb: e
                    .deletion_config
                    .as_ref()
                    .and_then(|d| d.max_local_media_size_in_mb),
                strategy_on_full_size: e
                    .deletion_config
                    .as_ref()
                    .and_then(|d| d.strategy_on_full_size.clone()),
            }),
        }
    }
}

impl From<&SignalingChannel> for SignalingChannelView {
    fn from(c: &SignalingChannel) -> Self {
        SignalingChannelView {
            channel_name: c.channel_name.clone(),
            channel_arn: c.channel_arn.clone(),
            channel_type: c.channel_type.clone(),
            channel_status: c.channel_status.clone(),
            creation_time: c.creation_time.to_rfc3339(),
            version: c.version.clone(),
            message_ttl_seconds: c.message_ttl_seconds,
            tags: c.tags.clone(),
            media_storage_config: c
                .media_storage_config
                .as_ref()
                .map(|m| MediaStorageConfigView {
                    status: m.status.clone(),
                    stream_arn: m.stream_arn.clone(),
                }),
        }
    }
}

// ---------------------------------------------------------------------------
// From view -> internal
// ---------------------------------------------------------------------------

impl From<KinesisVideoStateView> for KinesisVideoState {
    fn from(view: KinesisVideoStateView) -> Self {
        KinesisVideoState {
            streams: view
                .streams
                .into_iter()
                .map(|(k, v)| (k, Stream::from(v)))
                .collect(),
            channels: view
                .channels
                .into_iter()
                .map(|(k, v)| (k, SignalingChannel::from(v)))
                .collect(),
        }
    }
}

impl From<StreamView> for Stream {
    fn from(v: StreamView) -> Self {
        Stream {
            stream_name: v.stream_name,
            stream_arn: v.stream_arn,
            device_name: v.device_name,
            media_type: v.media_type,
            kms_key_id: v.kms_key_id,
            version: v.version,
            status: v.status,
            creation_time: parse_dt(&v.creation_time),
            data_retention_in_hours: v.data_retention_in_hours,
            tags: v.tags,
            image_generation_config: v.image_generation_config.map(|c| ImageGenerationConfig {
                status: c.status,
                image_selector_type: c.image_selector_type,
                destination_uri: c.destination_uri,
                destination_region: c.destination_region,
                sampling_interval: c.sampling_interval,
                format: c.format,
                format_config: c.format_config,
                height_pixels: c.height_pixels,
                width_pixels: c.width_pixels,
            }),
            notification_config: v.notification_config.map(|c| NotificationConfig {
                status: c.status,
                destination_uri: c.destination_uri,
            }),
            storage_config: v.storage_config.map(|c| StreamStorageConfig {
                default_storage_tier: c.default_storage_tier,
            }),
            edge_config: v.edge_config.map(|e| EdgeConfigState {
                hub_device_arn: e.hub_device_arn,
                sync_status: e.sync_status,
                creation_time: parse_dt(&e.creation_time),
                last_updated_time: parse_dt(&e.last_updated_time),
                failed_status_details: e.failed_status_details,
                recorder_config: RecorderConfig {
                    media_uri_secret_arn: e.recorder_media_uri_secret_arn,
                    media_uri_type: e.recorder_media_uri_type,
                    schedule_expression: e.recorder_schedule_expression,
                    duration_in_seconds: e.recorder_duration_in_seconds,
                },
                uploader_config: e.uploader_schedule_expression.map(|se| UploaderConfig {
                    schedule_expression: se,
                    duration_in_seconds: e.uploader_duration_in_seconds.unwrap_or(0),
                }),
                deletion_config: if e.delete_after_upload.is_some()
                    || e.edge_retention_in_hours.is_some()
                    || e.max_local_media_size_in_mb.is_some()
                {
                    Some(DeletionConfig {
                        delete_after_upload: e.delete_after_upload,
                        edge_retention_in_hours: e.edge_retention_in_hours,
                        max_local_media_size_in_mb: e.max_local_media_size_in_mb,
                        strategy_on_full_size: e.strategy_on_full_size,
                    })
                } else {
                    None
                },
            }),
        }
    }
}

impl From<SignalingChannelView> for SignalingChannel {
    fn from(v: SignalingChannelView) -> Self {
        SignalingChannel {
            channel_name: v.channel_name,
            channel_arn: v.channel_arn,
            channel_type: v.channel_type,
            channel_status: v.channel_status,
            creation_time: parse_dt(&v.creation_time),
            version: v.version,
            message_ttl_seconds: v.message_ttl_seconds,
            tags: v.tags,
            media_storage_config: v.media_storage_config.map(|m| MediaStorageConfig {
                status: m.status,
                stream_arn: m.stream_arn,
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for KinesisVideoService {
    type StateView = KinesisVideoStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        KinesisVideoStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = KinesisVideoState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in view.streams {
                guard.streams.insert(k, Stream::from(v));
            }
            for (k, v) in view.channels {
                guard.channels.insert(k, SignalingChannel::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
