//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-kinesisvideo

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSignalingChannelInput {
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    pub channel_name: String,
    #[serde(rename = "ChannelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    #[serde(rename = "SingleMasterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_master_configuration: Option<SingleMasterConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SingleMasterConfiguration {
    #[serde(rename = "MessageTtlSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_ttl_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSignalingChannelOutput {
    #[serde(rename = "ChannelARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStreamInput {
    #[serde(rename = "DataRetentionInHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_retention_in_hours: Option<i32>,
    #[serde(rename = "DeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MediaType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    pub stream_name: String,
    #[serde(rename = "StreamStorageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_storage_configuration: Option<StreamStorageConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamStorageConfiguration {
    #[serde(rename = "DefaultStorageTier")]
    #[serde(default)]
    pub default_storage_tier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStreamOutput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEdgeConfigurationInput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEdgeConfigurationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSignalingChannelInput {
    #[serde(rename = "ChannelARN")]
    #[serde(default)]
    pub channel_a_r_n: String,
    #[serde(rename = "CurrentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSignalingChannelOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStreamInput {
    #[serde(rename = "CurrentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    pub stream_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStreamOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEdgeConfigurationInput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEdgeConfigurationOutput {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "EdgeAgentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_agent_status: Option<EdgeAgentStatus>,
    #[serde(rename = "EdgeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_config: Option<EdgeConfig>,
    #[serde(rename = "FailedStatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_status_details: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "SyncStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EdgeAgentStatus {
    #[serde(rename = "LastRecorderStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_recorder_status: Option<LastRecorderStatus>,
    #[serde(rename = "LastUploaderStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_uploader_status: Option<LastUploaderStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LastRecorderStatus {
    #[serde(rename = "JobStatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status_details: Option<String>,
    #[serde(rename = "LastCollectedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_collected_time: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "RecorderStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorder_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LastUploaderStatus {
    #[serde(rename = "JobStatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status_details: Option<String>,
    #[serde(rename = "LastCollectedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_collected_time: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "UploaderStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploader_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EdgeConfig {
    #[serde(rename = "DeletionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_config: Option<DeletionConfig>,
    #[serde(rename = "HubDeviceArn")]
    #[serde(default)]
    pub hub_device_arn: String,
    #[serde(rename = "RecorderConfig")]
    #[serde(default)]
    pub recorder_config: RecorderConfig,
    #[serde(rename = "UploaderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploader_config: Option<UploaderConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletionConfig {
    #[serde(rename = "DeleteAfterUpload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_after_upload: Option<bool>,
    #[serde(rename = "EdgeRetentionInHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_retention_in_hours: Option<i32>,
    #[serde(rename = "LocalSizeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_size_config: Option<LocalSizeConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocalSizeConfig {
    #[serde(rename = "MaxLocalMediaSizeInMB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_local_media_size_in_m_b: Option<i32>,
    #[serde(rename = "StrategyOnFullSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_on_full_size: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecorderConfig {
    #[serde(rename = "MediaSourceConfig")]
    #[serde(default)]
    pub media_source_config: MediaSourceConfig,
    #[serde(rename = "ScheduleConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_config: Option<ScheduleConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaSourceConfig {
    #[serde(rename = "MediaUriSecretArn")]
    #[serde(default)]
    pub media_uri_secret_arn: String,
    #[serde(rename = "MediaUriType")]
    #[serde(default)]
    pub media_uri_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleConfig {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    pub duration_in_seconds: i32,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    pub schedule_expression: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UploaderConfig {
    #[serde(rename = "ScheduleConfig")]
    #[serde(default)]
    pub schedule_config: ScheduleConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImageGenerationConfigurationInput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImageGenerationConfigurationOutput {
    #[serde(rename = "ImageGenerationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_generation_configuration: Option<ImageGenerationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageGenerationConfiguration {
    #[serde(rename = "DestinationConfig")]
    #[serde(default)]
    pub destination_config: ImageGenerationDestinationConfig,
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
    #[serde(rename = "FormatConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "HeightPixels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_pixels: Option<i32>,
    #[serde(rename = "ImageSelectorType")]
    #[serde(default)]
    pub image_selector_type: String,
    #[serde(rename = "SamplingInterval")]
    #[serde(default)]
    pub sampling_interval: i32,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "WidthPixels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_pixels: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageGenerationDestinationConfig {
    #[serde(rename = "DestinationRegion")]
    #[serde(default)]
    pub destination_region: String,
    #[serde(rename = "Uri")]
    #[serde(default)]
    pub uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMappedResourceConfigurationInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMappedResourceConfigurationOutput {
    #[serde(rename = "MappedResourceConfigurationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapped_resource_configuration_list: Option<Vec<MappedResourceConfigurationListItem>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MappedResourceConfigurationListItem {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMediaStorageConfigurationInput {
    #[serde(rename = "ChannelARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_a_r_n: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMediaStorageConfigurationOutput {
    #[serde(rename = "MediaStorageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_storage_configuration: Option<MediaStorageConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaStorageConfiguration {
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNotificationConfigurationInput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNotificationConfigurationOutput {
    #[serde(rename = "NotificationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<NotificationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationConfiguration {
    #[serde(rename = "DestinationConfig")]
    #[serde(default)]
    pub destination_config: NotificationDestinationConfig,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationDestinationConfig {
    #[serde(rename = "Uri")]
    #[serde(default)]
    pub uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSignalingChannelInput {
    #[serde(rename = "ChannelARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_a_r_n: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSignalingChannelOutput {
    #[serde(rename = "ChannelInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_info: Option<ChannelInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelInfo {
    #[serde(rename = "ChannelARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_a_r_n: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "ChannelStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_status: Option<String>,
    #[serde(rename = "ChannelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "SingleMasterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_master_configuration: Option<SingleMasterConfiguration>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamInput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamOutput {
    #[serde(rename = "StreamInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_info: Option<StreamInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamInfo {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DataRetentionInHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_retention_in_hours: Option<i32>,
    #[serde(rename = "DeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MediaType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamStorageConfigurationInput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamStorageConfigurationOutput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamStorageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_storage_configuration: Option<StreamStorageConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataEndpointInput {
    #[serde(rename = "APIName")]
    #[serde(default)]
    pub a_p_i_name: String,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataEndpointOutput {
    #[serde(rename = "DataEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSignalingChannelEndpointInput {
    #[serde(rename = "ChannelARN")]
    #[serde(default)]
    pub channel_a_r_n: String,
    #[serde(rename = "SingleMasterChannelEndpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_master_channel_endpoint_configuration:
        Option<SingleMasterChannelEndpointConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SingleMasterChannelEndpointConfiguration {
    #[serde(rename = "Protocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSignalingChannelEndpointOutput {
    #[serde(rename = "ResourceEndpointList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_endpoint_list: Option<Vec<ResourceEndpointListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceEndpointListItem {
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "ResourceEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEdgeAgentConfigurationsInput {
    #[serde(rename = "HubDeviceArn")]
    #[serde(default)]
    pub hub_device_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEdgeAgentConfigurationsOutput {
    #[serde(rename = "EdgeConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_configs: Option<Vec<ListEdgeAgentConfigurationsEdgeConfig>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEdgeAgentConfigurationsEdgeConfig {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "EdgeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_config: Option<EdgeConfig>,
    #[serde(rename = "FailedStatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_status_details: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "SyncStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSignalingChannelsInput {
    #[serde(rename = "ChannelNameCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name_condition: Option<ChannelNameCondition>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelNameCondition {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "ComparisonValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSignalingChannelsOutput {
    #[serde(rename = "ChannelInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_info_list: Option<Vec<ChannelInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StreamNameCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name_condition: Option<StreamNameCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamNameCondition {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "ComparisonValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StreamInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_info_list: Option<Vec<StreamInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForStreamInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForStreamOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartEdgeConfigurationUpdateInput {
    #[serde(rename = "EdgeConfig")]
    #[serde(default)]
    pub edge_config: EdgeConfig,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartEdgeConfigurationUpdateOutput {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "EdgeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_config: Option<EdgeConfig>,
    #[serde(rename = "FailedStatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_status_details: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "SyncStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagStreamInput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagStreamOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeyList")]
    #[serde(default)]
    pub tag_key_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagStreamInput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "TagKeyList")]
    #[serde(default)]
    pub tag_key_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagStreamOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataRetentionInput {
    #[serde(rename = "CurrentVersion")]
    #[serde(default)]
    pub current_version: String,
    #[serde(rename = "DataRetentionChangeInHours")]
    #[serde(default)]
    pub data_retention_change_in_hours: i32,
    #[serde(rename = "Operation")]
    #[serde(default)]
    pub operation: String,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataRetentionOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateImageGenerationConfigurationInput {
    #[serde(rename = "ImageGenerationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_generation_configuration: Option<ImageGenerationConfiguration>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateImageGenerationConfigurationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMediaStorageConfigurationInput {
    #[serde(rename = "ChannelARN")]
    #[serde(default)]
    pub channel_a_r_n: String,
    #[serde(rename = "MediaStorageConfiguration")]
    #[serde(default)]
    pub media_storage_configuration: MediaStorageConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMediaStorageConfigurationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNotificationConfigurationInput {
    #[serde(rename = "NotificationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<NotificationConfiguration>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNotificationConfigurationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSignalingChannelInput {
    #[serde(rename = "ChannelARN")]
    #[serde(default)]
    pub channel_a_r_n: String,
    #[serde(rename = "CurrentVersion")]
    #[serde(default)]
    pub current_version: String,
    #[serde(rename = "SingleMasterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_master_configuration: Option<SingleMasterConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSignalingChannelOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStreamInput {
    #[serde(rename = "CurrentVersion")]
    #[serde(default)]
    pub current_version: String,
    #[serde(rename = "DeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "MediaType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStreamOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStreamStorageConfigurationInput {
    #[serde(rename = "CurrentVersion")]
    #[serde(default)]
    pub current_version: String,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamStorageConfiguration")]
    #[serde(default)]
    pub stream_storage_configuration: StreamStorageConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStreamStorageConfigurationOutput {}
