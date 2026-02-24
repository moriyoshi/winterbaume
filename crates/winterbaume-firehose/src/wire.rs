//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-firehose

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_delivery_stream_response(
    result: &CreateDeliveryStreamOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_delivery_stream_response(
    result: &DeleteDeliveryStreamOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_delivery_stream_response(
    result: &DescribeDeliveryStreamOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_delivery_streams_response(
    result: &ListDeliveryStreamsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_delivery_stream_response(
    result: &ListTagsForDeliveryStreamOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_record_response(result: &PutRecordOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_record_batch_response(result: &PutRecordBatchOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_delivery_stream_encryption_response(
    result: &StartDeliveryStreamEncryptionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_delivery_stream_encryption_response(
    result: &StopDeliveryStreamEncryptionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_delivery_stream_response(result: &TagDeliveryStreamOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_delivery_stream_response(
    result: &UntagDeliveryStreamOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_destination_response(result: &UpdateDestinationOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_delivery_stream_request(
    body: &[u8],
) -> Result<CreateDeliveryStreamInput, String> {
    if body.is_empty() {
        return Ok(CreateDeliveryStreamInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDeliveryStream request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_delivery_stream_request(
    body: &[u8],
) -> Result<DeleteDeliveryStreamInput, String> {
    if body.is_empty() {
        return Ok(DeleteDeliveryStreamInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDeliveryStream request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_delivery_stream_request(
    body: &[u8],
) -> Result<DescribeDeliveryStreamInput, String> {
    if body.is_empty() {
        return Ok(DescribeDeliveryStreamInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDeliveryStream request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_delivery_streams_request(
    body: &[u8],
) -> Result<ListDeliveryStreamsInput, String> {
    if body.is_empty() {
        return Ok(ListDeliveryStreamsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDeliveryStreams request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_delivery_stream_request(
    body: &[u8],
) -> Result<ListTagsForDeliveryStreamInput, String> {
    if body.is_empty() {
        return Ok(ListTagsForDeliveryStreamInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForDeliveryStream request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_record_request(body: &[u8]) -> Result<PutRecordInput, String> {
    if body.is_empty() {
        return Ok(PutRecordInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutRecord request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_record_batch_request(body: &[u8]) -> Result<PutRecordBatchInput, String> {
    if body.is_empty() {
        return Ok(PutRecordBatchInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutRecordBatch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_delivery_stream_encryption_request(
    body: &[u8],
) -> Result<StartDeliveryStreamEncryptionInput, String> {
    if body.is_empty() {
        return Ok(StartDeliveryStreamEncryptionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartDeliveryStreamEncryption request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_delivery_stream_encryption_request(
    body: &[u8],
) -> Result<StopDeliveryStreamEncryptionInput, String> {
    if body.is_empty() {
        return Ok(StopDeliveryStreamEncryptionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopDeliveryStreamEncryption request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_delivery_stream_request(
    body: &[u8],
) -> Result<TagDeliveryStreamInput, String> {
    if body.is_empty() {
        return Ok(TagDeliveryStreamInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagDeliveryStream request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_delivery_stream_request(
    body: &[u8],
) -> Result<UntagDeliveryStreamInput, String> {
    if body.is_empty() {
        return Ok(UntagDeliveryStreamInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagDeliveryStream request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_destination_request(
    body: &[u8],
) -> Result<UpdateDestinationInput, String> {
    if body.is_empty() {
        return Ok(UpdateDestinationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDestination request: {e}"))
}
