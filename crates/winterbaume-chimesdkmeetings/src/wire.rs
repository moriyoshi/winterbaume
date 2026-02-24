//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-chimesdkmeetings

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restJson protocol.
pub fn serialize_batch_create_attendee_response(
    result: &BatchCreateAttendeeResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_batch_update_attendee_capabilities_except_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_create_attendee_response(result: &CreateAttendeeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_meeting_response(result: &CreateMeetingResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_meeting_with_attendees_response(
    result: &CreateMeetingWithAttendeesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_attendee_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_meeting_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_get_attendee_response(result: &GetAttendeeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_meeting_response(result: &GetMeetingResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_attendees_response(result: &ListAttendeesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_start_meeting_transcription_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_stop_meeting_transcription_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_attendee_capabilities_response(
    result: &UpdateAttendeeCapabilitiesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_create_attendee_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchCreateAttendeeRequest, String> {
    let mut input = BatchCreateAttendeeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchCreateAttendeeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchCreateAttendee request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "MeetingId" => {
                input.meeting_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_update_attendee_capabilities_except_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchUpdateAttendeeCapabilitiesExceptRequest, String> {
    let mut input = BatchUpdateAttendeeCapabilitiesExceptRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<BatchUpdateAttendeeCapabilitiesExceptRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize BatchUpdateAttendeeCapabilitiesExcept request: {err}"
                )
            })?;
    }
    for (name, value) in labels {
        match *name {
            "MeetingId" => {
                input.meeting_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_attendee_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAttendeeRequest, String> {
    let mut input = CreateAttendeeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAttendeeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAttendee request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "MeetingId" => {
                input.meeting_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_meeting_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMeetingRequest, String> {
    let mut input = CreateMeetingRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateMeetingRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateMeeting request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_meeting_with_attendees_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMeetingWithAttendeesRequest, String> {
    let mut input = CreateMeetingWithAttendeesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateMeetingWithAttendeesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateMeetingWithAttendees request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_attendee_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAttendeeRequest, String> {
    let mut input = DeleteAttendeeRequest::default();
    for (name, value) in labels {
        match *name {
            "AttendeeId" => {
                input.attendee_id = value.to_string();
            }
            "MeetingId" => {
                input.meeting_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_meeting_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMeetingRequest, String> {
    let mut input = DeleteMeetingRequest::default();
    for (name, value) in labels {
        match *name {
            "MeetingId" => {
                input.meeting_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_attendee_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAttendeeRequest, String> {
    let mut input = GetAttendeeRequest::default();
    for (name, value) in labels {
        match *name {
            "AttendeeId" => {
                input.attendee_id = value.to_string();
            }
            "MeetingId" => {
                input.meeting_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_meeting_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMeetingRequest, String> {
    let mut input = GetMeetingRequest::default();
    for (name, value) in labels {
        match *name {
            "MeetingId" => {
                input.meeting_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_attendees_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAttendeesRequest, String> {
    let mut input = ListAttendeesRequest::default();
    for (name, value) in labels {
        match *name {
            "MeetingId" => {
                input.meeting_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max-results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next-token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceRequest, String> {
    let mut input = ListTagsForResourceRequest::default();
    if let Some(value) = query.get("arn") {
        input.resource_a_r_n = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_meeting_transcription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartMeetingTranscriptionRequest, String> {
    let mut input = StartMeetingTranscriptionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartMeetingTranscriptionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartMeetingTranscription request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "MeetingId" => {
                input.meeting_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_meeting_transcription_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopMeetingTranscriptionRequest, String> {
    let mut input = StopMeetingTranscriptionRequest::default();
    for (name, value) in labels {
        match *name {
            "MeetingId" => {
                input.meeting_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceRequest, String> {
    let mut input = TagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UntagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UntagResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_attendee_capabilities_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAttendeeCapabilitiesRequest, String> {
    let mut input = UpdateAttendeeCapabilitiesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAttendeeCapabilitiesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateAttendeeCapabilities request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "AttendeeId" => {
                input.attendee_id = value.to_string();
            }
            "MeetingId" => {
                input.meeting_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
