//! Serde-compatible view types for SQS state snapshots.

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StatefulService};

use crate::handlers::SqsService;
use crate::state::{QueueState, SqsState};
use crate::types::{MessageMoveTask, Queue};

/// Serializable view of the entire SQS state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SqsStateView {
    /// Queues keyed by queue name.
    #[serde(default)]
    pub queues: HashMap<String, QueueStateView>,
    /// Asynchronous DLQ-redrive task records (newest at the end).
    #[serde(default)]
    pub message_move_tasks: Vec<MessageMoveTaskView>,
}

/// Serializable view of a single SQS queue configuration.
///
/// Transient data (messages, deleted handles, permissions) is not included;
/// only the durable queue configuration is captured.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueueStateView {
    pub name: String,
    pub url: String,
    pub arn: String,
    pub region: String,
    pub account_id: String,
    /// Created timestamp in RFC 3339 format.
    pub created_timestamp: Option<String>,
    /// Last modified timestamp in RFC 3339 format.
    pub last_modified_timestamp: Option<String>,
    pub visibility_timeout: u32,
    pub delay_seconds: u32,
    pub maximum_message_size: u32,
    pub message_retention_period: u32,
    pub receive_wait_time_seconds: u32,
    pub fifo_queue: bool,
    #[serde(default)]
    pub content_based_deduplication: bool,
    /// Queue tags.
    #[serde(default)]
    pub tags: HashMap<String, String>,
    /// Raw RedrivePolicy JSON string.
    pub redrive_policy: Option<String>,
    /// Raw Policy JSON string.
    pub policy: Option<String>,
}

/// Serializable view of an asynchronous DLQ-redrive task.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageMoveTaskView {
    pub task_handle: String,
    pub source_arn: String,
    pub destination_arn: Option<String>,
    pub status: String,
    pub max_number_of_messages_per_second: Option<i32>,
    pub started_timestamp: i64,
    pub approximate_number_of_messages_moved: i64,
    pub approximate_number_of_messages_to_move: Option<i64>,
}

fn parse_datetime(s: Option<&str>) -> DateTime<Utc> {
    s.and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(Utc::now)
}

/// Extract dead letter target ARN from a redrive policy JSON string.
fn extract_dead_letter_target_arn(policy: Option<&str>) -> Option<String> {
    policy
        .and_then(|p| serde_json::from_str::<serde_json::Value>(p).ok())
        .and_then(|v| {
            v.get("deadLetterTargetArn")
                .and_then(|a| a.as_str())
                .map(|s| s.to_string())
        })
}

// --- From internal types to view types ---

impl From<&SqsState> for SqsStateView {
    fn from(state: &SqsState) -> Self {
        SqsStateView {
            queues: state
                .queues
                .iter()
                .map(|(k, v)| (k.clone(), QueueStateView::from(v)))
                .collect(),
            message_move_tasks: state
                .message_move_tasks
                .iter()
                .map(MessageMoveTaskView::from)
                .collect(),
        }
    }
}

impl From<&MessageMoveTask> for MessageMoveTaskView {
    fn from(t: &MessageMoveTask) -> Self {
        MessageMoveTaskView {
            task_handle: t.task_handle.clone(),
            source_arn: t.source_arn.clone(),
            destination_arn: t.destination_arn.clone(),
            status: t.status.clone(),
            max_number_of_messages_per_second: t.max_number_of_messages_per_second,
            started_timestamp: t.started_timestamp,
            approximate_number_of_messages_moved: t.approximate_number_of_messages_moved,
            approximate_number_of_messages_to_move: t.approximate_number_of_messages_to_move,
        }
    }
}

impl From<MessageMoveTaskView> for MessageMoveTask {
    fn from(view: MessageMoveTaskView) -> Self {
        MessageMoveTask {
            task_handle: view.task_handle,
            source_arn: view.source_arn,
            destination_arn: view.destination_arn,
            status: view.status,
            max_number_of_messages_per_second: view.max_number_of_messages_per_second,
            started_timestamp: view.started_timestamp,
            approximate_number_of_messages_moved: view.approximate_number_of_messages_moved,
            approximate_number_of_messages_to_move: view.approximate_number_of_messages_to_move,
        }
    }
}

impl From<&QueueState> for QueueStateView {
    fn from(qs: &QueueState) -> Self {
        QueueStateView {
            name: qs.queue.name.clone(),
            url: qs.queue.url.clone(),
            arn: qs.queue.arn.clone(),
            region: qs.queue.region.clone(),
            account_id: qs.queue.account_id.clone(),
            created_timestamp: Some(qs.queue.created_timestamp.to_rfc3339()),
            last_modified_timestamp: Some(qs.queue.last_modified_timestamp.to_rfc3339()),
            visibility_timeout: qs.queue.visibility_timeout,
            delay_seconds: qs.queue.delay_seconds,
            maximum_message_size: qs.queue.maximum_message_size,
            message_retention_period: qs.queue.message_retention_period,
            receive_wait_time_seconds: qs.queue.receive_wait_time_seconds,
            fifo_queue: qs.queue.fifo_queue,
            content_based_deduplication: qs.queue.content_based_deduplication,
            tags: qs.tags.clone(),
            redrive_policy: qs.redrive_policy.clone(),
            policy: qs.policy.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<SqsStateView> for SqsState {
    fn from(view: SqsStateView) -> Self {
        let message_move_tasks: Vec<MessageMoveTask> = view
            .message_move_tasks
            .into_iter()
            .map(MessageMoveTask::from)
            .collect();
        let message_move_task_counter = message_move_tasks.len() as u64;
        SqsState {
            queues: view
                .queues
                .into_iter()
                .map(|(k, v)| (k, QueueState::from(v)))
                .collect(),
            message_move_tasks,
            message_move_task_counter,
        }
    }
}

impl From<QueueStateView> for QueueState {
    fn from(view: QueueStateView) -> Self {
        let dead_letter_target_arn = extract_dead_letter_target_arn(view.redrive_policy.as_deref());
        QueueState {
            queue: Queue {
                name: view.name,
                url: view.url,
                arn: view.arn,
                region: view.region,
                account_id: view.account_id,
                created_timestamp: parse_datetime(view.created_timestamp.as_deref()),
                last_modified_timestamp: parse_datetime(view.last_modified_timestamp.as_deref()),
                visibility_timeout: view.visibility_timeout,
                delay_seconds: view.delay_seconds,
                maximum_message_size: view.maximum_message_size,
                message_retention_period: view.message_retention_period,
                receive_wait_time_seconds: view.receive_wait_time_seconds,
                fifo_queue: view.fifo_queue,
                content_based_deduplication: view.content_based_deduplication,
            },
            messages: Vec::new(),
            deleted_handles: HashSet::new(),
            tags: view.tags,
            permissions: HashMap::new(),
            dead_letter_target_arn,
            redrive_policy: view.redrive_policy,
            policy: view.policy,
            dedup_index: HashMap::new(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for SqsService {
    type StateView = SqsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        self.backend
            .snapshot(account_id.to_owned(), region.to_owned())
            .await
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), winterbaume_core::StateViewError> {
        self.backend
            .restore(account_id.to_owned(), region.to_owned(), view)
            .await?;
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), winterbaume_core::StateViewError> {
        self.backend
            .merge(account_id.to_owned(), region.to_owned(), view)
            .await?;
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
