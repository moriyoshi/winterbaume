//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-mediapackage

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
pub fn serialize_configure_logs_response(result: &ConfigureLogsResponse) -> MockResponse {
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
pub fn serialize_create_harvest_job_response(result: &CreateHarvestJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_origin_endpoint_response(
    result: &CreateOriginEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_channel_response(result: &DeleteChannelResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_origin_endpoint_response(
    result: &DeleteOriginEndpointResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_channel_response(result: &DescribeChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_harvest_job_response(
    result: &DescribeHarvestJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_origin_endpoint_response(
    result: &DescribeOriginEndpointResponse,
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
pub fn serialize_list_harvest_jobs_response(result: &ListHarvestJobsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_origin_endpoints_response(
    result: &ListOriginEndpointsResponse,
) -> MockResponse {
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

/// Serialize response for restJson protocol.
pub fn serialize_rotate_channel_credentials_response(
    result: &RotateChannelCredentialsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_rotate_ingest_endpoint_credentials_response(
    result: &RotateIngestEndpointCredentialsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_channel_response(result: &UpdateChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_origin_endpoint_response(
    result: &UpdateOriginEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_configure_logs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ConfigureLogsRequest, String> {
    let mut input = ConfigureLogsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ConfigureLogsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ConfigureLogs request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
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
pub fn deserialize_create_harvest_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateHarvestJobRequest, String> {
    let mut input = CreateHarvestJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateHarvestJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateHarvestJob request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_origin_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateOriginEndpointRequest, String> {
    let mut input = CreateOriginEndpointRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateOriginEndpointRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateOriginEndpoint request: {err}"))?;
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
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_origin_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteOriginEndpointRequest, String> {
    let mut input = DeleteOriginEndpointRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_channel_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeChannelRequest, String> {
    let mut input = DescribeChannelRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_harvest_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeHarvestJobRequest, String> {
    let mut input = DescribeHarvestJobRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_origin_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeOriginEndpointRequest, String> {
    let mut input = DescribeOriginEndpointRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
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
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_harvest_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListHarvestJobsRequest, String> {
    let mut input = ListHarvestJobsRequest::default();
    if let Some(value) = query.get("includeChannelId") {
        input.include_channel_id = Some(value.to_string());
    }
    if let Some(value) = query.get("includeStatus") {
        input.include_status = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_origin_endpoints_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListOriginEndpointsRequest, String> {
    let mut input = ListOriginEndpointsRequest::default();
    if let Some(value) = query.get("channelId") {
        input.channel_id = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
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
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_rotate_channel_credentials_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RotateChannelCredentialsRequest, String> {
    let mut input = RotateChannelCredentialsRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_rotate_ingest_endpoint_credentials_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RotateIngestEndpointCredentialsRequest, String> {
    let mut input = RotateIngestEndpointCredentialsRequest::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            "IngestEndpointId" => {
                input.ingest_endpoint_id = value.to_string();
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
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
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
            "ResourceArn" => {
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
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_origin_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateOriginEndpointRequest, String> {
    let mut input = UpdateOriginEndpointRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateOriginEndpointRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateOriginEndpoint request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
