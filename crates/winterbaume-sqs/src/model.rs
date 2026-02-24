//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-sqs

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddPermissionRequest {
    #[serde(rename = "AWSAccountIds")]
    #[serde(default)]
    pub a_w_s_account_ids: Vec<String>,
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: Vec<String>,
    #[serde(rename = "Label")]
    #[serde(default)]
    pub label: String,
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelMessageMoveTaskRequest {
    #[serde(rename = "TaskHandle")]
    #[serde(default)]
    pub task_handle: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelMessageMoveTaskResult {
    #[serde(rename = "ApproximateNumberOfMessagesMoved")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_number_of_messages_moved: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeMessageVisibilityBatchRequest {
    #[serde(rename = "Entries")]
    #[serde(default)]
    pub entries: Vec<ChangeMessageVisibilityBatchRequestEntry>,
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeMessageVisibilityBatchRequestEntry {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "ReceiptHandle")]
    #[serde(default)]
    pub receipt_handle: String,
    #[serde(rename = "VisibilityTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeMessageVisibilityBatchResult {
    #[serde(rename = "Failed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<BatchResultErrorEntry>>,
    #[serde(rename = "Successful")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<ChangeMessageVisibilityBatchResultEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchResultErrorEntry {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "SenderFault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_fault: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeMessageVisibilityBatchResultEntry {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeMessageVisibilityRequest {
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
    #[serde(rename = "ReceiptHandle")]
    #[serde(default)]
    pub receipt_handle: String,
    #[serde(rename = "VisibilityTimeout")]
    #[serde(default)]
    pub visibility_timeout: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateQueueRequest {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "QueueName")]
    #[serde(default)]
    pub queue_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateQueueResult {
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMessageBatchRequest {
    #[serde(rename = "Entries")]
    #[serde(default)]
    pub entries: Vec<DeleteMessageBatchRequestEntry>,
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMessageBatchRequestEntry {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "ReceiptHandle")]
    #[serde(default)]
    pub receipt_handle: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMessageBatchResult {
    #[serde(rename = "Failed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<BatchResultErrorEntry>>,
    #[serde(rename = "Successful")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<DeleteMessageBatchResultEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMessageBatchResultEntry {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMessageRequest {
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
    #[serde(rename = "ReceiptHandle")]
    #[serde(default)]
    pub receipt_handle: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteQueueRequest {
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueueAttributesRequest {
    #[serde(rename = "AttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<Vec<String>>,
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueueAttributesResult {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueueUrlRequest {
    #[serde(rename = "QueueName")]
    #[serde(default)]
    pub queue_name: String,
    #[serde(rename = "QueueOwnerAWSAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_owner_a_w_s_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueueUrlResult {
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeadLetterSourceQueuesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeadLetterSourceQueuesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "queueUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_urls: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMessageMoveTasksRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    pub source_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMessageMoveTasksResult {
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ListMessageMoveTasksResultEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMessageMoveTasksResultEntry {
    #[serde(rename = "ApproximateNumberOfMessagesMoved")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_number_of_messages_moved: Option<i64>,
    #[serde(rename = "ApproximateNumberOfMessagesToMove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_number_of_messages_to_move: Option<i64>,
    #[serde(rename = "DestinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "MaxNumberOfMessagesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_of_messages_per_second: Option<i32>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "StartedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_timestamp: Option<i64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TaskHandle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_handle: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQueueTagsRequest {
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQueueTagsResult {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQueuesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueueNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQueuesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueueUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_urls: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PurgeQueueRequest {
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReceiveMessageRequest {
    #[serde(rename = "AttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<Vec<String>>,
    #[serde(rename = "MaxNumberOfMessages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_of_messages: Option<i32>,
    #[serde(rename = "MessageAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_attribute_names: Option<Vec<String>>,
    #[serde(rename = "MessageSystemAttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_system_attribute_names: Option<Vec<String>>,
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
    #[serde(rename = "ReceiveRequestAttemptId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_request_attempt_id: Option<String>,
    #[serde(rename = "VisibilityTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_timeout: Option<i32>,
    #[serde(rename = "WaitTimeSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_time_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReceiveMessageResult {
    #[serde(rename = "Messages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Message {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "MD5OfBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_d5_of_body: Option<String>,
    #[serde(rename = "MD5OfMessageAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_d5_of_message_attributes: Option<String>,
    #[serde(rename = "MessageAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_attributes: Option<std::collections::HashMap<String, MessageAttributeValue>>,
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "ReceiptHandle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_handle: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageAttributeValue {
    #[serde(rename = "BinaryListValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_list_values: Option<Vec<String>>,
    #[serde(rename = "BinaryValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_value: Option<String>,
    #[serde(rename = "DataType")]
    #[serde(default)]
    pub data_type: String,
    #[serde(rename = "StringListValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_list_values: Option<Vec<String>>,
    #[serde(rename = "StringValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemovePermissionRequest {
    #[serde(rename = "Label")]
    #[serde(default)]
    pub label: String,
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendMessageBatchRequest {
    #[serde(rename = "Entries")]
    #[serde(default)]
    pub entries: Vec<SendMessageBatchRequestEntry>,
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendMessageBatchRequestEntry {
    #[serde(rename = "DelaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_seconds: Option<i32>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "MessageAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_attributes: Option<std::collections::HashMap<String, MessageAttributeValue>>,
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    pub message_body: String,
    #[serde(rename = "MessageDeduplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_deduplication_id: Option<String>,
    #[serde(rename = "MessageGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_group_id: Option<String>,
    #[serde(rename = "MessageSystemAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_system_attributes:
        Option<std::collections::HashMap<String, MessageSystemAttributeValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageSystemAttributeValue {
    #[serde(rename = "BinaryListValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_list_values: Option<Vec<String>>,
    #[serde(rename = "BinaryValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_value: Option<String>,
    #[serde(rename = "DataType")]
    #[serde(default)]
    pub data_type: String,
    #[serde(rename = "StringListValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_list_values: Option<Vec<String>>,
    #[serde(rename = "StringValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendMessageBatchResult {
    #[serde(rename = "Failed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<BatchResultErrorEntry>>,
    #[serde(rename = "Successful")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<SendMessageBatchResultEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendMessageBatchResultEntry {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MD5OfMessageAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_d5_of_message_attributes: Option<String>,
    #[serde(rename = "MD5OfMessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_d5_of_message_body: Option<String>,
    #[serde(rename = "MD5OfMessageSystemAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_d5_of_message_system_attributes: Option<String>,
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "SequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendMessageRequest {
    #[serde(rename = "DelaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_seconds: Option<i32>,
    #[serde(rename = "MessageAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_attributes: Option<std::collections::HashMap<String, MessageAttributeValue>>,
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    pub message_body: String,
    #[serde(rename = "MessageDeduplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_deduplication_id: Option<String>,
    #[serde(rename = "MessageGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_group_id: Option<String>,
    #[serde(rename = "MessageSystemAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_system_attributes:
        Option<std::collections::HashMap<String, MessageSystemAttributeValue>>,
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendMessageResult {
    #[serde(rename = "MD5OfMessageAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_d5_of_message_attributes: Option<String>,
    #[serde(rename = "MD5OfMessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_d5_of_message_body: Option<String>,
    #[serde(rename = "MD5OfMessageSystemAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_d5_of_message_system_attributes: Option<String>,
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "SequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetQueueAttributesRequest {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, String>,
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMessageMoveTaskRequest {
    #[serde(rename = "DestinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    #[serde(rename = "MaxNumberOfMessagesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_of_messages_per_second: Option<i32>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    pub source_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMessageMoveTaskResult {
    #[serde(rename = "TaskHandle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_handle: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagQueueRequest {
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagQueueRequest {
    #[serde(rename = "QueueUrl")]
    #[serde(default)]
    pub queue_url: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}
