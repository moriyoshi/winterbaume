//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-kinesisvideoarchivedmedia

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClipInput {
    #[serde(rename = "ClipFragmentSelector")]
    #[serde(default)]
    pub clip_fragment_selector: ClipFragmentSelector,
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
pub struct ClipFragmentSelector {
    #[serde(rename = "FragmentSelectorType")]
    #[serde(default)]
    pub fragment_selector_type: String,
    #[serde(rename = "TimestampRange")]
    #[serde(default)]
    pub timestamp_range: ClipTimestampRange,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClipTimestampRange {
    #[serde(rename = "EndTimestamp")]
    #[serde(default)]
    pub end_timestamp: f64,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    pub start_timestamp: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClipOutput {
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDASHStreamingSessionURLInput {
    #[serde(rename = "DASHFragmentSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_a_s_h_fragment_selector: Option<DASHFragmentSelector>,
    #[serde(rename = "DisplayFragmentNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_fragment_number: Option<String>,
    #[serde(rename = "DisplayFragmentTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_fragment_timestamp: Option<String>,
    #[serde(rename = "Expires")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i32>,
    #[serde(rename = "MaxManifestFragmentResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_manifest_fragment_results: Option<i64>,
    #[serde(rename = "PlaybackMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_mode: Option<String>,
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
pub struct DASHFragmentSelector {
    #[serde(rename = "FragmentSelectorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_selector_type: Option<String>,
    #[serde(rename = "TimestampRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_range: Option<DASHTimestampRange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DASHTimestampRange {
    #[serde(rename = "EndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<f64>,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDASHStreamingSessionURLOutput {
    #[serde(rename = "DASHStreamingSessionURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_a_s_h_streaming_session_u_r_l: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetHLSStreamingSessionURLInput {
    #[serde(rename = "ContainerFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_format: Option<String>,
    #[serde(rename = "DiscontinuityMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discontinuity_mode: Option<String>,
    #[serde(rename = "DisplayFragmentTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_fragment_timestamp: Option<String>,
    #[serde(rename = "Expires")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i32>,
    #[serde(rename = "HLSFragmentSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_l_s_fragment_selector: Option<HLSFragmentSelector>,
    #[serde(rename = "MaxMediaPlaylistFragmentResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_media_playlist_fragment_results: Option<i64>,
    #[serde(rename = "PlaybackMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_mode: Option<String>,
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
pub struct HLSFragmentSelector {
    #[serde(rename = "FragmentSelectorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_selector_type: Option<String>,
    #[serde(rename = "TimestampRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_range: Option<HLSTimestampRange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HLSTimestampRange {
    #[serde(rename = "EndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<f64>,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetHLSStreamingSessionURLOutput {
    #[serde(rename = "HLSStreamingSessionURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_l_s_streaming_session_u_r_l: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetImagesInput {
    #[serde(rename = "EndTimestamp")]
    #[serde(default)]
    pub end_timestamp: f64,
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
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SamplingInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_interval: Option<i32>,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    pub start_timestamp: f64,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "WidthPixels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_pixels: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetImagesOutput {
    #[serde(rename = "Images")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Image>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Image {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "ImageContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_content: Option<String>,
    #[serde(rename = "TimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMediaForFragmentListInput {
    #[serde(rename = "Fragments")]
    #[serde(default)]
    pub fragments: Vec<String>,
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
pub struct GetMediaForFragmentListOutput {
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFragmentsInput {
    #[serde(rename = "FragmentSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_selector: Option<FragmentSelector>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
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
pub struct FragmentSelector {
    #[serde(rename = "FragmentSelectorType")]
    #[serde(default)]
    pub fragment_selector_type: String,
    #[serde(rename = "TimestampRange")]
    #[serde(default)]
    pub timestamp_range: TimestampRange,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimestampRange {
    #[serde(rename = "EndTimestamp")]
    #[serde(default)]
    pub end_timestamp: f64,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    pub start_timestamp: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFragmentsOutput {
    #[serde(rename = "Fragments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragments: Option<Vec<Fragment>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Fragment {
    #[serde(rename = "FragmentLengthInMilliseconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_length_in_milliseconds: Option<i64>,
    #[serde(rename = "FragmentNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_number: Option<String>,
    #[serde(rename = "FragmentSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_size_in_bytes: Option<i64>,
    #[serde(rename = "ProducerTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_timestamp: Option<f64>,
    #[serde(rename = "ServerTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_timestamp: Option<f64>,
}
