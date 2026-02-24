use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A Kinesis Video stream.
#[derive(Debug, Clone)]
pub struct Stream {
    pub stream_name: String,
    pub stream_arn: String,
    pub device_name: Option<String>,
    pub media_type: Option<String>,
    pub kms_key_id: String,
    pub version: String,
    pub status: String,
    pub creation_time: DateTime<Utc>,
    pub data_retention_in_hours: i32,
    pub tags: HashMap<String, String>,
    pub image_generation_config: Option<ImageGenerationConfig>,
    pub notification_config: Option<NotificationConfig>,
    pub storage_config: Option<StreamStorageConfig>,
    pub edge_config: Option<EdgeConfigState>,
}

/// A Kinesis Video signaling channel.
#[derive(Debug, Clone)]
pub struct SignalingChannel {
    pub channel_name: String,
    pub channel_arn: String,
    pub channel_type: String,
    pub channel_status: String,
    pub creation_time: DateTime<Utc>,
    pub version: String,
    pub message_ttl_seconds: Option<i32>,
    pub tags: HashMap<String, String>,
    pub media_storage_config: Option<MediaStorageConfig>,
}

#[derive(Debug, Clone)]
pub struct ImageGenerationConfig {
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

#[derive(Debug, Clone)]
pub struct NotificationConfig {
    pub status: String,
    pub destination_uri: String,
}

#[derive(Debug, Clone)]
pub struct StreamStorageConfig {
    pub default_storage_tier: String,
}

#[derive(Debug, Clone)]
pub struct EdgeConfigState {
    pub hub_device_arn: String,
    pub sync_status: String,
    pub creation_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
    pub failed_status_details: Option<String>,
    pub recorder_config: RecorderConfig,
    pub uploader_config: Option<UploaderConfig>,
    pub deletion_config: Option<DeletionConfig>,
}

#[derive(Debug, Clone)]
pub struct RecorderConfig {
    pub media_uri_secret_arn: String,
    pub media_uri_type: String,
    pub schedule_expression: Option<String>,
    pub duration_in_seconds: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct UploaderConfig {
    pub schedule_expression: String,
    pub duration_in_seconds: i32,
}

#[derive(Debug, Clone)]
pub struct DeletionConfig {
    pub delete_after_upload: Option<bool>,
    pub edge_retention_in_hours: Option<i32>,
    pub max_local_media_size_in_mb: Option<i32>,
    pub strategy_on_full_size: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MediaStorageConfig {
    pub status: String,
    pub stream_arn: Option<String>,
}
