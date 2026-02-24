//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-dynamodbstreams

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_describe_stream_response(result: &DescribeStreamOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_records_response(result: &GetRecordsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_shard_iterator_response(result: &GetShardIteratorOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_streams_response(result: &ListStreamsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_stream_request(body: &[u8]) -> Result<DescribeStreamInput, String> {
    if body.is_empty() {
        return Ok(DescribeStreamInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeStream request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_records_request(body: &[u8]) -> Result<GetRecordsInput, String> {
    if body.is_empty() {
        return Ok(GetRecordsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetRecords request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_shard_iterator_request(
    body: &[u8],
) -> Result<GetShardIteratorInput, String> {
    if body.is_empty() {
        return Ok(GetShardIteratorInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetShardIterator request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_streams_request(body: &[u8]) -> Result<ListStreamsInput, String> {
    if body.is_empty() {
        return Ok(ListStreamsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListStreams request: {e}"))
}
