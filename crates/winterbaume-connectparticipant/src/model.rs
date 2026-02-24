//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-connectparticipant

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelParticipantAuthenticationRequest {
    #[serde(rename = "ConnectionToken")]
    #[serde(default)]
    pub connection_token: String,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelParticipantAuthenticationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteAttachmentUploadRequest {
    #[serde(rename = "AttachmentIds")]
    #[serde(default)]
    pub attachment_ids: Vec<String>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "ConnectionToken")]
    #[serde(default)]
    pub connection_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteAttachmentUploadResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateParticipantConnectionRequest {
    #[serde(rename = "ConnectParticipant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_participant: Option<bool>,
    #[serde(rename = "ParticipantToken")]
    #[serde(default)]
    pub participant_token: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateParticipantConnectionResponse {
    #[serde(rename = "ConnectionCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_credentials: Option<ConnectionCredentials>,
    #[serde(rename = "WebRTCConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_r_t_c_connection: Option<WebRTCConnection>,
    #[serde(rename = "Websocket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub websocket: Option<Websocket>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionCredentials {
    #[serde(rename = "ConnectionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_token: Option<String>,
    #[serde(rename = "Expiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebRTCConnection {
    #[serde(rename = "Attendee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee: Option<Attendee>,
    #[serde(rename = "Meeting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting: Option<WebRTCMeeting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Attendee {
    #[serde(rename = "AttendeeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee_id: Option<String>,
    #[serde(rename = "JoinToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebRTCMeeting {
    #[serde(rename = "MediaPlacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_placement: Option<WebRTCMediaPlacement>,
    #[serde(rename = "MeetingFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_features: Option<MeetingFeaturesConfiguration>,
    #[serde(rename = "MeetingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebRTCMediaPlacement {
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
    #[serde(rename = "SignalingUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signaling_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeetingFeaturesConfiguration {
    #[serde(rename = "Audio")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioFeatures>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioFeatures {
    #[serde(rename = "EchoReduction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo_reduction: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Websocket {
    #[serde(rename = "ConnectionExpiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_expiry: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeViewRequest {
    #[serde(rename = "ConnectionToken")]
    #[serde(default)]
    pub connection_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeViewResponse {
    #[serde(rename = "View")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<View>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct View {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ViewContent>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewContent {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(rename = "InputSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<String>,
    #[serde(rename = "Template")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisconnectParticipantRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ConnectionToken")]
    #[serde(default)]
    pub connection_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisconnectParticipantResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAttachmentRequest {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    pub attachment_id: String,
    #[serde(rename = "ConnectionToken")]
    #[serde(default)]
    pub connection_token: String,
    #[serde(rename = "UrlExpiryInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_expiry_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAttachmentResponse {
    #[serde(rename = "AttachmentSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_size_in_bytes: Option<i64>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "UrlExpiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_expiry: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAuthenticationUrlRequest {
    #[serde(rename = "ConnectionToken")]
    #[serde(default)]
    pub connection_token: String,
    #[serde(rename = "RedirectUri")]
    #[serde(default)]
    pub redirect_uri: String,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAuthenticationUrlResponse {
    #[serde(rename = "AuthenticationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTranscriptRequest {
    #[serde(rename = "ConnectionToken")]
    #[serde(default)]
    pub connection_token: String,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScanDirection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_direction: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "StartPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_position: Option<StartPosition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPosition {
    #[serde(rename = "AbsoluteTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MostRecent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTranscriptResponse {
    #[serde(rename = "InitialContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_contact_id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Transcript")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<Vec<Item>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "AbsoluteTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time: Option<String>,
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentItem>>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MessageMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_metadata: Option<MessageMetadata>,
    #[serde(rename = "ParticipantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    #[serde(rename = "ParticipantRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_role: Option<String>,
    #[serde(rename = "RelatedContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_contact_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachmentItem {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "AttachmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_name: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageMetadata {
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "MessageProcessingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_processing_status: Option<String>,
    #[serde(rename = "Receipts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipts: Option<Vec<Receipt>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Receipt {
    #[serde(rename = "DeliveredTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_timestamp: Option<String>,
    #[serde(rename = "ReadTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_timestamp: Option<String>,
    #[serde(rename = "RecipientParticipantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_participant_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendEventRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ConnectionToken")]
    #[serde(default)]
    pub connection_token: String,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    pub content_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendEventResponse {
    #[serde(rename = "AbsoluteTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendMessageRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ConnectionToken")]
    #[serde(default)]
    pub connection_token: String,
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    pub content_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendMessageResponse {
    #[serde(rename = "AbsoluteTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MessageMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_metadata: Option<MessageProcessingMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageProcessingMetadata {
    #[serde(rename = "MessageProcessingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_processing_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAttachmentUploadRequest {
    #[serde(rename = "AttachmentName")]
    #[serde(default)]
    pub attachment_name: String,
    #[serde(rename = "AttachmentSizeInBytes")]
    #[serde(default)]
    pub attachment_size_in_bytes: i64,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "ConnectionToken")]
    #[serde(default)]
    pub connection_token: String,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    pub content_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAttachmentUploadResponse {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "UploadMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_metadata: Option<UploadMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UploadMetadata {
    #[serde(rename = "HeadersToInclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers_to_include: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "UrlExpiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_expiry: Option<String>,
}
