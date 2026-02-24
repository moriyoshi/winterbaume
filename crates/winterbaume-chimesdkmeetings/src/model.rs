//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-chimesdkmeetings

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateAttendeeRequest {
    #[serde(rename = "Attendees")]
    #[serde(default)]
    pub attendees: Vec<CreateAttendeeRequestItem>,
    #[serde(rename = "MeetingId")]
    #[serde(default)]
    pub meeting_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAttendeeRequestItem {
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<AttendeeCapabilities>,
    #[serde(rename = "ExternalUserId")]
    #[serde(default)]
    pub external_user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttendeeCapabilities {
    #[serde(rename = "Audio")]
    #[serde(default)]
    pub audio: String,
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "Video")]
    #[serde(default)]
    pub video: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateAttendeeResponse {
    #[serde(rename = "Attendees")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<Attendee>>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<CreateAttendeeError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Attendee {
    #[serde(rename = "AttendeeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee_id: Option<String>,
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<AttendeeCapabilities>,
    #[serde(rename = "ExternalUserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_user_id: Option<String>,
    #[serde(rename = "JoinToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAttendeeError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ExternalUserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateAttendeeCapabilitiesExceptRequest {
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    pub capabilities: AttendeeCapabilities,
    #[serde(rename = "ExcludedAttendeeIds")]
    #[serde(default)]
    pub excluded_attendee_ids: Vec<AttendeeIdItem>,
    #[serde(rename = "MeetingId")]
    #[serde(default)]
    pub meeting_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttendeeIdItem {
    #[serde(rename = "AttendeeId")]
    #[serde(default)]
    pub attendee_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAttendeeRequest {
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<AttendeeCapabilities>,
    #[serde(rename = "ExternalUserId")]
    #[serde(default)]
    pub external_user_id: String,
    #[serde(rename = "MeetingId")]
    #[serde(default)]
    pub meeting_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAttendeeResponse {
    #[serde(rename = "Attendee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee: Option<Attendee>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMeetingRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    pub client_request_token: String,
    #[serde(rename = "ExternalMeetingId")]
    #[serde(default)]
    pub external_meeting_id: String,
    #[serde(rename = "MediaPlacementNetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_placement_network_type: Option<String>,
    #[serde(rename = "MediaRegion")]
    #[serde(default)]
    pub media_region: String,
    #[serde(rename = "MeetingFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_features: Option<MeetingFeaturesConfiguration>,
    #[serde(rename = "MeetingHostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_host_id: Option<String>,
    #[serde(rename = "NotificationsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_configuration: Option<NotificationsConfiguration>,
    #[serde(rename = "PrimaryMeetingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_meeting_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TenantIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeetingFeaturesConfiguration {
    #[serde(rename = "Attendee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee: Option<AttendeeFeatures>,
    #[serde(rename = "Audio")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioFeatures>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentFeatures>,
    #[serde(rename = "Video")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<VideoFeatures>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttendeeFeatures {
    #[serde(rename = "MaxCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioFeatures {
    #[serde(rename = "EchoReduction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo_reduction: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContentFeatures {
    #[serde(rename = "MaxResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_resolution: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VideoFeatures {
    #[serde(rename = "MaxResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_resolution: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationsConfiguration {
    #[serde(rename = "LambdaFunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_arn: Option<String>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "SqsQueueArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs_queue_arn: Option<String>,
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
pub struct CreateMeetingResponse {
    #[serde(rename = "Meeting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting: Option<Meeting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Meeting {
    #[serde(rename = "ExternalMeetingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_meeting_id: Option<String>,
    #[serde(rename = "MediaPlacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_placement: Option<MediaPlacement>,
    #[serde(rename = "MediaRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_region: Option<String>,
    #[serde(rename = "MeetingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_arn: Option<String>,
    #[serde(rename = "MeetingFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_features: Option<MeetingFeaturesConfiguration>,
    #[serde(rename = "MeetingHostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_host_id: Option<String>,
    #[serde(rename = "MeetingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_id: Option<String>,
    #[serde(rename = "PrimaryMeetingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_meeting_id: Option<String>,
    #[serde(rename = "TenantIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaPlacement {
    #[serde(rename = "AudioFallbackUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_fallback_url: Option<String>,
    #[serde(rename = "AudioHostUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_host_url: Option<String>,
    #[serde(rename = "EventIngestionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ingestion_url: Option<String>,
    #[serde(rename = "ScreenDataUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_data_url: Option<String>,
    #[serde(rename = "ScreenSharingUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_sharing_url: Option<String>,
    #[serde(rename = "ScreenViewingUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_viewing_url: Option<String>,
    #[serde(rename = "SignalingUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signaling_url: Option<String>,
    #[serde(rename = "TurnControlUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_control_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMeetingWithAttendeesRequest {
    #[serde(rename = "Attendees")]
    #[serde(default)]
    pub attendees: Vec<CreateAttendeeRequestItem>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    pub client_request_token: String,
    #[serde(rename = "ExternalMeetingId")]
    #[serde(default)]
    pub external_meeting_id: String,
    #[serde(rename = "MediaPlacementNetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_placement_network_type: Option<String>,
    #[serde(rename = "MediaRegion")]
    #[serde(default)]
    pub media_region: String,
    #[serde(rename = "MeetingFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_features: Option<MeetingFeaturesConfiguration>,
    #[serde(rename = "MeetingHostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_host_id: Option<String>,
    #[serde(rename = "NotificationsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_configuration: Option<NotificationsConfiguration>,
    #[serde(rename = "PrimaryMeetingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_meeting_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TenantIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMeetingWithAttendeesResponse {
    #[serde(rename = "Attendees")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<Attendee>>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<CreateAttendeeError>>,
    #[serde(rename = "Meeting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting: Option<Meeting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAttendeeRequest {
    #[serde(rename = "AttendeeId")]
    #[serde(default)]
    pub attendee_id: String,
    #[serde(rename = "MeetingId")]
    #[serde(default)]
    pub meeting_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMeetingRequest {
    #[serde(rename = "MeetingId")]
    #[serde(default)]
    pub meeting_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAttendeeRequest {
    #[serde(rename = "AttendeeId")]
    #[serde(default)]
    pub attendee_id: String,
    #[serde(rename = "MeetingId")]
    #[serde(default)]
    pub meeting_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAttendeeResponse {
    #[serde(rename = "Attendee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee: Option<Attendee>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMeetingRequest {
    #[serde(rename = "MeetingId")]
    #[serde(default)]
    pub meeting_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMeetingResponse {
    #[serde(rename = "Meeting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting: Option<Meeting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttendeesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MeetingId")]
    #[serde(default)]
    pub meeting_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttendeesResponse {
    #[serde(rename = "Attendees")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<Attendee>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMeetingTranscriptionRequest {
    #[serde(rename = "MeetingId")]
    #[serde(default)]
    pub meeting_id: String,
    #[serde(rename = "TranscriptionConfiguration")]
    #[serde(default)]
    pub transcription_configuration: TranscriptionConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TranscriptionConfiguration {
    #[serde(rename = "EngineTranscribeMedicalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_transcribe_medical_settings: Option<EngineTranscribeMedicalSettings>,
    #[serde(rename = "EngineTranscribeSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_transcribe_settings: Option<EngineTranscribeSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EngineTranscribeMedicalSettings {
    #[serde(rename = "ContentIdentificationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_identification_type: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "Specialty")]
    #[serde(default)]
    pub specialty: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EngineTranscribeSettings {
    #[serde(rename = "ContentIdentificationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_identification_type: Option<String>,
    #[serde(rename = "ContentRedactionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_redaction_type: Option<String>,
    #[serde(rename = "EnablePartialResultsStabilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_partial_results_stabilization: Option<bool>,
    #[serde(rename = "IdentifyLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify_language: Option<bool>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LanguageModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_model_name: Option<String>,
    #[serde(rename = "LanguageOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_options: Option<String>,
    #[serde(rename = "PartialResultsStability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_results_stability: Option<String>,
    #[serde(rename = "PiiEntityTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii_entity_types: Option<String>,
    #[serde(rename = "PreferredLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "VocabularyFilterMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_method: Option<String>,
    #[serde(rename = "VocabularyFilterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
    #[serde(rename = "VocabularyFilterNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_names: Option<String>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    #[serde(rename = "VocabularyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_names: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopMeetingTranscriptionRequest {
    #[serde(rename = "MeetingId")]
    #[serde(default)]
    pub meeting_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAttendeeCapabilitiesRequest {
    #[serde(rename = "AttendeeId")]
    #[serde(default)]
    pub attendee_id: String,
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    pub capabilities: AttendeeCapabilities,
    #[serde(rename = "MeetingId")]
    #[serde(default)]
    pub meeting_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAttendeeCapabilitiesResponse {
    #[serde(rename = "Attendee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee: Option<Attendee>,
}
