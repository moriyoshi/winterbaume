//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-kinesisvideoarchivedmedia

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
pub fn serialize_get_clip_response(result: &GetClipOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "content-type": set by caller from content_type field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_get_d_a_s_h_streaming_session_u_r_l_response(
    result: &GetDASHStreamingSessionURLOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_h_l_s_streaming_session_u_r_l_response(
    result: &GetHLSStreamingSessionURLOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_images_response(result: &GetImagesOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_media_for_fragment_list_response(
    result: &GetMediaForFragmentListOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "content-type": set by caller from content_type field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_list_fragments_response(result: &ListFragmentsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_clip_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetClipInput, String> {
    let mut input = GetClipInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetClipInput>(&request.body)
            .map_err(|err| format!("failed to deserialize GetClip request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_d_a_s_h_streaming_session_u_r_l_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDASHStreamingSessionURLInput, String> {
    let mut input = GetDASHStreamingSessionURLInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetDASHStreamingSessionURLInput>(&request.body).map_err(
            |err| format!("failed to deserialize GetDASHStreamingSessionURL request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_h_l_s_streaming_session_u_r_l_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetHLSStreamingSessionURLInput, String> {
    let mut input = GetHLSStreamingSessionURLInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetHLSStreamingSessionURLInput>(&request.body).map_err(
            |err| format!("failed to deserialize GetHLSStreamingSessionURL request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_images_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetImagesInput, String> {
    let mut input = GetImagesInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetImagesInput>(&request.body)
            .map_err(|err| format!("failed to deserialize GetImages request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_media_for_fragment_list_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMediaForFragmentListInput, String> {
    let mut input = GetMediaForFragmentListInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetMediaForFragmentListInput>(&request.body).map_err(
            |err| format!("failed to deserialize GetMediaForFragmentList request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_fragments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFragmentsInput, String> {
    let mut input = ListFragmentsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListFragmentsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListFragments request: {err}"))?;
    }
    Ok(input)
}
