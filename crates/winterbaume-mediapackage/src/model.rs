//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-mediapackage

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigureLogsRequest {
    #[serde(rename = "egressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "ingressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EgressAccessLogs {
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngressAccessLogs {
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigureLogsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "egressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "hlsIngest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ingressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsIngest {
    #[serde(rename = "ingestEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_endpoints: Option<Vec<IngestEndpoint>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestEndpoint {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "egressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "hlsIngest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ingressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHarvestJobRequest {
    #[serde(rename = "endTime")]
    #[serde(default)]
    pub end_time: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "originEndpointId")]
    #[serde(default)]
    pub origin_endpoint_id: String,
    #[serde(rename = "s3Destination")]
    #[serde(default)]
    pub s3_destination: S3Destination,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Destination {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "manifestKey")]
    #[serde(default)]
    pub manifest_key: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHarvestJobResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "originEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_id: Option<String>,
    #[serde(rename = "s3Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<S3Destination>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOriginEndpointRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[serde(rename = "channelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "cmafPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackageCreateOrUpdateParameters>,
    #[serde(rename = "dashPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "hlsPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "manifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "mssPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<String>,
    #[serde(rename = "startoverWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "timeDelaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Authorization {
    #[serde(rename = "cdnIdentifierSecret")]
    #[serde(default)]
    pub cdn_identifier_secret: String,
    #[serde(rename = "secretsRoleArn")]
    #[serde(default)]
    pub secrets_role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CmafPackageCreateOrUpdateParameters {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<CmafEncryption>,
    #[serde(rename = "hlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifests: Option<Vec<HlsManifestCreateOrUpdateParameters>>,
    #[serde(rename = "segmentDurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i32>,
    #[serde(rename = "segmentPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_prefix: Option<String>,
    #[serde(rename = "streamSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CmafEncryption {
    #[serde(rename = "constantInitializationVector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    #[serde(rename = "encryptionMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<String>,
    #[serde(rename = "keyRotationIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i32>,
    #[serde(rename = "spekeKeyProvider")]
    #[serde(default)]
    pub speke_key_provider: SpekeKeyProvider,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SpekeKeyProvider {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "encryptionContractConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_contract_configuration: Option<EncryptionContractConfiguration>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "systemIds")]
    #[serde(default)]
    pub system_ids: Vec<String>,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionContractConfiguration {
    #[serde(rename = "presetSpeke20Audio")]
    #[serde(default)]
    pub preset_speke20_audio: String,
    #[serde(rename = "presetSpeke20Video")]
    #[serde(default)]
    pub preset_speke20_video: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsManifestCreateOrUpdateParameters {
    #[serde(rename = "adMarkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<String>,
    #[serde(rename = "adTriggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_triggers: Option<Vec<String>>,
    #[serde(rename = "adsOnDeliveryRestrictions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_on_delivery_restrictions: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "includeIframeOnlyStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_stream: Option<bool>,
    #[serde(rename = "manifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "playlistType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_type: Option<String>,
    #[serde(rename = "playlistWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_window_seconds: Option<i32>,
    #[serde(rename = "programDateTimeIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamSelection {
    #[serde(rename = "maxVideoBitsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_video_bits_per_second: Option<i32>,
    #[serde(rename = "minVideoBitsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_video_bits_per_second: Option<i32>,
    #[serde(rename = "streamOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashPackage {
    #[serde(rename = "adTriggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_triggers: Option<Vec<String>>,
    #[serde(rename = "adsOnDeliveryRestrictions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_on_delivery_restrictions: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<DashEncryption>,
    #[serde(rename = "includeIframeOnlyStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_stream: Option<bool>,
    #[serde(rename = "manifestLayout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_layout: Option<String>,
    #[serde(rename = "manifestWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i32>,
    #[serde(rename = "minBufferTimeSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buffer_time_seconds: Option<i32>,
    #[serde(rename = "minUpdatePeriodSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_update_period_seconds: Option<i32>,
    #[serde(rename = "periodTriggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_triggers: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(rename = "segmentDurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i32>,
    #[serde(rename = "segmentTemplateFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_template_format: Option<String>,
    #[serde(rename = "streamSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
    #[serde(rename = "suggestedPresentationDelaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_presentation_delay_seconds: Option<i32>,
    #[serde(rename = "utcTiming")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc_timing: Option<String>,
    #[serde(rename = "utcTimingUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc_timing_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashEncryption {
    #[serde(rename = "keyRotationIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i32>,
    #[serde(rename = "spekeKeyProvider")]
    #[serde(default)]
    pub speke_key_provider: SpekeKeyProvider,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsPackage {
    #[serde(rename = "adMarkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<String>,
    #[serde(rename = "adTriggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_triggers: Option<Vec<String>>,
    #[serde(rename = "adsOnDeliveryRestrictions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_on_delivery_restrictions: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<HlsEncryption>,
    #[serde(rename = "includeDvbSubtitles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_dvb_subtitles: Option<bool>,
    #[serde(rename = "includeIframeOnlyStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_stream: Option<bool>,
    #[serde(rename = "playlistType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_type: Option<String>,
    #[serde(rename = "playlistWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_window_seconds: Option<i32>,
    #[serde(rename = "programDateTimeIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i32>,
    #[serde(rename = "segmentDurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i32>,
    #[serde(rename = "streamSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
    #[serde(rename = "useAudioRenditionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_audio_rendition_group: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsEncryption {
    #[serde(rename = "constantInitializationVector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    #[serde(rename = "encryptionMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<String>,
    #[serde(rename = "keyRotationIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i32>,
    #[serde(rename = "repeatExtXKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_ext_x_key: Option<bool>,
    #[serde(rename = "spekeKeyProvider")]
    #[serde(default)]
    pub speke_key_provider: SpekeKeyProvider,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MssPackage {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<MssEncryption>,
    #[serde(rename = "manifestWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i32>,
    #[serde(rename = "segmentDurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i32>,
    #[serde(rename = "streamSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MssEncryption {
    #[serde(rename = "spekeKeyProvider")]
    #[serde(default)]
    pub speke_key_provider: SpekeKeyProvider,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOriginEndpointResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "cmafPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "dashPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "hlsPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "manifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "mssPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<String>,
    #[serde(rename = "startoverWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "timeDelaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CmafPackage {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<CmafEncryption>,
    #[serde(rename = "hlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifests: Option<Vec<HlsManifest>>,
    #[serde(rename = "segmentDurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i32>,
    #[serde(rename = "segmentPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_prefix: Option<String>,
    #[serde(rename = "streamSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_selection: Option<StreamSelection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsManifest {
    #[serde(rename = "adMarkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<String>,
    #[serde(rename = "adTriggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_triggers: Option<Vec<String>>,
    #[serde(rename = "adsOnDeliveryRestrictions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_on_delivery_restrictions: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "includeIframeOnlyStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_stream: Option<bool>,
    #[serde(rename = "manifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "playlistType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_type: Option<String>,
    #[serde(rename = "playlistWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_window_seconds: Option<i32>,
    #[serde(rename = "programDateTimeIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOriginEndpointRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOriginEndpointResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeChannelRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeChannelResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "egressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "hlsIngest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ingressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHarvestJobRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHarvestJobResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "originEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_id: Option<String>,
    #[serde(rename = "s3Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<S3Destination>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOriginEndpointRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOriginEndpointResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "cmafPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "dashPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "hlsPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "manifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "mssPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<String>,
    #[serde(rename = "startoverWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "timeDelaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChannelsRequest {
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
pub struct ListChannelsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<Channel>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Channel {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "egressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "hlsIngest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ingressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListHarvestJobsRequest {
    #[serde(rename = "IncludeChannelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_channel_id: Option<String>,
    #[serde(rename = "IncludeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_status: Option<String>,
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
pub struct ListHarvestJobsResponse {
    #[serde(rename = "harvestJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harvest_jobs: Option<Vec<HarvestJob>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HarvestJob {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "originEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_id: Option<String>,
    #[serde(rename = "s3Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<S3Destination>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOriginEndpointsRequest {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
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
pub struct ListOriginEndpointsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "originEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoints: Option<Vec<OriginEndpoint>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OriginEndpoint {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "cmafPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "dashPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "hlsPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "manifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "mssPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<String>,
    #[serde(rename = "startoverWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "timeDelaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RotateChannelCredentialsRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RotateChannelCredentialsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "egressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "hlsIngest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ingressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RotateIngestEndpointCredentialsRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IngestEndpointId")]
    #[serde(default)]
    pub ingest_endpoint_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RotateIngestEndpointCredentialsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "egressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "hlsIngest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ingressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "egressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_access_logs: Option<EgressAccessLogs>,
    #[serde(rename = "hlsIngest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_ingest: Option<HlsIngest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ingressAccessLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_access_logs: Option<IngressAccessLogs>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOriginEndpointRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[serde(rename = "cmafPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackageCreateOrUpdateParameters>,
    #[serde(rename = "dashPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "hlsPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "manifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "mssPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<String>,
    #[serde(rename = "startoverWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i32>,
    #[serde(rename = "timeDelaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOriginEndpointResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "cmafPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_package: Option<CmafPackage>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "dashPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_package: Option<DashPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "hlsPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_package: Option<HlsPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "manifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "mssPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_package: Option<MssPackage>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination: Option<String>,
    #[serde(rename = "startoverWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "timeDelaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}
