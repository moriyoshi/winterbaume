//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-mediapackagev2

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelHarvestJobRequest {
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOriginEndpointPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChannelGroupsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOriginEndpointResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "ContainerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_type: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "DashManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_manifests: Option<Vec<GetDashManifestConfiguration>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "ForceEndpointErrorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_endpoint_error_configuration: Option<ForceEndpointErrorConfiguration>,
    #[serde(rename = "HlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifests: Option<Vec<GetHlsManifestConfiguration>>,
    #[serde(rename = "LowLatencyHlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_latency_hls_manifests: Option<Vec<GetLowLatencyHlsManifestConfiguration>>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "MssManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_manifests: Option<Vec<GetMssManifestConfiguration>>,
    #[serde(rename = "OriginEndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_name: Option<String>,
    #[serde(rename = "Segment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Segment>,
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDashManifestConfiguration {
    #[serde(rename = "BaseUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_urls: Option<Vec<DashBaseUrl>>,
    #[serde(rename = "Compactness")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compactness: Option<String>,
    #[serde(rename = "DrmSignaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drm_signaling: Option<String>,
    #[serde(rename = "DvbSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_settings: Option<DashDvbSettings>,
    #[serde(rename = "FilterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_configuration: Option<FilterConfiguration>,
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i32>,
    #[serde(rename = "MinBufferTimeSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buffer_time_seconds: Option<i32>,
    #[serde(rename = "MinUpdatePeriodSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_update_period_seconds: Option<i32>,
    #[serde(rename = "PeriodTriggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_triggers: Option<Vec<String>>,
    #[serde(rename = "Profiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    #[serde(rename = "ProgramInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_information: Option<DashProgramInformation>,
    #[serde(rename = "ScteDash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_dash: Option<ScteDash>,
    #[serde(rename = "SegmentTemplateFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_template_format: Option<String>,
    #[serde(rename = "SubtitleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle_configuration: Option<DashSubtitleConfiguration>,
    #[serde(rename = "SuggestedPresentationDelaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_presentation_delay_seconds: Option<i32>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "UtcTiming")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc_timing: Option<DashUtcTiming>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashBaseUrl {
    #[serde(rename = "DvbPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_priority: Option<i32>,
    #[serde(rename = "DvbWeight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_weight: Option<i32>,
    #[serde(rename = "ServiceLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_location: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashDvbSettings {
    #[serde(rename = "ErrorMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_metrics: Option<Vec<DashDvbMetricsReporting>>,
    #[serde(rename = "FontDownload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_download: Option<DashDvbFontDownload>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashDvbMetricsReporting {
    #[serde(rename = "Probability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<i32>,
    #[serde(rename = "ReportingUrl")]
    #[serde(default)]
    pub reporting_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashDvbFontDownload {
    #[serde(rename = "FontFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(rename = "MimeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterConfiguration {
    #[serde(rename = "ClipStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_start_time: Option<f64>,
    #[serde(rename = "DrmSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drm_settings: Option<String>,
    #[serde(rename = "End")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    #[serde(rename = "ManifestFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_filter: Option<String>,
    #[serde(rename = "Start")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
    #[serde(rename = "TimeDelaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delay_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashProgramInformation {
    #[serde(rename = "Copyright")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "MoreInformationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more_information_url: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScteDash {
    #[serde(rename = "AdMarkerDash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_marker_dash: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashSubtitleConfiguration {
    #[serde(rename = "TtmlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttml_configuration: Option<DashTtmlConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashTtmlConfiguration {
    #[serde(rename = "TtmlProfile")]
    #[serde(default)]
    pub ttml_profile: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashUtcTiming {
    #[serde(rename = "TimingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_mode: Option<String>,
    #[serde(rename = "TimingSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ForceEndpointErrorConfiguration {
    #[serde(rename = "EndpointErrorConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_error_conditions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetHlsManifestConfiguration {
    #[serde(rename = "ChildManifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_manifest_name: Option<String>,
    #[serde(rename = "FilterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_configuration: Option<FilterConfiguration>,
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i32>,
    #[serde(rename = "ProgramDateTimeIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i32>,
    #[serde(rename = "ScteHls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_hls: Option<ScteHls>,
    #[serde(rename = "StartTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_tag: Option<StartTag>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "UrlEncodeChildManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_encode_child_manifest: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScteHls {
    #[serde(rename = "AdMarkerHls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_marker_hls: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTag {
    #[serde(rename = "Precise")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precise: Option<bool>,
    #[serde(rename = "TimeOffset")]
    #[serde(default)]
    pub time_offset: f32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLowLatencyHlsManifestConfiguration {
    #[serde(rename = "ChildManifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_manifest_name: Option<String>,
    #[serde(rename = "FilterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_configuration: Option<FilterConfiguration>,
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i32>,
    #[serde(rename = "ProgramDateTimeIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i32>,
    #[serde(rename = "ScteHls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_hls: Option<ScteHls>,
    #[serde(rename = "StartTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_tag: Option<StartTag>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "UrlEncodeChildManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_encode_child_manifest: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMssManifestConfiguration {
    #[serde(rename = "FilterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_configuration: Option<FilterConfiguration>,
    #[serde(rename = "ManifestLayout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_layout: Option<String>,
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i32>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Segment {
    #[serde(rename = "Encryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "IncludeIframeOnlyStreams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_iframe_only_streams: Option<bool>,
    #[serde(rename = "Scte")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte: Option<Scte>,
    #[serde(rename = "SegmentDurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_duration_seconds: Option<i32>,
    #[serde(rename = "SegmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
    #[serde(rename = "TsIncludeDvbSubtitles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_include_dvb_subtitles: Option<bool>,
    #[serde(rename = "TsUseAudioRenditionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_use_audio_rendition_group: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Encryption {
    #[serde(rename = "CmafExcludeSegmentDrmMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_exclude_segment_drm_metadata: Option<bool>,
    #[serde(rename = "ConstantInitializationVector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    #[serde(rename = "EncryptionMethod")]
    #[serde(default)]
    pub encryption_method: EncryptionMethod,
    #[serde(rename = "KeyRotationIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_interval_seconds: Option<i32>,
    #[serde(rename = "SpekeKeyProvider")]
    #[serde(default)]
    pub speke_key_provider: SpekeKeyProvider,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionMethod {
    #[serde(rename = "CmafEncryptionMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_encryption_method: Option<String>,
    #[serde(rename = "IsmEncryptionMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ism_encryption_method: Option<String>,
    #[serde(rename = "TsEncryptionMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_encryption_method: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SpekeKeyProvider {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "DrmSystems")]
    #[serde(default)]
    pub drm_systems: Vec<String>,
    #[serde(rename = "EncryptionContractConfiguration")]
    #[serde(default)]
    pub encryption_contract_configuration: EncryptionContractConfiguration,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "Url")]
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionContractConfiguration {
    #[serde(rename = "PresetSpeke20Audio")]
    #[serde(default)]
    pub preset_speke20_audio: String,
    #[serde(rename = "PresetSpeke20Video")]
    #[serde(default)]
    pub preset_speke20_video: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte {
    #[serde(rename = "ScteFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_filter: Option<Vec<String>>,
    #[serde(rename = "ScteInSegments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_in_segments: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelGroupRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListHarvestJobsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutChannelPolicyRequest {
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetChannelStateResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "ResetAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOriginEndpointPolicyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetOriginEndpointStateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOriginEndpointRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelGroupRequest {
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    pub channel_group_name: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOriginEndpointResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "ContainerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_type: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "DashManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_manifests: Option<Vec<GetDashManifestConfiguration>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "ForceEndpointErrorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_endpoint_error_configuration: Option<ForceEndpointErrorConfiguration>,
    #[serde(rename = "HlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifests: Option<Vec<GetHlsManifestConfiguration>>,
    #[serde(rename = "LowLatencyHlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_latency_hls_manifests: Option<Vec<GetLowLatencyHlsManifestConfiguration>>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "MssManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_manifests: Option<Vec<GetMssManifestConfiguration>>,
    #[serde(rename = "OriginEndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_name: Option<String>,
    #[serde(rename = "Segment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Segment>,
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i32>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelHarvestJobResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChannelPolicyResponse {
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChannelResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "IngestEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_endpoints: Option<Vec<IngestEndpoint>>,
    #[serde(rename = "InputSwitchConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_switch_configuration: Option<InputSwitchConfiguration>,
    #[serde(rename = "InputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "OutputHeaderConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_header_configuration: Option<OutputHeaderConfiguration>,
    #[serde(rename = "ResetAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_at: Option<f64>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestEndpoint {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputSwitchConfiguration {
    #[serde(rename = "MQCSInputSwitching")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_q_c_s_input_switching: Option<bool>,
    #[serde(rename = "PreferredInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_input: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputHeaderConfiguration {
    #[serde(rename = "PublishMQCS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_m_q_c_s: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelGroupRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListHarvestJobsResponse {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<HarvestJob>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HarvestJob {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "HarvestJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harvest_job_name: Option<String>,
    #[serde(rename = "HarvestedManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harvested_manifests: Option<HarvestedManifests>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "OriginEndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_name: Option<String>,
    #[serde(rename = "ScheduleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_configuration: Option<HarvesterScheduleConfiguration>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Destination {
    #[serde(rename = "S3Destination")]
    #[serde(default)]
    pub s3_destination: S3DestinationConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DestinationConfig {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "DestinationPath")]
    #[serde(default)]
    pub destination_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HarvestedManifests {
    #[serde(rename = "DashManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_manifests: Option<Vec<HarvestedDashManifest>>,
    #[serde(rename = "HlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifests: Option<Vec<HarvestedHlsManifest>>,
    #[serde(rename = "LowLatencyHlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_latency_hls_manifests: Option<Vec<HarvestedLowLatencyHlsManifest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HarvestedDashManifest {
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    pub manifest_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HarvestedHlsManifest {
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    pub manifest_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HarvestedLowLatencyHlsManifest {
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    pub manifest_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HarvesterScheduleConfiguration {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChannelsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOriginEndpointResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "IngestEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_endpoints: Option<Vec<IngestEndpoint>>,
    #[serde(rename = "InputSwitchConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_switch_configuration: Option<InputSwitchConfiguration>,
    #[serde(rename = "InputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "OutputHeaderConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_header_configuration: Option<OutputHeaderConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOriginEndpointRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContainerType")]
    #[serde(default)]
    pub container_type: String,
    #[serde(rename = "DashManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_manifests: Option<Vec<CreateDashManifestConfiguration>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ForceEndpointErrorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_endpoint_error_configuration: Option<ForceEndpointErrorConfiguration>,
    #[serde(rename = "HlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifests: Option<Vec<CreateHlsManifestConfiguration>>,
    #[serde(rename = "LowLatencyHlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_latency_hls_manifests: Option<Vec<CreateLowLatencyHlsManifestConfiguration>>,
    #[serde(rename = "MssManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_manifests: Option<Vec<CreateMssManifestConfiguration>>,
    #[serde(rename = "OriginEndpointName")]
    #[serde(default)]
    pub origin_endpoint_name: String,
    #[serde(rename = "Segment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Segment>,
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i32>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDashManifestConfiguration {
    #[serde(rename = "BaseUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_urls: Option<Vec<DashBaseUrl>>,
    #[serde(rename = "Compactness")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compactness: Option<String>,
    #[serde(rename = "DrmSignaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drm_signaling: Option<String>,
    #[serde(rename = "DvbSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_settings: Option<DashDvbSettings>,
    #[serde(rename = "FilterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_configuration: Option<FilterConfiguration>,
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    pub manifest_name: String,
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i32>,
    #[serde(rename = "MinBufferTimeSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buffer_time_seconds: Option<i32>,
    #[serde(rename = "MinUpdatePeriodSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_update_period_seconds: Option<i32>,
    #[serde(rename = "PeriodTriggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_triggers: Option<Vec<String>>,
    #[serde(rename = "Profiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    #[serde(rename = "ProgramInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_information: Option<DashProgramInformation>,
    #[serde(rename = "ScteDash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_dash: Option<ScteDash>,
    #[serde(rename = "SegmentTemplateFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_template_format: Option<String>,
    #[serde(rename = "SubtitleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle_configuration: Option<DashSubtitleConfiguration>,
    #[serde(rename = "SuggestedPresentationDelaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_presentation_delay_seconds: Option<i32>,
    #[serde(rename = "UtcTiming")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utc_timing: Option<DashUtcTiming>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHlsManifestConfiguration {
    #[serde(rename = "ChildManifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_manifest_name: Option<String>,
    #[serde(rename = "FilterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_configuration: Option<FilterConfiguration>,
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    pub manifest_name: String,
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i32>,
    #[serde(rename = "ProgramDateTimeIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i32>,
    #[serde(rename = "ScteHls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_hls: Option<ScteHls>,
    #[serde(rename = "StartTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_tag: Option<StartTag>,
    #[serde(rename = "UrlEncodeChildManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_encode_child_manifest: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLowLatencyHlsManifestConfiguration {
    #[serde(rename = "ChildManifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_manifest_name: Option<String>,
    #[serde(rename = "FilterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_configuration: Option<FilterConfiguration>,
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    pub manifest_name: String,
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i32>,
    #[serde(rename = "ProgramDateTimeIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_interval_seconds: Option<i32>,
    #[serde(rename = "ScteHls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_hls: Option<ScteHls>,
    #[serde(rename = "StartTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_tag: Option<StartTag>,
    #[serde(rename = "UrlEncodeChildManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_encode_child_manifest: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMssManifestConfiguration {
    #[serde(rename = "FilterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_configuration: Option<FilterConfiguration>,
    #[serde(rename = "ManifestLayout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_layout: Option<String>,
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    pub manifest_name: String,
    #[serde(rename = "ManifestWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_window_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHarvestJobResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "HarvestJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harvest_job_name: Option<String>,
    #[serde(rename = "HarvestedManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harvested_manifests: Option<HarvestedManifests>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "OriginEndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_name: Option<String>,
    #[serde(rename = "ScheduleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_configuration: Option<HarvesterScheduleConfiguration>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetOriginEndpointStateResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "OriginEndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_name: Option<String>,
    #[serde(rename = "ResetAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOriginEndpointResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "ContainerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_type: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "DashManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_manifests: Option<Vec<GetDashManifestConfiguration>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "ForceEndpointErrorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_endpoint_error_configuration: Option<ForceEndpointErrorConfiguration>,
    #[serde(rename = "HlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifests: Option<Vec<GetHlsManifestConfiguration>>,
    #[serde(rename = "LowLatencyHlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_latency_hls_manifests: Option<Vec<GetLowLatencyHlsManifestConfiguration>>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "MssManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_manifests: Option<Vec<GetMssManifestConfiguration>>,
    #[serde(rename = "OriginEndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_name: Option<String>,
    #[serde(rename = "ResetAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_at: Option<f64>,
    #[serde(rename = "Segment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Segment>,
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i32>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOriginEndpointPolicyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelGroupResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "EgressDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_domain: Option<String>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutChannelPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "InputSwitchConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_switch_configuration: Option<InputSwitchConfiguration>,
    #[serde(rename = "OutputHeaderConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_header_configuration: Option<OutputHeaderConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChannelGroupResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "EgressDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_domain: Option<String>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChannelPolicyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChannelGroupsResponse {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ChannelGroupListConfiguration>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelGroupListConfiguration {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOriginEndpointRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "IngestEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_endpoints: Option<Vec<IngestEndpoint>>,
    #[serde(rename = "InputSwitchConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_switch_configuration: Option<InputSwitchConfiguration>,
    #[serde(rename = "InputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "OutputHeaderConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_header_configuration: Option<OutputHeaderConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetHarvestJobRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetHarvestJobResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "HarvestJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harvest_job_name: Option<String>,
    #[serde(rename = "HarvestedManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harvested_manifests: Option<HarvestedManifests>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "OriginEndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_name: Option<String>,
    #[serde(rename = "ScheduleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_configuration: Option<HarvesterScheduleConfiguration>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOriginEndpointRequest {
    #[serde(rename = "ContainerType")]
    #[serde(default)]
    pub container_type: String,
    #[serde(rename = "DashManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_manifests: Option<Vec<CreateDashManifestConfiguration>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "ForceEndpointErrorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_endpoint_error_configuration: Option<ForceEndpointErrorConfiguration>,
    #[serde(rename = "HlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifests: Option<Vec<CreateHlsManifestConfiguration>>,
    #[serde(rename = "LowLatencyHlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_latency_hls_manifests: Option<Vec<CreateLowLatencyHlsManifestConfiguration>>,
    #[serde(rename = "MssManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_manifests: Option<Vec<CreateMssManifestConfiguration>>,
    #[serde(rename = "Segment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Segment>,
    #[serde(rename = "StartoverWindowSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startover_window_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelGroupResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "EgressDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_domain: Option<String>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetChannelStateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutOriginEndpointPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOriginEndpointPolicyResponse {
    #[serde(rename = "CdnAuthConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_auth_configuration: Option<CdnAuthConfiguration>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "OriginEndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_name: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CdnAuthConfiguration {
    #[serde(rename = "CdnIdentifierSecretArns")]
    #[serde(default)]
    pub cdn_identifier_secret_arns: Vec<String>,
    #[serde(rename = "SecretsRoleArn")]
    #[serde(default)]
    pub secrets_role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelPolicyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChannelsResponse {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ChannelListConfiguration>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelListConfiguration {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChannelGroupRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOriginEndpointsResponse {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<OriginEndpointListConfiguration>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OriginEndpointListConfiguration {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ChannelGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group_name: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "ContainerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_type: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "DashManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_manifests: Option<Vec<ListDashManifestConfiguration>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ForceEndpointErrorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_endpoint_error_configuration: Option<ForceEndpointErrorConfiguration>,
    #[serde(rename = "HlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifests: Option<Vec<ListHlsManifestConfiguration>>,
    #[serde(rename = "LowLatencyHlsManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_latency_hls_manifests: Option<Vec<ListLowLatencyHlsManifestConfiguration>>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "MssManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mss_manifests: Option<Vec<ListMssManifestConfiguration>>,
    #[serde(rename = "OriginEndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDashManifestConfiguration {
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListHlsManifestConfiguration {
    #[serde(rename = "ChildManifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_manifest_name: Option<String>,
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLowLatencyHlsManifestConfiguration {
    #[serde(rename = "ChildManifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_manifest_name: Option<String>,
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMssManifestConfiguration {
    #[serde(rename = "ManifestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelRequest {
    #[serde(rename = "ChannelName")]
    #[serde(default)]
    pub channel_name: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InputSwitchConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_switch_configuration: Option<InputSwitchConfiguration>,
    #[serde(rename = "InputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(rename = "OutputHeaderConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_header_configuration: Option<OutputHeaderConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHarvestJobRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: Destination,
    #[serde(rename = "HarvestJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harvest_job_name: Option<String>,
    #[serde(rename = "HarvestedManifests")]
    #[serde(default)]
    pub harvested_manifests: HarvestedManifests,
    #[serde(rename = "ScheduleConfiguration")]
    #[serde(default)]
    pub schedule_configuration: HarvesterScheduleConfiguration,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutOriginEndpointPolicyRequest {
    #[serde(rename = "CdnAuthConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_auth_configuration: Option<CdnAuthConfiguration>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOriginEndpointsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}
