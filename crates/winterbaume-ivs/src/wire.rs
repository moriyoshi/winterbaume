//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ivs

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
pub fn serialize_batch_get_channel_response(result: &BatchGetChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "access-control-allow-origin": set by caller from access_control_allow_origin field
    // Header "access-control-expose-headers": set by caller from access_control_expose_headers field
    // Header "cache-control": set by caller from cache_control field
    // Header "content-security-policy": set by caller from content_security_policy field
    // Header "strict-transport-security": set by caller from strict_transport_security field
    // Header "x-content-type-options": set by caller from x_content_type_options field
    // Header "x-frame-options": set by caller from x_frame_options field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_stream_key_response(result: &BatchGetStreamKeyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "access-control-allow-origin": set by caller from access_control_allow_origin field
    // Header "access-control-expose-headers": set by caller from access_control_expose_headers field
    // Header "cache-control": set by caller from cache_control field
    // Header "content-security-policy": set by caller from content_security_policy field
    // Header "strict-transport-security": set by caller from strict_transport_security field
    // Header "x-content-type-options": set by caller from x_content_type_options field
    // Header "x-frame-options": set by caller from x_frame_options field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_start_viewer_session_revocation_response(
    result: &BatchStartViewerSessionRevocationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "access-control-allow-origin": set by caller from access_control_allow_origin field
    // Header "access-control-expose-headers": set by caller from access_control_expose_headers field
    // Header "cache-control": set by caller from cache_control field
    // Header "content-security-policy": set by caller from content_security_policy field
    // Header "strict-transport-security": set by caller from strict_transport_security field
    // Header "x-content-type-options": set by caller from x_content_type_options field
    // Header "x-frame-options": set by caller from x_frame_options field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_create_ad_configuration_response(
    result: &CreateAdConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_channel_response(result: &CreateChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_playback_restriction_policy_response(
    result: &CreatePlaybackRestrictionPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_recording_configuration_response(
    result: &CreateRecordingConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_stream_key_response(result: &CreateStreamKeyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_ad_configuration_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_channel_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_playback_key_pair_response(
    result: &DeletePlaybackKeyPairResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_playback_restriction_policy_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_recording_configuration_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_stream_key_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_get_ad_configuration_response(
    result: &GetAdConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_channel_response(result: &GetChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_playback_key_pair_response(
    result: &GetPlaybackKeyPairResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_playback_restriction_policy_response(
    result: &GetPlaybackRestrictionPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_recording_configuration_response(
    result: &GetRecordingConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_stream_response(result: &GetStreamResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_stream_key_response(result: &GetStreamKeyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_stream_session_response(result: &GetStreamSessionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_import_playback_key_pair_response(
    result: &ImportPlaybackKeyPairResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_insert_ad_break_response(result: &InsertAdBreakResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_ad_configurations_response(
    result: &ListAdConfigurationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_channels_response(result: &ListChannelsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_playback_key_pairs_response(
    result: &ListPlaybackKeyPairsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_playback_restriction_policies_response(
    result: &ListPlaybackRestrictionPoliciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_recording_configurations_response(
    result: &ListRecordingConfigurationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_stream_keys_response(result: &ListStreamKeysResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_stream_sessions_response(
    result: &ListStreamSessionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_streams_response(result: &ListStreamsResponse) -> MockResponse {
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
pub fn serialize_put_metadata_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_start_viewer_session_revocation_response(
    result: &StartViewerSessionRevocationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_stream_response(result: &StopStreamResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_channel_response(result: &UpdateChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_playback_restriction_policy_response(
    result: &UpdatePlaybackRestrictionPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetChannelRequest, String> {
    let mut input = BatchGetChannelRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetChannelRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchGetChannel request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_stream_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetStreamKeyRequest, String> {
    let mut input = BatchGetStreamKeyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetStreamKeyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchGetStreamKey request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_start_viewer_session_revocation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchStartViewerSessionRevocationRequest, String> {
    let mut input = BatchStartViewerSessionRevocationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchStartViewerSessionRevocationRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize BatchStartViewerSessionRevocation request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_ad_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAdConfigurationRequest, String> {
    let mut input = CreateAdConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAdConfigurationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAdConfiguration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateChannelRequest, String> {
    let mut input = CreateChannelRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateChannelRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateChannel request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_playback_restriction_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePlaybackRestrictionPolicyRequest, String> {
    let mut input = CreatePlaybackRestrictionPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePlaybackRestrictionPolicyRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreatePlaybackRestrictionPolicy request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_recording_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRecordingConfigurationRequest, String> {
    let mut input = CreateRecordingConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRecordingConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateRecordingConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_stream_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateStreamKeyRequest, String> {
    let mut input = CreateStreamKeyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateStreamKeyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateStreamKey request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_ad_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAdConfigurationRequest, String> {
    let mut input = DeleteAdConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteAdConfigurationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteAdConfiguration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteChannelRequest, String> {
    let mut input = DeleteChannelRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteChannelRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteChannel request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_playback_key_pair_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePlaybackKeyPairRequest, String> {
    let mut input = DeletePlaybackKeyPairRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeletePlaybackKeyPairRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeletePlaybackKeyPair request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_playback_restriction_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePlaybackRestrictionPolicyRequest, String> {
    let mut input = DeletePlaybackRestrictionPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeletePlaybackRestrictionPolicyRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DeletePlaybackRestrictionPolicy request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_recording_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRecordingConfigurationRequest, String> {
    let mut input = DeleteRecordingConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteRecordingConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DeleteRecordingConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_stream_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteStreamKeyRequest, String> {
    let mut input = DeleteStreamKeyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteStreamKeyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteStreamKey request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_ad_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAdConfigurationRequest, String> {
    let mut input = GetAdConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetAdConfigurationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetAdConfiguration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetChannelRequest, String> {
    let mut input = GetChannelRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetChannelRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetChannel request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_playback_key_pair_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPlaybackKeyPairRequest, String> {
    let mut input = GetPlaybackKeyPairRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetPlaybackKeyPairRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetPlaybackKeyPair request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_playback_restriction_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPlaybackRestrictionPolicyRequest, String> {
    let mut input = GetPlaybackRestrictionPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetPlaybackRestrictionPolicyRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize GetPlaybackRestrictionPolicy request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_recording_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRecordingConfigurationRequest, String> {
    let mut input = GetRecordingConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetRecordingConfigurationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize GetRecordingConfiguration request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetStreamRequest, String> {
    let mut input = GetStreamRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetStreamRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetStream request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_stream_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetStreamKeyRequest, String> {
    let mut input = GetStreamKeyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetStreamKeyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetStreamKey request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_stream_session_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetStreamSessionRequest, String> {
    let mut input = GetStreamSessionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetStreamSessionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetStreamSession request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_import_playback_key_pair_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ImportPlaybackKeyPairRequest, String> {
    let mut input = ImportPlaybackKeyPairRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ImportPlaybackKeyPairRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ImportPlaybackKeyPair request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_insert_ad_break_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<InsertAdBreakRequest, String> {
    let mut input = InsertAdBreakRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<InsertAdBreakRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize InsertAdBreak request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_ad_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAdConfigurationsRequest, String> {
    let mut input = ListAdConfigurationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAdConfigurationsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListAdConfigurations request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_channels_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListChannelsRequest, String> {
    let mut input = ListChannelsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListChannelsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListChannels request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_playback_key_pairs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPlaybackKeyPairsRequest, String> {
    let mut input = ListPlaybackKeyPairsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListPlaybackKeyPairsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListPlaybackKeyPairs request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_playback_restriction_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPlaybackRestrictionPoliciesRequest, String> {
    let mut input = ListPlaybackRestrictionPoliciesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListPlaybackRestrictionPoliciesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListPlaybackRestrictionPolicies request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_recording_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRecordingConfigurationsRequest, String> {
    let mut input = ListRecordingConfigurationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListRecordingConfigurationsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListRecordingConfigurations request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_stream_keys_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListStreamKeysRequest, String> {
    let mut input = ListStreamKeysRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListStreamKeysRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListStreamKeys request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_stream_sessions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListStreamSessionsRequest, String> {
    let mut input = ListStreamSessionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListStreamSessionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListStreamSessions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_streams_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListStreamsRequest, String> {
    let mut input = ListStreamsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListStreamsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListStreams request: {err}"))?;
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
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutMetadataRequest, String> {
    let mut input = PutMetadataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutMetadataRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutMetadata request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_viewer_session_revocation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartViewerSessionRevocationRequest, String> {
    let mut input = StartViewerSessionRevocationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartViewerSessionRevocationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize StartViewerSessionRevocation request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopStreamRequest, String> {
    let mut input = StopStreamRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StopStreamRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StopStream request: {err}"))?;
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
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
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
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("tagKeys") {
        input.tag_keys = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateChannelRequest, String> {
    let mut input = UpdateChannelRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateChannelRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateChannel request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_playback_restriction_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePlaybackRestrictionPolicyRequest, String> {
    let mut input = UpdatePlaybackRestrictionPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePlaybackRestrictionPolicyRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdatePlaybackRestrictionPolicy request: {err}")
            })?;
    }
    Ok(input)
}
