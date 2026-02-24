//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-sqs

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize void response for awsJson protocol.
pub fn serialize_add_permission_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_cancel_message_move_task_response(
    result: &CancelMessageMoveTaskResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_change_message_visibility_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_change_message_visibility_batch_response(
    result: &ChangeMessageVisibilityBatchResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_queue_response(result: &CreateQueueResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_message_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_message_batch_response(result: &DeleteMessageBatchResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_queue_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_queue_attributes_response(result: &GetQueueAttributesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_queue_url_response(result: &GetQueueUrlResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_dead_letter_source_queues_response(
    result: &ListDeadLetterSourceQueuesResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_message_move_tasks_response(
    result: &ListMessageMoveTasksResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_queue_tags_response(result: &ListQueueTagsResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_queues_response(result: &ListQueuesResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_purge_queue_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_receive_message_response(result: &ReceiveMessageResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_remove_permission_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_send_message_response(result: &SendMessageResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_send_message_batch_response(result: &SendMessageBatchResult) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_set_queue_attributes_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_message_move_task_response(
    result: &StartMessageMoveTaskResult,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_tag_queue_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_untag_queue_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_permission_request(body: &[u8]) -> Result<AddPermissionRequest, String> {
    if body.is_empty() {
        return Ok(AddPermissionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddPermission request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_cancel_message_move_task_request(
    body: &[u8],
) -> Result<CancelMessageMoveTaskRequest, String> {
    if body.is_empty() {
        return Ok(CancelMessageMoveTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CancelMessageMoveTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_change_message_visibility_request(
    body: &[u8],
) -> Result<ChangeMessageVisibilityRequest, String> {
    if body.is_empty() {
        return Ok(ChangeMessageVisibilityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ChangeMessageVisibility request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_change_message_visibility_batch_request(
    body: &[u8],
) -> Result<ChangeMessageVisibilityBatchRequest, String> {
    if body.is_empty() {
        return Ok(ChangeMessageVisibilityBatchRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ChangeMessageVisibilityBatch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_queue_request(body: &[u8]) -> Result<CreateQueueRequest, String> {
    if body.is_empty() {
        return Ok(CreateQueueRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateQueue request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_message_request(body: &[u8]) -> Result<DeleteMessageRequest, String> {
    if body.is_empty() {
        return Ok(DeleteMessageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteMessage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_message_batch_request(
    body: &[u8],
) -> Result<DeleteMessageBatchRequest, String> {
    if body.is_empty() {
        return Ok(DeleteMessageBatchRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteMessageBatch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_queue_request(body: &[u8]) -> Result<DeleteQueueRequest, String> {
    if body.is_empty() {
        return Ok(DeleteQueueRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteQueue request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_queue_attributes_request(
    body: &[u8],
) -> Result<GetQueueAttributesRequest, String> {
    if body.is_empty() {
        return Ok(GetQueueAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetQueueAttributes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_queue_url_request(body: &[u8]) -> Result<GetQueueUrlRequest, String> {
    if body.is_empty() {
        return Ok(GetQueueUrlRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetQueueUrl request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_dead_letter_source_queues_request(
    body: &[u8],
) -> Result<ListDeadLetterSourceQueuesRequest, String> {
    if body.is_empty() {
        return Ok(ListDeadLetterSourceQueuesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDeadLetterSourceQueues request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_message_move_tasks_request(
    body: &[u8],
) -> Result<ListMessageMoveTasksRequest, String> {
    if body.is_empty() {
        return Ok(ListMessageMoveTasksRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListMessageMoveTasks request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_queue_tags_request(body: &[u8]) -> Result<ListQueueTagsRequest, String> {
    if body.is_empty() {
        return Ok(ListQueueTagsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListQueueTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_queues_request(body: &[u8]) -> Result<ListQueuesRequest, String> {
    if body.is_empty() {
        return Ok(ListQueuesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListQueues request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_purge_queue_request(body: &[u8]) -> Result<PurgeQueueRequest, String> {
    if body.is_empty() {
        return Ok(PurgeQueueRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PurgeQueue request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_receive_message_request(body: &[u8]) -> Result<ReceiveMessageRequest, String> {
    if body.is_empty() {
        return Ok(ReceiveMessageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ReceiveMessage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_permission_request(
    body: &[u8],
) -> Result<RemovePermissionRequest, String> {
    if body.is_empty() {
        return Ok(RemovePermissionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemovePermission request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_send_message_request(body: &[u8]) -> Result<SendMessageRequest, String> {
    if body.is_empty() {
        return Ok(SendMessageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SendMessage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_send_message_batch_request(
    body: &[u8],
) -> Result<SendMessageBatchRequest, String> {
    if body.is_empty() {
        return Ok(SendMessageBatchRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SendMessageBatch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_queue_attributes_request(
    body: &[u8],
) -> Result<SetQueueAttributesRequest, String> {
    if body.is_empty() {
        return Ok(SetQueueAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetQueueAttributes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_message_move_task_request(
    body: &[u8],
) -> Result<StartMessageMoveTaskRequest, String> {
    if body.is_empty() {
        return Ok(StartMessageMoveTaskRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartMessageMoveTask request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_queue_request(body: &[u8]) -> Result<TagQueueRequest, String> {
    if body.is_empty() {
        return Ok(TagQueueRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize TagQueue request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_queue_request(body: &[u8]) -> Result<UntagQueueRequest, String> {
    if body.is_empty() {
        return Ok(UntagQueueRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagQueue request: {e}"))
}
