//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-medialive

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptInputDeviceTransferRequest {
    #[serde(rename = "InputDeviceId")]
    #[serde(default)]
    pub input_device_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptInputDeviceTransferResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteRequest {
    #[serde(rename = "channelIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    #[serde(rename = "inputIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ids: Option<Vec<String>>,
    #[serde(rename = "inputSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_security_group_ids: Option<Vec<String>>,
    #[serde(rename = "multiplexIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<BatchFailedResultModel>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<BatchSuccessfulResultModel>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchFailedResultModel {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchSuccessfulResultModel {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStartRequest {
    #[serde(rename = "channelIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    #[serde(rename = "multiplexIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStartResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<BatchFailedResultModel>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<BatchSuccessfulResultModel>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStopRequest {
    #[serde(rename = "channelIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    #[serde(rename = "multiplexIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchStopResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<BatchFailedResultModel>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<BatchSuccessfulResultModel>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateScheduleRequest {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates: Option<BatchScheduleActionCreateRequest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<BatchScheduleActionDeleteRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchScheduleActionCreateRequest {
    #[serde(rename = "scheduleActions")]
    #[serde(default)]
    pub schedule_actions: Vec<ScheduleAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleAction {
    #[serde(rename = "actionName")]
    #[serde(default)]
    pub action_name: String,
    #[serde(rename = "scheduleActionSettings")]
    #[serde(default)]
    pub schedule_action_settings: ScheduleActionSettings,
    #[serde(rename = "scheduleActionStartSettings")]
    #[serde(default)]
    pub schedule_action_start_settings: ScheduleActionStartSettings,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleActionSettings {
    #[serde(rename = "hlsId3SegmentTaggingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_id3_segment_tagging_settings: Option<HlsId3SegmentTaggingScheduleActionSettings>,
    #[serde(rename = "hlsTimedMetadataSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_timed_metadata_settings: Option<HlsTimedMetadataScheduleActionSettings>,
    #[serde(rename = "id3SegmentTaggingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id3_segment_tagging_settings: Option<Id3SegmentTaggingScheduleActionSettings>,
    #[serde(rename = "inputPrepareSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_prepare_settings: Option<InputPrepareScheduleActionSettings>,
    #[serde(rename = "inputSwitchSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_switch_settings: Option<InputSwitchScheduleActionSettings>,
    #[serde(rename = "motionGraphicsImageActivateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_graphics_image_activate_settings:
        Option<MotionGraphicsActivateScheduleActionSettings>,
    #[serde(rename = "motionGraphicsImageDeactivateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_graphics_image_deactivate_settings:
        Option<MotionGraphicsDeactivateScheduleActionSettings>,
    #[serde(rename = "pauseStateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_state_settings: Option<PauseStateScheduleActionSettings>,
    #[serde(rename = "scte35InputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_input_settings: Option<Scte35InputScheduleActionSettings>,
    #[serde(rename = "scte35ReturnToNetworkSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_return_to_network_settings: Option<Scte35ReturnToNetworkScheduleActionSettings>,
    #[serde(rename = "scte35SpliceInsertSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_splice_insert_settings: Option<Scte35SpliceInsertScheduleActionSettings>,
    #[serde(rename = "scte35TimeSignalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_time_signal_settings: Option<Scte35TimeSignalScheduleActionSettings>,
    #[serde(rename = "staticImageActivateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_image_activate_settings: Option<StaticImageActivateScheduleActionSettings>,
    #[serde(rename = "staticImageDeactivateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_image_deactivate_settings: Option<StaticImageDeactivateScheduleActionSettings>,
    #[serde(rename = "staticImageOutputActivateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_image_output_activate_settings:
        Option<StaticImageOutputActivateScheduleActionSettings>,
    #[serde(rename = "staticImageOutputDeactivateSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_image_output_deactivate_settings:
        Option<StaticImageOutputDeactivateScheduleActionSettings>,
    #[serde(rename = "timedMetadataSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_settings: Option<TimedMetadataScheduleActionSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsId3SegmentTaggingScheduleActionSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id3: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsTimedMetadataScheduleActionSettings {
    #[serde(default)]
    pub id3: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Id3SegmentTaggingScheduleActionSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id3: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputPrepareScheduleActionSettings {
    #[serde(rename = "inputAttachmentNameReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachment_name_reference: Option<String>,
    #[serde(rename = "inputClippingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_clipping_settings: Option<InputClippingSettings>,
    #[serde(rename = "urlPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_path: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputClippingSettings {
    #[serde(rename = "inputTimecodeSource")]
    #[serde(default)]
    pub input_timecode_source: String,
    #[serde(rename = "startTimecode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timecode: Option<StartTimecode>,
    #[serde(rename = "stopTimecode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timecode: Option<StopTimecode>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTimecode {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopTimecode {
    #[serde(rename = "lastFrameClippingBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_frame_clipping_behavior: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputSwitchScheduleActionSettings {
    #[serde(rename = "inputAttachmentNameReference")]
    #[serde(default)]
    pub input_attachment_name_reference: String,
    #[serde(rename = "inputClippingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_clipping_settings: Option<InputClippingSettings>,
    #[serde(rename = "urlPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_path: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MotionGraphicsActivateScheduleActionSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "passwordParam")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_param: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MotionGraphicsDeactivateScheduleActionSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PauseStateScheduleActionSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines: Option<Vec<PipelinePauseStateSettings>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelinePauseStateSettings {
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    pub pipeline_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte35InputScheduleActionSettings {
    #[serde(rename = "inputAttachmentNameReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachment_name_reference: Option<String>,
    #[serde(default)]
    pub mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte35ReturnToNetworkScheduleActionSettings {
    #[serde(rename = "spliceEventId")]
    #[serde(default)]
    pub splice_event_id: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte35SpliceInsertScheduleActionSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "spliceEventId")]
    #[serde(default)]
    pub splice_event_id: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte35TimeSignalScheduleActionSettings {
    #[serde(rename = "scte35Descriptors")]
    #[serde(default)]
    pub scte35_descriptors: Vec<Scte35Descriptor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte35Descriptor {
    #[serde(rename = "scte35DescriptorSettings")]
    #[serde(default)]
    pub scte35_descriptor_settings: Scte35DescriptorSettings,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte35DescriptorSettings {
    #[serde(rename = "segmentationDescriptorScte35DescriptorSettings")]
    #[serde(default)]
    pub segmentation_descriptor_scte35_descriptor_settings: Scte35SegmentationDescriptor,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte35SegmentationDescriptor {
    #[serde(rename = "deliveryRestrictions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_restrictions: Option<Scte35DeliveryRestrictions>,
    #[serde(rename = "segmentNum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_num: Option<i32>,
    #[serde(rename = "segmentationCancelIndicator")]
    #[serde(default)]
    pub segmentation_cancel_indicator: String,
    #[serde(rename = "segmentationDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_duration: Option<i64>,
    #[serde(rename = "segmentationEventId")]
    #[serde(default)]
    pub segmentation_event_id: i64,
    #[serde(rename = "segmentationTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_type_id: Option<i32>,
    #[serde(rename = "segmentationUpid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_upid: Option<String>,
    #[serde(rename = "segmentationUpidType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_upid_type: Option<i32>,
    #[serde(rename = "segmentsExpected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_expected: Option<i32>,
    #[serde(rename = "subSegmentNum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_segment_num: Option<i32>,
    #[serde(rename = "subSegmentsExpected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_segments_expected: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte35DeliveryRestrictions {
    #[serde(rename = "archiveAllowedFlag")]
    #[serde(default)]
    pub archive_allowed_flag: String,
    #[serde(rename = "deviceRestrictions")]
    #[serde(default)]
    pub device_restrictions: String,
    #[serde(rename = "noRegionalBlackoutFlag")]
    #[serde(default)]
    pub no_regional_blackout_flag: String,
    #[serde(rename = "webDeliveryAllowedFlag")]
    #[serde(default)]
    pub web_delivery_allowed_flag: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StaticImageActivateScheduleActionSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "fadeIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in: Option<i32>,
    #[serde(rename = "fadeOut")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(default)]
    pub image: InputLocation,
    #[serde(rename = "imageX")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_x: Option<i32>,
    #[serde(rename = "imageY")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_y: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputLocation {
    #[serde(rename = "passwordParam")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_param: Option<String>,
    #[serde(default)]
    pub uri: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StaticImageDeactivateScheduleActionSettings {
    #[serde(rename = "fadeOut")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StaticImageOutputActivateScheduleActionSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "fadeIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in: Option<i32>,
    #[serde(rename = "fadeOut")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(default)]
    pub image: InputLocation,
    #[serde(rename = "imageX")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_x: Option<i32>,
    #[serde(rename = "imageY")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_y: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<i32>,
    #[serde(rename = "outputNames")]
    #[serde(default)]
    pub output_names: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StaticImageOutputDeactivateScheduleActionSettings {
    #[serde(rename = "fadeOut")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<i32>,
    #[serde(rename = "outputNames")]
    #[serde(default)]
    pub output_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimedMetadataScheduleActionSettings {
    #[serde(default)]
    pub id3: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleActionStartSettings {
    #[serde(rename = "fixedModeScheduleActionStartSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_mode_schedule_action_start_settings: Option<FixedModeScheduleActionStartSettings>,
    #[serde(rename = "followModeScheduleActionStartSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_mode_schedule_action_start_settings: Option<FollowModeScheduleActionStartSettings>,
    #[serde(rename = "immediateModeScheduleActionStartSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immediate_mode_schedule_action_start_settings:
        Option<ImmediateModeScheduleActionStartSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FixedModeScheduleActionStartSettings {
    #[serde(default)]
    pub time: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FollowModeScheduleActionStartSettings {
    #[serde(rename = "followPoint")]
    #[serde(default)]
    pub follow_point: String,
    #[serde(rename = "referenceActionName")]
    #[serde(default)]
    pub reference_action_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImmediateModeScheduleActionStartSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchScheduleActionDeleteRequest {
    #[serde(rename = "actionNames")]
    #[serde(default)]
    pub action_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateScheduleResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates: Option<BatchScheduleActionCreateResult>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<BatchScheduleActionDeleteResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchScheduleActionCreateResult {
    #[serde(rename = "scheduleActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_actions: Option<Vec<ScheduleAction>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchScheduleActionDeleteResult {
    #[serde(rename = "scheduleActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_actions: Option<Vec<ScheduleAction>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelInputDeviceTransferRequest {
    #[serde(rename = "InputDeviceId")]
    #[serde(default)]
    pub input_device_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelInputDeviceTransferResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClaimDeviceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClaimDeviceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelPlacementGroupRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelPlacementGroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "clusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelRequest {
    #[serde(rename = "anywhereSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anywhere_settings: Option<AnywhereSettings>,
    #[serde(rename = "cdiInputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    #[serde(rename = "channelClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    #[serde(rename = "channelEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_engine_version: Option<ChannelEngineVersionRequest>,
    #[serde(rename = "channelSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    #[serde(rename = "dryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "encoderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    #[serde(rename = "inferenceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_settings: Option<InferenceSettings>,
    #[serde(rename = "inputAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    #[serde(rename = "inputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    #[serde(rename = "linkedChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_channel_settings: Option<LinkedChannelSettings>,
    #[serde(rename = "logLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceCreateSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnywhereSettings {
    #[serde(rename = "channelPlacementGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_placement_group_id: Option<String>,
    #[serde(rename = "clusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CdiInputSpecification {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelEngineVersionRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputDestination {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "logicalInterfaceNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_interface_names: Option<Vec<String>>,
    #[serde(rename = "mediaConnectRouterSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_router_settings: Option<Vec<MediaConnectRouterOutputDestinationSettings>>,
    #[serde(rename = "mediaPackageSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_package_settings: Option<Vec<MediaPackageOutputDestinationSettings>>,
    #[serde(rename = "multiplexSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexProgramChannelDestinationSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<OutputDestinationSettings>>,
    #[serde(rename = "srtSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_settings: Option<Vec<SrtOutputDestinationSettings>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaConnectRouterOutputDestinationSettings {
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaPackageOutputDestinationSettings {
    #[serde(rename = "channelEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_endpoint_id: Option<String>,
    #[serde(rename = "channelGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_group: Option<String>,
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "channelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "mediaPackageRegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_package_region_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexProgramChannelDestinationSettings {
    #[serde(rename = "multiplexId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_id: Option<String>,
    #[serde(rename = "programName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputDestinationSettings {
    #[serde(rename = "passwordParam")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_param: Option<String>,
    #[serde(rename = "streamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtOutputDestinationSettings {
    #[serde(rename = "connectionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_mode: Option<String>,
    #[serde(rename = "encryptionPassphraseSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_passphrase_secret_arn: Option<String>,
    #[serde(rename = "listenerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<i32>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncoderSettings {
    #[serde(rename = "audioDescriptions")]
    #[serde(default)]
    pub audio_descriptions: Vec<AudioDescription>,
    #[serde(rename = "availBlanking")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_blanking: Option<AvailBlanking>,
    #[serde(rename = "availConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_configuration: Option<AvailConfiguration>,
    #[serde(rename = "blackoutSlate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blackout_slate: Option<BlackoutSlate>,
    #[serde(rename = "captionDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_descriptions: Option<Vec<CaptionDescription>>,
    #[serde(rename = "colorCorrectionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_correction_settings: Option<ColorCorrectionSettings>,
    #[serde(rename = "featureActivations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_activations: Option<FeatureActivations>,
    #[serde(rename = "globalConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_configuration: Option<GlobalConfiguration>,
    #[serde(rename = "motionGraphicsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_graphics_configuration: Option<MotionGraphicsConfiguration>,
    #[serde(rename = "nielsenConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_configuration: Option<NielsenConfiguration>,
    #[serde(rename = "outputGroups")]
    #[serde(default)]
    pub output_groups: Vec<OutputGroup>,
    #[serde(rename = "thumbnailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_configuration: Option<ThumbnailConfiguration>,
    #[serde(rename = "timecodeConfig")]
    #[serde(default)]
    pub timecode_config: TimecodeConfig,
    #[serde(rename = "videoDescriptions")]
    #[serde(default)]
    pub video_descriptions: Vec<VideoDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioDescription {
    #[serde(rename = "audioDashRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_dash_roles: Option<Vec<String>>,
    #[serde(rename = "audioNormalizationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_normalization_settings: Option<AudioNormalizationSettings>,
    #[serde(rename = "audioSelectorName")]
    #[serde(default)]
    pub audio_selector_name: String,
    #[serde(rename = "audioType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_type: Option<String>,
    #[serde(rename = "audioTypeControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_type_control: Option<String>,
    #[serde(rename = "audioWatermarkingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_watermarking_settings: Option<AudioWatermarkSettings>,
    #[serde(rename = "codecSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_settings: Option<AudioCodecSettings>,
    #[serde(rename = "dvbDashAccessibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_dash_accessibility: Option<String>,
    #[serde(rename = "languageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "languageCodeControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code_control: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "remixSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remix_settings: Option<RemixSettings>,
    #[serde(rename = "streamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioNormalizationSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(rename = "algorithmControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_control: Option<String>,
    #[serde(rename = "targetLkfs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_lkfs: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioWatermarkSettings {
    #[serde(rename = "nielsenWatermarksSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_watermarks_settings: Option<NielsenWatermarksSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NielsenWatermarksSettings {
    #[serde(rename = "nielsenCbetSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_cbet_settings: Option<NielsenCBET>,
    #[serde(rename = "nielsenDistributionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_distribution_type: Option<String>,
    #[serde(rename = "nielsenNaesIiNwSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_naes_ii_nw_settings: Option<NielsenNaesIiNw>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NielsenCBET {
    #[serde(rename = "cbetCheckDigitString")]
    #[serde(default)]
    pub cbet_check_digit_string: String,
    #[serde(rename = "cbetStepaside")]
    #[serde(default)]
    pub cbet_stepaside: String,
    #[serde(default)]
    pub csid: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NielsenNaesIiNw {
    #[serde(rename = "checkDigitString")]
    #[serde(default)]
    pub check_digit_string: String,
    #[serde(default)]
    pub sid: f64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioCodecSettings {
    #[serde(rename = "aacSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aac_settings: Option<AacSettings>,
    #[serde(rename = "ac3Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac3_settings: Option<Ac3Settings>,
    #[serde(rename = "eac3AtmosSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eac3_atmos_settings: Option<Eac3AtmosSettings>,
    #[serde(rename = "eac3Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eac3_settings: Option<Eac3Settings>,
    #[serde(rename = "mp2Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp2_settings: Option<Mp2Settings>,
    #[serde(rename = "passThroughSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_through_settings: Option<PassThroughSettings>,
    #[serde(rename = "wavSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wav_settings: Option<WavSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AacSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,
    #[serde(rename = "codingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    #[serde(rename = "inputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(rename = "rateControlMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    #[serde(rename = "rawFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_format: Option<String>,
    #[serde(rename = "sampleRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<String>,
    #[serde(rename = "vbrQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbr_quality: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ac3Settings {
    #[serde(rename = "attenuationControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attenuation_control: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,
    #[serde(rename = "bitstreamMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream_mode: Option<String>,
    #[serde(rename = "codingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialnorm: Option<i32>,
    #[serde(rename = "drcProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drc_profile: Option<String>,
    #[serde(rename = "lfeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_filter: Option<String>,
    #[serde(rename = "metadataControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_control: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Eac3AtmosSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,
    #[serde(rename = "codingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialnorm: Option<i32>,
    #[serde(rename = "drcLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drc_line: Option<String>,
    #[serde(rename = "drcRf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drc_rf: Option<String>,
    #[serde(rename = "heightTrim")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_trim: Option<f64>,
    #[serde(rename = "surroundTrim")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround_trim: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Eac3Settings {
    #[serde(rename = "attenuationControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attenuation_control: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,
    #[serde(rename = "bitstreamMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream_mode: Option<String>,
    #[serde(rename = "codingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    #[serde(rename = "dcFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_filter: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialnorm: Option<i32>,
    #[serde(rename = "drcLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drc_line: Option<String>,
    #[serde(rename = "drcRf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drc_rf: Option<String>,
    #[serde(rename = "lfeControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_control: Option<String>,
    #[serde(rename = "lfeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_filter: Option<String>,
    #[serde(rename = "loRoCenterMixLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_center_mix_level: Option<f64>,
    #[serde(rename = "loRoSurroundMixLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_surround_mix_level: Option<f64>,
    #[serde(rename = "ltRtCenterMixLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_center_mix_level: Option<f64>,
    #[serde(rename = "ltRtSurroundMixLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_surround_mix_level: Option<f64>,
    #[serde(rename = "metadataControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_control: Option<String>,
    #[serde(rename = "passthroughControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_control: Option<String>,
    #[serde(rename = "phaseControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_control: Option<String>,
    #[serde(rename = "stereoDownmix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stereo_downmix: Option<String>,
    #[serde(rename = "surroundExMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround_ex_mode: Option<String>,
    #[serde(rename = "surroundMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Mp2Settings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,
    #[serde(rename = "codingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    #[serde(rename = "sampleRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PassThroughSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WavSettings {
    #[serde(rename = "bitDepth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<f64>,
    #[serde(rename = "codingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    #[serde(rename = "sampleRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemixSettings {
    #[serde(rename = "channelMappings")]
    #[serde(default)]
    pub channel_mappings: Vec<AudioChannelMapping>,
    #[serde(rename = "channelsIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels_in: Option<i32>,
    #[serde(rename = "channelsOut")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels_out: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioChannelMapping {
    #[serde(rename = "inputChannelLevels")]
    #[serde(default)]
    pub input_channel_levels: Vec<InputChannelLevel>,
    #[serde(rename = "outputChannel")]
    #[serde(default)]
    pub output_channel: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputChannelLevel {
    #[serde(default)]
    pub gain: i32,
    #[serde(rename = "inputChannel")]
    #[serde(default)]
    pub input_channel: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailBlanking {
    #[serde(rename = "availBlankingImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_blanking_image: Option<InputLocation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailConfiguration {
    #[serde(rename = "availSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_settings: Option<AvailSettings>,
    #[serde(rename = "scte35SegmentationScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_segmentation_scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esam: Option<Esam>,
    #[serde(rename = "scte35SpliceInsert")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_splice_insert: Option<Scte35SpliceInsert>,
    #[serde(rename = "scte35TimeSignalApos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_time_signal_apos: Option<Scte35TimeSignalApos>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Esam {
    #[serde(rename = "acquisitionPointId")]
    #[serde(default)]
    pub acquisition_point_id: String,
    #[serde(rename = "adAvailOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_avail_offset: Option<i32>,
    #[serde(rename = "passwordParam")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_param: Option<String>,
    #[serde(rename = "poisEndpoint")]
    #[serde(default)]
    pub pois_endpoint: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "zoneIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_identity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte35SpliceInsert {
    #[serde(rename = "adAvailOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_avail_offset: Option<i32>,
    #[serde(rename = "noRegionalBlackoutFlag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_regional_blackout_flag: Option<String>,
    #[serde(rename = "webDeliveryAllowedFlag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_delivery_allowed_flag: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte35TimeSignalApos {
    #[serde(rename = "adAvailOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_avail_offset: Option<i32>,
    #[serde(rename = "noRegionalBlackoutFlag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_regional_blackout_flag: Option<String>,
    #[serde(rename = "webDeliveryAllowedFlag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_delivery_allowed_flag: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlackoutSlate {
    #[serde(rename = "blackoutSlateImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blackout_slate_image: Option<InputLocation>,
    #[serde(rename = "networkEndBlackout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_end_blackout: Option<String>,
    #[serde(rename = "networkEndBlackoutImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_end_blackout_image: Option<InputLocation>,
    #[serde(rename = "networkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CaptionDescription {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<String>,
    #[serde(rename = "captionDashRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_dash_roles: Option<Vec<String>>,
    #[serde(rename = "captionSelectorName")]
    #[serde(default)]
    pub caption_selector_name: String,
    #[serde(rename = "destinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<CaptionDestinationSettings>,
    #[serde(rename = "dvbDashAccessibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_dash_accessibility: Option<String>,
    #[serde(rename = "languageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "languageDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_description: Option<String>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CaptionDestinationSettings {
    #[serde(rename = "aribDestinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib_destination_settings: Option<AribDestinationSettings>,
    #[serde(rename = "burnInDestinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burn_in_destination_settings: Option<BurnInDestinationSettings>,
    #[serde(rename = "dvbSubDestinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_destination_settings: Option<DvbSubDestinationSettings>,
    #[serde(rename = "ebuTtDDestinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebu_tt_d_destination_settings: Option<EbuTtDDestinationSettings>,
    #[serde(rename = "embeddedDestinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_destination_settings: Option<EmbeddedDestinationSettings>,
    #[serde(rename = "embeddedPlusScte20DestinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_plus_scte20_destination_settings: Option<EmbeddedPlusScte20DestinationSettings>,
    #[serde(rename = "rtmpCaptionInfoDestinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtmp_caption_info_destination_settings: Option<RtmpCaptionInfoDestinationSettings>,
    #[serde(rename = "scte20PlusEmbeddedDestinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte20_plus_embedded_destination_settings: Option<Scte20PlusEmbeddedDestinationSettings>,
    #[serde(rename = "scte27DestinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte27_destination_settings: Option<Scte27DestinationSettings>,
    #[serde(rename = "smpteTtDestinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smpte_tt_destination_settings: Option<SmpteTtDestinationSettings>,
    #[serde(rename = "teletextDestinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_destination_settings: Option<TeletextDestinationSettings>,
    #[serde(rename = "ttmlDestinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttml_destination_settings: Option<TtmlDestinationSettings>,
    #[serde(rename = "webvttDestinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webvtt_destination_settings: Option<WebvttDestinationSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AribDestinationSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BurnInDestinationSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
    #[serde(rename = "backgroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "backgroundOpacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<InputLocation>,
    #[serde(rename = "fontColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<String>,
    #[serde(rename = "fontOpacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_opacity: Option<i32>,
    #[serde(rename = "fontResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_resolution: Option<i32>,
    #[serde(rename = "fontSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<String>,
    #[serde(rename = "outlineColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<String>,
    #[serde(rename = "outlineSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_size: Option<i32>,
    #[serde(rename = "shadowColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_color: Option<String>,
    #[serde(rename = "shadowOpacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_opacity: Option<i32>,
    #[serde(rename = "shadowXOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_x_offset: Option<i32>,
    #[serde(rename = "shadowYOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_y_offset: Option<i32>,
    #[serde(rename = "subtitleRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle_rows: Option<String>,
    #[serde(rename = "teletextGridControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_grid_control: Option<String>,
    #[serde(rename = "xPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_position: Option<i32>,
    #[serde(rename = "yPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_position: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DvbSubDestinationSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
    #[serde(rename = "backgroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "backgroundOpacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<InputLocation>,
    #[serde(rename = "fontColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<String>,
    #[serde(rename = "fontOpacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_opacity: Option<i32>,
    #[serde(rename = "fontResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_resolution: Option<i32>,
    #[serde(rename = "fontSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<String>,
    #[serde(rename = "outlineColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<String>,
    #[serde(rename = "outlineSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_size: Option<i32>,
    #[serde(rename = "shadowColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_color: Option<String>,
    #[serde(rename = "shadowOpacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_opacity: Option<i32>,
    #[serde(rename = "shadowXOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_x_offset: Option<i32>,
    #[serde(rename = "shadowYOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_y_offset: Option<i32>,
    #[serde(rename = "subtitleRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle_rows: Option<String>,
    #[serde(rename = "teletextGridControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_grid_control: Option<String>,
    #[serde(rename = "xPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_position: Option<i32>,
    #[serde(rename = "yPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_position: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EbuTtDDestinationSettings {
    #[serde(rename = "copyrightHolder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright_holder: Option<String>,
    #[serde(rename = "defaultFontSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_font_size: Option<i32>,
    #[serde(rename = "defaultLineHeight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_line_height: Option<i32>,
    #[serde(rename = "fillLineGap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_line_gap: Option<String>,
    #[serde(rename = "fontFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(rename = "styleControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_control: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmbeddedDestinationSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmbeddedPlusScte20DestinationSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RtmpCaptionInfoDestinationSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte20PlusEmbeddedDestinationSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte27DestinationSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SmpteTtDestinationSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TeletextDestinationSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TtmlDestinationSettings {
    #[serde(rename = "styleControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_control: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebvttDestinationSettings {
    #[serde(rename = "styleControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_control: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColorCorrectionSettings {
    #[serde(rename = "globalColorCorrections")]
    #[serde(default)]
    pub global_color_corrections: Vec<ColorCorrection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColorCorrection {
    #[serde(rename = "inputColorSpace")]
    #[serde(default)]
    pub input_color_space: String,
    #[serde(rename = "outputColorSpace")]
    #[serde(default)]
    pub output_color_space: String,
    #[serde(default)]
    pub uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FeatureActivations {
    #[serde(rename = "inputPrepareScheduleActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_prepare_schedule_actions: Option<String>,
    #[serde(rename = "outputStaticImageOverlayScheduleActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_static_image_overlay_schedule_actions: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalConfiguration {
    #[serde(rename = "initialAudioGain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_audio_gain: Option<i32>,
    #[serde(rename = "inputEndAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_end_action: Option<String>,
    #[serde(rename = "inputLossBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_behavior: Option<InputLossBehavior>,
    #[serde(rename = "outputLockingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_locking_mode: Option<String>,
    #[serde(rename = "outputLockingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_locking_settings: Option<OutputLockingSettings>,
    #[serde(rename = "outputTimingSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_timing_source: Option<String>,
    #[serde(rename = "supportLowFramerateInputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_low_framerate_inputs: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputLossBehavior {
    #[serde(rename = "blackFrameMsec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_frame_msec: Option<i32>,
    #[serde(rename = "inputLossImageColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_image_color: Option<String>,
    #[serde(rename = "inputLossImageSlate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_image_slate: Option<InputLocation>,
    #[serde(rename = "inputLossImageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_image_type: Option<String>,
    #[serde(rename = "repeatFrameMsec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_frame_msec: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputLockingSettings {
    #[serde(rename = "disabledLockingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_locking_settings: Option<DisabledLockingSettings>,
    #[serde(rename = "epochLockingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch_locking_settings: Option<EpochLockingSettings>,
    #[serde(rename = "pipelineLockingSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_locking_settings: Option<PipelineLockingSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisabledLockingSettings {
    #[serde(rename = "customEpoch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_epoch: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EpochLockingSettings {
    #[serde(rename = "customEpoch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_epoch: Option<String>,
    #[serde(rename = "jamSyncTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jam_sync_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineLockingSettings {
    #[serde(rename = "customEpoch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_epoch: Option<String>,
    #[serde(rename = "pipelineLockingMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_locking_method: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MotionGraphicsConfiguration {
    #[serde(rename = "motionGraphicsInsertion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_graphics_insertion: Option<String>,
    #[serde(rename = "motionGraphicsSettings")]
    #[serde(default)]
    pub motion_graphics_settings: MotionGraphicsSettings,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MotionGraphicsSettings {
    #[serde(rename = "htmlMotionGraphicsSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_motion_graphics_settings: Option<HtmlMotionGraphicsSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HtmlMotionGraphicsSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NielsenConfiguration {
    #[serde(rename = "distributorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor_id: Option<String>,
    #[serde(rename = "nielsenPcmToId3Tagging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_pcm_to_id3_tagging: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "outputGroupSettings")]
    #[serde(default)]
    pub output_group_settings: OutputGroupSettings,
    #[serde(default)]
    pub outputs: Vec<Output>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputGroupSettings {
    #[serde(rename = "archiveGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_group_settings: Option<ArchiveGroupSettings>,
    #[serde(rename = "cmafIngestGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_ingest_group_settings: Option<CmafIngestGroupSettings>,
    #[serde(rename = "frameCaptureGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_group_settings: Option<FrameCaptureGroupSettings>,
    #[serde(rename = "hlsGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_group_settings: Option<HlsGroupSettings>,
    #[serde(rename = "mediaConnectRouterGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_router_group_settings: Option<MediaConnectRouterGroupSettings>,
    #[serde(rename = "mediaPackageGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_package_group_settings: Option<MediaPackageGroupSettings>,
    #[serde(rename = "msSmoothGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_smooth_group_settings: Option<MsSmoothGroupSettings>,
    #[serde(rename = "multiplexGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_group_settings: Option<MultiplexGroupSettings>,
    #[serde(rename = "rtmpGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtmp_group_settings: Option<RtmpGroupSettings>,
    #[serde(rename = "srtGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_group_settings: Option<SrtGroupSettings>,
    #[serde(rename = "udpGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_group_settings: Option<UdpGroupSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArchiveGroupSettings {
    #[serde(rename = "archiveCdnSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_cdn_settings: Option<ArchiveCdnSettings>,
    #[serde(default)]
    pub destination: OutputLocationRef,
    #[serde(rename = "rolloverInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_interval: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArchiveCdnSettings {
    #[serde(rename = "archiveS3Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_s3_settings: Option<ArchiveS3Settings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArchiveS3Settings {
    #[serde(rename = "cannedAcl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputLocationRef {
    #[serde(rename = "destinationRefId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ref_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CmafIngestGroupSettings {
    #[serde(rename = "additionalDestinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_destinations: Option<Vec<AdditionalDestinations>>,
    #[serde(rename = "captionLanguageMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_language_mappings: Option<Vec<CmafIngestCaptionLanguageMapping>>,
    #[serde(default)]
    pub destination: OutputLocationRef,
    #[serde(rename = "id3Behavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id3_behavior: Option<String>,
    #[serde(rename = "id3NameModifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id3_name_modifier: Option<String>,
    #[serde(rename = "klvBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv_behavior: Option<String>,
    #[serde(rename = "klvNameModifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv_name_modifier: Option<String>,
    #[serde(rename = "nielsenId3Behavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id3_behavior: Option<String>,
    #[serde(rename = "nielsenId3NameModifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id3_name_modifier: Option<String>,
    #[serde(rename = "scte35NameModifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_name_modifier: Option<String>,
    #[serde(rename = "scte35Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_type: Option<String>,
    #[serde(rename = "segmentLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<i32>,
    #[serde(rename = "segmentLengthUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_length_units: Option<String>,
    #[serde(rename = "sendDelayMs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_delay_ms: Option<i32>,
    #[serde(rename = "timedMetadataId3Frame")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id3_frame: Option<String>,
    #[serde(rename = "timedMetadataId3Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id3_period: Option<i32>,
    #[serde(rename = "timedMetadataPassthrough")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_passthrough: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdditionalDestinations {
    #[serde(default)]
    pub destination: OutputLocationRef,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CmafIngestCaptionLanguageMapping {
    #[serde(rename = "captionChannel")]
    #[serde(default)]
    pub caption_channel: i32,
    #[serde(rename = "languageCode")]
    #[serde(default)]
    pub language_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FrameCaptureGroupSettings {
    #[serde(default)]
    pub destination: OutputLocationRef,
    #[serde(rename = "frameCaptureCdnSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_cdn_settings: Option<FrameCaptureCdnSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FrameCaptureCdnSettings {
    #[serde(rename = "frameCaptureS3Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_s3_settings: Option<FrameCaptureS3Settings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FrameCaptureS3Settings {
    #[serde(rename = "cannedAcl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsGroupSettings {
    #[serde(rename = "adMarkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<Vec<String>>,
    #[serde(rename = "baseUrlContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url_content: Option<String>,
    #[serde(rename = "baseUrlContent1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url_content1: Option<String>,
    #[serde(rename = "baseUrlManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url_manifest: Option<String>,
    #[serde(rename = "baseUrlManifest1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url_manifest1: Option<String>,
    #[serde(rename = "captionLanguageMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_language_mappings: Option<Vec<CaptionLanguageMapping>>,
    #[serde(rename = "captionLanguageSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_language_setting: Option<String>,
    #[serde(rename = "clientCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cache: Option<String>,
    #[serde(rename = "codecSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_specification: Option<String>,
    #[serde(rename = "constantIv")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_iv: Option<String>,
    #[serde(default)]
    pub destination: OutputLocationRef,
    #[serde(rename = "directoryStructure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_structure: Option<String>,
    #[serde(rename = "discontinuityTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discontinuity_tags: Option<String>,
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "hlsCdnSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_cdn_settings: Option<HlsCdnSettings>,
    #[serde(rename = "hlsId3SegmentTagging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_id3_segment_tagging: Option<String>,
    #[serde(rename = "iFrameOnlyPlaylists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_frame_only_playlists: Option<String>,
    #[serde(rename = "incompleteSegmentBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_segment_behavior: Option<String>,
    #[serde(rename = "indexNSegments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_n_segments: Option<i32>,
    #[serde(rename = "inputLossAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_action: Option<String>,
    #[serde(rename = "ivInManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv_in_manifest: Option<String>,
    #[serde(rename = "ivSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv_source: Option<String>,
    #[serde(rename = "keepSegments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_segments: Option<i32>,
    #[serde(rename = "keyFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_format: Option<String>,
    #[serde(rename = "keyFormatVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_format_versions: Option<String>,
    #[serde(rename = "keyProviderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_provider_settings: Option<KeyProviderSettings>,
    #[serde(rename = "manifestCompression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_compression: Option<String>,
    #[serde(rename = "manifestDurationFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_duration_format: Option<String>,
    #[serde(rename = "minSegmentLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_segment_length: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "outputSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_selection: Option<String>,
    #[serde(rename = "programDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time: Option<String>,
    #[serde(rename = "programDateTimeClock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_clock: Option<String>,
    #[serde(rename = "programDateTimePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_period: Option<i32>,
    #[serde(rename = "redundantManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_manifest: Option<String>,
    #[serde(rename = "segmentLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<i32>,
    #[serde(rename = "segmentationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_mode: Option<String>,
    #[serde(rename = "segmentsPerSubdirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_per_subdirectory: Option<i32>,
    #[serde(rename = "streamInfResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_inf_resolution: Option<String>,
    #[serde(rename = "timedMetadataId3Frame")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id3_frame: Option<String>,
    #[serde(rename = "timedMetadataId3Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id3_period: Option<i32>,
    #[serde(rename = "timestampDeltaMilliseconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_delta_milliseconds: Option<i32>,
    #[serde(rename = "tsFileMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_file_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CaptionLanguageMapping {
    #[serde(rename = "captionChannel")]
    #[serde(default)]
    pub caption_channel: i32,
    #[serde(rename = "languageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "languageDescription")]
    #[serde(default)]
    pub language_description: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsCdnSettings {
    #[serde(rename = "hlsAkamaiSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_akamai_settings: Option<HlsAkamaiSettings>,
    #[serde(rename = "hlsBasicPutSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_basic_put_settings: Option<HlsBasicPutSettings>,
    #[serde(rename = "hlsMediaStoreSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_media_store_settings: Option<HlsMediaStoreSettings>,
    #[serde(rename = "hlsS3Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_s3_settings: Option<HlsS3Settings>,
    #[serde(rename = "hlsWebdavSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_webdav_settings: Option<HlsWebdavSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsAkamaiSettings {
    #[serde(rename = "connectionRetryInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i32>,
    #[serde(rename = "filecacheDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i32>,
    #[serde(rename = "httpTransferMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_transfer_mode: Option<String>,
    #[serde(rename = "numRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i32>,
    #[serde(rename = "restartDelay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsBasicPutSettings {
    #[serde(rename = "connectionRetryInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i32>,
    #[serde(rename = "filecacheDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i32>,
    #[serde(rename = "numRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i32>,
    #[serde(rename = "restartDelay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsMediaStoreSettings {
    #[serde(rename = "connectionRetryInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i32>,
    #[serde(rename = "filecacheDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i32>,
    #[serde(rename = "mediaStoreStorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_store_storage_class: Option<String>,
    #[serde(rename = "numRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i32>,
    #[serde(rename = "restartDelay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsS3Settings {
    #[serde(rename = "cannedAcl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsWebdavSettings {
    #[serde(rename = "connectionRetryInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i32>,
    #[serde(rename = "filecacheDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i32>,
    #[serde(rename = "httpTransferMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_transfer_mode: Option<String>,
    #[serde(rename = "numRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i32>,
    #[serde(rename = "restartDelay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyProviderSettings {
    #[serde(rename = "staticKeySettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_key_settings: Option<StaticKeySettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StaticKeySettings {
    #[serde(rename = "keyProviderServer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_provider_server: Option<InputLocation>,
    #[serde(rename = "staticKeyValue")]
    #[serde(default)]
    pub static_key_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaConnectRouterGroupSettings {
    #[serde(rename = "availabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaPackageGroupSettings {
    #[serde(default)]
    pub destination: OutputLocationRef,
    #[serde(rename = "mediapackageV2GroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediapackage_v2_group_settings: Option<MediaPackageV2GroupSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaPackageV2GroupSettings {
    #[serde(rename = "additionalDestinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_destinations: Option<Vec<MediaPackageAdditionalDestinations>>,
    #[serde(rename = "captionLanguageMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_language_mappings: Option<Vec<CaptionLanguageMapping>>,
    #[serde(rename = "id3Behavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id3_behavior: Option<String>,
    #[serde(rename = "klvBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv_behavior: Option<String>,
    #[serde(rename = "nielsenId3Behavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id3_behavior: Option<String>,
    #[serde(rename = "scte35Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_type: Option<String>,
    #[serde(rename = "segmentLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<i32>,
    #[serde(rename = "segmentLengthUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_length_units: Option<String>,
    #[serde(rename = "timedMetadataId3Frame")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id3_frame: Option<String>,
    #[serde(rename = "timedMetadataId3Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id3_period: Option<i32>,
    #[serde(rename = "timedMetadataPassthrough")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_passthrough: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaPackageAdditionalDestinations {
    #[serde(default)]
    pub destination: OutputLocationRef,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MsSmoothGroupSettings {
    #[serde(rename = "acquisitionPointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_point_id: Option<String>,
    #[serde(rename = "audioOnlyTimecodeControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only_timecode_control: Option<String>,
    #[serde(rename = "certificateMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_mode: Option<String>,
    #[serde(rename = "connectionRetryInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i32>,
    #[serde(default)]
    pub destination: OutputLocationRef,
    #[serde(rename = "eventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "eventIdMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id_mode: Option<String>,
    #[serde(rename = "eventStopBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_stop_behavior: Option<String>,
    #[serde(rename = "filecacheDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i32>,
    #[serde(rename = "fragmentLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_length: Option<i32>,
    #[serde(rename = "inputLossAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_action: Option<String>,
    #[serde(rename = "numRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i32>,
    #[serde(rename = "restartDelay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i32>,
    #[serde(rename = "segmentationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_mode: Option<String>,
    #[serde(rename = "sendDelayMs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_delay_ms: Option<i32>,
    #[serde(rename = "sparseTrackType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparse_track_type: Option<String>,
    #[serde(rename = "streamManifestBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_manifest_behavior: Option<String>,
    #[serde(rename = "timestampOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_offset: Option<String>,
    #[serde(rename = "timestampOffsetMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_offset_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexGroupSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RtmpGroupSettings {
    #[serde(rename = "adMarkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<Vec<String>>,
    #[serde(rename = "authenticationScheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_scheme: Option<String>,
    #[serde(rename = "cacheFullBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_full_behavior: Option<String>,
    #[serde(rename = "cacheLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_length: Option<i32>,
    #[serde(rename = "captionData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_data: Option<String>,
    #[serde(rename = "includeFillerNalUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_filler_nal_units: Option<String>,
    #[serde(rename = "inputLossAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_action: Option<String>,
    #[serde(rename = "restartDelay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtGroupSettings {
    #[serde(rename = "inputLossAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_action: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UdpGroupSettings {
    #[serde(rename = "inputLossAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_action: Option<String>,
    #[serde(rename = "timedMetadataId3Frame")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id3_frame: Option<String>,
    #[serde(rename = "timedMetadataId3Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id3_period: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Output {
    #[serde(rename = "audioDescriptionNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_description_names: Option<Vec<String>>,
    #[serde(rename = "captionDescriptionNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_description_names: Option<Vec<String>>,
    #[serde(rename = "outputName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_name: Option<String>,
    #[serde(rename = "outputSettings")]
    #[serde(default)]
    pub output_settings: OutputSettings,
    #[serde(rename = "videoDescriptionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_description_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputSettings {
    #[serde(rename = "archiveOutputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_output_settings: Option<ArchiveOutputSettings>,
    #[serde(rename = "cmafIngestOutputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_ingest_output_settings: Option<CmafIngestOutputSettings>,
    #[serde(rename = "frameCaptureOutputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_output_settings: Option<FrameCaptureOutputSettings>,
    #[serde(rename = "hlsOutputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_output_settings: Option<HlsOutputSettings>,
    #[serde(rename = "mediaConnectRouterOutputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_router_output_settings: Option<MediaConnectRouterOutputSettings>,
    #[serde(rename = "mediaPackageOutputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_package_output_settings: Option<MediaPackageOutputSettings>,
    #[serde(rename = "msSmoothOutputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_smooth_output_settings: Option<MsSmoothOutputSettings>,
    #[serde(rename = "multiplexOutputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_output_settings: Option<MultiplexOutputSettings>,
    #[serde(rename = "rtmpOutputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtmp_output_settings: Option<RtmpOutputSettings>,
    #[serde(rename = "srtOutputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_output_settings: Option<SrtOutputSettings>,
    #[serde(rename = "udpOutputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_output_settings: Option<UdpOutputSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArchiveOutputSettings {
    #[serde(rename = "containerSettings")]
    #[serde(default)]
    pub container_settings: ArchiveContainerSettings,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(rename = "nameModifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArchiveContainerSettings {
    #[serde(rename = "m2tsSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m2ts_settings: Option<M2tsSettings>,
    #[serde(rename = "rawSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_settings: Option<RawSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct M2tsSettings {
    #[serde(rename = "absentInputAudioBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absent_input_audio_behavior: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib: Option<String>,
    #[serde(rename = "aribCaptionsPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib_captions_pid: Option<String>,
    #[serde(rename = "aribCaptionsPidControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib_captions_pid_control: Option<String>,
    #[serde(rename = "audioBufferModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_buffer_model: Option<String>,
    #[serde(rename = "audioFramesPerPes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_frames_per_pes: Option<i32>,
    #[serde(rename = "audioPids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pids: Option<String>,
    #[serde(rename = "audioStreamType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_stream_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    #[serde(rename = "bufferModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_model: Option<String>,
    #[serde(rename = "ccDescriptor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_descriptor: Option<String>,
    #[serde(rename = "dvbNitSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_nit_settings: Option<DvbNitSettings>,
    #[serde(rename = "dvbSdtSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sdt_settings: Option<DvbSdtSettings>,
    #[serde(rename = "dvbSubPids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_pids: Option<String>,
    #[serde(rename = "dvbTdtSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_tdt_settings: Option<DvbTdtSettings>,
    #[serde(rename = "dvbTeletextPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_teletext_pid: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebif: Option<String>,
    #[serde(rename = "ebpAudioInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_audio_interval: Option<String>,
    #[serde(rename = "ebpLookaheadMs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_lookahead_ms: Option<i32>,
    #[serde(rename = "ebpPlacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_placement: Option<String>,
    #[serde(rename = "ecmPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecm_pid: Option<String>,
    #[serde(rename = "esRateInPes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es_rate_in_pes: Option<String>,
    #[serde(rename = "etvPlatformPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etv_platform_pid: Option<String>,
    #[serde(rename = "etvSignalPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etv_signal_pid: Option<String>,
    #[serde(rename = "fragmentTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv: Option<String>,
    #[serde(rename = "klvDataPids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv_data_pids: Option<String>,
    #[serde(rename = "nielsenId3Behavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id3_behavior: Option<String>,
    #[serde(rename = "nullPacketBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_packet_bitrate: Option<f64>,
    #[serde(rename = "patInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pat_interval: Option<i32>,
    #[serde(rename = "pcrControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_control: Option<String>,
    #[serde(rename = "pcrPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_period: Option<i32>,
    #[serde(rename = "pcrPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<String>,
    #[serde(rename = "pmtInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_interval: Option<i32>,
    #[serde(rename = "pmtPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_pid: Option<String>,
    #[serde(rename = "programNum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_num: Option<i32>,
    #[serde(rename = "rateMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_mode: Option<String>,
    #[serde(rename = "scte27Pids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte27_pids: Option<String>,
    #[serde(rename = "scte35Control")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_control: Option<String>,
    #[serde(rename = "scte35Pid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_pid: Option<String>,
    #[serde(rename = "scte35PrerollPullupMilliseconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_preroll_pullup_milliseconds: Option<f64>,
    #[serde(rename = "segmentationMarkers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_markers: Option<String>,
    #[serde(rename = "segmentationStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_style: Option<String>,
    #[serde(rename = "segmentationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_time: Option<f64>,
    #[serde(rename = "timedMetadataBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_behavior: Option<String>,
    #[serde(rename = "timedMetadataPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_pid: Option<String>,
    #[serde(rename = "transportStreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_id: Option<i32>,
    #[serde(rename = "videoPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DvbNitSettings {
    #[serde(rename = "networkId")]
    #[serde(default)]
    pub network_id: i32,
    #[serde(rename = "networkName")]
    #[serde(default)]
    pub network_name: String,
    #[serde(rename = "repInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_interval: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DvbSdtSettings {
    #[serde(rename = "outputSdt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_sdt: Option<String>,
    #[serde(rename = "repInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_interval: Option<i32>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "serviceProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_provider_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DvbTdtSettings {
    #[serde(rename = "repInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_interval: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RawSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CmafIngestOutputSettings {
    #[serde(rename = "nameModifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FrameCaptureOutputSettings {
    #[serde(rename = "nameModifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsOutputSettings {
    #[serde(rename = "h265PackagingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h265_packaging_type: Option<String>,
    #[serde(rename = "hlsSettings")]
    #[serde(default)]
    pub hls_settings: HlsSettings,
    #[serde(rename = "nameModifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<String>,
    #[serde(rename = "segmentModifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_modifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsSettings {
    #[serde(rename = "audioOnlyHlsSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only_hls_settings: Option<AudioOnlyHlsSettings>,
    #[serde(rename = "fmp4HlsSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fmp4_hls_settings: Option<Fmp4HlsSettings>,
    #[serde(rename = "frameCaptureHlsSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_hls_settings: Option<FrameCaptureHlsSettings>,
    #[serde(rename = "standardHlsSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_hls_settings: Option<StandardHlsSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioOnlyHlsSettings {
    #[serde(rename = "audioGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_group_id: Option<String>,
    #[serde(rename = "audioOnlyImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only_image: Option<InputLocation>,
    #[serde(rename = "audioTrackType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_track_type: Option<String>,
    #[serde(rename = "segmentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Fmp4HlsSettings {
    #[serde(rename = "audioRenditionSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_rendition_sets: Option<String>,
    #[serde(rename = "nielsenId3Behavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id3_behavior: Option<String>,
    #[serde(rename = "timedMetadataBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_behavior: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FrameCaptureHlsSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandardHlsSettings {
    #[serde(rename = "audioRenditionSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_rendition_sets: Option<String>,
    #[serde(rename = "m3u8Settings")]
    #[serde(default)]
    pub m3u8_settings: M3u8Settings,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct M3u8Settings {
    #[serde(rename = "audioFramesPerPes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_frames_per_pes: Option<i32>,
    #[serde(rename = "audioPids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pids: Option<String>,
    #[serde(rename = "ecmPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecm_pid: Option<String>,
    #[serde(rename = "klvBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv_behavior: Option<String>,
    #[serde(rename = "klvDataPids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv_data_pids: Option<String>,
    #[serde(rename = "nielsenId3Behavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id3_behavior: Option<String>,
    #[serde(rename = "patInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pat_interval: Option<i32>,
    #[serde(rename = "pcrControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_control: Option<String>,
    #[serde(rename = "pcrPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_period: Option<i32>,
    #[serde(rename = "pcrPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<String>,
    #[serde(rename = "pmtInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_interval: Option<i32>,
    #[serde(rename = "pmtPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_pid: Option<String>,
    #[serde(rename = "programNum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_num: Option<i32>,
    #[serde(rename = "scte35Behavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_behavior: Option<String>,
    #[serde(rename = "scte35Pid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_pid: Option<String>,
    #[serde(rename = "timedMetadataBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_behavior: Option<String>,
    #[serde(rename = "timedMetadataPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_pid: Option<String>,
    #[serde(rename = "transportStreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_id: Option<i32>,
    #[serde(rename = "videoPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaConnectRouterOutputSettings {
    #[serde(rename = "connectedRouterInputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_router_inputs: Option<MediaConnectRouterOutputConnectionMap>,
    #[serde(rename = "containerSettings")]
    #[serde(default)]
    pub container_settings: MediaConnectRouterContainerSettings,
    #[serde(default)]
    pub destination: OutputLocationRef,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaConnectRouterOutputConnectionMap {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline0: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline1: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaConnectRouterContainerSettings {
    #[serde(rename = "m2tsSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m2ts_settings: Option<M2tsSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaPackageOutputSettings {
    #[serde(rename = "mediaPackageV2DestinationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_package_v2_destination_settings: Option<MediaPackageV2DestinationSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaPackageV2DestinationSettings {
    #[serde(rename = "audioGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_group_id: Option<String>,
    #[serde(rename = "audioRenditionSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_rendition_sets: Option<String>,
    #[serde(rename = "hlsAutoSelect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_auto_select: Option<String>,
    #[serde(rename = "hlsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_default: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MsSmoothOutputSettings {
    #[serde(rename = "h265PackagingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h265_packaging_type: Option<String>,
    #[serde(rename = "nameModifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexOutputSettings {
    #[serde(rename = "containerSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<MultiplexContainerSettings>,
    #[serde(default)]
    pub destination: OutputLocationRef,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexContainerSettings {
    #[serde(rename = "multiplexM2tsSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_m2ts_settings: Option<MultiplexM2tsSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexM2tsSettings {
    #[serde(rename = "absentInputAudioBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absent_input_audio_behavior: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib: Option<String>,
    #[serde(rename = "audioBufferModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_buffer_model: Option<String>,
    #[serde(rename = "audioFramesPerPes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_frames_per_pes: Option<i32>,
    #[serde(rename = "audioStreamType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_stream_type: Option<String>,
    #[serde(rename = "ccDescriptor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_descriptor: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebif: Option<String>,
    #[serde(rename = "esRateInPes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es_rate_in_pes: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv: Option<String>,
    #[serde(rename = "nielsenId3Behavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id3_behavior: Option<String>,
    #[serde(rename = "pcrControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_control: Option<String>,
    #[serde(rename = "pcrPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_period: Option<i32>,
    #[serde(rename = "scte35Control")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_control: Option<String>,
    #[serde(rename = "scte35PrerollPullupMilliseconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_preroll_pullup_milliseconds: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RtmpOutputSettings {
    #[serde(rename = "certificateMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_mode: Option<String>,
    #[serde(rename = "connectionRetryInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i32>,
    #[serde(default)]
    pub destination: OutputLocationRef,
    #[serde(rename = "numRetries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtOutputSettings {
    #[serde(rename = "bufferMsec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_msec: Option<i32>,
    #[serde(rename = "containerSettings")]
    #[serde(default)]
    pub container_settings: UdpContainerSettings,
    #[serde(default)]
    pub destination: OutputLocationRef,
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UdpContainerSettings {
    #[serde(rename = "m2tsSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m2ts_settings: Option<M2tsSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UdpOutputSettings {
    #[serde(rename = "bufferMsec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_msec: Option<i32>,
    #[serde(rename = "containerSettings")]
    #[serde(default)]
    pub container_settings: UdpContainerSettings,
    #[serde(default)]
    pub destination: OutputLocationRef,
    #[serde(rename = "fecOutputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fec_output_settings: Option<FecOutputSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FecOutputSettings {
    #[serde(rename = "columnDepth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_depth: Option<i32>,
    #[serde(rename = "includeFec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fec: Option<String>,
    #[serde(rename = "rowLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_length: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThumbnailConfiguration {
    #[serde(default)]
    pub state: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimecodeConfig {
    #[serde(default)]
    pub source: String,
    #[serde(rename = "syncThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_threshold: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VideoDescription {
    #[serde(rename = "codecSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_settings: Option<VideoCodecSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "respondToAfd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respond_to_afd: Option<String>,
    #[serde(rename = "scalingBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_behavior: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VideoCodecSettings {
    #[serde(rename = "av1Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub av1_settings: Option<Av1Settings>,
    #[serde(rename = "frameCaptureSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_settings: Option<FrameCaptureSettings>,
    #[serde(rename = "h264Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h264_settings: Option<H264Settings>,
    #[serde(rename = "h265Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h265_settings: Option<H265Settings>,
    #[serde(rename = "mpeg2Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg2_settings: Option<Mpeg2Settings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Av1Settings {
    #[serde(rename = "afdSignaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afd_signaling: Option<String>,
    #[serde(rename = "bitDepth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    #[serde(rename = "bufSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buf_size: Option<i32>,
    #[serde(rename = "colorSpaceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_settings: Option<Av1ColorSpaceSettings>,
    #[serde(rename = "fixedAfd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_afd: Option<String>,
    #[serde(rename = "framerateDenominator")]
    #[serde(default)]
    pub framerate_denominator: i32,
    #[serde(rename = "framerateNumerator")]
    #[serde(default)]
    pub framerate_numerator: i32,
    #[serde(rename = "gopSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    #[serde(rename = "gopSizeUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "lookAheadRateControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_ahead_rate_control: Option<String>,
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
    #[serde(rename = "minBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_bitrate: Option<i32>,
    #[serde(rename = "minIInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_i_interval: Option<i32>,
    #[serde(rename = "parDenominator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i32>,
    #[serde(rename = "parNumerator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i32>,
    #[serde(rename = "qvbrQualityLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level: Option<i32>,
    #[serde(rename = "rateControlMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    #[serde(rename = "sceneChangeDetect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<String>,
    #[serde(rename = "spatialAq")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_aq: Option<String>,
    #[serde(rename = "temporalAq")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_aq: Option<String>,
    #[serde(rename = "timecodeBurninSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_burnin_settings: Option<TimecodeBurninSettings>,
    #[serde(rename = "timecodeInsertion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_insertion: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Av1ColorSpaceSettings {
    #[serde(rename = "colorSpacePassthroughSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_passthrough_settings: Option<ColorSpacePassthroughSettings>,
    #[serde(rename = "hdr10Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdr10_settings: Option<Hdr10Settings>,
    #[serde(rename = "hlg2020Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hlg2020_settings: Option<Hlg2020Settings>,
    #[serde(rename = "rec601Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec601_settings: Option<Rec601Settings>,
    #[serde(rename = "rec709Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec709_settings: Option<Rec709Settings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColorSpacePassthroughSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Hdr10Settings {
    #[serde(rename = "maxCll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cll: Option<i32>,
    #[serde(rename = "maxFall")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fall: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Hlg2020Settings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rec601Settings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rec709Settings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimecodeBurninSettings {
    #[serde(rename = "fontSize")]
    #[serde(default)]
    pub font_size: String,
    #[serde(default)]
    pub position: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FrameCaptureSettings {
    #[serde(rename = "captureInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_interval: Option<i32>,
    #[serde(rename = "captureIntervalUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_interval_units: Option<String>,
    #[serde(rename = "timecodeBurninSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_burnin_settings: Option<TimecodeBurninSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct H264Settings {
    #[serde(rename = "adaptiveQuantization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    #[serde(rename = "afdSignaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afd_signaling: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    #[serde(rename = "bufFillPct")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buf_fill_pct: Option<i32>,
    #[serde(rename = "bufSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buf_size: Option<i32>,
    #[serde(rename = "colorMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_metadata: Option<String>,
    #[serde(rename = "colorSpaceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_settings: Option<H264ColorSpaceSettings>,
    #[serde(rename = "entropyEncoding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entropy_encoding: Option<String>,
    #[serde(rename = "filterSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_settings: Option<H264FilterSettings>,
    #[serde(rename = "fixedAfd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_afd: Option<String>,
    #[serde(rename = "flickerAq")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flicker_aq: Option<String>,
    #[serde(rename = "forceFieldPictures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_field_pictures: Option<String>,
    #[serde(rename = "framerateControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    #[serde(rename = "framerateDenominator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i32>,
    #[serde(rename = "framerateNumerator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i32>,
    #[serde(rename = "gopBReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_b_reference: Option<String>,
    #[serde(rename = "gopClosedCadence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i32>,
    #[serde(rename = "gopNumBFrames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_num_b_frames: Option<i32>,
    #[serde(rename = "gopSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    #[serde(rename = "gopSizeUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "lookAheadRateControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_ahead_rate_control: Option<String>,
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
    #[serde(rename = "minBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_bitrate: Option<i32>,
    #[serde(rename = "minIInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_i_interval: Option<i32>,
    #[serde(rename = "minQp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_qp: Option<i32>,
    #[serde(rename = "numRefFrames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_ref_frames: Option<i32>,
    #[serde(rename = "parControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_control: Option<String>,
    #[serde(rename = "parDenominator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i32>,
    #[serde(rename = "parNumerator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(rename = "qualityLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_level: Option<String>,
    #[serde(rename = "qvbrQualityLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level: Option<i32>,
    #[serde(rename = "rateControlMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    #[serde(rename = "sceneChangeDetect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub softness: Option<i32>,
    #[serde(rename = "spatialAq")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_aq: Option<String>,
    #[serde(rename = "subgopLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgop_length: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax: Option<String>,
    #[serde(rename = "temporalAq")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_aq: Option<String>,
    #[serde(rename = "timecodeBurninSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_burnin_settings: Option<TimecodeBurninSettings>,
    #[serde(rename = "timecodeInsertion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_insertion: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct H264ColorSpaceSettings {
    #[serde(rename = "colorSpacePassthroughSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_passthrough_settings: Option<ColorSpacePassthroughSettings>,
    #[serde(rename = "rec601Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec601_settings: Option<Rec601Settings>,
    #[serde(rename = "rec709Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec709_settings: Option<Rec709Settings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct H264FilterSettings {
    #[serde(rename = "bandwidthReductionFilterSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_reduction_filter_settings: Option<BandwidthReductionFilterSettings>,
    #[serde(rename = "temporalFilterSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_filter_settings: Option<TemporalFilterSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BandwidthReductionFilterSettings {
    #[serde(rename = "postFilterSharpening")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_filter_sharpening: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemporalFilterSettings {
    #[serde(rename = "postFilterSharpening")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_filter_sharpening: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct H265Settings {
    #[serde(rename = "adaptiveQuantization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    #[serde(rename = "afdSignaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afd_signaling: Option<String>,
    #[serde(rename = "alternativeTransferFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_transfer_function: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    #[serde(rename = "bufSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buf_size: Option<i32>,
    #[serde(rename = "colorMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_metadata: Option<String>,
    #[serde(rename = "colorSpaceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_settings: Option<H265ColorSpaceSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deblocking: Option<String>,
    #[serde(rename = "filterSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_settings: Option<H265FilterSettings>,
    #[serde(rename = "fixedAfd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_afd: Option<String>,
    #[serde(rename = "flickerAq")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flicker_aq: Option<String>,
    #[serde(rename = "framerateDenominator")]
    #[serde(default)]
    pub framerate_denominator: i32,
    #[serde(rename = "framerateNumerator")]
    #[serde(default)]
    pub framerate_numerator: i32,
    #[serde(rename = "gopBReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_b_reference: Option<String>,
    #[serde(rename = "gopClosedCadence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i32>,
    #[serde(rename = "gopNumBFrames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_num_b_frames: Option<i32>,
    #[serde(rename = "gopSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    #[serde(rename = "gopSizeUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "lookAheadRateControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_ahead_rate_control: Option<String>,
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
    #[serde(rename = "minBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_bitrate: Option<i32>,
    #[serde(rename = "minIInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_i_interval: Option<i32>,
    #[serde(rename = "minQp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_qp: Option<i32>,
    #[serde(rename = "mvOverPictureBoundaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mv_over_picture_boundaries: Option<String>,
    #[serde(rename = "mvTemporalPredictor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mv_temporal_predictor: Option<String>,
    #[serde(rename = "parDenominator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i32>,
    #[serde(rename = "parNumerator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(rename = "qvbrQualityLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level: Option<i32>,
    #[serde(rename = "rateControlMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    #[serde(rename = "sceneChangeDetect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i32>,
    #[serde(rename = "subgopLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgop_length: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "tileHeight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_height: Option<i32>,
    #[serde(rename = "tilePadding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_padding: Option<String>,
    #[serde(rename = "tileWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_width: Option<i32>,
    #[serde(rename = "timecodeBurninSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_burnin_settings: Option<TimecodeBurninSettings>,
    #[serde(rename = "timecodeInsertion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_insertion: Option<String>,
    #[serde(rename = "treeblockSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treeblock_size: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct H265ColorSpaceSettings {
    #[serde(rename = "colorSpacePassthroughSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_passthrough_settings: Option<ColorSpacePassthroughSettings>,
    #[serde(rename = "dolbyVision81Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dolby_vision81_settings: Option<DolbyVision81Settings>,
    #[serde(rename = "hdr10Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdr10_settings: Option<Hdr10Settings>,
    #[serde(rename = "hlg2020Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hlg2020_settings: Option<Hlg2020Settings>,
    #[serde(rename = "rec601Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec601_settings: Option<Rec601Settings>,
    #[serde(rename = "rec709Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec709_settings: Option<Rec709Settings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DolbyVision81Settings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct H265FilterSettings {
    #[serde(rename = "bandwidthReductionFilterSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_reduction_filter_settings: Option<BandwidthReductionFilterSettings>,
    #[serde(rename = "temporalFilterSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_filter_settings: Option<TemporalFilterSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Mpeg2Settings {
    #[serde(rename = "adaptiveQuantization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    #[serde(rename = "afdSignaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afd_signaling: Option<String>,
    #[serde(rename = "colorMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_metadata: Option<String>,
    #[serde(rename = "colorSpace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,
    #[serde(rename = "displayAspectRatio")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_aspect_ratio: Option<String>,
    #[serde(rename = "filterSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_settings: Option<Mpeg2FilterSettings>,
    #[serde(rename = "fixedAfd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_afd: Option<String>,
    #[serde(rename = "framerateDenominator")]
    #[serde(default)]
    pub framerate_denominator: i32,
    #[serde(rename = "framerateNumerator")]
    #[serde(default)]
    pub framerate_numerator: i32,
    #[serde(rename = "gopClosedCadence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i32>,
    #[serde(rename = "gopNumBFrames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_num_b_frames: Option<i32>,
    #[serde(rename = "gopSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    #[serde(rename = "gopSizeUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<String>,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    #[serde(rename = "subgopLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgop_length: Option<String>,
    #[serde(rename = "timecodeBurninSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_burnin_settings: Option<TimecodeBurninSettings>,
    #[serde(rename = "timecodeInsertion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_insertion: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Mpeg2FilterSettings {
    #[serde(rename = "temporalFilterSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_filter_settings: Option<TemporalFilterSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InferenceSettings {
    #[serde(rename = "feedArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputAttachment {
    #[serde(rename = "automaticInputFailoverSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_input_failover_settings: Option<AutomaticInputFailoverSettings>,
    #[serde(rename = "inputAttachmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachment_name: Option<String>,
    #[serde(rename = "inputId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_id: Option<String>,
    #[serde(rename = "inputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_settings: Option<InputSettings>,
    #[serde(rename = "logicalInterfaceNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_interface_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomaticInputFailoverSettings {
    #[serde(rename = "errorClearTimeMsec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_clear_time_msec: Option<i32>,
    #[serde(rename = "failoverConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_conditions: Option<Vec<FailoverCondition>>,
    #[serde(rename = "inputPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_preference: Option<String>,
    #[serde(rename = "secondaryInputId")]
    #[serde(default)]
    pub secondary_input_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailoverCondition {
    #[serde(rename = "failoverConditionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_condition_settings: Option<FailoverConditionSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailoverConditionSettings {
    #[serde(rename = "audioSilenceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_silence_settings: Option<AudioSilenceFailoverSettings>,
    #[serde(rename = "inputLossSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_settings: Option<InputLossFailoverSettings>,
    #[serde(rename = "videoBlackSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_black_settings: Option<VideoBlackFailoverSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioSilenceFailoverSettings {
    #[serde(rename = "audioSelectorName")]
    #[serde(default)]
    pub audio_selector_name: String,
    #[serde(rename = "audioSilenceThresholdMsec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_silence_threshold_msec: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputLossFailoverSettings {
    #[serde(rename = "inputLossThresholdMsec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_threshold_msec: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VideoBlackFailoverSettings {
    #[serde(rename = "blackDetectThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_detect_threshold: Option<f64>,
    #[serde(rename = "videoBlackThresholdMsec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_black_threshold_msec: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputSettings {
    #[serde(rename = "audioSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selectors: Option<Vec<AudioSelector>>,
    #[serde(rename = "captionSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_selectors: Option<Vec<CaptionSelector>>,
    #[serde(rename = "deblockFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deblock_filter: Option<String>,
    #[serde(rename = "denoiseFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denoise_filter: Option<String>,
    #[serde(rename = "filterStrength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_strength: Option<i32>,
    #[serde(rename = "inputFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_filter: Option<String>,
    #[serde(rename = "networkInputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_input_settings: Option<NetworkInputSettings>,
    #[serde(rename = "scte35Pid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_pid: Option<i32>,
    #[serde(rename = "smpte2038DataPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smpte2038_data_preference: Option<String>,
    #[serde(rename = "sourceEndBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_end_behavior: Option<String>,
    #[serde(rename = "videoSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_selector: Option<VideoSelector>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioSelector {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "selectorSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector_settings: Option<AudioSelectorSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioSelectorSettings {
    #[serde(rename = "audioHlsRenditionSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_hls_rendition_selection: Option<AudioHlsRenditionSelection>,
    #[serde(rename = "audioLanguageSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_language_selection: Option<AudioLanguageSelection>,
    #[serde(rename = "audioPidSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pid_selection: Option<AudioPidSelection>,
    #[serde(rename = "audioTrackSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_track_selection: Option<AudioTrackSelection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioHlsRenditionSelection {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioLanguageSelection {
    #[serde(rename = "languageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "languageSelectionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_selection_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioPidSelection {
    #[serde(default)]
    pub pid: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioTrackSelection {
    #[serde(rename = "dolbyEDecode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dolby_e_decode: Option<AudioDolbyEDecode>,
    #[serde(default)]
    pub tracks: Vec<AudioTrack>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioDolbyEDecode {
    #[serde(rename = "programSelection")]
    #[serde(default)]
    pub program_selection: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioTrack {
    #[serde(default)]
    pub track: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CaptionSelector {
    #[serde(rename = "languageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "selectorSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector_settings: Option<CaptionSelectorSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CaptionSelectorSettings {
    #[serde(rename = "ancillarySourceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancillary_source_settings: Option<AncillarySourceSettings>,
    #[serde(rename = "aribSourceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib_source_settings: Option<AribSourceSettings>,
    #[serde(rename = "dvbSubSourceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_source_settings: Option<DvbSubSourceSettings>,
    #[serde(rename = "embeddedSourceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_source_settings: Option<EmbeddedSourceSettings>,
    #[serde(rename = "scte20SourceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte20_source_settings: Option<Scte20SourceSettings>,
    #[serde(rename = "scte27SourceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte27_source_settings: Option<Scte27SourceSettings>,
    #[serde(rename = "teletextSourceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_source_settings: Option<TeletextSourceSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AncillarySourceSettings {
    #[serde(rename = "sourceAncillaryChannelNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ancillary_channel_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AribSourceSettings {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DvbSubSourceSettings {
    #[serde(rename = "ocrLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_language: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmbeddedSourceSettings {
    #[serde(rename = "convert608To708")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert608_to708: Option<String>,
    #[serde(rename = "scte20Detection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte20_detection: Option<String>,
    #[serde(rename = "source608ChannelNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source608_channel_number: Option<i32>,
    #[serde(rename = "source608TrackNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source608_track_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte20SourceSettings {
    #[serde(rename = "convert608To708")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert608_to708: Option<String>,
    #[serde(rename = "source608ChannelNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source608_channel_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scte27SourceSettings {
    #[serde(rename = "ocrLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_language: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TeletextSourceSettings {
    #[serde(rename = "outputRectangle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_rectangle: Option<CaptionRectangle>,
    #[serde(rename = "pageNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CaptionRectangle {
    #[serde(default)]
    pub height: f64,
    #[serde(rename = "leftOffset")]
    #[serde(default)]
    pub left_offset: f64,
    #[serde(rename = "topOffset")]
    #[serde(default)]
    pub top_offset: f64,
    #[serde(default)]
    pub width: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkInputSettings {
    #[serde(rename = "hlsInputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_input_settings: Option<HlsInputSettings>,
    #[serde(rename = "multicastInputSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicast_input_settings: Option<MulticastInputSettings>,
    #[serde(rename = "serverValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_validation: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HlsInputSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    #[serde(rename = "bufferSegments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_segments: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(rename = "retryInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
    #[serde(rename = "scte35Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MulticastInputSettings {
    #[serde(rename = "sourceIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VideoSelector {
    #[serde(rename = "colorSpace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,
    #[serde(rename = "colorSpaceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_settings: Option<VideoSelectorColorSpaceSettings>,
    #[serde(rename = "colorSpaceUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_usage: Option<String>,
    #[serde(rename = "selectorSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector_settings: Option<VideoSelectorSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VideoSelectorColorSpaceSettings {
    #[serde(rename = "hdr10Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdr10_settings: Option<Hdr10Settings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VideoSelectorSettings {
    #[serde(rename = "videoSelectorPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_selector_pid: Option<VideoSelectorPid>,
    #[serde(rename = "videoSelectorProgramId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_selector_program_id: Option<VideoSelectorProgramId>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VideoSelectorPid {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VideoSelectorProgramId {
    #[serde(rename = "programId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputSpecification {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(rename = "maximumBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LinkedChannelSettings {
    #[serde(rename = "followerChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_channel_settings: Option<FollowerChannelSettings>,
    #[serde(rename = "primaryChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_channel_settings: Option<PrimaryChannelSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FollowerChannelSettings {
    #[serde(rename = "linkedChannelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_channel_type: Option<String>,
    #[serde(rename = "primaryChannelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_channel_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrimaryChannelSettings {
    #[serde(rename = "linkedChannelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_channel_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceCreateSettings {
    #[serde(rename = "maintenanceDay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_day: Option<String>,
    #[serde(rename = "maintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcOutputSettings {
    #[serde(rename = "publicAddressAllocationIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_address_allocation_ids: Option<Vec<String>>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Channel {
    #[serde(rename = "anywhereSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anywhere_settings: Option<DescribeAnywhereSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "cdiInputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    #[serde(rename = "channelClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    #[serde(rename = "channelEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_engine_version: Option<ChannelEngineVersionResponse>,
    #[serde(rename = "channelSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    #[serde(rename = "egressEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<ChannelEgressEndpoint>>,
    #[serde(rename = "encoderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inferenceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_settings: Option<DescribeInferenceSettings>,
    #[serde(rename = "inputAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    #[serde(rename = "inputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    #[serde(rename = "linkedChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_channel_settings: Option<DescribeLinkedChannelSettings>,
    #[serde(rename = "logLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelineDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<PipelineDetail>>,
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettingsDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAnywhereSettings {
    #[serde(rename = "channelPlacementGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_placement_group_id: Option<String>,
    #[serde(rename = "clusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelEngineVersionResponse {
    #[serde(rename = "expirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelEgressEndpoint {
    #[serde(rename = "sourceIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInferenceSettings {
    #[serde(rename = "feedArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLinkedChannelSettings {
    #[serde(rename = "followerChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_channel_settings: Option<DescribeFollowerChannelSettings>,
    #[serde(rename = "primaryChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_channel_settings: Option<DescribePrimaryChannelSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFollowerChannelSettings {
    #[serde(rename = "linkedChannelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_channel_type: Option<String>,
    #[serde(rename = "primaryChannelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_channel_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePrimaryChannelSettings {
    #[serde(rename = "followingChannelArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following_channel_arns: Option<Vec<String>>,
    #[serde(rename = "linkedChannelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_channel_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceStatus {
    #[serde(rename = "maintenanceDay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_day: Option<String>,
    #[serde(rename = "maintenanceDeadline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_deadline: Option<String>,
    #[serde(rename = "maintenanceScheduledDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_scheduled_date: Option<String>,
    #[serde(rename = "maintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineDetail {
    #[serde(rename = "activeInputAttachmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_input_attachment_name: Option<String>,
    #[serde(rename = "activeInputSwitchActionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_input_switch_action_name: Option<String>,
    #[serde(rename = "activeMotionGraphicsActionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_motion_graphics_action_name: Option<String>,
    #[serde(rename = "activeMotionGraphicsUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_motion_graphics_uri: Option<String>,
    #[serde(rename = "channelEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_engine_version: Option<ChannelEngineVersionResponse>,
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcOutputSettingsDescription {
    #[serde(rename = "availabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(rename = "networkInterfaceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_ids: Option<Vec<String>>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCloudWatchAlarmTemplateGroupRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCloudWatchAlarmTemplateGroupResponse {
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCloudWatchAlarmTemplateRequest {
    #[serde(rename = "comparisonOperator")]
    #[serde(default)]
    pub comparison_operator: String,
    #[serde(rename = "datapointsToAlarm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints_to_alarm: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "evaluationPeriods")]
    #[serde(default)]
    pub evaluation_periods: i32,
    #[serde(rename = "groupIdentifier")]
    #[serde(default)]
    pub group_identifier: String,
    #[serde(rename = "metricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub period: i32,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    pub statistic: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetResourceType")]
    #[serde(default)]
    pub target_resource_type: String,
    #[serde(default)]
    pub threshold: f64,
    #[serde(rename = "treatMissingData")]
    #[serde(default)]
    pub treat_missing_data: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCloudWatchAlarmTemplateResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "comparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "datapointsToAlarm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints_to_alarm: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "evaluationPeriods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_periods: Option<i32>,
    #[serde(rename = "groupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "treatMissingData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_missing_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterRequest {
    #[serde(rename = "clusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(rename = "instanceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<ClusterNetworkSettingsCreateRequest>,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterNetworkSettingsCreateRequest {
    #[serde(rename = "defaultRoute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route: Option<String>,
    #[serde(rename = "interfaceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_mappings: Option<Vec<InterfaceMappingCreateRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InterfaceMappingCreateRequest {
    #[serde(rename = "logicalInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_interface_name: Option<String>,
    #[serde(rename = "networkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    #[serde(rename = "clusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "instanceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<ClusterNetworkSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterNetworkSettings {
    #[serde(rename = "defaultRoute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route: Option<String>,
    #[serde(rename = "interfaceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_mappings: Option<Vec<InterfaceMapping>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InterfaceMapping {
    #[serde(rename = "logicalInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_interface_name: Option<String>,
    #[serde(rename = "networkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventBridgeRuleTemplateGroupRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventBridgeRuleTemplateGroupResponse {
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventBridgeRuleTemplateRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "eventTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_targets: Option<Vec<EventBridgeRuleTemplateTarget>>,
    #[serde(rename = "eventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(rename = "groupIdentifier")]
    #[serde(default)]
    pub group_identifier: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventBridgeRuleTemplateTarget {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventBridgeRuleTemplateResponse {
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
    #[serde(rename = "eventTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_targets: Option<Vec<EventBridgeRuleTemplateTarget>>,
    #[serde(rename = "eventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "groupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInputRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<InputDestinationRequest>>,
    #[serde(rename = "inputDevices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_devices: Option<Vec<InputDeviceSettings>>,
    #[serde(rename = "inputNetworkLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_network_location: Option<String>,
    #[serde(rename = "inputSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_security_groups: Option<Vec<String>>,
    #[serde(rename = "mediaConnectFlows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_flows: Option<Vec<MediaConnectFlowRequest>>,
    #[serde(rename = "multicastSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicast_settings: Option<MulticastSettingsCreateRequest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "routerSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_settings: Option<RouterSettings>,
    #[serde(rename = "sdiSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_sources: Option<Vec<String>>,
    #[serde(rename = "smpte2110ReceiverGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smpte2110_receiver_group_settings: Option<Smpte2110ReceiverGroupSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<InputSourceRequest>>,
    #[serde(rename = "srtSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_settings: Option<SrtSettingsRequest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<InputVpcRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDestinationRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "networkRoutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_routes: Option<Vec<InputRequestDestinationRoute>>,
    #[serde(rename = "staticIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ip_address: Option<String>,
    #[serde(rename = "streamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputRequestDestinationRoute {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDeviceSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaConnectFlowRequest {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MulticastSettingsCreateRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<MulticastSourceCreateRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MulticastSourceCreateRequest {
    #[serde(rename = "sourceIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<RouterDestinationSettings>>,
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterDestinationSettings {
    #[serde(rename = "availabilityZoneName")]
    #[serde(default)]
    pub availability_zone_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Smpte2110ReceiverGroupSettings {
    #[serde(rename = "smpte2110ReceiverGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smpte2110_receiver_groups: Option<Vec<Smpte2110ReceiverGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Smpte2110ReceiverGroup {
    #[serde(rename = "sdpSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdp_settings: Option<Smpte2110ReceiverGroupSdpSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Smpte2110ReceiverGroupSdpSettings {
    #[serde(rename = "ancillarySdps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancillary_sdps: Option<Vec<InputSdpLocation>>,
    #[serde(rename = "audioSdps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_sdps: Option<Vec<InputSdpLocation>>,
    #[serde(rename = "videoSdp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_sdp: Option<InputSdpLocation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputSdpLocation {
    #[serde(rename = "mediaIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_index: Option<i32>,
    #[serde(rename = "sdpUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdp_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputSourceRequest {
    #[serde(rename = "passwordParam")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_param: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtSettingsRequest {
    #[serde(rename = "srtCallerSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_caller_sources: Option<Vec<SrtCallerSourceRequest>>,
    #[serde(rename = "srtListenerSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_listener_settings: Option<SrtListenerSettingsRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtCallerSourceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption: Option<SrtCallerDecryptionRequest>,
    #[serde(rename = "minimumLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_latency: Option<i32>,
    #[serde(rename = "srtListenerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_listener_address: Option<String>,
    #[serde(rename = "srtListenerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_listener_port: Option<String>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtCallerDecryptionRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(rename = "passphraseSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase_secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtListenerSettingsRequest {
    #[serde(default)]
    pub decryption: SrtListenerDecryptionRequest,
    #[serde(rename = "minimumLatency")]
    #[serde(default)]
    pub minimum_latency: i32,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtListenerDecryptionRequest {
    #[serde(default)]
    pub algorithm: String,
    #[serde(rename = "passphraseSecretArn")]
    #[serde(default)]
    pub passphrase_secret_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputVpcRequest {
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInputResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Input {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "attachedChannels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_channels: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<InputDestination>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inputClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_class: Option<String>,
    #[serde(rename = "inputDevices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_devices: Option<Vec<InputDeviceSettings>>,
    #[serde(rename = "inputNetworkLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_network_location: Option<String>,
    #[serde(rename = "inputPartnerIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_partner_ids: Option<Vec<String>>,
    #[serde(rename = "inputSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_source_type: Option<String>,
    #[serde(rename = "mediaConnectFlows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_flows: Option<Vec<MediaConnectFlow>>,
    #[serde(rename = "multicastSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicast_settings: Option<MulticastSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "routerSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_settings: Option<RouterInputSettings>,
    #[serde(rename = "sdiSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_sources: Option<Vec<String>>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "smpte2110ReceiverGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smpte2110_receiver_group_settings: Option<Smpte2110ReceiverGroupSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<InputSource>>,
    #[serde(rename = "srtSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_settings: Option<SrtSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDestination {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "networkRoutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_routes: Option<Vec<InputDestinationRoute>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<InputDestinationVpc>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDestinationRoute {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDestinationVpc {
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "networkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaConnectFlow {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MulticastSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<MulticastSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MulticastSource {
    #[serde(rename = "sourceIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterInputSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<RouterDestination>>,
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouterDestination {
    #[serde(rename = "availabilityZoneName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_name: Option<String>,
    #[serde(rename = "routerOutputArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_output_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputSource {
    #[serde(rename = "passwordParam")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_param: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtSettings {
    #[serde(rename = "srtCallerSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_caller_sources: Option<Vec<SrtCallerSource>>,
    #[serde(rename = "srtListenerSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_listener_settings: Option<SrtListenerSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtCallerSource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption: Option<SrtCallerDecryption>,
    #[serde(rename = "minimumLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_latency: Option<i32>,
    #[serde(rename = "srtListenerAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_listener_address: Option<String>,
    #[serde(rename = "srtListenerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_listener_port: Option<String>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtCallerDecryption {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(rename = "passphraseSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase_secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtListenerSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption: Option<SrtListenerDecryption>,
    #[serde(rename = "minimumLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_latency: Option<i32>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SrtListenerDecryption {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(rename = "passphraseSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase_secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInputSecurityGroupRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "whitelistRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_rules: Option<Vec<InputWhitelistRuleCidr>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputWhitelistRuleCidr {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInputSecurityGroupResponse {
    #[serde(rename = "securityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<InputSecurityGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputSecurityGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "whitelistRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_rules: Option<Vec<InputWhitelistRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputWhitelistRule {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMultiplexProgramRequest {
    #[serde(rename = "MultiplexId")]
    #[serde(default)]
    pub multiplex_id: String,
    #[serde(rename = "multiplexProgramSettings")]
    #[serde(default)]
    pub multiplex_program_settings: MultiplexProgramSettings,
    #[serde(rename = "programName")]
    #[serde(default)]
    pub program_name: String,
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexProgramSettings {
    #[serde(rename = "preferredChannelPipeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_channel_pipeline: Option<String>,
    #[serde(rename = "programNumber")]
    #[serde(default)]
    pub program_number: i32,
    #[serde(rename = "serviceDescriptor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_descriptor: Option<MultiplexProgramServiceDescriptor>,
    #[serde(rename = "videoSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_settings: Option<MultiplexVideoSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexProgramServiceDescriptor {
    #[serde(rename = "providerName")]
    #[serde(default)]
    pub provider_name: String,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    pub service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexVideoSettings {
    #[serde(rename = "constantBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_bitrate: Option<i32>,
    #[serde(rename = "statmuxSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statmux_settings: Option<MultiplexStatmuxVideoSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexStatmuxVideoSettings {
    #[serde(rename = "maximumBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<i32>,
    #[serde(rename = "minimumBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_bitrate: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMultiplexProgramResponse {
    #[serde(rename = "multiplexProgram")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_program: Option<MultiplexProgram>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexProgram {
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "multiplexProgramSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_program_settings: Option<MultiplexProgramSettings>,
    #[serde(rename = "packetIdentifiersMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_identifiers_map: Option<MultiplexProgramPacketIdentifiersMap>,
    #[serde(rename = "pipelineDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<MultiplexProgramPipelineDetail>>,
    #[serde(rename = "programName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexProgramPacketIdentifiersMap {
    #[serde(rename = "aribCaptionsPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib_captions_pid: Option<i32>,
    #[serde(rename = "audioPids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pids: Option<Vec<i32>>,
    #[serde(rename = "dvbSubPids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_pids: Option<Vec<i32>>,
    #[serde(rename = "dvbTeletextPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_teletext_pid: Option<i32>,
    #[serde(rename = "dvbTeletextPids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_teletext_pids: Option<Vec<i32>>,
    #[serde(rename = "ecmPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecm_pid: Option<i32>,
    #[serde(rename = "etvPlatformPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etv_platform_pid: Option<i32>,
    #[serde(rename = "etvSignalPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etv_signal_pid: Option<i32>,
    #[serde(rename = "klvDataPids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv_data_pids: Option<Vec<i32>>,
    #[serde(rename = "pcrPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<i32>,
    #[serde(rename = "pmtPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_pid: Option<i32>,
    #[serde(rename = "privateMetadataPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_metadata_pid: Option<i32>,
    #[serde(rename = "scte27Pids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte27_pids: Option<Vec<i32>>,
    #[serde(rename = "scte35Pid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte35_pid: Option<i32>,
    #[serde(rename = "smpte2038Pid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smpte2038_pid: Option<i32>,
    #[serde(rename = "timedMetadataPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_pid: Option<i32>,
    #[serde(rename = "videoPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pid: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexProgramPipelineDetail {
    #[serde(rename = "activeChannelPipeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_channel_pipeline: Option<String>,
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMultiplexRequest {
    #[serde(rename = "availabilityZones")]
    #[serde(default)]
    pub availability_zones: Vec<String>,
    #[serde(rename = "multiplexSettings")]
    #[serde(default)]
    pub multiplex_settings: MultiplexSettings,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexSettings {
    #[serde(rename = "maximumVideoBufferDelayMilliseconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_video_buffer_delay_milliseconds: Option<i32>,
    #[serde(rename = "transportStreamBitrate")]
    #[serde(default)]
    pub transport_stream_bitrate: i32,
    #[serde(rename = "transportStreamId")]
    #[serde(default)]
    pub transport_stream_id: i32,
    #[serde(rename = "transportStreamReservedBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_reserved_bitrate: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMultiplexResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex: Option<Multiplex>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Multiplex {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<MultiplexOutputDestination>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "multiplexSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i32>,
    #[serde(rename = "programCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexOutputDestination {
    #[serde(rename = "mediaConnectSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_settings: Option<MultiplexMediaConnectOutputDestinationSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexMediaConnectOutputDestinationSettings {
    #[serde(rename = "entitlementArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNetworkRequest {
    #[serde(rename = "ipPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_pools: Option<Vec<IpPoolCreateRequest>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<RouteCreateRequest>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpPoolCreateRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteCreateRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNetworkResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "associatedClusterIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_cluster_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ipPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_pools: Option<Vec<IpPool>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<Route>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpPool {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Route {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNodeRegistrationScriptRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nodeInterfaceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_interface_mappings: Option<Vec<NodeInterfaceMapping>>,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeInterfaceMapping {
    #[serde(rename = "logicalInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_interface_name: Option<String>,
    #[serde(rename = "networkInterfaceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_mode: Option<String>,
    #[serde(rename = "physicalInterfaceIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_interface_ip_addresses: Option<Vec<String>>,
    #[serde(rename = "physicalInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_interface_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNodeRegistrationScriptResponse {
    #[serde(rename = "nodeRegistrationScript")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_registration_script: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNodeRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nodeInterfaceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_interface_mappings: Option<Vec<NodeInterfaceMappingCreateRequest>>,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeInterfaceMappingCreateRequest {
    #[serde(rename = "logicalInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_interface_name: Option<String>,
    #[serde(rename = "networkInterfaceMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_mode: Option<String>,
    #[serde(rename = "physicalInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_interface_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNodeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelPlacementGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_placement_groups: Option<Vec<String>>,
    #[serde(rename = "clusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "connectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "instanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nodeInterfaceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_interface_mappings: Option<Vec<NodeInterfaceMapping>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "sdiSourceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_source_mappings: Option<Vec<SdiSourceMapping>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SdiSourceMapping {
    #[serde(rename = "cardNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number: Option<i32>,
    #[serde(rename = "channelNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_number: Option<i32>,
    #[serde(rename = "sdiSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePartnerInputRequest {
    #[serde(rename = "InputId")]
    #[serde(default)]
    pub input_id: String,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePartnerInputResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSdiSourceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSdiSourceResponse {
    #[serde(rename = "sdiSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_source: Option<SdiSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SdiSource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSignalMapRequest {
    #[serde(rename = "cloudWatchAlarmTemplateGroupIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_alarm_template_group_identifiers: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "discoveryEntryPointArn")]
    #[serde(default)]
    pub discovery_entry_point_arn: String,
    #[serde(rename = "eventBridgeRuleTemplateGroupIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_rule_template_group_identifiers: Option<Vec<String>>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSignalMapResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "cloudWatchAlarmTemplateGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_alarm_template_group_ids: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "discoveryEntryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_entry_point_arn: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "eventBridgeRuleTemplateGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_rule_template_group_ids: Option<Vec<String>>,
    #[serde(rename = "failedMediaResourceMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_media_resource_map: Option<std::collections::HashMap<String, MediaResource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastDiscoveredAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_discovered_at: Option<String>,
    #[serde(rename = "lastSuccessfulMonitorDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_monitor_deployment: Option<SuccessfulMonitorDeployment>,
    #[serde(rename = "mediaResourceMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_resource_map: Option<std::collections::HashMap<String, MediaResource>>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "monitorChangesPendingDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_changes_pending_deployment: Option<bool>,
    #[serde(rename = "monitorDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_deployment: Option<MonitorDeployment>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaResource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<MediaResourceNeighbor>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<MediaResourceNeighbor>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaResourceNeighbor {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuccessfulMonitorDeployment {
    #[serde(rename = "detailsUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_uri: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitorDeployment {
    #[serde(rename = "detailsUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_uri: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTagsRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelPlacementGroupRequest {
    #[serde(rename = "ChannelPlacementGroupId")]
    #[serde(default)]
    pub channel_placement_group_id: String,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelPlacementGroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "clusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelRequest {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelResponse {
    #[serde(rename = "anywhereSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anywhere_settings: Option<DescribeAnywhereSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "cdiInputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    #[serde(rename = "channelClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    #[serde(rename = "channelEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_engine_version: Option<ChannelEngineVersionResponse>,
    #[serde(rename = "channelSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    #[serde(rename = "egressEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<ChannelEgressEndpoint>>,
    #[serde(rename = "encoderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inferenceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_settings: Option<DescribeInferenceSettings>,
    #[serde(rename = "inputAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    #[serde(rename = "inputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    #[serde(rename = "linkedChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_channel_settings: Option<DescribeLinkedChannelSettings>,
    #[serde(rename = "logLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelineDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<PipelineDetail>>,
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettingsDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCloudWatchAlarmTemplateGroupRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCloudWatchAlarmTemplateRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    #[serde(rename = "clusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "instanceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<ClusterNetworkSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEventBridgeRuleTemplateGroupRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEventBridgeRuleTemplateRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInputRequest {
    #[serde(rename = "InputId")]
    #[serde(default)]
    pub input_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInputResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInputSecurityGroupRequest {
    #[serde(rename = "InputSecurityGroupId")]
    #[serde(default)]
    pub input_security_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInputSecurityGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMultiplexProgramRequest {
    #[serde(rename = "MultiplexId")]
    #[serde(default)]
    pub multiplex_id: String,
    #[serde(rename = "ProgramName")]
    #[serde(default)]
    pub program_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMultiplexProgramResponse {
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "multiplexProgramSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_program_settings: Option<MultiplexProgramSettings>,
    #[serde(rename = "packetIdentifiersMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_identifiers_map: Option<MultiplexProgramPacketIdentifiersMap>,
    #[serde(rename = "pipelineDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<MultiplexProgramPipelineDetail>>,
    #[serde(rename = "programName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMultiplexRequest {
    #[serde(rename = "MultiplexId")]
    #[serde(default)]
    pub multiplex_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMultiplexResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<MultiplexOutputDestination>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "multiplexSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i32>,
    #[serde(rename = "programCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNetworkRequest {
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNetworkResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "associatedClusterIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_cluster_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ipPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_pools: Option<Vec<IpPool>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<Route>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNodeRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    pub node_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNodeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelPlacementGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_placement_groups: Option<Vec<String>>,
    #[serde(rename = "clusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "connectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "instanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nodeInterfaceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_interface_mappings: Option<Vec<NodeInterfaceMapping>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "sdiSourceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_source_mappings: Option<Vec<SdiSourceMapping>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReservationRequest {
    #[serde(rename = "ReservationId")]
    #[serde(default)]
    pub reservation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReservationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "currencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "durationUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_units: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "fixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "offeringDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_description: Option<String>,
    #[serde(rename = "offeringId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    #[serde(rename = "offeringType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "renewalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_settings: Option<RenewalSettings>,
    #[serde(rename = "reservationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ReservationResourceSpecification>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "usagePrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RenewalSettings {
    #[serde(rename = "automaticRenewal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_renewal: Option<String>,
    #[serde(rename = "renewalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservationResourceSpecification {
    #[serde(rename = "channelClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(rename = "maximumBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<String>,
    #[serde(rename = "maximumFramerate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_framerate: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "specialFeature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_feature: Option<String>,
    #[serde(rename = "videoQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduleRequest {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSdiSourceRequest {
    #[serde(rename = "SdiSourceId")]
    #[serde(default)]
    pub sdi_source_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSdiSourceResponse {
    #[serde(rename = "sdiSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_source: Option<SdiSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSignalMapRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTagsRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountConfigurationResponse {
    #[serde(rename = "accountConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_configuration: Option<AccountConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountConfiguration {
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeChannelPlacementGroupRequest {
    #[serde(rename = "ChannelPlacementGroupId")]
    #[serde(default)]
    pub channel_placement_group_id: String,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeChannelPlacementGroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "clusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeChannelRequest {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeChannelResponse {
    #[serde(rename = "anywhereSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anywhere_settings: Option<DescribeAnywhereSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "cdiInputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    #[serde(rename = "channelClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    #[serde(rename = "channelEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_engine_version: Option<ChannelEngineVersionResponse>,
    #[serde(rename = "channelSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    #[serde(rename = "egressEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<ChannelEgressEndpoint>>,
    #[serde(rename = "encoderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inferenceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_settings: Option<DescribeInferenceSettings>,
    #[serde(rename = "inputAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    #[serde(rename = "inputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    #[serde(rename = "linkedChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_channel_settings: Option<DescribeLinkedChannelSettings>,
    #[serde(rename = "logLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelineDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<PipelineDetail>>,
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettingsDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    #[serde(rename = "clusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "instanceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<ClusterNetworkSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInputDeviceRequest {
    #[serde(rename = "InputDeviceId")]
    #[serde(default)]
    pub input_device_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInputDeviceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "connectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "deviceSettingsSyncState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_settings_sync_state: Option<String>,
    #[serde(rename = "deviceUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_update_status: Option<String>,
    #[serde(rename = "hdDeviceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd_device_settings: Option<InputDeviceHdSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "macAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "medialiveInputArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medialive_input_arns: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<InputDeviceNetworkSettings>,
    #[serde(rename = "outputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,
    #[serde(rename = "serialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "uhdDeviceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uhd_device_settings: Option<InputDeviceUhdSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDeviceHdSettings {
    #[serde(rename = "activeInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_input: Option<String>,
    #[serde(rename = "configuredInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_input: Option<String>,
    #[serde(rename = "deviceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "latencyMs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<i32>,
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDeviceNetworkSettings {
    #[serde(rename = "dnsAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_addresses: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(rename = "ipAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "ipScheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_scheme: Option<String>,
    #[serde(rename = "subnetMask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mask: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDeviceUhdSettings {
    #[serde(rename = "activeInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_input: Option<String>,
    #[serde(rename = "audioChannelPairs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_channel_pairs: Option<Vec<InputDeviceUhdAudioChannelPairConfig>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(rename = "configuredInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_input: Option<String>,
    #[serde(rename = "deviceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "inputResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_resolution: Option<String>,
    #[serde(rename = "latencyMs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<i32>,
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
    #[serde(rename = "mediaconnectSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediaconnect_settings: Option<InputDeviceMediaConnectSettings>,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDeviceUhdAudioChannelPairConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDeviceMediaConnectSettings {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "sourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInputDeviceThumbnailRequest {
    #[serde(rename = "Accept")]
    #[serde(default)]
    pub accept: String,
    #[serde(rename = "InputDeviceId")]
    #[serde(default)]
    pub input_device_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInputDeviceThumbnailResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "ContentLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInputRequest {
    #[serde(rename = "InputId")]
    #[serde(default)]
    pub input_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInputResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "attachedChannels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_channels: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<InputDestination>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inputClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_class: Option<String>,
    #[serde(rename = "inputDevices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_devices: Option<Vec<InputDeviceSettings>>,
    #[serde(rename = "inputNetworkLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_network_location: Option<String>,
    #[serde(rename = "inputPartnerIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_partner_ids: Option<Vec<String>>,
    #[serde(rename = "inputSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_source_type: Option<String>,
    #[serde(rename = "mediaConnectFlows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_flows: Option<Vec<MediaConnectFlow>>,
    #[serde(rename = "multicastSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicast_settings: Option<MulticastSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "routerSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_settings: Option<RouterInputSettings>,
    #[serde(rename = "sdiSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_sources: Option<Vec<String>>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "smpte2110ReceiverGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smpte2110_receiver_group_settings: Option<Smpte2110ReceiverGroupSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<InputSource>>,
    #[serde(rename = "srtSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_settings: Option<SrtSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInputSecurityGroupRequest {
    #[serde(rename = "InputSecurityGroupId")]
    #[serde(default)]
    pub input_security_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInputSecurityGroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "whitelistRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_rules: Option<Vec<InputWhitelistRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMultiplexProgramRequest {
    #[serde(rename = "MultiplexId")]
    #[serde(default)]
    pub multiplex_id: String,
    #[serde(rename = "ProgramName")]
    #[serde(default)]
    pub program_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMultiplexProgramResponse {
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "multiplexProgramSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_program_settings: Option<MultiplexProgramSettings>,
    #[serde(rename = "packetIdentifiersMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_identifiers_map: Option<MultiplexProgramPacketIdentifiersMap>,
    #[serde(rename = "pipelineDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<MultiplexProgramPipelineDetail>>,
    #[serde(rename = "programName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMultiplexRequest {
    #[serde(rename = "MultiplexId")]
    #[serde(default)]
    pub multiplex_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMultiplexResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<MultiplexOutputDestination>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "multiplexSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i32>,
    #[serde(rename = "programCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNetworkRequest {
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNetworkResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "associatedClusterIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_cluster_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ipPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_pools: Option<Vec<IpPool>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<Route>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNodeRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    pub node_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNodeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelPlacementGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_placement_groups: Option<Vec<String>>,
    #[serde(rename = "clusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "connectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "instanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nodeInterfaceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_interface_mappings: Option<Vec<NodeInterfaceMapping>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "sdiSourceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_source_mappings: Option<Vec<SdiSourceMapping>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOfferingRequest {
    #[serde(rename = "OfferingId")]
    #[serde(default)]
    pub offering_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOfferingResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "currencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "durationUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_units: Option<String>,
    #[serde(rename = "fixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(rename = "offeringDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_description: Option<String>,
    #[serde(rename = "offeringId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    #[serde(rename = "offeringType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ReservationResourceSpecification>,
    #[serde(rename = "usagePrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReservationRequest {
    #[serde(rename = "ReservationId")]
    #[serde(default)]
    pub reservation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReservationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "currencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "durationUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_units: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "fixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "offeringDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_description: Option<String>,
    #[serde(rename = "offeringId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    #[serde(rename = "offeringType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "renewalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_settings: Option<RenewalSettings>,
    #[serde(rename = "reservationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ReservationResourceSpecification>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "usagePrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScheduleRequest {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: String,
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
pub struct DescribeScheduleResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "scheduleActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_actions: Option<Vec<ScheduleAction>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSdiSourceRequest {
    #[serde(rename = "SdiSourceId")]
    #[serde(default)]
    pub sdi_source_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSdiSourceResponse {
    #[serde(rename = "sdiSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_source: Option<SdiSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThumbnailsRequest {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "PipelineId")]
    #[serde(default)]
    pub pipeline_id: String,
    #[serde(rename = "ThumbnailType")]
    #[serde(default)]
    pub thumbnail_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThumbnailsResponse {
    #[serde(rename = "thumbnailDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_details: Option<Vec<ThumbnailDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThumbnailDetail {
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnails: Option<Vec<Thumbnail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Thumbnail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "contentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "thumbnailType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_type: Option<String>,
    #[serde(rename = "timeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCloudWatchAlarmTemplateGroupRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCloudWatchAlarmTemplateGroupResponse {
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCloudWatchAlarmTemplateRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCloudWatchAlarmTemplateResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "comparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "datapointsToAlarm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints_to_alarm: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "evaluationPeriods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_periods: Option<i32>,
    #[serde(rename = "groupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "treatMissingData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_missing_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventBridgeRuleTemplateGroupRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventBridgeRuleTemplateGroupResponse {
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventBridgeRuleTemplateRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventBridgeRuleTemplateResponse {
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
    #[serde(rename = "eventTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_targets: Option<Vec<EventBridgeRuleTemplateTarget>>,
    #[serde(rename = "eventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "groupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSignalMapRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSignalMapResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "cloudWatchAlarmTemplateGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_alarm_template_group_ids: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "discoveryEntryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_entry_point_arn: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "eventBridgeRuleTemplateGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_rule_template_group_ids: Option<Vec<String>>,
    #[serde(rename = "failedMediaResourceMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_media_resource_map: Option<std::collections::HashMap<String, MediaResource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastDiscoveredAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_discovered_at: Option<String>,
    #[serde(rename = "lastSuccessfulMonitorDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_monitor_deployment: Option<SuccessfulMonitorDeployment>,
    #[serde(rename = "mediaResourceMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_resource_map: Option<std::collections::HashMap<String, MediaResource>>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "monitorChangesPendingDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_changes_pending_deployment: Option<bool>,
    #[serde(rename = "monitorDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_deployment: Option<MonitorDeployment>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAlertsRequest {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StateFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAlertsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<ChannelAlert>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelAlert {
    #[serde(rename = "alertType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<String>,
    #[serde(rename = "clearedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleared_timestamp: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    #[serde(rename = "setTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_timestamp: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChannelPlacementGroupsRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
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
pub struct ListChannelPlacementGroupsResponse {
    #[serde(rename = "channelPlacementGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_placement_groups: Option<Vec<DescribeChannelPlacementGroupSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeChannelPlacementGroupSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "clusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
    pub channels: Option<Vec<ChannelSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelSummary {
    #[serde(rename = "anywhereSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anywhere_settings: Option<DescribeAnywhereSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "cdiInputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    #[serde(rename = "channelClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    #[serde(rename = "channelEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_engine_version: Option<ChannelEngineVersionResponse>,
    #[serde(rename = "channelSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    #[serde(rename = "egressEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<ChannelEgressEndpoint>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inferenceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_settings: Option<DescribeInferenceSettings>,
    #[serde(rename = "inputAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    #[serde(rename = "inputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    #[serde(rename = "linkedChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_channel_settings: Option<DescribeLinkedChannelSettings>,
    #[serde(rename = "logLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "usedChannelEngineVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_channel_engine_versions: Option<Vec<ChannelEngineVersionResponse>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettingsDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCloudWatchAlarmTemplateGroupsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "SignalMapIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_map_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCloudWatchAlarmTemplateGroupsResponse {
    #[serde(rename = "cloudWatchAlarmTemplateGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_alarm_template_groups: Option<Vec<CloudWatchAlarmTemplateGroupSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchAlarmTemplateGroupSummary {
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "templateCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCloudWatchAlarmTemplatesRequest {
    #[serde(rename = "GroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "SignalMapIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_map_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCloudWatchAlarmTemplatesResponse {
    #[serde(rename = "cloudWatchAlarmTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_alarm_templates: Option<Vec<CloudWatchAlarmTemplateSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchAlarmTemplateSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "comparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "datapointsToAlarm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints_to_alarm: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "evaluationPeriods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_periods: Option<i32>,
    #[serde(rename = "groupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "treatMissingData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_missing_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClusterAlertsRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StateFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClusterAlertsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<ClusterAlert>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterAlert {
    #[serde(rename = "alertType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<String>,
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "clearedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleared_timestamp: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "nodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "setTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_timestamp: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClustersRequest {
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
pub struct ListClustersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<DescribeClusterSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClusterSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    #[serde(rename = "clusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "instanceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<ClusterNetworkSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventBridgeRuleTemplateGroupsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SignalMapIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_map_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventBridgeRuleTemplateGroupsResponse {
    #[serde(rename = "eventBridgeRuleTemplateGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_rule_template_groups: Option<Vec<EventBridgeRuleTemplateGroupSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventBridgeRuleTemplateGroupSummary {
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "templateCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventBridgeRuleTemplatesRequest {
    #[serde(rename = "GroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SignalMapIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_map_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventBridgeRuleTemplatesResponse {
    #[serde(rename = "eventBridgeRuleTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_rule_templates: Option<Vec<EventBridgeRuleTemplateSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventBridgeRuleTemplateSummary {
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
    #[serde(rename = "eventTargetCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_target_count: Option<i32>,
    #[serde(rename = "eventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "groupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInputDeviceTransfersRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TransferType")]
    #[serde(default)]
    pub transfer_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInputDeviceTransfersResponse {
    #[serde(rename = "inputDeviceTransfers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_device_transfers: Option<Vec<TransferringInputDeviceSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransferringInputDeviceSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "targetCustomerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_customer_id: Option<String>,
    #[serde(rename = "transferType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInputDevicesRequest {
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
pub struct ListInputDevicesResponse {
    #[serde(rename = "inputDevices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_devices: Option<Vec<InputDeviceSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDeviceSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "connectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "deviceSettingsSyncState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_settings_sync_state: Option<String>,
    #[serde(rename = "deviceUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_update_status: Option<String>,
    #[serde(rename = "hdDeviceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd_device_settings: Option<InputDeviceHdSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "macAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "medialiveInputArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medialive_input_arns: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<InputDeviceNetworkSettings>,
    #[serde(rename = "outputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,
    #[serde(rename = "serialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "uhdDeviceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uhd_device_settings: Option<InputDeviceUhdSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInputSecurityGroupsRequest {
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
pub struct ListInputSecurityGroupsResponse {
    #[serde(rename = "inputSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_security_groups: Option<Vec<InputSecurityGroup>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInputsRequest {
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
pub struct ListInputsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<Input>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMultiplexAlertsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MultiplexId")]
    #[serde(default)]
    pub multiplex_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StateFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMultiplexAlertsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<MultiplexAlert>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexAlert {
    #[serde(rename = "alertType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<String>,
    #[serde(rename = "clearedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleared_timestamp: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "pipelineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    #[serde(rename = "setTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_timestamp: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMultiplexProgramsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MultiplexId")]
    #[serde(default)]
    pub multiplex_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMultiplexProgramsResponse {
    #[serde(rename = "multiplexPrograms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_programs: Option<Vec<MultiplexProgramSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexProgramSummary {
    #[serde(rename = "channelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "programName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMultiplexesRequest {
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
pub struct ListMultiplexesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplexes: Option<Vec<MultiplexSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "multiplexSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexSettingsSummary>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i32>,
    #[serde(rename = "programCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiplexSettingsSummary {
    #[serde(rename = "transportStreamBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_bitrate: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNetworksRequest {
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
pub struct ListNetworksResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<DescribeNetworkSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNetworkSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "associatedClusterIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_cluster_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ipPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_pools: Option<Vec<IpPool>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<Route>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNodesRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
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
pub struct ListNodesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<DescribeNodeSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNodeSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelPlacementGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_placement_groups: Option<Vec<String>>,
    #[serde(rename = "clusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "connectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "instanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "managedInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_instance_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nodeInterfaceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_interface_mappings: Option<Vec<NodeInterfaceMapping>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "sdiSourceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_source_mappings: Option<Vec<SdiSourceMapping>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOfferingsRequest {
    #[serde(rename = "ChannelClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    #[serde(rename = "ChannelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_configuration: Option<String>,
    #[serde(rename = "Codec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MaximumBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<String>,
    #[serde(rename = "MaximumFramerate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_framerate: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Resolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "SpecialFeature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_feature: Option<String>,
    #[serde(rename = "VideoQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOfferingsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offerings: Option<Vec<Offering>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Offering {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "currencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "durationUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_units: Option<String>,
    #[serde(rename = "fixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(rename = "offeringDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_description: Option<String>,
    #[serde(rename = "offeringId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    #[serde(rename = "offeringType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ReservationResourceSpecification>,
    #[serde(rename = "usagePrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReservationsRequest {
    #[serde(rename = "ChannelClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    #[serde(rename = "Codec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MaximumBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<String>,
    #[serde(rename = "MaximumFramerate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_framerate: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Resolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "SpecialFeature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_feature: Option<String>,
    #[serde(rename = "VideoQuality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReservationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservations: Option<Vec<Reservation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Reservation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "currencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "durationUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_units: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "fixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "offeringDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_description: Option<String>,
    #[serde(rename = "offeringId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    #[serde(rename = "offeringType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "renewalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_settings: Option<RenewalSettings>,
    #[serde(rename = "reservationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ReservationResourceSpecification>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "usagePrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSdiSourcesRequest {
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
pub struct ListSdiSourcesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sdiSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_sources: Option<Vec<SdiSourceSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SdiSourceSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSignalMapsRequest {
    #[serde(rename = "CloudWatchAlarmTemplateGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_alarm_template_group_identifier: Option<String>,
    #[serde(rename = "EventBridgeRuleTemplateGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_rule_template_group_identifier: Option<String>,
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
pub struct ListSignalMapsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "signalMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_maps: Option<Vec<SignalMapSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignalMapSummary {
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "monitorDeploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_deployment_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
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
pub struct ListVersionsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVersionsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<ChannelEngineVersionResponse>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PurchaseOfferingRequest {
    #[serde(default)]
    pub count: i32,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OfferingId")]
    #[serde(default)]
    pub offering_id: String,
    #[serde(rename = "renewalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_settings: Option<RenewalSettings>,
    #[serde(rename = "requestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PurchaseOfferingResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<Reservation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootInputDeviceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<String>,
    #[serde(rename = "InputDeviceId")]
    #[serde(default)]
    pub input_device_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootInputDeviceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectInputDeviceTransferRequest {
    #[serde(rename = "InputDeviceId")]
    #[serde(default)]
    pub input_device_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectInputDeviceTransferResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestartChannelPipelinesRequest {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "pipelineIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestartChannelPipelinesResponse {
    #[serde(rename = "anywhereSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anywhere_settings: Option<DescribeAnywhereSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "cdiInputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    #[serde(rename = "channelClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    #[serde(rename = "channelEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_engine_version: Option<ChannelEngineVersionResponse>,
    #[serde(rename = "channelSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    #[serde(rename = "egressEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<ChannelEgressEndpoint>>,
    #[serde(rename = "encoderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inferenceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_settings: Option<DescribeInferenceSettings>,
    #[serde(rename = "inputAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    #[serde(rename = "inputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    #[serde(rename = "linkedChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_channel_settings: Option<DescribeLinkedChannelSettings>,
    #[serde(rename = "logLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceStatus>,
    #[serde(rename = "maintenanceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelineDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<PipelineDetail>>,
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettingsDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartChannelRequest {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartChannelResponse {
    #[serde(rename = "anywhereSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anywhere_settings: Option<DescribeAnywhereSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "cdiInputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    #[serde(rename = "channelClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    #[serde(rename = "channelEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_engine_version: Option<ChannelEngineVersionResponse>,
    #[serde(rename = "channelSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    #[serde(rename = "egressEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<ChannelEgressEndpoint>>,
    #[serde(rename = "encoderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inferenceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_settings: Option<DescribeInferenceSettings>,
    #[serde(rename = "inputAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    #[serde(rename = "inputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    #[serde(rename = "linkedChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_channel_settings: Option<DescribeLinkedChannelSettings>,
    #[serde(rename = "logLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelineDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<PipelineDetail>>,
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettingsDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDeleteMonitorDeploymentRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDeleteMonitorDeploymentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "cloudWatchAlarmTemplateGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_alarm_template_group_ids: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "discoveryEntryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_entry_point_arn: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "eventBridgeRuleTemplateGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_rule_template_group_ids: Option<Vec<String>>,
    #[serde(rename = "failedMediaResourceMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_media_resource_map: Option<std::collections::HashMap<String, MediaResource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastDiscoveredAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_discovered_at: Option<String>,
    #[serde(rename = "lastSuccessfulMonitorDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_monitor_deployment: Option<SuccessfulMonitorDeployment>,
    #[serde(rename = "mediaResourceMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_resource_map: Option<std::collections::HashMap<String, MediaResource>>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "monitorChangesPendingDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_changes_pending_deployment: Option<bool>,
    #[serde(rename = "monitorDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_deployment: Option<MonitorDeployment>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartInputDeviceMaintenanceWindowRequest {
    #[serde(rename = "InputDeviceId")]
    #[serde(default)]
    pub input_device_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartInputDeviceMaintenanceWindowResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartInputDeviceRequest {
    #[serde(rename = "InputDeviceId")]
    #[serde(default)]
    pub input_device_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartInputDeviceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMonitorDeploymentRequest {
    #[serde(rename = "dryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMonitorDeploymentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "cloudWatchAlarmTemplateGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_alarm_template_group_ids: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "discoveryEntryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_entry_point_arn: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "eventBridgeRuleTemplateGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_rule_template_group_ids: Option<Vec<String>>,
    #[serde(rename = "failedMediaResourceMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_media_resource_map: Option<std::collections::HashMap<String, MediaResource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastDiscoveredAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_discovered_at: Option<String>,
    #[serde(rename = "lastSuccessfulMonitorDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_monitor_deployment: Option<SuccessfulMonitorDeployment>,
    #[serde(rename = "mediaResourceMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_resource_map: Option<std::collections::HashMap<String, MediaResource>>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "monitorChangesPendingDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_changes_pending_deployment: Option<bool>,
    #[serde(rename = "monitorDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_deployment: Option<MonitorDeployment>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMultiplexRequest {
    #[serde(rename = "MultiplexId")]
    #[serde(default)]
    pub multiplex_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMultiplexResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<MultiplexOutputDestination>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "multiplexSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i32>,
    #[serde(rename = "programCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartUpdateSignalMapRequest {
    #[serde(rename = "cloudWatchAlarmTemplateGroupIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_alarm_template_group_identifiers: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "discoveryEntryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_entry_point_arn: Option<String>,
    #[serde(rename = "eventBridgeRuleTemplateGroupIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_rule_template_group_identifiers: Option<Vec<String>>,
    #[serde(rename = "forceRediscovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_rediscovery: Option<bool>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartUpdateSignalMapResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "cloudWatchAlarmTemplateGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_alarm_template_group_ids: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "discoveryEntryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_entry_point_arn: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "eventBridgeRuleTemplateGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_rule_template_group_ids: Option<Vec<String>>,
    #[serde(rename = "failedMediaResourceMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_media_resource_map: Option<std::collections::HashMap<String, MediaResource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastDiscoveredAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_discovered_at: Option<String>,
    #[serde(rename = "lastSuccessfulMonitorDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_monitor_deployment: Option<SuccessfulMonitorDeployment>,
    #[serde(rename = "mediaResourceMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_resource_map: Option<std::collections::HashMap<String, MediaResource>>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "monitorChangesPendingDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_changes_pending_deployment: Option<bool>,
    #[serde(rename = "monitorDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_deployment: Option<MonitorDeployment>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopChannelRequest {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopChannelResponse {
    #[serde(rename = "anywhereSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anywhere_settings: Option<DescribeAnywhereSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "cdiInputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    #[serde(rename = "channelClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    #[serde(rename = "channelEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_engine_version: Option<ChannelEngineVersionResponse>,
    #[serde(rename = "channelSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    #[serde(rename = "egressEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<ChannelEgressEndpoint>>,
    #[serde(rename = "encoderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inferenceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_settings: Option<DescribeInferenceSettings>,
    #[serde(rename = "inputAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    #[serde(rename = "inputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    #[serde(rename = "linkedChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_channel_settings: Option<DescribeLinkedChannelSettings>,
    #[serde(rename = "logLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelineDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<PipelineDetail>>,
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettingsDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopInputDeviceRequest {
    #[serde(rename = "InputDeviceId")]
    #[serde(default)]
    pub input_device_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopInputDeviceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopMultiplexRequest {
    #[serde(rename = "MultiplexId")]
    #[serde(default)]
    pub multiplex_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopMultiplexResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<MultiplexOutputDestination>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "multiplexSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i32>,
    #[serde(rename = "programCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransferInputDeviceRequest {
    #[serde(rename = "InputDeviceId")]
    #[serde(default)]
    pub input_device_id: String,
    #[serde(rename = "targetCustomerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_customer_id: Option<String>,
    #[serde(rename = "targetRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_region: Option<String>,
    #[serde(rename = "transferMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransferInputDeviceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountConfigurationRequest {
    #[serde(rename = "accountConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_configuration: Option<AccountConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountConfigurationResponse {
    #[serde(rename = "accountConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_configuration: Option<AccountConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelClassRequest {
    #[serde(rename = "channelClass")]
    #[serde(default)]
    pub channel_class: String,
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelClassResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelPlacementGroupRequest {
    #[serde(rename = "ChannelPlacementGroupId")]
    #[serde(default)]
    pub channel_placement_group_id: String,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelPlacementGroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "clusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelRequest {
    #[serde(rename = "anywhereSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anywhere_settings: Option<AnywhereSettings>,
    #[serde(rename = "cdiInputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    #[serde(rename = "channelEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_engine_version: Option<ChannelEngineVersionRequest>,
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: String,
    #[serde(rename = "channelSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_security_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    #[serde(rename = "dryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "encoderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    #[serde(rename = "inferenceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_settings: Option<InferenceSettings>,
    #[serde(rename = "inputAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    #[serde(rename = "inputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    #[serde(rename = "linkedChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_channel_settings: Option<LinkedChannelSettings>,
    #[serde(rename = "logLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceUpdateSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "specialRouterSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_router_settings: Option<SpecialRouterSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceUpdateSettings {
    #[serde(rename = "maintenanceDay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_day: Option<String>,
    #[serde(rename = "maintenanceScheduledDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_scheduled_date: Option<String>,
    #[serde(rename = "maintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SpecialRouterSettings {
    #[serde(rename = "routerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCloudWatchAlarmTemplateGroupRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCloudWatchAlarmTemplateGroupResponse {
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCloudWatchAlarmTemplateRequest {
    #[serde(rename = "comparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "datapointsToAlarm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints_to_alarm: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "evaluationPeriods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_periods: Option<i32>,
    #[serde(rename = "groupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(rename = "targetResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "treatMissingData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_missing_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCloudWatchAlarmTemplateResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "comparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "datapointsToAlarm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints_to_alarm: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "evaluationPeriods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_periods: Option<i32>,
    #[serde(rename = "groupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "targetResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "treatMissingData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_missing_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<ClusterNetworkSettingsUpdateRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterNetworkSettingsUpdateRequest {
    #[serde(rename = "defaultRoute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route: Option<String>,
    #[serde(rename = "interfaceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_mappings: Option<Vec<InterfaceMappingUpdateRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InterfaceMappingUpdateRequest {
    #[serde(rename = "logicalInterfaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_interface_name: Option<String>,
    #[serde(rename = "networkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    #[serde(rename = "clusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<ClusterNetworkSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEventBridgeRuleTemplateGroupRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEventBridgeRuleTemplateGroupResponse {
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
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEventBridgeRuleTemplateRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "eventTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_targets: Option<Vec<EventBridgeRuleTemplateTarget>>,
    #[serde(rename = "eventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "groupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEventBridgeRuleTemplateResponse {
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
    #[serde(rename = "eventTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_targets: Option<Vec<EventBridgeRuleTemplateTarget>>,
    #[serde(rename = "eventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "groupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInputDeviceRequest {
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "hdDeviceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd_device_settings: Option<InputDeviceConfigurableSettings>,
    #[serde(rename = "InputDeviceId")]
    #[serde(default)]
    pub input_device_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "uhdDeviceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uhd_device_settings: Option<InputDeviceConfigurableSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDeviceConfigurableSettings {
    #[serde(rename = "audioChannelPairs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_channel_pairs: Option<Vec<InputDeviceConfigurableAudioChannelPairConfig>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(rename = "configuredInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_input: Option<String>,
    #[serde(rename = "inputResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_resolution: Option<String>,
    #[serde(rename = "latencyMs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<i32>,
    #[serde(rename = "maxBitrate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i32>,
    #[serde(rename = "mediaconnectSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediaconnect_settings: Option<InputDeviceMediaConnectConfigurableSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDeviceConfigurableAudioChannelPairConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDeviceMediaConnectConfigurableSettings {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "sourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInputDeviceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "connectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "deviceSettingsSyncState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_settings_sync_state: Option<String>,
    #[serde(rename = "deviceUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_update_status: Option<String>,
    #[serde(rename = "hdDeviceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd_device_settings: Option<InputDeviceHdSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "macAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "medialiveInputArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medialive_input_arns: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<InputDeviceNetworkSettings>,
    #[serde(rename = "outputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,
    #[serde(rename = "serialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "uhdDeviceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uhd_device_settings: Option<InputDeviceUhdSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInputRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<InputDestinationRequest>>,
    #[serde(rename = "inputDevices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_devices: Option<Vec<InputDeviceRequest>>,
    #[serde(rename = "InputId")]
    #[serde(default)]
    pub input_id: String,
    #[serde(rename = "inputSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_security_groups: Option<Vec<String>>,
    #[serde(rename = "mediaConnectFlows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_flows: Option<Vec<MediaConnectFlowRequest>>,
    #[serde(rename = "multicastSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicast_settings: Option<MulticastSettingsUpdateRequest>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "sdiSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_sources: Option<Vec<String>>,
    #[serde(rename = "smpte2110ReceiverGroupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smpte2110_receiver_group_settings: Option<Smpte2110ReceiverGroupSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<InputSourceRequest>>,
    #[serde(rename = "specialRouterSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_router_settings: Option<SpecialRouterSettings>,
    #[serde(rename = "srtSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_settings: Option<SrtSettingsRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDeviceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MulticastSettingsUpdateRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<MulticastSourceUpdateRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MulticastSourceUpdateRequest {
    #[serde(rename = "sourceIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInputResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInputSecurityGroupRequest {
    #[serde(rename = "InputSecurityGroupId")]
    #[serde(default)]
    pub input_security_group_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "whitelistRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_rules: Option<Vec<InputWhitelistRuleCidr>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInputSecurityGroupResponse {
    #[serde(rename = "securityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<InputSecurityGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMultiplexProgramRequest {
    #[serde(rename = "MultiplexId")]
    #[serde(default)]
    pub multiplex_id: String,
    #[serde(rename = "multiplexProgramSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_program_settings: Option<MultiplexProgramSettings>,
    #[serde(rename = "ProgramName")]
    #[serde(default)]
    pub program_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMultiplexProgramResponse {
    #[serde(rename = "multiplexProgram")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_program: Option<MultiplexProgram>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMultiplexRequest {
    #[serde(rename = "MultiplexId")]
    #[serde(default)]
    pub multiplex_id: String,
    #[serde(rename = "multiplexSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "packetIdentifiersMapping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_identifiers_mapping:
        Option<std::collections::HashMap<String, MultiplexProgramPacketIdentifiersMap>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMultiplexResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex: Option<Multiplex>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNetworkRequest {
    #[serde(rename = "ipPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_pools: Option<Vec<IpPoolUpdateRequest>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkId")]
    #[serde(default)]
    pub network_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<RouteUpdateRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpPoolUpdateRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteUpdateRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNetworkResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "associatedClusterIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_cluster_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ipPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_pools: Option<Vec<IpPool>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<Route>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNodeRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    pub node_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "sdiSourceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_source_mappings: Option<Vec<SdiSourceMappingUpdateRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SdiSourceMappingUpdateRequest {
    #[serde(rename = "cardNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number: Option<i32>,
    #[serde(rename = "channelNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_number: Option<i32>,
    #[serde(rename = "sdiSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNodeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelPlacementGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_placement_groups: Option<Vec<String>>,
    #[serde(rename = "clusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "connectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "instanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nodeInterfaceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_interface_mappings: Option<Vec<NodeInterfaceMapping>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "sdiSourceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_source_mappings: Option<Vec<SdiSourceMapping>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNodeStateRequest {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    pub cluster_id: String,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    pub node_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNodeStateResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "channelPlacementGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_placement_groups: Option<Vec<String>>,
    #[serde(rename = "clusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "connectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "instanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nodeInterfaceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_interface_mappings: Option<Vec<NodeInterfaceMapping>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "sdiSourceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_source_mappings: Option<Vec<SdiSourceMapping>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReservationRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "renewalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_settings: Option<RenewalSettings>,
    #[serde(rename = "ReservationId")]
    #[serde(default)]
    pub reservation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReservationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<Reservation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSdiSourceRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SdiSourceId")]
    #[serde(default)]
    pub sdi_source_id: String,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSdiSourceResponse {
    #[serde(rename = "sdiSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_source: Option<SdiSource>,
}
