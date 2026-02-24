//! Serde-compatible view types for Kinesis state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::KinesisService;
use crate::state::KinesisState;
use crate::types::{ShardData, Stream, StreamConsumer};

/// Serializable view of the entire Kinesis state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KinesisStateView {
    /// Streams keyed by stream name.
    #[serde(default)]
    pub streams: HashMap<String, StreamView>,
    /// Tags keyed by resource ARN (for TagResource / UntagResource / ListTagsForResource).
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// Account-level minimum throughput billing commitment status.
    #[serde(default)]
    pub account_settings_commitment_status: String,
}

/// Serializable view of a single Kinesis stream (durable fields only).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamView {
    /// Stream name.
    pub name: String,
    /// Stream ARN.
    pub arn: String,
    /// Stream status.
    pub status: String,
    /// Stream mode (PROVISIONED or ON_DEMAND).
    pub stream_mode: String,
    /// Shards in the stream.
    #[serde(default)]
    pub shards: Vec<ShardView>,
    /// Retention period in hours.
    pub retention_period_hours: u32,
    /// Encryption type.
    pub encryption_type: String,
    /// KMS key ID (if encrypted).
    pub key_id: Option<String>,
    /// Tags.
    #[serde(default)]
    pub tags: HashMap<String, String>,
    /// Registered consumers.
    #[serde(default)]
    pub consumers: Vec<ConsumerView>,
    /// Stream creation timestamp in RFC 3339 format.
    pub created_timestamp: Option<String>,
    /// Account ID.
    pub account_id: String,
    /// Enhanced monitoring shard-level metrics.
    #[serde(default)]
    pub enhanced_monitoring: Vec<String>,
    /// Resource policy (raw JSON string).
    pub resource_policy: Option<String>,
    /// Maximum record size in KiB (if set).
    #[serde(default)]
    pub max_record_size_in_ki_b: Option<i32>,
}

/// Serializable view of a Kinesis shard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardView {
    pub shard_id: String,
    pub starting_hash_key: String,
    pub ending_hash_key: String,
    pub parent_shard_id: Option<String>,
    pub adjacent_parent_shard_id: Option<String>,
    pub closed: bool,
}

/// Serializable view of a registered stream consumer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerView {
    pub consumer_name: String,
    pub consumer_arn: String,
    pub consumer_status: String,
    pub creation_timestamp: Option<String>,
}

// --- From internal types to view types ---

impl From<&KinesisState> for KinesisStateView {
    fn from(state: &KinesisState) -> Self {
        KinesisStateView {
            streams: state
                .streams
                .iter()
                .map(|(k, v)| (k.clone(), StreamView::from(v)))
                .collect(),
            resource_tags: state.resource_tags.clone(),
            account_settings_commitment_status: state.account_settings_commitment_status.clone(),
        }
    }
}

impl From<&Stream> for StreamView {
    fn from(s: &Stream) -> Self {
        StreamView {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: s.status.clone(),
            stream_mode: s.stream_mode.clone(),
            shards: s.shards.iter().map(ShardView::from).collect(),
            retention_period_hours: s.retention_period_hours,
            encryption_type: s.encryption_type.clone(),
            key_id: s.key_id.clone(),
            tags: s.tags.clone(),
            consumers: s.consumers.iter().map(ConsumerView::from).collect(),
            created_timestamp: Some(s.created_timestamp.to_rfc3339()),
            account_id: s.account_id.clone(),
            enhanced_monitoring: s.enhanced_monitoring.clone(),
            resource_policy: s.resource_policy.clone(),
            max_record_size_in_ki_b: s.max_record_size_in_ki_b,
        }
    }
}

impl From<&ShardData> for ShardView {
    fn from(sd: &ShardData) -> Self {
        ShardView {
            shard_id: sd.shard_id.clone(),
            starting_hash_key: sd.starting_hash_key.clone(),
            ending_hash_key: sd.ending_hash_key.clone(),
            parent_shard_id: sd.parent_shard_id.clone(),
            adjacent_parent_shard_id: sd.adjacent_parent_shard_id.clone(),
            closed: sd.closed,
        }
    }
}

impl From<&StreamConsumer> for ConsumerView {
    fn from(c: &StreamConsumer) -> Self {
        ConsumerView {
            consumer_name: c.consumer_name.clone(),
            consumer_arn: c.consumer_arn.clone(),
            consumer_status: c.consumer_status.clone(),
            creation_timestamp: Some(c.creation_timestamp.to_rfc3339()),
        }
    }
}

// --- From view types to internal types ---

impl From<KinesisStateView> for KinesisState {
    fn from(view: KinesisStateView) -> Self {
        let mut state = KinesisState::default();
        state.streams = view
            .streams
            .into_iter()
            .map(|(k, v)| (k, Stream::from(v)))
            .collect();
        state.resource_tags = view.resource_tags;
        state.account_settings_commitment_status = view.account_settings_commitment_status;
        state
    }
}

impl From<StreamView> for Stream {
    fn from(v: StreamView) -> Self {
        let created_timestamp = v
            .created_timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        Stream {
            name: v.name,
            arn: v.arn,
            status: v.status,
            stream_mode: v.stream_mode,
            shards: v.shards.into_iter().map(ShardData::from).collect(),
            records: Vec::new(),
            retention_period_hours: v.retention_period_hours,
            encryption_type: v.encryption_type,
            key_id: v.key_id,
            tags: v.tags,
            consumers: v.consumers.into_iter().map(StreamConsumer::from).collect(),
            created_timestamp,
            account_id: v.account_id,
            enhanced_monitoring: v.enhanced_monitoring,
            resource_policy: v.resource_policy,
            max_record_size_in_ki_b: v.max_record_size_in_ki_b,
        }
    }
}

impl From<ShardView> for ShardData {
    fn from(v: ShardView) -> Self {
        ShardData {
            shard_id: v.shard_id,
            starting_hash_key: v.starting_hash_key,
            ending_hash_key: v.ending_hash_key,
            parent_shard_id: v.parent_shard_id,
            adjacent_parent_shard_id: v.adjacent_parent_shard_id,
            closed: v.closed,
        }
    }
}

impl From<ConsumerView> for StreamConsumer {
    fn from(v: ConsumerView) -> Self {
        let creation_timestamp = v
            .creation_timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        StreamConsumer {
            consumer_name: v.consumer_name,
            consumer_arn: v.consumer_arn,
            consumer_status: v.consumer_status,
            creation_timestamp,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for KinesisService {
    type StateView = KinesisStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        KinesisStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = KinesisState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (name, stream_view) in view.streams {
                guard.streams.insert(name, Stream::from(stream_view));
            }
            // Account-level singleton: overwrite only when the incoming view
            // explicitly carries a non-empty value, preserving the existing
            // setting otherwise (matches the xray singleton precedent).
            if !view.account_settings_commitment_status.is_empty() {
                guard.account_settings_commitment_status = view.account_settings_commitment_status;
            }
            // Per-ARN tag maps: deep-merge so existing tags on other ARNs are
            // preserved and per-key tags within an ARN are unioned (incoming
            // wins on key collision). Matches the EmrServerless precedent.
            for (arn, tags) in view.resource_tags {
                guard.resource_tags.entry(arn).or_default().extend(tags);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
