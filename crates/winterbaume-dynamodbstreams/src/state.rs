//! In-memory state for the DynamoDB Streams mock service.
//!
//! Stream existence is derived from DynamoDB table state (via the shared
//! `DynamoDbBackend`). This module only tracks transient shard iterator state.

use std::collections::HashMap;

use winterbaume_dynamodb::DynamoDbError;

/// Error type for DynamoDB Streams operations.
#[derive(Debug, thiserror::Error)]
pub enum DynamoDbStreamsError {
    /// The shard iterator has expired or was not found.
    #[error("Iterator expired or not found")]
    ExpiredIteratorException,

    /// An error propagated from the underlying DynamoDB backend.
    #[error(transparent)]
    BackendError(#[from] DynamoDbError),
}

/// Position of a shard iterator.
pub struct ShardIteratorState {
    pub stream_arn: String,
    pub shard_id: String,
    /// The next record to read has sequence_number >= next_sequence_number.
    /// Starts at 0 for TRIM_HORIZON, u64::MAX for LATEST.
    pub next_sequence_number: u64,
}

/// Per-account, per-region state for DynamoDB Streams.
///
/// Streams themselves are owned by DynamoDB table state; this struct only
/// tracks the transient shard iterator mapping used to validate GetRecords
/// calls within a session.
#[derive(Default)]
pub struct DynamoDbStreamsState {
    /// Shard iterators keyed by iterator ID.
    pub shard_iterators: HashMap<String, ShardIteratorState>,
}
