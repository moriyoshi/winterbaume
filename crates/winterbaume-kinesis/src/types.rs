use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Stream {
    pub name: String,
    pub arn: String,
    pub status: String,
    pub shards: Vec<ShardData>,
    pub records: Vec<Record>,
    pub created_timestamp: DateTime<Utc>,
    pub tags: HashMap<String, String>,
    pub consumers: Vec<StreamConsumer>,
    pub retention_period_hours: u32,
    pub encryption_type: String,
    pub key_id: Option<String>,
    pub enhanced_monitoring: Vec<String>,
    pub resource_policy: Option<String>,
    pub stream_mode: String,
    pub account_id: String,
    pub max_record_size_in_ki_b: Option<i32>,
    /// Sequence number counter per shard within this stream. Real AWS
    /// Kinesis sequence numbers are monotonic per shard, not globally;
    /// each `put_record*` call increments the counter for the resolved
    /// target shard and uses the new value as the `SequenceNumber`.
    /// Missing entries default to zero on first use.
    pub next_sequence_per_shard: HashMap<String, u64>,
}

#[derive(Debug, Clone)]
pub struct ShardData {
    pub shard_id: String,
    pub starting_hash_key: String,
    pub ending_hash_key: String,
    pub parent_shard_id: Option<String>,
    pub adjacent_parent_shard_id: Option<String>,
    pub closed: bool,
}

#[derive(Debug, Clone)]
pub struct StreamConsumer {
    pub consumer_name: String,
    pub consumer_arn: String,
    pub consumer_status: String,
    pub creation_timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub sequence_number: String,
    pub partition_key: String,
    pub shard_id: String,
    pub timestamp: DateTime<Utc>,
}
