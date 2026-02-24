//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-kinesis

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsToStreamInput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStreamInput {
    #[serde(rename = "MaxRecordSizeInKiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_record_size_in_ki_b: Option<i32>,
    #[serde(rename = "ShardCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    #[serde(rename = "StreamModeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_mode_details: Option<StreamModeDetails>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    pub stream_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "WarmThroughputMiBps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput_mi_bps: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamModeDetails {
    #[serde(rename = "StreamMode")]
    #[serde(default)]
    pub stream_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecreaseStreamRetentionPeriodInput {
    #[serde(rename = "RetentionPeriodHours")]
    #[serde(default)]
    pub retention_period_hours: i32,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStreamInput {
    #[serde(rename = "EnforceConsumerDeletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_consumer_deletion: Option<bool>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterStreamConsumerInput {
    #[serde(rename = "ConsumerARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_a_r_n: Option<String>,
    #[serde(rename = "ConsumerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_name: Option<String>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountSettingsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountSettingsOutput {
    #[serde(rename = "MinimumThroughputBillingCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_throughput_billing_commitment: Option<MinimumThroughputBillingCommitmentOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MinimumThroughputBillingCommitmentOutput {
    #[serde(rename = "EarliestAllowedEndAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_allowed_end_at: Option<f64>,
    #[serde(rename = "EndedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<f64>,
    #[serde(rename = "StartedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLimitsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLimitsOutput {
    #[serde(rename = "OnDemandStreamCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_stream_count: Option<i32>,
    #[serde(rename = "OnDemandStreamCountLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_stream_count_limit: Option<i32>,
    #[serde(rename = "OpenShardCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_shard_count: Option<i32>,
    #[serde(rename = "ShardLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_limit: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamConsumerInput {
    #[serde(rename = "ConsumerARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_a_r_n: Option<String>,
    #[serde(rename = "ConsumerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_name: Option<String>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamConsumerOutput {
    #[serde(rename = "ConsumerDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_description: Option<ConsumerDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConsumerDescription {
    #[serde(rename = "ConsumerARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_a_r_n: Option<String>,
    #[serde(rename = "ConsumerCreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_creation_timestamp: Option<f64>,
    #[serde(rename = "ConsumerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_name: Option<String>,
    #[serde(rename = "ConsumerStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_status: Option<String>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamInput {
    #[serde(rename = "ExclusiveStartShardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_shard_id: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamOutput {
    #[serde(rename = "StreamDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_description: Option<StreamDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamDescription {
    #[serde(rename = "EncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "EnhancedMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<Vec<EnhancedMetrics>>,
    #[serde(rename = "HasMoreShards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more_shards: Option<bool>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "RetentionPeriodHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_hours: Option<i32>,
    #[serde(rename = "Shards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shards: Option<Vec<Shard>>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamCreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_creation_timestamp: Option<f64>,
    #[serde(rename = "StreamModeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_mode_details: Option<StreamModeDetails>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnhancedMetrics {
    #[serde(rename = "ShardLevelMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_level_metrics: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Shard {
    #[serde(rename = "AdjacentParentShardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjacent_parent_shard_id: Option<String>,
    #[serde(rename = "HashKeyRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_key_range: Option<HashKeyRange>,
    #[serde(rename = "ParentShardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_shard_id: Option<String>,
    #[serde(rename = "SequenceNumberRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number_range: Option<SequenceNumberRange>,
    #[serde(rename = "ShardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HashKeyRange {
    #[serde(rename = "EndingHashKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_hash_key: Option<String>,
    #[serde(rename = "StartingHashKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_hash_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SequenceNumberRange {
    #[serde(rename = "EndingSequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_sequence_number: Option<String>,
    #[serde(rename = "StartingSequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_sequence_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamSummaryInput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamSummaryOutput {
    #[serde(rename = "StreamDescriptionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_description_summary: Option<StreamDescriptionSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamDescriptionSummary {
    #[serde(rename = "ConsumerCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_count: Option<i32>,
    #[serde(rename = "EncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "EnhancedMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<Vec<EnhancedMetrics>>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "MaxRecordSizeInKiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_record_size_in_ki_b: Option<i32>,
    #[serde(rename = "OpenShardCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_shard_count: Option<i32>,
    #[serde(rename = "RetentionPeriodHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_hours: Option<i32>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamCreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_creation_timestamp: Option<f64>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamModeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_mode_details: Option<StreamModeDetails>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_status: Option<String>,
    #[serde(rename = "WarmThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput: Option<WarmThroughputObject>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WarmThroughputObject {
    #[serde(rename = "CurrentMiBps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_mi_bps: Option<i32>,
    #[serde(rename = "TargetMiBps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_mi_bps: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableEnhancedMonitoringInput {
    #[serde(rename = "ShardLevelMetrics")]
    #[serde(default)]
    pub shard_level_metrics: Vec<String>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableEnhancedMonitoringInput {
    #[serde(rename = "ShardLevelMetrics")]
    #[serde(default)]
    pub shard_level_metrics: Vec<String>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnhancedMonitoringOutput {
    #[serde(rename = "CurrentShardLevelMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_shard_level_metrics: Option<Vec<String>>,
    #[serde(rename = "DesiredShardLevelMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_shard_level_metrics: Option<Vec<String>>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecordsInput {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "ShardIterator")]
    #[serde(default)]
    pub shard_iterator: String,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecordsOutput {
    #[serde(rename = "ChildShards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_shards: Option<Vec<ChildShard>>,
    #[serde(rename = "MillisBehindLatest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub millis_behind_latest: Option<i64>,
    #[serde(rename = "NextShardIterator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_shard_iterator: Option<String>,
    #[serde(rename = "Records")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChildShard {
    #[serde(rename = "HashKeyRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_key_range: Option<HashKeyRange>,
    #[serde(rename = "ParentShards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_shards: Option<Vec<String>>,
    #[serde(rename = "ShardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Record {
    #[serde(rename = "ApproximateArrivalTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_arrival_timestamp: Option<f64>,
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "EncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "PartitionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
    #[serde(rename = "SequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyOutput {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetShardIteratorInput {
    #[serde(rename = "ShardId")]
    #[serde(default)]
    pub shard_id: String,
    #[serde(rename = "ShardIteratorType")]
    #[serde(default)]
    pub shard_iterator_type: String,
    #[serde(rename = "StartingSequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_sequence_number: Option<String>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetShardIteratorOutput {
    #[serde(rename = "ShardIterator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_iterator: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IncreaseStreamRetentionPeriodInput {
    #[serde(rename = "RetentionPeriodHours")]
    #[serde(default)]
    pub retention_period_hours: i32,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListShardsInput {
    #[serde(rename = "ExclusiveStartShardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_shard_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ShardFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_filter: Option<ShardFilter>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamCreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_creation_timestamp: Option<f64>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShardFilter {
    #[serde(rename = "ShardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListShardsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Shards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shards: Option<Vec<Shard>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamConsumersInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    pub stream_a_r_n: String,
    #[serde(rename = "StreamCreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_creation_timestamp: Option<f64>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamConsumersOutput {
    #[serde(rename = "Consumers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumers: Option<Vec<Consumer>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Consumer {
    #[serde(rename = "ConsumerARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_a_r_n: Option<String>,
    #[serde(rename = "ConsumerCreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_creation_timestamp: Option<f64>,
    #[serde(rename = "ConsumerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_name: Option<String>,
    #[serde(rename = "ConsumerStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamsInput {
    #[serde(rename = "ExclusiveStartStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_stream_name: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamsOutput {
    #[serde(rename = "HasMoreStreams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more_streams: Option<bool>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StreamNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_names: Option<Vec<String>>,
    #[serde(rename = "StreamSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_summaries: Option<Vec<StreamSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamSummary {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamCreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_creation_timestamp: Option<f64>,
    #[serde(rename = "StreamModeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_mode_details: Option<StreamModeDetails>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForStreamInput {
    #[serde(rename = "ExclusiveStartTagKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_tag_key: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForStreamOutput {
    #[serde(rename = "HasMoreTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more_tags: Option<bool>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MergeShardsInput {
    #[serde(rename = "AdjacentShardToMerge")]
    #[serde(default)]
    pub adjacent_shard_to_merge: String,
    #[serde(rename = "ShardToMerge")]
    #[serde(default)]
    pub shard_to_merge: String,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRecordInput {
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
    #[serde(rename = "ExplicitHashKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_hash_key: Option<String>,
    #[serde(rename = "PartitionKey")]
    #[serde(default)]
    pub partition_key: String,
    #[serde(rename = "SequenceNumberForOrdering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number_for_ordering: Option<String>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRecordOutput {
    #[serde(rename = "EncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "SequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    #[serde(rename = "ShardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRecordsInput {
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<PutRecordsRequestEntry>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRecordsRequestEntry {
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
    #[serde(rename = "ExplicitHashKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_hash_key: Option<String>,
    #[serde(rename = "PartitionKey")]
    #[serde(default)]
    pub partition_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRecordsOutput {
    #[serde(rename = "EncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "FailedRecordCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_record_count: Option<i32>,
    #[serde(rename = "Records")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<PutRecordsResultEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRecordsResultEntry {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "SequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    #[serde(rename = "ShardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyInput {
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterStreamConsumerInput {
    #[serde(rename = "ConsumerName")]
    #[serde(default)]
    pub consumer_name: String,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    pub stream_a_r_n: String,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterStreamConsumerOutput {
    #[serde(rename = "Consumer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer: Option<Consumer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsFromStreamInput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SplitShardInput {
    #[serde(rename = "NewStartingHashKey")]
    #[serde(default)]
    pub new_starting_hash_key: String,
    #[serde(rename = "ShardToSplit")]
    #[serde(default)]
    pub shard_to_split: String,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartStreamEncryptionInput {
    #[serde(rename = "EncryptionType")]
    #[serde(default)]
    pub encryption_type: String,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopStreamEncryptionInput {
    #[serde(rename = "EncryptionType")]
    #[serde(default)]
    pub encryption_type: String,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscribeToShardInput {
    #[serde(rename = "ConsumerARN")]
    #[serde(default)]
    pub consumer_a_r_n: String,
    #[serde(rename = "ShardId")]
    #[serde(default)]
    pub shard_id: String,
    #[serde(rename = "StartingPosition")]
    #[serde(default)]
    pub starting_position: StartingPosition,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartingPosition {
    #[serde(rename = "SequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscribeToShardOutput {
    #[serde(rename = "EventStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_stream: Option<SubscribeToShardEventStream>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscribeToShardEventStream {
    #[serde(rename = "InternalFailureException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_failure_exception: Option<InternalFailureException>,
    #[serde(rename = "KMSAccessDeniedException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_access_denied_exception: Option<KMSAccessDeniedException>,
    #[serde(rename = "KMSDisabledException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_disabled_exception: Option<KMSDisabledException>,
    #[serde(rename = "KMSInvalidStateException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_invalid_state_exception: Option<KMSInvalidStateException>,
    #[serde(rename = "KMSNotFoundException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_not_found_exception: Option<KMSNotFoundException>,
    #[serde(rename = "KMSOptInRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_opt_in_required: Option<KMSOptInRequired>,
    #[serde(rename = "KMSThrottlingException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_throttling_exception: Option<KMSThrottlingException>,
    #[serde(rename = "ResourceInUseException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_in_use_exception: Option<ResourceInUseException>,
    #[serde(rename = "ResourceNotFoundException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_not_found_exception: Option<ResourceNotFoundException>,
    #[serde(rename = "SubscribeToShardEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_to_shard_event: Option<SubscribeToShardEvent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InternalFailureException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KMSAccessDeniedException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KMSDisabledException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KMSInvalidStateException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KMSNotFoundException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KMSOptInRequired {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KMSThrottlingException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceInUseException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceNotFoundException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscribeToShardEvent {
    #[serde(rename = "ChildShards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_shards: Option<Vec<ChildShard>>,
    #[serde(rename = "ContinuationSequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_sequence_number: Option<String>,
    #[serde(rename = "MillisBehindLatest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub millis_behind_latest: Option<i64>,
    #[serde(rename = "Records")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountSettingsInput {
    #[serde(rename = "MinimumThroughputBillingCommitment")]
    #[serde(default)]
    pub minimum_throughput_billing_commitment: MinimumThroughputBillingCommitmentInput,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MinimumThroughputBillingCommitmentInput {
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountSettingsOutput {
    #[serde(rename = "MinimumThroughputBillingCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_throughput_billing_commitment: Option<MinimumThroughputBillingCommitmentOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMaxRecordSizeInput {
    #[serde(rename = "MaxRecordSizeInKiB")]
    #[serde(default)]
    pub max_record_size_in_ki_b: i32,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateShardCountInput {
    #[serde(rename = "ScalingType")]
    #[serde(default)]
    pub scaling_type: String,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "TargetShardCount")]
    #[serde(default)]
    pub target_shard_count: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateShardCountOutput {
    #[serde(rename = "CurrentShardCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_shard_count: Option<i32>,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "TargetShardCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_shard_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStreamModeInput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    pub stream_a_r_n: String,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamModeDetails")]
    #[serde(default)]
    pub stream_mode_details: StreamModeDetails,
    #[serde(rename = "WarmThroughputMiBps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput_mi_bps: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStreamWarmThroughputInput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "WarmThroughputMiBps")]
    #[serde(default)]
    pub warm_throughput_mi_bps: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStreamWarmThroughputOutput {
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_a_r_n: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "WarmThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_throughput: Option<WarmThroughputObject>,
}
