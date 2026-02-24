//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ivs

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPlaybackRestrictionPoliciesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "playbackRestrictionPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_restriction_policies: Option<Vec<PlaybackRestrictionPolicySummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlaybackRestrictionPolicySummary {
    #[serde(rename = "allowedCountries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_countries: Option<Vec<String>>,
    #[serde(rename = "allowedOrigins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "enableStrictOriginEnforcement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_strict_origin_enforcement: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecordingConfigurationResponse {
    #[serde(rename = "recordingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_configuration: Option<RecordingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordingConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "destinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_configuration: Option<DestinationConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "recordingReconnectWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_reconnect_window_seconds: Option<i32>,
    #[serde(rename = "renditionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendition_configuration: Option<RenditionConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "thumbnailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_configuration: Option<ThumbnailConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3DestinationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DestinationConfiguration {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RenditionConfiguration {
    #[serde(rename = "renditionSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendition_selection: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renditions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThumbnailConfiguration {
    #[serde(rename = "recordingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<Vec<String>>,
    #[serde(rename = "targetIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_interval_seconds: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetChannelRequest {
    #[serde(default)]
    pub arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePlaybackKeyPairRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePlaybackRestrictionPolicyResponse {
    #[serde(rename = "playbackRestrictionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_restriction_policy: Option<PlaybackRestrictionPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlaybackRestrictionPolicy {
    #[serde(rename = "allowedCountries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_countries: Option<Vec<String>>,
    #[serde(rename = "allowedOrigins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "enableStrictOriginEnforcement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_strict_origin_enforcement: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streams: Option<Vec<StreamSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamSummary {
    #[serde(rename = "channelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "viewerCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    #[serde(rename = "streamKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_key: Option<StreamKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Channel {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized: Option<bool>,
    #[serde(rename = "containerFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_format: Option<String>,
    #[serde(rename = "ingestEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_endpoint: Option<String>,
    #[serde(rename = "insecureIngest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ingest: Option<bool>,
    #[serde(rename = "latencyMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_mode: Option<String>,
    #[serde(rename = "multitrackInputConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multitrack_input_configuration: Option<MultitrackInputConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "playbackRestrictionPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_restriction_policy_arn: Option<String>,
    #[serde(rename = "playbackUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    #[serde(rename = "recordingConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_configuration_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt: Option<Srt>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultitrackInputConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "maximumResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_resolution: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Srt {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamKey {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPlaybackKeyPairsResponse {
    #[serde(rename = "keyPairs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pairs: Option<Vec<PlaybackKeyPairSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlaybackKeyPairSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRecordingConfigurationResponse {
    #[serde(rename = "recordingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_configuration: Option<RecordingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStreamRequest {
    #[serde(rename = "channelArn")]
    #[serde(default)]
    pub channel_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecordingConfigurationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "recordingConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_configurations: Option<Vec<RecordingConfigurationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordingConfigurationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "destinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_configuration: Option<DestinationConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChannelResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPlaybackKeyPairRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamKeysRequest {
    #[serde(rename = "channelArn")]
    #[serde(default)]
    pub channel_arn: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStreamResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<Stream>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Stream {
    #[serde(rename = "channelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    #[serde(rename = "playbackUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_url: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "viewerCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStreamKeyResponse {
    #[serde(rename = "streamKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_key: Option<StreamKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecordingConfigurationsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamsRequest {
    #[serde(rename = "filterBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by: Option<StreamFilters>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamFilters {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized: Option<bool>,
    #[serde(rename = "containerFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_format: Option<String>,
    #[serde(rename = "insecureIngest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ingest: Option<bool>,
    #[serde(rename = "latencyMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_mode: Option<String>,
    #[serde(rename = "multitrackInputConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multitrack_input_configuration: Option<MultitrackInputConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "playbackRestrictionPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_restriction_policy_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    #[serde(rename = "recordingConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_configuration_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPlaybackRestrictionPoliciesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutMetadataRequest {
    #[serde(rename = "channelArn")]
    #[serde(default)]
    pub channel_arn: String,
    #[serde(default)]
    pub metadata: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStreamKeyRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartViewerSessionRevocationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPlaybackRestrictionPolicyResponse {
    #[serde(rename = "playbackRestrictionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_restriction_policy: Option<PlaybackRestrictionPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopStreamResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRecordingConfigurationRequest {
    #[serde(rename = "destinationConfiguration")]
    #[serde(default)]
    pub destination_configuration: DestinationConfiguration,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "recordingReconnectWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_reconnect_window_seconds: Option<i32>,
    #[serde(rename = "renditionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendition_configuration: Option<RenditionConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "thumbnailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_configuration: Option<ThumbnailConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStreamKeyRequest {
    #[serde(rename = "channelArn")]
    #[serde(default)]
    pub channel_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamKeysResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "streamKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_keys: Option<Vec<StreamKeySummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamKeySummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPlaybackKeyPairResponse {
    #[serde(rename = "keyPair")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<PlaybackKeyPair>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlaybackKeyPair {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPlaybackRestrictionPolicyRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStartViewerSessionRevocationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchStartViewerSessionRevocationError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStartViewerSessionRevocationError {
    #[serde(rename = "channelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "viewerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStreamKeyRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePlaybackRestrictionPolicyRequest {
    #[serde(rename = "allowedCountries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_countries: Option<Vec<String>>,
    #[serde(rename = "allowedOrigins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "enableStrictOriginEnforcement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_strict_origin_enforcement: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecordingConfigurationRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChannelRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartViewerSessionRevocationRequest {
    #[serde(rename = "channelArn")]
    #[serde(default)]
    pub channel_arn: String,
    #[serde(rename = "viewerId")]
    #[serde(default)]
    pub viewer_id: String,
    #[serde(rename = "viewerSessionVersionsLessThanOrEqualTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_session_versions_less_than_or_equal_to: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportPlaybackKeyPairResponse {
    #[serde(rename = "keyPair")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<PlaybackKeyPair>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChannelsRequest {
    #[serde(rename = "filterByName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_name: Option<String>,
    #[serde(rename = "filterByPlaybackRestrictionPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_playback_restriction_policy_arn: Option<String>,
    #[serde(rename = "filterByRecordingConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_recording_configuration_arn: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStreamKeyResponse {
    #[serde(rename = "streamKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_key: Option<StreamKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePlaybackRestrictionPolicyRequest {
    #[serde(rename = "allowedCountries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_countries: Option<Vec<String>>,
    #[serde(rename = "allowedOrigins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    #[serde(rename = "enableStrictOriginEnforcement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_strict_origin_enforcement: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStartViewerSessionRevocationRequest {
    #[serde(rename = "viewerSessions")]
    #[serde(default)]
    pub viewer_sessions: Vec<BatchStartViewerSessionRevocationViewerSession>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStartViewerSessionRevocationViewerSession {
    #[serde(rename = "channelArn")]
    #[serde(default)]
    pub channel_arn: String,
    #[serde(rename = "viewerId")]
    #[serde(default)]
    pub viewer_id: String,
    #[serde(rename = "viewerSessionVersionsLessThanOrEqualTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_session_versions_less_than_or_equal_to: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePlaybackRestrictionPolicyRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePlaybackRestrictionPolicyResponse {
    #[serde(rename = "playbackRestrictionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_restriction_policy: Option<PlaybackRestrictionPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetChannelResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<Channel>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchError {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePlaybackKeyPairResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelRequest {
    #[serde(default)]
    pub arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized: Option<bool>,
    #[serde(rename = "containerFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_format: Option<String>,
    #[serde(rename = "insecureIngest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ingest: Option<bool>,
    #[serde(rename = "latencyMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_mode: Option<String>,
    #[serde(rename = "multitrackInputConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multitrack_input_configuration: Option<MultitrackInputConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "playbackRestrictionPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_restriction_policy_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    #[serde(rename = "recordingConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_configuration_arn: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRecordingConfigurationRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStreamSessionRequest {
    #[serde(rename = "channelArn")]
    #[serde(default)]
    pub channel_arn: String,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopStreamRequest {
    #[serde(rename = "channelArn")]
    #[serde(default)]
    pub channel_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamSessionsRequest {
    #[serde(rename = "channelArn")]
    #[serde(default)]
    pub channel_arn: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStreamSessionResponse {
    #[serde(rename = "streamSession")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_session: Option<StreamSession>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamSession {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "ingestConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_configuration: Option<IngestConfiguration>,
    #[serde(rename = "ingestConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_configurations: Option<IngestConfigurations>,
    #[serde(rename = "recordingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_configuration: Option<RecordingConfiguration>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "truncatedEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated_events: Option<Vec<StreamEvent>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<VideoConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(rename = "sampleRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
    #[serde(rename = "targetBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_bitrate: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VideoConfiguration {
    #[serde(rename = "avcLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avc_level: Option<String>,
    #[serde(rename = "avcProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avc_profile: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(rename = "targetBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_bitrate: Option<i64>,
    #[serde(rename = "targetFramerate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_framerate: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
    #[serde(rename = "videoHeight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i64>,
    #[serde(rename = "videoWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestConfigurations {
    #[serde(rename = "audioConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_configurations: Option<Vec<AudioConfiguration>>,
    #[serde(rename = "videoConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_configurations: Option<Vec<VideoConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamEvent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "eventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPlaybackKeyPairsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChannelsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<ChannelSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized: Option<bool>,
    #[serde(rename = "insecureIngest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ingest: Option<bool>,
    #[serde(rename = "latencyMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "playbackRestrictionPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_restriction_policy_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    #[serde(rename = "recordingConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_configuration_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetStreamKeyResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchError>>,
    #[serde(rename = "streamKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_keys: Option<Vec<StreamKey>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetStreamKeyRequest {
    #[serde(default)]
    pub arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamSessionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "streamSessions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_sessions: Option<Vec<StreamSessionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamSessionSummary {
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "hasErrorEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_error_event: Option<bool>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportPlaybackKeyPairRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publicKeyMaterial")]
    #[serde(default)]
    pub public_key_material: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}
