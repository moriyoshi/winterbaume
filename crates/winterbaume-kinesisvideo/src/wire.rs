//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-kinesisvideo

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
pub fn serialize_create_signaling_channel_response(
    result: &CreateSignalingChannelOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_stream_response(result: &CreateStreamOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_edge_configuration_response(
    result: &DeleteEdgeConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_signaling_channel_response(
    result: &DeleteSignalingChannelOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_stream_response(result: &DeleteStreamOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_edge_configuration_response(
    result: &DescribeEdgeConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_image_generation_configuration_response(
    result: &DescribeImageGenerationConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_mapped_resource_configuration_response(
    result: &DescribeMappedResourceConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_media_storage_configuration_response(
    result: &DescribeMediaStorageConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_notification_configuration_response(
    result: &DescribeNotificationConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_signaling_channel_response(
    result: &DescribeSignalingChannelOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_stream_response(result: &DescribeStreamOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_stream_storage_configuration_response(
    result: &DescribeStreamStorageConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_data_endpoint_response(result: &GetDataEndpointOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_signaling_channel_endpoint_response(
    result: &GetSignalingChannelEndpointOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_edge_agent_configurations_response(
    result: &ListEdgeAgentConfigurationsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_signaling_channels_response(
    result: &ListSignalingChannelsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_streams_response(result: &ListStreamsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_stream_response(result: &ListTagsForStreamOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_edge_configuration_update_response(
    result: &StartEdgeConfigurationUpdateOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_stream_response(result: &TagStreamOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_stream_response(result: &UntagStreamOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_data_retention_response(
    result: &UpdateDataRetentionOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_image_generation_configuration_response(
    result: &UpdateImageGenerationConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_media_storage_configuration_response(
    result: &UpdateMediaStorageConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_notification_configuration_response(
    result: &UpdateNotificationConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_signaling_channel_response(
    result: &UpdateSignalingChannelOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_stream_response(result: &UpdateStreamOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_stream_storage_configuration_response(
    result: &UpdateStreamStorageConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_signaling_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSignalingChannelInput, String> {
    let mut input = CreateSignalingChannelInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSignalingChannelInput>(&request.body).map_err(
            |err| format!("failed to deserialize CreateSignalingChannel request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateStreamInput, String> {
    let mut input = CreateStreamInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateStreamInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateStream request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_edge_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteEdgeConfigurationInput, String> {
    let mut input = DeleteEdgeConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteEdgeConfigurationInput>(&request.body).map_err(
            |err| format!("failed to deserialize DeleteEdgeConfiguration request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_signaling_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteSignalingChannelInput, String> {
    let mut input = DeleteSignalingChannelInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteSignalingChannelInput>(&request.body).map_err(
            |err| format!("failed to deserialize DeleteSignalingChannel request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteStreamInput, String> {
    let mut input = DeleteStreamInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteStreamInput>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteStream request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_edge_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeEdgeConfigurationInput, String> {
    let mut input = DescribeEdgeConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeEdgeConfigurationInput>(&request.body).map_err(
            |err| format!("failed to deserialize DescribeEdgeConfiguration request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_image_generation_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeImageGenerationConfigurationInput, String> {
    let mut input = DescribeImageGenerationConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeImageGenerationConfigurationInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeImageGenerationConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_mapped_resource_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeMappedResourceConfigurationInput, String> {
    let mut input = DescribeMappedResourceConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeMappedResourceConfigurationInput>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize DescribeMappedResourceConfiguration request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_media_storage_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeMediaStorageConfigurationInput, String> {
    let mut input = DescribeMediaStorageConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeMediaStorageConfigurationInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeMediaStorageConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_notification_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeNotificationConfigurationInput, String> {
    let mut input = DescribeNotificationConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeNotificationConfigurationInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeNotificationConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_signaling_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeSignalingChannelInput, String> {
    let mut input = DescribeSignalingChannelInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeSignalingChannelInput>(&request.body).map_err(
            |err| format!("failed to deserialize DescribeSignalingChannel request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeStreamInput, String> {
    let mut input = DescribeStreamInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeStreamInput>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeStream request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_stream_storage_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeStreamStorageConfigurationInput, String> {
    let mut input = DescribeStreamStorageConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeStreamStorageConfigurationInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeStreamStorageConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_data_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDataEndpointInput, String> {
    let mut input = GetDataEndpointInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetDataEndpointInput>(&request.body)
            .map_err(|err| format!("failed to deserialize GetDataEndpoint request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_signaling_channel_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSignalingChannelEndpointInput, String> {
    let mut input = GetSignalingChannelEndpointInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetSignalingChannelEndpointInput>(&request.body).map_err(
            |err| format!("failed to deserialize GetSignalingChannelEndpoint request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_edge_agent_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEdgeAgentConfigurationsInput, String> {
    let mut input = ListEdgeAgentConfigurationsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListEdgeAgentConfigurationsInput>(&request.body).map_err(
            |err| format!("failed to deserialize ListEdgeAgentConfigurations request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_signaling_channels_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSignalingChannelsInput, String> {
    let mut input = ListSignalingChannelsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListSignalingChannelsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListSignalingChannels request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_streams_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListStreamsInput, String> {
    let mut input = ListStreamsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListStreamsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListStreams request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceInput, String> {
    let mut input = ListTagsForResourceInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTagsForResourceInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTagsForResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_for_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForStreamInput, String> {
    let mut input = ListTagsForStreamInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTagsForStreamInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTagsForStream request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_edge_configuration_update_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartEdgeConfigurationUpdateInput, String> {
    let mut input = StartEdgeConfigurationUpdateInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartEdgeConfigurationUpdateInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize StartEdgeConfigurationUpdate request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceInput, String> {
    let mut input = TagResourceInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceInput>(&request.body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagStreamInput, String> {
    let mut input = TagStreamInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagStreamInput>(&request.body)
            .map_err(|err| format!("failed to deserialize TagStream request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceInput, String> {
    let mut input = UntagResourceInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UntagResourceInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UntagResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagStreamInput, String> {
    let mut input = UntagStreamInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UntagStreamInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UntagStream request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_data_retention_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDataRetentionInput, String> {
    let mut input = UpdateDataRetentionInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDataRetentionInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDataRetention request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_image_generation_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateImageGenerationConfigurationInput, String> {
    let mut input = UpdateImageGenerationConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateImageGenerationConfigurationInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateImageGenerationConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_media_storage_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateMediaStorageConfigurationInput, String> {
    let mut input = UpdateMediaStorageConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateMediaStorageConfigurationInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateMediaStorageConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_notification_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateNotificationConfigurationInput, String> {
    let mut input = UpdateNotificationConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateNotificationConfigurationInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateNotificationConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_signaling_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSignalingChannelInput, String> {
    let mut input = UpdateSignalingChannelInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSignalingChannelInput>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateSignalingChannel request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateStreamInput, String> {
    let mut input = UpdateStreamInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateStreamInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateStream request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_stream_storage_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateStreamStorageConfigurationInput, String> {
    let mut input = UpdateStreamStorageConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateStreamStorageConfigurationInput>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateStreamStorageConfiguration request: {err}")
            })?;
    }
    Ok(input)
}
