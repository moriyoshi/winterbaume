//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-kinesis

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize void response for awsJson protocol.
pub fn serialize_add_tags_to_stream_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_create_stream_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_decrease_stream_retention_period_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_resource_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_stream_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_deregister_stream_consumer_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_account_settings_response(
    result: &DescribeAccountSettingsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_limits_response(result: &DescribeLimitsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_stream_response(result: &DescribeStreamOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_stream_consumer_response(
    result: &DescribeStreamConsumerOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_stream_summary_response(
    result: &DescribeStreamSummaryOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_enhanced_monitoring_response(
    result: &EnhancedMonitoringOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_enhanced_monitoring_response(
    result: &EnhancedMonitoringOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_records_response(result: &GetRecordsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_resource_policy_response(result: &GetResourcePolicyOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_shard_iterator_response(result: &GetShardIteratorOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_increase_stream_retention_period_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_shards_response(result: &ListShardsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_stream_consumers_response(
    result: &ListStreamConsumersOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_streams_response(result: &ListStreamsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_stream_response(result: &ListTagsForStreamOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_merge_shards_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_record_response(result: &PutRecordOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_records_response(result: &PutRecordsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_resource_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_register_stream_consumer_response(
    result: &RegisterStreamConsumerOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_remove_tags_from_stream_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_split_shard_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_start_stream_encryption_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_stop_stream_encryption_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_subscribe_to_shard_response(result: &SubscribeToShardOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_account_settings_response(
    result: &UpdateAccountSettingsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_max_record_size_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_shard_count_response(result: &UpdateShardCountOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_stream_mode_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_stream_warm_throughput_response(
    result: &UpdateStreamWarmThroughputOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_tags_to_stream_request(body: &[u8]) -> Result<AddTagsToStreamInput, String> {
    if body.is_empty() {
        return Ok(AddTagsToStreamInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddTagsToStream request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_stream_request(body: &[u8]) -> Result<CreateStreamInput, String> {
    if body.is_empty() {
        return Ok(CreateStreamInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateStream request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_decrease_stream_retention_period_request(
    body: &[u8],
) -> Result<DecreaseStreamRetentionPeriodInput, String> {
    if body.is_empty() {
        return Ok(DecreaseStreamRetentionPeriodInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DecreaseStreamRetentionPeriod request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resource_policy_request(
    body: &[u8],
) -> Result<DeleteResourcePolicyInput, String> {
    if body.is_empty() {
        return Ok(DeleteResourcePolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_stream_request(body: &[u8]) -> Result<DeleteStreamInput, String> {
    if body.is_empty() {
        return Ok(DeleteStreamInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteStream request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_deregister_stream_consumer_request(
    body: &[u8],
) -> Result<DeregisterStreamConsumerInput, String> {
    if body.is_empty() {
        return Ok(DeregisterStreamConsumerInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeregisterStreamConsumer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_account_settings_request(
    body: &[u8],
) -> Result<DescribeAccountSettingsInput, String> {
    if body.is_empty() {
        return Ok(DescribeAccountSettingsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeAccountSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_limits_request(body: &[u8]) -> Result<DescribeLimitsInput, String> {
    if body.is_empty() {
        return Ok(DescribeLimitsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLimits request: {e}"))
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
pub fn deserialize_describe_stream_consumer_request(
    body: &[u8],
) -> Result<DescribeStreamConsumerInput, String> {
    if body.is_empty() {
        return Ok(DescribeStreamConsumerInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeStreamConsumer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_stream_summary_request(
    body: &[u8],
) -> Result<DescribeStreamSummaryInput, String> {
    if body.is_empty() {
        return Ok(DescribeStreamSummaryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeStreamSummary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_enhanced_monitoring_request(
    body: &[u8],
) -> Result<DisableEnhancedMonitoringInput, String> {
    if body.is_empty() {
        return Ok(DisableEnhancedMonitoringInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableEnhancedMonitoring request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_enhanced_monitoring_request(
    body: &[u8],
) -> Result<EnableEnhancedMonitoringInput, String> {
    if body.is_empty() {
        return Ok(EnableEnhancedMonitoringInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableEnhancedMonitoring request: {e}"))
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
pub fn deserialize_get_resource_policy_request(
    body: &[u8],
) -> Result<GetResourcePolicyInput, String> {
    if body.is_empty() {
        return Ok(GetResourcePolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetResourcePolicy request: {e}"))
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
pub fn deserialize_increase_stream_retention_period_request(
    body: &[u8],
) -> Result<IncreaseStreamRetentionPeriodInput, String> {
    if body.is_empty() {
        return Ok(IncreaseStreamRetentionPeriodInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize IncreaseStreamRetentionPeriod request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_shards_request(body: &[u8]) -> Result<ListShardsInput, String> {
    if body.is_empty() {
        return Ok(ListShardsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListShards request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_stream_consumers_request(
    body: &[u8],
) -> Result<ListStreamConsumersInput, String> {
    if body.is_empty() {
        return Ok(ListStreamConsumersInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListStreamConsumers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_streams_request(body: &[u8]) -> Result<ListStreamsInput, String> {
    if body.is_empty() {
        return Ok(ListStreamsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListStreams request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceInput, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_stream_request(
    body: &[u8],
) -> Result<ListTagsForStreamInput, String> {
    if body.is_empty() {
        return Ok(ListTagsForStreamInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForStream request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_merge_shards_request(body: &[u8]) -> Result<MergeShardsInput, String> {
    if body.is_empty() {
        return Ok(MergeShardsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize MergeShards request: {e}"))
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
pub fn deserialize_put_records_request(body: &[u8]) -> Result<PutRecordsInput, String> {
    if body.is_empty() {
        return Ok(PutRecordsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutRecords request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_resource_policy_request(
    body: &[u8],
) -> Result<PutResourcePolicyInput, String> {
    if body.is_empty() {
        return Ok(PutResourcePolicyInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_register_stream_consumer_request(
    body: &[u8],
) -> Result<RegisterStreamConsumerInput, String> {
    if body.is_empty() {
        return Ok(RegisterStreamConsumerInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RegisterStreamConsumer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_tags_from_stream_request(
    body: &[u8],
) -> Result<RemoveTagsFromStreamInput, String> {
    if body.is_empty() {
        return Ok(RemoveTagsFromStreamInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveTagsFromStream request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_split_shard_request(body: &[u8]) -> Result<SplitShardInput, String> {
    if body.is_empty() {
        return Ok(SplitShardInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SplitShard request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_stream_encryption_request(
    body: &[u8],
) -> Result<StartStreamEncryptionInput, String> {
    if body.is_empty() {
        return Ok(StartStreamEncryptionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartStreamEncryption request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_stream_encryption_request(
    body: &[u8],
) -> Result<StopStreamEncryptionInput, String> {
    if body.is_empty() {
        return Ok(StopStreamEncryptionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopStreamEncryption request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_subscribe_to_shard_request(
    body: &[u8],
) -> Result<SubscribeToShardInput, String> {
    if body.is_empty() {
        return Ok(SubscribeToShardInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SubscribeToShard request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceInput, String> {
    if body.is_empty() {
        return Ok(TagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceInput, String> {
    if body.is_empty() {
        return Ok(UntagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_account_settings_request(
    body: &[u8],
) -> Result<UpdateAccountSettingsInput, String> {
    if body.is_empty() {
        return Ok(UpdateAccountSettingsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAccountSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_max_record_size_request(
    body: &[u8],
) -> Result<UpdateMaxRecordSizeInput, String> {
    if body.is_empty() {
        return Ok(UpdateMaxRecordSizeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateMaxRecordSize request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_shard_count_request(
    body: &[u8],
) -> Result<UpdateShardCountInput, String> {
    if body.is_empty() {
        return Ok(UpdateShardCountInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateShardCount request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_stream_mode_request(
    body: &[u8],
) -> Result<UpdateStreamModeInput, String> {
    if body.is_empty() {
        return Ok(UpdateStreamModeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateStreamMode request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_stream_warm_throughput_request(
    body: &[u8],
) -> Result<UpdateStreamWarmThroughputInput, String> {
    if body.is_empty() {
        return Ok(UpdateStreamWarmThroughputInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateStreamWarmThroughput request: {e}"))
}
